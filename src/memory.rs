extern crate byteorder;
use std::io::Cursor;
use std::io::Write;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub enum MemoryError {
  PPUAccessViolation(String),
  APUAccessViolation(String),
}

pub struct Memory {
    pub mem: Vec<u8>,
}


impl Memory {
    pub fn new() -> Memory {
        let mut m =  Memory {
            mem: Vec::with_capacity(::std::u16::MAX as usize)
        };
        unsafe { &m.mem.set_len(::std::u16::MAX as usize); }
        return m;
    }

    // meant for a 'raw' write interface, not meant to be used by the 6502 processor itself, more
    // tests and other tools to be able to read blocks of memory quickly and easily
    //
    pub fn write(self:&mut Memory, index:usize, inp: &[u8]) {
        if inp.len() + index > self.mem.len() {
            panic!("memory vec length is {}, input array goes from {} to {}",self.mem.len(),index,inp.len()+index)
        }
        &self.mem[index..(index+inp.len())].clone_from_slice(inp);
    }
    pub fn read8(self:&Memory,addr: u16) -> Result<u8,MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) => Ok(self.mem[raddr]),
            Err(err) => Err(err)
        }
    }
    pub fn read16(self:&Memory,addr: u16)  -> Result<u16,MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) =>{
                // splices are exclusive on the upper range (half open)
                let mut rdr = Cursor::new(&self.mem[raddr..(raddr+2)]);
                Ok(rdr.read_u16::<LittleEndian>().unwrap())
            },
            Err(err) => Err(err)
        }
    }
    pub fn write8(self:&mut Memory,addr: u16, val:u8) -> Option<MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) => {
                self.mem[raddr] = val;
                Option::None
            },
            Err(err) => Option::Some(err)
        }
    }
    pub fn write16(self:&mut Memory,addr: u16, val:u16) -> Option<MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) => {
                // splices are exclusive on the upper range (half open)
                (&mut self.mem[raddr..(raddr+2)]).write_u16::<LittleEndian>(val).unwrap();
                Option::None
            },
            Err(err) => Option::Some(err)
        }
    }

    // https://en.wikibooks.org/wiki/NES_Programming/Memory_Map
    //
    fn resolve_address(self:&Memory,addr: u16) -> Result<usize,MemoryError> {
        if addr <= 0x1FFF { return Ok( (addr & 0x7FF) as usize); }
        if addr <= 0x2007 { return Err(MemoryError::PPUAccessViolation("PPU Memory access is currently not implemented".to_string())); }
        if addr <= 0x3FFF { return Ok( (addr & 0x7FF & 0x08) as usize); }
        if addr <= 0x401F { return Err(MemoryError::APUAccessViolation("Memory access is currently not implemented".to_string())); }
        Ok(addr as usize)
    }
}
