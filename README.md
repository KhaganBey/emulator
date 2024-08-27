# GB Emulator

A (WIP) Gameboy emulator written in Rust.

**Status**: CPU only

The emulator can read game boy roms (.gb files) without MBC and execute opcodes, outputting writes to the serial port to the command terminal. It has three modes: boot, main and debug.

* Boot stops the emulator when the boot room finishes executing. 
* Debug logs the state of the emulator to a log file after every CPU instruction.
* Main is the default mode.

## CPU

The CPU is pretty much completed with both opcode tables fully implemented. It passes all CPU tests from blargg. The timer, however is still not fully accurate.

### Timer

The emulator's timer is built from a t-cycle perspective but only runs every M-cycle (i.e. every 4 t-cycles) due to the CPU being the only working part for now. [GBEDG's timer breakdown](https://hacktix.github.io/GBEDG/timers/) was used as a base. 

The entire timer will probably be overhauled sometime when I feel like it, though it will stay like this for now. 

## Other Parts

### PPU

The PPU (pixel processing unit) has tile logic implemented and pretty much nothing else. It is very much a work in progress.

### MBC

Memory bank controllers will be the first thing to be implemented after the PPU, so just a matter of time.

### APU

I am not a masochist. Therefore, audio is not even considered for the foreseeable future.

## Platform Layer

As a platform layer, [Rust-SDL2](https://docs.rs/sdl2/latest/sdl2/) is used, but it doesn't connect to the PPU or the joypad registers yet. Other than opening a window, everything about it is a work in progress.

I will work on this and the PPU simultaneously when I return to the project.