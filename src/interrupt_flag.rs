pub struct InterruptFlag {
    pub vblank: bool,
    pub stat: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool
}

impl InterruptFlag {
    pub fn new() -> InterruptFlag {
        InterruptFlag {
            vblank: false,
            stat: false,
            timer: false,
            serial: false,
            joypad: false
        }
    }

    pub fn from_byte(&mut self, byte: u8) {
        self.vblank = (byte & 0b00000001) != 0;
        self.stat = (byte & 0b00000010) != 0;
        self.timer = (byte & 0b00000100) != 0;
        self.serial = (byte & 0b00001000) != 0;
        self.joypad = (byte & 0b00010000) != 0;
    } 

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0b00000000;

        byte = byte & (self.vblank as u8);
        byte = byte & ((self.stat as u8) << 1);
        byte = byte & ((self.timer as u8) << 2);
        byte = byte & ((self.serial as u8) << 3);
        byte = byte & ((self.joypad as u8) << 4);

        byte
    }
}