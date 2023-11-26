# MSP Emulator

This project is an emulator for a subset of the  MSP-430 instruction set.
I am no longer working on this emulator, but there are definitely still bugs.
Note that running the current `main.c` program, which is a connect 4 game, works with "--opt_level=off" but breaks with "--opt_level=2".

## Goals

The original goal of this repository was to implement the MSP-430 instruction set on the FPGA as a
final project for the CS M152A course. Since there exist C compilers for this instruction set,
this would have allowed us to compile C code to run on the FPGA. I found, however, that the compiler
tends to rely on the msp430 ABI for routines such as multiplication, remainders, floating point
operations, and even bit shifting. I did not have time, or motivation, to implement any routines in the ABI.

The MSP-430 [instruction set](https://en.wikipedia.org/wiki/TI_MSP430) was chosen for its simplicity.
It only contains 27 instructions in 3 "families". The different addressing modes makes the instruction
set extremely flexible, despite having few instructions. I have written the emulator in a style similar
to how I would have implemented the instruction set in Verilog (implemented in multiple stages,
while being mindful of which "registers" were modified on which stage). Even in Rust, it was difficult to
keep trace through and debug.

I have decided not to complete this project because of lack of time at the end of the quarter,
and instead I will be submitting an implementation of my own instruction set.
