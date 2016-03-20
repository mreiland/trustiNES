extern crate byteorder;
pub mod rom_loader;
pub mod memory;
pub mod cpu;

use std::env;

fn main() {
    let cpu: cpu::CpuState;
    cpu::opcode::load_from_file("resources/opcodes.csv");

    match rom_loader::load_ines("roms/nestest.nes") {
      Ok(_) => (),
      Err(err) => println!("Error: {:?}", err),
    }
}

