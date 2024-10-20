use artnet::{create_socket, ArtNetClient};
use config::{parse_yaml_into, Binding, Config, Fixture};
use handlers::{handle_websocket_message, WebsocketHandlingError};
use logger::{debug, error, info, warning};
use std::{
    collections::HashMap, fmt::Error, fs::read_to_string, net::TcpListener, sync::Arc, thread,
};
use tungstenite::accept;

#[cfg(test)]
mod test;

mod handlers;

fn main() -> Result<(), Error> {
    let config_file_path = std::env::args()
        .nth(1)
        .expect("no config file path provided");
    assert!(
        config_file_path.ends_with(".yaml"),
        "config file must be a yaml file"
    );

    let (fixtures, bindings, configuration) = read_parse_config_file(&config_file_path);

    let tcp_server = TcpListener::bind(configuration.get_server_bind()).expect(&format!(
        "failed to bind to {}",
        configuration.get_server_bind()
    ));
    info(&format!(
        "server started: ws://{}",
        configuration.get_server_bind()
    ));

    let send_via_artnet_udp_socket = Arc::new(
        create_socket(
            configuration.get_artnet_bind().to_string(),
            configuration.get_artnet_send().to_string(),
            configuration.get_artnet_broadcast(),
        )
        .expect(&format!(
            "error binding artnet socket on {}",
            configuration.get_artnet_bind()
        )),
    );
    let artnet_client = Arc::new(ArtNetClient::new(
        send_via_artnet_udp_socket,
        configuration.get_artnet_universe(),
    ));

    {
        let artnet_client_artnet_client_commit_regulary_clone = Arc::clone(&artnet_client);
        thread::spawn(move || loop {
            thread::sleep(std::time::Duration::from_millis(50));
            artnet_client_artnet_client_commit_regulary_clone
                .commit()
                .expect("failed to commit artnet data");
        });
    }

    for tcp_stream in tcp_server.incoming() {
        let artnet_client_current_thread_clone = Arc::clone(&artnet_client);
        let fixtures_current_thread_clone = Arc::clone(&fixtures);
        let bindings_current_thread_clone = Arc::clone(&bindings);

        thread::spawn(move || {
            let mut websocket = accept(tcp_stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read().expect("failed to read socket message");
                if msg.is_empty() || msg.is_close() || msg.is_ping() || msg.is_pong() {
                    continue;
                }
                let ws_msg_str = msg.to_text().expect("failed to convert message to string");

                debug(&format!("msg over websocket: {}", ws_msg_str));

                let handling_result = handle_websocket_message(
                    ws_msg_str,
                    artnet_client_current_thread_clone.clone(),
                    fixtures_current_thread_clone.clone(),
                    bindings_current_thread_clone.clone(),
                );

                match handling_result {
                    Ok(_) => {
                        websocket
                            .send("OK".into())
                            .expect("failed to send a response to the client");
                    }
                    Err(err) => match err {
                        WebsocketHandlingError::IoError(err) => {
                            warning(&format!("IO error: {}", err));
                            websocket
                                .send(err.to_string().into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::UnknownMessage(err_str) => {
                            warning(&err_str);
                            websocket
                                .send(err_str.into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::ParseError(err_str) => {
                            warning(&err_str);
                            websocket
                                .send(err_str.into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::ChannelNotFound(err_str) => {
                            warning(&err_str);
                            websocket
                                .send(err_str.into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::FixtureNotFound(err_str) => {
                            warning(&err_str);
                            websocket
                                .send(err_str.into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::InvalidActionFormat(err_str) => {
                            warning(&err_str);
                            websocket
                                .send(err_str.into())
                                .expect("failed to send a response to the client");
                        }
                        WebsocketHandlingError::VariableNotFound(err_str) => {
                            warning(&err_str);
                            websocket
                                .send(err_str.into())
                                .expect("failed to send a response to the client");
                        }
                    },
                }
            }
        });
    }

    Ok(())
}

fn read_parse_config_file(
    config_file_path: &str,
) -> (
    Arc<HashMap<String, Fixture>>,
    Arc<HashMap<String, Binding>>,
    Config,
) {
    let config_file_content_str =
        read_to_string(&config_file_path).expect("failed to read config file");
    let (fixtures, bindings, config) = match parse_yaml_into(&config_file_content_str) {
        Ok(result) => {
            debug("successfully parsed config file");
            result
        }
        Err(err) => {
            error(&format!("failed to parse config file: {}", err));
            panic!("failed to parse config file");
        }
    };

    let fixtures = Arc::new(fixtures);
    let bindings = Arc::new(bindings);
    (fixtures, bindings, config)
}
