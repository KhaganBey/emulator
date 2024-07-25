#![allow(unused_variables)]
#![allow(while_true)]
#![allow(dead_code)]
#![allow(unused_mut)]
use std::io::Read;

mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flag;

fn main() {
    let boot_rom_path = "./roms/dmg_boot.bin";
    let test_rom_path = "./gb-test-roms-master/cpu_instrs/individual/01-special.gb";

    let boot_rom = buffer_from_file(boot_rom_path);
    let game_rom = buffer_from_file(test_rom_path);

    let mut _cpu = cpu::CPU::new(boot_rom, game_rom);
    println!("ok!");

    let mut cycles : u64 = 0; // Using u64 for testing purposes
    let mut file = std::fs::File::create("./logs/log.txt").expect("Error");
    while true {

        if _cpu.pc >= 0x100 && _cpu.is_booted == false {
            _cpu.is_booted = true;
            println!(""); // 329480 CPU cycles later
            println!(" S U C C E S S ");
            println!("Boot successfuly completed! Continuing...");
            //std::process::exit(0)
        }

        if !_cpu.is_halted {
            cycles += _cpu.step() as u64;
            if cycles > 0 { cycles = 0; }
        } else {
            println!("PAUSE");
        }
    }
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Could not read file");
    
    buffer
}