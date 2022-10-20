
/*
0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0
    [Instruction]  Registers         [  Operand                       ]
[condition]      [ R1   ][ R2   ]
[^]
*/


pub const NOP: u8 = 0x00; //no operation
pub const LUB: u8 = 0x01; //load upper byte into R1
pub const LLB: u8 = 0x02; //load lower byte into R1
pub const EQ: u8 = 0x03; //sets 'equals' bitfield to 1 if R1 == R2
pub const CEQ: u8 = 0x04; //clears the 'equals' bitfield in flags register
pub const MOV: u8 = 0x05; //move value from R1 to R2
pub const ADD: u8 = 0x06; //adds R1 and R2 and stores in R1
pub const SUB: u8 = 0x07; //subtracts R2 from R1 and stores in R1
pub const CLR: u8 = 0x08;

pub const BOOTROM: [u32; 3] = [
    0x01_20_FFFF,
    0x02_20_CCCC,
    0x00_23_0000
    
    ];