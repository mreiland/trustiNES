use cpu::common_defs::OpcodeExecInfo;
use cpu::state::CpuState;

pub struct CpuExecutor {
    op_table: Vec<OpcodeExecInfo>,
}

impl CpuExecutor {
    pub fn new(opcodes: Vec<OpcodeExecInfo> ) -> CpuExecutor {
        CpuExecutor {
            op_table: opcodes,
        }
    }

    pub fn fetch_and_decode(self: &CpuExecutor, mut cpu_state: CpuState) -> CpuState {
        cpu_state
    }
    pub fn execute(self: &CpuExecutor, mut cpu_state: CpuState) -> CpuState {
        cpu_state
    }
    pub fn step(self: &CpuExecutor, mut cpu_state: CpuState) -> CpuState {
        cpu_state = self.fetch_and_decode(cpu_state);
        cpu_state = self.execute(cpu_state);
        cpu_state
    }
}
