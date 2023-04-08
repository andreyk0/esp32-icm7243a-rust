pub const NUM_CHARS: usize = 8;

pub struct Circ {
    chars: Vec<u8>,
    offset: usize,
    addr: usize,
}

impl Circ {
    pub fn new(chars: Vec<u8>) -> Self {
        Circ {
            chars,
            offset: 0,
            addr: 0,
        }
    }
}

impl Iterator for Circ {
    type Item = (usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.chars.is_empty() {
            None
        } else if self.chars.len() <= NUM_CHARS {
            let a = self.addr;
            let c = self.chars.get(self.addr).map(|c| *c).unwrap_or(' ' as u8);
            self.addr += 1;
            if self.addr > NUM_CHARS {
                None
            } else {
                Some((a, c))
            }
        } else {
            let a = self.addr;
            let c = self
                .chars
                .get((self.offset + self.addr) % self.chars.len())
                .map(|c| *c)
                .unwrap_or(' ' as u8);

            self.addr += 1;
            if self.addr >= NUM_CHARS {
                self.addr = 0;
                self.offset += 1;
                if self.offset == self.chars.len() {
                    self.offset = 0;
                }
            }

            Some((a, c))
        }
    }
}
