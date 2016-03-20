//public mods
pub mod opcode;

//private mods
mod state;
mod executor;

// hoisted interfaces
pub use self::state::CpuState;
pub use self::state::DecodeInfo;

pub use self::executor::CpuExecutor;

