        .include "ports.asm"

; Fibonacci
; prints fib numbers up to 233
; 0,1,1,2,3,5,...
; tmp a: num1
; tmp b: num2
fib:    mov a, 1
        mov b, 0
        mov c, 6
.loop:  add a, b
        pst b, ticker
        add b, a
        pst a, ticker
        btd c, .loop
        ret


main:   jsr fib