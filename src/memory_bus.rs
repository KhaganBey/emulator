use crate::gpu::GPU;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const ROM_BANK_N_BEGIN: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_BEGIN + 1;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const EXTERNAL_RAM_BEGIN: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const ECHO_RAM_BEGIN: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const UNUSED_BEGIN: usize = 0xFEA0;
pub const UNUSED_END: usize = 0xFEFF;

pub const IO_REGISTERS_BEGIN: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;
pub const IO_REGISTERS_SIZE: usize = IO_REGISTERS_END - IO_REGISTERS_BEGIN;

pub const ZERO_PAGE_BEGIN: usize = 0xFF80;
pub const ZERO_PAGE_END: usize = 0xFFFE;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_BEGIN + 1;

pub const INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;

pub struct MemoryBus {
    boot_rom: [u8; BOOT_ROM_SIZE],
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
    io_temp: [u8; IO_REGISTERS_SIZE],
    gpu: GPU
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Vec<u8>, game_rom: Vec<u8>) -> MemoryBus {
        let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);

        if boot_rom.len() != BOOT_ROM_SIZE { panic!("Invalid boot rom, size does not match reality."); }

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        for i in 0 ..= ROM_BANK_0_SIZE - 1 {
            rom_bank_0[i] = game_rom[i];
        }

        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        for i in 0 ..= ROM_BANK_N_SIZE - 1 {
            rom_bank_n[i] = game_rom[ROM_BANK_0_SIZE + i];
        }

        let mut io_temp = [0; IO_REGISTERS_SIZE];
        io_temp[0xff44 - IO_REGISTERS_BEGIN] = 0x90;

        MemoryBus {
            boot_rom,
            rom_bank_0,
            rom_bank_n,
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            io_temp,
            gpu: GPU::new()
        }
    }

    pub fn step(&mut self, cycles: u8) {
        //
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;

        match address {
            BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                if let Some(boot_rom) = Some(self.boot_rom) {
                    boot_rom[address]
                } else {
                    self.rom_bank_0[address]
                }
            }
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                self.rom_bank_0[address]
            }
            ROM_BANK_N_BEGIN ..= ROM_BANK_N_END => self.rom_bank_n[address - ROM_BANK_N_BEGIN],
            VRAM_BEGIN ..= VRAM_END => { self.gpu.read_vram(address - VRAM_BEGIN) }
            WORKING_RAM_BEGIN ..= WORKING_RAM_END => self.working_ram[address - WORKING_RAM_BEGIN],
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => self.working_ram[address - ECHO_RAM_BEGIN],
            //OAM_BEGIN ..= OAM_END =>
            IO_REGISTERS_BEGIN ..= IO_REGISTERS_END => {
                println!("IO registers are not implemented yet, but the execution of the program will continue.");
                self.io_temp[address - IO_REGISTERS_BEGIN]
            }
            UNUSED_BEGIN ..= UNUSED_END => { 0 }
            ZERO_PAGE_BEGIN ..= ZERO_PAGE_END => self.zero_page[address - ZERO_PAGE_BEGIN],

            _ => panic!("Memory address 0x{:x} invalid or not supported yet.", address)
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;

        match address {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                self.rom_bank_0[address] = byte;
            }
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte)
            }
            EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
                self.external_ram[address - EXTERNAL_RAM_BEGIN] = byte;
            }
            WORKING_RAM_BEGIN ..= WORKING_RAM_END => {
                self.working_ram[address - WORKING_RAM_BEGIN] = byte;
            }
            OAM_BEGIN ..= OAM_END => {
                println!("OAM is not implemented yet, but the execution of the program will continue.");
            }
            IO_REGISTERS_BEGIN..=IO_REGISTERS_END => {
                println!("IO Registers are not implemented yet, but the execution of the program will continue.");
                self.io_temp[address - IO_REGISTERS_BEGIN] = byte;
            }
            UNUSED_BEGIN ..= UNUSED_END => { }
            ZERO_PAGE_BEGIN ..= ZERO_PAGE_END => {
                self.zero_page[address - ZERO_PAGE_BEGIN] = byte;
            }
            //INTERRUPT_ENABLE_REGISTER => 

            _ => panic!("Memory address 0x{:x} invalid or not supported yet.", address)
        }
    }
}