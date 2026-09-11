pub struct VGADevice {
    pub framebuffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub mode: VGAMode,
    pub dac_palette: [[u8; 3]; 256],
    pub current_dac_index: u8,
    pub dac_read_write: u8, // 0=read, 1=write
    pub dac_color_component: u8,
    pub misc_output: u8,
    pub sequencer_index: u8,
    pub sequencer_regs: [u8; 8],
    pub crtc_index: u8,
    pub crtc_regs: [u8; 25],
    pub graphics_index: u8,
    pub graphics_regs: [u8; 9],
    pub attr_index: u8,
    pub attr_regs: [u8; 21],
    pub attr_flip_flop: bool,
    pub text_cursor_pos: u16,
    pub text_page: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VGAMode {
    Text80x25,
    Graphics320x200,
    Graphics640x480,
    Graphics800x600,
    Graphics1024x768,
}

impl VGADevice {
    pub fn new() -> Self {
        let width = 640;
        let height = 480;
        let mut dac_palette = [[0u8; 3]; 256];
        
        // Initialize default VGA text mode palette
        for i in 0..16 {
            dac_palette[i] = vga_default_palette(i);
        }
        
        VGADevice {
            framebuffer: vec![0u8; width * height * 4],
            width,
            height,
            mode: VGAMode::Text80x25,
            dac_palette,
            current_dac_index: 0,
            dac_read_write: 0,
            dac_color_component: 0,
            misc_output: 0,
            sequencer_index: 0,
            sequencer_regs: [0; 8],
            crtc_index: 0,
            crtc_regs: [0; 25],
            graphics_index: 0,
            graphics_regs: [0; 9],
            attr_index: 0,
            attr_regs: [0; 21],
            attr_flip_flop: false,
            text_cursor_pos: 0,
            text_page: 0,
        }
    }

    pub fn io_read(&mut self, port: u16) -> u8 {
        match port {
            0x3C7 => self.dac_read_write, // DAC state
            0x3C8 => self.current_dac_index,
            0x3C9 => {
                let color = self.dac_palette[self.current_dac_index as usize][self.dac_color_component as usize];
                self.dac_color_component += 1;
                if self.dac_color_component >= 3 {
                    self.dac_color_component = 0;
                    self.current_dac_index = self.current_dac_index.wrapping_add(1);
                }
                color
            }
            0x3CC => self.misc_output,
            0x3CA => 0, // Feature control read
            0x3D4 => self.crtc_index,
            0x3D5 => {
                if (self.crtc_index as usize) < self.crtc_regs.len() {
                    self.crtc_regs[self.crtc_index as usize]
                } else {
                    0
                }
            }
            0x3DA => {
                // Status register 1 - always return VBlank=0, retrace=0
                0
            }
            _ => 0,
        }
    }

    pub fn io_write(&mut self, port: u16, value: u8) {
        match port {
            0x3C0 => {
                if !self.attr_flip_flop {
                    self.attr_index = value & 0x1F;
                } else {
                    if (self.attr_index as usize) < self.attr_regs.len() {
                        self.attr_regs[self.attr_index as usize] = value;
                    }
                }
                self.attr_flip_flop = !self.attr_flip_flop;
            }
            0x3C1 => {
                if (self.attr_index as usize) < self.attr_regs.len() {
                    self.attr_regs[self.attr_index as usize] = value;
                }
            }
            0x3C2 => {
                self.misc_output = value;
            }
            0x3C4 => {
                self.sequencer_index = value;
            }
            0x3C5 => {
                if (self.sequencer_index as usize) < self.sequencer_regs.len() {
                    self.sequencer_regs[self.sequencer_index as usize] = value;
                }
            }
            0x3C7 => {
                self.current_dac_index = value;
                self.dac_read_write = 0;
                self.dac_color_component = 0;
            }
            0x3C8 => {
                self.current_dac_index = value;
                self.dac_read_write = 1;
                self.dac_color_component = 0;
            }
            0x3C9 => {
                self.dac_palette[self.current_dac_index as usize][self.dac_color_component as usize] = value;
                self.dac_color_component += 1;
                if self.dac_color_component >= 3 {
                    self.dac_color_component = 0;
                    self.current_dac_index = self.current_dac_index.wrapping_add(1);
                }
            }
            0x3CE => {
                self.graphics_index = value;
            }
            0x3CF => {
                if (self.graphics_index as usize) < self.graphics_regs.len() {
                    self.graphics_regs[self.graphics_index as usize] = value;
                }
            }
            0x3D4 => {
                self.crtc_index = value;
            }
            0x3D5 => {
                if (self.crtc_index as usize) < self.crtc_regs.len() {
                    self.crtc_regs[self.crtc_index as usize] = value;
                }
                // Cursor position
                if self.crtc_index == 0x0E {
                    self.text_cursor_pos = (self.text_cursor_pos & 0x00FF) | ((value as u16) << 8);
                } else if self.crtc_index == 0x0F {
                    self.text_cursor_pos = (self.text_cursor_pos & 0xFF00) | value as u16;
                }
            }
            _ => {}
        }
    }

    pub fn get_framebuffer(&self) -> Vec<u8> {
        self.framebuffer.clone()
    }

    pub fn update_text_mode(&mut self, memory: &[u8]) {
        let base_addr = 0xB8000;
        let cols = 80;
        let rows = 25;
        
        for row in 0..rows {
            for col in 0..cols {
                let offset = base_addr + (row * cols + col) * 2;
                let char_byte = if offset < memory.len() { memory[offset] } else { 0 };
                let attr_byte = if offset + 1 < memory.len() { memory[offset + 1] } else { 0 };
                
                let fg = attr_byte & 0x0F;
                let bg = (attr_byte >> 4) & 0x0F;
                
                let fg_color = self.dac_palette[fg as usize];
                let bg_color = self.dac_palette[bg as usize];
                
                // Simple character rendering - just fill with colors
                let pixel_x = col * 8;
                let pixel_y = row * 16;
                
                for py in 0..16 {
                    for px in 0..8 {
                        let fb_idx = ((pixel_y + py) * self.width + (pixel_x + px)) * 4;
                        if fb_idx + 3 < self.framebuffer.len() {
                            // Simple font rendering for visible characters
                            let is_set = char_byte != 0 && self.is_pixel_set(char_byte, px, py);
                            if is_set {
                                self.framebuffer[fb_idx] = fg_color[0] * 4;
                                self.framebuffer[fb_idx + 1] = fg_color[1] * 4;
                                self.framebuffer[fb_idx + 2] = fg_color[2] * 4;
                            } else {
                                self.framebuffer[fb_idx] = bg_color[0] * 4;
                                self.framebuffer[fb_idx + 1] = bg_color[1] * 4;
                                self.framebuffer[fb_idx + 2] = bg_color[2] * 4;
                            }
                            self.framebuffer[fb_idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    fn is_pixel_set(&self, _char_byte: u8, _px: usize, _py: usize) -> bool {
        // Simplified - in a real implementation this would use a font bitmap
        false
    }

    pub fn update_graphics_mode(&mut self, memory: &[u8]) {
        // Mode 13h: 320x200, 256 colors
        let base_addr = 0xA0000;
        for y in 0..200usize {
            for x in 0..320usize {
                let offset = base_addr + y * 320 + x;
                let color_idx = if offset < memory.len() { memory[offset] } else { 0 };
                let color = self.dac_palette[color_idx as usize];
                
                // Scale to framebuffer
                let scale_x = self.width / 320;
                let scale_y = self.height / 200;
                
                for sy in 0..scale_y {
                    for sx in 0..scale_x {
                        let fb_x = x * scale_x + sx;
                        let fb_y = y * scale_y + sy;
                        let fb_idx = (fb_y * self.width + fb_x) * 4;
                        if fb_idx + 3 < self.framebuffer.len() {
                            self.framebuffer[fb_idx] = color[0] * 4;
                            self.framebuffer[fb_idx + 1] = color[1] * 4;
                            self.framebuffer[fb_idx + 2] = color[2] * 4;
                            self.framebuffer[fb_idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }
}

fn vga_default_palette(index: u8) -> [u8; 3] {
    match index {
        0 => [0, 0, 0],       // Black
        1 => [0, 0, 42],      // Blue
        2 => [0, 42, 0],      // Green
        3 => [0, 42, 42],     // Cyan
        4 => [42, 0, 0],      // Red
        5 => [42, 0, 42],     // Magenta
        6 => [42, 21, 0],     // Brown
        7 => [42, 42, 42],    // Light Gray
        8 => [21, 21, 21],    // Dark Gray
        9 => [21, 21, 63],    // Light Blue
        10 => [21, 63, 21],   // Light Green
        11 => [21, 63, 63],   // Light Cyan
        12 => [63, 21, 21],   // Light Red
        13 => [63, 21, 63],   // Light Magenta
        14 => [63, 63, 21],   // Yellow
        15 => [63, 63, 63],   // White
        _ => [0, 0, 0],
    }
}
