use cpu::common_defs::OpcodeExecInfo;
use cpu::CpuState;
use cpu::DecodeRegister;
use cpu::common_defs::address_mode::AddressMode;
use cpu::common_defs::opcode_class::OpcodeClass;
use memory::Memory;

macro_rules! set_zs {
    ($cpu_state:expr,$val:expr) => (set_z!($cpu_state,$val);set_s!($cpu_state,$val););
}
macro_rules! set_z {
    ($cpu_state:expr,$val:expr) => ($cpu_state.Z = $val == 0;);
}
macro_rules! set_s {
    ($cpu_state:expr,$val:expr) => ($cpu_state.S = $val >= 0x80;);
}
macro_rules! stack_push8 {
    ($cpu_state:expr,$mem:expr,$val:expr) => (try!($mem.write8(0x0100 as u16 | $cpu_state.sp as u16,$val)); $cpu_state.sp-=1;);
}
macro_rules! stack_push16 {
    ($cpu_state:expr,$mem:expr,$val:expr) => (try!($mem.write16(0x0100 as u16 | $cpu_state.sp as u16 -1,$val)); $cpu_state.sp-=2;);
}
macro_rules! stack_pull8 {
    ($cpu_state:expr,$mem:expr) => {
        {
            $cpu_state.sp+=1;
            $mem.read8(0x0100 as u16 | $cpu_state.sp as u16)
        }
    }
}
macro_rules! stack_pull16 {
    ($cpu_state:expr,$mem:expr) => {
        {
            $cpu_state.sp+=2;
            $mem.read16(0x0100 as u16 | $cpu_state.sp as u16 -1)
        }
    }
}
macro_rules! compare {
    ($cpu_state:expr,$a:expr,$b:expr) => {
        {
            set_zs!($cpu_state, ($a as i16 - $b as i16) as u8);
            $cpu_state.C = $a >= $b;
        }
    }
}



#[derive(Debug)]
pub enum ExecutionError {
  MemoryError(::memory::MemoryError),
  UnexpectedOpcode(String),
  UnexpectedAddressMode(String)
}
impl From<::memory::MemoryError> for ExecutionError {
    fn from(err: ::memory::MemoryError) -> ExecutionError {
        ExecutionError::MemoryError(err)
    }
}

pub struct CpuExecutor {
    op_table: Vec<OpcodeExecInfo>,
}

impl CpuExecutor {
	/// Construct a new CpuExecutor.
    pub fn new(opcodes: Vec<OpcodeExecInfo> ) -> CpuExecutor {
        CpuExecutor {
            op_table:  opcodes,
        }
    }

	/// Reset the given cpu_state.
    pub fn power_on(self: &CpuExecutor, cpu_state: &mut CpuState, mem:&mut Memory) {
        self.reset(cpu_state,mem);
    }
	/// Reset the given cpu_state.
    pub fn reset(self: &CpuExecutor, cpu_state: &mut CpuState, mem:&mut Memory) {
        cpu_state.pc = mem.read16(0xFFFC).unwrap();
        cpu_state.sp = 0xFD;
        cpu_state.pack_flags(0x24);
    }
    
    /// Execute a single instruction in the given memory and cpu_state context.
    pub fn step(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) -> Result<(),ExecutionError> {
        try!(self.fetch_and_decode(cpu_state,mem));
        try!(self.execute(cpu_state,mem));
        Ok(())
    }

	/// Fetch the next instruction and perform address resolution.
    pub fn fetch_and_decode(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) -> Result<(),ExecutionError> {
        cpu_state.instruction_register = mem.read8(cpu_state.pc).unwrap();
        cpu_state.decode_register = try!(self.decode(cpu_state,mem));
        cpu_state.pc = cpu_state.pc+1;
        Ok(())
    }
    
    /// Perform address resolution, returning the info in a DecodeRegister.
    fn decode(self: &CpuExecutor, cpu_state: &CpuState, mem: &Memory) -> Result<DecodeRegister,ExecutionError> {
        let mut dr = DecodeRegister {
            info : self.op_table[cpu_state.instruction_register as usize].clone(),
            ..Default::default()
        };
        match dr.info.address_mode {
            // no explicit addresses for the following modes
            AddressMode::Accumulator  => { },
            AddressMode::Implied      => { },

            // explicit addresses from here on out
            AddressMode::Absolute => {
                dr.addr_final  = Some(mem.read16(cpu_state.pc+1).unwrap());
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::AbsoluteX       => {
                dr.addr_final  = Some( ((mem.read16(cpu_state.pc+1).unwrap() as u32 + cpu_state.x as u32) % 65536) as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::AbsoluteY       => {
                dr.addr_final  = Some( ((mem.read16(cpu_state.pc+1).unwrap() as u32 + cpu_state.y as u32) % 65536) as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::Immediate       => {
                dr.addr_final  = Some(cpu_state.pc+1);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::Indirect        => {
                dr.addr_intermediate = Some(mem.read16(cpu_state.pc+1).unwrap());
                dr.addr_final        = Some(mem.read16(dr.addr_intermediate.unwrap()).unwrap());
            },
            AddressMode::IndexedIndirect => {
                dr.addr_intermediate = Some( (mem.read8(cpu_state.pc+1).unwrap() + cpu_state.x) as u16);
                dr.addr_final        = Some(mem.read16(dr.addr_intermediate.unwrap()).unwrap());
                dr.value_final       = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::IndirectIndexed => {
                dr.addr_init         = Some(mem.read8(cpu_state.pc+1).unwrap() as u16);
                dr.addr_intermediate = Some(mem.read16(dr.addr_init.unwrap()).unwrap());
                dr.addr_final        = Some( ((dr.addr_intermediate.unwrap() as u32 + cpu_state.y as u32) % 65536) as u16);
                dr.value_final       = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::Relative        => {
                dr.value_final = Some(mem.read8(cpu_state.pc+1).unwrap());

                // NOTE: relative is from the end of the current instruction and relative
                // instructions are 2 bytes long, so we add 2 before adding in the specified offset
                //
                if dr.value_final.unwrap() < 0x80 { dr.addr_final  = Some(cpu_state.pc + 2 + dr.value_final.unwrap() as u16); }
                else                              { dr.addr_final  = Some(cpu_state.pc + 2 + dr.value_final.unwrap() as u16 - 0x100);}
            },
            AddressMode::ZeroPage        => {
                dr.addr_final  = Some(mem.read8(cpu_state.pc+1).unwrap() as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::ZeroPageX       => {
                dr.addr_final  = Some( (mem.read8(cpu_state.pc+1).unwrap() as u16 + cpu_state.x as u16) % 256);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::ZeroPageY       => {
                dr.addr_final  = Some( (mem.read8(cpu_state.pc+1).unwrap() as u16 + cpu_state.y as u16) % 256);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            _ => { return Err(ExecutionError::UnexpectedAddressMode(format!("unrecognized addressing mode '{:?}' while decoding instruction_register!",dr.info.address_mode))); }
        }

        return Ok(dr);
    }
    
    /// Perform the current instruction, returning the CpuState after execution.
    pub fn execute(self: &CpuExecutor, cpu_state: &mut CpuState, mem:&mut Memory) -> Result<(),ExecutionError> {
    	// Figure out which opcode is being executed.
    	match cpu_state.decode_register.info.opcode_class {
    		OpcodeClass::ADC => {
                let a = cpu_state.a;
                let b = cpu_state.decode_register.value_final.unwrap();
                let sum:u16 = a as u16 + b as u16 + cpu_state.C as u16;
                cpu_state.a = sum as u8;
                set_zs!(cpu_state,cpu_state.a);

                cpu_state.C = sum > 0xFF;
                cpu_state.V = ((a^b)&0x80) == 0 && ((a^cpu_state.a)&0x80) != 0;

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::AND => {
				cpu_state.a = cpu_state.a & cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.a);
				
                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::ASL => {
				cpu_state.a = cpu_state.a << 1;
				cpu_state.C = (128 & cpu_state.a) > 0;
    			if cpu_state.a == 0 { cpu_state.Z = true; }
				
                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::BCC => {
                if !cpu_state.C { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else            { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BCS => {
                if cpu_state.C { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else           { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BEQ => {
                if cpu_state.Z { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else           { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BIT => {
                cpu_state.V = (cpu_state.decode_register.value_final.unwrap() >> 6) & 1 > 0;
                set_z!(cpu_state,cpu_state.decode_register.value_final.unwrap() & cpu_state.a);
                set_s!(cpu_state,cpu_state.decode_register.value_final.unwrap());

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::BMI => {
                if cpu_state.S { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else           { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BNE => {
                if !cpu_state.Z { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else            { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BPL => {
                if !cpu_state.S { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else            { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BVC => {
                if !cpu_state.V { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else            { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::BVS => {
                if cpu_state.V { cpu_state.pc = cpu_state.decode_register.addr_final.unwrap(); }
                else           { cpu_state.pc += cpu_state.decode_register.info.len as u16-1;  }
    		},
    		OpcodeClass::CMP => {
                compare!(cpu_state,cpu_state.a,cpu_state.decode_register.value_final.unwrap());

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::CLC => {
                cpu_state.C = false;
    		},
    		OpcodeClass::CLD => {
                cpu_state.D = false;
    		},
    		OpcodeClass::CLV => {
                cpu_state.V = false;
    		},
    		OpcodeClass::CPX => {
                compare!(cpu_state,cpu_state.x,cpu_state.decode_register.value_final.unwrap());

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::CPY => {
                compare!(cpu_state,cpu_state.y,cpu_state.decode_register.value_final.unwrap());

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::DEX => {
                cpu_state.x = (cpu_state.x as i16 - 1) as u8;
                set_zs!(cpu_state,cpu_state.x);
    		},
    		OpcodeClass::DEY => {
                cpu_state.y = (cpu_state.y as i16 - 1) as u8;
                set_zs!(cpu_state,cpu_state.y);
    		},
    		OpcodeClass::EOR => {
				cpu_state.a = cpu_state.a ^ cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.a);
				
                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::INX => {
                cpu_state.x = (cpu_state.x as u16 + 1) as u8;
                set_zs!(cpu_state,cpu_state.x);
    		},
    		OpcodeClass::INY => {
                cpu_state.y = (cpu_state.y as u16 + 1) as u8;
                set_zs!(cpu_state,cpu_state.y);
    		},
    		OpcodeClass::LDA => {
    			cpu_state.a = cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.a);

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::LDX => {
    			cpu_state.x = cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.x);

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::LDY => {
    			cpu_state.y = cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.y);

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::LSR => {
    			cpu_state.a = cpu_state.a >> 1;
    			cpu_state.C = 1 & cpu_state.a > 0;
    			if cpu_state.a == 0 { cpu_state.Z = true; }
    			
                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::JMP => {
    			cpu_state.pc = cpu_state.decode_register.addr_final.unwrap();
    		},
    		OpcodeClass::JSR => {
                stack_push16!(cpu_state,mem,cpu_state.pc+1);
    			cpu_state.pc = cpu_state.decode_register.addr_final.unwrap();
    		},
    		OpcodeClass::NOP => {
            },
    		OpcodeClass::ORA => {
                cpu_state.a = cpu_state.a | cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.a);

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
            },
    		OpcodeClass::PHA => {
                stack_push8!(cpu_state,mem,cpu_state.a);
            },
            // http://wiki.nesdev.com/w/index.php/Status_flags
    		OpcodeClass::PHP => {
                stack_push8!(cpu_state,mem,cpu_state.unpack_flags() | 0x30);
            },
    		OpcodeClass::PLA => {
                cpu_state.a = stack_pull8!(cpu_state,mem).unwrap();
                set_zs!(cpu_state,cpu_state.a);
            },
            //http://wiki.nesdev.com/w/index.php/Status_flags
    		OpcodeClass::PLP => {
                let val = (stack_pull8!(cpu_state,mem).unwrap()&0xEF) | 0x20;
                cpu_state.pack_flags(val);
            },
    		OpcodeClass::RTS => {
                cpu_state.pc = stack_pull16!(cpu_state,mem).unwrap() + 1;
            },
    		OpcodeClass::SBC => {
                let a = cpu_state.a;
                let b = cpu_state.decode_register.value_final.unwrap();
                let diff:i16 = a as i16 - b as i16 - !cpu_state.C as i16;
                cpu_state.a = diff as u8;
                set_zs!(cpu_state,cpu_state.a);

                cpu_state.C = diff >= 0x00;
                cpu_state.V = ((a^b)&0x80) != 0 && ((a^cpu_state.a)&0x80) != 0;

                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::SEC => {
                cpu_state.C = true;
    		},
    		OpcodeClass::SED => {
                cpu_state.D = true;
    		},
    		OpcodeClass::SEI => {
                cpu_state.I = true;
    		},
    		OpcodeClass::STA => {
                try!(mem.write8(cpu_state.decode_register.addr_final.unwrap(),cpu_state.a));
                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::STX => {
                try!(mem.write8(cpu_state.decode_register.addr_final.unwrap(),cpu_state.x));
                cpu_state.pc += cpu_state.decode_register.info.len as u16-1;
    		},
    		OpcodeClass::TAX => {
                cpu_state.x = cpu_state.a;
                set_zs!(cpu_state,cpu_state.x);
    		},
    		OpcodeClass::TAY => {
                cpu_state.y = cpu_state.a;
                set_zs!(cpu_state,cpu_state.y);
    		},
    		OpcodeClass::TSX => {
                cpu_state.x = cpu_state.sp;
                set_zs!(cpu_state,cpu_state.x);
    		},
    		OpcodeClass::TXS => {
                cpu_state.sp = cpu_state.x;
    		},
    		OpcodeClass::TXA => {
                cpu_state.a = cpu_state.x;
                set_zs!(cpu_state,cpu_state.a);
    		},
    		OpcodeClass::TYA => {
                cpu_state.a = cpu_state.y;
                set_zs!(cpu_state,cpu_state.a);
    		},
    		
			_ => { return Err(ExecutionError::UnexpectedOpcode(format!("Unrecognised opcode class: {:?}", cpu_state.decode_register.info.opcode_class)));}

    	}
        Ok(())
    }
}

