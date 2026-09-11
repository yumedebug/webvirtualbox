use super::instructions::Instruction;
use super::super::memory::Memory;

pub fn decode(memory: &Memory, addr: usize, opcode: u8) -> (Instruction, usize) {
    match opcode {
        // NOP
        0x90 => (Instruction::NOP, 1),
        
        // HLT
        0xF4 => (Instruction::HLT, 1),
        
        // MOV reg8, imm8
        0xB0..=0xB7 => {
            let reg = opcode - 0xB0;
            let imm = memory.read_byte(addr + 1);
            (Instruction::MovReg8Imm8(reg, imm), 2)
        }
        
        // MOV reg16, imm16
        0xB8..=0xBF => {
            let reg = opcode - 0xB8;
            let imm = memory.read_word(addr + 1);
            (Instruction::MovReg16Imm16(reg, imm), 3)
        }
        
        // MOV r/m8, r8
        0x88 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::MovRM8R8(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovMem8R8(effective_addr, reg), 2 + len)
            }
        }
        
        // MOV r/m16, r16
        0x89 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::MovRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // MOV r8, r/m8
        0x8A => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::MovR8RM8(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovR8Mem8(reg, effective_addr), 2 + len)
            }
        }
        
        // MOV r16, r/m16
        0x8B => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::MovR16RM16(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovR16Mem16(reg, effective_addr), 2 + len)
            }
        }
        
        // MOV r/m8, imm8
        0xC6 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, _reg, rm) = decode_modrm(modrm);
            let imm = memory.read_byte(addr + 2);
            if mod_ == 3 {
                (Instruction::MovRM8Imm8(rm, imm), 3)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovMem8Imm8(effective_addr, imm), 2 + len + 1)
            }
        }
        
        // MOV r/m16, imm16
        0xC7 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, _reg, rm) = decode_modrm(modrm);
            let imm = memory.read_word(addr + 2);
            if mod_ == 3 {
                (Instruction::MovRM16Imm16(rm, imm), 4)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovMem16Imm16(effective_addr, imm), 2 + len + 2)
            }
        }
        
        // MOV AL, moffs8
        0xA0 => {
            let addr16 = memory.read_word(addr + 1);
            (Instruction::MovALMem8(addr16 as u32), 3)
        }
        
        // MOV AX, moffs16
        0xA1 => {
            let addr16 = memory.read_word(addr + 1);
            (Instruction::MovAXMem16(addr16 as u32), 3)
        }
        
        // MOV moffs8, AL
        0xA2 => {
            let addr16 = memory.read_word(addr + 1);
            (Instruction::MovMem8AL(addr16 as u32), 3)
        }
        
        // MOV moffs16, AX
        0xA3 => {
            let addr16 = memory.read_word(addr + 1);
            (Instruction::MovMem16AX(addr16 as u32), 3)
        }
        
        // ADD r/m8, r8
        0x00 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::AddRM8R8(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::AddMem8R8(effective_addr, reg), 2 + len)
            }
        }
        
        // ADD r/m16, r16
        0x01 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::AddRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::AddMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // ADD r8, r/m8
        0x02 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::AddR8RM8(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::AddR8Mem8(reg, effective_addr), 2 + len)
            }
        }
        
        // ADD r16, r/m16
        0x03 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::AddR16RM16(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::AddR16Mem16(reg, effective_addr), 2 + len)
            }
        }
        
        // ADD AL, imm8
        0x04 => {
            let imm = memory.read_byte(addr + 1);
            (Instruction::AddALImm8(imm), 2)
        }
        
        // ADD AX, imm16
        0x05 => {
            let imm = memory.read_word(addr + 1);
            (Instruction::AddAXImm16(imm), 3)
        }
        
        // SUB r/m16, r16
        0x29 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::SubRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::SubMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // SUB r16, r/m16
        0x2B => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::SubR16RM16(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::SubR16Mem16(reg, effective_addr), 2 + len)
            }
        }
        
        // SUB AL, imm8
        0x2C => {
            let imm = memory.read_byte(addr + 1);
            (Instruction::SubALImm8(imm), 2)
        }
        
        // SUB AX, imm16
        0x2D => {
            let imm = memory.read_word(addr + 1);
            (Instruction::SubAXImm16(imm), 3)
        }
        
        // CMP r/m8, r8
        0x38 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::CmpRM8R8(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::CmpMem8R8(effective_addr, reg), 2 + len)
            }
        }
        
        // CMP r/m16, r16
        0x39 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::CmpRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::CmpMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // CMP AL, imm8
        0x3C => {
            let imm = memory.read_byte(addr + 1);
            (Instruction::CmpALImm8(imm), 2)
        }
        
        // CMP AX, imm16
        0x3D => {
            let imm = memory.read_word(addr + 1);
            (Instruction::CmpAXImm16(imm), 3)
        }
        
        // INC r16
        0x40..=0x47 => {
            let reg = opcode - 0x40;
            (Instruction::IncR16(reg), 1)
        }
        
        // DEC r16
        0x48..=0x4F => {
            let reg = opcode - 0x48;
            (Instruction::DecR16(reg), 1)
        }
        
        // PUSH r16
        0x50..=0x57 => {
            let reg = opcode - 0x50;
            (Instruction::PushR16(reg), 1)
        }
        
        // POP r16
        0x58..=0x5F => {
            let reg = opcode - 0x58;
            (Instruction::PopR16(reg), 1)
        }
        
        // JMP rel8
        0xEB => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JmpRel8(offset), 2)
        }
        
        // JMP rel16
        0xE9 => {
            let offset = memory.read_word(addr + 1) as i16;
            (Instruction::JmpRel16(offset), 3)
        }
        
        // JMP far
        0xEA => {
            let offset = memory.read_word(addr + 1);
            let segment = memory.read_word(addr + 3);
            (Instruction::JmpFar(segment, offset), 5)
        }
        
        // JZ/JE rel8
        0x74 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JzRel8(offset), 2)
        }
        
        // JNZ/JNE rel8
        0x75 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JnzRel8(offset), 2)
        }
        
        // JC/JB rel8
        0x72 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JcRel8(offset), 2)
        }
        
        // JNC/JAE rel8
        0x73 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JncRel8(offset), 2)
        }
        
        // JS rel8
        0x78 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JsRel8(offset), 2)
        }
        
        // JNS rel8
        0x79 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JnsRel8(offset), 2)
        }
        
        // CALL rel16
        0xE8 => {
            let offset = memory.read_word(addr + 1) as i16;
            (Instruction::CallRel16(offset), 3)
        }
        
        // RET
        0xC3 => (Instruction::Ret, 1),
        
        // INT imm8
        0xCD => {
            let interrupt = memory.read_byte(addr + 1);
            (Instruction::Int(interrupt), 2)
        }
        
        // IRET
        0xCF => (Instruction::Iret, 1),
        
        // CLI
        0xFA => (Instruction::Cli, 1),
        
        // STI
        0xFB => (Instruction::Sti, 1),
        
        // CLD
        0xFC => (Instruction::Cld, 1),
        
        // STD
        0xFD => (Instruction::Std, 1),
        
        // IN AL, imm8
        0xE4 => {
            let port = memory.read_byte(addr + 1);
            (Instruction::InALImm8(port), 2)
        }
        
        // IN AX, imm8
        0xE5 => {
            let port = memory.read_byte(addr + 1);
            (Instruction::InAXImm8(port), 2)
        }
        
        // IN AL, DX
        0xEC => (Instruction::InALDX, 1),
        
        // IN AX, DX
        0xED => (Instruction::InAXDX, 1),
        
        // OUT imm8, AL
        0xE6 => {
            let port = memory.read_byte(addr + 1);
            (Instruction::OutImm8AL(port), 2)
        }
        
        // OUT imm8, AX
        0xE7 => {
            let port = memory.read_byte(addr + 1);
            (Instruction::OutImm8AX(port), 2)
        }
        
        // OUT DX, AL
        0xEE => (Instruction::OutDXAL, 1),
        
        // OUT DX, AX
        0xEF => (Instruction::OutDXAX, 1),
        
        // XOR r/m16, r16
        0x31 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::XorRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::XorMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // XOR r16, r/m16
        0x33 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::XorR16RM16(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::XorR16Mem16(reg, effective_addr), 2 + len)
            }
        }
        
        // AND r/m16, r16
        0x21 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::AndRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::AndMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // OR r/m16, r16
        0x09 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::OrRM16R16(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::OrMem16R16(effective_addr, reg), 2 + len)
            }
        }
        
        // MOV segment register
        0x8E => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::MovSregR16(reg, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovSregMem16(reg, effective_addr), 2 + len)
            }
        }
        
        // MOV r/m16, Sreg
        0x8C => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::MovRM16Sreg(rm, reg), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::MovMem16Sreg(effective_addr, reg), 2 + len)
            }
        }
        
        // LES
        0xC4 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ != 3 {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Les(reg, effective_addr), 2 + len)
            } else {
                (Instruction::NOP, 2) // Invalid
            }
        }
        
        // LDS
        0xC5 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ != 3 {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Lds(reg, effective_addr), 2 + len)
            } else {
                (Instruction::NOP, 2) // Invalid
            }
        }
        
        // LEA
        0x8D => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, reg, rm) = decode_modrm(modrm);
            if mod_ != 3 {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Lea(reg, effective_addr), 2 + len)
            } else {
                (Instruction::NOP, 2)
            }
        }
        
        // REP prefix
        0xF3 => {
            let next_opcode = memory.read_byte(addr + 1);
            match next_opcode {
                0xA4 => (Instruction::RepMovsb, 2),
                0xA5 => (Instruction::RepMovsw, 2),
                0xAA => (Instruction::RepStosb, 2),
                0xAB => (Instruction::RepStosw, 2),
                0xA6 => (Instruction::RepCmpsb, 2),
                _ => (Instruction::NOP, 2),
            }
        }
        
        // MOVSB
        0xA4 => (Instruction::Movsb, 1),
        
        // MOVSW
        0xA5 => (Instruction::Movsw, 1),
        
        // STOSB
        0xAA => (Instruction::Stosb, 1),
        
        // STOSW
        0xAB => (Instruction::Stosw, 1),
        
        // CMPSB
        0xA6 => (Instruction::Cmpsb, 1),
        
        // Segment overrides
        0x26 => {
            // ES override - skip for now
            let next = memory.read_byte(addr + 1);
            let (inst, len) = decode(memory, addr + 1, next);
            (inst, len + 1)
        }
        0x2E => {
            let next = memory.read_byte(addr + 1);
            let (inst, len) = decode(memory, addr + 1, next);
            (inst, len + 1)
        }
        0x36 => {
            let next = memory.read_byte(addr + 1);
            let (inst, len) = decode(memory, addr + 1, next);
            (inst, len + 1)
        }
        0x3E => {
            let next = memory.read_byte(addr + 1);
            let (inst, len) = decode(memory, addr + 1, next);
            (inst, len + 1)
        }
        
        // Group 1: ADD/OR/ADC/SBB/AND/SUB/XOR/CMP r/m8, imm8
        0x80 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            let imm = memory.read_byte(addr + 2);
            if mod_ == 3 {
                (Instruction::Group1RM8Imm8(op, rm, imm), 3)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group1Mem8Imm8(op, effective_addr, imm), 2 + len + 1)
            }
        }
        
        // Group 1: ADD/OR/ADC/SBB/AND/SUB/XOR/CMP r/m16, imm16
        0x81 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            let imm = memory.read_word(addr + 2);
            if mod_ == 3 {
                (Instruction::Group1RM16Imm16(op, rm, imm), 4)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group1Mem16Imm16(op, effective_addr, imm), 2 + len + 2)
            }
        }
        
        // Group 1: ADD/OR/ADC/SBB/AND/SUB/XOR/CMP r/m16, imm8 (sign-extended)
        0x83 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            let imm = memory.read_byte(addr + 2) as i8 as i16 as u16;
            if mod_ == 3 {
                (Instruction::Group1RM16Imm8(op, rm, imm), 3)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group1Mem16Imm8(op, effective_addr, imm), 2 + len + 1)
            }
        }
        
        // Group 2: Shift/Rotate r/m8
        0xD0 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::Group2RM8Imm(op, rm, 1), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group2Mem8Imm(op, effective_addr, 1), 2 + len)
            }
        }
        
        // Group 2: Shift/Rotate r/m16
        0xD1 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::Group2RM16Imm(op, rm, 1), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group2Mem16Imm(op, effective_addr, 1), 2 + len)
            }
        }
        
        // Group 2: Shift/Rotate r/m8, CL
        0xD2 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::Group2RM8CL(op, rm), 2)
            } else {
                (Instruction::NOP, 2)
            }
        }
        
        // Group 2: Shift/Rotate r/m16, CL
        0xD3 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::Group2RM16CL(op, rm), 2)
            } else {
                (Instruction::NOP, 2)
            }
        }
        
        // Group 3: TEST/NOT/NEG/MUL/IMUL/DIV/IDIV r/m8
        0xF6 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                if op == 0 {
                    let imm = memory.read_byte(addr + 2);
                    (Instruction::TestRM8Imm8(rm, imm), 3)
                } else {
                    (Instruction::Group3RM8(op, rm), 2)
                }
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                if op == 0 {
                    let imm = memory.read_byte(addr + 2 + len);
                    (Instruction::TestMem8Imm8(effective_addr, imm), 2 + len + 1)
                } else {
                    (Instruction::Group3Mem8(op, effective_addr), 2 + len)
                }
            }
        }
        
        // Group 3: TEST/NOT/NEG/MUL/IMUL/DIV/IDIV r/m16
        0xF7 => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                if op == 0 {
                    let imm = memory.read_word(addr + 2);
                    (Instruction::TestRM16Imm16(rm, imm), 4)
                } else {
                    (Instruction::Group3RM16(op, rm), 2)
                }
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                if op == 0 {
                    let imm = memory.read_word(addr + 2 + len);
                    (Instruction::TestMem16Imm16(effective_addr, imm), 2 + len + 2)
                } else {
                    (Instruction::Group3Mem16(op, effective_addr), 2 + len)
                }
            }
        }
        
        // Group 4: INC/DEC r/m8
        0xFE => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::Group4RM8(op, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group4Mem8(op, effective_addr), 2 + len)
            }
        }
        
        // Group 5: INC/DEC/CALL/JMP/PUSH r/m16
        0xFF => {
            let modrm = memory.read_byte(addr + 1);
            let (mod_, op, rm) = decode_modrm(modrm);
            if mod_ == 3 {
                (Instruction::Group5RM16(op, rm), 2)
            } else {
                let (effective_addr, len) = decode_effective_addr(memory, addr + 2, mod_, rm);
                (Instruction::Group5Mem16(op, effective_addr), 2 + len)
            }
        }
        
        // PUSH imm8
        0x6A => {
            let imm = memory.read_byte(addr + 1) as i8 as i16;
            (Instruction::PushImm16(imm as u16), 2)
        }
        
        // PUSH imm16
        0x68 => {
            let imm = memory.read_word(addr + 1);
            (Instruction::PushImm16(imm), 3)
        }
        
        // XCHG AX, r16
        0x91..=0x97 => {
            let reg = opcode - 0x90;
            (Instruction::XchgAXR16(reg), 1)
        }
        
        // CBW
        0x98 => (Instruction::Cbw, 1),
        
        // CWD
        0x99 => (Instruction::Cwd, 1),
        
        // LOOP
        0xE2 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::LoopRel8(offset), 2)
        }
        
        // LOOPZ
        0xE1 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::LoopzRel8(offset), 2)
        }
        
        // LOOPNZ
        0xE0 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::LoopnzRel8(offset), 2)
        }
        
        // JCXZ
        0xE3 => {
            let offset = memory.read_byte(addr + 1) as i8;
            (Instruction::JcxzRel8(offset), 2)
        }
        
        // Default: unknown instruction
        _ => (Instruction::Unknown(opcode), 1),
    }
}

fn decode_modrm(modrm: u8) -> (u8, u8, u8) {
    let mod_ = (modrm >> 6) & 0x03;
    let reg = (modrm >> 3) & 0x07;
    let rm = modrm & 0x07;
    (mod_, reg, rm)
}

fn decode_effective_addr(memory: &Memory, addr: usize, mod_: u8, rm: u8) -> (u32, usize) {
    match (mod_, rm) {
        (0, 0) => {
            let bx = memory.read_word(0) as u32; // placeholder
            (bx, 0) // [BX+SI]
        }
        (0, 6) => {
            let disp = memory.read_word(addr) as u32;
            (disp, 2)
        }
        (1, 6) => {
            let disp = memory.read_byte(addr) as i8 as u16 as u32;
            (disp, 1)
        }
        (2, 6) => {
            let disp = memory.read_word(addr) as u32;
            (disp, 2)
        }
        _ => {
            let disp = memory.read_word(addr) as u32;
            (disp, 2)
        }
    }
}
