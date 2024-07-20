pub enum Instruction {
    // Arithmetic Instructions
    ADD(ArithmeticTarget),
    ADDHL(ADDHLTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

    INC(IncDecTarget),
    DEC(IncDecTarget),

    CCF,
    SCF,
    CPL,

    RLCA,
    RLA,
    RRCA,
    RRA,

    // Prefix Instructions
    BIT(PrefixTarget),
    RES(PrefixTarget),
    SET(PrefixTarget),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),

}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL
}

pub enum ADDHLTarget {
    BC, DE, HL,
}

pub enum IncDecTarget {
    A, B, C, D, E, H, L, BC, DE, HL
}

pub enum PrefixTarget {
    A, B, C, D, E, H, L, HL
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),

            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),

            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),

            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),

            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),

            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),

            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL)),

            0x07 => Some(Instruction::RLCA),
            0x17 => Some(Instruction::RLA),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),

            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),

            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),

            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),

            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),

            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),

            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),

            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
            
            0x0F => Some(Instruction::RRCA),
            0x1F => Some(Instruction::RRA),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
        }
    } 
}