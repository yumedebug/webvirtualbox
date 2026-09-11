pub struct Memory {
    data: Vec<u8>,
    size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0u8; size],
            size,
        }
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        if addr < self.size {
            self.data[addr]
        } else {
            0xFF
        }
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) {
        if addr < self.size {
            self.data[addr] = value;
        }
    }

    pub fn read_word(&self, addr: usize) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn write_word(&mut self, addr: usize, value: u16) {
        self.write_byte(addr, value as u8);
        self.write_byte(addr + 1, (value >> 8) as u8);
    }

    pub fn read_dword(&self, addr: usize) -> u32 {
        let lo = self.read_word(addr) as u32;
        let hi = self.read_word(addr + 2) as u32;
        (hi << 16) | lo
    }

    pub fn write_dword(&mut self, addr: usize, value: u32) {
        self.write_word(addr, value as u16);
        self.write_word(addr + 2, (value >> 16) as u16);
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
