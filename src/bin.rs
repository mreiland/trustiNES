extern crate byteorder;
pub mod rom_loader;
pub mod memory;
pub mod cpu;
pub mod logger;

use cpu::CpuState;
use memory::Memory;


fn main() {
    let rom = rom_loader::load_ines("roms/nestest.nes").unwrap();
    let mut cpu: CpuState = Default::default();
    let mut mem:Memory = Memory::new();

    // TODO: handle this properly instead of just unwrapping
    let opcode_info = cpu::opcode::load_from_file("resources/opcodes.csv").unwrap();
    let executor = cpu::CpuExecutor::new(opcode_info.0);
    let mut logger = logger::NesTest::new("resources/nestest.out",opcode_info.1);

    executor.power_on(&mut cpu, &mut mem);
    cpu.pc = 0xC000;
    mem.write(0,&rom);

    for i in 0..10 {
        executor.fetch_and_decode(&mut cpu,&mut mem).unwrap();
        logger.log_after_fetch(&cpu,&mem);
        executor.execute(&mut cpu,&mut mem).unwrap();
    }
}

