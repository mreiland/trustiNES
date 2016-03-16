pub struct DecodeInfo {
    stub:bool,
}

pub struct CpuState {
    pc:u16,
    sp: u8,
    a:  u8,
    x:  u8,
    y:  u8,

    // flags
    C: bool, // carry
    Z: bool, // zero
    I: bool, // interrupt
    D: bool, // decimal
    B: bool, // break
             // bit 5 is not used by the nes and is always 1
    V: bool, // overflow
    S: bool, // sign/negative

    // these are not strictly 6502 registers, but are useful for modeling the cpu
    instruction_register: u16,
    decode_register:DecodeInfo,
}

