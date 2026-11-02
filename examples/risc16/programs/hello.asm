        .include "ports.asm"

text:   .u16 "Hello, World!\n", 0

main:   lw r1, r2, text
        beq r1, r0, end
        add r2, 1
        sw r1, r0, char
        beq r0, r0, main
end:
