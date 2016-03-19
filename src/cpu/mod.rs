extern crate csv;

use std::io;
use std::path::Path;
use std::num;
use std::collections::HashMap;

pub struct DebugInfo {
    opcode:u16,
    name:String,
    address_mode_name:String,
    notes:String,
}
pub struct ExecInfo {
    opcode:u16,
    len:u8,
    cycles:u8,
    page_cycles:u8,
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


#[derive(Debug)]
pub enum OpcodeLoadError {
  Io(io::Error),
  CSV(csv::Error),
  ParseInt(num::ParseIntError),
}

impl From<csv::Error> for OpcodeLoadError {
    fn from(err:csv::Error) -> OpcodeLoadError {
        OpcodeLoadError::CSV(err)
    }
}
impl From<num::ParseIntError> for OpcodeLoadError {
    fn from(err:num::ParseIntError) -> OpcodeLoadError {
        OpcodeLoadError::ParseInt(err)
    }
}

pub fn load_opcodes<P:AsRef<Path>>(file_path: P) -> Result<(HashMap<u16,ExecInfo>,HashMap<u16,DebugInfo>),OpcodeLoadError> {
    let mut rdr = try!(csv::Reader::from_file(file_path));
    rdr = rdr.has_headers(true);
    rdr = rdr.flexible(false); // all records are the same length

    let mut exec_info_hash = HashMap::<u16,ExecInfo>::new();
    let mut debug_info_hash = HashMap::<u16,DebugInfo>::new();

    for rec in rdr.decode() {
        let (opcodeString,name,address_mode_name,len,cycles,page_cycles,notes) : (String,String,String,u8,u8,u8,String) = rec.unwrap();
        let opcode = try!(u16::from_str_radix(&opcodeString[2..],16)); // from_str_radix won't parse 0x

        let mut debug_info = DebugInfo { opcode : opcode, name : name, address_mode_name : address_mode_name, notes : notes, };
        debug_info_hash.insert(opcode,debug_info);

        let mut exec_info = ExecInfo { opcode: opcode, len: len, cycles: cycles, page_cycles: page_cycles, };
        exec_info_hash.insert(opcode,exec_info);
    }
    Ok((exec_info_hash,debug_info_hash))
}

