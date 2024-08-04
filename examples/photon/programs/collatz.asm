        .include "ports.asm"

; Collatz conjecture
; a: start value a >= 1
; b,c: scratch
collatz:
        shl c, a        ; c = a << 1
        shrx b, a       ; b = a >> 1
        pst a, ticker   ; print value
        addc a, c       ; a = 3a + 1 (c will be set to 1 if value is used)
        mvf a, b        ; a%2? 3a+1 : a/2
        bt b, collatz
        ret

main:   mov a, 19
        jsr collatz
