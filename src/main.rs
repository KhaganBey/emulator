#![allow(unused_variables)]
#![allow(while_true)]
#![allow(dead_code)]
#![allow(unused_mut)]
use std::io::Read;

mod cpu;
mod gpu;
mod memory_bus;

fn main() {
    let boot_rom_path = "./roms/dmg.bin";
    let test_rom_path = "./gb-test-roms-master/cpu_instrs/individual/08-misc instrs.gb";

    let boot_rom = buffer_from_file(boot_rom_path);
    let game_rom = buffer_from_file(test_rom_path);

    let mut _cpu = cpu::CPU::new(boot_rom, game_rom);
    println!("ok!");

    while true {
        _cpu.step(); 
    }
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Could not read file");
    
    buffer
}