pub const ARTNET_NAME: &[u8; 8] = b"Art-Net\0";
pub const ARTNET_VERSION: u8 = 14;
pub const ARTNET_OPCODE: u8 = 80;
pub const ARTNET_HEADER_SIZE: usize = 18;

/// see also https://art-net.org.uk/downloads/art-net.pdf
pub fn build_artnet_package(universe: &u16, data: &[u8]) -> Vec<u8> {
    let mut data_length = data.len();

    assert!(data_length <= 512, "data length is too long");

    data_length += data_length % 2;

    let h_uni = (universe >> 8) as u8;
    let l_uni = (universe & 0xff) as u8;
    let h_len = (data_length >> 8) as u8;
    let l_len = (data_length & 0xff) as u8;

    // allocate the size of the package in advance to avoid reallocations
    let mut package = Vec::with_capacity(ARTNET_HEADER_SIZE + data_length);

    // add the headers
    package.extend_from_slice(ARTNET_NAME);
    package.extend_from_slice(&[0, ARTNET_OPCODE]);
    package.extend_from_slice(&[0, ARTNET_VERSION]);
    package.extend_from_slice(&[0, 0]);
    package.extend_from_slice(&[l_uni, h_uni, h_len, l_len]);
    // add the data
    package.extend_from_slice(&data[..data_length.min(data.len())]);

    package
}
