use crate::datatypes::*;
use crate::consts::*;

mod consts;
mod datatypes;




fn execute(cmd: Word, cpu: &mut Cpu) {
    let r1 = cpu.get_first_reg_id(cmd);
    let r2 = cpu.get_second_reg_id(cmd);

    let r1_val = to_int(cpu.get_reg(r1));
    let r2_val = to_int(cpu.get_reg(r2));

    let operand = to_int(cmd) & 0xFFFF;

    if (to_int(cpu.reg.flags) & 1 == 0 && (cmd[0] >> 7) == 1 && cmd[0] != 0xFF) {
        return;
    }

    

    match ((cmd[0] & 0x3F)) {
        NOP => {},
        LLB => {cpu.set_reg(r1, word((r1_val & 0xFFFF0000) | operand));},
        LUB => {cpu.set_reg(r1, word((r1_val & 0x0000FFFF) | (operand << 16)));},
        MOV => {cpu.set_reg(r1, word(r2_val))},
        EQ => {cpu.reg.flags = word(to_int(cpu.reg.flags) | ((r1_val == r2_val) as u32))},
        CEQ=> {cpu.reg.flags = word(to_int(cpu.reg.flags) & 0)}
        ADD => {cpu.set_reg(r1, word(r1_val + r2_val))},
        SUB => {cpu.set_reg(r1, word(r1_val - r2_val))},
        CLR => {cpu.set_reg(r1, word(0)); cpu.set_reg(r2, word(0))},
        0x3F => {std::process::exit(1);}
        _ => {}
    }

}

fn main() {
    let mut cpu = Cpu::new();
    cpu.memory.load(&BOOTROM, 0);


    while true {

        cpu.memory.debug_tick();

        execute(cpu.memory.get(to_int(cpu.reg.pc)), &mut cpu);

        cpu.reg.pc = word(to_int(cpu.reg.pc) + 1);
    }

}
