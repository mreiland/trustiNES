pub enum AddressMode {
    Absolute = 1, AbsoluteX, AbsoluteY, Accumulator, Immediate, Implied, Indirect, IndexedIndirect, IndirectIndexed, Relative,
    ZeroPage, ZeroPageX, ZeroPageY,
}

pub enum OpcodeClass {
    ADC = 1, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY,
    EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED,
    SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,

    // illegal opcodes
    ILL_AHX, ILL_ALR, ILL_ANC, ILL_ARR, ILL_AXS, ILL_DCP, ILL_ISC, ILL_KIL, ILL_LAS, ILL_LAX1, ILL_LAX2, ILL_NOP,
    ILL_RLA, ILL_RRA, ILL_SAX, ILL_SBC, ILL_SHX, ILL_SHY, ILL_SLO, ILL_SRE, ILL_TAS, ILL_XAA,
}

#[derive(Default)]
pub struct OpcodeDebugInfo {
    pub opcode: u16,
    pub name: String,
    pub address_mode_name: String,
    pub notes: String,
}

#[derive(Default)]
pub struct OpcodeExecInfo {
    pub opcode: u16,
    pub len: u8,
    pub cycles: u8,
    pub page_cycles: u8,
}
