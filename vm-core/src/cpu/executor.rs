use super::CPU;
use super::instructions::Instruction;
use super::super::memory::Memory;
use super::super::devices::DeviceManager;

pub fn execute(cpu: &mut CPU, memory: &mut Memory, devices: &mut DeviceManager, instruction: &Instruction, length: usize) {
    match instruction {
        Instruction::NOP => {
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::HLT => {
            cpu.halted = true;
        }
        
        // MOV reg8, imm8
        Instruction::MovReg8Imm8(reg, imm) => {
            cpu.regs.set_reg8(*reg, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // MOV reg16, imm16
        Instruction::MovReg16Imm16(reg, imm) => {
            cpu.regs.set_reg16(*reg, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // MOV r/m8, r8
        Instruction::MovRM8R8(rm, reg) => {
            let value = cpu.regs.get_reg8(*reg);
            cpu.regs.set_reg8(*rm, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovRM16R16(rm, reg) => {
            let value = cpu.regs.get_reg16(*reg);
            cpu.regs.set_reg16(*rm, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovR8RM8(reg, rm) => {
            let value = cpu.regs.get_reg8(*rm);
            cpu.regs.set_reg8(*reg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovR16RM16(reg, rm) => {
            let value = cpu.regs.get_reg16(*rm);
            cpu.regs.set_reg16(*reg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovRM8Imm8(rm, imm) => {
            cpu.regs.set_reg8(*rm, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovRM16Imm16(rm, imm) => {
            cpu.regs.set_reg16(*rm, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovALMem8(addr) => {
            let phys = (cpu.regs.ds as u32) << 4 | addr;
            let value = memory.read_byte(phys as usize);
            cpu.regs.set_al(value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovAXMem16(addr) => {
            let phys = (cpu.regs.ds as u32) << 4 | addr;
            let value = memory.read_word(phys as usize);
            cpu.regs.set_ax(value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem8AL(addr) => {
            let phys = (cpu.regs.ds as u32) << 4 | addr;
            let value = cpu.regs.al();
            memory.write_byte(phys as usize, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem16AX(addr) => {
            let phys = (cpu.regs.ds as u32) << 4 | addr;
            let value = cpu.regs.ax();
            memory.write_word(phys as usize, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem8R8(addr, reg) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = cpu.regs.get_reg8(*reg);
            memory.write_byte(phys as usize, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem16R16(addr, reg) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = cpu.regs.get_reg16(*reg);
            memory.write_word(phys as usize, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovR8Mem8(reg, addr) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = memory.read_byte(phys as usize);
            cpu.regs.set_reg8(*reg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovR16Mem16(reg, addr) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = memory.read_word(phys as usize);
            cpu.regs.set_reg16(*reg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem8Imm8(addr, imm) => {
            let phys = get_phys_addr(cpu, *addr);
            memory.write_byte(phys as usize, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem16Imm16(addr, imm) => {
            let phys = get_phys_addr(cpu, *addr);
            memory.write_word(phys as usize, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovSregR16(sreg, rm) => {
            let value = cpu.regs.get_reg16(*rm);
            set_segment_reg(cpu, *sreg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovSregMem16(sreg, addr) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = memory.read_word(phys as usize);
            set_segment_reg(cpu, *sreg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovRM16Sreg(rm, sreg) => {
            let value = get_segment_reg(cpu, *sreg);
            cpu.regs.set_reg16(*rm, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::MovMem16Sreg(addr, sreg) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = get_segment_reg(cpu, *sreg);
            memory.write_word(phys as usize, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // ADD
        Instruction::AddALImm8(imm) => {
            let result = (cpu.regs.al() as u16) + (*imm as u16);
            cpu.flags.set_cf(result > 0xFF);
            cpu.regs.set_al(result as u8);
            cpu.flags.update_flags_8(result as u8);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::AddAXImm16(imm) => {
            let result = (cpu.regs.ax() as u32) + (*imm as u32);
            cpu.flags.set_cf(result > 0xFFFF);
            cpu.regs.set_ax(result as u16);
            cpu.flags.update_flags_16(result as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::AddRM16R16(rm, reg) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = cpu.regs.get_reg16(*reg);
            let result = (a as u32) + (b as u32);
            cpu.flags.set_cf(result > 0xFFFF);
            cpu.regs.set_reg16(*rm, result as u16);
            cpu.flags.update_flags_16(result as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::AddR16RM16(reg, rm) => {
            let a = cpu.regs.get_reg16(*reg);
            let b = cpu.regs.get_reg16(*rm);
            let result = (a as u32) + (b as u32);
            cpu.flags.set_cf(result > 0xFFFF);
            cpu.regs.set_reg16(*reg, result as u16);
            cpu.flags.update_flags_16(result as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // SUB
        Instruction::SubALImm8(imm) => {
            let a = cpu.regs.al();
            let result = (a as i16) - (*imm as i16);
            cpu.flags.set_cf(a < *imm);
            cpu.regs.set_al(result as u8);
            cpu.flags.update_flags_8(result as u8);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::SubAXImm16(imm) => {
            let a = cpu.regs.ax();
            let result = (a as i32) - (*imm as i32);
            cpu.flags.set_cf(a < *imm);
            cpu.regs.set_ax(result as u16);
            cpu.flags.update_flags_16(result as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::SubRM16R16(rm, reg) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = cpu.regs.get_reg16(*reg);
            cpu.flags.set_cf(a < b);
            let result = a.wrapping_sub(b);
            cpu.regs.set_reg16(*rm, result);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::SubR16RM16(reg, rm) => {
            let a = cpu.regs.get_reg16(*reg);
            let b = cpu.regs.get_reg16(*rm);
            cpu.flags.set_cf(a < b);
            let result = a.wrapping_sub(b);
            cpu.regs.set_reg16(*reg, result);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // CMP
        Instruction::CmpALImm8(imm) => {
            let a = cpu.regs.al();
            cpu.flags.set_cf(a < *imm);
            let result = a.wrapping_sub(*imm);
            cpu.flags.update_flags_8(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::CmpAXImm16(imm) => {
            let a = cpu.regs.ax();
            cpu.flags.set_cf(a < *imm);
            let result = a.wrapping_sub(*imm);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::CmpRM16R16(rm, reg) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = cpu.regs.get_reg16(*reg);
            cpu.flags.set_cf(a < b);
            let result = a.wrapping_sub(b);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // INC/DEC
        Instruction::IncR16(reg) => {
            let value = cpu.regs.get_reg16(*reg);
            let result = value.wrapping_add(1);
            cpu.regs.set_reg16(*reg, result);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::DecR16(reg) => {
            let value = cpu.regs.get_reg16(*reg);
            let result = value.wrapping_sub(1);
            cpu.regs.set_reg16(*reg, result);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // PUSH/POP
        Instruction::PushR16(reg) => {
            let value = cpu.regs.get_reg16(*reg);
            push16(cpu, memory, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::PopR16(reg) => {
            let value = pop16(cpu, memory);
            cpu.regs.set_reg16(*reg, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::PushImm16(imm) => {
            push16(cpu, memory, *imm);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // JMP
        Instruction::JmpRel8(offset) => {
            cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
        }
        
        Instruction::JmpRel16(offset) => {
            cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
        }
        
        Instruction::JmpFar(segment, offset) => {
            cpu.regs.cs = *segment;
            cpu.eip = *offset as u32;
        }
        
        // Conditional jumps
        Instruction::JzRel8(offset) => {
            if cpu.flags.zf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::JnzRel8(offset) => {
            if !cpu.flags.zf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::JcRel8(offset) => {
            if cpu.flags.cf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::JncRel8(offset) => {
            if !cpu.flags.cf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::JsRel8(offset) => {
            if cpu.flags.sf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::JnsRel8(offset) => {
            if !cpu.flags.sf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::JcxzRel8(offset) => {
            if cpu.regs.cx() == 0 {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        // CALL/RET
        Instruction::CallRel16(offset) => {
            let return_addr = cpu.eip.wrapping_add(length as u32) as u16;
            push16(cpu, memory, return_addr);
            cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
        }
        
        Instruction::Ret => {
            cpu.eip = pop16(cpu, memory) as u32;
        }
        
        // INT
        Instruction::Int(interrupt) => {
            push16(cpu, memory, cpu.flags.raw() as u16);
            push16(cpu, memory, cpu.regs.cs);
            push16(cpu, memory, cpu.eip.wrapping_add(length as u32) as u16);
            
            let ivt_addr = (*interrupt as u32) * 4;
            let ip = memory.read_word(ivt_addr as usize);
            let cs = memory.read_word(ivt_addr as usize + 2);
            
            cpu.regs.cs = cs;
            cpu.eip = ip as u32;
            cpu.interrupt_flag = false;
        }
        
        Instruction::Iret => {
            cpu.eip = pop16(cpu, memory) as u32;
            cpu.regs.cs = pop16(cpu, memory);
            let flags = pop16(cpu, memory);
            cpu.flags.set_raw(flags as u32);
        }
        
        // Flags
        Instruction::Cli => {
            cpu.interrupt_flag = false;
            cpu.flags.set_if(false);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Sti => {
            cpu.interrupt_flag = true;
            cpu.flags.set_if(true);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Cld => {
            cpu.flags.set_df(false);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Std => {
            cpu.flags.set_df(true);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // I/O
        Instruction::InALImm8(port) => {
            let value = devices.io_read(*port as u16);
            cpu.regs.set_al(value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::InAXImm8(port) => {
            let lo = devices.io_read(*port as u16);
            let hi = devices.io_read((*port as u16) + 1);
            cpu.regs.set_ax(((hi as u16) << 8) | lo as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::InALDX => {
            let port = cpu.regs.dx();
            let value = devices.io_read(port);
            cpu.regs.set_al(value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::InAXDX => {
            let port = cpu.regs.dx();
            let lo = devices.io_read(port);
            let hi = devices.io_read(port + 1);
            cpu.regs.set_ax(((hi as u16) << 8) | lo as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::OutImm8AL(port) => {
            let value = cpu.regs.al();
            devices.io_write(*port as u16, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::OutImm8AX(port) => {
            let value = cpu.regs.ax();
            devices.io_write(*port as u16, value as u8);
            devices.io_write((*port as u16) + 1, (value >> 8) as u8);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::OutDXAL => {
            let port = cpu.regs.dx();
            let value = cpu.regs.al();
            devices.io_write(port, value);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::OutDXAX => {
            let port = cpu.regs.dx();
            let value = cpu.regs.ax();
            devices.io_write(port, value as u8);
            devices.io_write(port + 1, (value >> 8) as u8);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // Logic
        Instruction::XorRM16R16(rm, reg) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = cpu.regs.get_reg16(*reg);
            let result = a ^ b;
            cpu.regs.set_reg16(*rm, result);
            cpu.flags.set_cf(false);
            cpu.flags.set_of(false);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::XorR16RM16(reg, rm) => {
            let a = cpu.regs.get_reg16(*reg);
            let b = cpu.regs.get_reg16(*rm);
            let result = a ^ b;
            cpu.regs.set_reg16(*reg, result);
            cpu.flags.set_cf(false);
            cpu.flags.set_of(false);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::AndRM16R16(rm, reg) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = cpu.regs.get_reg16(*reg);
            let result = a & b;
            cpu.regs.set_reg16(*rm, result);
            cpu.flags.set_cf(false);
            cpu.flags.set_of(false);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::OrRM16R16(rm, reg) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = cpu.regs.get_reg16(*reg);
            let result = a | b;
            cpu.regs.set_reg16(*rm, result);
            cpu.flags.set_cf(false);
            cpu.flags.set_of(false);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // String operations
        Instruction::Movsb => {
            let src = ((cpu.regs.ds as u32) << 4) + cpu.regs.esi as u32;
            let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
            let value = memory.read_byte(src as usize);
            memory.write_byte(dst as usize, value);
            let dir = if cpu.flags.df() { -1i32 } else { 1 };
            cpu.regs.esi = cpu.regs.esi.wrapping_add(dir as u32);
            cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Movsw => {
            let src = ((cpu.regs.ds as u32) << 4) + cpu.regs.esi as u32;
            let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
            let value = memory.read_word(src as usize);
            memory.write_word(dst as usize, value);
            let dir = if cpu.flags.df() { -2i32 } else { 2 };
            cpu.regs.esi = cpu.regs.esi.wrapping_add(dir as u32);
            cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Stosb => {
            let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
            let value = cpu.regs.al();
            memory.write_byte(dst as usize, value);
            let dir = if cpu.flags.df() { -1i32 } else { 1 };
            cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Stosw => {
            let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
            let value = cpu.regs.ax();
            memory.write_word(dst as usize, value);
            let dir = if cpu.flags.df() { -2i32 } else { 2 };
            cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::RepMovsb | Instruction::RepMovsw | Instruction::RepStosb | Instruction::RepStosw | Instruction::RepCmpsb => {
            if cpu.regs.cx() == 0 {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
                return;
            }
            
            match instruction {
                Instruction::RepMovsb => {
                    let src = ((cpu.regs.ds as u32) << 4) + cpu.regs.esi as u32;
                    let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
                    let value = memory.read_byte(src as usize);
                    memory.write_byte(dst as usize, value);
                    let dir = if cpu.flags.df() { -1i32 } else { 1 };
                    cpu.regs.esi = cpu.regs.esi.wrapping_add(dir as u32);
                    cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
                }
                Instruction::RepMovsw => {
                    let src = ((cpu.regs.ds as u32) << 4) + cpu.regs.esi as u32;
                    let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
                    let value = memory.read_word(src as usize);
                    memory.write_word(dst as usize, value);
                    let dir = if cpu.flags.df() { -2i32 } else { 2 };
                    cpu.regs.esi = cpu.regs.esi.wrapping_add(dir as u32);
                    cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
                }
                Instruction::RepStosb => {
                    let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
                    let value = cpu.regs.al();
                    memory.write_byte(dst as usize, value);
                    let dir = if cpu.flags.df() { -1i32 } else { 1 };
                    cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
                }
                Instruction::RepStosw => {
                    let dst = ((cpu.regs.es as u32) << 4) + cpu.regs.edi as u32;
                    let value = cpu.regs.ax();
                    memory.write_word(dst as usize, value);
                    let dir = if cpu.flags.df() { -2i32 } else { 2 };
                    cpu.regs.edi = cpu.regs.edi.wrapping_add(dir as u32);
                }
                _ => {}
            }
            
            cpu.regs.set_cx(cpu.regs.cx().wrapping_sub(1));
            if cpu.regs.cx() != 0 {
                // Repeat - don't advance EIP
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        // LEA
        Instruction::Lea(reg, addr) => {
            cpu.regs.set_reg16(*reg, *addr as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // LES
        Instruction::Les(reg, addr) => {
            let phys = get_phys_addr(cpu, *addr);
            let offset = memory.read_word(phys as usize);
            let segment = memory.read_word(phys as usize + 2);
            cpu.regs.set_reg16(*reg, offset);
            cpu.regs.es = segment;
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // LDS
        Instruction::Lds(reg, addr) => {
            let phys = get_phys_addr(cpu, *addr);
            let offset = memory.read_word(phys as usize);
            let segment = memory.read_word(phys as usize + 2);
            cpu.regs.set_reg16(*reg, offset);
            cpu.regs.ds = segment;
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // Group 1
        Instruction::Group1RM16Imm16(op, rm, imm) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = *imm;
            let result = match op {
                0 => a.wrapping_add(b), // ADD
                1 => a | b,             // OR
                2 => a.wrapping_add(b), // ADC (simplified)
                3 => a.wrapping_sub(b), // SBB (simplified)
                4 => a & b,             // AND
                5 => a.wrapping_sub(b), // SUB
                6 => a ^ b,             // XOR
                7 => a.wrapping_sub(b), // CMP
                _ => a,
            };
            if *op != 7 {
                cpu.regs.set_reg16(*rm, result);
            }
            if *op == 0 || *op == 3 || *op == 5 || *op == 7 {
                cpu.flags.set_cf(a < b);
            }
            if *op != 1 && *op != 4 && *op != 6 {
                cpu.flags.update_flags_16(result);
            } else {
                cpu.flags.update_flags_16(result);
                cpu.flags.set_cf(false);
                cpu.flags.set_of(false);
            }
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Group1RM16Imm8(op, rm, imm) => {
            let a = cpu.regs.get_reg16(*rm);
            let b = *imm;
            let result = match op {
                0 => a.wrapping_add(b),
                1 => a | b,
                4 => a & b,
                5 => a.wrapping_sub(b),
                6 => a ^ b,
                7 => a.wrapping_sub(b),
                _ => a,
            };
            if *op != 7 {
                cpu.regs.set_reg16(*rm, result);
            }
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // Group 2
        Instruction::Group2RM16Imm(op, rm, count) => {
            let value = cpu.regs.get_reg16(*rm);
            let result = match op {
                0 => value.rotate_left(*count as u32), // ROL
                1 => value.rotate_right(*count as u32), // ROR
                4 => value << *count,                   // SHL
                5 => value >> *count,                   // SHR
                7 => ((value as i16) >> *count) as u16, // SAR
                _ => value,
            };
            cpu.regs.set_reg16(*rm, result);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // Group 3
        Instruction::TestRM16Imm16(rm, imm) => {
            let a = cpu.regs.get_reg16(*rm);
            let result = a & *imm;
            cpu.flags.set_cf(false);
            cpu.flags.set_of(false);
            cpu.flags.update_flags_16(result);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Group3RM16(op, rm) => {
            let value = cpu.regs.get_reg16(*rm);
            match op {
                2 => { // NOT
                    cpu.regs.set_reg16(*rm, !value);
                }
                3 => { // NEG
                    let result = 0u16.wrapping_sub(value);
                    cpu.flags.set_cf(value != 0);
                    cpu.regs.set_reg16(*rm, result);
                    cpu.flags.update_flags_16(result);
                }
                _ => {}
            }
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // Group 5
        Instruction::Group5RM16(op, rm) => {
            match op {
                0 => { // INC r/m16
                    let value = cpu.regs.get_reg16(*rm);
                    let result = value.wrapping_add(1);
                    cpu.regs.set_reg16(*rm, result);
                    cpu.flags.update_flags_16(result);
                }
                1 => { // DEC r/m16
                    let value = cpu.regs.get_reg16(*rm);
                    let result = value.wrapping_sub(1);
                    cpu.regs.set_reg16(*rm, result);
                    cpu.flags.update_flags_16(result);
                }
                2 => { // CALL r/m16
                    let addr = cpu.regs.get_reg16(*rm);
                    let return_addr = cpu.eip.wrapping_add(length as u32) as u16;
                    push16(cpu, memory, return_addr);
                    cpu.eip = addr as u32;
                    return;
                }
                4 => { // JMP r/m16
                    let addr = cpu.regs.get_reg16(*rm);
                    cpu.eip = addr as u32;
                    return;
                }
                6 => { // PUSH r/m16
                    let value = cpu.regs.get_reg16(*rm);
                    push16(cpu, memory, value);
                }
                _ => {}
            }
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        Instruction::Group5Mem16(op, addr) => {
            let phys = get_phys_addr(cpu, *addr);
            let value = memory.read_word(phys as usize);
            match op {
                2 => { // CALL m16
                    let return_addr = cpu.eip.wrapping_add(length as u32) as u16;
                    push16(cpu, memory, return_addr);
                    cpu.eip = value as u32;
                    return;
                }
                4 => { // JMP m16
                    cpu.eip = value as u32;
                    return;
                }
                6 => { // PUSH m16
                    push16(cpu, memory, value);
                }
                _ => {}
            }
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // XCHG
        Instruction::XchgAXR16(reg) => {
            let a = cpu.regs.ax();
            let b = cpu.regs.get_reg16(*reg);
            cpu.regs.set_ax(b);
            cpu.regs.set_reg16(*reg, a);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // CBW
        Instruction::Cbw => {
            let al = cpu.regs.al() as i8;
            cpu.regs.set_ax(al as i16 as u16);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // CWD
        Instruction::Cwd => {
            let ax = cpu.regs.ax() as i16;
            let dx = if ax < 0 { 0xFFFF } else { 0 };
            cpu.regs.set_dx(dx);
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // LOOP
        Instruction::LoopRel8(offset) => {
            cpu.regs.set_cx(cpu.regs.cx().wrapping_sub(1));
            if cpu.regs.cx() != 0 {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::LoopzRel8(offset) => {
            cpu.regs.set_cx(cpu.regs.cx().wrapping_sub(1));
            if cpu.regs.cx() != 0 && cpu.flags.zf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        Instruction::LoopnzRel8(offset) => {
            cpu.regs.set_cx(cpu.regs.cx().wrapping_sub(1));
            if cpu.regs.cx() != 0 && !cpu.flags.zf() {
                cpu.eip = cpu.eip.wrapping_add(length as u32).wrapping_add(*offset as i32 as u32);
            } else {
                cpu.eip = cpu.eip.wrapping_add(length as u32);
            }
        }
        
        // Default: unknown instruction - just advance
        Instruction::Unknown(_) => {
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
        
        // Catch-all for remaining variants
        _ => {
            cpu.eip = cpu.eip.wrapping_add(length as u32);
        }
    }
}

fn get_phys_addr(cpu: &CPU, offset: u32) -> u32 {
    (cpu.regs.ds as u32) << 4 | offset
}

fn set_segment_reg(cpu: &mut CPU, reg: u8, value: u16) {
    match reg {
        0 => cpu.regs.es = value,
        1 => cpu.regs.cs = value,
        2 => cpu.regs.ss = value,
        3 => cpu.regs.ds = value,
        4 => cpu.regs.fs = value,
        5 => cpu.regs.gs = value,
        _ => {}
    }
}

fn get_segment_reg(cpu: &CPU, reg: u8) -> u16 {
    match reg {
        0 => cpu.regs.es,
        1 => cpu.regs.cs,
        2 => cpu.regs.ss,
        3 => cpu.regs.ds,
        4 => cpu.regs.fs,
        5 => cpu.regs.gs,
        _ => 0,
    }
}

fn push16(cpu: &mut CPU, memory: &mut Memory, value: u16) {
    cpu.regs.esp = cpu.regs.esp.wrapping_sub(2);
    let addr = ((cpu.regs.ss as u32) << 4) + cpu.regs.esp as u32;
    memory.write_word(addr as usize, value);
}

fn pop16(cpu: &mut CPU, memory: &mut Memory) -> u16 {
    let addr = ((cpu.regs.ss as u32) << 4) + cpu.regs.esp as u32;
    let value = memory.read_word(addr as usize);
    cpu.regs.esp = cpu.regs.esp.wrapping_add(2);
    value
}
