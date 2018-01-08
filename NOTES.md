# Notes

## Emulator Structure

```c
struct nes {
 Memory cpuMemory;
 Memory ppuMemory;
 Memory objectAttributeMemory;
 
 CPU cpu;
 PPU ppu;
 APU apu;
 
 Cartridge cartridge;
 
 Interrupts interrupts;
 
 MMU mmu;
 
 Renderer renderer;
 
 Joypad joypads[NES_NUM_JOYPADS];
};
```

## CPU, PPU & APU

* CPU and PPU each have their own address space
* Separate area for sprite attributes in memory known as "object attribute memory". CPU writes to this, PPU reads from it.

The CPU is a modified version of the MOS 6502.

On a real-world NES, all three devices run completely in parralel. However, giving each device a seperate thread to run in could make synchronisation hard and might tank performance. For performance, a sequential approach is probably still the best way to go about things.

* Most common (ugly) solution: the CPU keeping an exact track of how many cycles the current operation had taken, and then passing that number to the audio and picture processing units to see whether, in that amount of time, they could have done something themselves. 

* Another suggestion I read was on using memory accesses to synchronise hardware in a transparent way without having to pass around a CPU cycle count variable to other devices. In my sequential program I implemented this by writing wrapper functions which guard around access to the CPU memory, and whenever the CPU wants to read or write to memory it has to go through the wrapper functions. These functions do indeed pass the byte to be written on to the real CPU memory (or return the byte to be read from the real memory), but they also allows the audio and picture processing units to advance by calling ppu_step and
apu_step.

### Zero page addressing mode

The CPU has a few addressing modes. One of them, `zero page`, takes a one-byte address instead of a two-byte address as an operand. This limits the addressing space to the first half (`$0000` to `$00FF`) of the accessible memory (hence, the "zero page"), and allows the instructions to be written without using the additional `$00`, and thus requires less CPU cycles to complete.

For example, these both accomplish the same thing (load value of memory location `$0000` into the accumulator):

```
LDA $00            ; zero page
LDA $0000          ; non-zero page
```

However, the first instruction is only two bytes long and requires three clock cycles to complete. The second instruction is three bytes in length and requires four clock cycles to execute. Obviously, the difference in execution time could significantly improve performance in repetitive code.


## Interrupts

Many device are free to generate their own interrupts, which are then handled by the
CPU (unless it is currently ignoring interrupts). 

There are three general types of interrupts:
* **NMI** - **N**on **M**askable **I**nterrupts, interrupts which the CPU cannot "mask away", it must handle them
* **IRQ** - **I**nterrupt **R**e**Q**uests, interrupts which the CPU is free to ignore
* **RESET** - The initial power on / reset interrupt generated when you hit the power switch

The CPU can also generate "fake" interrupts by a programmer giving it the BRK opcode, which are treated as IRQ interrupts and
trap to the IRQ handler.


## Cool Stuff

Cycle Accuracy in JSBeeb by Matt Godbolt: https://www.youtube.com/watch?v=7WuRq-Wmw5o&t=26%3A33s

Debugger in Godbolt's JSBeed: https://www.youtube.com/watch?v=7WuRq-Wmw5o&t=29m00s