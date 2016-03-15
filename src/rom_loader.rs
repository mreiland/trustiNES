use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;
use std::io::Seek;
use std::io::SeekFrom;

#[derive(Debug)]
pub enum InesError {
  Io(io::Error),
  InesFormat(String),
  Unsupported(String),
}

impl From<io::Error> for InesError {
    fn from(err: io::Error) -> InesError {
        InesError::Io(err)
    }
}
#[repr(C,packed)]
pub struct InesHeader {
  ines_identifier: u32,
  prg_rom_banks: u8,
  chr_rom_banks: u8,
  flags_6: u8,
  flags_7: u8,
  prg_ram_units: u8,
  padding: [u8;7],
}

// http://wiki.nesdev.com/w/index.php/INES
//
pub fn load_ines<P:AsRef<Path>>(file_path: P) -> Result<Vec<u8>,InesError> {
    let mut file = try!(File::open(&file_path));
    let metadata = try!(::std::fs::metadata(&file_path));
    if metadata.len() < ::std::mem::size_of::<InesHeader>() as u64 {
        return Err(InesError::InesFormat("File is smaller than the ines header.".to_string()));
    }
    
    let header = unsafe {
        let mut header_buf = [0u8;16];
        let _ = file.read(&mut header_buf);

        ::std::mem::transmute::<_,InesHeader>(header_buf)
    };
    if header.ines_identifier != 0x1A53454E {
        return Err(InesError::InesFormat("Did not find ines header identifier".to_string()));
    }
    if header.flags_7 != 0 {
        return Err(InesError::Unsupported("This version of the ines format is not supported".to_string()));
    }
    if header.flags_6 & 0x04 != 0 {
        return Err(InesError::Unsupported("Loading trainers is not supported".to_string()));
    }

    let _  = file.seek(SeekFrom::Start(0));

    let mut file_bytes = Vec::<u8>::new();
    try!(file.read_to_end(&mut file_bytes));

    // https://en.wikibooks.org/wiki/NES_Programming/Memory_Map
    //
    let mut nes_bytes = Vec::<u8>::with_capacity(::std::u16::MAX as usize);
    nes_bytes.resize(::std::u16::MAX as usize,0);

    let prg_addr = 16;
    let prg_page_len = 0xBFFF - 0x8000;

    // NOTE: the below implementation is terrible.  I don't know how to memcpy from 1
    //       vector to another, so we do it manually.  As far as I'm concerned, this is
    //       *more* error prone than simply calling a memcpy function, but I'm still
    //       learning rust and it is what it is.
    //

    // nes has 2 prg-rom pages at 0x8000 and 0xC000
    // if the rom has only a single prg-rom page, then we mirror it
    //
    for i in 0..prg_page_len {
        nes_bytes[0x8000+i] = file_bytes[prg_addr+i];
        if header.prg_rom_banks == 1 {
            nes_bytes[0xC000+i] = file_bytes[prg_addr+i];
        }
        else {
            nes_bytes[0xC000+i] = file_bytes[prg_addr+prg_page_len+i];
        }
    }

    Ok(nes_bytes)
}

