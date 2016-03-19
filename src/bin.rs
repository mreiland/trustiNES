extern crate byteorder;
pub mod rom_loader;
pub mod memory;
pub mod cpu;

use std::env;

fn main() {
    let cpu: cpu::CpuState;
    cpu::load_opcodes("resources/opcodes.csv");


    match rom_loader::load_ines("roms/nestest.nes") {
    //match load_ines(args[1]) {
      Ok(_) => (),
      Err(err) => println!("Error: {:?}", err),
    }
}

