use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

mod wshandlers;

fn main() {
    let tcp_server = TcpListener::bind("0.0.0.0:3000").unwrap();
    for tcp_stream in tcp_server.incoming() {
        spawn(move || {
            let mut websocket = accept(tcp_stream.unwrap()).expect("Error during handshake");
            loop {
                let msg = websocket.read().expect("Error reading message");
                
                let data = msg.into_text().expect("Error converting message to text");

                println!("Received: {}", data);

                websocket.send("0".into()).unwrap();
            }
        });
    }
}
