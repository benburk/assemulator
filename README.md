# Assemulator

Assemulator gives you an assembler and emulator for custom CPUs by implementing some simple functions.

I have designed a few toy CPUs, and it is tedious to write an assembler and emulator each time.

## Assembler features

- Global and local labels
- Macros
- Expression evaluation: `(1 + label) * 4`, `symbol >> 0xff`, etc.
- Namespaced file inclusion e.g. `string: .include "string.asm"`
- Assembler directives
- Conditional assembly
- Helpful error messages

## Emulator features

- Real-time user input for games via 6 buttons (arrow keys and ZX)
- Prompt user input from stdin
- Print characters and integers to stdout
- Generate random numbers
- Bitmap screen

## Example

[examples/risc16/main.rs](examples/risc16/main.rs) shows an example implementation for a simple 8 instruction, 16-bit, load-store CPU. It implements the following `Processor` trait:

```rust
fn new(program_counter: u64, program: Vec<u8>, data: Vec<u8>) -> Self;
fn parse(address: u64, opcode: Opcode, arguments: &[Argument]) -> Result<Vec<u8>, String>;
fn step(&mut self) -> usize;
```

...allowing us to write programs such as:

```asm
; 8x8=8 bit shifting multiplication
; r1: product
; r2: multiplicand
; r3: multiplier
; r4: mask
; r5: mask & multiplier

.doadd: add r1, r2              ; product += multiplicand
.loop:  add r2, r2              ; multiplicand <<= 1
        add r4, r4              ; mask <<= 1
        beq r4, r0, .end        ; if mask == 0, end
multiply:
        and r5, r3, r4          ; r5 = mask & multiplier
        beq r5, r4, .doadd
        beq r0, r0, .loop       ; .loop is a local label
        
.end:   st r1, r0, ticker       ; print the value
```

## Features planned / in progress

- Fixed-point constants (i0.8, i5.3, i8.16, i32.32, etc.)
- Step debugging
- Directives
  - org: start code at specific address
  - align: round symbol address up to multiple of align
  - zero: fill with n bytes of zeros
- Prettier emulator interface
- Graphics for the emulator
