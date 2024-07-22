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
use self::instructions::JumpTest;
use self::instructions::LoadType;
use self::instructions::LoadByteSource;
use self::instructions::LoadByteTarget;
use self::instructions::LoadWordTarget;
use self::instructions::Indirect;
use self::instructions::StackTarget;

pub struct CPU { 
    pub registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, adress: u16) -> u8 {
        self.memory[adress as usize]
    }

    fn write_byte(&mut self, adress: u16, byte: u8) {
        self.memory[adress as usize] = byte;
    }
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction);
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let least_significant_byte = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        
        self.sp = self.sp.wrapping_add(1);
        let most_significant_bye = (self.bus.read_byte(self.sp) as u16) << 8;

        most_significant_bye | least_significant_byte
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);

        if should_jump {
            self.push(self.pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn ret(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc + 1) as u16)
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc
        }

        match instruction {
            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.add(value);
                        self.registers.a = new_value;
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
                    ADDHLTarget::SP => {
                        let value = self.sp;
                        let new_value = self.add_hl(value);
                        self.sp = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.and(value);
                        self.registers.a = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.or(value);
                        self.registers.a = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
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
                    ArithmeticTarget::D8 => {
                        let value = self.registers.l;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.a as u16;
                        self.cp_hl(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.read_next_byte();
                        self.cp(value);
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
                    IncDecTarget::SP => {
                        let value = self.sp;
                        let new_value = self.inc16(value);
                        self.sp = new_value;
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
                    IncDecTarget::SP => {
                        let value = self.sp;
                        let new_value = self.dec16(value);
                        self.sp = new_value;
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
                let new_value = self.rotate_r_flag(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                let new_value = self.rotate_l_flag(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                let new_value = self.rotate_r(self.registers.a);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                let new_value = self.rotate_l(self.registers.a);
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
            Instruction::SWAP(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.swap(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.swap(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.swap(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.swap(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.swap(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.swap(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.swap(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.swap(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SLA(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.shift_l(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.shift_l(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.shift_l(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.shift_l(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.shift_l(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.shift_l(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.shift_l(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.shift_l(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::SRA(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.shift_r(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.shift_r(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.shift_r(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.shift_r(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.shift_r(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.shift_r(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.shift_r(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.shift_r(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::SRL(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.shift_r_logical(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.shift_r_logical(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.shift_r_logical(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.shift_r_logical(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.shift_r_logical(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.shift_r_logical(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.shift_r_logical(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.shift_r_logical(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::RR(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.rotate_r_flag(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::RL(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.rotate_l_flag(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::RRC(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_r(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_r(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_r(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_r(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_r(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_r(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_r(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.rotate_r(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::RLC(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_l(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_l(value);
                        self.registers.b = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_l(value);
                        self.registers.c = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_l(value);
                        self.registers.d = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_l(value);
                        self.registers.e = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_l(value);
                        self.registers.h = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_l(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.registers.l;
                        let new_value = self.rotate_l(value);
                        self.registers.l = new_value;
                        self.pc.wrapping_add(1)
                    }
                }   
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };

                self.jump(jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };

                self.jump_relative(jump_condition)
            } 
            Instruction::JPI => {
                let value = self.registers.get_hl();

                value
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::HL => self.bus.read_byte(self.registers.get_hl()),
                            LoadByteSource::D8 => self.read_next_byte()
                        };

                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HL => self.bus.write_byte(self.registers.get_hl(), source_value)
                        };

                        match source {
                            LoadByteSource::D8  => self.pc.wrapping_add(2),
                            _                   => self.pc.wrapping_add(1),
                          }
                    }

                    LoadType::Word(target) => {
                        let word = self.read_next_word();

                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(word),
                            LoadWordTarget::DE => self.registers.set_de(word),
                            LoadWordTarget::HL => self.registers.set_hl(word),
                            LoadWordTarget::SP => self.sp = word
                        }

                        self.pc.wrapping_add(3)
                    }

                    LoadType::IndirectFromA(source) => {
                        self.registers.a = match source {
                            Indirect::BCIndirect => self.bus.read_byte(self.registers.get_bc()),
                            Indirect::DEIndirect => self.bus.read_byte(self.registers.get_de()),
                            Indirect::HLIndirectMinus => {
                                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.bus.read_byte(self.registers.get_hl())
                            },
                            Indirect::HLIndirectPlus => {
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                                self.bus.read_byte(self.registers.get_hl())
                            },
                            Indirect::LastByteIndirect => self.bus.read_byte(0xFF00 + self.registers.c as u16),
                            Indirect::WordIndirect => self.bus.read_byte(self.read_next_word())
                        };
                    
                        match source {
                            Indirect::WordIndirect => self.pc.wrapping_add(3),
                            _ => self.pc.wrapping_add(1)
                        }
                    }

                    LoadType::AFromIndirect(target) => {
                        let a = self.registers.a;

                        match target {
                            Indirect::BCIndirect => self.bus.write_byte(self.registers.get_bc(), a),
                            Indirect::DEIndirect => self.bus.write_byte(self.registers.get_de(), a),
                            Indirect::HLIndirectMinus => {
                                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.bus.write_byte(self.registers.get_hl(), a)
                            },
                            Indirect::HLIndirectPlus => {
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                                self.bus.write_byte(self.registers.get_hl(), a)
                            },
                            Indirect::LastByteIndirect => self.bus.write_byte((0xFF00 + self.registers.c) as u16, a),
                            Indirect::WordIndirect => self.bus.write_byte(self.read_next_word(), a)
                        }

                        match target {
                            Indirect::WordIndirect => self.pc.wrapping_add(3),
                            _ => self.pc.wrapping_add(1)
                        }
                    }

                    LoadType::AFromByteAddress => {
                        let offset = self.read_next_byte() as u16;
                        self.registers.a = self.bus.read_byte(0xFF00 + offset);
                        self.pc.wrapping_add(2)
                    }

                    LoadType::ByteAddressFromA => {
                        let offset = self.read_next_byte() as u16;
                        self.bus.write_byte(0xFF00 + offset, self.registers.a);
                        self.pc.wrapping_add(2)
                    }


                }
            }
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };

                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };

                self.ret(jump_condition)
            }
            Instruction::PUSH(target) => {
                match target {
                    StackTarget::AF => self.push(self.registers.get_af()),
                    StackTarget::BC => self.push(self.registers.get_bc()),
                    StackTarget::DE => self.push(self.registers.get_de()),
                    StackTarget::HL => self.push(self.registers.get_hl())
                }

                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let res = self.pop();
                match target {
                    StackTarget::AF => self.registers.set_af(res),
                    StackTarget::BC => self.registers.set_bc(res),
                    StackTarget::DE => self.registers.set_de(res),
                    StackTarget::HL => self.registers.set_hl(res)
                }

                self.pc.wrapping_add(1)
            }
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

    pub fn shift_r(&mut self, value: u8) -> u8 {
        let b7 = value & 0b10000000;
        let b0 = value & 0b00000001;
        let new_value = b7 | (value >> 1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b0 != 0;

        new_value
    }

    pub fn shift_l(&mut self, value: u8) -> u8 {
        let b7 = value & 0b10000000;
        let new_value = value << 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b7 != 0;

        new_value
    }

    pub fn shift_r_logical(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 0b1;

        new_value
    }

    pub fn rotate_r_flag(&mut self, value: u8) -> u8 {
        let b0 = value & 0b00000001;
        let transfer_bit = (self.registers.f.carry as u8) << 7;
        let new_value = transfer_bit | (value >> 1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b0 != 0;

        new_value
    }

    pub fn rotate_l_flag(&mut self, value: u8) -> u8 {
        let b7 = value & 0b10000000;
        let transfer_bit = self.registers.f.carry as u8;
        let new_value = (value << 1) | transfer_bit;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b7 != 0;

        new_value
    }

    pub fn rotate_r(&mut self, value: u8) -> u8 {
        let b0 = value & 0b00000001;
        let transfer_bit = b0 << 7;
        let new_value = transfer_bit | (value >> 1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b0 != 0;

        new_value
    }

    pub fn rotate_l(&mut self, value: u8) -> u8 {
        let b7 = value & 0b10000000;
        let transfer_bit = b7 >> 7;
        let new_value = (value << 1) | transfer_bit;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b7 != 0;

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

    pub fn swap(&mut self, value: u8) -> u8 {
        let upper_nibble = value & 0b11110000;
        let lower_nibble = value & 0b00001111;

        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        
        let new_value = (upper_nibble >> 4) | (lower_nibble << 4);
        new_value
    }

    pub fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;

            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    pub fn jump_relative(&self, should_jump: bool) -> u16 {
        if should_jump {
            let offset = self.bus.read_byte(self.pc + 1) as i8;
            (self.pc as i8 + offset) as u16
        } else {
            self.pc.wrapping_add(3)
        }
    }
}