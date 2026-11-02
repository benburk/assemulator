        .include "ports.asm"

msg:    .strz "Hello, World!\n"

; print a null-terminated string
; leaves its length in X
main:   mov x, 0
.loop:  mov a, ramX, msg    ; load the character
        cmp a, 0
        beq .end            ; stop at the null terminator
        mov ramK, char, a   ; print char
        inc x
        jmp .loop
.end:   mov ramK, ticker, x ; strlen
