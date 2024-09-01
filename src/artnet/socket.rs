use crate::artnet::package::build_artnet_package;
use std::net::UdpSocket;

/// Creates an udp socket to send artnet packages
/// # Example
/// ```rust
/// use artnet::socket::create_socket;
/// 
/// let send_artnet = create_socket("0.0.0.0:6454".to_string(), "255.255.255.255:6454".to_string(), true).unwrap();
/// send_artnet(&0, &[255; 512], None).unwrap(); // send to the default address
/// send_artnet(&0, &[255; 512], Some("127.0.0.1:6454")).unwrap(); // send to a specific address
/// ```
/// # Arguments
/// * `bind_addr` - the address to bind the socket
/// * `default_addr` - the default address to send the packages (can be overwritten by the `addr` parameter of the returned closure)
/// * `broadcast` - if the socket should be able to send broadcast packages
/// # Returns
/// a function to send the artnet packages
pub fn create_socket(
    bind_addr: String,
    default_addr: String,
    broadcast: bool,
) -> std::io::Result<impl Fn(&u16, &[u8], Option<&str>) -> std::io::Result<()>> {
    let socket = UdpSocket::bind(bind_addr)?;
    socket.set_broadcast(broadcast)?;

    let send_artnet =
        move |universe: &u16, data: &[u8], addr: Option<&str>| -> std::io::Result<()> {
            let artnet_packet = build_artnet_package(universe, data);
            let addr = addr.unwrap_or(&default_addr);
            socket.send_to(&artnet_packet, addr)?;
            Ok(())
        };

    Ok(send_artnet)
}
