; A8:8*B8:8 = A8:0*B8:0 + 2*A8:0*B0:8 + 2*A0:8*B8:0 + A0:8*B:08


; 8x8=8 Signed multiplication
; runtime proportional to length of multiplier
; computes a += b * c
; out a: product
; arg a: value to add to product
; arg b: multiplicand 
; arg c: multiplier
; clobbers b, c
mult:   shrx c          ; c/=2, x=lsb
        cad a, b        ; add if odd
        shl b           ; b *= 2
        bt c, mult      ; loop while c!=0
        ret


; 8x8=8 multiplication by repeated addition
; smallest possible program?
; arg a: initial product
; arg b: multiplicand
; arg c: multiplier
; clobbers b, c
loop:   add a, b
mult2:  btd c, loop
        ret


main:   mov b, 3
        mov c, 15
        jsr mult
        pst a, ticker



