use std::sync::{Arc, Mutex};

/// A thread-safe ArtNet client to handle the ArtNet data
pub struct ArtNetClient {
    send_fn: Arc<dyn Fn(&u16, &[u8], Option<&str>) -> std::io::Result<()>>,
    data: Arc<Mutex<[u8; 512]>>,
    universe: u16,
}

impl ArtNetClient {
    pub fn new(
        send_fn: Arc<dyn Fn(&u16, &[u8], Option<&str>) -> std::io::Result<()>>,
        universe: u16,
    ) -> Self {
        Self {
            send_fn,
            data: Arc::new(Mutex::new([0; 512])),
            universe,
        }
    }

    pub fn set_data(&self, data: [u8; 512]) {
        let mut data_guard = self.data.lock().unwrap();
        *data_guard = data;
    }

    pub fn get_data(&self) -> [u8; 512] {
        *self.data.lock().unwrap()
    }

    /// set's a single channel value
    pub fn set_single(&self, channel: u16, value: u8) {
        let channel = channel as usize;
        assert!(channel < 512);
        let mut data_guard = self.data.lock().unwrap();
        data_guard[channel] = value;
    }

    /// set's mutliple channel values
    pub fn set_multiple(&self, start_channel: u16, values: &[u8]) {
        let start_channel = start_channel as usize;
        let end_channel = start_channel + values.len();
        assert!(end_channel <= 512);
        let mut data_guard = self.data.lock().unwrap();
        for (i, value) in values.iter().enumerate() {
            data_guard[start_channel + i] = *value;
        }
    }

    pub fn commit(&self) -> std::io::Result<()> {
        (self.send_fn)(&self.universe, &self.get_data(), None)
    }

    pub fn commit_with_addr(&self, addr: &str) -> std::io::Result<()> {
        (self.send_fn)(&self.universe, &self.get_data(), Some(addr))
    }
}

unsafe impl Sync for ArtNetClient {}
unsafe impl Send for ArtNetClient {}
