use artnet::ArtNetClient;

// TODO
pub fn handle_websocket_message(_data: &str, client: &mut ArtNetClient<'_>) -> Result<(), std::io::Error> {
    // TODO
    client.set_single(0, 0);
    Ok(())
}
