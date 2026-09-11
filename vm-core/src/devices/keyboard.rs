pub struct KeyboardDevice {
    buffer: [u8; 32],
    read_pos: usize,
    write_pos: usize,
    count: usize,
    status: u8,
}

impl KeyboardDevice {
    pub fn new() -> Self {
        KeyboardDevice {
            buffer: [0; 32],
            read_pos: 0,
            write_pos: 0,
            count: 0,
            status: 0x10, // System flag
        }
    }

    pub fn key_down(&mut self, scancode: u8) {
        if self.count < 32 {
            self.buffer[self.write_pos] = scancode;
            self.write_pos = (self.write_pos + 1) % 32;
            self.count += 1;
            self.status |= 0x01; // Output buffer full
        }
    }

    pub fn key_up(&mut self, scancode: u8) {
        if self.count < 32 {
            self.buffer[self.write_pos] = scancode | 0x80; // Break code
            self.write_pos = (self.write_pos + 1) % 32;
            self.count += 1;
            self.status |= 0x01;
        }
    }

    pub fn read_data(&mut self) -> u8 {
        if self.count > 0 {
            let value = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % 32;
            self.count -= 1;
            if self.count == 0 {
                self.status &= !0x01;
            }
            value
        } else {
            0
        }
    }

    pub fn read_status(&self) -> u8 {
        self.status
    }

    pub fn write_data(&mut self, _value: u8) {
        // Handle keyboard controller commands
    }

    pub fn write_command(&mut self, _value: u8) {
        // Handle keyboard controller commands
    }

    pub fn has_data(&self) -> bool {
        self.count > 0
    }
}
