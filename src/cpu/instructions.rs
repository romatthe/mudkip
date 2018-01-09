use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use cpu::AddressingMode;

type OpCode = u8;

// Mnemonics for all 6502 CPU instructions
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php/6502_Opcodes
#[derive(PartialEq, Debug)]
pub enum Mnemonic {
    LDA, LDX, LDY, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,     // Storage
    ADC, DEC, DEX, DEY, INC, INX, INY, SBC,                         // Math
    AND, ASL, BIT, EOR, LSR, ORA, ROL, ROR,                         // Bitwise
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,                         // Branch
    JMP, JSR, RTI, RTS,                                             // Jump
    CLC, CLD, CLI, CLV, CMP, CPX, CPY, SEC, SED, SEI,               // Registers
    PHA, PHP, PLA, PLP,                                             // Stack
    BRK, NOP,                                                       // System
    UNKNOWN
}

// Decodes a single-byte opcode into a richer Instruction data structure
// Ref: http://www.6502.org/tutorials/6502opcodes.html
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php/6502_Opcodes
pub fn decode(opcode: OpCode) -> Instruction {
    let (mnemonic, mode, length, cycles) =
        match opcode {
            // LDA
            0xa9 => (Mnemonic::LDA, AddressingMode::IMM, 2, 2),
            0xa5 => (Mnemonic::LDA, AddressingMode::ZPG, 2, 3),
            0xb5 => (Mnemonic::LDA, AddressingMode::ZPX, 2, 4),
            0xad => (Mnemonic::LDA, AddressingMode::ABS, 3, 4),
            0xbd => (Mnemonic::LDA, AddressingMode::ABX, 3, 4),
            0xb9 => (Mnemonic::LDA, AddressingMode::ABY, 3, 4),
            0xa1 => (Mnemonic::LDA, AddressingMode::IDX, 2, 6),
            0xb1 => (Mnemonic::LDA, AddressingMode::IDY, 2, 5),
            // LDX
            0xa2 => (Mnemonic::LDX, AddressingMode::IMM, 2, 2),
            0xa6 => (Mnemonic::LDX, AddressingMode::ZPG, 2, 3),
            0xb6 => (Mnemonic::LDX, AddressingMode::ZPY, 2, 4),
            0xae => (Mnemonic::LDX, AddressingMode::ABS, 3, 4),
            0xbe => (Mnemonic::LDX, AddressingMode::ABY, 3, 4),
            // LDY
            0xa0 => (Mnemonic::LDY, AddressingMode::IMM, 2, 2),
            0xa4 => (Mnemonic::LDY, AddressingMode::ZPG, 2, 3),
            0xb4 => (Mnemonic::LDY, AddressingMode::ZPX, 2, 4),
            0xac => (Mnemonic::LDY, AddressingMode::ABS, 3, 4),
            0xbc => (Mnemonic::LDY, AddressingMode::ABX, 3, 4),
            // STA
            0x85 => (Mnemonic::STA, AddressingMode::ZPG, 2, 3),
            0x95 => (Mnemonic::STA, AddressingMode::ZPX, 2, 4),
            0x8d => (Mnemonic::STA, AddressingMode::ABS, 3, 4),
            0x9d => (Mnemonic::STA, AddressingMode::ABX, 3, 5),
            0x99 => (Mnemonic::STA, AddressingMode::ABY, 3, 5),
            0x81 => (Mnemonic::STA, AddressingMode::IDX, 2, 6),
            0x91 => (Mnemonic::STA, AddressingMode::IDY, 2, 6),
            // STX
            0x86 => (Mnemonic::STX, AddressingMode::ZPG, 2, 3),
            0x96 => (Mnemonic::STX, AddressingMode::ZPY, 2, 4),
            0x8e => (Mnemonic::STX, AddressingMode::ABS, 3, 4),
            // STY
            0x84 => (Mnemonic::STY, AddressingMode::ZPG, 2, 3),
            0x94 => (Mnemonic::STY, AddressingMode::ZPX, 2, 4),
            0x8c => (Mnemonic::STY, AddressingMode::ABS, 3, 4),
            // TAX
            0xaa => (Mnemonic::TAX, AddressingMode::IMP, 1, 2),
            // TAY
            0xa8 => (Mnemonic::TAY, AddressingMode::IMP, 1, 2),
            // TSX
            0xba => (Mnemonic::TSX, AddressingMode::IMP, 1, 2),
            // TXA
            0x8a => (Mnemonic::TXA, AddressingMode::IMP, 1, 2),
            // TXS
            0x9a => (Mnemonic::TXS, AddressingMode::IMP, 1, 2),
            // TYA
            0x98 => (Mnemonic::TYA, AddressingMode::IMP, 1, 2),
            // ADC
            0x69 => (Mnemonic::ADC, AddressingMode::IMM, 2, 2),
            0x65 => (Mnemonic::ADC, AddressingMode::ZPG, 2, 3),
            0x75 => (Mnemonic::ADC, AddressingMode::ZPX, 2, 4),
            0x6d => (Mnemonic::ADC, AddressingMode::ABS, 3, 4),
            0x7d => (Mnemonic::ADC, AddressingMode::ABX, 3, 4),
            0x79 => (Mnemonic::ADC, AddressingMode::ABY, 3, 4),
            0x61 => (Mnemonic::ADC, AddressingMode::IDX, 2, 6),
            0x71 => (Mnemonic::ADC, AddressingMode::IDY, 2, 5),
            // DEC
            0xc6 => (Mnemonic::DEC, AddressingMode::ZPG, 2, 5),
            0xd6 => (Mnemonic::DEC, AddressingMode::ZPX, 2, 6),
            0xce => (Mnemonic::DEC, AddressingMode::ABS, 3, 6),
            0xde => (Mnemonic::DEC, AddressingMode::ABX, 3, 7),
            // DEX
            0xca => (Mnemonic::DEX, AddressingMode::IMP, 1, 2),
            // DEY
            0x88 => (Mnemonic::DEY, AddressingMode::IMP, 1, 2),
            // INC
            0xe6 => (Mnemonic::INC, AddressingMode::ZPG, 2, 5),
            0xf6 => (Mnemonic::INC, AddressingMode::ZPX, 2, 6),
            0xee => (Mnemonic::INC, AddressingMode::ABS, 3, 6),
            0xfe => (Mnemonic::INC, AddressingMode::ABX, 3, 7),
            // INX
            0xe8 => (Mnemonic::INX, AddressingMode::IMP, 1, 2),
            // INY
            0xc8 => (Mnemonic::INY, AddressingMode::IMP, 1, 2),
            // SBC
            0xe9 => (Mnemonic::SBC, AddressingMode::IMM, 2, 2),
            0xe5 => (Mnemonic::SBC, AddressingMode::ZPG, 2, 3),
            0xf5 => (Mnemonic::SBC, AddressingMode::ZPX, 2, 4),
            0xed => (Mnemonic::SBC, AddressingMode::ABS, 3, 5),
            0xfd => (Mnemonic::SBC, AddressingMode::ABX, 3, 5),
            0xf9 => (Mnemonic::SBC, AddressingMode::ABY, 3, 5),
            0xe1 => (Mnemonic::SBC, AddressingMode::IDX, 2, 6),
            0xf1 => (Mnemonic::SBC, AddressingMode::IDY, 2, 5),
            // AND
            0x29 => (Mnemonic::AND, AddressingMode::IMM, 2, 2),
            0x25 => (Mnemonic::AND, AddressingMode::ZPG, 2, 3),
            0x35 => (Mnemonic::AND, AddressingMode::ZPX, 2, 4),
            0x2d => (Mnemonic::AND, AddressingMode::ABS, 3, 4),
            0x3d => (Mnemonic::AND, AddressingMode::ABX, 3, 4),
            0x39 => (Mnemonic::AND, AddressingMode::ABY, 3, 4),
            0x21 => (Mnemonic::AND, AddressingMode::IDX, 2, 6),
            0x31 => (Mnemonic::AND, AddressingMode::IDY, 2, 5),
            // ASL
            0x0a => (Mnemonic::ASL, AddressingMode::ACC, 1, 2),
            0x06 => (Mnemonic::ASL, AddressingMode::ZPG, 2, 5),
            0x16 => (Mnemonic::ASL, AddressingMode::ZPX, 2, 6),
            0x0e => (Mnemonic::ASL, AddressingMode::ABS, 4, 6),
            0x1e => (Mnemonic::ASL, AddressingMode::ABX, 4, 7),
            // BIT
            0x24 => (Mnemonic::BIT, AddressingMode::ZPG, 2, 3),
            0x2c => (Mnemonic::BIT, AddressingMode::ABS, 3, 4),
            // EOR
            0x49 => (Mnemonic::EOR, AddressingMode::IMM, 2, 2),
            0x45 => (Mnemonic::EOR, AddressingMode::ZPG, 2, 3),
            0x55 => (Mnemonic::EOR, AddressingMode::ZPX, 2, 4),
            0x4d => (Mnemonic::EOR, AddressingMode::ABS, 3, 4),
            0x5d => (Mnemonic::EOR, AddressingMode::ABX, 3, 4),
            0x59 => (Mnemonic::EOR, AddressingMode::ABY, 3, 4),
            0x41 => (Mnemonic::EOR, AddressingMode::IDX, 2, 6),
            0x51 => (Mnemonic::EOR, AddressingMode::IDY, 2, 5),
            // LSR
            0x4a => (Mnemonic::LSR, AddressingMode::ACC, 1, 2),
            0x46 => (Mnemonic::LSR, AddressingMode::ZPG, 2, 5),
            0x56 => (Mnemonic::LSR, AddressingMode::ZPX, 2, 6),
            0x4e => (Mnemonic::LSR, AddressingMode::ABS, 3, 6),
            0x5e => (Mnemonic::LSR, AddressingMode::ABX, 3, 7),
            // ORA
            0x09 => (Mnemonic::ORA, AddressingMode::IMM, 2, 2),
            0x05 => (Mnemonic::ORA, AddressingMode::ZPG, 2, 3),
            0x15 => (Mnemonic::ORA, AddressingMode::ZPX, 2, 4),
            0x0d => (Mnemonic::ORA, AddressingMode::ABS, 3, 4),
            0x1d => (Mnemonic::ORA, AddressingMode::ABX, 3, 4),
            0x19 => (Mnemonic::ORA, AddressingMode::ABY, 3, 4),
            0x01 => (Mnemonic::ORA, AddressingMode::IDX, 2, 6),
            0x11 => (Mnemonic::ORA, AddressingMode::IDY, 2, 5),
            // ROL
            0x2a => (Mnemonic::ROL, AddressingMode::ACC, 1, 2),
            0x26 => (Mnemonic::ROL, AddressingMode::ACC, 2, 5),
            0x36 => (Mnemonic::ROL, AddressingMode::ACC, 2, 6),
            0x2e => (Mnemonic::ROL, AddressingMode::ACC, 3, 6),
            0x3e => (Mnemonic::ROL, AddressingMode::ACC, 3, 7),
            // ROR
            0x6a => (Mnemonic::ROR, AddressingMode::ACC, 1, 2),
            0x66 => (Mnemonic::ROR, AddressingMode::ACC, 2, 5),
            0x76 => (Mnemonic::ROR, AddressingMode::ACC, 2, 6),
            0x6e => (Mnemonic::ROR, AddressingMode::ACC, 3, 6),
            0x7e => (Mnemonic::ROR, AddressingMode::ACC, 3, 7),
            // BPL
            0x10 => (Mnemonic::BPL, AddressingMode::REL, 2, 2),
            // MBI
            0x30 => (Mnemonic::BMI, AddressingMode::REL, 2, 2),
            // BVC
            0x50 => (Mnemonic::BVC, AddressingMode::REL, 2, 2),
            // BVS
            0x70 => (Mnemonic::BVS, AddressingMode::REL, 2, 2),
            // BCC
            0x90 => (Mnemonic::BCC, AddressingMode::REL, 2, 2),
            // BCS
            0xB0 => (Mnemonic::BCS, AddressingMode::REL, 2, 2),
            // BNE
            0xD0 => (Mnemonic::BNE, AddressingMode::REL, 2, 2),
            // BEQ
            0xF0 => (Mnemonic::BEQ, AddressingMode::REL, 2, 2),
            // JMP
            0x4c => (Mnemonic::JMP, AddressingMode::ABS, 3, 3),
            0x6c => (Mnemonic::JMP, AddressingMode::IND, 3, 5),
            // JSR
            0x20 => (Mnemonic::JSR, AddressingMode::ABS, 3, 6),
            // RTI
            0x40 => (Mnemonic::RTI, AddressingMode::IMP, 1, 6),
            // RTS
            0x60 => (Mnemonic::RTS, AddressingMode::IMP, 1, 6),
            // CLC
            0x18 => (Mnemonic::CLC, AddressingMode::IMP, 1, 2),
            // SEC
            0x38 => (Mnemonic::SEC, AddressingMode::IMP, 1, 2),
            // CLI
            0x58 => (Mnemonic::CLI, AddressingMode::IMP, 1, 2),
            // SEI
            0x78 => (Mnemonic::SEI, AddressingMode::IMP, 1, 2),
            // CLV
            0xb8 => (Mnemonic::CLV, AddressingMode::IMP, 1, 2),
            // CLD
            0xd8 => (Mnemonic::CLD, AddressingMode::IMP, 1, 2),
            // SED
            0xf8 => (Mnemonic::SED, AddressingMode::IMP, 1, 2),
            // CMP
            0xc9 => (Mnemonic::CMP, AddressingMode::IMM, 2, 2),
            0xc5 => (Mnemonic::CMP, AddressingMode::ZPG, 2, 3),
            0xd5 => (Mnemonic::CMP, AddressingMode::ZPX, 2, 4),
            0xcd => (Mnemonic::CMP, AddressingMode::ABS, 3, 4),
            0xdd => (Mnemonic::CMP, AddressingMode::ABX, 3, 4),
            0xd9 => (Mnemonic::CMP, AddressingMode::ABY, 3, 4),
            0xc1 => (Mnemonic::CMP, AddressingMode::IDX, 2, 6),
            0xd1 => (Mnemonic::CMP, AddressingMode::IDY, 2, 5),
            // CPX
            0xe0 => (Mnemonic::CPX, AddressingMode::IMM, 2, 2),
            0xe4 => (Mnemonic::CPX, AddressingMode::ZPG, 2, 3),
            0xec => (Mnemonic::CPX, AddressingMode::ABS, 3, 4),
            // CPX
            0xc0 => (Mnemonic::CPY, AddressingMode::IMM, 2, 2),
            0xc4 => (Mnemonic::CPY, AddressingMode::ZPG, 2, 3),
            0xcc => (Mnemonic::CPY, AddressingMode::ABS, 3, 4),
            // PHA
            0x48 => (Mnemonic::PHA, AddressingMode::IMP, 1, 3),
            // PHP
            0x08 => (Mnemonic::PHP, AddressingMode::IMP, 1, 3),
            // PLA
            0x68 => (Mnemonic::PLA, AddressingMode::IMP, 1, 4),
            // PLP
            0x28 => (Mnemonic::PLP, AddressingMode::IMP, 1, 4),
            // BRK
            0x00 => (Mnemonic::BRK, AddressingMode::IMP, 1, 7),
            // NOP
            0xea => (Mnemonic::NOP, AddressingMode::IMP, 1, 2),

            _    => (Mnemonic::UNKNOWN, AddressingMode::UNKNOWN, 1, 1)
        };

    Instruction::new(opcode, mnemonic, mode, length, cycles)
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub mnemonic: Mnemonic,
    pub mode: AddressingMode,
    pub length: u8,
    pub cycles: u8
}

impl Instruction {
    fn new(opcode: OpCode, mnemonic: Mnemonic, mode: AddressingMode, length: u8, cycles: u8) -> Instruction {
        Instruction { opcode: opcode, mnemonic: mnemonic, mode: mode, length: length, cycles: cycles }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(0x{:02x}\t{:?}\t{:?}\tLength: {}\tCycles: {})", self.opcode, self.mnemonic, self.mode, self.length, self.cycles)
    }
}
