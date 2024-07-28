pub struct InterruptFlag {
    pub vblank: bool,
    pub stat: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
    rest: u8
}

impl InterruptFlag {
    pub fn new() -> InterruptFlag {
        InterruptFlag {
            vblank: false,
            stat: false,
            timer: false,
            serial: false,
            joypad: false,
            rest: 0xF0
        }
    }

    pub fn from_byte(&mut self, byte: u8) {
        self.vblank = (byte & 0b00000001) != 0;
        self.stat = (byte & 0b00000010) != 0;
        self.timer = (byte & 0b00000100) != 0;
        self.serial = (byte & 0b00001000) != 0;
        self.joypad = (byte & 0b00010000) != 0;
        self.rest = byte & 0xF0;
    } 

    pub fn to_byte(&self) -> u8 {
        self.rest |
        ((self.joypad as u8) << 4) |
        ((self.serial as u8) << 3) |
        ((self.timer as u8) << 2) |
        ((self.stat as u8) << 1) |
        (self.vblank as u8)
    }
}