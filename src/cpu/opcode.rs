extern crate csv;

use std;
use std::io;
use std::path::Path;
use std::num;
use std::collections::HashMap;
pub use cpu::common_defs::OpcodeExecInfo;
pub use cpu::common_defs::OpcodeDebugInfo;


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

pub fn load_from_file<P:AsRef<Path>>(file_path: P) -> Result<(Vec<OpcodeExecInfo>,Vec<OpcodeDebugInfo>),OpcodeLoadError> {
    let mut rdr = try!(csv::Reader::from_file(file_path));
    rdr = rdr.has_headers(true);
    rdr = rdr.flexible(false); // all records are the same length

    let mut exec_info_hash = Vec::<OpcodeExecInfo>::new();
    let mut debug_info_hash = Vec::<OpcodeDebugInfo>::new();

    for rec in rdr.decode() {
        let (opcodeString,name,address_mode_name,len,cycles,page_cycles,notes) : (String,String,String,u8,u8,u8,String) = rec.unwrap();
        let opcode = try!(u16::from_str_radix(&opcodeString[2..],16)); // from_str_radix won't parse 0x

        let mut debug_info = OpcodeDebugInfo { opcode : opcode, name : name, address_mode_name : address_mode_name, notes : notes, };
        debug_info_hash.push(debug_info);

        let mut exec_info = OpcodeExecInfo { opcode: opcode, len: len, cycles: cycles, page_cycles: page_cycles, };
        exec_info_hash.push(exec_info);
    }
    Ok((exec_info_hash,debug_info_hash))
}
