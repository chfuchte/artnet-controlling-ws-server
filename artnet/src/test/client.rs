use std::sync::Arc;
use crate::ArtNetClient;

fn get_test_client() -> ArtNetClient {
    ArtNetClient::new(Arc::new(|_, _, _| Ok(())), 0)
}

#[test]
fn set_data() {
    let client = get_test_client();
    let data = [1; 512];
    client.set_data(data);
    assert_eq!(client.get_data(), [1; 512]);
}

#[test]
fn set_single() {
    let client = get_test_client();
    client.set_single(0, 1);
    assert_eq!(client.get_data()[0], 1);
}

#[test]
fn set_multiple() {
    let client = get_test_client();
    client.set_multiple(5, &[1, 2, 3]);
    assert_eq!(client.get_data()[5], 1);
    assert_eq!(client.get_data()[6], 2);
    assert_eq!(client.get_data()[7], 3);
}
