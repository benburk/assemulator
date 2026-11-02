        .include "ports.asm"

msg:    .strz "Hello, World!\n"

; print a null-terminated string
; leaves its length in X
main:   mov x, 0
.loop:  mov a, [msg+x]      ; load the character
        cmp a, 0
        beq .end            ; stop at the null terminator
        mov [char], a       ; print char
        inc x
        jmp .loop
.end:   mov [ticker], x     ; strlen
