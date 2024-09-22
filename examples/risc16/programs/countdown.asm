        .include "ports.asm"

; print the values from 5 to 0
        add r1, 5
loop:   sw r1, r0, ticker
        beq r1, r0, end
        add r1, -1
        beq r0, r0, loop
end:
