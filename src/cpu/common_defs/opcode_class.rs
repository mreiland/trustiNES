use std::str::FromStr;

#[derive(PartialEq,Clone,Debug)]
#[allow(non_camel_case_types)]
pub enum OpcodeClass {
    None = 0,
    ADC, AND, ASL,BCC,
    BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC,
    BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY,
    DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP,
    JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA,
    PHP, PLA, PLP, ROL,
    ROR, RTI, RTS, SBC,
    SEC, SED, SEI, STA,
    STX, STY, TAX, TAY,
    TSX, TXA, TXS, TYA,

    // illegal opcodes
    ILL_AHX,  ILL_ALR,  ILL_ANC,
    ILL_ARR,  ILL_AXS,  ILL_DCP,
    ILL_ISC,  ILL_KIL,  ILL_LAS,
    ILL_LAX1, ILL_LAX2, ILL_NOP,
    ILL_RLA,  ILL_RRA,  ILL_SAX,
    ILL_SBC,  ILL_SHX,  ILL_SHY,
    ILL_SLO,  ILL_SRE,  ILL_TAS,
    ILL_XAA,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidString(String)
}

impl FromStr for OpcodeClass {
    type Err = ParseError;

    fn from_str(s:&str) -> Result<Self,Self::Err> {
        match s {
            "ADC" => Ok(OpcodeClass::ADC), "AND" => Ok(OpcodeClass::AND), "ASL" => Ok(OpcodeClass::ASL), "BCC" => Ok(OpcodeClass::BCC),
            "BCS" => Ok(OpcodeClass::BCS), "BEQ" => Ok(OpcodeClass::BEQ), "BIT" => Ok(OpcodeClass::BIT), "BMI" => Ok(OpcodeClass::BMI),
            "BNE" => Ok(OpcodeClass::BNE), "BPL" => Ok(OpcodeClass::BPL), "BRK" => Ok(OpcodeClass::BRK), "BVC" => Ok(OpcodeClass::BVC),
            "BVS" => Ok(OpcodeClass::BVS), "CLC" => Ok(OpcodeClass::CLC), "CLD" => Ok(OpcodeClass::CLD), "CLI" => Ok(OpcodeClass::CLI),
            "CLV" => Ok(OpcodeClass::CLV), "CMP" => Ok(OpcodeClass::CMP), "CPX" => Ok(OpcodeClass::CPX), "CPY" => Ok(OpcodeClass::CPY),
            "DEC" => Ok(OpcodeClass::DEC), "DEX" => Ok(OpcodeClass::DEX), "DEY" => Ok(OpcodeClass::DEY), "EOR" => Ok(OpcodeClass::EOR),
            "INC" => Ok(OpcodeClass::INC), "INX" => Ok(OpcodeClass::INX), "INY" => Ok(OpcodeClass::INY), "JMP" => Ok(OpcodeClass::JMP),
            "JSR" => Ok(OpcodeClass::JSR), "LDA" => Ok(OpcodeClass::LDA), "LDX" => Ok(OpcodeClass::LDX), "LDY" => Ok(OpcodeClass::LDY),
            "LSR" => Ok(OpcodeClass::LSR), "NOP" => Ok(OpcodeClass::NOP), "ORA" => Ok(OpcodeClass::ORA), "PHA" => Ok(OpcodeClass::PHA),
            "PHP" => Ok(OpcodeClass::PHP), "PLA" => Ok(OpcodeClass::PLA), "PLP" => Ok(OpcodeClass::PLP), "ROL" => Ok(OpcodeClass::ROL),
            "ROR" => Ok(OpcodeClass::ROR), "RTI" => Ok(OpcodeClass::RTI), "RTS" => Ok(OpcodeClass::RTS), "SBC" => Ok(OpcodeClass::SBC),
            "SEC" => Ok(OpcodeClass::SEC), "SED" => Ok(OpcodeClass::SED), "SEI" => Ok(OpcodeClass::SEI), "STA" => Ok(OpcodeClass::STA),
            "STX" => Ok(OpcodeClass::STX), "STY" => Ok(OpcodeClass::STY), "TAX" => Ok(OpcodeClass::TAX), "TAY" => Ok(OpcodeClass::TAY),
            "TSX" => Ok(OpcodeClass::TSX), "TXA" => Ok(OpcodeClass::TXA), "TXS" => Ok(OpcodeClass::TXS), "TYA" => Ok(OpcodeClass::TYA),

            "ILL_AHX"  => Ok(OpcodeClass::ILL_AHX ), "ILL_ALR"  => Ok(OpcodeClass::ILL_ALR ), "ILL_ANC" => Ok(OpcodeClass::ILL_ANC),
            "ILL_ARR"  => Ok(OpcodeClass::ILL_ARR ), "ILL_AXS"  => Ok(OpcodeClass::ILL_AXS ), "ILL_DCP" => Ok(OpcodeClass::ILL_DCP),
            "ILL_ISC"  => Ok(OpcodeClass::ILL_ISC ), "ILL_KIL"  => Ok(OpcodeClass::ILL_KIL ), "ILL_LAS" => Ok(OpcodeClass::ILL_LAS),
            "ILL_LAX1" => Ok(OpcodeClass::ILL_LAX1), "ILL_LAX2" => Ok(OpcodeClass::ILL_LAX2), "ILL_NOP" => Ok(OpcodeClass::ILL_NOP),
            "ILL_RLA"  => Ok(OpcodeClass::ILL_RLA ), "ILL_RRA"  => Ok(OpcodeClass::ILL_RRA ), "ILL_SAX" => Ok(OpcodeClass::ILL_SAX),
            "ILL_SBC"  => Ok(OpcodeClass::ILL_SBC ), "ILL_SHX"  => Ok(OpcodeClass::ILL_SHX ), "ILL_SHY" => Ok(OpcodeClass::ILL_SHY),
            "ILL_SLO"  => Ok(OpcodeClass::ILL_SLO ), "ILL_SRE"  => Ok(OpcodeClass::ILL_SRE ), "ILL_TAS" => Ok(OpcodeClass::ILL_TAS),
            "ILL_XAA"  => Ok(OpcodeClass::ILL_XAA ),

            _ => Err(ParseError::InvalidString(s.to_string()))
        }
    }
}

impl Default for OpcodeClass {
    fn default() -> OpcodeClass { OpcodeClass::None }
}
