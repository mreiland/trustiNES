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
    pub fn reset(self: &CpuExecutor, cpu_state: &mut CpuState, mem:&mut Memory) {
        cpu_state.pc = mem.read16(0xFFFC).unwrap();
        cpu_state.sp = 0xFD;
        cpu_state.pack_flags(0x24);
    }
    
    /// Execute a single instruction in the given memory and cpu_state context.
    pub fn step(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) {
        self.fetch_and_decode(cpu_state,mem);
        let new_cpu_state = self.execute(cpu_state,mem);
        // Perform logging, recording here...
        *cpu_state = new_cpu_state
    }

	/// Fetch the next instruction and perform address resolution.
    pub fn fetch_and_decode(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) {
        cpu_state.instruction_register = mem.read8(cpu_state.pc).unwrap();
        cpu_state.decode_register = self.decode(cpu_state,mem);
        cpu_state.pc = cpu_state.pc + cpu_state.decode_register.info.len as u16;
    }
    
    /// Perform address resolution, returning the info in a DecodeRegister.
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
                dr.addr_final  = Some( ((mem.read16(cpu_state.pc+1).unwrap() as u32 + cpu_state.x as u32) % 65535) as u16);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::AbsoluteY       => {
                dr.addr_final  = Some( ((mem.read16(cpu_state.pc+1).unwrap() as u32 + cpu_state.y as u32) % 65535) as u16);
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
                dr.addr_intermediate = Some( ((mem.read8(cpu_state.pc+1).unwrap() as u32 + cpu_state.x as u32) % 65535 ) as u16);
                dr.addr_final        = Some(mem.read16(dr.addr_intermediate.unwrap()).unwrap());
                dr.value_final       = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::IndirectIndexed => {
                dr.addr_intermediate = Some(mem.read16(cpu_state.pc+1).unwrap());
                dr.addr_final        = Some( ((dr.addr_intermediate.unwrap() as u32 + cpu_state.y as u32) % 65535) as u16);
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
                dr.addr_final  = Some( (mem.read8(cpu_state.pc+1).unwrap() as u16 + cpu_state.x as u16) % 255);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            AddressMode::ZeroPageY       => {
                dr.addr_final  = Some( (mem.read8(cpu_state.pc+1).unwrap() as u16 + cpu_state.y as u16) % 255);
                dr.value_final = Some(mem.read8(dr.addr_final.unwrap()).unwrap());
            },
            _ => panic!("unrecognized addressing mode while decoding instruction_register!")
        }

        return dr;
    }
    
    /// Perform the current instruction, returning the CpuState after execution.
    pub fn execute(self: &CpuExecutor, mut cpu_state: &CpuState, mut mem:&Memory) -> CpuState {
    	
    	// The state of the CPU after executing the current instruction.
    	let mut new_cpu_state = cpu_state.clone();
    	
    	// Figure out which opcode is being executed.
    	let ref decode_register = cpu_state.decode_register;
    	
    	match decode_register.info.opcode_class {
    		
    		// ASL (Arithmetic Shift Left)
    		OpcodeClass::ASL => {

				// Shift accumulator to the left.
				new_cpu_state.a = cpu_state.a << 1;
				// Carry flag gets set to the most significant bit.
				new_cpu_state.C = (128 & cpu_state.a) > 0;
				// Set zero flag if result is 0.
    			if new_cpu_state.a == 0 { new_cpu_state.Z = true }
				
    		},
    		
    		// LDA (Load Accumulator)
    		OpcodeClass::LDA => {
    			new_cpu_state.a = decode_register.value_final.unwrap()
    		},
    		
    		// LDX (Load X)
    		OpcodeClass::LDX => {
    			new_cpu_state.x = decode_register.value_final.unwrap()
    		},
    		
    		// LDY (Load Y)
    		OpcodeClass::LDY => {
    			new_cpu_state.y = decode_register.value_final.unwrap()
    		},
    		
    		// LSR (Logical Shift Right)
    		OpcodeClass::LSR => {
    			
    			// Shift accumulator to the right.
    			new_cpu_state.a = cpu_state.a >> 1;
    			// Carry flag gets set to the least significant bit.
    			new_cpu_state.C = 1 & cpu_state.a > 0;
    			// Set zero flag if result is 0.
    			if new_cpu_state.a == 0 { new_cpu_state.Z = true }
    			
    			
    		},
    		
    		// NOP (No Operation)
    		OpcodeClass::NOP => {},
    		
    		// Default: not sure what this opcode is.
			_ => panic!("Unrecognised opcode class: {:?}", decode_register.info.opcode_class)

    	}
    	
    	// Expression returned is the updated cpu_state.
    	new_cpu_state
    }
    
}

