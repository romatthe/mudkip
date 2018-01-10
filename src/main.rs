#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate nom;



mod cpu;
mod nes;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use cpu::disassembler;
use cpu::instructions;
use cpu::instructions::Instruction;
use nes::NES;
use nes::rom;
use nes::rom::ROM;

fn main() {
    let mut file = File::open("smb.nes").unwrap();
    let rom = rom::load_from_file(&mut file).unwrap();

    disassembler::disassemble(rom);

    let file = File::open("smb.nes").unwrap();
    // let mut program = Vec::new();
    // file.read_to_end(&mut program);

    //let rom = rom::load_from_file("smb.nes").unwrap();

    //let nes = NES::new(file);
    //&nes.run();

    //rom.prg_rom.iter()
    //    .map(move |instr| instructions::decode(*instr))
    //    .for_each(|instr| println!("{}", instr));

    //println!("{:?}", rom);
}
