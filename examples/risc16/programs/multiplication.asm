        .include "ports.asm"
        .include "macros.asm"
        MOVI r2, 12             ; multiplicand
        MOVI r3, 154            ; multiplier
        MOVI r4, 1              ; mask = 1
        JMP mult

; 8x8=8 bit shifting multiplication
; performs 16 iterations every time
; r1: product
; r2: multiplicand
; r3: multiplier
; r4: mask
; r5: mask & multiplier

doadd:  add r1, r2              ; product += multiplicand
loop:   add r2, r2              ; multiplicand <<= 1
        add r4, r4              ; mask <<= 1
        beq r4, r0, end         ; if mask == 0, end
mult:   AND r5, r3, r4          ; r5 = mask & multiplier
        beq r5, r4, doadd
        JMP loop
        
end:    PRINT r1
