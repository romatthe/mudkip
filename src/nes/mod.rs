pub mod rom;

use std::str;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use nom::{IResult, be_u8};

const TRAINER_LENGTH: usize = 512;
const PRG_ROM_PAGE_LENGTH: usize = 16384;
const CHR_ROM_PAGE_LENGTH: usize = 8192;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ScreenMode {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum System {
    NES,
    VsUnisystem,
    PlayChoice10,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Region {
    PAL,
    NTSC
}

// Structure for the Flags6 bitflags:
// Ref: https://wiki.nesdev.com/w/index.php/INES#Flags_6
bitflags! {
    struct Flags6: u8 {
        const VERTICAL =        0b0000_0001;
        const SRAM =            0b0000_0010;
        const TRAINER =         0b0000_0100;
        const FOUR_SCREEN =     0b0000_1000;
        const LOWER_NYBBLE =    0b1111_0000;
    }
}


// Structure for the Flags7 bitflags:
// Ref: https://wiki.nesdev.com/w/index.php/INES#Flags_7
bitflags! {
    struct Flags7: u8 {
        const VS_UNISYSTEM =    0b0000_0001;
        const PLAYCHOICE_10 =   0b0000_0010;
        const NES_20 =          0b0000_1100;
    }
}

// Structure for the Flags9 bitflags:
// Ref: https://wiki.nesdev.com/w/index.php/INES#Flags_9
bitflags! {
    struct Flags9: u8 {
        const TV_SYSTEM =       0b0000_0001;    // (0: NTSC; 1: PAL)
    }
}

impl Into<ScreenMode> for Flags6 {
    fn into(self) -> ScreenMode {
        if self.contains(Flags6::FOUR_SCREEN) {
            ScreenMode::FourScreen
        } else if self.contains(Flags6::VERTICAL) {
            ScreenMode::Vertical
        } else {
            ScreenMode::Horizontal
        }
    }
}

impl Into<System> for Flags7 {
    fn into(self) -> System {
        if self.contains(Flags7::VS_UNISYSTEM) {
            System::VsUnisystem
        } else if self.contains(Flags7::PLAYCHOICE_10) {
            System::PlayChoice10
        } else {
            System::NES
        }
    }
}

impl Into<Region> for Flags9 {
    fn into(self) -> Region {
        if self.contains(Flags9::TV_SYSTEM) {
            Region::PAL
        } else {
            Region::NTSC
        }
    }
}

// Note: thus currently only supports the iNES format, not the NES2.0 format
// Ref: https://wiki.nesdev.com/w/index.php/INES
#[derive(Debug)]
struct Rom {
    header: Header,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>
}

impl Rom {
    pub fn new(header: Header, prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Rom {
        Rom { header: header, prg_rom: prg_rom, chr_rom: chr_rom }
    }
}

#[derive(Debug)]
struct Header {
    prg_size: usize,
    chr_size: usize,
    trainer: bool,
    screen_mode: ScreenMode,
    system: System,
    region: Region,
}

pub fn test() {
    let mut f = File::open("smb.nes").expect("File not found");
    let mut buf: Vec<u8> = vec![];
    f.read_to_end(&mut buf).unwrap();

    match parse_ines(&buf) {
        IResult::Incomplete(needed) => println!("{:?}", needed),
        IResult::Done(_, val)       => println!("{:?}", val),
        _                           => println!("Aww shucks!")
    };
}

impl Header {
    pub fn new(prg_size: usize, chr_size: usize, flags6: u8, flags7: u8, prg_ram: u8, flags9: u8, flags10: u8) -> Header {
        let flags6 = Flags6::from_bits(flags6).expect("Failed to parse bit-flags from ROM"); // Parse the u8 into a Flags6 bitflag structure
        let flags7 = Flags7::from_bits(flags7).expect("Failed to parse bit-flags from ROM"); // Parse the u8 into a Flags7 bitflag structure
        let flags9 = Flags9::from_bits(flags9).expect("Failed to parse bit-flags from ROM"); // Parse the u8 into a Flags9 bitflag structure

        Header {
            prg_size: prg_size,
            chr_size: chr_size,
            trainer: flags6.contains(Flags6::TRAINER),
            screen_mode: flags6.into(),
            system: flags7.into(),
            region: flags9.into()
        }
    }
}

named!(parse_ines<&[u8], Rom>,
    do_parse!(
        header:     parse_header     >>
                    // If bit 2 in flags6 is set (aka the Trainer flag), the next 512 bytes contain a trainer
                    cond!(header.trainer, take!(TRAINER_LENGTH)) >>
        prg_rom:    take!(header.prg_size * PRG_ROM_PAGE_LENGTH) >>
        chr_rom:    take!(header.chr_size * CHR_ROM_PAGE_LENGTH) >>

        (Rom::new(header, prg_rom.into(), chr_rom.into()))
    )
);

named!(parse_header<&[u8], Header>,
    do_parse!(
                    tag!("NES")     >>
                    tag!([0x1a])    >>
        prg_size:   be_u8           >>
        chr_size:   be_u8           >>
        flags6:     be_u8           >>
        flags7:     be_u8           >>
        prg_ram:    be_u8           >>
        flags9:     be_u8           >>
        flags10:    be_u8           >>
                    take!(5)        >>  // These are the remaining 5 0x00 bytes

        (Header::new(prg_size as usize, chr_size as usize, flags6, flags7, prg_ram, flags9, flags10))
    )
);