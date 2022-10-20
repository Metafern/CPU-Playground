use std::array;
use crate::consts::*;

pub type Word = [u8; 4];

const RAM_SIZE: u64 = 0xFFFF;




pub fn word(num: u32) -> Word {return [(num >> 24 & 0xFF) as u8, (num >> 16 & 0xFF) as u8, (num >> 8 & 0xFF) as u8, (num >> 0 & 0xFF) as u8]}
pub fn to_int(word: Word) -> u32 {
    return (word[0] as u32) << 24 | (word[1] as u32) << 16 | (word[2] as u32) << 8 | (word[3] as u32) << 0;
}
pub struct Registers {
    pub a: Word,
    pub b: Word,
    pub c: Word,
    pub d: Word,
    pub sp: Word,
    pub pc: Word,
    pub addr: Word,
    pub flags: Word
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {a: word(0), b: word(0), c: word(0), d: word(0), sp: word(0), pc: word(0), addr: word(0), flags: word(0)};
    }
}

pub struct Mmu {
    gp: [Word; 0xFFFF],
    debug: [Word; 0xFFFF]
}

impl Mmu {
    pub fn new() -> Mmu {return Mmu {gp: [word(0xFFFFFFFF); 0xFFFF], debug: [word(0); 0xFFFF]};}
    
    pub fn get(&self, addr: u32) -> Word {
        return self.gp[addr as usize];
    }

    pub fn set(&mut self, addr: u32, value: u32) {
        if addr < 0xFFFF {
            self.gp[addr as usize] = word(value);
        }
        else {
            self.debug[addr as usize - 0xFFFF] = word(value);
        }
    }

    pub fn load(&mut self, data: &[u32], mut offset: usize) {

        if offset < 0xFFFF {
            for val in data {
                self.gp[offset] = word(*val);
                offset += 1;
            }
        }
        else {
            for val in data {
                self.debug[offset - 0xFFFF] = word(*val);
                offset += 1;
            }
        }

    }

    pub fn debug_tick(&mut self) {

        if to_int(self.debug[0xFFFE]) == 0x00 {
            return;
        }

        let mut out = String::new();
        'outer: for i in self.debug {
            for j in i {
                if j != 0 {
                    out.push(j as char);
                }
                else {
                    break 'outer;
                }
            }
        }
        print!("{out}");
        self.debug[0xFFFE] = word(0);
    }
}

pub struct Cpu {
    pub reg: Registers,
    pub memory: Mmu
}

impl Cpu {

    pub fn new() -> Cpu {return Cpu {reg: Registers::new(), memory: Mmu::new()};}

    pub fn get_first_reg_id(&mut self, cmd: Word) -> u8 {
       return (cmd[1] >> 4) & 0x0F;
    }
    
    pub fn get_second_reg_id(&mut self, cmd: Word) -> u8 {
        return (cmd[1] >> 0) & 0x0F;
     }

     pub fn set_reg(&mut self, id: u8, value: Word) {
        match id {
            0 => {self.reg.sp = value},
            1 => {self.reg.pc = value},
            2 => {self.reg.a = value},
            3 => {self.reg.b = value},
            4 => {self.reg.c = value},
            5 => {self.reg.d = value},
            _ => {}
        }
     }

     pub fn get_reg(&self, id: u8) -> Word {
        return match id {
            0 => self.reg.sp,
            1 => self.reg.pc,
            2 => self.reg.a,
            3 => self.reg.b,
            4 => self.reg.c,
            5 => self.reg.d,
            _ => word(0xCCCCCCCC)
        }
     }
}

