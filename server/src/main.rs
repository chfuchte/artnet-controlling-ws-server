use artnet::{create_socket, ArtNetClient};
use logger::{debug, warning};
use std::{net::TcpListener, sync::Arc, thread};
use tungstenite::accept;
use wshandlers::{handle_websocket_message, WebsocketHandlingError};

mod wshandlers;

fn main() {
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
