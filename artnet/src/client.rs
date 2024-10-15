pub struct ArtNetClient<'a> {
    send_fn: &'a dyn Fn(&u16, &[u8], Option<&str>) -> std::io::Result<()>,
    data: [u8; 512],
    universe: u16,
}

impl<'a> ArtNetClient<'a> {
    pub fn new(send_fn: &'a dyn Fn(&u16, &[u8], Option<&str>) -> std::io::Result<()>, universe: u16) -> Self {
        Self {
            send_fn,
            data: [0; 512],
            universe,
        }
    }

    pub fn set_data(&mut self, data: [u8; 512]) {
        self.data = data;
    }

    pub fn get_data(&self) -> &[u8; 512] {
        &self.data
    }

    /// set's a single channel value
    pub fn set_single(&mut self, channel: u16, value: u8) {
        let channel = channel as usize;
        assert!(channel < 512);
        self.data[channel] = value;
    }

    /// set's mutliple channel values
    pub fn set_multiple(&mut self, start_channel: u16, values: &[u8]) {
        let start_channel = start_channel as usize;
        let end_channel = start_channel + values.len();
        assert!(end_channel <= 512);
        for (i, value) in values.iter().enumerate() {
            self.data[start_channel + i] = *value;
        }
    }

    pub fn commit(&self) -> std::io::Result<()> {
        (self.send_fn)(&self.universe, &self.data, None)
    }

    pub fn commit_with_addr(&self, addr: &str) -> std::io::Result<()> {
        (self.send_fn)(&self.universe, &self.data, Some(addr))
    }
}