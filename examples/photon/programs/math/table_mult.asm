; table multiplication
; works if a+b <= 31
; TODO: need to take abs when subtracting
; arg a: multiplicand
; arg b: multiplier
; tmp c: temporary
; table of first 31 squares divided by 4
table:  .i8  0,  0,  1,  2,  4,  6,  9, 12 
        .i8 16, 20, 25, 30, 36, 42, 49, 56
        .i8 64, 72, 81, 90, 100, 110, 121, 132
        .i8 144, 156, 169, 182, 196, 210, 225, 240

table_multiply:
        mov c, a
        add a, b
        sub b, c
        ld a, a, table  ; a = (a+b)^2/4
        ld b, b, table  ; b = (a-b)^2/4
        sub a, b
        ret


main:   mov a, 5
        mov b, 7
        jsr table_multiply
        pst a, @ticker
