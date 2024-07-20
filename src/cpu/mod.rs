pub mod flags_register;
pub mod instructions;
pub mod registers;

use self::registers::Registers;
use self::instructions::Instruction;

use self::instructions::ArithmeticTarget;
use self::instructions::ADDHLTarget;
use self::instructions::IncDecTarget;
use self::instructions::BitPosition;
use self::instructions::PrefixTarget;

pub struct CPU { 
    pub registers: Registers,
    pc: u16,
    bus: MemoryBus
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, adress: u16) -> u8 {
        self.memory[adress as usize]
    }
}

impl CPU {
    fn step(&self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction);
        } else {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => { 
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => { 
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => { 
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => { 
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.add_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::ADDHL(target) => { 
                match target {
                    ADDHLTarget::BC => {
                        let value = self.registers.get_hl();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ADDHLTarget::DE => {
                        let value = self.registers.get_hl();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ADDHLTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.adc_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.sub_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.sbc_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.and_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::OR(target) => {
                match target  {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.or_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        let new_value = self.xor_hl(value);
                        self.registers.a = new_value as u8;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.b;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        self.cp_hl(value);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::INC(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.inc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.inc(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.inc(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.inc(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.inc(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.inc(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.inc(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.inc16(value);
                        self.registers.set_bc(new_value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.inc16(value);
                        self.registers.set_de(new_value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.inc16(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::DEC(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.dec(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.dec(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.dec(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.dec(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.dec(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.dec(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.dec(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.dec16(value);
                        self.registers.set_bc(new_value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.dec16(value);
                        self.registers.set_de(new_value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.dec16(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::CCF => {
                self.ccf();
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.scf();
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                self.cpl();
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                let new_value = self.shift_r_flag(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                let new_value = self.shift_l_flag(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                let new_value = self.shift_r(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                let new_value = self.shift_l(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(target, bit_position) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.get_hl() as u8;
                        self.bit_test(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SET(target, bit_position) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.get_hl() as u8;
                        self.bit_set(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::RES(target, bit_position) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.get_hl() as u8;
                        self.bit_reset(value, bit_position);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            //Instruction::
    }
  }

    pub fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is tests if the sum of the lower nibbles of the value and register A
        // fit in 4 bits, or 0xF. If yes, then the flag is set to true.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    pub fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (new_value, did_overflow) = hl.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half carry tests if the sum of the lower three nibbles of the value and the register(s)
        // HL fit in 12 bits, or 0xFFF. If yes, then the flag is set to true. 
        self.registers.f.half_carry = (value & 0xFFF) + (hl & 0xFFF) > 0xFFF;

        new_value
    }

    pub fn adc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (newer_value, did_overflow2) = self.registers.a.overflowing_add(did_overflow as u8);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow | did_overflow2;
        self.registers.f.half_carry = (value & 0xF) + (self.registers.a & 0xF) > 0xF;

        newer_value
    }

    pub fn adc_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        let (newer_value, did_overflow2) = self.registers.get_hl().overflowing_add(did_overflow as u16);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow | did_overflow2;
        self.registers.f.half_carry = (value & 0xFFF) + (self.registers.get_hl() & 0xFFF) > 0xFFF;

        newer_value
    }

    pub fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i32) - ((value & 0xF) as i32) < 0;

        new_value
    }

    pub fn sub_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.get_hl() & 0xFFF) as i32) - ((value & 0xFFF) as i32) < 0;

        new_value
    }

    pub fn sbc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let (newer_value, did_overflow2) = self.registers.a.overflowing_sub(did_overflow as u8);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow | did_overflow2;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i32) - ((value & 0xF) as i32) < 0;

        newer_value
    }

    pub fn sbc_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_sub(value);
        let (newer_value, did_overflow2) = self.registers.get_hl().overflowing_sub(did_overflow as u16);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow | did_overflow2;
        self.registers.f.half_carry = ((self.registers.get_hl() & 0xFFF) as i32) - ((value & 0xFFF) as i32) < 0;

        newer_value
    }

    pub fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;

        new_value
    }

    pub fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    pub fn and_hl(&mut self, value: u16) -> u16 {
        let new_value = value & self.registers.get_hl();

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;

        new_value
    }

    pub fn or_hl(&mut self, value: u16) -> u16 {
        let new_value = value | self.registers.get_hl();

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    pub fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    pub fn xor_hl(&mut self, value: u16) -> u16 {
        let new_value = value ^ self.registers.get_hl();

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    pub fn cp(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) as i32) - ((value & 0xF) as i32) < 0;
    }

    pub fn cp_hl(&mut self, value: u16) {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xFFF) as i32) - ((value & 0xFFF) as i32) < 0;
    }

    pub fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) == 0xF;

        new_value
    }

    pub fn inc16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_add(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xFFF) == 0xFFF;

        new_value
    }

    pub fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) == 0x0;

        new_value
    }

    pub fn dec16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_sub(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xFFF) == 0x0;

        new_value
    }

    pub fn ccf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    pub fn scf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
    }

    pub fn shift_r_flag(&mut self, value: u8) -> u8 {
        let transfer_bit = (self.registers.f.carry as u8) << 7;
        let new_value = transfer_bit | (value >> 1);

        self.registers.f.carry = (value & 0b1) == 0b1;

        new_value
    }

    pub fn shift_l_flag(&mut self, value: u8) -> u8 {
        let transfer_bit = (self.registers.f.carry as u8);
        let new_value = (value << 1) | transfer_bit;

        self.registers.f.carry = ((value) & 0x80) == 0x80;

        new_value
    }

    pub fn shift_r(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;

        self.registers.f.carry = (value & 0b1) == 0b1;

        new_value
    }

    pub fn shift_l(&mut self, value: u8) -> u8 {
        let new_value = value << 1;

        self.registers.f.carry = ((value) & 0x80) == 0x80;

        new_value
    }

    pub fn bit_test(&mut self, value: u8, bit_pos: BitPosition) {
        let position : u8 = bit_pos.into();
        let bit = (value >> position) & 0b1;
        self.registers.f.zero = bit != 0;

        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    pub fn bit_set(&mut self, value: u8, bit_pos: BitPosition) -> u8 {
        let position : u8 = bit_pos.into();
        let new_value = value | (0b00000001 << position);

        new_value
    }

    pub fn bit_reset(&mut self, value: u8, bit_pos: BitPosition) -> u8 {
        let position : u8 = bit_pos.into();
        let new_value = value & !(0b00000001 << position);

        new_value
    }
}