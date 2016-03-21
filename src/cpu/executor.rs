use cpu::common_defs::OpcodeExecInfo;
use cpu::state::CpuState;
use memory::Memory;
use std::collections::HashMap;

pub struct CpuExecutor {
    op_table: HashMap<u16,OpcodeExecInfo>,
}

impl CpuExecutor {
    pub fn new(opcodes: Vec<OpcodeExecInfo> ) -> CpuExecutor {
        // Construct the HashMap and copy it over to CpuExecutor's constructor.
        let mut op_table_temp: HashMap<u16, OpcodeExecInfo> = HashMap::new();
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

    pub fn reset(self: &CpuExecutor, mut cpu_state: &CpuState, mut mem:&Memory) {
    }

    pub fn fetch_and_decode(self: &CpuExecutor, mut cpu_state: &CpuState,mut mem:&Memory) {
    }
    pub fn execute(self: &CpuExecutor, mut cpu_state: &CpuState,mut mem:&Memory) {
    }
    pub fn step(self: &CpuExecutor, mut cpu_state: &CpuState,mut mem:&Memory) {
        self.fetch_and_decode(cpu_state,mem);
        self.execute(cpu_state,mem);
    }
}
