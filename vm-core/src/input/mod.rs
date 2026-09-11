pub struct InputManager {
    keyboard_buffer: Vec<u8>,
    mouse_x: i16,
    mouse_y: i16,
    mouse_buttons: u8,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keyboard_buffer: Vec::new(),
            mouse_x: 0,
            mouse_y: 0,
            mouse_buttons: 0,
        }
    }

    pub fn key_down(&mut self, scancode: u8) {
        self.keyboard_buffer.push(scancode);
    }

    pub fn key_up(&mut self, scancode: u8) {
        self.keyboard_buffer.push(scancode | 0x80);
    }

    pub fn mouse_move(&mut self, x: i16, y: i16) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn mouse_button(&mut self, button: u8, pressed: bool) {
        if pressed {
            self.mouse_buttons |= 1 << button;
        } else {
            self.mouse_buttons &= !(1 << button);
        }
    }

    pub fn get_keyboard_scancode(&mut self) -> Option<u8> {
        if !self.keyboard_buffer.is_empty() {
            Some(self.keyboard_buffer.remove(0))
        } else {
            None
        }
    }

    pub fn get_mouse_x(&self) -> i16 {
        self.mouse_x
    }

    pub fn get_mouse_y(&self) -> i16 {
        self.mouse_y
    }

    pub fn get_mouse_buttons(&self) -> u8 {
        self.mouse_buttons
    }
}
