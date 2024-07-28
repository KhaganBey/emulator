#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use std::io::Read;
use std::io::Write;

use json::JsonValue;

mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flag;
mod timer;

//use cpu::instructions::Instruction;
//use cpu::flags_register::FlagsRegister;

enum Mode { 
    Main,
    Boot, 
    Debug
}

// Blargg CPU Tests
// 1: passes
// 2: passes
// 3: passes
// 4: passes
// 5: passes
// 6: passes
// 7: passes
// 8: passes
// 9: passes
// 10: passes
// 11: passes

fn main() {
    let mode = Mode::Main;

    let boot_rom_path = "./roms/dmg_boot.bin";
    let test_rom_path = "./tests_blargg/cpu_instrs/individual/02-interrupts.gb";

    let boot_rom = read_rom(boot_rom_path);
    let game_rom = read_rom(test_rom_path);

    println!("ok!");

    let mut cycles : u8 = 0;
    
    match mode {
        Mode::Boot =>{
            let memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut _cpu = cpu::CPU::new(memory_bus);
            loop {
                if _cpu.pc >= 0x100 {
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Exiting...");
                    
                    std::process::exit(0)
                }

                cycles = _cpu.step();
                let timer_interrupt = _cpu.bus.timer.ticks(cycles);
                if timer_interrupt {
                    _cpu.bus.request_timer_interrupt();
                }
            }
        }

        Mode::Main => {
            let memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut _cpu = cpu::CPU::new(memory_bus);
            loop {
                if _cpu.pc >= 0x100 && _cpu.is_booted == false {
                    _cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    println!("");
                }
            
                cycles = _cpu.step();
                let timer_interrupt = _cpu.bus.timer.ticks(cycles);
                if timer_interrupt {
                    _cpu.bus.request_timer_interrupt();
                }
            }
        }

        // This mode writes emulator state to a log file after every instruction
        // Change log path to ensure your old logs don't get overwritten
        Mode::Debug => {
            let memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut _cpu = cpu::CPU::new(memory_bus);
            let mut file = std::fs::File::create("./logs/log_2.txt").expect("error creating file");

            loop {
                if _cpu.pc >= 0x100 && _cpu.is_booted == false {
                    _cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    //writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", _cpu.registers.a, u8::from(_cpu.registers.f), _cpu.registers.b, _cpu.registers.c, _cpu.registers.d, _cpu.registers.e, _cpu.registers.h, _cpu.registers.l, _cpu.sp, _cpu.pc, _cpu.bus.read_byte(_cpu.pc), _cpu.bus.read_byte(_cpu.pc + 1), _cpu.bus.read_byte(_cpu.pc + 2), _cpu.bus.read_byte(_cpu.pc + 3)).expect("error logging to file");
                }
            
                cycles = _cpu.step();
                let timer_interrupt = _cpu.bus.timer.ticks(cycles);
                if timer_interrupt {
                    _cpu.bus.request_timer_interrupt();
                }
                if _cpu.is_booted { writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", _cpu.registers.a, u8::from(_cpu.registers.f), _cpu.registers.b, _cpu.registers.c, _cpu.registers.d, _cpu.registers.e, _cpu.registers.h, _cpu.registers.l, _cpu.sp, _cpu.pc, _cpu.bus.read_byte(_cpu.pc), _cpu.bus.read_byte(_cpu.pc + 1), _cpu.bus.read_byte(_cpu.pc + 2), _cpu.bus.read_byte(_cpu.pc + 3)).expect("error logging to file"); }
            }
        }
    }
}

fn read_rom(path: &str) -> Vec<u8> {
    let error_message: String = format!("Could not read rom at {}", path.to_string());

    let mut file = std::fs::File::open(path).expect(&error_message);
    let mut rom = Vec::new();

    file.read_to_end(&mut rom).expect(&error_message);
    
    rom
}

fn read_json(path: &str) -> JsonValue {
    let error_message: String = format!("Could not read test file at {}", path.to_string());
    let mut file = std::fs::File::open(path).expect(&error_message);

    let mut array = String::new();
    file.read_to_string(&mut array).expect(&error_message);

    let json_array = json::parse(&array).expect(&error_message);
    json_array
}