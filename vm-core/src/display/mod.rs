pub struct DisplayManager {
    pub width: usize,
    pub height: usize,
    pub framebuffer: Vec<u8>,
}

impl DisplayManager {
    pub fn new(width: usize, height: usize) -> Self {
        DisplayManager {
            width,
            height,
            framebuffer: vec![0u8; width * height * 4],
        }
    }

    pub fn get_framebuffer(&self) -> Vec<u8> {
        self.framebuffer.clone()
    }

    pub fn update_from_vga(&mut self, vga_fb: &[u8]) {
        let len = self.framebuffer.len().min(vga_fb.len());
        self.framebuffer[..len].copy_from_slice(&vga_fb[..len]);
    }
}
