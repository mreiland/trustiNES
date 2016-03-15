use std::env;
pub mod rom_loader;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
      println!("Usage: main FILE");
      return;
    }
  
    println!("Loading {}", args[1]);

    match rom_loader::load_ines("roms/nestest.nes") {
    //match load_ines(args[1]) {
      Ok(n) => (),
      Err(err) => println!("Error: {:?}", err),
    }
}
