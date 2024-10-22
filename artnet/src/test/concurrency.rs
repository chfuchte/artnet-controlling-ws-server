use crate::ArtNetClient;
use std::{sync::Arc, thread};

fn get_test_client_arc() -> Arc<ArtNetClient> {
    Arc::new(ArtNetClient::new(Arc::new(|_, _, _| Ok(())), 0))
}

#[test]
fn simultaneous_writes() {
    let client = get_test_client_arc();
    let handle1 = {
        let client1 = Arc::clone(&client);
        thread::spawn(move || {
            client1.set_single(0, 128);
        })
    };

    let handle2 = {
        let client2 = Arc::clone(&client);
        thread::spawn(move || {
            client2.set_single(0, 255);
        })
    };

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Check the final value at index 0
    let data = client.get_data();
    assert!(data[0] == 128 || data[0] == 255);
}

#[test]
fn concurrent_reads_and_writes() {
    let client = get_test_client_arc();
    let handle1 = {
        let client1 = Arc::clone(&client);
        thread::spawn(move || {
            client1.set_single(0, 255);
            client1.set_single(1, 128);
        })
    };

    let handle2 = {
        let client2 = Arc::clone(&client);
        thread::spawn(move || {
            let data = client2.get_data();
            assert!(data[0] == 0 || data[0] == 255);
            assert!(data[1] == 0 || data[1] == 128);
        })
    };

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn multiple_threads_modifying_different_indices() {
    let client = get_test_client_arc();

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let client_clone = Arc::clone(&client);
            thread::spawn(move || {
                client_clone.set_single(i, (i + 1) as u8);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let data = client.get_data();
    for i in 0..10 {
        assert_eq!(data[i], (i + 1) as u8);
    }
}
