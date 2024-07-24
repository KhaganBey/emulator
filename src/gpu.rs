use crate::memory_bus::VRAM_SIZE;

#[derive(Copy,Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384]
}

impl GPU {
    pub fn new() ->GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [empty_tile(); 384]
        }
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;
        /*if address > 0x1800 { return }

        let normalised_address = address & 0xFFEF;

        let byte1 = self.vram[normalised_address];
        let byte2 = self.vram[normalised_address + 1];

        let tile_address = address / 16;
        let row_address = (address % 16) / 2;

        for pixel_address in 0 ..= 8 {
            let mask = 1 << (7 - pixel_address);
            let least_significant_byte = byte1 & mask;
            let most_significant_byte = byte2 & mask;

            let value = match (least_significant_byte != 0, most_significant_byte != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero
            };

            self.tile_set[tile_address][row_address][pixel_address] = value;
        }*/
    }
}