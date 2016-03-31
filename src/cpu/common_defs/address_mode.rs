use std::str::FromStr;

#[derive(Debug)]
pub enum AddressMode {
    None = 0,
    Absolute,        AbsoluteX,
    AbsoluteY,       Accumulator,
    Immediate,       Implied,
    Indirect,        IndexedIndirect,
    IndirectIndexed, Relative,
    ZeroPage,        ZeroPageX,
    ZeroPageY,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidString(String)
}

impl FromStr for AddressMode {
    type Err = ParseError;

    fn from_str(s:&str) -> Result<Self,Self::Err> {
        match s {
            "Absolute"        => Ok(AddressMode::Absolute       ), "AbsoluteX"       => Ok(AddressMode::AbsoluteX),
            "AbsoluteY"       => Ok(AddressMode::AbsoluteY      ), "Accumulator"     => Ok(AddressMode::Accumulator),
            "Immediate"       => Ok(AddressMode::Immediate      ), "Implied"         => Ok(AddressMode::Implied),
            "Indirect"        => Ok(AddressMode::Indirect       ), "IndexedIndirect" => Ok(AddressMode::IndexedIndirect),
            "IndirectIndexed" => Ok(AddressMode::IndirectIndexed), "Relative"        => Ok(AddressMode::Relative),
            "ZeroPage"        => Ok(AddressMode::ZeroPage       ), "ZeroPageX"       => Ok(AddressMode::ZeroPageX),
            "ZeroPageY"       => Ok(AddressMode::ZeroPageY      ),

            _ => Err(ParseError::InvalidString(s.to_string()))
        }
    }
}

impl Default for AddressMode {
    fn default() -> AddressMode { AddressMode::None }
}
