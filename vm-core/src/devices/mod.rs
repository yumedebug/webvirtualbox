pub mod vga;
pub mod keyboard;
pub mod pic;
pub mod pit;

use vga::VGADevice;
use keyboard::KeyboardDevice;
use pic::PIC;
use pit::PIT;

pub struct DeviceManager {
    pub vga: VGADevice,
    pub keyboard: KeyboardDevice,
    pub pic: PIC,
    pub pit: PIT,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            vga: VGADevice::new(),
            keyboard: KeyboardDevice::new(),
            pic: PIC::new(),
            pit: PIT::new(),
        }
    }

    pub fn io_read(&mut self, port: u16) -> u8 {
        match port {
            // VGA registers
            0x3C0..=0x3DF => self.vga.io_read(port),
            
            // PIC
            0x20..=0x21 => self.pic.io_read(port),
            0xA0..=0xA1 => self.pic.io_read(port),
            
            // PIT
            0x40..=0x43 => self.pit.io_read(port),
            
            // Keyboard
            0x60 => self.keyboard.read_data(),
            0x64 => self.keyboard.read_status(),
            
            // CMOS/RTC
            0x70..=0x71 => 0,
            
            // Serial ports
            0x3F8..=0x3FF => 0,
            
            _ => 0xFF,
        }
    }

    pub fn io_write(&mut self, port: u16, value: u8) {
        match port {
            // VGA registers
            0x3C0..=0x3DF => self.vga.io_write(port, value),
            
            // PIC
            0x20..=0x21 => self.pic.io_write(port, value),
            0xA0..=0xA1 => self.pic.io_write(port, value),
            
            // PIT
            0x40..=0x43 => self.pit.io_write(port, value),
            
            // Keyboard
            0x60 => self.keyboard.write_data(value),
            0x64 => self.keyboard.write_command(value),
            
            // CMOS/RTC
            0x70 => {} // CMOS address
            0x71 => {} // CMOS data
            
            // Speaker
            0x61 => {}
            
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        self.pit.tick();
    }
}
