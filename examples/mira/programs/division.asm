; B = dividend
; C = divisor
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