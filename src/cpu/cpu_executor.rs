use cpu::common_defs::OpcodeExecInfo;
use cpu::CpuState;
use cpu::DecodeRegister;
use cpu::common_defs::address_mode::AddressMode;
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
        cpu_state.pc=cpu_state.pc+1;
    }
    pub fn execute(self: &CpuExecutor, mut cpu_state: &CpuState,mut mem:&Memory) {
    }
    pub fn step(self: &CpuExecutor, cpu_state: &mut CpuState,mem:&mut Memory) {
        self.fetch_and_decode(cpu_state,mem);
        self.execute(cpu_state,mem);
    }

    fn decode(self: &CpuExecutor, cpu_state: &CpuState, mem: &Memory) -> DecodeRegister {
        let dr = DecodeRegister {
            info : self.op_table[&cpu_state.instruction_register].clone(),
            ..Default::default()
        };
        match dr.info.address_mode {
            AddressMode::Absolute        => { },
            AddressMode::AbsoluteX       => { },
            AddressMode::AbsoluteY       => { },
            AddressMode::Accumulator     => { },
            AddressMode::Immediate       => { },
            AddressMode::Implied         => { },
            AddressMode::Indirect        => { },
            AddressMode::IndexedIndirect => { },
            AddressMode::IndirectIndexed => { },
            AddressMode::Relative        => { },
            AddressMode::ZeroPage        => { },
            AddressMode::ZeroPageX       => { },
            AddressMode::ZeroPageY       => { },
            _ => panic!("unrecognized addressing mode while decoding instruction_register!")
        }

        return dr;
    }
}

