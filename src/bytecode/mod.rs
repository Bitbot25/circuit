use std::fmt::Debug;

pub mod op;

pub struct ByteStream {
    bytes: Vec<u8>,
}

impl ByteStream {
    pub fn new() -> ByteStream {
        ByteStream { bytes: Vec::new() }
    }

    pub fn emit(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
}

impl Iterator for ByteStream {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        let (val, remain) = self.bytes.split_at(1);
        let val = val[0];
        self.bytes = remain.to_vec();
        Some(val)
    }
}

impl Debug for ByteStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ByteStream ")?;
        f.debug_list()
            .entries(self.bytes.iter())
            .finish()
    }
}

// TODO: Make a Display implementation that shows the actual instruction names