extern crate csv;

use std::io;
use std::path::Path;
use std::num;
use cpu::common_defs::OpcodeExecInfo;
use cpu::common_defs::OpcodeDebugInfo;
use cpu::common_defs::opcode_class;
use cpu::common_defs::address_mode;

#[derive(Debug)]
pub enum OpcodeLoadError {
  Io(io::Error),
  CSV(csv::Error),
  ParseInt(num::ParseIntError),
  ParseOpcodeClass(opcode_class::ParseError),
  ParseAddressMode(address_mode::ParseError),
  DuplicateOpcode(String),
  TooManyOpcodes
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
impl From<opcode_class::ParseError> for OpcodeLoadError {
    fn from(err:opcode_class::ParseError) -> OpcodeLoadError {
        OpcodeLoadError::ParseOpcodeClass(err)
    }
}
impl From<address_mode::ParseError> for OpcodeLoadError {
    fn from(err:address_mode::ParseError) -> OpcodeLoadError {
        OpcodeLoadError::ParseAddressMode(err)
    }
}

pub fn load_from_file<P:AsRef<Path>>(file_path: P) -> Result<(Vec<OpcodeExecInfo>,Vec<OpcodeDebugInfo>),OpcodeLoadError> {
    let mut rdr = try!(csv::Reader::from_file(file_path));
    rdr = rdr.has_headers(true);
    rdr = rdr.flexible(false); // all records are the same length

    let max_opcode_num = 256;

    let mut exec_info_vec = Vec::with_capacity(max_opcode_num);
    let mut debug_info_vec = Vec::with_capacity(max_opcode_num);
    let mut duplicate_check = Vec::<u8>::new();

    unsafe {
        exec_info_vec.set_len( max_opcode_num);
        debug_info_vec.set_len(max_opcode_num);
    }

    let mut count = 0;

    for rec in rdr.decode() {
        if count > max_opcode_num {
            return Err(OpcodeLoadError::TooManyOpcodes);
        }

        let (opcode_string,name,address_mode_name,len,cycles,page_cycles,notes) : (String,String,String,u8,u8,u8,String) = rec.unwrap();
        let opcode = try!(u8::from_str_radix(&opcode_string[2..],16)); // from_str_radix won't parse 0x

        if duplicate_check.contains(&opcode) {
            return Err(OpcodeLoadError::DuplicateOpcode(format!("{:X}",opcode)));
        }
        duplicate_check.push(opcode);

        let debug_info = OpcodeDebugInfo { opcode : opcode, name : name.trim().to_string(), address_mode_name : address_mode_name.trim().to_string(), notes : notes, };

        let address_mode = try!(debug_info.address_mode_name.parse::<address_mode::AddressMode>());
        let opcode_class = try!(name.trim().to_string().parse::<opcode_class::OpcodeClass>());

        let exec_info = OpcodeExecInfo { opcode: opcode, len: len, cycles: cycles, page_cycles: page_cycles, address_mode: address_mode,opcode_class:opcode_class };

        debug_info_vec[opcode as usize] = debug_info;
        exec_info_vec[opcode as usize] = exec_info;

        count = count+1;
    }
    Ok((exec_info_vec,debug_info_vec))
}

