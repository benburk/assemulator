    .include "ports.asm"

; B = dividend
; C = divisor
; out B: quotient
; out A: remainder
main:
    mov b, 27
    mov c, 7
    mov a, 0
    mov x, 8
    
    shl b
L1: rol a
    cmp c
    bcc L2
    sub c
L2: rol b
    dec x
    bne L1

    mov [ticker], b ; 3
    mov [ticker], a ; 6
