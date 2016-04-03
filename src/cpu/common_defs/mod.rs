//public mods
pub mod address_mode;
pub mod opcode_class;

//private mods
use std::clone::Clone;

// hoisted interfaces

#[derive(Default)]
pub struct OpcodeDebugInfo {
    pub opcode: u8,
    pub name: String,
    pub address_mode_name: String,
    pub notes: String,
}

#[derive(Default)]
pub struct OpcodeExecInfo {
    pub opcode: u8,
    pub opcode_class:opcode_class::OpcodeClass,
    pub address_mode:address_mode::AddressMode,
    pub len: u8,
    pub cycles: u8,
    pub page_cycles: u8,
}

