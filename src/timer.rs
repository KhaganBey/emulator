pub struct Timer {
    div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    edge: u8,
    tima_overflow: bool,
    overflow_cycle: u8,
    new_tima: u8,
    new_tma: u8
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            edge: 0,
            tima_overflow: false,
            overflow_cycle: 0,
            new_tima: 0,
            new_tma: 0
        }
    }

    pub fn ticks(&mut self, cycles: u8) -> bool {
        let mut timer_interrupt = false;

        for i in 0..cycles {
            timer_interrupt = timer_interrupt | self.tick();
        }
    
        timer_interrupt
    }

    pub fn tick(&mut self) -> bool {
        // Increment div register
        let mut timer_interrupt = false;
        self.div = self.div.wrapping_add(1);
        if self.new_tma != 0 { self.new_tma = 0; }

        // Check for overflow
        if self.tima_overflow {
                if self.overflow_cycle == 4 {
                    self.tima = self.tma;
                    if self.new_tima != 0 { self.new_tima = 0; }
                    timer_interrupt = true;
                    self.overflow_cycle = 0;
                } else {
                    self.overflow_cycle += 1;
                }
        }

        // Get the and_result
        let tac_lower = self.tac & 0b00000011;
        let bit_pos: u8 = match tac_lower {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => 0
        };
        let div_bit: u8 = ((self.div & (0b0000000000000001 << bit_pos)) >> bit_pos) as u8;
        let timer_enable: u8 = (self.tac & 0b00000100) >> 2;

        let and_result = div_bit & timer_enable;


        // Check for falling edge
        if self.edge == 1 && and_result == 0 {
            if self.tima == 0xFF {
                self.tima_overflow = true;
                self.overflow_cycle = 1;
            } else {
                self.tima += 1;
            }
        } 
        self.edge = and_result;

        if self.new_tima != 0 { 
            self.tima = self.new_tima;
            self.new_tima = 0;
         }

         timer_interrupt
    }

    pub fn read_div(&self) -> u8 {
        ((self.div & 0b11110000) >> 8) as u8
    }

    pub fn write_div(&mut self, byte: u8) {
        self.div = 0;
    }
    
    pub fn write_tima(&mut self, new_value: u8) {
        self.tima_overflow = false;
        self.new_tima = new_value;
    }

    pub fn write_tma(&mut self, new_value: u8) {
        self.new_tma = new_value;
    }
}