#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use std::io::Read;
use std::io::Write;

mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flag;
mod timer;

enum Mode { 
    Main,
    Boot, 
    Debug
}

fn main() {
    let mode = Mode::Main;

    let boot_rom_path = "./roms/dmg_boot.bin";
    let test_rom_path = "./tests_blargg/instr_timing/instr_timing.gb";

    let boot_rom = read_rom(boot_rom_path);
    let game_rom = read_rom(test_rom_path);

    println!("ok!");
    
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

                let mut cycles = _cpu.step();
                if cycles != 4 { panic!("Unhandled CPU cycle detected.") }

                while cycles > 0 {
                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        //println!("Requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        //println!("Requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        //println!("Requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        //println!("Requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    cycles = 0;
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
            
                let mut cycles = _cpu.step();
                if cycles != 4 { panic!("Unhandled CPU cycle detected.") }

                while cycles > 0 {
                    if _cpu.pc > 0xC2D2 && _cpu.pc < 0xC2CF { println!("Main:"); }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        println!("Main requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        println!("Main requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        println!("Main requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                        println!("Main requesting timer interrupt at pc 0x{:02X}", _cpu.pc);
                    }

                    cycles = 0;
                }
            }
        }

        // This mode writes emulator state to a log file after every instruction
        // Change log path to ensure your old logs don't get overwritten
        Mode::Debug => {
            let memory_bus = memory_bus::MemoryBus::new(boot_rom, game_rom);
            let mut _cpu = cpu::CPU::new(memory_bus);
            let mut file = std::fs::File::create("./logs/log_itiming.txt").expect("error creating file");

            loop {
                if _cpu.pc >= 0x100 && _cpu.is_booted == false {
                    _cpu.is_booted = true;
                    println!(""); // 329480 CPU cycles later
                    println!(" S U C C E S S ");
                    println!("Boot successfuly completed! Continuing...");
                    //writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", _cpu.registers.a, u8::from(_cpu.registers.f), _cpu.registers.b, _cpu.registers.c, _cpu.registers.d, _cpu.registers.e, _cpu.registers.h, _cpu.registers.l, _cpu.sp, _cpu.pc, _cpu.bus.read_byte(_cpu.pc), _cpu.bus.read_byte(_cpu.pc + 1), _cpu.bus.read_byte(_cpu.pc + 2), _cpu.bus.read_byte(_cpu.pc + 3)).expect("error logging to file");
                }
            
                let mut cycles = _cpu.step();
                if cycles != 4 { panic!("Unhandled CPU cycle detected.") }

                while cycles > 0 {
                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                    }

                    if _cpu.bus.timer.tick() {
                        _cpu.bus.request_timer_interrupt();
                    }

                    cycles = 0;
                }

                if _cpu.is_booted { writeln!(&mut file, "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X}). tima: {:08b}. if: {:08b}", _cpu.registers.a, u8::from(_cpu.registers.f), _cpu.registers.b, _cpu.registers.c, _cpu.registers.d, _cpu.registers.e, _cpu.registers.h, _cpu.registers.l, _cpu.sp, _cpu.pc, _cpu.bus.read_byte(_cpu.pc), _cpu.bus.read_byte(_cpu.pc + 1), _cpu.bus.read_byte(_cpu.pc + 2), _cpu.bus.read_byte(_cpu.pc + 3), _cpu.bus.timer.tima, _cpu.bus.interrupt_flag.to_byte()).expect("error logging to file"); }
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