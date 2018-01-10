pub mod rom;

use std::fs::File;
use std::io::stdin;

use cpu::Cpu;
use ROM;

pub struct NES {
    cpu: Cpu,
    rom: ROM
}

impl NES {
    pub fn new(mut file: File) -> NES {
        NES { cpu: Cpu::new(), rom: rom::load_from_file(&mut file).unwrap() }
    }

    pub fn load_rom(&mut self, mut file: File) {
        self.cpu = Cpu::new();
        self.rom = rom::load_from_file(&mut file).unwrap();
    }

    pub fn run(mut self) {
        self.cpu.program = self.rom.prg_rom;

        println!("{:?}", self.cpu.program);
        let mut guess = String::new();

        loop {
            self.cpu.step();

            stdin().read_line(&mut guess)
                .expect("Failed to read line");
        }
    }
}