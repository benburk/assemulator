        .include "ports.asm"

JMP:    .macro dest
        beq r0, r0, $dest
        .endm

PRINT:  .macro src
        sw $src, r0, ticker
        .endm

MOVI:   .macro dest, argument
        lui $dest, ($argument >> 6) & 0x3ff
        add $dest, $dest, $argument & 0x3f
        .endm

AND:    .macro dest, src1, src2
        nand $dest, $src1, $src2
        NOT $dest, $dest
        .endm

NOT:    .macro dest, src
        nand $dest, $src, $src
        .endm

SUB:    .macro dest, src1, src2
        NOT $dest, $src2         ; invert src2
        add $dest, 1             ; add 1 to src2
        add $dest, $src1         ; add src1 to src2
        .endm



movi:   .macro dest, x

        .if $x >> 6 != 0 && ($x >> 6) & 1023 != 1023
        lui $dest, ($x >> 6) & 1023
        add $dest, $dest, $x & 63
        .endif

        .if $x >> 6 == 0 || ($x >> 6) & 1023 == 1023
        add $dest, r1, $x & 127
        .endif
        .endm
