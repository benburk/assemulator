        .include "ports.asm"

text:   .i16 'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n', '\0'

main:   ld r1, r2, text
        beq r1, r0, end
        add r2, 1
        st r1, r0, char
        beq r0, r0, main
end:
