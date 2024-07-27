use crate::cpu::CPU;

pub struct Timer {
    pub cpu: CPU,
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    edge: u8,
    tima_overflow: bool,
    overflow_cycle: TCycle,
    new_tima: u8,
    new_tma: u8
}

enum TCycle {
    ZERO,
    ONE,
    TWO,
    THREE
}

impl Timer {
    pub fn new(cpu: CPU) -> Timer {
        Timer {
            cpu: cpu,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            edge: 0,
            tima_overflow: false,
            overflow_cycle: TCycle::ZERO,
            new_tima: 0,
            new_tma: 0
        }
    }

    pub fn ticks(&mut self, cycles: u8) {
        for i in 0..cycles {
            self.tick();
        }
    }

    fn tick(&mut self) {
        // Increment div register
        self.div = self.div.wrapping_add(1);
        if self.new_tma != 0 { self.new_tma = 0; }

        // Check for overflow
        self.overflow_cycle = match self.overflow_cycle {
            TCycle::ZERO => {
                if self.tima_overflow {
                    self.tima = self.tma;
                    if self.new_tima != 0 { self.new_tima = 0; }
                    self.cpu.bus.interrupt_flag.timer = true;
                }

                self.get_next_t()
            }
            _ => { self.get_next_t() } 
        };

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
            let (new_value, did_overflow) = self.tima.overflowing_add(1);
            self.tima_overflow = did_overflow;
            self.overflow_cycle = TCycle::ONE;
        } 
        self.edge = and_result;

        if self.new_tima != 0 { 
            self.tima = self.new_tima;
            self.new_tima = 0;
         }

    }

    pub fn get_next_t(&self) -> TCycle {
        match self.overflow_cycle {
            TCycle::ZERO => TCycle::ONE,
            TCycle::ONE => TCycle::TWO,
            TCycle::TWO => TCycle::THREE,
            TCycle::THREE =>TCycle::ZERO
        }
    }

    pub fn read_div(&self) -> u8 {
        ((self.div & 0b11110000) >> 8) as u8
    }

    pub fn write_div(&mut self) {
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