# Photon

- 16-bit fixed-length, two-operand, load-store, RISC instruction set
- 8-bit word size
- 8 general purpose registers
- Separate 8-bit address space for programs and data
- Single flag "x", can be set by math, shift, and compare ops

## Implementation

- Feasible to implement in Minecraft
- 3 ticks per cycle, 1 cycle data forwarding hazard
- stall on memory read and (some) port loads
- easy-to-decode instruction formats

## Instruction set

```
Bit pattern             Opcode  Description                 Logic
----------------------  ------  --------------------------  ----------------------
aaa  0i000  bbb  00000  and     bitwise and                 a = a & b
aaa  0i000  bbb  00001  or      bitwise inclusive or        a = a | b
aaa  0i000  bbb  00010  xor     bitwise exclusive or        a = a ^ b
aaa  0i000  bbb  00011  mov     move                        a = b
aaa  0i000  bbb  00100  tst     test                        x = a & b != 0
aaa  0i000  bbb  00101  eq      equal                       x = a == b
aaa  0i000  bbb  00110  geq     greater or equal unsigned   x = a >= b (unsigned)
aaa  0i000  bbb  00111  ges     greater or equal signed     x = a >= b (signed)
aaa  00000  bbb  010mm  add     add                         a = a + b
aaa  00000  bbb  011mm  sub     subtract                    a = a - b
aaa  00000  bbb  100mm  shl     shift left                  a = b << 1
aaa  00000  bbb  101mm  shr     shift right                 a = b >> 1
aaa  00000  bbb  11000  mvt     move if true                a = b if x
aaa  00000  bbb  11001  mvf     move if false               a = b if !x
aaa  00000  bbb  11010  cad     conditional add             a += b if x
aaa  00000  bbb  11011  csb     conditional subtract        a -= b if x
aaa  00000  bbb  11100  neg     negate                      a = -b
aaa  00000  bbb  11101  swap    swap nibbles                a = b << 4 | b >> 4
aaa  00000  bbb  11110  psh     push memory                 [--b] = a
aaa  00000  bbb  11111  pop     pop memory                  a = [b++]
aaa  10m00    kkkkkkkk  bt      branch if true              pc = k if x
aaa  10m01    kkkkkkkk  bf      branch if false             pc = k if !x
aaa  10m10    kkkkkkkk  jmp     jump                        pc = a + k
aaa  10011    kkkkkkkk  jsr     jump subroutine             a = pc; pc = k
aaa  10111    kkkkkkkk  btd     branch if true; decrement   a? a--; pc = k
aaa  11000    kkkkkkkk  ld      memory load                 a = [b+k]
aaa  11001    kkkkkkkk  st      memory store                [b+k] = a
aaa  11010    kkkkkkkk  pld     port load                   a = port[k]
aaa  11011    kkkkkkkk  pst     port store                  port[k] = a
aaa  11100  bbb  kkkkk  ld      memory load                 a = [b+k]
aaa  11101  bbb  kkkkk  st      memory store                [b+k] = a
aaa  11110  bbb  kkkkk  adi     add immediate               a = b + k
```

Notes:
- `i` bit denotes whether to use immediate. Uses the 3 proceeding bits as instruction selector
- `m` bits denote modifiers for different instruction types (for arith and shifting: include x / write out x), (for branching: whether to use register)

## Example

```
; 8x8=8 Signed multiply-accumulate (a += b * c)
; runtime proportional to length of multiplier
; out a: product
; arg a: value to add to product
; arg b: multiplicand 
; arg c: multiplier
; clobbers b, c
mult:   shrx c          ; c/=2, x=lsb
        cad a, b        ; add if odd
        shl b           ; b *= 2
        bt c, mult      ; loop while c!=0
        ret

main:   mov b, 3
        mov c, 15
        jsr mult
        pst a, @ticker
```

Run the program with `cargo run --release -- ./programs/math/multiplication.asm run` and `45` should be printed to stdout. Check out the programs folder for more examples.




## Rationale

Why one flag instead of multple?
- Fewer branch instructions
- More orthogonal
- More-straight forward conditional instructions

Why btd?
- A few architectures had djnz (8051, z80). I instead test if the value is zero first.
- check condition at the end


Instructions that could be nice to add
- conditional negate
- 3 operand add, sub, logic ops to save some moves
        - this would complicate decoding a decent amount, probably not worth



## Conditional instructions

- Conditional move and arithmetic to remove unpredictable branches
- taken branches usually take an extra cycle anyways, so this is a faster way to "skip over" code

