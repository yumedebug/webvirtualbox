pub struct PIC {
    // Master PIC
    master_icw: [u8; 4],
    master_ocw: [u8; 2],
    master_imr: u8,
    master_isr: u8,
    master_irr: u8,
    master_vector_base: u8,
    master_init_state: u8,
    
    // Slave PIC
    slave_icw: [u8; 4],
    slave_ocw: [u8; 2],
    slave_imr: u8,
    slave_isr: u8,
    slave_irr: u8,
    slave_vector_base: u8,
    slave_init_state: u8,
}

impl PIC {
    pub fn new() -> Self {
        PIC {
            master_icw: [0; 4],
            master_ocw: [0; 2],
            master_imr: 0xFF,
            master_isr: 0,
            master_irr: 0,
            master_vector_base: 0x08,
            master_init_state: 0,
            
            slave_icw: [0; 4],
            slave_ocw: [0; 2],
            slave_imr: 0xFF,
            slave_isr: 0,
            slave_irr: 0,
            slave_vector_base: 0x70,
            slave_init_state: 0,
        }
    }

    pub fn io_read(&mut self, port: u16) -> u8 {
        match port {
            0x20 => {
                if self.master_ocw[0] & 0x08 != 0 {
                    self.master_isr
                } else {
                    self.master_irr
                }
            }
            0x21 => self.master_imr,
            0xA0 => {
                if self.slave_ocw[0] & 0x08 != 0 {
                    self.slave_isr
                } else {
                    self.slave_irr
                }
            }
            0xA1 => self.slave_imr,
            _ => 0,
        }
    }

    pub fn io_write(&mut self, port: u16, value: u8) {
        match port {
            0x20 => {
                if value & 0x10 != 0 {
                    // ICW1
                    self.master_icw[0] = value;
                    self.master_init_state = 1;
                } else if value == 0x20 {
                    // EOI
                    let highest = self.get_highest_isr(true);
                    if highest != 0xFF {
                        self.master_isr &= !(1 << highest);
                    }
                } else {
                    self.master_ocw[0] = value;
                }
            }
            0x21 => {
                if self.master_init_state == 1 {
                    // ICW2
                    self.master_vector_base = value & 0xF8;
                    self.master_init_state = 2;
                } else if self.master_init_state == 2 {
                    // ICW3
                    self.master_icw[2] = value;
                    self.master_init_state = 3;
                } else if self.master_init_state == 3 {
                    // ICW4
                    self.master_icw[3] = value;
                    self.master_init_state = 0;
                    self.master_imr = 0;
                } else {
                    // OCW1 - IMR
                    self.master_imr = value;
                }
            }
            0xA0 => {
                if value & 0x10 != 0 {
                    self.slave_icw[0] = value;
                    self.slave_init_state = 1;
                } else if value == 0x20 {
                    let highest = self.get_highest_isr(false);
                    if highest != 0xFF {
                        self.slave_isr &= !(1 << highest);
                    }
                } else {
                    self.slave_ocw[0] = value;
                }
            }
            0xA1 => {
                if self.slave_init_state == 1 {
                    self.slave_vector_base = value & 0xF8;
                    self.slave_init_state = 2;
                } else if self.slave_init_state == 2 {
                    self.slave_icw[2] = value;
                    self.slave_init_state = 3;
                } else if self.slave_init_state == 3 {
                    self.slave_icw[3] = value;
                    self.slave_init_state = 0;
                    self.slave_imr = 0;
                } else {
                    self.slave_imr = value;
                }
            }
            _ => {}
        }
    }

    pub fn raise_irq(&mut self, irq: u8) {
        if irq < 8 {
            self.master_irr |= 1 << irq;
        } else if irq < 16 {
            self.slave_irr |= 1 << (irq - 8);
            self.master_irr |= 1 << 2; // IRQ2 cascades slave
        }
    }

    pub fn get_interrupt(&mut self) -> Option<u8> {
        // Check slave first if IRQ2 is active
        if self.master_irr & self.master_isr == 0 && (self.master_irr & 0x04) != 0 {
            let slave_pending = self.slave_irr & !self.slave_imr & !self.slave_isr;
            if slave_pending != 0 {
                for i in 0..8 {
                    if slave_pending & (1 << i) != 0 {
                        self.slave_isr |= 1 << i;
                        self.slave_irr &= !(1 << i);
                        return Some(self.slave_vector_base + i);
                    }
                }
            }
        }
        
        let master_pending = self.master_irr & !self.master_imr & !self.master_isr;
        if master_pending != 0 {
            for i in 0..8 {
                if master_pending & (1 << i) != 0 && i != 2 {
                    self.master_isr |= 1 << i;
                    self.master_irr &= !(1 << i);
                    return Some(self.master_vector_base + i);
                }
            }
        }
        
        None
    }

    fn get_highest_isr(&self, master: bool) -> u8 {
        let isr = if master { self.master_isr } else { self.slave_isr };
        for i in (0..8).rev() {
            if isr & (1 << i) != 0 {
                return i;
            }
        }
        0xFF
    }
}
