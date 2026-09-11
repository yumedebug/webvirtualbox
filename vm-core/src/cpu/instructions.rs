#[derive(Debug)]
pub enum Instruction {
    NOP,
    HLT,
    
    // MOV
    MovReg8Imm8(u8, u8),
    MovReg16Imm16(u8, u16),
    MovRM8R8(u8, u8),
    MovRM16R16(u8, u8),
    MovR8RM8(u8, u8),
    MovR16RM16(u8, u8),
    MovRM8Imm8(u8, u8),
    MovRM16Imm16(u8, u16),
    MovMem8R8(u32, u8),
    MovMem16R16(u32, u8),
    MovR8Mem8(u8, u32),
    MovR16Mem16(u8, u32),
    MovMem8Imm8(u32, u8),
    MovMem16Imm16(u32, u16),
    MovALMem8(u32),
    MovAXMem16(u32),
    MovMem8AL(u32),
    MovMem16AX(u32),
    MovSregR16(u8, u8),
    MovSregMem16(u8, u32),
    MovRM16Sreg(u8, u8),
    MovMem16Sreg(u32, u8),
    
    // ADD
    AddRM8R8(u8, u8),
    AddRM16R16(u8, u8),
    AddR8RM8(u8, u8),
    AddR16RM16(u8, u8),
    AddALImm8(u8),
    AddAXImm16(u16),
    AddMem8R8(u32, u8),
    AddMem16R16(u32, u8),
    AddR8Mem8(u8, u32),
    AddR16Mem16(u8, u32),
    
    // SUB
    SubRM16R16(u8, u8),
    SubR16RM16(u8, u8),
    SubALImm8(u8),
    SubAXImm16(u16),
    SubMem16R16(u32, u8),
    SubR16Mem16(u8, u32),
    
    // CMP
    CmpRM8R8(u8, u8),
    CmpRM16R16(u8, u8),
    CmpALImm8(u8),
    CmpAXImm16(u16),
    CmpMem8R8(u32, u8),
    CmpMem16R16(u32, u8),
    
    // INC/DEC
    IncR16(u8),
    DecR16(u8),
    
    // PUSH/POP
    PushR16(u8),
    PopR16(u8),
    PushImm16(u16),
    
    // JMP
    JmpRel8(i8),
    JmpRel16(i16),
    JmpFar(u16, u16),
    
    // Conditional jumps
    JzRel8(i8),
    JnzRel8(i8),
    JcRel8(i8),
    JncRel8(i8),
    JsRel8(i8),
    JnsRel8(i8),
    JcxzRel8(i8),
    
    // CALL/RET
    CallRel16(i16),
    Ret,
    
    // INT
    Int(u8),
    Iret,
    
    // Flags
    Cli,
    Sti,
    Cld,
    Std,
    
    // I/O
    InALImm8(u8),
    InAXImm8(u8),
    InALDX,
    InAXDX,
    OutImm8AL(u8),
    OutImm8AX(u8),
    OutDXAL,
    OutDXAX,
    
    // Logic
    XorRM16R16(u8, u8),
    XorR16RM16(u8, u8),
    XorMem16R16(u32, u8),
    XorR16Mem16(u8, u32),
    AndRM16R16(u8, u8),
    AndMem16R16(u32, u8),
    OrRM16R16(u8, u8),
    OrMem16R16(u32, u8),
    
    // String operations
    Movsb,
    Movsw,
    Stosb,
    Stosw,
    Cmpsb,
    RepMovsb,
    RepMovsw,
    RepStosb,
    RepStosw,
    RepCmpsb,
    
    // Load effective address
    Lea(u8, u32),
    Les(u8, u32),
    Lds(u8, u32),
    
    // Group 1
    Group1RM8Imm8(u8, u8, u8),
    Group1RM16Imm16(u8, u8, u16),
    Group1RM16Imm8(u8, u8, u16),
    Group1Mem8Imm8(u8, u32, u8),
    Group1Mem16Imm16(u8, u32, u16),
    Group1Mem16Imm8(u8, u32, u16),
    
    // Group 2
    Group2RM8Imm(u8, u8, u8),
    Group2RM16Imm(u8, u8, u8),
    Group2RM8CL(u8, u8),
    Group2RM16CL(u8, u8),
    Group2Mem8Imm(u8, u32, u8),
    Group2Mem16Imm(u8, u32, u8),
    
    // Group 3
    TestRM8Imm8(u8, u8),
    TestRM16Imm16(u8, u16),
    TestMem8Imm8(u32, u8),
    TestMem16Imm16(u32, u16),
    Group3RM8(u8, u8),
    Group3RM16(u8, u8),
    Group3Mem8(u8, u32),
    Group3Mem16(u8, u32),
    
    // Group 4
    Group4RM8(u8, u8),
    Group4Mem8(u8, u32),
    
    // Group 5
    Group5RM16(u8, u8),
    Group5Mem16(u8, u32),
    
    // Misc
    XchgAXR16(u8),
    Cbw,
    Cwd,
    LoopRel8(i8),
    LoopzRel8(i8),
    LoopnzRel8(i8),
    
    Unknown(u8),
}
