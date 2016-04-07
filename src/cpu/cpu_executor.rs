use cpu::common_defs::OpcodeExecInfo;
use cpu::CpuState;
use cpu::DecodeRegister;
use cpu::common_defs::address_mode::AddressMode;
use cpu::common_defs::opcode_class::OpcodeClass;
use memory::Memory;
use std::collections::HashMap;

pub struct CpuExecutor {
    op_table: HashMap<u8,OpcodeExecInfo>,
}

impl CpuExecutor {
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

    pub fn reset(self: &CpuExecutor, cpu_state: &mut CpuState, mem:&mut Memory) {
        cpu_state.pc = mem.read16(0xFFFC).unwrap();
        cpu_state.sp = 0xFD;
        cpu_state.pack_flags(0x24);
    }

    pub fn fetch_and_decode(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) {
        cpu_state.instruction_register = mem.read8(cpu_state.pc).unwrap();
        cpu_state.decode_register = self.decode(cpu_state,mem);
        cpu_state.pc = cpu_state.pc + cpu_state.decode_register.info.len as u16;
    }
    pub fn execute(self: &CpuExecutor, mut cpu_state: &CpuState, mut mem:&Memory) {
    	
    	// The state of the CPU after executing the current instruction.
    	let mut new_cpu_state = cpu_state.clone();
    	
    	// Figure out which opcode is being executed.
    	let ref decode_register = cpu_state.decode_register;
    	match decode_register.info.opcode_class {
    		
    		// LDA (Load Accumulator)
    		OpcodeClass::LDA => {
    			new_cpu_state.a = decode_register.value_final.unwrap();
    		},
    		
    		// LDX (Load X)
    		OpcodeClass::LDX => {
    			new_cpu_state.x = decode_register.value_final.unwrap();
    		}
    		
    		// LDY (Load Y)
    		OpcodeClass::LDY => {
    			new_cpu_state.y = decode_register.value_final.unwrap();
    		}
    		
    		// NOP (No Operation)
    		OpcodeClass::NOP => {}
    		
			_ => panic!("Unrecognised opcode class")

    	}
    	
    	new_cpu_state;
    	
    }
    
    pub fn step(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) {
        self.fetch_and_decode(cpu_state,mem);
        self.execute(cpu_state,mem);
    }

    fn decode(self: &CpuExecutor, cpu_state: &CpuState, mem: &Memory) -> DecodeRegister {
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
                dr.addr_final  = Some(mem.read16(cpu_state.pc+1).unwrap() + cpu_state.x as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::AbsoluteY       => {
                dr.addr_final  = Some(mem.read16(cpu_state.pc+1).unwrap() + cpu_state.y as u16);
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
                dr.addr_final        = Some(mem.read16(dr.addr_intermediate.unwrap()).unwrap())
            },
            AddressMode::IndirectIndexed => {
                dr.addr_intermediate = Some(mem.read16(cpu_state.pc+1).unwrap());
                dr.addr_final        = Some(dr.addr_intermediate.unwrap() + cpu_state.y as u16);
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
                dr.addr_final  = Some( (mem.read8(cpu_state.pc+1).unwrap() + cpu_state.x) as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::ZeroPageY       => {
                dr.addr_final  = Some( (mem.read8(cpu_state.pc+1).unwrap() + cpu_state.y) as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            _ => panic!("unrecognized addressing mode while decoding instruction_register!")
        }

        return dr;
    }
}

