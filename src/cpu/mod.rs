pub struct ExecInfo {
    stub: bool,
}
pub struct DecodeInfo {
    addr_init: u16,
    addr_intermediate: u16,
    addr_final: u16,

    value_init: u8,
    value_intermediate: u8,
    value_final: u8,

    info: ExecInfo,
}

#[allow(non_snake_case)]
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

impl CpuState {
  fn unpack_flags(self: &CpuState) -> u8 {
        ( (self.C as u8) << 0)
      | ( (self.Z as u8) << 1)
      | ( (self.I as u8) << 2)
      | ( (self.D as u8) << 3)
      | ( (self.B as u8) << 4)
      | ( (1           ) << 5) // flag 5 is unused
      | ( (self.V as u8) << 6)
      | ( (self.S as u8) << 7)
  }
  fn pack_flags(self: &mut CpuState,flags:u8) {
    self.C        = ( (flags >> 0) & 1) == 1;
    self.Z        = ( (flags >> 1) & 1) == 1; 
    self.I        = ( (flags >> 2) & 1) == 1; 
    self.D        = ( (flags >> 3) & 1) == 1; 
    self.B        = ( (flags >> 4) & 1) == 1; 
    // flags 5 is unused
    self.V        = ( (flags >> 6) & 1) == 1; 
    self.S        = ( (flags >> 7) & 1) == 1; 
  }
}
