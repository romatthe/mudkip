use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

type OpCode = u8;

// Mnemonics for all 6502 CPU instructions
// Original source: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php/6502_Opcodes
#[derive(PartialEq, Debug)]
pub enum Mnemonic {
    LDA, LDX, LDY, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,     // Storage
    ADC, DEC, DEX, DEY, INC, INX, INY, SBC,                         // Math
    AND, ASL, BIT, EOR, LSR, ORA, ROL, ROR,                         // Bitwise
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,                         // Branch
    JMP, JSR, RTI, RTS,                                             // Jump
    CLC, CLD, CLI, CLV, CMP, CPX, CPY, SEC, SED, SEI,               // Registers
    PHA, PHP, PLA, PLP,                                             // Stack
    BRK, NOP                                                        // System
}

// All possible 6502 addressing modes
// Addressing modes define how the CPU fetched the required operands for an instructions
// Original source: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php?title=Addressing_Modes
#[derive(PartialEq, Debug)]
pub enum AddressingMode {
    Immediate,              // Operand value is contained in instruction itself,            ex: LDA #$07
    ZeroPage,               // Operand is an address and only the low byte is used,         ex: LDA $EE
    Absolute,               // Operand is an address and and both bytes are used,           ex: LDA $16A0
    Implied,                // No operands, addressing is implied by the instruction,       eg: TAX
    Accumulator,            // No operands, accumulator is implied,                         eg: ASL
    IndexedX,               // Operand is 2-byte address, X register is added to it         eg: STA $1000,X
    IndexedY,               // Operand is 2-byte address, Y register is added to it         eg: STA $1000,Y
    ZeroPageIndexedX,       // Operand is 1-byte address, X register is added to it         eg: STA $00,X
    ZeroPageIndexedY,       // Operand is 1-byte address, Y register is added to it         eg: STA $00,Y
    Indirect,               // Memory location is 2-byte pointer at adjacent locations      eg: JMP ($0020)
    PreIndexedIndirect,     // 2-bit pointer from 1-byte address and adding X register      eg: LDA ($40, X)
    PostIndexedIndirect,    // 2-bit pointer from 1-byte address and adding Y after read    eg: LDA ($46), Y
    Relative,               // 1-byte signed operand is added to the program counter        eg: BEQ $04
}

// Decodes a single-byte opcode into a richer Instruction data structure
pub fn decode(opcode: OpCode) -> Instruction {
    Instruction::new(Mnemonic::BRK, AddressingMode::Immediate, 0x00)
}

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub mode: AddressingMode,
    pub opcode: OpCode
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, mode: AddressingMode, opcode: OpCode) -> Instruction {
        Instruction { mnemonic: mnemonic, mode: mode, opcode: opcode }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(0x{:02x}\t{:?})", self.opcode, self.mnemonic)
    }
}
