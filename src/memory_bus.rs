use crate::gpu::{GPU, VRAM_BEGIN, VRAM_END};

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
    gpu: GPU
}

impl MemoryBus {
    pub fn read_byte(&self, adress: u16) -> u8 {
        let adress = adress as usize;

        match adress {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.read_vram(adress - VRAM_BEGIN)
            }

            _ => panic!("Memory adress not supported yet.")
        }
    }

    pub fn write_byte(&mut self, adress: u16, byte: u8) {
        let address = adress as usize;

        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte)
            }

            _ => panic!("Memory adress not supported yet.")
        }
    }
}