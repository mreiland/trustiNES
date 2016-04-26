use cpu::common_defs::OpcodeExecInfo;
use cpu::CpuState;
use cpu::DecodeRegister;
use cpu::common_defs::address_mode::AddressMode;
use cpu::common_defs::opcode_class::OpcodeClass;
use memory::Memory;
use std::collections::HashMap;


macro_rules! set_zs {
    ($cpu_state:expr,$val:expr) => ($cpu_state.Z = $val == 0;$cpu_state.S = $val >= 0x80 );
}

macro_rules! stack_push8 {
    ($cpu_state:expr,$mem:expr,$val:expr) => (try!($mem.write8(0x0100 | $cpu_state.sp,$val)); $cpu_state.sp-=1;);
}
macro_rules! stack_push16 {
    ($cpu_state:expr,$mem:expr,$val:expr) => (try!($mem.write16(0x0100 as u16 | $cpu_state.sp as u16 -1,$val)); $cpu_state.sp-=2;);
}
macro_rules! stack_pull8 {
    ($cpu_state:expr,$mem:expr) => {
        {
            $cpu_state.pc+=1;
            $mem.read8(0x0100 | $cpu_state.sp;)
        }
    }
}
macro_rules! stack_pull16 {
    ($cpu_state:expr,$mem:expr) => {
        {
            $cpu_state.pc+=2;
            $mem.read16(0x0100 as u16 | $cpu_state.sp as u16 -1)
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
    op_table: HashMap<u8,OpcodeExecInfo>,
}

impl CpuExecutor {
	
	/// Construct a new CpuExecutor.
    pub fn new(opcodes: Vec<OpcodeExecInfo> ) -> CpuExecutor {
        // Construct the HashMap and copy it over to CpuExecutor's constructor.
        let mut op_table_temp: HashMap<u8, OpcodeExecInfo> = HashMap::new();
        for oc in opcodes {
            if op_table_temp.contains_key(&oc.opcode) {
                println!("Encountered duplicate opcode: {:#x}", oc.opcode);
                println!("Replacing previous opcode, which was {{ opcode: {:#x}, len: {}, cycles: {}, page_cycles: {} }}", oc.opcode, oc.len, oc.cycles, oc.page_cycles);
            }
            op_table_temp.insert(oc.opcode, oc);
        }
        
        CpuExecutor {
            op_table: op_table_temp,
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
    pub fn step(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) {
        self.fetch_and_decode(cpu_state,mem);
        self.execute(cpu_state,mem);
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
            info : self.op_table[&cpu_state.instruction_register].clone(),
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
                dr.addr_final  = Some(cpu_state.pc+1);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
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
    		OpcodeClass::ASL => {
				cpu_state.a = cpu_state.a << 1;
				cpu_state.C = (128 & cpu_state.a) > 0;
    			if cpu_state.a == 0 { cpu_state.Z = true; }
				
                cpu_state.pc = cpu_state.pc + (cpu_state.decode_register.info.len as u16 -1);
    		},
    		OpcodeClass::LDA => {
    			cpu_state.a = cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.a);

                cpu_state.pc = cpu_state.pc + (cpu_state.decode_register.info.len as u16 -1);
    		},
    		OpcodeClass::LDX => {
    			cpu_state.x = cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.x);

                cpu_state.pc = cpu_state.pc + (cpu_state.decode_register.info.len as u16 -1);
    		},
    		OpcodeClass::LDY => {
    			cpu_state.y = cpu_state.decode_register.value_final.unwrap();
                set_zs!(cpu_state,cpu_state.y);

                cpu_state.pc = cpu_state.pc + (cpu_state.decode_register.info.len as u16 -1);
    		},
    		OpcodeClass::LSR => {
    			cpu_state.a = cpu_state.a >> 1;
    			cpu_state.C = 1 & cpu_state.a > 0;
    			if cpu_state.a == 0 { cpu_state.Z = true; }
    			
                cpu_state.pc = cpu_state.pc + (cpu_state.decode_register.info.len as u16 -1);
    		},
    		OpcodeClass::JMP => {
    			cpu_state.pc = cpu_state.decode_register.addr_final.unwrap();
    		},
    		OpcodeClass::NOP => {},
    		OpcodeClass::STX => {
                try!(mem.write8(cpu_state.decode_register.addr_final.unwrap(),cpu_state.decode_register.value_final.unwrap()));
                cpu_state.pc = cpu_state.pc + (cpu_state.decode_register.info.len as u16 -1);
    		},
    		
			_ => { return Err(ExecutionError::UnexpectedOpcode(format!("Unrecognised opcode class: {:?}", cpu_state.decode_register.info.opcode_class)));}

    	}
        Ok(())
    }
}

