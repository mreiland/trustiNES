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
    
    let mut header_buf = [0u8;16];
    try!(file.read(&mut header_buf));
    let header = unsafe { ::std::mem::transmute::<_,InesHeader>(header_buf) };

    if header.ines_identifier != 0x1A53454E {
        return Err(InesError::InesFormat("Did not find ines header identifier".to_string()));
    }
    if header.flags_7 != 0 {
        return Err(InesError::Unsupported("This version of the ines format is not supported".to_string()));
    }
    if header.flags_6 & 0x04 != 0 {
        return Err(InesError::Unsupported("Loading trainers is not supported".to_string()));
    }

    try!(file.seek(SeekFrom::Start(0)));

    let mut file_bytes = Vec::<u8>::new();
    try!(file.read_to_end(&mut file_bytes));

    // https://en.wikibooks.org/wiki/NES_Programming/Memory_Map
    //
    let mut nes_bytes = Vec::<u8>::with_capacity(::std::u16::MAX as usize);
    nes_bytes.resize(::std::u16::MAX as usize,0);

    let prg_addr = 16;
    let prg_page_len = 0xBFFF - 0x8000;

    // nes has 2 prg-rom pages at 0x8000 and 0xC000
    // if the rom has only a single prg-rom page, then we mirror it
    let file_page = &file_bytes[prg_addr..(prg_page_len+prg_addr)];
    let file_page2 = if header.prg_rom_banks == 1 {
        file_page
    } else {
        &file_bytes[prg_addr..(prg_addr+prg_page_len+prg_addr)]
    };
    
    &nes_bytes[0x8000..(prg_page_len+0x8000)].clone_from_slice(file_page);
    &nes_bytes[0xC000..(prg_page_len+0xC000)].clone_from_slice(file_page2);
    
    Ok(nes_bytes)
}

