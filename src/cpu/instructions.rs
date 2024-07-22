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
    BIT(PrefixTarget, BitPosition),
    RES(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),

    // Jump Instructions
    JP(JumpTest),
    JR(JumpTest),
    JPI,

    // Load Instructions
    LD(LoadType)
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

pub enum BitPosition {
    B0, B1, B2, B3, B4, B5, B6, B7
}

impl std::convert::From<BitPosition> for u8 {
    fn from(pos : BitPosition) -> u8 {
        match pos {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7
        }
    }
}

pub enum JumpTest {
    NotZero, Zero, NotCarry, Carry, Always
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HL
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HL
}

pub enum LoadWordTarget {
    BC, DE, HL, SP
}

pub enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

pub enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
  Word(LoadWordTarget),
  AFromIndirect(Indirect),
  IndirectFromA(Indirect),
  AFromByteAddress,
  ByteAddressFromA
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    } 

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {

        }
    }

    fn from_byte_not_prefixed(byte: u8) ->Option<Instruction> {
        match byte {
            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
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
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),

            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some (Instruction::INC(IncDecTarget::HL)),
            //0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xC3 => Some(Instruction::JP(JumpTest::Always)),

            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some (Instruction::INC(IncDecTarget::H)),
            //0x34 => Some (Instruction::INC(IncDecTarget::HL)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),

            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            //0x34 => Some(Instruction::INC(IncDecTarget::HL)),
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

            0x18 => Some(Instruction::JR(JumpTest::Always)),
            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x38 =>Some(Instruction::JR(JumpTest::Carry)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),

            0x09 => Some(Instruction::ADDHL(ADDHLTarget::BC)),
            0x19 => Some(Instruction::ADDHL(ADDHLTarget::DE)),
            0x29 => Some(Instruction::ADDHL(ADDHLTarget::HL)),
            //0x39 => Some(Instruction::ADDHL(ADDHLTarget::SP)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),

            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xDA => Some(Instruction::JP(JumpTest::Carry)),

            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            //0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),

            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),

            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),

            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
            //0xCE =>
            //0xDE =>
            //0xEE =>
            //0xFE =>
            
            0x0F => Some(Instruction::RRCA),
            0x1F => Some(Instruction::RRA),
            0x2F => Some(Instruction::CPL),
            0x3F => Some(Instruction::CCF),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
        }
    }
}