.include "ports.asm"

        mov b, 0b110111

; Count set bits (popcnt)
; input in b, result in a, c=0
.loop:  adc c
        shr b
        bne .loop
        adc c       ; dont forget final bit

        mov ticker, a

; 89 -> 10 001 001