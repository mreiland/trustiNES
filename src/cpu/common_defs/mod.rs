//public mods

//private mods
mod address_mode;
mod opcode_class;

// hoisted interfaces
pub use self::address_mode::AddressMode;
pub use self::opcode_class::OpcodeClass;

#[derive(Default)]
pub struct OpcodeDebugInfo {
    pub opcode: u16,
    pub name: String,
    pub address_mode_name: String,
    pub notes: String,
}

#[derive(Default)]
pub struct OpcodeExecInfo {
    pub opcode: u16,
    pub opcode_class:OpcodeClass,
    pub address_mode:AddressMode,
    pub len: u8,
    pub cycles: u8,
    pub page_cycles: u8,
}

