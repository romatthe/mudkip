extern crate ansi_term;
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate clap;
#[macro_use]
extern crate nom;



mod cpu;
mod nes;

use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use ansi_term::Colour::Red;
use byteorder::{LittleEndian, ReadBytesExt};
use clap::{Arg, App, AppSettings, SubCommand};
use cpu::disassembler;
use cpu::instructions;
use cpu::instructions::Instruction;
use nes::NES;
use nes::rom;
use nes::rom::ROM;

fn main() {
    let input = App::new("Mudkip")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1")
        .author("Robin Mattheussen <robin.mattheussen@gmail.com>")
        .about("A bare-bones NES emulator")
        .subcommand(SubCommand::with_name("disassemble")
            .about("Disassembles the target ROM")
            .version("1.0")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("/path/to/file")
                .required(true)
                .help("Path to the ROM you want to disassemble")))
        .get_matches();

    match input.subcommand() {
        // Use the disassemble function of the emulator
        ("disassemble", Some(file_input)) => {
            let path_str = file_input.value_of("file").unwrap();
            let path = Path::new(path_str);

            match File::open(path) {
                Ok(mut file) => {
                    let rom = rom::load_from_file(&mut file).unwrap();
                    disassembler::disassemble(rom);
                }
                Err(err) => eprintln!("{} Invalid path {:?} specified!", Red.bold().paint("error:"), path)
            }
        }

        _ => ()
    }
}
