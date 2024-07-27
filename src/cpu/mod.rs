pub mod flags_register;
pub mod instructions;
pub mod registers;

use crate::memory_bus::MemoryBus;
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

pub const VBLANK: u16 = 0x40;
pub const STAT: u16 = 0x48;
pub const TIMER: u16 = 0x50;
pub const SERIAL: u16 = 0x58;
pub const JOYPAD: u16 = 0x60;

pub struct CPU { 
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub is_halted: bool,
    pub is_booted: bool,
    pub ime: bool
}

impl CPU {
    pub fn new(boot_rom: Vec<u8>, game_rom: Vec<u8>) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x00,
            sp: 0x00,
            bus: MemoryBus::new(boot_rom, game_rom),
            is_halted: false,
            is_booted: false,
            ime: true
        }
    }


    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            let previous = instruction_byte;
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        
        let (next_pc, mut cycles) = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:2x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found: {} at 0x{:4x}", description, self.pc);
        };
        
        self.bus.step(cycles);
        if !self.is_halted { self.pc = next_pc; }
        if self.bus.interrupted() { self.is_halted = false; }

        let mut interrupted = false;
        if self.ime && self.bus.interrupted() {
            if self.bus.interrupt_enable.vblank && self.bus.interrupt_flag.vblank {
                interrupted = true;
                self.bus.interrupt_flag.vblank = false;
                self.interrupt(VBLANK)
            }
            if self.bus.interrupt_enable.stat && self.bus.interrupt_flag.stat {
                interrupted = true;
                self.bus.interrupt_flag.stat = false;
                self.interrupt(STAT)
            }
            if self.bus.interrupt_enable.timer && self.bus.interrupt_flag.timer {
                interrupted = true;
                self.bus.interrupt_flag.timer = false;
                self.interrupt(TIMER)
            }
            if self.bus.interrupt_enable.serial && self.bus.interrupt_flag.serial {
                interrupted = true;
                self.bus.interrupt_flag.serial = false;
                self.interrupt(SERIAL)
            }
            if self.bus.interrupt_enable.joypad && self.bus.interrupt_flag.joypad {
                interrupted = true;
                self.bus.interrupt_flag.joypad = false;
                self.interrupt(JOYPAD)
            }
        }

        if interrupted { cycles += 20 }

        cycles
    }

    fn interrupt(&mut self, location: u16) {
        self.ime = false;
        self.push(self.pc);
        self.pc = location;
        self.bus.step(20);
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let least_significant_byte = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        
        let most_significant_bye = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        
        (most_significant_bye << 8) | least_significant_byte
    }

    fn call(&mut self, should_jump: bool) -> (u16, u8) {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            (self.read_next_word(), 24)
        } else {
            (next_pc, 12)
        }
    }

    fn ret(&mut self, should_jump: bool, always: bool) -> (u16, u8) {
        if always {
            (self.pop(), 16)
        } else if should_jump {
            (self.pop(), 20)
        } else {
            (self.pc.wrapping_add(1), 8)
        }
    }

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    pub fn read_next_word(&self) -> u16 {
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc + 1) as u16)
        
    }

    fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        match instruction {
            Instruction::NOP => {
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::HALT => {
                self.is_halted = true;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => { 
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => { 
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => { 
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => { 
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::ADDHL(target) => { 
                match target {
                    ADDHLTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    ADDHLTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    ADDHLTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    ADDHLTarget::SP => {
                        let value = self.sp;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.sub(value);
                        self.registers.a = new_value as u8;
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.and(value);
                        self.registers.a = new_value as u8;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::OR(target) => {
                match target  {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.or(value);
                        self.registers.a = new_value as u8;
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.cp(value);
                        (self.pc.wrapping_add(1), 4)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.cp(value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        self.cp(value);
                        (self.pc.wrapping_add(2), 8)
                    }
                }
            }
            Instruction::INC(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.inc(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.inc(value);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.inc(value);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.inc(value);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.inc(value);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.inc(value);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.inc(value);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.inc16(value);
                        self.registers.set_bc(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.inc16(value);
                        self.registers.set_de(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.inc16(value);
                        self.registers.set_hl(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::SP => {
                        let value = self.sp;
                        let new_value = self.inc16(value);
                        self.sp = new_value;
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.inc(value);
                        self.bus.write_byte(hl, new_value);
                        (self.pc.wrapping_add(1), 12)
                    }
                }
            }
            Instruction::DEC(target) => {
                match target {
                    IncDecTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.dec(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.dec(value);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.dec(value);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.dec(value);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.dec(value);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.dec(value);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.dec(value);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(1), 4)
                    }
                    IncDecTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.dec16(value);
                        self.registers.set_bc(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.dec16(value);
                        self.registers.set_de(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.dec16(value);
                        self.registers.set_hl(new_value);
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::SP => {
                        let value = self.sp;
                        let new_value = self.dec16(value);
                        self.sp = new_value;
                        (self.pc.wrapping_add(1), 8)
                    }
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.dec(value);
                        self.bus.write_byte(hl, new_value);
                        (self.pc.wrapping_add(1), 12)
                    }
                }
            }
            Instruction::CCF => {
                self.ccf();
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::SCF => {
                self.scf();
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::CPL => {
                self.cpl();
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RRA => {
                let new_value = self.rotate_r_flag(self.registers.a, false);
                self.registers.a = new_value;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RLA => {
                let new_value = self.rotate_l_flag(self.registers.a, false);
                self.registers.a = new_value;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RRCA => {
                let new_value = self.rotate_r(self.registers.a, false);
                self.registers.a = new_value;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RLCA => {
                let new_value = self.rotate_l(self.registers.a, false);
                self.registers.a = new_value;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::BIT(target, bit_position) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.bit_test(value, bit_position);
                        (self.pc.wrapping_add(2), 12)
                    }
                }
            }
            Instruction::SET(target, bit_position) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        self.registers.a = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        self.registers.b = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        self.registers.c = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        self.registers.d = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        self.registers.e = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        self.registers.h = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        self.registers.l = self.bit_set(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.bit_set(value, bit_position);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }
            }
            Instruction::RES(target, bit_position) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        self.registers.a = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        self.registers.b = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        self.registers.c = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        self.registers.d = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        self.registers.e = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        self.registers.h = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        self.registers.l = self.bit_reset(value, bit_position);
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.bit_reset(value, bit_position);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.swap(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.swap(value);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.swap(value);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.swap(value);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.swap(value);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.swap(value);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.swap(value);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.swap(value);
                        self.bus.write_byte(hl, new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }
            }
            Instruction::SLA(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.shift_l(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.shift_l(value);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.shift_l(value);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.shift_l(value);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.shift_l(value);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.shift_l(value);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.shift_l(value);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.shift_l(value);
                        self.bus.write_byte(hl, new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }   
            }
            Instruction::SRA(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.shift_r(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.shift_r(value);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.shift_r(value);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.shift_r(value);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.shift_r(value);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.shift_r(value);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.shift_r(value);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.shift_r(value);
                        self.bus.write_byte(hl, new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }   
            }
            Instruction::SRL(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.shift_r_logical(value);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.shift_r_logical(value);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.shift_r_logical(value);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.shift_r_logical(value);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.shift_r_logical(value);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.shift_r_logical(value);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.shift_r_logical(value);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.shift_r_logical(value);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }   
            }
            Instruction::RR(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_r_flag(value, true);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.rotate_r_flag(value, true);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }   
            }
            Instruction::RL(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_l_flag(value, true);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.rotate_l_flag(value, true);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }   
            }
            Instruction::RRC(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_r(value, true);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_r(value, true);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_r(value, true);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_r(value, true);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_r(value, true);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_r(value, true);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_r(value, true);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.rotate_r(value, true);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
                    }
                }   
            }
            Instruction::RLC(target) => {
                match target {
                    PrefixTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.rotate_l(value, true);
                        self.registers.a = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.rotate_l(value, true);
                        self.registers.b = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.rotate_l(value, true);
                        self.registers.c = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.rotate_l(value, true);
                        self.registers.d = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.rotate_l(value, true);
                        self.registers.e = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.rotate_l(value, true);
                        self.registers.h = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.rotate_l(value, true);
                        self.registers.l = new_value;
                        (self.pc.wrapping_add(2), 8)
                    }
                    PrefixTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.rotate_l(value, true);
                        self.bus.write_byte(self.registers.get_hl(), new_value);
                        (self.pc.wrapping_add(2), 16)
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

                (value, 4)
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
                            LoadByteSource::D8  => match target {
                                    LoadByteTarget::HL => (self.pc.wrapping_add(2), 12),
                                    _ => (self.pc.wrapping_add(2), 8)
                            }
                            LoadByteSource::HL =>(self.pc.wrapping_add(1), 8),
                            _                   => match target {
                                LoadByteTarget::HL => (self.pc.wrapping_add(1), 8),
                                _ => (self.pc.wrapping_add(1), 4)
                            }
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

                        (self.pc.wrapping_add(3), 12)
                    }

                    LoadType::AFromIndirect(source) => {
                        self.registers.a = match source {
                            Indirect::BCIndirect => self.bus.read_byte(self.registers.get_bc()),
                            Indirect::DEIndirect => self.bus.read_byte(self.registers.get_de()),
                            Indirect::HLIndirectMinus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_sub(1));
                                self.bus.read_byte(hl)
                            },
                            Indirect::HLIndirectPlus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_add(1));
                                self.bus.read_byte(hl)
                            },
                            Indirect::LastByteIndirect => self.bus.read_byte(0xFF00 + self.registers.c as u16),
                            Indirect::WordIndirect => self.bus.read_byte(self.read_next_word())
                        };
                    
                        match source {
                            Indirect::WordIndirect => (self.pc.wrapping_add(3), 16),
                            _ => (self.pc.wrapping_add(1), 8)
                        }
                    }

                    LoadType::IndirectFromA(target) => {
                        let a = self.registers.a;

                        match target {
                            Indirect::BCIndirect => {
                                let bc = self.registers.get_bc();
                                self.bus.write_byte(bc, a)
                            }
                            Indirect::DEIndirect => {
                                let de = self.registers.get_de();
                                self.bus.write_byte(de, a)
                            }
                            Indirect::HLIndirectMinus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_sub(1));
                                self.bus.write_byte(hl, a);
                            }
                            Indirect::HLIndirectPlus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_add(1));
                                self.bus.write_byte(hl, a);
                            }
                            Indirect::WordIndirect => {
                                let word = self.read_next_word();
                                self.bus.write_byte(word, a);
                            }
                            Indirect::LastByteIndirect => {
                                let c = self.registers.c as u16;
                                self.bus.write_byte(0xFF00 + c, a);
                            }
                        }

                        match target {
                            Indirect::WordIndirect => (self.pc.wrapping_add(3), 16),
                            _ => (self.pc.wrapping_add(1), 8)
                        }
                    }

                    LoadType::AFromByteAddress => {
                        let offset = self.read_next_byte() as u16;
                        self.registers.a = self.bus.read_byte(0xFF00 + offset);
                        (self.pc.wrapping_add(2), 12)
                    }

                    LoadType::ByteAddressFromA => {
                        let offset = self.read_next_byte() as u16;
                        self.bus.write_byte(0xFF00 + offset, self.registers.a);
                        (self.pc.wrapping_add(2), 12)
                    }

                    LoadType::SPFromHL => {
                        self.sp = self.registers.get_hl();
                        (self.pc.wrapping_add(1), 8)
                    }

                    LoadType::WordFromSP => {
                        let address = self.read_next_word();
                        let sp = self.sp;
                        
                        self.bus.write_byte(address, (sp & 0x00FF) as u8);
                        self.bus.write_byte(address.wrapping_add(1), ((sp & 0xFF00) >> 8) as u8);

                        (self.pc.wrapping_add(3), 20)
                    }

                    LoadType::HLFromSPPlus => {
                        let offset = self.read_next_byte() as i8 as i16 as u16;
                        let result = self.sp.wrapping_add(offset);
                        self.registers.set_hl(result);

                        self.registers.f.zero = false;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (self.sp & 0xF) + (offset & 0xF) > 0xF;
                        self.registers.f.carry = (self.sp & 0xFF) + (offset & 0xFF) > 0xFF;

                        (self.pc.wrapping_add(2), 12)
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

                let always = match test {
                    JumpTest::Always => true,
                    _ => false
                };

                self.ret(jump_condition, always)
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl()
                };
                self.push(value);
                (self.pc.wrapping_add(1), 16)
            }
            Instruction::POP(target) => {
                let res = self.pop();
                match target {
                    StackTarget::AF => self.registers.set_af(res),
                    StackTarget::BC => self.registers.set_bc(res),
                    StackTarget::DE => self.registers.set_de(res),
                    StackTarget::HL => self.registers.set_hl(res)
                }

                (self.pc.wrapping_add(1), 12)
            }
            Instruction::DI => {
                self.ime = false;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::EI => {
                self.ime = true;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::DAA => {
                let new_value = self.decimal_adjust(self.registers.a);
                self.registers.a = new_value;

                (self.pc.wrapping_add(1), 4)
            }
            Instruction::ADDSP => {
                let byte = self.read_next_byte() as i8 as i16 as u16;
                let new_value = self.sp.wrapping_add(byte);

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.carry = (self.sp & 0xFF) + (byte & 0xFF) > 0xFF;
                self.registers.f.half_carry = (self.sp & 0xF) + (byte & 0xF) > 0xF;

                self.sp = new_value;
                (self.pc.wrapping_add(2), 16)
            }
            Instruction::RST(target) => {
                self.push(self.pc.wrapping_add(1));
                (target.to_hex(), 16)
            }
            Instruction::RETI => {
                let new_pc = self.pop();
                (new_pc, 4)
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
        
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half carry tests if the sum of the lower three nibbles of the value and the register(s)
        // HL fit in 12 bits, or 0xFFF. If yes, then the flag is set to true. 
        self.registers.f.half_carry = (value & 0xFFF) + (hl & 0xFFF) > 0xFFF;

        new_value
    }

    pub fn adc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (newer_value, did_overflow2) = new_value.overflowing_add(self.registers.f.carry as u8);

        self.registers.f.zero = newer_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) + (self.registers.a & 0xF) + (self.registers.f.carry as u8) > 0xF;
        self.registers.f.carry = did_overflow | did_overflow2;

        newer_value
    }

    pub fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);

        new_value
    }

    pub fn sbc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let (newer_value, did_overflow2) = new_value.overflowing_sub(self.registers.f.carry as u8);

        self.registers.f.zero = newer_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + (self.registers.f.carry as u8);
        self.registers.f.carry = did_overflow | did_overflow2;

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

    pub fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    pub fn cp(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        self.registers.f.carry = self.registers.a < value;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
    }

    pub fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = value & 0xF == 0xF;

        new_value
    }

    pub fn inc16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_add(1);

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

    pub fn rotate_r_flag(&mut self, value: u8, zero: bool) -> u8 {
        let transfer_bit = (self.registers.f.carry as u8) << 7;
        let new_value = transfer_bit | (value >> 1);

        self.registers.f.zero = zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;

        new_value
    }

    pub fn rotate_l_flag(&mut self, value: u8, zero: bool) -> u8 {
        let transfer_bit = self.registers.f.carry as u8;
        let new_value = (value << 1) | transfer_bit;

        self.registers.f.zero = zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;

        new_value
    }

    pub fn rotate_r(&mut self, value: u8, zero: bool) -> u8 {
        let b0 = value & 0b00000001;
        let transfer_bit = b0 << 7;
        let new_value = transfer_bit | (value >> 1);

        self.registers.f.zero = zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b0 != 0;

        new_value
    }

    pub fn rotate_l(&mut self, value: u8, zero: bool) -> u8 {
        let b7 = (value & 0x80) >> 7;
        let new_value = (value << 1) | b7;

        self.registers.f.zero = zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = b7 == 1;

        new_value
    }

    pub fn bit_test(&mut self, value: u8, bit_pos: BitPosition) {
        let position : u8 = bit_pos.into();
        let bit = (value >> position) & 0b1;
        
        self.registers.f.zero = bit == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    pub fn bit_set(&mut self, value: u8, bit_pos: BitPosition) -> u8 {
        let position : u8 = bit_pos.into();
        value | (1 << position)
    }

    pub fn bit_reset(&mut self, value: u8, bit_pos: BitPosition) -> u8 {
        let pos: u8 = bit_pos.into();
        value & !(1 << pos)
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

    pub fn jump(&self, should_jump: bool) -> (u16, u8) {
        
        if should_jump {
            (self.read_next_word(), 16)
        } else {
            (self.pc.wrapping_add(3), 12)
        }
    }

    pub fn jump_relative(&self, should_jump: bool) -> (u16, u8) {
        let next_pc = self.pc.wrapping_add(2);

        if should_jump {
            let offset = self.read_next_byte() as i8;

            let nexter_pc = if offset >= 0 {
                next_pc.wrapping_add(offset as u16)
            } else {
                next_pc.wrapping_sub(offset.abs() as u16)
            };
            (nexter_pc, 12)
        } else {
            (next_pc, 8)
        }
    }

    pub fn decimal_adjust(&mut self, value: u8) -> u8 {
        let mut offset: u8 = 0;
        let f = self.registers.f;

        if f.half_carry || ((value & 0x0F) > 0x09 && !f.subtract) {
            offset |= 0x06;
        }

        if f.carry || (value > 0x99 && !f.subtract) {
            offset |= 0x60;
            self.registers.f.carry = true;
        }

        let new_value: u8 = if self.registers.f.subtract { value.wrapping_sub(offset) } else { value.wrapping_add(offset) };

        self.registers.f.zero = new_value == 0;
        self.registers.f.half_carry = false;

        new_value
    }
}