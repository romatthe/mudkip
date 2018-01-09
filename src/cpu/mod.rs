pub mod instructions;
pub mod memory;

// The NES CPU had access to 2Kb (or 8192 bytes of RAM)
// Ref: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System#Technical_specifications
type WorkingMemory = [u8; 2048];

// Type aliases for the individual registers of the CPU
// Ref: https://wiki.nesdev.com/w/index.php/CPU_registers
type RegisterA = u8;        // Accumulator
type RegisterX = u8;        // Index X register
type RegisterY = u8;        // Index Y register
type RegisterPC = Address;      // Program Counter register
type RegisterS = u8;        // Stack pointer
type RegisterP = u8;        // Status register. Actually only has 6-bits that are useful. See below

// Structure representing the Status Register
// Some notes on the status register
// Ref: https://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
// 7  bit  0
// ---- ----
// NVss DIZC
// |||| ||||
// |||| |||+- Carry: 1 if last addition or shift resulted in a carry, or if last subtraction resulted in no borrow
// |||| ||+-- Zero: 1 if last operation resulted in a 0 value
// |||| |+--- Interrupt: Interrupt inhibit (0: /IRQ and /NMI get through; 1: only /NMI gets through)
// |||| +---- Decimal: 1 to make ADC and SBC use binary-coded decimal arithmetic (ignored on second-source 6502 like that in the NES)
// ||++------ s: No effect, used by the stack copy
// |+-------- Overflow: 1 if last ADC or SBC resulted in signed overflow, or D6 from last BIT
// +--------- Negative: Set to bit 7 of the last operation
bitflags! {
    struct StatusRegister: u8 {
        const C = 0b0000_0001;  // Carry
        const Z = 0b0000_0010;  // Zero
        const I = 0b0000_0100;  // Interrupt
        const D = 0b0000_1000;  // Decimal
        const V = 0b0100_0000;  // Overflow
        const N = 0b1000_0000;  // Negative
    }
}

type Address = u16;

// All possible 6502 addressing modes
// Addressing modes define how the CPU fetched the required operands for an instructions
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php?title=Addressing_Modes
#[derive(PartialEq, Debug)]
pub enum AddressingMode {
    ZPG,        // ZeroPage             Operand is an address and only the low byte is used,         ex: LDA $EE
    ZPX,        // Indexed ZeroPage X   Operand is 1-byte address, X register is added to it         eg: STA $00,X
    ZPY,        // Indexed Zeropage Y   Operand is 1-byte address, Y register is added to it         eg: STA $00,Y
    ABS,        // Absolute             Operand is an address and and both bytes are used,           ex: LDA $16A0
    ABX,        // Indexed Absolute X   Operand is 2-byte address, X register is added to it         eg: STA $1000,X
    ABY,        // Indexed Absolute Y   Operand is 2-byte address, Y register is added to it         eg: STA $1000,Y
    IND,        // Indirect             Memory location is 2-byte pointer at adjacent locations      eg: JMP ($0020)
    IMP,        // Implied              No operands, addressing is implied by the instruction,       eg: TAX
    ACC,        // Accumulator          No operands, accumulator is implied,                         eg: ASL
    IMM,        // Immediate            Operand value is contained in instruction itself,            ex: LDA #$07
    REL,        // Relative             1-byte signed operand is added to the program counter        eg: BEQ $04
    IDX,        // Indexed Indirect     2-byte pointer from 1-byte address and adding X register     eg: LDA ($40, X)
    IDY,        // Indirect Indexed     2-byte pointer from 1-byte address and adding Y after read   eg: LDA ($46), Y
    UNKNOWN
}

// Structs for each addressing mode that implement the Addressing trait
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php?title=Addressing_Modes
#[derive(Debug, Copy, Clone)]
struct ImmediateAddressing;
#[derive(Debug, Copy, Clone)]
struct ZeroPageAddressing;
#[derive(Debug, Copy, Clone)]
struct AbsoluteAddressing;
#[derive(Debug, Copy, Clone)]
struct ImpliedAddressing;
#[derive(Debug, Copy, Clone)]
struct AccumulatorAddressing;
#[derive(Debug, Copy, Clone)]
struct IndexedXAddressing;
#[derive(Debug, Copy, Clone)]
struct IndexedYAddressing;
#[derive(Debug, Copy, Clone)]
struct ZeroPageIndexedXAddressing;
#[derive(Debug, Copy, Clone)]
struct ZeroPageIndexedYAddressing;
#[derive(Debug, Copy, Clone)]
struct IndirectAddressing;
#[derive(Debug, Copy, Clone)]
struct PreIndexedIndirect;
#[derive(Debug, Copy, Clone)]
struct PostIndexedIndirectAddressing;
#[derive(Debug, Copy, Clone)]
struct RelativeAddressing;

pub struct Cpu {
    memory: WorkingMemory,
    registers: CpuRegisters,
    status: StatusRegister,
    pub program: Vec<u8>
}

struct CpuRegisters {
    a: RegisterA,
    x: RegisterX,
    y: RegisterY,
    pc: RegisterPC,
    s: RegisterS,
    p: RegisterP
}

impl CpuRegisters {
    fn new() -> CpuRegisters {
        CpuRegisters { a: 0x00, x: 0x00, y: 0x00, pc: 0x00, s: 0x00, p: 0x00 }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        // TODO: Figure out initial state of the Status Register
        Cpu { memory: [0; 2048], registers: CpuRegisters::new(), status: StatusRegister{ bits: 0 }, program: vec![] }
    }

    // Powers on the machine and sets the initial state
    // Ref: https://wiki.nesdev.com/w/index.php/CPU_power_up_state
    pub fn power_on (&mut self) {
        // TODO need clearer info on what this does precisely
        self.status.bits = 0xfd; // This is 0b1111 1101
    }

    // Resets the machine and sets the initial state
    // Ref: https://wiki.nesdev.com/w/index.php/CPU_power_up_state
    pub fn reset(&mut self) {
        // TODO need clearer info on what this does precisely
    }

    // Takes a single-step through the execution process, reading the first instruction at the Program Counter and executing it
    pub fn step(&mut self) {
        // Fetch the instruction currently at the Program Counter
        //opcode := OpCode(cpu.memory.fetch(cpu.registers.PC))
        //inst, ok := cpu.instructions[opcode]

        // Raise the Program Counter
        // Execute the current instruction, calling .exec() returns the amount of Cycles to consume
        //cycles := inst.exec(cpu)

        // Count cycles
        //for _ = range cpu.clock.ticker.C {
        //  cycles--

        //  if cycles == 0 {
        //    break
        //  }
        //}

        let pc = self.registers.pc;
        let instruction = instructions::decode(self.program[pc as usize]);

        let operands: Vec<_> = self.program.iter().skip(pc as usize).take(instruction.length as usize).collect();

        println!("{} - Operands: {:?}", instruction, operands);

        self.registers.pc = pc.wrapping_add(instruction.length as u16);
    }
}