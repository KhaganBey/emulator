#![allow(unused_variables)]
#![allow(dead_code)]
use std::io::Read;

use json::JsonValue;

mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flag;

use cpu::instructions::Instruction;
enum Mode { 
    Main,
    Boot, 
    Test 
}

// Blargg CPU Tests
// 1: passes
// 2: fail
// 3: loop?
// 4: loop?
// 5: passes
// 6: passes
// 7: title loop
// 8: title loop
// 9: title loop
// 10: passes
// 11: fail

fn main() {
    let mode = Mode::Test;
    let boot_rom_path = "./roms/dmg_boot.bin";
    let test_rom_path = "./tests_blargg/cpu_instrs/individual/09-op r,r.gb";

    let boot_rom = read_rom(boot_rom_path);
    let game_rom = read_rom(test_rom_path);

    let mut _cpu = cpu::CPU::new(boot_rom, game_rom);
    println!("ok!");

    let mut cycles : u64 = 0; // Using u64 for testing purposes
    
    match mode {
        Mode::Boot =>{
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
            loop {
                if _cpu.pc >= 0x100 && _cpu.is_booted == false {
                    _cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                }
            
                if !_cpu.is_halted {
                    cycles += _cpu.step() as u64;
                    if cycles > 0 { cycles = 0; }
                } else {
                    println!("PAUSE");
                }
            }
        }

        Mode::Test => {
            println!("Instruction Tests Starting");

            for n in 0x00..255 {
                if n == 0xCB {
                    println!("");
                    println!("");
                    println!("");
                    println!("CB Instruction Tests Starting");
                }

                let inst: Option<Instruction> = Instruction::from_byte(n, false);
                match inst {
                    None => println!("Instruction of byte {} either not implemented or not part of test suite.", n),
                
                    _ => {
                        let test_path = format!("./tests/{:02x}.json", n);
                        println!("Doing the tests at {}", test_path);
                        let tests: JsonValue = read_json(&test_path);
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