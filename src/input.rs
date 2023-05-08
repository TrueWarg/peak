use std::{
    io::{BufRead, Read},
    thread,
    time::Duration,
};

pub struct DefferedInput<'a> {
    pub input: &'a [u8],
    pub delay_secs: u64,
}

impl<'a> Read for DefferedInput<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.input.read(buf)
    }
}

impl<'a> BufRead for DefferedInput<'a> {
    #[inline]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(self.input)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        thread::sleep(Duration::from_secs(self.delay_secs));
        self.input = &self.input[amt..];
    }
}
