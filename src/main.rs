#![allow(unused_variables)]
#![allow(dead_code)]
use std::io::Read;
use std::io::Write;

use json::JsonValue;

mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flag;

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
// 2: fail => Implement clock and proper interruptions
// 3: passes
// 4: passes
// 5: passes
// 6: passes
// 7: fail => Implement proper interruptions for 0xD9 RETI
// 8: passes
// 9: passes
// 10: passes
// 11: passes

fn main() {
    let mode = Mode::Main;

    let boot_rom_path = "./roms/dmg_boot.bin";
    let test_rom_path = "./tests_blargg/cpu_instrs/individual/01-special.gb";

    let boot_rom = read_rom(boot_rom_path);
    let game_rom = read_rom(test_rom_path);

    println!("ok!");

    let mut cycles : u64 = 0; // Using u64 for testing purposes
    
    match mode {
        Mode::Boot =>{
            let mut _cpu = cpu::CPU::new(boot_rom, game_rom);
            loop {
                if _cpu.pc >= 0x100 {
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Exiting...");
                    
                    std::process::exit(0)
                }

                cycles += _cpu.step() as u64;
            }
        }

        Mode::Main => {
            let mut _cpu = cpu::CPU::new(boot_rom, game_rom);
            loop {
                if _cpu.pc >= 0x100 && _cpu.is_booted == false {
                    _cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    println!("");
                }
            
                if !_cpu.is_halted {
                    cycles += _cpu.step() as u64;
                } else {
                    println!("PAUSE");
                }
            }
        }

        // This mode writes emulator state to a log file after every instruction
        // Change log path to ensure your old logs don't get overwritten
        Mode::Debug => {
            let mut _cpu = cpu::CPU::new(boot_rom, game_rom);
            let mut file = std::fs::File::create("./logs/log_9.txt").expect("error creating file");

            loop {
                if _cpu.pc >= 0x100 && _cpu.is_booted == false {
                    _cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", _cpu.registers.a, u8::from(_cpu.registers.f), _cpu.registers.b, _cpu.registers.c, _cpu.registers.d, _cpu.registers.e, _cpu.registers.h, _cpu.registers.l, _cpu.sp, _cpu.pc, _cpu.bus.read_byte(_cpu.pc), _cpu.bus.read_byte(_cpu.pc + 1), _cpu.bus.read_byte(_cpu.pc + 2), _cpu.bus.read_byte(_cpu.pc + 3)).expect("error logging to file");
                }
            
                if !_cpu.is_halted {
                    cycles += _cpu.step() as u64;
                    if cycles > 0 { cycles = 0; }
                    if _cpu.is_booted { writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", _cpu.registers.a, u8::from(_cpu.registers.f), _cpu.registers.b, _cpu.registers.c, _cpu.registers.d, _cpu.registers.e, _cpu.registers.h, _cpu.registers.l, _cpu.sp, _cpu.pc, _cpu.bus.read_byte(_cpu.pc), _cpu.bus.read_byte(_cpu.pc + 1), _cpu.bus.read_byte(_cpu.pc + 2), _cpu.bus.read_byte(_cpu.pc + 3)).expect("error logging to file"); }
                } else {
                    println!("PAUSE");
                }
            }
        }

        // This mode is still a WIP, not sure if it will ever get finished
        Mode::Test => {
            let test_rom: Vec<u8> = Vec::new();
            let mut _cpu = cpu::CPU::new(boot_rom, test_rom);
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
                            _cpu.registers.a = JsonValue::as_u8(&tests[t]["initial"]["a"]).expect("Is value a u8?");
                            _cpu.registers.b = JsonValue::as_u8(&tests[t]["initial"]["b"]).expect("Is value a u8?");
                            _cpu.registers.c = JsonValue::as_u8(&tests[t]["initial"]["c"]).expect("Is value a u8?");
                            _cpu.registers.d = JsonValue::as_u8(&tests[t]["initial"]["d"]).expect("Is value a u8?");
                            _cpu.registers.e = JsonValue::as_u8(&tests[t]["initial"]["e"]).expect("Is value a u8?");
                            _cpu.registers.h = JsonValue::as_u8(&tests[t]["initial"]["h"]).expect("Is value a u8?");
                            _cpu.registers.l = JsonValue::as_u8(&tests[t]["initial"]["l"]).expect("Is value a u8?");
                            _cpu.registers.f = FlagsRegister::from(JsonValue::as_u8(&tests[t]["initial"]["f"]).expect("Is value a flags register?"));
                            _cpu.pc = JsonValue::as_u16(&tests[t]["initial"]["pc"]).expect("Is value a u16?");
                            _cpu.sp = JsonValue::as_u16(&tests[t]["initial"]["sp"]).expect("Is value a u16?");
                            
                            // Initialise the memory
                            for m in 0..tests[t]["initial"]["ram"].len() {
                                let address = JsonValue::as_u16(&tests[t]["initial"]["ram"][m][0]).expect("Is value a u16 memory address?");
                                let byte = JsonValue::as_u8(&tests[t]["initial"]["ram"][m][1]).expect("Is value a u8?");
                                _cpu.bus.write_byte(address, byte);
                            }

                            // Run the test
                            let mut cycle: usize = 0;
                            let mut loopy = 0;
                            loop {
                                cycle += (_cpu.step() / 4) as usize;
                                loopy += 1;
                                println!("{}", cycle);
                                if !(tests[t]["cycles"][loopy].is_null()) { 
                                    let address = JsonValue::as_u16(&tests[t]["cycles"][cycle][0]).expect("Is value a u16 memory address?");
                                    let value = JsonValue::as_u8(&tests[t]["cycles"][cycle][0]).expect("Is value a u8?");
                                    let _type = JsonValue::as_str(&tests[t]["cycles"][cycle][0]).expect("Is value a string?");
                                
                                    match _type {
                                        "read" => {
                                            let byte = _cpu.bus.read_byte(address);
                                            println!("Just read byte 0x{:x} at address 0x{:4x}. The value given is {}", byte, address, value);
                                        }
                                        "write" => {
                                            _cpu.bus.write_byte(address, value);
                                            println!("Just written byte 0x{:x} at address 0x{:4x}.", value, address);
                                        }
                                        _ => { println!("This cycle's bus operation could not be retrieved."); }
                                    }
                                } else if loopy >= tests[t]["cycles"].len() {
                                    // Check emulator state with expected
                                    if !(_cpu.registers.a == JsonValue::as_u8(&tests[t]["final"]["a"]).expect("Is value a u8?")) { panic!("Error: register a wrong: 0x{:x}", _cpu.registers.a) }
                                    if !(_cpu.registers.b == JsonValue::as_u8(&tests[t]["final"]["b"]).expect("Is value a u8?")) { panic!("Error: register b wrong") }
                                    if !(_cpu.registers.c == JsonValue::as_u8(&tests[t]["final"]["c"]).expect("Is value a u8?")) { panic!("Error: register c wrong") }
                                    if !(_cpu.registers.d == JsonValue::as_u8(&tests[t]["final"]["d"]).expect("Is value a u8?")) { panic!("Error: register d wrong") }
                                    if !(_cpu.registers.e == JsonValue::as_u8(&tests[t]["final"]["e"]).expect("Is value a u8?")) { panic!("Error: register e wrong") }
                                    if !(_cpu.registers.h == JsonValue::as_u8(&tests[t]["final"]["h"]).expect("Is value a u8?")) { panic!("Error: register h wrong") }
                                    if !(_cpu.registers.l == JsonValue::as_u8(&tests[t]["final"]["l"]).expect("Is value a u8?")) { panic!("Error: register l wrong: 0x{:x}", _cpu.registers.l) }
                                    if !(_cpu.registers.f == FlagsRegister::from(JsonValue::as_u8(&tests[t]["final"]["f"]).expect("Is value a flags register?"))) { println!("Error: flags register wrong") }
                                    if !(_cpu.pc == JsonValue::as_u16(&tests[t]["final"]["pc"]).expect("Is value a u16?")) { panic!("Error: pc wrong") }
                                    if !(_cpu.sp == JsonValue::as_u16(&tests[t]["final"]["sp"]).expect("Is value a u16?")) { panic!("Error: sp wrong") }
                                
                                    println!("passed");
                                    break
                                }
                            }
                        }
                    }
                }
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