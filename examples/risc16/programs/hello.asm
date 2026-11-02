        .include "ports.asm"

text:   .u16 "Hello, World!\n", 0

main:   lw r1, [text+r2]
        beq r1, r0, end
        add r2, 1
        sw r1, [char]
        beq r0, r0, main
end:
