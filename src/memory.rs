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
    mem: Vec<u8>,
}


impl Memory {
    fn read8(self:&Memory,addr: u16) -> Result<u8,MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) => Ok(self.mem[raddr]),
            Err(err) => Err(err)
        }
    }
    fn read16(self:&Memory,addr: u16)  -> Result<u16,MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) =>{
                let mut rdr = Cursor::new(&self.mem[raddr..raddr+1]);
                Ok(rdr.read_u16::<LittleEndian>().unwrap())
            },
            Err(err) => Err(err)
        }
    }
    fn write8(self:&mut Memory,addr: u16, val:u8) -> Option<MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) => {
                self.mem[raddr] = val;
                Option::None
            },
            Err(err) => Option::Some(err)
        }
    }
    fn write16(self:&mut Memory,addr: u16, val:u16) -> Option<MemoryError> {
        match self.resolve_address(addr) {
            Ok(raddr) => {
                (&mut self.mem[raddr..(raddr+1)]).write_u16::<LittleEndian>(val).unwrap();
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
