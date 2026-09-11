// TypeScript VM Core - Full x86 CPU emulator
// This serves as the runtime when WASM is not available

export class Registers {
  eax = 0; ebx = 0; ecx = 0; edx = 0;
  esi = 0; edi = 0; ebp = 0; esp = 0xFFFE;
  cs = 0xF000; ds = 0; es = 0; fs = 0; gs = 0; ss = 0;

  al(): number { return this.eax & 0xFF; }
  ah(): number { return (this.eax >> 8) & 0xFF; }
  ax(): number { return this.eax & 0xFFFF; }
  bl(): number { return this.ebx & 0xFF; }
  bh(): number { return (this.ebx >> 8) & 0xFF; }
  bx(): number { return this.ebx & 0xFFFF; }
  cl(): number { return this.ecx & 0xFF; }
  ch(): number { return (this.ecx >> 8) & 0xFF; }
  cx(): number { return this.ecx & 0xFFFF; }
  dl(): number { return this.edx & 0xFF; }
  dh(): number { return (this.edx >> 8) & 0xFF; }
  dx(): number { return this.edx & 0xFFFF; }
  bp(): number { return this.ebp & 0xFFFF; }
  si(): number { return this.esi & 0xFFFF; }
  di(): number { return this.edi & 0xFFFF; }

  setAL(v: number) { this.eax = (this.eax & 0xFFFFFF00) | (v & 0xFF); }
  setAH(v: number) { this.eax = (this.eax & 0xFFFF00FF) | ((v & 0xFF) << 8); }
  setAX(v: number) { this.eax = (this.eax & 0xFFFF0000) | (v & 0xFFFF); }
  setBL(v: number) { this.ebx = (this.ebx & 0xFFFFFF00) | (v & 0xFF); }
  setBH(v: number) { this.ebx = (this.ebx & 0xFFFF00FF) | ((v & 0xFF) << 8); }
  setBX(v: number) { this.ebx = (this.ebx & 0xFFFF0000) | (v & 0xFFFF); }
  setCL(v: number) { this.ecx = (this.ecx & 0xFFFFFF00) | (v & 0xFF); }
  setCH(v: number) { this.ecx = (this.ecx & 0xFFFF00FF) | ((v & 0xFF) << 8); }
  setCX(v: number) { this.ecx = (this.ecx & 0xFFFF0000) | (v & 0xFFFF); }
  setDL(v: number) { this.edx = (this.edx & 0xFFFFFF00) | (v & 0xFF); }
  setDH(v: number) { this.edx = (this.edx & 0xFFFF00FF) | ((v & 0xFF) << 8); }
  setDX(v: number) { this.edx = (this.edx & 0xFFFF0000) | (v & 0xFFFF); }

  getReg8(r: number): number {
    switch (r) {
      case 0: return this.al(); case 1: return this.cl();
      case 2: return this.dl(); case 3: return this.bl();
      case 4: return this.ah(); case 5: return this.ch();
      case 6: return this.dh(); case 7: return this.bh();
      default: return 0;
    }
  }

  setReg8(r: number, v: number) {
    switch (r) {
      case 0: this.setAL(v); break; case 1: this.setCL(v); break;
      case 2: this.setDL(v); break; case 3: this.setBL(v); break;
      case 4: this.setAH(v); break; case 5: this.setCH(v); break;
      case 6: this.setDH(v); break; case 7: this.setBH(v); break;
    }
  }

  getReg16(r: number): number {
    switch (r) {
      case 0: return this.ax(); case 1: return this.cx();
      case 2: return this.dx(); case 3: return this.bx();
      case 4: return this.esp & 0xFFFF; case 5: return this.ebp & 0xFFFF;
      case 6: return this.esi & 0xFFFF; case 7: return this.edi & 0xFFFF;
      default: return 0;
    }
  }

  setReg16(r: number, v: number) {
    v &= 0xFFFF;
    switch (r) {
      case 0: this.setAX(v); break; case 1: this.setCX(v); break;
      case 2: this.setDX(v); break; case 3: this.setBX(v); break;
      case 4: this.esp = (this.esp & 0xFFFF0000) | v; break;
      case 5: this.ebp = (this.ebp & 0xFFFF0000) | v; break;
      case 6: this.esi = (this.esi & 0xFFFF0000) | v; break;
      case 7: this.edi = (this.edi & 0xFFFF0000) | v; break;
    }
  }

  getReg32(r: number): number {
    switch (r) {
      case 0: return this.eax; case 1: return this.ecx;
      case 2: return this.edx; case 3: return this.ebx;
      case 4: return this.esp; case 5: return this.ebp;
      case 6: return this.esi; case 7: return this.edi;
      default: return 0;
    }
  }

  setReg32(r: number, v: number) {
    switch (r) {
      case 0: this.eax = v; break; case 1: this.ecx = v; break;
      case 2: this.edx = v; break; case 3: this.ebx = v; break;
      case 4: this.esp = v; break; case 5: this.ebp = v; break;
      case 6: this.esi = v; break; case 7: this.edi = v; break;
    }
  }
}

export class Flags {
  private value = 0x00000002;

  raw(): number { return this.value; }
  setRaw(v: number) { this.value = v; }

  cf(): boolean { return (this.value & 0x0001) !== 0; }
  setCF(v: boolean) { if (v) this.value |= 0x0001; else this.value &= ~0x0001; }
  pf(): boolean { return (this.value & 0x0004) !== 0; }
  setPF(v: boolean) { if (v) this.value |= 0x0004; else this.value &= ~0x0004; }
  af(): boolean { return (this.value & 0x0010) !== 0; }
  setAF(v: boolean) { if (v) this.value |= 0x0010; else this.value &= ~0x0010; }
  zf(): boolean { return (this.value & 0x0040) !== 0; }
  setZF(v: boolean) { if (v) this.value |= 0x0040; else this.value &= ~0x0040; }
  sf(): boolean { return (this.value & 0x0080) !== 0; }
  setSF(v: boolean) { if (v) this.value |= 0x0080; else this.value &= ~0x0080; }
  tf(): boolean { return (this.value & 0x0100) !== 0; }
  setTF(v: boolean) { if (v) this.value |= 0x0100; else this.value &= ~0x0100; }
  IF(): boolean { return (this.value & 0x0200) !== 0; }
  setIF(v: boolean) { if (v) this.value |= 0x0200; else this.value &= ~0x0200; }
  df(): boolean { return (this.value & 0x0400) !== 0; }
  setDF(v: boolean) { if (v) this.value |= 0x0400; else this.value &= ~0x0400; }
  of(): boolean { return (this.value & 0x0800) !== 0; }
  setOF(v: boolean) { if (v) this.value |= 0x0800; else this.value &= ~0x0800; }

  updateFlags8(result: number) {
    this.setZF((result & 0xFF) === 0);
    this.setSF((result & 0x80) !== 0);
    // Parity: count bits in low byte
    let v = result & 0xFF;
    v ^= v >> 4;
    v ^= v >> 2;
    v ^= v >> 1;
    this.setPF((v & 1) === 0);
  }

  updateFlags16(result: number) {
    this.setZF((result & 0xFFFF) === 0);
    this.setSF((result & 0x8000) !== 0);
    // Parity: count bits in low byte only
    let v = result & 0xFF;
    v ^= v >> 4;
    v ^= v >> 2;
    v ^= v >> 1;
    this.setPF((v & 1) === 0);
  }

  updateFlags32(result: number) {
    this.setZF(result === 0);
    this.setSF((result & 0x80000000) !== 0);
    // Parity: count bits in low byte only
    let v = result & 0xFF;
    v ^= v >> 4;
    v ^= v >> 2;
    v ^= v >> 1;
    this.setPF((v & 1) === 0);
  }
}

export class VMCore {
  regs = new Registers();
  flags = new Flags();
  eip = 0;
  halted = false;
  interruptFlag = true;
  memory: Uint8Array;
  memorySize: number;
  
  // Devices
  keyboardBuffer: number[] = [];
  mouseButtons = 0;
  mouseX = 0;
  mouseY = 0;
  
  // VGA
  framebuffer: Uint8Array;
  fbWidth = 640;
  fbHeight = 480;
  dacPalette: number[][] = [];
  
  // Storage
  diskImage: Uint8Array | null = null;
  isoImage: Uint8Array | null = null;
  
  // PIC
  masterIMR = 0xFF;
  masterIRR = 0;
  masterISR = 0;
  masterVectorBase = 0x08;
  
  // VGA text mode cursor
  textCursorPos = 0;

  constructor(memorySize: number) {
    this.memorySize = memorySize;
    this.memory = new Uint8Array(memorySize);
    this.framebuffer = new Uint8Array(this.fbWidth * this.fbHeight * 4);
    this.initPalette();
  }

  initPalette() {
    const defaultPalette: number[][] = [
      [0,0,0],[0,0,170],[0,170,0],[0,170,170],
      [170,0,0],[170,0,170],[170,85,0],[170,170,170],
      [85,85,85],[85,85,255],[85,255,85],[85,255,255],
      [255,85,85],[255,85,255],[255,255,85],[255,255,255]
    ];
    this.dacPalette = [];
    for (let i = 0; i < 256; i++) {
      this.dacPalette.push(i < 16 ? defaultPalette[i] : [0, 0, 0]);
    }
  }

  reset() {
    this.regs = new Registers();
    this.flags = new Flags();
    this.eip = 0;
    this.halted = false;
    this.interruptFlag = true;
    this.textCursorPos = 0;
  }

  boot() {
    this.setupBios();
  }

  setupBios() {
    // Setup IVT
    this.setupIVT();
    
    // Generate minimal BIOS
    const biosCode = this.generateMinimalBios();
    const biosStart = 0xF0000;
    
    for (let i = 0; i < biosCode.length && biosStart + i < this.memorySize; i++) {
      this.memory[biosStart + i] = biosCode[i];
    }
    
    // Set entry point
    this.regs.cs = 0xF000;
    this.eip = 0x0000;
    
    // Initialize BDA (BIOS Data Area)
    // 0x0449: Video mode (3 = 80x25 text)
    this.memory[0x0449] = 0x03;
    // 0x044A: Number of columns
    this.writeWord(0x044A, 80);
    // 0x0450: Cursor position (page 0)
    this.writeWord(0x0450, 0);
    
    // Clear screen
    for (let i = 0xB8000; i < 0xB8000 + 80 * 25 * 2; i += 2) {
      this.memory[i] = 0x20; // space
      this.memory[i + 1] = 0x07; // white on black
    }
  }

  setupIVT() {
    const biosBase = 0xF0000;
    
    // INT 10h - Video
    this.writeWord(0x10 * 4, (biosBase + 0x1000) & 0xFFFF);
    this.writeWord(0x10 * 4 + 2, 0xF000);
    
    // INT 13h - Disk
    this.writeWord(0x13 * 4, (biosBase + 0x2000) & 0xFFFF);
    this.writeWord(0x13 * 4 + 2, 0xF000);
    
    // INT 16h - Keyboard
    this.writeWord(0x16 * 4, (biosBase + 0x3000) & 0xFFFF);
    this.writeWord(0x16 * 4 + 2, 0xF000);
    
    // INT 19h - Bootstrap
    this.writeWord(0x19 * 4, (biosBase + 0x4000) & 0xFFFF);
    this.writeWord(0x19 * 4 + 2, 0xF000);
    
    // INT 12h - Memory size
    this.writeWord(0x12 * 4, (biosBase + 0x5000) & 0xFFFF);
    this.writeWord(0x12 * 4 + 2, 0xF000);
    
    // INT 15h - System services
    this.writeWord(0x15 * 4, (biosBase + 0x6000) & 0xFFFF);
    this.writeWord(0x15 * 4 + 2, 0xF000);
  }

  generateMinimalBios(): Uint8Array {
    const bios = new Uint8Array(0x10000);
    let o = 0;
    
    // Entry point: Initialize and call INT 19h
    bios[o++] = 0xFA; // CLI
    bios[o++] = 0xB8; bios[o++] = 0x00; bios[o++] = 0x00; // MOV AX, 0
    bios[o++] = 0x8E; bios[o++] = 0xD8; // MOV DS, AX
    bios[o++] = 0x8E; bios[o++] = 0xC0; // MOV ES, AX
    bios[o++] = 0x8E; bios[o++] = 0xD0; // MOV SS, AX
    bios[o++] = 0xBC; bios[o++] = 0x00; bios[o++] = 0x7C; // MOV SP, 0x7C00
    bios[o++] = 0xFB; // STI
    
    // Print boot message
    // MOV AH, 0x06 (scroll/clear)
    bios[o++] = 0xB4; bios[o++] = 0x06;
    // XOR AL, AL (full screen)
    bios[o++] = 0x32; bios[o++] = 0xC0;
    // XOR BH, BH (attribute)
    bios[o++] = 0x30; bios[o++] = 0xFF;
    // XOR CX, CX (top-left)
    bios[o++] = 0x31; bios[o++] = 0xC9;
    // MOV DH, 24
    bios[o++] = 0xB6; bios[o++] = 0x18;
    // MOV DL, 79
    bios[o++] = 0xB2; bios[o++] = 0x4F;
    // INT 10h
    bios[o++] = 0xCD; bios[o++] = 0x10;
    
    // Set cursor to 0,0
    bios[o++] = 0xB4; bios[o++] = 0x02; // AH=2
    bios[o++] = 0x30; bios[o++] = 0xFF; // XOR BH, BH
    bios[o++] = 0x30; bios[o++] = 0xD2; // XOR DL, DL (col)
    bios[o++] = 0x30; bios[o++] = 0xE6; // XOR DH, DH (row)
    bios[o++] = 0xCD; bios[o++] = 0x10;
    
    // Print "WebVirtualBox BIOS v0.1"
    const msg = "WebVirtualBox BIOS v0.1\r\n";
    for (let i = 0; i < msg.length; i++) {
      bios[o++] = 0xB4; bios[o++] = 0x0E; // AH=0x0E (teletype)
      bios[o++] = 0xB0; bios[o++] = msg.charCodeAt(i); // AL = char
      bios[o++] = 0xBB; bios[o++] = 0x00; bios[o++] = 0x00; // BX=0 (page 0)
      bios[o++] = 0xCD; bios[o++] = 0x10;
    }
    
    // Print "Loading boot sector..."
    const msg2 = "Loading boot sector...\r\n";
    for (let i = 0; i < msg2.length; i++) {
      bios[o++] = 0xB4; bios[o++] = 0x0E;
      bios[o++] = 0xB0; bios[o++] = msg2.charCodeAt(i);
      bios[o++] = 0xBB; bios[o++] = 0x00; bios[o++] = 0x00;
      bios[o++] = 0xCD; bios[o++] = 0x10;
    }
    
    // INT 19h - Bootstrap
    bios[o++] = 0xCD; bios[o++] = 0x19;
    
    // If boot fails, print message
    const msg3 = "\r\nNo bootable device found.\r\n";
    for (let i = 0; i < msg3.length; i++) {
      bios[o++] = 0xB4; bios[o++] = 0x0E;
      bios[o++] = 0xB0; bios[o++] = msg3.charCodeAt(i);
      bios[o++] = 0xBB; bios[o++] = 0x00; bios[o++] = 0x00;
      bios[o++] = 0xCD; bios[o++] = 0x10;
    }
    
    // HLT loop
    bios[o++] = 0xF4;
    bios[o++] = 0xEB; bios[o++] = 0xFE;
    
    // === INT 10h at offset 0x1000 ===
    o = 0x1000;
    
    // Video services dispatcher
    // AH=0x00: Set video mode
    // AH=0x02: Set cursor
    // AH=0x03: Get cursor
    // AH=0x06: Scroll up
    // AH=0x0E: Teletype
    // AH=0x0F: Get video mode
    
    // INT 10h dispatcher
    // Layout (corrected to avoid overlaps):
    // 0x1000: CMP AH,0x00 (3) JZ set_mode (2)   → next_ip=0x1005, target=0x1020, offset=0x1B
    // 0x1005: CMP AH,0x02 (3) JZ set_cursor (2) → next_ip=0x100A, target=0x1030, offset=0x26
    // 0x100A: CMP AH,0x03 (3) JZ get_cursor (2) → next_ip=0x100F, target=0x1050, offset=0x41
    // 0x100F: CMP AH,0x06 (3) JZ scroll_up (2)  → next_ip=0x1014, target=0x1070, offset=0x5C
    // 0x1014: CMP AH,0x0E (3) JZ teletype (2)   → next_ip=0x1019, target=0x10A0, offset=0x87
    // 0x1019: CMP AH,0x0F (3) JZ get_mode (2)   → next_ip=0x101E, target=0x1090, offset=0x72
    // 0x101E: IRET
    //
    // Handlers:
    // set_mode   = 0x1020 (11 bytes, ends 0x102B)
    // set_cursor = 0x1030 (23 bytes, ends 0x1047)
    // get_cursor = 0x1050 (19 bytes, ends 0x1063)
    // scroll_up  = 0x1070 (26 bytes, ends 0x108A)
    // get_mode   = 0x1090 (7 bytes, ends 0x1097)
    // teletype   = 0x10A0
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x00; // CMP AH, 0x00
    bios[o++] = 0x74; bios[o++] = 0x1B; // JZ set_mode (0x1005+0x1B=0x1020)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x02; // CMP AH, 0x02
    bios[o++] = 0x74; bios[o++] = 0x26; // JZ set_cursor (0x100A+0x26=0x1030)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x03; // CMP AH, 0x03
    bios[o++] = 0x74; bios[o++] = 0x41; // JZ get_cursor (0x100F+0x41=0x1050)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x06; // CMP AH, 0x06
    bios[o++] = 0x74; bios[o++] = 0x5C; // JZ scroll_up (0x1014+0x5C=0x1070)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x0E; // CMP AH, 0x0E
    bios[o++] = 0x74; bios[o++] = 0x87; // JZ teletype (0x1019+0x87=0x10A0)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x0F; // CMP AH, 0x0F
    bios[o++] = 0x74; bios[o++] = 0x72; // JZ get_mode (0x101E+0x72=0x1090)
    
    bios[o++] = 0xCF; // IRET (at 0x101E, ends at 0x101F)
    
    // Set video mode (offset 0x1020)
    o = 0x1020;
    // Store mode in BDA
    bios[o++] = 0x50; // PUSH AX
    bios[o++] = 0xB8; bios[o++] = 0x40; bios[o++] = 0x00; // MOV AX, 0x0040
    bios[o++] = 0x8E; bios[o++] = 0xD8; // MOV DS, AX (use DS for BDA access)
    bios[o++] = 0x58; // POP AX (restore AX with mode in AL)
    bios[o++] = 0xA2; bios[o++] = 0x49; bios[o++] = 0x00; // MOV [0x49], AL
    bios[o++] = 0xCF; // IRET
    
    // Set cursor (offset 0x1030)
    o = 0x1030;
    // DH=row, DL=col, BH=page
    bios[o++] = 0x50; // PUSH AX
    bios[o++] = 0x53; // PUSH BX
    bios[o++] = 0xB8; bios[o++] = 0x40; bios[o++] = 0x00; // MOV AX, 0x0040
    bios[o++] = 0x8E; bios[o++] = 0xD8; // MOV DS, AX
    // Calculate offset: DH * 80 + DL (16-bit arithmetic)
    bios[o++] = 0x88; bios[o++] = 0xF0; // MOV AL, DH
    bios[o++] = 0x32; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xB3; bios[o++] = 0x50; // MOV BL, 80
    bios[o++] = 0xF6; bios[o++] = 0xE3; // MUL BL → AX = DH * 80
    bios[o++] = 0x01; bios[o++] = 0xD0; // ADD AX, DX (16-bit add, not 8-bit!)
    bios[o++] = 0xA3; bios[o++] = 0x50; bios[o++] = 0x00; // MOV [0x50], AX
    bios[o++] = 0x5B; // POP BX
    bios[o++] = 0x58; // POP AX
    bios[o++] = 0xCF; // IRET
    
    // Get cursor (offset 0x1050)
    o = 0x1050;
    bios[o++] = 0xB8; bios[o++] = 0x40; bios[o++] = 0x00; // MOV AX, 0x0040
    bios[o++] = 0x8E; bios[o++] = 0xD8; // MOV DS, AX
    bios[o++] = 0xA1; bios[o++] = 0x50; bios[o++] = 0x00; // MOV AX, [0x50]
    bios[o++] = 0x88; bios[o++] = 0xC6; // MOV DH, AL (row)
    bios[o++] = 0xB3; bios[o++] = 0x50; // MOV BL, 80
    bios[o++] = 0xF6; bios[o++] = 0xF3; // DIV BL
    bios[o++] = 0x88; bios[o++] = 0xE2; // MOV DL, AH (col)
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    // Scroll up (offset 0x1070)
    o = 0x1070;
    bios[o++] = 0x50; // PUSH AX
    bios[o++] = 0x51; // PUSH CX
    bios[o++] = 0x57; // PUSH DI
    bios[o++] = 0x56; // PUSH SI
    bios[o++] = 0xB8; bios[o++] = 0x00; bios[o++] = 0xB8; // MOV AX, 0xB800
    bios[o++] = 0x8E; bios[o++] = 0xC0; // MOV ES, AX
    bios[o++] = 0xBF; bios[o++] = 0x00; bios[o++] = 0x00; // MOV DI, 0
    bios[o++] = 0xB9; bios[o++] = 0xD0; bios[o++] = 0x07; // MOV CX, 2000
    bios[o++] = 0xB8; bios[o++] = 0x20; bios[o++] = 0x07; // MOV AX, 0x0720
    bios[o++] = 0xFC; // CLD
    bios[o++] = 0xF3; bios[o++] = 0xAB; // REP STOSW
    bios[o++] = 0x5E; // POP SI
    bios[o++] = 0x5F; // POP DI
    bios[o++] = 0x59; // POP CX
    bios[o++] = 0x58; // POP AX
    bios[o++] = 0xCF; // IRET
    
    // Get video mode (offset 0x1090)
    o = 0x1090;
    bios[o++] = 0xB0; bios[o++] = 0x03; // MOV AL, 3 (mode)
    bios[o++] = 0xB4; bios[o++] = 0x50; // MOV AH, 80 (cols)
    bios[o++] = 0xB7; bios[o++] = 0x00; // MOV BH, 0 (page)
    bios[o++] = 0xCF; // IRET
    
    // Teletype output (offset 0x10A0)
    // AH=0x0E, AL=char, BH=page
    // Strategy: save AL to CL before overwriting AX
    // Each path (normal/CR/LF) ends with register restore + IRET
    o = 0x10A0;
    
    bios[o++] = 0x52; // PUSH DX
    bios[o++] = 0x53; // PUSH BX
    bios[o++] = 0x06; // PUSH ES
    bios[o++] = 0x1E; // PUSH DS
    
    // Save character (AL) to CL
    bios[o++] = 0x88; bios[o++] = 0xC1; // MOV CL, AL
    
    // CMP CL, 0x0D (CR)
    bios[o++] = 0x80; bios[o++] = 0xF9; bios[o++] = 0x0D;
    bios[o++] = 0x74; bios[o++] = 0x33; // JZ handle_cr (0x108B+0x33=0x10BE)
    
    // CMP CL, 0x0A (LF)
    bios[o++] = 0x80; bios[o++] = 0xF9; bios[o++] = 0x0A;
    bios[o++] = 0x74; bios[o++] = 0x43; // JZ handle_lf (0x1090+0x43=0x10D3)
    
    // === Normal character path ===
    // Get cursor position from BDA
    bios[o++] = 0xB8; bios[o++] = 0x40; bios[o++] = 0x00; // MOV AX, 0x0040
    bios[o++] = 0x8E; bios[o++] = 0xD8; // MOV DS, AX
    bios[o++] = 0x8B; bios[o++] = 0x1E; bios[o++] = 0x50; bios[o++] = 0x00; // MOV BX, [0x50]
    bios[o++] = 0xD1; bios[o++] = 0xE3; // SHL BX, 1
    
    // Set ES to video memory
    bios[o++] = 0xB8; bios[o++] = 0x00; bios[o++] = 0xB8; // MOV AX, 0xB800
    bios[o++] = 0x8E; bios[o++] = 0xC0; // MOV ES, AX
    
    // Write char (CL) and attribute (0x07)
    bios[o++] = 0x26; bios[o++] = 0x88; bios[o++] = 0x0F; // MOV [ES:BX], CL
    bios[o++] = 0x26; bios[o++] = 0xC6; bios[o++] = 0x47; bios[o++] = 0x01; bios[o++] = 0x07; // MOV byte [ES:BX+1], 0x07
    
    // Advance cursor
    bios[o++] = 0xB8; bios[o++] = 0x40; bios[o++] = 0x00; // MOV AX, 0x0040
    bios[o++] = 0x8E; bios[o++] = 0xD8; // MOV DS, AX
    bios[o++] = 0xFF; bios[o++] = 0x06; bios[o++] = 0x50; bios[o++] = 0x00; // INC word [0x50]
    
    // Check if cursor >= 2000 (end of screen)
    bios[o++] = 0x81; bios[o++] = 0x3E; bios[o++] = 0x50; bios[o++] = 0x00; // CMP word [0x50], 2000
    bios[o++] = 0xE0; bios[o++] = 0x07;
    bios[o++] = 0x72; bios[o++] = 0x00; // JB restore (0x10B9+0x00=0x10B9)
    
    // Reset cursor to 0
    bios[o++] = 0xC7; bios[o++] = 0x06; bios[o++] = 0x50; bios[o++] = 0x00; // MOV word [0x50], 0
    bios[o++] = 0x00; bios[o++] = 0x00;
    
    // restore: (shared restore code)
    bios[o++] = 0x1F; // POP DS
    bios[o++] = 0x07; // POP ES
    bios[o++] = 0x5B; // POP BX
    bios[o++] = 0x5A; // POP DX
    bios[o++] = 0xCF; // IRET
    
    // === handle_cr: cursor to beginning of current row ===
    // row = cursor / 80; new_cursor = row * 80
    bios[o++] = 0xA1; bios[o++] = 0x50; bios[o++] = 0x00; // MOV AX, [0x50]
    bios[o++] = 0xB3; bios[o++] = 0x50; // MOV BL, 80
    bios[o++] = 0xF6; bios[o++] = 0xF3; // DIV BL → AL=row, AH=col
    bios[o++] = 0x32; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xB3; bios[o++] = 0x50; // MOV BL, 80
    bios[o++] = 0xF6; bios[o++] = 0xE3; // MUL BL → AX = row * 80
    bios[o++] = 0xA3; bios[o++] = 0x50; bios[o++] = 0x00; // MOV [0x50], AX
    // Fall through to restore
    
    bios[o++] = 0x1F; // POP DS
    bios[o++] = 0x07; // POP ES
    bios[o++] = 0x5B; // POP BX
    bios[o++] = 0x5A; // POP DX
    bios[o++] = 0xCF; // IRET
    
    // === handle_lf: advance cursor by 80 (next row) ===
    bios[o++] = 0x81; bios[o++] = 0x06; bios[o++] = 0x50; bios[o++] = 0x00; // ADD word [0x50], 80
    bios[o++] = 0x50; bios[o++] = 0x00;
    
    // Check if past end of screen
    bios[o++] = 0x81; bios[o++] = 0x3E; bios[o++] = 0x50; bios[o++] = 0x00; // CMP word [0x50], 2000
    bios[o++] = 0xE0; bios[o++] = 0x07;
    bios[o++] = 0x72; bios[o++] = 0x06; // JB restore_lf (0x10E1+0x06=0x10E7)
    
    // Reset to 0 if past end
    bios[o++] = 0xC7; bios[o++] = 0x06; bios[o++] = 0x50; bios[o++] = 0x00; // MOV word [0x50], 0
    bios[o++] = 0x00; bios[o++] = 0x00;
    
    // restore_lf:
    bios[o++] = 0x1F; // POP DS
    bios[o++] = 0x07; // POP ES
    bios[o++] = 0x5B; // POP BX
    bios[o++] = 0x5A; // POP DX
    bios[o++] = 0xCF; // IRET
    
    // === INT 13h at offset 0x2000 ===
    // Layout:
    // 0x2000-0x2016: Dispatcher
    // 0x2018: reset handler
    // 0x201C: read handler
    // 0x2020: params handler
    // 0x2024: disktype handler
    
    o = 0x2000;
    
    // CMP AH, 0x00
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x00;
    bios[o++] = 0x74; bios[o++] = 0x13; // JZ reset (0x2005+0x13=0x2018)
    
    // CMP AH, 0x02
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x02;
    bios[o++] = 0x74; bios[o++] = 0x12; // JZ read (0x200A+0x12=0x201C)
    
    // CMP AH, 0x08
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x08;
    bios[o++] = 0x74; bios[o++] = 0x11; // JZ params (0x200F+0x11=0x2020)
    
    // CMP AH, 0x15
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x15;
    bios[o++] = 0x74; bios[o++] = 0x10; // JZ disktype (0x2014+0x10=0x2024)
    
    // Default: error
    bios[o++] = 0xB4; bios[o++] = 0x01; // MOV AH, 1
    bios[o++] = 0xCF; // IRET
    
    // Reset handler (offset 0x2018)
    o = 0x2018;
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    // Read sectors handler (offset 0x201C)
    o = 0x201C;
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    // Get drive params handler (offset 0x2020)
    o = 0x2020;
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    // Get disk type handler (offset 0x2024)
    o = 0x2024;
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    // === INT 16h at offset 0x3000 ===
    // Layout:
    // 0x3000-0x300F: Dispatcher
    // 0x3010: read_key handler
    // 0x3020: check_key handler
    // 0x3030: get_shift handler
    
    o = 0x3000;
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x00; // CMP AH, 0
    bios[o++] = 0x74; bios[o++] = 0x0B; // JZ read_key (0x3005+0x0B=0x3010)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x01; // CMP AH, 1
    bios[o++] = 0x74; bios[o++] = 0x16; // JZ check_key (0x300A+0x16=0x3020)
    
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x02; // CMP AH, 2
    bios[o++] = 0x74; bios[o++] = 0x21; // JZ get_shift (0x300F+0x21=0x3030)
    
    bios[o++] = 0xCF; // IRET
    
    // Read key handler (offset 0x3010)
    o = 0x3010;
    // Loop: IN AL, 0x64; TEST AL, 1; JZ loop
    bios[o++] = 0xE4; bios[o++] = 0x64; // IN AL, 0x64 (at 0x3010)
    bios[o++] = 0xA8; bios[o++] = 0x01; // TEST AL, 1 (at 0x3012)
    bios[o++] = 0x74; bios[o++] = 0xFA; // JZ -6 (back to 0x3010)
    bios[o++] = 0xE4; bios[o++] = 0x60; // IN AL, 0x60
    bios[o++] = 0x88; bios[o++] = 0xC4; // MOV AH, AL
    bios[o++] = 0x32; bios[o++] = 0xC0; // XOR AL, AL
    bios[o++] = 0xCF; // IRET
    
    // Check key handler (offset 0x3020)
    o = 0x3020;
    bios[o++] = 0xE4; bios[o++] = 0x64; // IN AL, 0x64
    bios[o++] = 0xA8; bios[o++] = 0x01; // TEST AL, 1
    bios[o++] = 0x74; bios[o++] = 0x06; // JZ no_key
    bios[o++] = 0xE4; bios[o++] = 0x60; // IN AL, 0x60
    bios[o++] = 0x88; bios[o++] = 0xC4; // MOV AH, AL
    bios[o++] = 0x32; bios[o++] = 0xC0; // XOR AL, AL
    bios[o++] = 0xCF; // IRET
    // no_key:
    bios[o++] = 0xF9; // STC (set carry = no key)
    bios[o++] = 0xCF; // IRET
    
    // Get shift flags handler (offset 0x3030)
    o = 0x3030;
    bios[o++] = 0x32; bios[o++] = 0xC0; // XOR AL, AL
    bios[o++] = 0xCF; // IRET
    
    // === INT 19h at offset 0x4000 ===
    o = 0x4000;
    
    // Bootstrap - load boot sector and jump to it
    // For now, just set up a simple boot sector in memory
    // and jump to 0x7C00
    
    // Set up a minimal boot sector that prints a message
    const bootMsg = "Guest OS booting...";
    const bootAddr = 0x7C00;
    
    // Write a minimal boot sector to memory
    let bo = bootAddr;
    // Set up segments
    this.memory[bo++] = 0xFA; // CLI
    this.memory[bo++] = 0x31; this.memory[bo++] = 0xC0; // XOR AX, AX
    this.memory[bo++] = 0x8E; this.memory[bo++] = 0xD8; // MOV DS, AX
    this.memory[bo++] = 0x8E; this.memory[bo++] = 0xD0; // MOV SS, AX
    this.memory[bo++] = 0xBC; this.memory[bo++] = 0x00; this.memory[bo++] = 0x7C; // MOV SP, 0x7C00
    this.memory[bo++] = 0xFB; // STI
    
    // Print message
    for (let i = 0; i < bootMsg.length; i++) {
      this.memory[bo++] = 0xB4; this.memory[bo++] = 0x0E; // MOV AH, 0x0E
      this.memory[bo++] = 0xB0; this.memory[bo++] = bootMsg.charCodeAt(i);
      this.memory[bo++] = 0xBB; this.memory[bo++] = 0x00; this.memory[bo++] = 0x00;
      this.memory[bo++] = 0xCD; this.memory[bo++] = 0x10;
    }
    
    // Newline
    this.memory[bo++] = 0xB4; this.memory[bo++] = 0x0E;
    this.memory[bo++] = 0xB0; this.memory[bo++] = 0x0D;
    this.memory[bo++] = 0xCD; this.memory[bo++] = 0x10;
    this.memory[bo++] = 0xB4; this.memory[bo++] = 0x0E;
    this.memory[bo++] = 0xB0; this.memory[bo++] = 0x0A;
    this.memory[bo++] = 0xCD; this.memory[bo++] = 0x10;
    
    // Print "VM is running. Waiting for input..."
    const msg4 = "VM is running. Waiting for input...\r\n";
    for (let i = 0; i < msg4.length; i++) {
      this.memory[bo++] = 0xB4; this.memory[bo++] = 0x0E;
      this.memory[bo++] = 0xB0; this.memory[bo++] = msg4.charCodeAt(i);
      this.memory[bo++] = 0xBB; this.memory[bo++] = 0x00; this.memory[bo++] = 0x00;
      this.memory[bo++] = 0xCD; this.memory[bo++] = 0x10;
    }
    
    // HLT loop
    this.memory[bo++] = 0xF4;
    this.memory[bo++] = 0xEB; this.memory[bo++] = 0xFE;
    
    // Boot signature
    this.memory[0x7DFE] = 0x55;
    this.memory[0x7DFF] = 0xAA;
    
    // INT 19h code: jump to boot sector
    bios[o++] = 0xEA; // JMP FAR
    bios[o++] = 0x00; bios[o++] = 0x7C; // offset 0x7C00
    bios[o++] = 0x00; bios[o++] = 0x00; // segment 0x0000
    
    // === INT 12h at offset 0x5000 ===
    o = 0x5000;
    bios[o++] = 0xB8; bios[o++] = 0x00; bios[o++] = 0x02; // MOV AX, 0x200 (512KB)
    bios[o++] = 0xCF; // IRET
    
    // === INT 15h at offset 0x6000 ===
    o = 0x6000;
    bios[o++] = 0x80; bios[o++] = 0xFC; bios[o++] = 0x88; // CMP AH, 0x88
    bios[o++] = 0x74; bios[o++] = 0x03; // JZ ext_mem (0x6005+0x03=0x6008)
    
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    // Extended memory size
    o = 0x6000 + 0x08;
    bios[o++] = 0xB8; bios[o++] = 0x00; bios[o++] = 0x3C; // MOV AX, 0x3C00 (60MB)
    bios[o++] = 0x30; bios[o++] = 0xE4; // XOR AH, AH
    bios[o++] = 0xCF; // IRET
    
    return bios;
  }

  segmentOverride: number = -1; // -1 = none, 0=ES, 1=CS, 2=SS, 3=DS

  step(): number {
    if (this.halted) return 0;
    
    const physAddr = ((this.regs.cs & 0xFFFF) << 4) + this.eip;
    let opcode = this.readByte(physAddr);
    
    // Handle segment override prefixes
    this.segmentOverride = -1;
    while (opcode === 0x26 || opcode === 0x2E || opcode === 0x36 || opcode === 0x3E) {
      switch (opcode) {
        case 0x26: this.segmentOverride = 0; break; // ES
        case 0x2E: this.segmentOverride = 1; break; // CS
        case 0x36: this.segmentOverride = 2; break; // SS
        case 0x3E: this.segmentOverride = 3; break; // DS
      }
      this.eip = (this.eip + 1) & 0xFFFF;
      const newPhys = ((this.regs.cs & 0xFFFF) << 4) + this.eip;
      opcode = this.readByte(newPhys);
    }
    
    this.executeInstruction(((this.regs.cs & 0xFFFF) << 4) + this.eip, opcode);
    return 1;
  }

  runFrame() {
    const cyclesPerFrame = 50000;
    for (let i = 0; i < cyclesPerFrame; i++) {
      if (this.halted) break;
      this.step();
    }
    this.updateDisplay();
  }

  executeInstruction(physAddr: number, opcode: number) {
    switch (opcode) {
      case 0x90: // NOP
        this.eip += 1;
        break;
        
      case 0xF4: // HLT
        this.halted = true;
        break;
        
      case 0xFA: // CLI
        this.interruptFlag = false;
        this.flags.setIF(false);
        this.eip += 1;
        break;
        
      case 0xFB: // STI
        this.interruptFlag = true;
        this.flags.setIF(true);
        this.eip += 1;
        break;
        
      case 0xFC: // CLD
        this.flags.setDF(false);
        this.eip += 1;
        break;
        
      case 0xFD: // STD
        this.flags.setDF(true);
        this.eip += 1;
        break;
        
      // MOV reg8, imm8
      case 0xB0: case 0xB1: case 0xB2: case 0xB3:
      case 0xB4: case 0xB5: case 0xB6: case 0xB7: {
        const reg = opcode - 0xB0;
        const imm = this.readByte(physAddr + 1);
        this.regs.setReg8(reg, imm);
        this.eip += 2;
        break;
      }
      
      // MOV reg16, imm16
      case 0xB8: case 0xB9: case 0xBA: case 0xBB:
      case 0xBC: case 0xBD: case 0xBE: case 0xBF: {
        const reg = opcode - 0xB8;
        const imm = this.readWord(physAddr + 1);
        this.regs.setReg16(reg, imm);
        this.eip += 3;
        break;
      }
      
      // MOV r/m16, r16
      case 0x89: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          this.regs.setReg16(rm, this.regs.getReg16(reg));
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.writeWord(addr, this.regs.getReg16(reg));
          this.eip += 2 + len;
        }
        break;
      }
      
      // MOV r16, r/m16
      case 0x8B: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          this.regs.setReg16(reg, this.regs.getReg16(rm));
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.regs.setReg16(reg, this.readWord(addr));
          this.eip += 2 + len;
        }
        break;
      }
      
      // MOV r/m8, r8
      case 0x88: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          this.regs.setReg8(rm, this.regs.getReg8(reg));
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.writeByte(addr, this.regs.getReg8(reg));
          this.eip += 2 + len;
        }
        break;
      }
      
      // MOV r8, r/m8
      case 0x8A: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          this.regs.setReg8(reg, this.regs.getReg8(rm));
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.regs.setReg8(reg, this.readByte(addr));
          this.eip += 2 + len;
        }
        break;
      }
      
      // MOV AL, moffs8
      case 0xA0: {
        const addr = this.readWord(physAddr + 1);
        this.regs.setAL(this.readByte((this.regs.ds << 4) + addr));
        this.eip += 3;
        break;
      }
      
      // MOV AX, moffs16
      case 0xA1: {
        const addr = this.readWord(physAddr + 1);
        this.regs.setAX(this.readWord((this.regs.ds << 4) + addr));
        this.eip += 3;
        break;
      }
      
      // MOV moffs8, AL
      case 0xA2: {
        const addr = this.readWord(physAddr + 1);
        this.writeByte((this.regs.ds << 4) + addr, this.regs.al());
        this.eip += 3;
        break;
      }
      
      // MOV moffs16, AX
      case 0xA3: {
        const addr = this.readWord(physAddr + 1);
        this.writeWord((this.regs.ds << 4) + addr, this.regs.ax());
        this.eip += 3;
        break;
      }
      
      // MOV Sreg, r/m16
      case 0x8E: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        let value: number;
        let len = 0;
        if (mod === 3) {
          value = this.regs.getReg16(rm);
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          len = this.eaLength(mod, rm);
          value = this.readWord(addr);
        }
        this.setSegReg(reg, value);
        this.eip += 2 + len;
        break;
      }
      
      // MOV r/m16, Sreg
      case 0x8C: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        const value = this.getSegReg(reg);
        if (mod === 3) {
          this.regs.setReg16(rm, value);
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.writeWord(addr, value);
          this.eip += 2 + len;
        }
        break;
      }
      
      // MOV r/m16, imm16
      case 0xC7: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, , rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const imm = this.readWord(physAddr + 2);
          this.regs.setReg16(rm, imm);
          this.eip += 4;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          const imm = this.readWord(physAddr + 2 + len);
          this.writeWord(addr, imm);
          this.eip += 2 + len + 2;
        }
        break;
      }
      
      // MOV r/m8, imm8
      case 0xC6: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, , rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const imm = this.readByte(physAddr + 2);
          this.regs.setReg8(rm, imm);
          this.eip += 3;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          const imm = this.readByte(physAddr + 2 + len);
          this.writeByte(addr, imm);
          this.eip += 2 + len + 1;
        }
        break;
      }
      
      // ADD AL, imm8
      case 0x04: {
        const imm = this.readByte(physAddr + 1);
        const a = this.regs.al();
        const result = (a + imm) & 0xFF;
        this.flags.setCF(a + imm > 0xFF);
        this.regs.setAL(result);
        this.flags.updateFlags8(result);
        this.eip += 2;
        break;
      }
      
      // ADD AX, imm16
      case 0x05: {
        const imm = this.readWord(physAddr + 1);
        const a = this.regs.ax();
        const result = (a + imm) & 0xFFFF;
        this.flags.setCF(a + imm > 0xFFFF);
        this.regs.setAX(result);
        this.flags.updateFlags16(result);
        this.eip += 3;
        break;
      }
      
      // ADD r/m16, r16
      case 0x01: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const a = this.regs.getReg16(rm);
          const b = this.regs.getReg16(reg);
          const result = (a + b) & 0xFFFF;
          this.flags.setCF(a + b > 0xFFFF);
          this.regs.setReg16(rm, result);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          const a = this.readWord(addr);
          const b = this.regs.getReg16(reg);
          const result = (a + b) & 0xFFFF;
          this.flags.setCF(a + b > 0xFFFF);
          this.writeWord(addr, result);
          this.flags.updateFlags16(result);
          this.eip += 2 + len;
        }
        break;
      }
      
      // ADD r16, r/m16
      case 0x03: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const a = this.regs.getReg16(reg);
          const b = this.regs.getReg16(rm);
          const result = (a + b) & 0xFFFF;
          this.flags.setCF(a + b > 0xFFFF);
          this.regs.setReg16(reg, result);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          const a = this.regs.getReg16(reg);
          const b = this.readWord(addr);
          const result = (a + b) & 0xFFFF;
          this.flags.setCF(a + b > 0xFFFF);
          this.regs.setReg16(reg, result);
          this.flags.updateFlags16(result);
          this.eip += 2 + len;
        }
        break;
      }
      
      // SUB AL, imm8
      case 0x2C: {
        const imm = this.readByte(physAddr + 1);
        const a = this.regs.al();
        this.flags.setCF(a < imm);
        const result = (a - imm) & 0xFF;
        this.regs.setAL(result);
        this.flags.updateFlags8(result);
        this.eip += 2;
        break;
      }
      
      // SUB AX, imm16
      case 0x2D: {
        const imm = this.readWord(physAddr + 1);
        const a = this.regs.ax();
        this.flags.setCF(a < imm);
        const result = (a - imm) & 0xFFFF;
        this.regs.setAX(result);
        this.flags.updateFlags16(result);
        this.eip += 3;
        break;
      }
      
      // SUB r/m16, r16
      case 0x29: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const a = this.regs.getReg16(rm);
          const b = this.regs.getReg16(reg);
          this.flags.setCF(a < b);
          const result = (a - b) & 0xFFFF;
          this.regs.setReg16(rm, result);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // CMP AL, imm8
      case 0x3C: {
        const imm = this.readByte(physAddr + 1);
        const a = this.regs.al();
        this.flags.setCF(a < imm);
        this.flags.updateFlags8((a - imm) & 0xFF);
        this.eip += 2;
        break;
      }
      
      // CMP AX, imm16
      case 0x3D: {
        const imm = this.readWord(physAddr + 1);
        const a = this.regs.ax();
        this.flags.setCF(a < imm);
        this.flags.updateFlags16((a - imm) & 0xFFFF);
        this.eip += 3;
        break;
      }
      
      // CMP r/m16, r16
      case 0x39: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const a = this.regs.getReg16(rm);
          const b = this.regs.getReg16(reg);
          this.flags.setCF(a < b);
          this.flags.updateFlags16((a - b) & 0xFFFF);
          this.eip += 2;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          const a = this.readWord(addr);
          const b = this.regs.getReg16(reg);
          this.flags.setCF(a < b);
          this.flags.updateFlags16((a - b) & 0xFFFF);
          this.eip += 2 + len;
        }
        break;
      }
      
      // INC r16
      case 0x40: case 0x41: case 0x42: case 0x43:
      case 0x44: case 0x45: case 0x46: case 0x47: {
        const reg = opcode - 0x40;
        const v = this.regs.getReg16(reg);
        const result = (v + 1) & 0xFFFF;
        this.regs.setReg16(reg, result);
        this.flags.updateFlags16(result);
        this.eip += 1;
        break;
      }
      
      // DEC r16
      case 0x48: case 0x49: case 0x4A: case 0x4B:
      case 0x4C: case 0x4D: case 0x4E: case 0x4F: {
        const reg = opcode - 0x48;
        const v = this.regs.getReg16(reg);
        const result = (v - 1) & 0xFFFF;
        this.regs.setReg16(reg, result);
        this.flags.updateFlags16(result);
        this.eip += 1;
        break;
      }
      
      // PUSH r16
      case 0x50: case 0x51: case 0x52: case 0x53:
      case 0x54: case 0x55: case 0x56: case 0x57: {
        const reg = opcode - 0x50;
        this.push16(this.regs.getReg16(reg));
        this.eip += 1;
        break;
      }
      
      // POP r16
      case 0x58: case 0x59: case 0x5A: case 0x5B:
      case 0x5C: case 0x5D: case 0x5E: case 0x5F: {
        const reg = opcode - 0x58;
        this.regs.setReg16(reg, this.pop16());
        this.eip += 1;
        break;
      }
      
      // PUSH ES
      case 0x06:
        this.push16(this.regs.es);
        this.eip += 1;
        break;
      
      // PUSH CS
      case 0x0E:
        this.push16(this.regs.cs);
        this.eip += 1;
        break;
      
      // PUSH SS
      case 0x16:
        this.push16(this.regs.ss);
        this.eip += 1;
        break;
      
      // PUSH DS
      case 0x1E:
        this.push16(this.regs.ds);
        this.eip += 1;
        break;
      
      // POP ES
      case 0x07:
        this.regs.es = this.pop16();
        this.eip += 1;
        break;
      
      // POP SS
      case 0x17:
        this.regs.ss = this.pop16();
        this.eip += 1;
        break;
      
      // POP DS
      case 0x1F:
        this.regs.ds = this.pop16();
        this.eip += 1;
        break;
      
      // JMP rel8
      case 0xEB: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        break;
      }
      
      // JMP rel16
      case 0xE9: {
        const offset = this.readWord(physAddr + 1);
        const signedOffset = offset > 32767 ? offset - 65536 : offset;
        this.eip = (this.eip + 3 + signedOffset) & 0xFFFF;
        break;
      }
      
      // JMP far
      case 0xEA: {
        const offset = this.readWord(physAddr + 1);
        const segment = this.readWord(physAddr + 3);
        this.regs.cs = segment;
        this.eip = offset;
        break;
      }
      
      // JZ/JE rel8
      case 0x74: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (this.flags.zf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JNZ/JNE rel8
      case 0x75: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (!this.flags.zf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JC/JB rel8
      case 0x72: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (this.flags.cf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JNC/JAE rel8
      case 0x73: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (!this.flags.cf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JS rel8
      case 0x78: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (this.flags.sf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JNS rel8
      case 0x79: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (!this.flags.sf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JBE rel8
      case 0x76: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (this.flags.cf() || this.flags.zf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JA rel8
      case 0x77: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (!this.flags.cf() && !this.flags.zf()) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // CALL rel16
      case 0xE8: {
        const offset = this.readWord(physAddr + 1);
        const signedOffset = offset > 32767 ? offset - 65536 : offset;
        this.push16((this.eip + 3) & 0xFFFF);
        this.eip = (this.eip + 3 + signedOffset) & 0xFFFF;
        break;
      }
      
      // RET
      case 0xC3:
        this.eip = this.pop16();
        break;
      
      // INT imm8
      case 0xCD: {
        const intNum = this.readByte(physAddr + 1);
        this.push16(this.flags.raw() & 0xFFFF);
        this.push16(this.regs.cs);
        this.push16((this.eip + 2) & 0xFFFF);
        
        const ivtAddr = intNum * 4;
        const ip = this.readWord(ivtAddr);
        const cs = this.readWord(ivtAddr + 2);
        
        this.regs.cs = cs;
        this.eip = ip;
        this.interruptFlag = false;
        break;
      }
      
      // IRET
      case 0xCF: {
        this.eip = this.pop16();
        this.regs.cs = this.pop16();
        this.flags.setRaw(this.pop16());
        break;
      }
      
      // IN AL, imm8
      case 0xE4: {
        const port = this.readByte(physAddr + 1);
        this.regs.setAL(this.ioRead(port));
        this.eip += 2;
        break;
      }
      
      // IN AX, imm8
      case 0xE5: {
        const port = this.readByte(physAddr + 1);
        const lo = this.ioRead(port);
        const hi = this.ioRead(port + 1);
        this.regs.setAX((hi << 8) | lo);
        this.eip += 2;
        break;
      }
      
      // IN AL, DX
      case 0xEC: {
        this.regs.setAL(this.ioRead(this.regs.dx()));
        this.eip += 1;
        break;
      }
      
      // OUT imm8, AL
      case 0xE6: {
        const port = this.readByte(physAddr + 1);
        this.ioWrite(port, this.regs.al());
        this.eip += 2;
        break;
      }
      
      // OUT DX, AL
      case 0xEE: {
        this.ioWrite(this.regs.dx(), this.regs.al());
        this.eip += 1;
        break;
      }
      
      // XOR r/m16, r16
      case 0x31: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const result = (this.regs.getReg16(rm) ^ this.regs.getReg16(reg)) & 0xFFFF;
          this.regs.setReg16(rm, result);
          this.flags.setCF(false);
          this.flags.setOF(false);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // XOR r16, r/m16
      case 0x33: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const result = (this.regs.getReg16(reg) ^ this.regs.getReg16(rm)) & 0xFFFF;
          this.regs.setReg16(reg, result);
          this.flags.setCF(false);
          this.flags.setOF(false);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // XOR AL, AL (common pattern)
      case 0x32: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const result = (this.regs.getReg8(reg) ^ this.regs.getReg8(rm)) & 0xFF;
          this.regs.setReg8(reg, result);
          this.flags.setCF(false);
          this.flags.updateFlags8(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // AND r/m16, r16
      case 0x21: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const result = (this.regs.getReg16(rm) & this.regs.getReg16(reg)) & 0xFFFF;
          this.regs.setReg16(rm, result);
          this.flags.setCF(false);
          this.flags.setOF(false);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // AND AL, imm8
      case 0x24: {
        const imm = this.readByte(physAddr + 1);
        const result = (this.regs.al() & imm) & 0xFF;
        this.regs.setAL(result);
        this.flags.setCF(false);
        this.flags.updateFlags8(result);
        this.eip += 2;
        break;
      }
      
      // OR r/m16, r16
      case 0x09: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const result = (this.regs.getReg16(rm) | this.regs.getReg16(reg)) & 0xFFFF;
          this.regs.setReg16(rm, result);
          this.flags.setCF(false);
          this.flags.setOF(false);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // OR AL, imm8
      case 0x0C: {
        const imm = this.readByte(physAddr + 1);
        const result = (this.regs.al() | imm) & 0xFF;
        this.regs.setAL(result);
        this.flags.setCF(false);
        this.flags.updateFlags8(result);
        this.eip += 2;
        break;
      }
      
      // TEST AL, imm8
      case 0xA8: {
        const imm = this.readByte(physAddr + 1);
        const result = (this.regs.al() & imm) & 0xFF;
        this.flags.setCF(false);
        this.flags.setOF(false);
        this.flags.updateFlags8(result);
        this.eip += 2;
        break;
      }
      
      // TEST r/m16, r16
      case 0x85: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const result = (this.regs.getReg16(rm) & this.regs.getReg16(reg)) & 0xFFFF;
          this.flags.setCF(false);
          this.flags.setOF(false);
          this.flags.updateFlags16(result);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // Group 1: r/m16, imm16
      case 0x81: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const imm = this.readWord(physAddr + 2);
          this.execGroup1_16(op, rm, imm, true);
          this.eip += 4;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          const imm = this.readWord(physAddr + 2 + len);
          this.execGroup1_mem16(op, addr, imm);
          this.eip += 2 + len + 2;
        }
        break;
      }
      
      // Group 1: r/m16, imm8
      case 0x83: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        const imm8 = this.readByte(physAddr + 2);
        const imm = imm8 > 127 ? imm8 - 256 + 65536 : imm8;
        if (mod === 3) {
          this.execGroup1_16(op, rm, imm & 0xFFFF, true);
          this.eip += 3;
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.execGroup1_mem16(op, addr, imm & 0xFFFF);
          this.eip += 2 + len + 1;
        }
        break;
      }
      
      // Group 1: r/m8, imm8
      case 0x80: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        const imm = this.readByte(physAddr + 2);
        if (mod === 3) {
          this.execGroup1_8(op, rm, imm);
          this.eip += 3;
        } else {
          this.eip += 3;
        }
        break;
      }
      
      // Group 2: shift r/m16, 1
      case 0xD1: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          this.execGroup2_16(op, rm, 1);
          this.eip += 2;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // Group 3: TEST/NOT/NEG r/m16
      case 0xF7: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          if (op === 0) { // TEST
            const imm = this.readWord(physAddr + 2);
            const result = (this.regs.getReg16(rm) & imm) & 0xFFFF;
            this.flags.setCF(false);
            this.flags.setOF(false);
            this.flags.updateFlags16(result);
            this.eip += 4;
          } else if (op === 2) { // NOT
            this.regs.setReg16(rm, (~this.regs.getReg16(rm)) & 0xFFFF);
            this.eip += 2;
          } else if (op === 3) { // NEG
            const v = this.regs.getReg16(rm);
            this.flags.setCF(v !== 0);
            const result = (0 - v) & 0xFFFF;
            this.regs.setReg16(rm, result);
            this.flags.updateFlags16(result);
            this.eip += 2;
          } else {
            this.eip += 2;
          }
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // Group 5: r/m16 (INC/DEC/CALL/JMP/PUSH)
      case 0xFF: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          switch (op) {
            case 0: { // INC
              const v = this.regs.getReg16(rm);
              const result = (v + 1) & 0xFFFF;
              this.regs.setReg16(rm, result);
              this.flags.updateFlags16(result);
              this.eip += 2;
              break;
            }
            case 1: { // DEC
              const v = this.regs.getReg16(rm);
              const result = (v - 1) & 0xFFFF;
              this.regs.setReg16(rm, result);
              this.flags.updateFlags16(result);
              this.eip += 2;
              break;
            }
            case 2: { // CALL r/m16
              const addr = this.regs.getReg16(rm);
              this.push16((this.eip + 2) & 0xFFFF);
              this.eip = addr;
              break;
            }
            case 4: { // JMP r/m16
              this.eip = this.regs.getReg16(rm);
              break;
            }
            case 6: { // PUSH r/m16
              this.push16(this.regs.getReg16(rm));
              this.eip += 2;
              break;
            }
            default:
              this.eip += 2;
          }
        } else {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          switch (op) {
            case 0: { // INC m16
              const v = this.readWord(addr);
              const result = (v + 1) & 0xFFFF;
              this.writeWord(addr, result);
              this.flags.updateFlags16(result);
              this.eip += 2 + len;
              break;
            }
            case 1: { // DEC m16
              const v = this.readWord(addr);
              const result = (v - 1) & 0xFFFF;
              this.writeWord(addr, result);
              this.flags.updateFlags16(result);
              this.eip += 2 + len;
              break;
            }
            case 2: { // CALL m16
              const target = this.readWord(addr);
              this.push16((this.eip + 2 + len) & 0xFFFF);
              this.eip = target;
              break;
            }
            case 4: { // JMP m16
              this.eip = this.readWord(addr);
              break;
            }
            case 6: { // PUSH m16
              this.push16(this.readWord(addr));
              this.eip += 2 + len;
              break;
            }
            default:
              this.eip += 2 + len;
          }
        }
        break;
      }
      
      // PUSH imm8
      case 0x6A: {
        const imm = this.readByte(physAddr + 1);
        const signed = imm > 127 ? imm - 256 : imm;
        this.push16(signed & 0xFFFF);
        this.eip += 2;
        break;
      }
      
      // PUSH imm16
      case 0x68: {
        const imm = this.readWord(physAddr + 1);
        this.push16(imm);
        this.eip += 3;
        break;
      }
      
      // XCHG AX, r16
      case 0x91: case 0x92: case 0x93:
      case 0x94: case 0x95: case 0x96: case 0x97: {
        const reg = opcode - 0x90;
        const tmp = this.regs.ax();
        this.regs.setAX(this.regs.getReg16(reg));
        this.regs.setReg16(reg, tmp);
        this.eip += 1;
        break;
      }
      
      // CBW
      case 0x98: {
        const al = this.regs.al();
        this.regs.setAX(al > 127 ? al | 0xFF00 : al);
        this.eip += 1;
        break;
      }
      
      // CWD
      case 0x99: {
        this.regs.setDX(this.regs.ax() > 32767 ? 0xFFFF : 0);
        this.eip += 1;
        break;
      }
      
      // REP prefix
      case 0xF3: {
        const nextOp = this.readByte(physAddr + 1);
        if (nextOp === 0xAB && this.regs.cx() > 0) {
          // REP STOSW
          const dst = (this.regs.es << 4) + this.regs.edi;
          this.writeWord(dst, this.regs.ax());
          const dir = this.flags.df() ? -2 : 2;
          this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
          this.regs.setCX(this.regs.cx() - 1);
          if (this.regs.cx() !== 0) {
            // Repeat
          } else {
            this.eip += 2;
          }
        } else if (nextOp === 0xAA && this.regs.cx() > 0) {
          // REP STOSB
          const dst = (this.regs.es << 4) + this.regs.edi;
          this.writeByte(dst, this.regs.al());
          const dir = this.flags.df() ? -1 : 1;
          this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
          this.regs.setCX(this.regs.cx() - 1);
          if (this.regs.cx() !== 0) {
            // Repeat
          } else {
            this.eip += 2;
          }
        } else if (nextOp === 0xA5 && this.regs.cx() > 0) {
          // REP MOVSW
          const src = (this.regs.ds << 4) + this.regs.esi;
          const dst = (this.regs.es << 4) + this.regs.edi;
          this.writeWord(dst, this.readWord(src));
          const dir = this.flags.df() ? -2 : 2;
          this.regs.esi = (this.regs.esi + dir) & 0xFFFF;
          this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
          this.regs.setCX(this.regs.cx() - 1);
          if (this.regs.cx() !== 0) {
            // Repeat
          } else {
            this.eip += 2;
          }
        } else if (nextOp === 0xA4 && this.regs.cx() > 0) {
          // REP MOVSB
          const src = (this.regs.ds << 4) + this.regs.esi;
          const dst = (this.regs.es << 4) + this.regs.edi;
          this.writeByte(dst, this.readByte(src));
          const dir = this.flags.df() ? -1 : 1;
          this.regs.esi = (this.regs.esi + dir) & 0xFFFF;
          this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
          this.regs.setCX(this.regs.cx() - 1);
          if (this.regs.cx() !== 0) {
            // Repeat
          } else {
            this.eip += 2;
          }
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // STOSW
      case 0xAB: {
        const dst = (this.regs.es << 4) + this.regs.edi;
        this.writeWord(dst, this.regs.ax());
        const dir = this.flags.df() ? -2 : 2;
        this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
        this.eip += 1;
        break;
      }
      
      // STOSB
      case 0xAA: {
        const dst = (this.regs.es << 4) + this.regs.edi;
        this.writeByte(dst, this.regs.al());
        const dir = this.flags.df() ? -1 : 1;
        this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
        this.eip += 1;
        break;
      }
      
      // MOVSB
      case 0xA4: {
        const src = (this.regs.ds << 4) + this.regs.esi;
        const dst = (this.regs.es << 4) + this.regs.edi;
        this.writeByte(dst, this.readByte(src));
        const dir = this.flags.df() ? -1 : 1;
        this.regs.esi = (this.regs.esi + dir) & 0xFFFF;
        this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
        this.eip += 1;
        break;
      }
      
      // MOVSW
      case 0xA5: {
        const src = (this.regs.ds << 4) + this.regs.esi;
        const dst = (this.regs.es << 4) + this.regs.edi;
        this.writeWord(dst, this.readWord(src));
        const dir = this.flags.df() ? -2 : 2;
        this.regs.esi = (this.regs.esi + dir) & 0xFFFF;
        this.regs.edi = (this.regs.edi + dir) & 0xFFFF;
        this.eip += 1;
        break;
      }
      
      // LEA
      case 0x8D: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, reg, rm] = this.decodeModRM(modrm);
        if (mod !== 3) {
          const addr = this.calcEA(mod, rm, physAddr + 2);
          const len = this.eaLength(mod, rm);
          this.regs.setReg16(reg, addr & 0xFFFF);
          this.eip += 2 + len;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // Segment overrides - handled in step() now
      case 0x26: case 0x2E: case 0x36: case 0x3E: {
        this.eip = (this.eip + 1) & 0xFFFF;
        break;
      }
      
      // LOOP
      case 0xE2: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        this.regs.setCX(this.regs.cx() - 1);
        if (this.regs.cx() !== 0) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // JCXZ
      case 0xE3: {
        const offset = this.readByte(physAddr + 1);
        const signedOffset = offset > 127 ? offset - 256 : offset;
        if (this.regs.cx() === 0) {
          this.eip = (this.eip + 2 + signedOffset) & 0xFFFF;
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // SETcc
      case 0x0F: {
        const secondOp = this.readByte(physAddr + 1);
        if (secondOp >= 0x90 && secondOp <= 0x9F) {
          const modrm = this.readByte(physAddr + 2);
          const [mod, cond, rm] = this.decodeModRM(modrm);
          let condMet = false;
          switch (cond) {
            case 0: condMet = this.flags.of(); break;
            case 1: condMet = !this.flags.of(); break;
            case 2: condMet = this.flags.cf(); break;
            case 3: condMet = !this.flags.cf(); break;
            case 4: condMet = this.flags.zf(); break;
            case 5: condMet = !this.flags.zf(); break;
            case 6: condMet = this.flags.cf() || this.flags.zf(); break;
            case 7: condMet = !this.flags.cf() && !this.flags.zf(); break;
            case 8: condMet = this.flags.sf(); break;
            case 9: condMet = !this.flags.sf(); break;
            default: condMet = false;
          }
          if (mod === 3) {
            this.regs.setReg8(rm, condMet ? 1 : 0);
            this.eip += 3;
          } else {
            this.eip += 3;
          }
        } else {
          this.eip += 2;
        }
        break;
      }
      
      // MUL/IMUL/DIV/IDIV
      case 0xF6: {
        const modrm = this.readByte(physAddr + 1);
        const [mod, op, rm] = this.decodeModRM(modrm);
        if (mod === 3) {
          const v = this.regs.getReg8(rm);
          switch (op) {
            case 4: { // MUL
              const result = this.regs.al() * v;
              this.regs.setAX(result & 0xFFFF);
              this.flags.setCF((result >> 8) !== 0);
              this.eip += 2;
              break;
            }
            case 6: { // DIV
              const dividend = this.regs.ax();
              if (v !== 0) {
                this.regs.setAL(Math.floor(dividend / v) & 0xFF);
                this.regs.setAH(dividend % v);
              }
              this.eip += 2;
              break;
            }
            default:
              this.eip += 2;
          }
        } else {
          this.eip += 2;
        }
        break;
      }
      
      default:
        // Unknown instruction - advance by 1
        this.eip = (this.eip + 1) & 0xFFFF;
        break;
    }
    
    // Wrap EIP
    this.eip &= 0xFFFF;
  }

  execGroup1_16(op: number, rm: number, imm: number, isReg: boolean) {
    const a = this.regs.getReg16(rm);
    let result: number;
    switch (op) {
      case 0: result = (a + imm) & 0xFFFF; this.flags.setCF(a + imm > 0xFFFF); break;
      case 1: result = (a | imm) & 0xFFFF; this.flags.setCF(false); break;
      case 4: result = (a & imm) & 0xFFFF; this.flags.setCF(false); break;
      case 5: result = (a - imm) & 0xFFFF; this.flags.setCF(a < imm); break;
      case 6: result = (a ^ imm) & 0xFFFF; this.flags.setCF(false); break;
      case 7: result = (a - imm) & 0xFFFF; this.flags.setCF(a < imm); break; // CMP
      default: result = a;
    }
    if (op !== 7) this.regs.setReg16(rm, result);
    this.flags.updateFlags16(result);
    if (op === 1 || op === 4 || op === 6) { this.flags.setCF(false); this.flags.setOF(false); }
  }

  execGroup1_mem16(op: number, addr: number, imm: number) {
    const a = this.readWord(addr);
    let result: number;
    switch (op) {
      case 0: result = (a + imm) & 0xFFFF; this.flags.setCF(a + imm > 0xFFFF); break; // ADD
      case 1: result = (a | imm) & 0xFFFF; this.flags.setCF(false); break; // OR
      case 4: result = (a & imm) & 0xFFFF; this.flags.setCF(false); break; // AND
      case 5: result = (a - imm) & 0xFFFF; this.flags.setCF(a < imm); break; // SUB
      case 6: result = (a ^ imm) & 0xFFFF; this.flags.setCF(false); break; // XOR
      case 7: result = (a - imm) & 0xFFFF; this.flags.setCF(a < imm); break; // CMP
      default: result = a;
    }
    if (op !== 7) this.writeWord(addr, result);
    this.flags.updateFlags16(result);
    if (op === 1 || op === 4 || op === 6) { this.flags.setCF(false); this.flags.setOF(false); }
  }

  execGroup1_8(op: number, rm: number, imm: number) {
    const a = this.regs.getReg8(rm);
    let result: number;
    switch (op) {
      case 0: result = (a + imm) & 0xFF; this.flags.setCF(a + imm > 0xFF); break;
      case 1: result = (a | imm) & 0xFF; this.flags.setCF(false); break;
      case 4: result = (a & imm) & 0xFF; this.flags.setCF(false); break;
      case 5: result = (a - imm) & 0xFF; this.flags.setCF(a < imm); break;
      case 6: result = (a ^ imm) & 0xFF; this.flags.setCF(false); break;
      case 7: result = (a - imm) & 0xFF; this.flags.setCF(a < imm); break;
      default: result = a;
    }
    if (op !== 7) this.regs.setReg8(rm, result);
    this.flags.updateFlags8(result);
  }

  execGroup2_16(op: number, rm: number, count: number) {
    let v = this.regs.getReg16(rm);
    switch (op) {
      case 0: v = ((v << count) | (v >> (16 - count))) & 0xFFFF; break; // ROL
      case 1: v = ((v >> count) | (v << (16 - count))) & 0xFFFF; break; // ROR
      case 4: v = (v << count) & 0xFFFF; break; // SHL
      case 5: v = (v >> count) & 0xFFFF; break; // SHR
      case 7: v = ((v as number) >> count) & 0xFFFF; break; // SAR
    }
    this.regs.setReg16(rm, v);
    this.flags.updateFlags16(v);
  }

  decodeModRM(modrm: number): [number, number, number] {
    return [(modrm >> 6) & 3, (modrm >> 3) & 7, modrm & 7];
  }

  calcEA(mod: number, rm: number, dispAddr: number): number {
    let offset = 0;
    switch (rm) {
      case 0: offset = (this.regs.bx() + this.regs.si()) & 0xFFFF; break;
      case 1: offset = (this.regs.bx() + this.regs.di()) & 0xFFFF; break;
      case 2: offset = (this.regs.bp() + this.regs.si()) & 0xFFFF; break;
      case 3: offset = (this.regs.bp() + this.regs.di()) & 0xFFFF; break;
      case 4: offset = this.regs.si(); break;
      case 5: offset = this.regs.di(); break;
      case 6:
        if (mod === 0) {
          offset = this.readWord(dispAddr);
        } else {
          offset = this.regs.bp();
        }
        break;
      case 7: offset = this.regs.bx(); break;
    }
    
    if (mod === 1) {
      const disp = this.readByte(dispAddr);
      offset = (offset + (disp > 127 ? disp - 256 : disp)) & 0xFFFF;
    } else if (mod === 2) {
      offset = (offset + this.readWord(dispAddr)) & 0xFFFF;
    }
    
    // Use segment override if set
    let segBase: number;
    if (this.segmentOverride >= 0) {
      switch (this.segmentOverride) {
        case 0: segBase = this.regs.es; break;
        case 1: segBase = this.regs.cs; break;
        case 2: segBase = this.regs.ss; break;
        case 3: segBase = this.regs.ds; break;
        default: segBase = this.regs.ds;
      }
    } else if (rm === 2 || rm === 3 || (rm === 6 && mod !== 0)) {
      segBase = this.regs.ss; // BP-based addressing uses SS
    } else {
      segBase = this.regs.ds;
    }
    
    return ((segBase << 4) + offset) & 0xFFFFF;
  }

  eaLength(mod: number, rm: number): number {
    if (mod === 0 && rm === 6) return 2;
    if (mod === 1) return 1;
    if (mod === 2) return 2;
    return 0;
  }

  setSegReg(reg: number, value: number) {
    switch (reg) {
      case 0: this.regs.es = value; break;
      case 1: this.regs.cs = value; break;
      case 2: this.regs.ss = value; break;
      case 3: this.regs.ds = value; break;
      case 4: this.regs.fs = value; break;
      case 5: this.regs.gs = value; break;
    }
  }

  getSegReg(reg: number): number {
    switch (reg) {
      case 0: return this.regs.es;
      case 1: return this.regs.cs;
      case 2: return this.regs.ss;
      case 3: return this.regs.ds;
      case 4: return this.regs.fs;
      case 5: return this.regs.gs;
      default: return 0;
    }
  }

  push16(value: number) {
    this.regs.esp = (this.regs.esp - 2) & 0xFFFF;
    const addr = (this.regs.ss << 4) + this.regs.esp;
    this.writeWord(addr, value & 0xFFFF);
  }

  pop16(): number {
    const addr = (this.regs.ss << 4) + this.regs.esp;
    const value = this.readWord(addr);
    this.regs.esp = (this.regs.esp + 2) & 0xFFFF;
    return value;
  }

  readByte(addr: number): number {
    addr &= 0xFFFFF;
    if (addr < this.memorySize) return this.memory[addr];
    return 0xFF;
  }

  writeByte(addr: number, value: number) {
    addr &= 0xFFFFF;
    if (addr < this.memorySize) this.memory[addr] = value & 0xFF;
  }

  readWord(addr: number): number {
    return this.readByte(addr) | (this.readByte(addr + 1) << 8);
  }

  writeWord(addr: number, value: number) {
    this.writeByte(addr, value & 0xFF);
    this.writeByte(addr + 1, (value >> 8) & 0xFF);
  }

  ioRead(port: number): number {
    switch (port) {
      case 0x60: // Keyboard data
        if (this.keyboardBuffer.length > 0) {
          return this.keyboardBuffer.shift()!;
        }
        return 0;
      case 0x64: // Keyboard status
        return this.keyboardBuffer.length > 0 ? 0x01 : 0x00;
      case 0x3DA: // VGA status
        return 0;
      default:
        return 0xFF;
    }
  }

  ioWrite(port: number, value: number) {
    // Handle I/O writes
  }

  updateDisplay() {
    // Update text mode display
    for (let row = 0; row < 25; row++) {
      for (let col = 0; col < 80; col++) {
        const offset = 0xB8000 + (row * 80 + col) * 2;
        const charByte = this.memory[offset] || 0;
        const attrByte = this.memory[offset + 1] || 0x07;
        
        const fg = attrByte & 0x0F;
        const bg = (attrByte >> 4) & 0x0F;
        
        const fgColor = this.dacPalette[fg] || [170, 170, 170];
        const bgColor = this.dacPalette[bg] || [0, 0, 0];
        
        const px = col * 8;
        const py = row * 16;
        
        // Render simple bitmap font
        for (let cy = 0; cy < 16; cy++) {
          for (let cx = 0; cx < 8; cx++) {
            const fbIdx = ((py + cy) * this.fbWidth + (px + cx)) * 4;
            if (fbIdx + 3 < this.framebuffer.length) {
              const isSet = charByte !== 0 && charByte !== 0x20 && this.getCharPixel(charByte, cx, cy);
              if (isSet) {
                this.framebuffer[fbIdx] = fgColor[0];
                this.framebuffer[fbIdx + 1] = fgColor[1];
                this.framebuffer[fbIdx + 2] = fgColor[2];
              } else {
                this.framebuffer[fbIdx] = bgColor[0];
                this.framebuffer[fbIdx + 1] = bgColor[1];
                this.framebuffer[fbIdx + 2] = bgColor[2];
              }
              this.framebuffer[fbIdx + 3] = 255;
            }
          }
        }
      }
    }
    
    // Draw cursor - read position from BDA (0x0450)
    const cursorPos = this.readWord(0x0450);
    const cursorRow = Math.floor(cursorPos / 80);
    const cursorCol = cursorPos % 80;
    if (cursorRow < 25 && cursorCol < 80) {
      const cpx = cursorCol * 8;
      const cpy = cursorRow * 16 + 14;
      for (let cy = 0; cy < 2; cy++) {
        for (let cx = 0; cx < 8; cx++) {
          const fbIdx = ((cpy + cy) * this.fbWidth + (cpx + cx)) * 4;
          if (fbIdx + 3 < this.framebuffer.length) {
            this.framebuffer[fbIdx] = 170;
            this.framebuffer[fbIdx + 1] = 170;
            this.framebuffer[fbIdx + 2] = 170;
            this.framebuffer[fbIdx + 3] = 255;
          }
        }
      }
    }
  }

  getCharPixel(charCode: number, x: number, y: number): boolean {
    // Simple 8x16 font for printable ASCII
    // This is a minimal bitmap font
    if (charCode < 32 || charCode > 126) return false;
    
    // Use a simple pattern-based approach for common characters
    const font = BASIC_FONT[charCode - 32];
    if (!font) return false;
    
    return (font[y] & (0x80 >> x)) !== 0;
  }

  keyDown(scancode: number) {
    this.keyboardBuffer.push(scancode);
  }

  keyUp(scancode: number) {
    this.keyboardBuffer.push(scancode | 0x80);
  }

  mouseMove(x: number, y: number) {
    this.mouseX = x;
    this.mouseY = y;
  }

  mouseButton(button: number, pressed: boolean) {
    if (pressed) {
      this.mouseButtons |= (1 << button);
    } else {
      this.mouseButtons &= ~(1 << button);
    }
  }

  loadBios(data: Uint8Array) {
    const start = 0x100000 - data.length;
    for (let i = 0; i < data.length; i++) {
      if (start + i < this.memorySize) {
        this.memory[start + i] = data[i];
      }
    }
  }

  loadBootSector(data: Uint8Array) {
    for (let i = 0; i < Math.min(data.length, 512); i++) {
      this.memory[0x7C00 + i] = data[i];
    }
    this.regs.cs = 0;
    this.eip = 0x7C00;
  }

  loadISO(data: Uint8Array) {
    this.isoImage = data;
  }

  loadDisk(data: Uint8Array) {
    this.diskImage = data;
  }

  getFramebuffer(): Uint8Array {
    // Return a copy to prevent external mutation
    // For performance-critical paths, consider using transferable objects
    const copy = new Uint8Array(this.framebuffer.length);
    copy.set(this.framebuffer);
    return copy;
  }

  getFramebufferWidth(): number {
    return this.fbWidth;
  }

  getFramebufferHeight(): number {
    return this.fbHeight;
  }

  getCPUState(): string {
    return `EAX=${this.regs.eax.toString(16).padStart(8,'0')} EBX=${this.regs.ebx.toString(16).padStart(8,'0')} ECX=${this.regs.ecx.toString(16).padStart(8,'0')} EDX=${this.regs.edx.toString(16).padStart(8,'0')}\nESI=${this.regs.esi.toString(16).padStart(8,'0')} EDI=${this.regs.edi.toString(16).padStart(8,'0')} EBP=${this.regs.ebp.toString(16).padStart(8,'0')} ESP=${this.regs.esp.toString(16).padStart(8,'0')}\nEIP=${this.eip.toString(16).padStart(8,'0')} CS=${this.regs.cs.toString(16).padStart(4,'0')} DS=${this.regs.ds.toString(16).padStart(4,'0')} SS=${this.regs.ss.toString(16).padStart(4,'0')}\nFLAGS=${this.flags.raw().toString(16).padStart(8,'0')}`;
  }

  isHalted(): boolean {
    return this.halted;
  }

  setBootOrder(order: number) {
    // Store boot order
  }
}

// Basic 8x16 font for ASCII 32-126
// Each character is 16 bytes (one per row), 8 pixels wide
const BASIC_FONT: number[][] = [];

// Initialize with basic patterns
for (let i = 0; i < 95; i++) {
  BASIC_FONT.push(new Array(16).fill(0));
}

// Space (0) - already empty

// ! (1) 
BASIC_FONT[1] = [0,0,0,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0,0x18,0x18,0,0,0];

// " (2)
BASIC_FONT[2] = [0,0,0x66,0x66,0x66,0,0,0,0,0,0,0,0,0,0,0];

// # (3)
BASIC_FONT[3] = [0,0,0x6C,0x6C,0xFE,0x6C,0x6C,0x6C,0xFE,0x6C,0x6C,0,0,0,0,0];

// Numbers 0-9
BASIC_FONT[16] = [0,0,0x3C,0x66,0x6E,0x7E,0x76,0x66,0x66,0x66,0x3C,0,0,0,0,0]; // 0
BASIC_FONT[17] = [0,0,0x18,0x38,0x18,0x18,0x18,0x18,0x18,0x18,0x7E,0,0,0,0,0]; // 1
BASIC_FONT[18] = [0,0,0x3C,0x66,0x06,0x06,0x0C,0x18,0x30,0x60,0x7E,0,0,0,0,0]; // 2
BASIC_FONT[19] = [0,0,0x3C,0x66,0x06,0x06,0x1C,0x06,0x06,0x66,0x3C,0,0,0,0,0]; // 3
BASIC_FONT[20] = [0,0,0x0C,0x1C,0x3C,0x6C,0xCC,0xFE,0x0C,0x0C,0x0C,0,0,0,0,0]; // 4
BASIC_FONT[21] = [0,0,0x7E,0x60,0x60,0x7C,0x06,0x06,0x06,0x66,0x3C,0,0,0,0,0]; // 5
BASIC_FONT[22] = [0,0,0x3C,0x66,0x60,0x60,0x7C,0x66,0x66,0x66,0x3C,0,0,0,0,0]; // 6
BASIC_FONT[23] = [0,0,0x7E,0x06,0x06,0x0C,0x18,0x18,0x18,0x18,0x18,0,0,0,0,0]; // 7
BASIC_FONT[24] = [0,0,0x3C,0x66,0x66,0x66,0x3C,0x66,0x66,0x66,0x3C,0,0,0,0,0]; // 8
BASIC_FONT[25] = [0,0,0x3C,0x66,0x66,0x66,0x3E,0x06,0x06,0x66,0x3C,0,0,0,0,0]; // 9

// Uppercase letters
BASIC_FONT[33] = [0,0,0x3C,0x66,0x66,0x7E,0x66,0x66,0x66,0x66,0x66,0,0,0,0,0]; // A
BASIC_FONT[34] = [0,0,0x7C,0x66,0x66,0x7C,0x66,0x66,0x66,0x66,0x7C,0,0,0,0,0]; // B
BASIC_FONT[35] = [0,0,0x3C,0x66,0x60,0x60,0x60,0x60,0x60,0x66,0x3C,0,0,0,0,0]; // C
BASIC_FONT[36] = [0,0,0x78,0x6C,0x66,0x66,0x66,0x66,0x66,0x6C,0x78,0,0,0,0,0]; // D
BASIC_FONT[37] = [0,0,0x7E,0x60,0x60,0x7C,0x60,0x60,0x60,0x60,0x7E,0,0,0,0,0]; // E
BASIC_FONT[38] = [0,0,0x7E,0x60,0x60,0x7C,0x60,0x60,0x60,0x60,0x60,0,0,0,0,0]; // F
BASIC_FONT[40] = [0,0,0x3C,0x66,0x60,0x60,0x6E,0x66,0x66,0x66,0x3E,0,0,0,0,0]; // G
BASIC_FONT[43] = [0,0,0x66,0x66,0x66,0x7E,0x66,0x66,0x66,0x66,0x66,0,0,0,0,0]; // H
BASIC_FONT[44] = [0,0,0x3C,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3C,0,0,0,0,0]; // I
BASIC_FONT[45] = [0,0,0x1E,0x0C,0x0C,0x0C,0x0C,0x0C,0x6C,0x6C,0x38,0,0,0,0,0]; // J
BASIC_FONT[46] = [0,0,0x66,0x6C,0x78,0x70,0x78,0x6C,0x66,0x66,0x66,0,0,0,0,0]; // K
BASIC_FONT[47] = [0,0,0x60,0x60,0x60,0x60,0x60,0x60,0x60,0x60,0x7E,0,0,0,0,0]; // L
BASIC_FONT[48] = [0,0,0xC6,0xEE,0xFE,0xD6,0xC6,0xC6,0xC6,0xC6,0xC6,0,0,0,0,0]; // M
BASIC_FONT[49] = [0,0,0x66,0x76,0x7E,0x7E,0x6E,0x66,0x66,0x66,0x66,0,0,0,0,0]; // N
BASIC_FONT[50] = [0,0,0x3C,0x66,0x66,0x66,0x66,0x66,0x66,0x66,0x3C,0,0,0,0,0]; // O
BASIC_FONT[51] = [0,0,0x7C,0x66,0x66,0x66,0x7C,0x60,0x60,0x60,0x60,0,0,0,0,0]; // P
BASIC_FONT[52] = [0,0,0x3C,0x66,0x66,0x66,0x66,0x66,0x66,0x6C,0x3E,0,0,0,0,0]; // Q (approx)
BASIC_FONT[53] = [0,0,0x7C,0x66,0x66,0x66,0x7C,0x6C,0x66,0x66,0x66,0,0,0,0,0]; // R
BASIC_FONT[54] = [0,0,0x3C,0x66,0x60,0x60,0x3C,0x06,0x06,0x66,0x3C,0,0,0,0,0]; // S
BASIC_FONT[55] = [0,0,0x7E,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0,0,0,0,0]; // T
BASIC_FONT[56] = [0,0,0x66,0x66,0x66,0x66,0x66,0x66,0x66,0x66,0x3C,0,0,0,0,0]; // U
BASIC_FONT[57] = [0,0,0x66,0x66,0x66,0x66,0x66,0x66,0x66,0x3C,0x18,0,0,0,0,0]; // V
BASIC_FONT[58] = [0,0,0xC6,0xC6,0xC6,0xC6,0xD6,0xD6,0xFE,0xEE,0x6C,0,0,0,0,0]; // W
BASIC_FONT[59] = [0,0,0x66,0x66,0x3C,0x3C,0x3C,0x3C,0x3C,0x66,0x66,0,0,0,0,0]; // X
BASIC_FONT[60] = [0,0,0x66,0x66,0x66,0x3C,0x18,0x18,0x18,0x18,0x18,0,0,0,0,0]; // Y
BASIC_FONT[61] = [0,0,0x7E,0x06,0x0C,0x18,0x30,0x60,0x60,0x60,0x7E,0,0,0,0,0]; // Z

// Lowercase letters (similar to uppercase but shifted down)
for (let i = 0; i < 26; i++) {
  const upper = BASIC_FONT[33 + i];
  const lower = upper.map(row => row >> 1);
  BASIC_FONT[65 + i] = lower;
}

// Common symbols
BASIC_FONT[13] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0xFF]; // - (underscore)
BASIC_FONT[14] = [0,0,0x18,0x18,0x18,0,0,0,0,0,0,0,0,0,0,0]; // .
BASIC_FONT[15] = [0,0,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0,0,0,0,0,0,0]; // /
BASIC_FONT[16-32+95] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]; // (handled above)

// Colon
BASIC_FONT[27] = [0,0,0,0x18,0x18,0,0,0x18,0x18,0,0,0,0,0,0,0]; // :

// Period
BASIC_FONT[14] = [0,0,0,0,0,0,0,0,0,0,0x18,0x18,0,0,0,0]; // .
