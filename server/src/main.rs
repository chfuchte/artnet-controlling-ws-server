use artnet::{create_socket, ArtNetClient};
use config::parse_yaml_into;
use logger::{debug, warning};
use std::{fs::read_to_string, net::TcpListener, sync::Arc, thread};
use tungstenite::accept;
use wshandlers::{handle_websocket_message, WebsocketHandlingError};

#[cfg(test)]
mod test;

mod config;
mod wshandlers;

fn main() {
    let config_file_path = std::env::args()
        .nth(1)
        .expect("no config file path provided");
    assert!(
        config_file_path.ends_with(".yaml"),
        "config file must be a yaml file"
    );
    let config_file_content_str =
        read_to_string(&config_file_path).expect("failed to read config file");
    let (fixtures, bindings) =
        parse_yaml_into(&config_file_content_str).expect("invalid config file");

    let fixtures = Arc::new(fixtures);
    let bindings = Arc::new(bindings);

    let tcp_server = TcpListener::bind("0.0.0.0:3000").unwrap();
    let send_via_artnet_udp_socket = Arc::new(
        create_socket(
            "0.0.0.0:6454".to_string(),
            "255.255.255.255:6454".to_string(),
            true,
        )
        .expect("Error creating socket"),
    );
    let artnet_client = Arc::new(ArtNetClient::new(send_via_artnet_udp_socket, 0));

    let artnet_client_artnet_client_commit_regulary_clone = Arc::clone(&artnet_client);
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(50));
        artnet_client_artnet_client_commit_regulary_clone
            .commit()
            .expect("failed to commit artnet data");
    });

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

                debug(&format!("websocket message recieved: {}", ws_msg_str));

                let handling_result = handle_websocket_message(
                    ws_msg_str,
                    artnet_client_current_thread_clone.clone(),
                    fixtures_current_thread_clone.clone(),
                    bindings_current_thread_clone.clone(),
                );

                if handling_result.is_err() {
                    match handling_result.err().unwrap() {
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
                    }
                } else {
                    websocket
                        .send("OK".into())
                        .expect("failed to send a response to the client");
                }
            }
        });
    }
}
