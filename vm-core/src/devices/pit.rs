pub struct PIT {
    channels: [PITChannel; 3],
    counter: u64,
}

struct PITChannel {
    count: u16,
    reload: u16,
    mode: u8,
    rw_mode: u8,
    bcd: bool,
    gate: bool,
    output: bool,
    toggle: bool,
    read_latch: u16,
    write_latch: u16,
}

impl PITChannel {
    fn new() -> Self {
        PITChannel {
            count: 0,
            reload: 0xFFFF,
            mode: 0,
            rw_mode: 0,
            bcd: false,
            gate: true,
            output: false,
            toggle: false,
            read_latch: 0,
            write_latch: 0,
        }
    }
}

impl PIT {
    pub fn new() -> Self {
        PIT {
            channels: [PITChannel::new(), PITChannel::new(), PITChannel::new()],
            counter: 0,
        }
    }

    pub fn io_read(&mut self, port: u16) -> u8 {
        let channel_idx = (port - 0x40) as usize;
        if channel_idx > 2 {
            return 0;
        }
        
        let ch = &mut self.channels[channel_idx];
        if ch.toggle {
            ch.toggle = false;
            (ch.read_latch >> 8) as u8
        } else {
            ch.toggle = true;
            ch.read_latch as u8
        }
    }

    pub fn io_write(&mut self, port: u16, value: u8) {
        if port == 0x43 {
            // Control word
            let channel = ((value >> 6) & 0x03) as usize;
            if channel > 2 {
                return;
            }
            let ch = &mut self.channels[channel];
            ch.rw_mode = (value >> 4) & 0x03;
            ch.mode = (value >> 1) & 0x07;
            ch.bcd = (value & 0x01) != 0;
            ch.toggle = false;
            
            if ch.rw_mode == 0 {
                // Latch count
                ch.read_latch = ch.count;
            }
        } else {
            let channel_idx = (port - 0x40) as usize;
            if channel_idx > 2 {
                return;
            }
            let ch = &mut self.channels[channel_idx];
            match ch.rw_mode {
                1 => { // LSB only
                    ch.reload = (ch.reload & 0xFF00) | value as u16;
                    ch.count = ch.reload;
                }
                2 => { // MSB only
                    ch.reload = (ch.reload & 0x00FF) | ((value as u16) << 8);
                    ch.count = ch.reload;
                }
                3 => { // LSB then MSB
                    if !ch.toggle {
                        ch.write_latch = (ch.write_latch & 0xFF00) | value as u16;
                        ch.toggle = true;
                    } else {
                        ch.write_latch = (ch.write_latch & 0x00FF) | ((value as u16) << 8);
                        ch.reload = ch.write_latch;
                        ch.count = ch.reload;
                        ch.toggle = false;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) {
        self.counter += 1;
        
        for ch in &mut self.channels {
            if ch.gate && ch.count > 0 {
                ch.count -= 1;
                if ch.count == 0 {
                    ch.output = !ch.output;
                    ch.count = ch.reload;
                }
            }
        }
    }
}
