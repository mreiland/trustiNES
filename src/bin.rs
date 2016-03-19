extern crate byteorder;
pub mod rom_loader;
pub mod memory;
pub mod cpu;

use std::env;

fn main() {
    //let args: Vec<String> = env::args().collect();
    
    //if args.len() < 2 {
      //println!("Usage: main FILE");
      //return;
    //}

    //println!("Loading {}", args[1]);
    let cpu: cpu::CpuState;

    match rom_loader::load_ines("roms/nestest.nes") {
    //match load_ines(args[1]) {
      Ok(_) => (),
      Err(err) => println!("Error: {:?}", err),
    }
}

