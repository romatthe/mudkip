#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;

mod cpu;
mod nes;

use std::fs::File;
use std::io::Read;

use cpu::instructions;
use cpu::instructions::Instruction;
use nes::rom;

fn main() {
    // let mut file = File::open("smb.nes").unwrap();
    // let mut program = Vec::new();
    // file.read_to_end(&mut program);

    let rom = rom::load_from_file("smb.nes").unwrap();

    rom.prg_rom.iter()
        .map(move |instr| instructions::decode(*instr))
        .for_each(|instr| println!("{}", instr));

    println!("{:?}", rom);
}
