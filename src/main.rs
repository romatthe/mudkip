mod cpu;

use cpu::instructions;
use cpu::instructions::Instruction;

fn main() {
    println!("Hello, world!");

    let program: Vec<u8> = vec![0x00];
    program.iter()
        .map(move |instr| instructions::decode(*instr))
        .for_each(|instr| println!("{:?}", instr));
}
