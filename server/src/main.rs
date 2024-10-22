use artnet::ArtNetClient;
use config::{yaml::parse_yaml_into, Binding, Config, Fixture};
use core::panic;
use handlers::{handle_websocket_message, WebsocketHandlingError};
use logger::{debug, error, info, warning};
use std::{
    collections::HashMap, fmt::Error, fs::read_to_string, net::TcpListener, sync::Arc, thread,
    time::Duration,
};
use tungstenite::accept;

#[cfg(test)]
mod test;

mod handlers;

fn main() -> Result<(), Error> {
    let (fixtures, bindings, config) = read_parse_config_file();

    {
        let dfc_allowed = config.get_allow_direct_fixture_control();
        let commit_every_ms = config.get_send_every_ms().unwrap_or(50);
        if dfc_allowed && commit_every_ms == 0 {
            panic!("direct fixture control is enabled but commit_every_ms is set to 0");
        }
        if dfc_allowed {
            debug!(
                "direct fixture control is enabled with regular commit every {}ms",
                commit_every_ms
            );
        }
    }

    let tcp_server_bind = config.get_server_bind();
    let tcp_server = TcpListener::bind(tcp_server_bind)
        .expect(&format!("failed to bind to {}", tcp_server_bind));
    info!("server started: ws://{}", tcp_server_bind);

    let send_via_artnet_udp_socket = Arc::new(
        artnet::create_socket(
            config.get_artnet_bind().to_string(),
            config.get_artnet_send().to_string(),
            config.get_artnet_broadcast(),
        )
        .expect(&format!(
            "error binding artnet socket on {}",
            config.get_artnet_bind()
        )),
    );
    let artnet_client = Arc::new(ArtNetClient::new(
        send_via_artnet_udp_socket,
        config.get_artnet_universe(),
    ));

    if config.get_send_every_ms().is_some() {
        let artnet_client_artnet_client_commit_regulary_clone = Arc::clone(&artnet_client);
        let configuration_commit_regulary_clone = Arc::clone(&config);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(
                configuration_commit_regulary_clone
                    .get_send_every_ms()
                    .unwrap(),
            ));
            let commit_result = artnet_client_artnet_client_commit_regulary_clone.commit();
            if let Err(err) = commit_result {
                error!("error sending artnet package: {:?}", err);
            }
        });
    }

    for tcp_stream in tcp_server.incoming() {
        let artnet_client_current_thread_clone = Arc::clone(&artnet_client);
        let fixtures_current_thread_clone = Arc::clone(&fixtures);
        let bindings_current_thread_clone = Arc::clone(&bindings);
        let allow_direct_fixture_control = Arc::new(config.get_allow_direct_fixture_control());

        thread::spawn(move || {
            let tcp_stream = match tcp_stream {
                Ok(stream) => stream,
                Err(err) => {
                    error!("failed to accept incoming TCP stream: {:?}", err);
                    return;
                }
            };
            let mut websocket = match accept(tcp_stream) {
                Ok(ws) => ws,
                Err(err) => {
                    error!("failed to accept WebSocket connection: {:?}", err);
                    return;
                }
            };

            loop {
                let msg = match websocket.read() {
                    Ok(msg) => msg,
                    Err(err) => {
                        error!("failed to read from WebSocket: {:?}", err);
                        break; // Break the loop if there's an error reading from the socket
                    }
                };
                if msg.is_close() {
                    info!("WebSocket connection closed by client.");
                    break;
                }
                if msg.is_empty() || msg.is_ping() || msg.is_pong() {
                    continue;
                }

                let ws_msg_str = match msg.to_text() {
                    Ok(text) => text,
                    Err(err) => {
                        error!("failed to convert WebSocket message to text: {:?}", err);
                        break; // Exit loop on message parsing error
                    }
                };

                debug!("recieved message over websocket: {}", ws_msg_str);

                let handling_result = handle_websocket_message(
                    ws_msg_str,
                    artnet_client_current_thread_clone.clone(),
                    fixtures_current_thread_clone.clone(),
                    bindings_current_thread_clone.clone(),
                    allow_direct_fixture_control.clone(),
                );

                match handling_result {
                    Ok(_) => {
                        websocket
                            .send("OK".into())
                            .expect("failed to send a response to the client");
                    }
                    Err(err) => match err {
                        WebsocketHandlingError::ChannelNotFound(fixture_name, channel_name) => {
                            warning!(
                                "channel {} could not be found on fixture {}. check your config.",
                                channel_name,
                                fixture_name
                            );
                            websocket
                                .send(
                                    format!(
                                        "channel {} could not be found on fixture {}",
                                        channel_name, fixture_name
                                    )
                                    .into(),
                                )
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::FixtureNotFound(fixture_name) => {
                            warning!(
                                "fixture {} does not exist. check your config.",
                                fixture_name
                            );
                            websocket
                                .send(format!("fixture {} does not exist", fixture_name).into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::DfcDisabledButMsgIsInDfcFormat => {
                            warning!("dfc is disabled but message is in dfc format");
                            websocket
                                .send("dfc is disabled but message is in dfc format".into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::IoError(io_err) => {
                            error!("error sending artnet package: {:?}", io_err);
                            websocket
                                .send("error sending artnet package".into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::UnknownMessage(msg) => {
                            warning!("unknown message: {}", msg);
                            websocket
                                .send("unknown message".into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::ExtractVariableError(val) => {
                            warning!("variable not found for: {}", val);
                            websocket
                                .send("variable not found".into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::ParseVariableToNumError(parse_err) => {
                            warning!("failed to parse variable to number (u8): {:?}", parse_err);
                            websocket
                                .send("failed to parse variable to number.".into())
                                .expect("failed to send a response to the client");
                        }
                    },
                }
            }
        });
    }

    Ok(())
}

fn read_parse_config_file() -> (
    Arc<HashMap<String, Fixture>>,
    Arc<HashMap<String, Binding>>,
    Arc<Config>,
) {
    let config_file_path = std::env::args()
        .nth(1)
        .expect("please provide a config file path as the first argument");
    if config_file_path.ends_with(".yaml") || config_file_path.ends_with(".yml") {
        debug!("using configuration: {}", config_file_path);
    } else {
        panic!("file format not supported. Use .yaml or .yml");
    }

    let config_file_content_str = read_to_string(&config_file_path)
        .expect("failed to read configuration file. Does file exist?");

    let (fixtures, bindings, config) = match parse_yaml_into(&config_file_content_str) {
        Ok(result) => result,
        Err(err) => {
            panic!("failed to parse configuration file: {:?}", err);
        }
    };

    let fixtures = Arc::new(fixtures);
    let bindings = Arc::new(bindings);
    let config = Arc::new(config);
    (fixtures, bindings, config)
}
