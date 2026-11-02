        .include "../ports.asm"

; table multiplication
; works if a+b <= 31
; TODO: need to take abs when subtracting
; arg a: multiplicand
; arg b: multiplier
; tmp c: temporary
; table of first 31 squares divided by 4
table:  .u8  0,  0,  1,  2,  4,  6,  9, 12 
        .u8 16, 20, 25, 30, 36, 42, 49, 56
        .u8 64, 72, 81, 90, 100, 110, 121, 132
        .u8 144, 156, 169, 182, 196, 210, 225, 240

table_multiply:
        mov c, a
        add a, b
        sub b, c
        ld a, [table+a]   ; a = (a+b)^2/4
        ld b, [table+b]   ; b = (a-b)^2/4
        sub a, b
        ret


main:   mov a, 5
        mov b, 7
        jsr table_multiply
        pst a, ticker
