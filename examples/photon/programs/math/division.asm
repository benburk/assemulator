; 8/8=Q8 R8 - Shifting division
; arg a: dividend
; arg b: divisor
; out a: remainder
; out c: quotient
l2:     shl b
        add d, 1
div2:   geq a, b
        bt l2

l1:     geq a, b
        csb a, b
        shlc c
        shr b
        btd d, l1
        ret


; repeated subtraction
loop:   csb a, b
        add c, 1
divide: geq a, b
        bt loop
        ret

main:   mov a, 27
        mov b, 0b111
        jsr div2
        pst c, @ticker
        pst a, @ticker
