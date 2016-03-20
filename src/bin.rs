extern crate byteorder;
pub mod rom_loader;
pub mod memory;
pub mod cpu;

use std::env;
use cpu::CpuState;
use memory::Memory;

fn main() {
    let rom = rom_loader::load_ines("roms/nestest.nes").unwrap();
    let mut cpu: CpuState = Default::default();
    let mem:Memory = Memory::new();
    // TODO: implement loading the rom into the correct position in memory

    // TODO: handle this properly instead of just unwrapping
    let opcode_info = cpu::opcode::load_from_file("resources/opcodes.csv").unwrap();
    let executor = cpu::CpuExecutor::new(opcode_info.0);

    for i in 0..10 {
        executor.fetch_and_decode(&cpu,&mem);
        executor.execute(&cpu,&mem);
    }
}

