use artnet::{create_socket, ArtNetClient};
use logger::{debug, warning};
use std::{borrow::BorrowMut, net::TcpListener};
use tungstenite::accept;
use wshandlers::{handle_websocket_message, WebsocketHandlingError};

mod wshandlers;

fn main() {
    let tcp_server = TcpListener::bind("0.0.0.0:3000").unwrap();

    let artnet_socket = create_socket(
        "0.0.0.0:6454".to_string(),
        "255.255.255.255:6454".to_string(),
        true,
    )
    .expect("Error creating socket");
    let mut artnet_client = ArtNetClient::new(&artnet_socket, 0);

    // one connection at a time because there would be conflicts in the ArtNet data
    for tcp_stream in tcp_server.incoming() {
        let mut websocket = accept(tcp_stream.unwrap()).unwrap();

        loop {
            let msg = websocket.read().expect("failed to read socket message");

            // Handle control frames (ping, pong, close)
            if msg.is_empty() || msg.is_close() || msg.is_ping() || msg.is_pong() {
                continue;
            }

            let ws_msg_str = msg.to_text().expect("failed to convert message to string");

            debug(&format!("websocket message recieved: {}", ws_msg_str));

            let handling_result = handle_websocket_message(ws_msg_str, artnet_client.borrow_mut());

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
    }
}
