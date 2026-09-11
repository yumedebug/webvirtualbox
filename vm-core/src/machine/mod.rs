use crate::cpu::CPU;
use crate::memory::Memory;
use crate::devices::DeviceManager;
use crate::storage::StorageManager;
use crate::display::DisplayManager;
use crate::input::InputManager;

pub struct VirtualMachine {
    pub cpu: CPU,
    pub memory: Memory,
    pub devices: DeviceManager,
    pub storage: StorageManager,
    pub display: DisplayManager,
    pub input: InputManager,
    pub boot_order: u8,
    pub running: bool,
}

impl VirtualMachine {
    pub fn new(memory_size: usize) -> Self {
        VirtualMachine {
            cpu: CPU::new(),
            memory: Memory::new(memory_size),
            devices: DeviceManager::new(),
            storage: StorageManager::new(),
            display: DisplayManager::new(640, 480),
            input: InputManager::new(),
            boot_order: 0, // 0 = CD first, 1 = HDD first
            running: false,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.running = false;
    }

    pub fn boot(&mut self) {
        self.setup_bios();
        self.running = true;
    }

    fn setup_bios(&mut self) {
        // Setup minimal BIOS that loads boot sector
        // This is a minimal BIOS implementation that:
        // 1. Sets up IVT
        // 2. Loads boot sector from CD/DVD to 0x7C00
        // 3. Jumps to it
        
        let bios_code = self.generate_minimal_bios();
        let bios_start = 0xF0000; // BIOS ROM area in real mode
        
        for (i, &byte) in bios_code.iter().enumerate() {
            if bios_start + i < self.memory.size() {
                self.memory.write_byte(bios_start + i, byte);
            }
        }
        
        // Set reset vector
        self.cpu.regs.cs = 0xF000;
        self.cpu.eip = 0x0000;
        
        // Setup IVT with BIOS interrupt handlers
        self.setup_ivt();
    }

    fn setup_ivt(&mut self) {
        // Set up interrupt vector table entries for BIOS interrupts
        let bios_base = 0xF0000u32;
        
        // INT 10h - Video services
        let handler_10 = bios_base + 0x1000;
        self.memory.write_word(0x10 * 4, handler_10 as u16);
        self.memory.write_word(0x10 * 4 + 2, 0xF000);
        
        // INT 13h - Disk services
        let handler_13 = bios_base + 0x2000;
        self.memory.write_word(0x13 * 4, handler_13 as u16);
        self.memory.write_word(0x13 * 4 + 2, 0xF000);
        
        // INT 16h - Keyboard services
        let handler_16 = bios_base + 0x3000;
        self.memory.write_word(0x16 * 4, handler_16 as u16);
        self.memory.write_word(0x16 * 4 + 2, 0xF000);
        
        // INT 19h - Bootstrap loader
        let handler_19 = bios_base + 0x4000;
        self.memory.write_word(0x19 * 4, handler_19 as u16);
        self.memory.write_word(0x19 * 4 + 2, 0xF000);
        
        // INT 12h - Memory size
        let handler_12 = bios_base + 0x5000;
        self.memory.write_word(0x12 * 4, handler_12 as u16);
        self.memory.write_word(0x12 * 4 + 2, 0xF000);
        
        // INT 15h - System services
        let handler_15 = bios_base + 0x6000;
        self.memory.write_word(0x15 * 4, handler_15 as u16);
        self.memory.write_word(0x15 * 4 + 2, 0xF000);
    }

    fn generate_minimal_bios(&self) -> Vec<u8> {
        let mut bios = vec![0u8; 0x10000]; // 64KB BIOS area
        
        // BIOS entry point (at offset 0x0000, CS=0xF000, IP=0x0000)
        // This code initializes the system and loads the boot sector
        
        // Entry: Initialize segments and stack
        let mut offset = 0;
        
        // CLI
        bios[offset] = 0xFA; offset += 1;
        
        // MOV AX, 0x0000
        bios[offset] = 0xB8; bios[offset+1] = 0x00; bios[offset+2] = 0x00; offset += 3;
        
        // MOV DS, AX
        bios[offset] = 0x8E; bios[offset+1] = 0xD8; offset += 2;
        
        // MOV ES, AX
        bios[offset] = 0x8E; bios[offset+1] = 0xC0; offset += 2;
        
        // MOV SS, AX
        bios[offset] = 0x8E; bios[offset+1] = 0xD0; offset += 2;
        
        // MOV SP, 0x7C00
        bios[offset] = 0xBC; bios[offset+1] = 0x00; bios[offset+2] = 0x7C; offset += 3;
        
        // STI
        bios[offset] = 0xFB; offset += 1;
        
        // INT 19h - Bootstrap
        bios[offset] = 0xCD; bios[offset+1] = 0x19; offset += 1;
        
        // HLT loop
        bios[offset] = 0xF4; offset += 1;
        // JMP to HLT (infinite loop)
        bios[offset] = 0xEB; bios[offset+1] = 0xFE; offset += 2;
        
        // === INT 10h handler at offset 0x1000 ===
        let int10_offset = 0x1000;
        offset = int10_offset;
        
        // INT 10h - Video services
        // AH=0x00: Set video mode
        // AH=0x0E: Teletype output
        // AH=0x02: Set cursor position
        // AH=0x03: Get cursor position
        // AH=0x06: Scroll up
        // AH=0x0F: Get video mode
        
        // CMP AH, 0x00
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x00; offset += 3;
        // JZ set_video_mode
        bios[offset] = 0x74; bios[offset+1] = 0x30; offset += 2;
        
        // CMP AH, 0x0E
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x0E; offset += 3;
        // JZ teletype_output
        bios[offset] = 0x74; bios[offset+1] = 0x40; offset += 2;
        
        // CMP AH, 0x02
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x02; offset += 3;
        // JZ set_cursor
        bios[offset] = 0x74; bios[offset+1] = 0x50; offset += 2;
        
        // CMP AH, 0x03
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x03; offset += 3;
        // JZ get_cursor
        bios[offset] = 0x74; bios[offset+1] = 0x55; offset += 2;
        
        // CMP AH, 0x06
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x06; offset += 3;
        // JZ scroll_up
        bios[offset] = 0x74; bios[offset+1] = 0x5A; offset += 2;
        
        // CMP AH, 0x0F
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x0F; offset += 3;
        // JZ get_video_mode
        bios[offset] = 0x74; bios[offset+1] = 0x5F; offset += 2;
        
        // Default: IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Set video mode (AH=0x00)
        offset = int10_offset + 0x32;
        // Store mode in BDA at 0x0449
        // MOV BX, 0x0040
        bios[offset] = 0xBB; bios[offset+1] = 0x40; bios[offset+2] = 0x00; offset += 3;
        // MOV ES, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xC3; offset += 2;
        // MOV byte [ES:0x49], AL
        bios[offset] = 0x26; bios[offset] = 0x26; // ES override
        bios[offset] = 0xA2; bios[offset+1] = 0x49; bios[offset+2] = 0x00; offset += 3;
        // IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Teletype output (AH=0x0E) - write char to VGA text buffer
        offset = int10_offset + 0x40 + 0x32;
        // This writes AL to the screen at current cursor position
        // Simplified: write to BDA cursor position
        // PUSH AX
        bios[offset] = 0x50; offset += 1;
        // PUSH BX
        bios[offset] = 0x53; offset += 1;
        // PUSH ES
        bios[offset] = 0x06; offset += 1;
        
        // MOV BX, 0xB800
        bios[offset] = 0xBB; bios[offset+1] = 0x00; bios[offset+2] = 0xB8; offset += 3;
        // MOV ES, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xC3; offset += 2;
        
        // Get cursor position from BDA
        // MOV BX, 0x0040
        bios[offset] = 0xBB; bios[offset+1] = 0x40; bios[offset+2] = 0x00; offset += 3;
        // PUSH DS
        bios[offset] = 0x1E; offset += 1;
        // MOV DS, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xDB; offset += 2;
        // MOV BX, [0x50] (cursor position)
        bios[offset] = 0x8B; bios[offset+1] = 0x1E; bios[offset+2] = 0x50; bios[offset+3] = 0x00; offset += 4;
        // POP DS
        bios[offset] = 0x1F; offset += 1;
        
        // SHL BX, 1 (multiply by 2 for char+attr)
        bios[offset] = 0xD1; bios[offset+1] = 0xE3; offset += 2;
        
        // Store character
        // MOV [ES:BX], AL
        bios[offset] = 0x26; bios[offset] = 0x88; bios[offset+1] = 0x07; offset += 2;
        // MOV byte [ES:BX+1], 0x07 (attribute: white on black)
        bios[offset] = 0x26; bios[offset] = 0xC6; bios[offset+1] = 0x47; bios[offset+2] = 0x01; bios[offset+3] = 0x07; offset += 4;
        
        // Advance cursor
        // INC BX (cursor position in BDA)
        // We need to increment the cursor in BDA
        // For simplicity, just advance by 1
        // MOV BX, 0x0040
        bios[offset] = 0xBB; bios[offset+1] = 0x40; bios[offset+2] = 0x00; offset += 3;
        // PUSH DS
        bios[offset] = 0x1E; offset += 1;
        // MOV DS, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xDB; offset += 2;
        // INC word [0x50]
        bios[offset] = 0xFF; bios[offset+1] = 0x06; bios[offset+2] = 0x50; bios[offset+3] = 0x00; offset += 4;
        // POP DS
        bios[offset] = 0x1F; offset += 1;
        
        // POP ES
        bios[offset] = 0x07; offset += 1;
        // POP BX
        bios[offset] = 0x5B; offset += 1;
        // POP AX
        bios[offset] = 0x58; offset += 1;
        // IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Set cursor position (AH=0x02)
        offset = int10_offset + 0x50 + 0x32;
        // DH = row, DL = col, BH = page
        // Store to BDA at 0x0450
        // MOV BX, 0x0040
        bios[offset] = 0xBB; bios[offset+1] = 0x40; bios[offset+2] = 0x00; offset += 3;
        // MOV ES, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xC3; offset += 2;
        // MOV AL, DH
        bios[offset] = 0x88; bios[offset+1] = 0xF0; offset += 2;
        // MOV AH, 50 (columns)
        bios[offset] = 0xB4; bios[offset+1] = 0x50; offset += 2;
        // MUL AH -> AX = DH * 80
        bios[offset] = 0xF6; bios[offset+1] = 0xE4; offset += 2;
        // ADD AL, DL
        bios[offset] = 0x00; bios[offset+1] = 0xD0; offset += 2;
        // MOV [ES:0x50], AX
        bios[offset] = 0x26; bios[offset] = 0xA3; bios[offset+1] = 0x50; bios[offset+2] = 0x00; offset += 3;
        // IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Get cursor position (AH=0x03)
        offset = int10_offset + 0x55 + 0x32;
        // Return cursor in DX
        // MOV BX, 0x0040
        bios[offset] = 0xBB; bios[offset+1] = 0x40; bios[offset+2] = 0x00; offset += 3;
        // MOV DS, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xDB; offset += 2;
        // MOV AX, [0x50]
        bios[offset] = 0x8B; bios[offset+1] = 0x06; bios[offset+2] = 0x50; bios[offset+3] = 0x00; offset += 4;
        // MOV DX, AX
        bios[offset] = 0x89; bios[offset+1] = 0xC2; offset += 2;
        // MOV AH, 0 (cursor shape)
        bios[offset] = 0xB4; bios[offset+1] = 0x00; offset += 2;
        // MOV BH, 0
        bios[offset] = 0xB7; bios[offset+1] = 0x00; offset += 2;
        // IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Scroll up (AH=0x06) - simplified: just clear
        offset = int10_offset + 0x5A + 0x32;
        // Clear screen by filling B8000 with spaces
        // MOV DI, 0
        bios[offset] = 0xBF; bios[offset+1] = 0x00; bios[offset+2] = 0x00; offset += 3;
        // MOV CX, 2000 (80*25)
        bios[offset] = 0xB9; bios[offset+1] = 0xD0; bios[offset+2] = 0x07; offset += 3;
        // MOV AX, 0x0720 (space with attr)
        bios[offset] = 0xB8; bios[offset+1] = 0x20; bios[offset+2] = 0x07; offset += 3;
        // MOV BX, 0xB800
        bios[offset] = 0xBB; bios[offset+1] = 0x00; bios[offset+2] = 0xB8; offset += 3;
        // MOV ES, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xC3; offset += 2;
        // REP STOSW
        bios[offset] = 0xF3; bios[offset+1] = 0xAB; offset += 2;
        // Reset cursor
        // MOV BX, 0x0040
        bios[offset] = 0xBB; bios[offset+1] = 0x40; bios[offset+2] = 0x00; offset += 3;
        // MOV DS, BX
        bios[offset] = 0x8E; bios[offset+1] = 0xDB; offset += 2;
        // MOV word [0x50], 0
        bios[offset] = 0xC7; bios[offset+1] = 0x06; bios[offset+2] = 0x50; bios[offset+3] = 0x00; bios[offset+4] = 0x00; bios[offset+5] = 0x00; offset += 6;
        // IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Get video mode (AH=0x0F)
        offset = int10_offset + 0x5F + 0x32;
        // Return mode in AL, columns in AH, page in BH
        bios[offset] = 0xB0; bios[offset+1] = 0x03; // Mode 3 (80x25 text)
        offset += 2;
        bios[offset] = 0xB4; bios[offset+1] = 0x50; // 80 columns
        offset += 2;
        bios[offset] = 0xB7; bios[offset+1] = 0x00; // Page 0
        offset += 2;
        bios[offset] = 0xCF; offset += 1; // IRET
        
        // === INT 13h handler at offset 0x2000 ===
        let int13_offset = 0x2000;
        offset = int13_offset;
        
        // INT 13h - Disk services
        // AH=0x00: Reset disk
        // AH=0x02: Read sectors
        // AH=0x08: Get drive parameters
        // AH=0x15: Get disk type
        // AH=0x41: Check extensions present
        // AH=0x42: Extended read
        
        // CMP AH, 0x00
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x00; offset += 3;
        // JZ reset_disk
        bios[offset] = 0x74; bios[offset+1] = 0x08; offset += 2;
        
        // CMP AH, 0x02
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x02; offset += 3;
        // JZ read_sectors
        bios[offset] = 0x74; bios[offset+1] = 0x10; offset += 2;
        
        // CMP AH, 0x08
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x08; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x40; offset += 2;
        
        // CMP AH, 0x15
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x15; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x50; offset += 2;
        
        // CMP AH, 0x41
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x41; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x60; offset += 2;
        
        // CMP AH, 0x42
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x42; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x70; offset += 2;
        
        // Default: error, set AH=1 (invalid command)
        bios[offset] = 0xB4; bios[offset+1] = 0x01; offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Reset disk (AH=0x00)
        offset = int13_offset + 0x0A;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH (success)
        offset += 2;
        bios[offset] = 0xCF; offset += 1; // IRET
        
        // Read sectors (AH=0x02) - simplified
        // AL = sector count, CH = cylinder, CL = sector, DH = head, DL = drive
        // ES:BX = buffer
        offset = int13_offset + 0x12;
        // For now, just return success
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Get drive parameters (AH=0x08)
        offset = int13_offset + 0x42;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xB7; bios[offset+1] = 0x00; // BL = drive type
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Get disk type (AH=0x15)
        offset = int13_offset + 0x52;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Check extensions (AH=0x41)
        offset = int13_offset + 0x62;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xB9; bios[offset+1] = 0x21; bios[offset+2] = 0x41; // BX = 0xAA41
        offset += 3;
        bios[offset] = 0xCF; offset += 1;
        
        // Extended read (AH=0x42)
        offset = int13_offset + 0x72;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // === INT 16h handler at offset 0x3000 ===
        let int16_offset = 0x3000;
        offset = int16_offset;
        
        // INT 16h - Keyboard services
        // AH=0x00: Read key
        // AH=0x01: Check key
        // AH=0x02: Get shift flags
        
        // CMP AH, 0x00
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x00; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x08; offset += 2;
        
        // CMP AH, 0x01
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x01; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x10; offset += 2;
        
        // CMP AH, 0x02
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x02; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x18; offset += 2;
        
        // Default
        bios[offset] = 0xCF; offset += 1;
        
        // Read key (AH=0x00) - wait for key from port 0x60
        offset = int16_offset + 0x0A;
        // IN AL, 0x64 (check status)
        bios[offset] = 0xE4; bios[offset+1] = 0x64; offset += 2;
        // TEST AL, 0x01
        bios[offset] = 0xA8; bios[offset+1] = 0x01; offset += 2;
        // JZ wait
        bios[offset] = 0x74; bios[offset+1] = 0xFA; offset += 2; // Jump back
        // IN AL, 0x60
        bios[offset] = 0xE4; bios[offset+1] = 0x60; offset += 2;
        // MOV AH, AL
        bios[offset] = 0x88; bios[offset+1] = 0xC4; offset += 2;
        // XOR AL, AL (no ASCII for now)
        bios[offset] = 0x32; bios[offset+1] = 0xC0; offset += 2;
        // IRET
        bios[offset] = 0xCF; offset += 1;
        
        // Check key (AH=0x01)
        offset = int16_offset + 0x12;
        bios[offset] = 0xE4; bios[offset+1] = 0x64; offset += 2;
        bios[offset] = 0xA8; bios[offset+1] = 0x01; offset += 2;
        bios[offset] = 0x74; bios[offset+1] = 0x05; offset += 2; // JZ no_key
        bios[offset] = 0x9C; offset += 1; // PUSHF
        bios[offset] = 0xFA; offset += 1; // CLI
        bios[offset] = 0xE4; bios[offset+1] = 0x60; offset += 2;
        bios[offset] = 0x88; bios[offset+1] = 0xC4; offset += 2;
        bios[offset] = 0x9D; offset += 1; // POPF
        bios[offset] = 0xCF; offset += 1;
        // no_key:
        bios[offset] = 0x0F; bios[offset+1] = 0x9C; bios[offset+2] = 0xC0; // SETZ AL
        offset += 3;
        bios[offset] = 0xCF; offset += 1;
        
        // Get shift flags (AH=0x02)
        offset = int16_offset + 0x1A;
        bios[offset] = 0x32; bios[offset+1] = 0xC0; // XOR AL, AL
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // === INT 19h handler at offset 0x4000 ===
        let int19_offset = 0x4000;
        offset = int19_offset;
        
        // INT 19h - Bootstrap loader
        // Load boot sector from drive 0 (CD-ROM) to 0x7C00
        
        // MOV AX, 0x0201 (read 1 sector)
        bios[offset] = 0xB8; bios[offset+1] = 0x01; bios[offset+2] = 0x02; offset += 3;
        // MOV BX, 0x7C00
        bios[offset] = 0xBB; bios[offset+1] = 0x00; bios[offset+2] = 0x7C; offset += 3;
        // MOV CX, 0x0001 (cylinder 0, sector 1)
        bios[offset] = 0xB9; bios[offset+1] = 0x01; bios[offset+2] = 0x00; offset += 3;
        // MOV DX, 0x0080 (drive 0, head 0) - try HDD first
        bios[offset] = 0xBA; bios[offset+1] = 0x80; bios[offset+2] = 0x00; offset += 3;
        // INT 13h
        bios[offset] = 0xCD; bios[offset+1] = 0x13; offset += 2;
        
        // Check if successful
        // TEST AH, AH
        bios[offset] = 0x84; bios[offset+1] = 0xE4; offset += 2;
        // JZ boot_success
        bios[offset] = 0x74; bios[offset+1] = 0x08; offset += 2;
        
        // Try CD-ROM (drive 0xE0)
        bios[offset] = 0xBA; bios[offset+1] = 0xE0; bios[offset+2] = 0x00; offset += 3;
        bios[offset] = 0xCD; bios[offset+1] = 0x13; offset += 2;
        
        // boot_success:
        offset = int19_offset + 0x1A;
        // JMP 0x0000:0x7C00
        bios[offset] = 0xEA; 
        bios[offset+1] = 0x00; bios[offset+2] = 0x7C; // offset
        bios[offset+3] = 0x00; bios[offset+4] = 0x00; // segment
        offset += 5;
        
        // === INT 12h handler at offset 0x5000 ===
        let int12_offset = 0x5000;
        offset = int12_offset;
        
        // INT 12h - Memory size
        // Return conventional memory size in AX (in KB)
        bios[offset] = 0xB8; bios[offset+1] = 0x00; bios[offset+2] = 0x02; // 512KB
        offset += 3;
        bios[offset] = 0xCF; offset += 1;
        
        // === INT 15h handler at offset 0x6000 ===
        let int15_offset = 0x6000;
        offset = int15_offset;
        
        // INT 15h - System services
        // AH=0x86: Wait
        // AH=0x87: Move extended memory
        // AH=0x88: Extended memory size
        // AH=0xE820: Memory map
        // AH=0xC0: Get system configuration
        
        // CMP AH, 0x88
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0x88; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x08; offset += 2;
        
        // CMP AH, 0xE8
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0xE8; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x10; offset += 2;
        
        // CMP AH, 0xC0
        bios[offset] = 0x80; bios[offset+1] = 0xFC; bios[offset+2] = 0xC0; offset += 3;
        bios[offset] = 0x74; bios[offset+1] = 0x20; offset += 2;
        
        // Default: error
        bios[offset] = 0xB4; bios[offset+1] = 0x86; offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Extended memory size (AH=0x88)
        offset = int15_offset + 0x0A;
        bios[offset] = 0xB8; bios[offset+1] = 0x00; bios[offset+2] = 0x3C; // 60MB extended
        offset += 3;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Memory map (AH=0xE8, AL=0x20)
        offset = int15_offset + 0x12;
        // For now just return error
        bios[offset] = 0xB4; bios[offset+1] = 0x86; // AH = error
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        // Get system config (AH=0xC0)
        offset = int15_offset + 0x22;
        bios[offset] = 0x30; bios[offset+1] = 0xE4; // XOR AH, AH
        offset += 2;
        bios[offset] = 0xCF; offset += 1;
        
        bios.truncate(offset + 1);
        bios.resize(0x10000, 0);
        bios
    }

    pub fn run_frame(&mut self) {
        // Execute enough cycles for one frame (~16ms at 1MHz)
        let cycles_per_frame = 16000;
        for _ in 0..cycles_per_frame {
            if self.cpu.halted {
                break;
            }
            self.cpu.step(&mut self.memory, &mut self.devices);
            self.devices.tick();
        }
        
        // Update display
        self.update_display();
        
        // Process keyboard input
        self.process_keyboard();
    }

    fn update_display(&mut self) {
        // Update VGA framebuffer from memory
        let mem_slice = self.memory.as_slice();
        self.devices.vga.update_text_mode(mem_slice);
        let fb = self.devices.vga.get_framebuffer();
        self.display.update_from_vga(&fb);
    }

    fn process_keyboard(&mut self) {
        while let Some(scancode) = self.input.get_keyboard_scancode() {
            if scancode & 0x80 != 0 {
                self.devices.keyboard.key_up(scancode & 0x7F);
            } else {
                self.devices.keyboard.key_down(scancode);
            }
            // Raise keyboard IRQ
            self.devices.pic.raise_irq(1);
        }
    }
}
