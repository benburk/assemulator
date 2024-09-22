# Load-store

- 16-bit fixed-length, three-operand, load-store, RISC instruction set
- 16-bit word size
- 8 general purpose registers
- Separate 8-bit address space for programs and data
- No flags
- Easy to generate code for
- Easy to map on to other ISAs
- Easy to pipeline / out-of-order


## Instruction set

```
Bit pattern             Opcode  Description                 Logic
----------------------  ------  --------------------------  ----------------------
000 aaa bbb ccc 0000    and     bitwise and                 a = b & c
000 aaa bbb ccc 0001    ior     bitwise inclusive or        a = b | c
000 aaa bbb ccc 0010    xor     bitwise exclusive or        a = b ^ c
000 aaa bbb ccc 0011    mul     multiply
000 aaa bbb ccc 0100    div     divide
000 aaa bbb ccc 0101    mod     modulo
000 aaa bbb ccc 0110    add     add                         a = b + c
000 aaa bbb ccc 0111    sub     subtract                    a = b - c
000 aaa bbb ccc 1000    shl     shift left                  a = b << c
000 aaa bbb ccc 1001    shr     shift right                 a = b >> c
000 aaa bbb ccc 1010    geq     greater than or equal       a = b >= c
000 aaa bbb ccc 1011    ges     greater or equal signed     a = b >= c (signed)
000 aaa bbb ccc 1100    mvt     move if true                a = b if c
000 aaa bbb ccc 1101    mvf     move if false               a = b if !c
000 aaa bbb ccc 1110    
000 aaa bbb ccc 1111    
001 aaa bbb  kkkkkkk    ld      load from memory            a = ram[b + k]
010 aaa bbb  kkkkkkk    st      store to memory             ram[b+k] = a
011 aaa bbb  kkkkkkk    addi    add immediate               a = b + k
100 aaa bbb  kkkkkkk    jalr    jump and link register
101 aaa   kkkkkkkkkk    lui     load upper immediate        a = k << 6
110 aaa   kkkkkkkkkk    bt      move if false               if a:  pc += k
111 aaa   kkkkkkkkkk    bf      move if false               if !a: pc += k
```
