#![allow(unused_variables)]
#![allow(dead_code)]
use std::io::Read;
use std::io::Write;

use json::JsonValue;

mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flag;
mod timer;

use cpu::instructions::Instruction;
use cpu::flags_register::FlagsRegister;

enum Mode { 
    Main,
    Boot, 
    Test,
    Debug
}

// Blargg CPU Tests
// 1: passes
// 2: fail => Implement clock and proper interruption handling
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
            let mut timer = timer::Timer::new(_cpu);
            loop {
                if timer.cpu.pc >= 0x100 {
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Exiting...");
                    
                    std::process::exit(0)
                }

                cycles = timer.cpu.step();
                timer.ticks(cycles);
                cycles = 0;
            }
        }

        Mode::Main => {
            let memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut _cpu = cpu::CPU::new(memory_bus);
            let mut timer = timer::Timer::new(_cpu);
            loop {
                if timer.cpu.pc >= 0x100 && timer.cpu.is_booted == false {
                    timer.cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    println!("");
                }
            
                if !timer.cpu.is_halted {
                    cycles = timer.cpu.step();
                    timer.ticks(cycles);
                    cycles = 0;
                } else {
                    println!("PAUSE");
                }
            }
        }

        // This mode writes emulator state to a log file after every instruction
        // Change log path to ensure your old logs don't get overwritten
        Mode::Debug => {
            let memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut _cpu = cpu::CPU::new(memory_bus);
            let mut timer = timer::Timer::new(_cpu);
            let mut file = std::fs::File::create("./logs/log_7.txt").expect("error creating file");

            loop {
                if timer.cpu.pc >= 0x100 && timer.cpu.is_booted == false {
                    timer.cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", timer.cpu.registers.a, u8::from(timer.cpu.registers.f), timer.cpu.registers.b, timer.cpu.registers.c, timer.cpu.registers.d, timer.cpu.registers.e, timer.cpu.registers.h, timer.cpu.registers.l, timer.cpu.sp, timer.cpu.pc, timer.cpu.bus.read_byte(timer.cpu.pc), timer.cpu.bus.read_byte(timer.cpu.pc + 1), timer.cpu.bus.read_byte(timer.cpu.pc + 2), timer.cpu.bus.read_byte(timer.cpu.pc + 3)).expect("error logging to file");
                }
            
                if !timer.cpu.is_halted {
                    cycles = timer.cpu.step();
                    timer.ticks(cycles);
                    cycles = 0;
                    if timer.cpu.is_booted { writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", timer.cpu.registers.a, u8::from(timer.cpu.registers.f), timer.cpu.registers.b, timer.cpu.registers.c, timer.cpu.registers.d, timer.cpu.registers.e, timer.cpu.registers.h, timer.cpu.registers.l, timer.cpu.sp, timer.cpu.pc, timer.cpu.bus.read_byte(timer.cpu.pc), timer.cpu.bus.read_byte(timer.cpu.pc + 1), timer.cpu.bus.read_byte(timer.cpu.pc + 2), timer.cpu.bus.read_byte(timer.cpu.pc + 3)).expect("error logging to file"); }
                } else {
                    println!("PAUSE");
                }
            }
        }
        Mode::Test => {}
        // This mode is still a WIP, not sure if it will ever get finished
        /*Mode::Test => {
            let test_rom: Vec<u8> = Vec::new();
            let mut memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut timer.cpu = cpu::CPU::new(memory_bus);
            let mut timer = timer::Timer::new(memory_bus);
            println!("Instruction Tests Starting");

            for n in 0x00..10 {
                if n == 0xCB {
                    println!("");
                    println!("");
                    println!("");
                    println!("CB Instruction Tests Starting");
                }

                println!("");

                let inst: Option<Instruction> = Instruction::from_byte(n, false);
                match inst {
                    None => println!("Instruction of byte {} either not implemented or not part of test suite.", n),
                
                    _ => {
                        let test_path = format!("./tests/{:02x}.json", n);
                        let tests: JsonValue = read_json(&test_path);
                        println!("Starting the tests at {}", test_path);

                        for t in 0..tests.len() {
                            println!("Executing test {} at {}", tests[t]["name"], test_path);
                            // Initialise the cpu
                            timer.cpu.registers.a = JsonValue::as_u8(&tests[t]["initial"]["a"]).expect("Is value a u8?");
                            timer.cpu.registers.b = JsonValue::as_u8(&tests[t]["initial"]["b"]).expect("Is value a u8?");
                            timer.cpu.registers.c = JsonValue::as_u8(&tests[t]["initial"]["c"]).expect("Is value a u8?");
                            timer.cpu.registers.d = JsonValue::as_u8(&tests[t]["initial"]["d"]).expect("Is value a u8?");
                            timer.cpu.registers.e = JsonValue::as_u8(&tests[t]["initial"]["e"]).expect("Is value a u8?");
                            timer.cpu.registers.h = JsonValue::as_u8(&tests[t]["initial"]["h"]).expect("Is value a u8?");
                            timer.cpu.registers.l = JsonValue::as_u8(&tests[t]["initial"]["l"]).expect("Is value a u8?");
                            timer.cpu.registers.f = FlagsRegister::from(JsonValue::as_u8(&tests[t]["initial"]["f"]).expect("Is value a flags register?"));
                            timer.cpu.pc = JsonValue::as_u16(&tests[t]["initial"]["pc"]).expect("Is value a u16?");
                            timer.cpu.sp = JsonValue::as_u16(&tests[t]["initial"]["sp"]).expect("Is value a u16?");
                            
                            // Initialise the memory
                            for m in 0..tests[t]["initial"]["ram"].len() {
                                let address = JsonValue::as_u16(&tests[t]["initial"]["ram"][m][0]).expect("Is value a u16 memory address?");
                                let byte = JsonValue::as_u8(&tests[t]["initial"]["ram"][m][1]).expect("Is value a u8?");
                                timer.cpu.bus.write_byte(address, byte);
                            }

                            // Run the test
                            let mut cycle: usize = 0;
                            let mut loopy = 0;
                            loop {
                                cycle += (timer.cpu.step() / 4) as usize;
                                loopy += 1;
                                println!("{}", cycle);
                                if !(tests[t]["cycles"][loopy].is_null()) { 
                                    let address = JsonValue::as_u16(&tests[t]["cycles"][cycle][0]).expect("Is value a u16 memory address?");
                                    let value = JsonValue::as_u8(&tests[t]["cycles"][cycle][0]).expect("Is value a u8?");
                                    let _type = JsonValue::as_str(&tests[t]["cycles"][cycle][0]).expect("Is value a string?");
                                
                                    match _type {
                                        "read" => {
                                            let byte = timer.cpu.bus.read_byte(address);
                                            println!("Just read byte 0x{:x} at address 0x{:4x}. The value given is {}", byte, address, value);
                                        }
                                        "write" => {
                                            timer.cpu.bus.write_byte(address, value);
                                            println!("Just written byte 0x{:x} at address 0x{:4x}.", value, address);
                                        }
                                        _ => { println!("This cycle's bus operation could not be retrieved."); }
                                    }
                                } else if loopy >= tests[t]["cycles"].len() {
                                    // Check emulator state with expected
                                    if !(timer.cpu.registers.a == JsonValue::as_u8(&tests[t]["final"]["a"]).expect("Is value a u8?")) { panic!("Error: register a wrong: 0x{:x}", timer.cpu.registers.a) }
                                    if !(timer.cpu.registers.b == JsonValue::as_u8(&tests[t]["final"]["b"]).expect("Is value a u8?")) { panic!("Error: register b wrong") }
                                    if !(timer.cpu.registers.c == JsonValue::as_u8(&tests[t]["final"]["c"]).expect("Is value a u8?")) { panic!("Error: register c wrong") }
                                    if !(timer.cpu.registers.d == JsonValue::as_u8(&tests[t]["final"]["d"]).expect("Is value a u8?")) { panic!("Error: register d wrong") }
                                    if !(timer.cpu.registers.e == JsonValue::as_u8(&tests[t]["final"]["e"]).expect("Is value a u8?")) { panic!("Error: register e wrong") }
                                    if !(timer.cpu.registers.h == JsonValue::as_u8(&tests[t]["final"]["h"]).expect("Is value a u8?")) { panic!("Error: register h wrong") }
                                    if !(timer.cpu.registers.l == JsonValue::as_u8(&tests[t]["final"]["l"]).expect("Is value a u8?")) { panic!("Error: register l wrong: 0x{:x}", timer.cpu.registers.l) }
                                    if !(timer.cpu.registers.f == FlagsRegister::from(JsonValue::as_u8(&tests[t]["final"]["f"]).expect("Is value a flags register?"))) { println!("Error: flags register wrong") }
                                    if !(timer.cpu.pc == JsonValue::as_u16(&tests[t]["final"]["pc"]).expect("Is value a u16?")) { panic!("Error: pc wrong") }
                                    if !(timer.cpu.sp == JsonValue::as_u16(&tests[t]["final"]["sp"]).expect("Is value a u16?")) { panic!("Error: sp wrong") }
                                
                                    println!("passed");
                                    break
                                }
                            }
                        }
                    }
                }
            }
        }*/
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