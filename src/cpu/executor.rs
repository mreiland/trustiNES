use cpu::common_defs::OpcodeExecInfo;
use cpu::state::CpuState;
use memory::Memory;

pub struct CpuExecutor {
    op_table: Vec<OpcodeExecInfo>,
}

impl CpuExecutor {
    pub fn new(opcodes: Vec<OpcodeExecInfo> ) -> CpuExecutor {
        CpuExecutor {
            op_table: opcodes,
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
