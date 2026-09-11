use wasm_bindgen::prelude::*;

pub mod cpu;
pub mod memory;
pub mod devices;
pub mod storage;
pub mod display;
pub mod input;
pub mod machine;

use machine::VirtualMachine;

#[wasm_bindgen]
pub struct WasmVM {
    vm: VirtualMachine,
}

#[wasm_bindgen]
impl WasmVM {
    #[wasm_bindgen(constructor)]
    pub fn new(memory_size: u32) -> WasmVM {
        WasmVM {
            vm: VirtualMachine::new(memory_size as usize),
        }
    }

    pub fn reset(&mut self) {
        self.vm.reset();
    }

    pub fn step(&mut self) -> u32 {
        self.vm.cpu.step(&mut self.vm.memory, &mut self.vm.devices)
    }

    pub fn run_cycles(&mut self, cycles: u32) -> u32 {
        let mut executed = 0u32;
        for _ in 0..cycles {
            if self.vm.cpu.halted {
                break;
            }
            self.vm.cpu.step(&mut self.vm.memory, &mut self.vm.devices);
            executed += 1;
        }
        executed
    }

    pub fn get_framebuffer(&self) -> Vec<u8> {
        self.vm.display.get_framebuffer()
    }

    pub fn get_framebuffer_width(&self) -> u32 {
        self.vm.display.width as u32
    }

    pub fn get_framebuffer_height(&self) -> u32 {
        self.vm.display.height as u32
    }

    pub fn key_down(&mut self, scancode: u8) {
        self.vm.input.key_down(scancode);
    }

    pub fn key_up(&mut self, scancode: u8) {
        self.vm.input.key_up(scancode);
    }

    pub fn mouse_move(&mut self, x: i16, y: i16) {
        self.vm.input.mouse_move(x, y);
    }

    pub fn mouse_button(&mut self, button: u8, pressed: bool) {
        self.vm.input.mouse_button(button, pressed);
    }

    pub fn load_bios(&mut self, data: &[u8]) {
        let bios_start = 0x100000 - data.len(); // Load at top of 1MB
        for (i, &byte) in data.iter().enumerate() {
            self.vm.memory.write_byte(bios_start + i, byte);
        }
        self.vm.cpu.set_ip(0xFFF0_u16); // BIOS entry point
    }

    pub fn load_boot_sector(&mut self, data: &[u8]) {
        // Load to 0x7C00 (standard boot location)
        for (i, &byte) in data.iter().enumerate() {
            if i < 512 {
                self.vm.memory.write_byte(0x7C00 + i, byte);
            }
        }
        self.vm.cpu.set_cs_ip(0x0000, 0x7C00);
    }

    pub fn write_memory(&mut self, addr: u32, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.vm.memory.write_byte((addr as usize) + i, byte);
        }
    }

    pub fn read_memory(&self, addr: u32, length: u32) -> Vec<u8> {
        let mut result = Vec::with_capacity(length as usize);
        for i in 0..length {
            result.push(self.vm.memory.read_byte((addr + i) as usize));
        }
        result
    }

    pub fn load_disk_image(&mut self, data: &[u8]) {
        self.vm.storage.load_disk_image(data.to_vec());
    }

    pub fn load_iso(&mut self, data: &[u8]) {
        self.vm.storage.load_iso(data.to_vec());
    }

    pub fn get_cpu_state(&self) -> String {
        format!(
            "EAX={:08X} EBX={:08X} ECX={:08X} EDX={:08X}\n\
             ESI={:08X} EDI={:08X} EBP={:08X} ESP={:08X}\n\
             EIP={:08X} EFLAGS={:08X}\n\
             CS={:04X} DS={:04X} ES={:04X} SS={:04X}",
            self.vm.cpu.regs.eax,
            self.vm.cpu.regs.ebx,
            self.vm.cpu.regs.ecx,
            self.vm.cpu.regs.edx,
            self.vm.cpu.regs.esi,
            self.vm.cpu.regs.edi,
            self.vm.cpu.regs.ebp,
            self.vm.cpu.regs.esp,
            self.vm.cpu.eip,
            self.vm.cpu.flags.raw(),
            self.vm.cpu.regs.cs,
            self.vm.cpu.regs.ds,
            self.vm.cpu.regs.es,
            self.vm.cpu.regs.ss,
        )
    }

    pub fn is_halted(&self) -> bool {
        self.vm.cpu.halted
    }

    pub fn set_boot_order(&mut self, order: u8) {
        self.vm.boot_order = order;
    }
}
