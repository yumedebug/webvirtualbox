pub mod registers;
pub mod decoder;
pub mod executor;
pub mod flags;
pub mod instructions;

use registers::Registers;
use flags::Flags;

pub struct CPU {
    pub regs: Registers,
    pub eip: u32,
    pub flags: Flags,
    pub halted: bool,
    pub interrupt_flag: bool,
    pub pending_interrupt: Option<u8>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            regs: Registers::new(),
            eip: 0x0000,
            flags: Flags::new(),
            halted: false,
            interrupt_flag: true,
            pending_interrupt: None,
        }
    }

    pub fn reset(&mut self) {
        self.regs = Registers::new();
        self.eip = 0xFFF0;
        self.flags = Flags::new();
        self.halted = false;
        self.interrupt_flag = true;
        self.pending_interrupt = None;
    }

    pub fn set_ip(&mut self, ip: u16) {
        self.eip = ip as u32;
    }

    pub fn set_cs_ip(&mut self, cs: u16, ip: u16) {
        self.regs.cs = cs;
        self.eip = ip as u32;
    }

    pub fn step(&mut self, memory: &mut super::memory::Memory, devices: &mut super::devices::DeviceManager) -> u32 {
        if self.halted {
            return 0;
        }

        // Handle pending interrupts
        if let Some(interrupt) = self.pending_interrupt.take() {
            self.handle_interrupt(interrupt, memory, devices);
            return 1;
        }

        let phys_addr = ((self.regs.cs as u32) << 4) + self.eip;
        let opcode = memory.read_byte(phys_addr as usize);
        
        let (instruction, length) = decoder::decode(memory, phys_addr as usize, opcode);
        
        executor::execute(self, memory, devices, &instruction, length);
        
        1
    }

    pub fn handle_interrupt(&mut self, interrupt: u8, memory: &mut super::memory::Memory, _devices: &mut super::devices::DeviceManager) {
        // Push flags, CS, IP to stack
        let sp = self.regs.esp as usize;
        let ss_base = (self.regs.ss as u32) << 4;
        
        // Push flags
        let flags = self.flags.raw();
        memory.write_byte(ss_base as usize + sp - 1, (flags >> 8) as u8);
        memory.write_byte(ss_base as usize + sp - 2, flags as u8);
        
        // Push CS
        memory.write_byte(ss_base as usize + sp - 3, (self.regs.cs >> 8) as u8);
        memory.write_byte(ss_base as usize + sp - 4, self.regs.cs as u8);
        
        // Push IP
        memory.write_byte(ss_base as usize + sp - 5, (self.eip >> 8) as u8);
        memory.write_byte(ss_base as usize + sp - 6, self.eip as u8);
        
        self.regs.esp -= 6;
        
        // Load IVT entry
        let ivt_addr = (interrupt as u32) * 4;
        let ip = memory.read_word(ivt_addr as usize) as u32;
        let cs = memory.read_word(ivt_addr as usize + 2) as u16;
        
        self.regs.cs = cs;
        self.eip = ip;
        self.interrupt_flag = false;
    }

    pub fn read_byte_operand(&self, memory: &super::memory::Memory, addr: usize) -> u8 {
        memory.read_byte(addr)
    }

    pub fn write_byte_operand(&self, memory: &mut super::memory::Memory, addr: usize, value: u8) {
        memory.write_byte(addr, value);
    }
}
