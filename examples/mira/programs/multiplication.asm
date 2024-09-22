.include "ports.asm"



;         
; ; 8x8=16 multiply
; do_add: add b
;         mov x, a  ; store low byte
;         mov a, y  ; load high byte
;         adc num1Hi
;         mov y, a  ; store high byte
;         mov a, x  ; load low byte
; 
; 
; loop:   shl b
;         rol num1Hi   ; x used as high byte of B
; start:  shr c
;         bcs do_add
;         bne loop


main:
        mov b, 19
        mov c, 13


; 8x8=8 multiply
; multiplicant in b
; multiplier in c
; output in a

        jmp .start
.do_add:
        add b
.loop:  shl b
.start: shr c
        bcs .do_add
        bne .loop

        mov ticker, a
