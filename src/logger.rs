use std::fs::File;
use std::io::Write;
use std::path::Path;
use cpu::CpuState;
use memory::Memory;
use cpu::AddressMode;
use std::fmt::UpperHex;

use std::vec;
use cpu::OpcodeDebugInfo;

//collections::vec::Vec<cpu::common_defs::OpcodeDebugInfo>

pub struct NesTest {
    f:File,
    op_info: Vec<OpcodeDebugInfo>,
}

impl NesTest {
    pub fn new<P:AsRef<Path>>(file_path: P, opcode_info:Vec<OpcodeDebugInfo>) -> NesTest {
        NesTest {
            f : File::create(&file_path).unwrap(),
            op_info: opcode_info,
        }
    }

    pub fn log_after_fetch(self: &mut NesTest, cpu_state: &CpuState, mem: &Memory) {
        let pc = cpu_state.pc-1;
        let dr = &cpu_state.decode_register;
        let len = cpu_state.decode_register.info.len;
        let opcode = cpu_state.instruction_register;
        let info = &self.op_info[opcode as usize];

        let mut s:String = format!("{:X}  ",pc).to_owned();

        match len {
            1 => { s.push_str(&format!("{:X}           ",cpu_state.instruction_register)) },
            2 => { s.push_str(&format!("{:X} {:X}      ",cpu_state.instruction_register,mem.read8(pc+1).unwrap())) },
            3 => { s.push_str(&format!("{:X} {:X} {:X} ",cpu_state.instruction_register,mem.read8(pc+1).unwrap(),mem.read8(pc+2).unwrap())) },
            _ => panic!("instructions should have a length of 1, 2, or 3.")
        }


        match dr.info.address_mode {
            // no explicit addresses for the following modes
            AddressMode::Accumulator  => { panic!("Accumulator addressing mode is unimplemented"); },
            AddressMode::Implied      => { panic!("Implied addressing mode is unimplemented"); },

            //explicit addresses from here on out
            AddressMode::Absolute =>        {
                s.push_str(&format!("{} ${:X}",info.name,mem.read16(pc+1).unwrap()));
            },
            AddressMode::AbsoluteX       => { self.f.write(s.as_bytes()); panic!("AbsoluteX addressing mode is unimplemented"); },
            AddressMode::AbsoluteY       => { self.f.write(s.as_bytes()); panic!("AbsoluteY addressing mode is unimplemented"); },
            AddressMode::Immediate       => { self.f.write(s.as_bytes()); panic!("Immediate addressing mode is unimplemented"); },
            AddressMode::Indirect        => { self.f.write(s.as_bytes()); panic!("Indirect addressing mode is unimplemented"); },
            AddressMode::IndexedIndirect => { self.f.write(s.as_bytes()); panic!("IndexedIndirect addressing mode is unimplemented"); },
            AddressMode::IndirectIndexed => { self.f.write(s.as_bytes()); panic!("IndirectIndexed addressing mode is unimplemented"); },
            AddressMode::Relative        => { self.f.write(s.as_bytes()); panic!("Relative addressing mode is unimplemented"); },
            AddressMode::ZeroPage        => { self.f.write(s.as_bytes()); panic!("ZeroPage addressing mode is unimplemented"); },
            AddressMode::ZeroPageX       => { self.f.write(s.as_bytes()); panic!("ZeroPageX addressing mode is unimplemented"); },
            AddressMode::ZeroPageY       => { self.f.write(s.as_bytes()); panic!("ZeroPageY addressing mode is unimplemented"); },
            _ => { self.f.write(s.as_bytes()); panic!("unrecognized addressing mode") }
        }
        let len = s.len();
        s.push_str(&format!("{output:>0$}",48-len,output="")); // spacing
        s.push_str(&format!("A:{:0>2X}",cpu_state.a));
        s.push_str(&format!(" X:{:0>2X}",cpu_state.x));
        s.push_str(&format!(" Y:{:0>2X}",cpu_state.y));
        s.push_str(&format!(" P:{:0>2X}",cpu_state.unpack_flags()));
        s.push_str(&format!(" SP:{:0>2X}\n",cpu_state.sp));

        self.f.write(s.as_bytes());
    }
}

