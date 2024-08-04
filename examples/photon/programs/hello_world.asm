        .include "ports.asm"

; Prints a null-terminated string
; arg a: pointer to string
; clobbers b
print_str:
        jmp .start
.loop:  pst b, char     ; print char
.start: pop b, a        ; load char
        bt b, .loop     ; loop while char != 0
        ret

hello:  .strz "Hello, World!\n"

main:   mov a, hello
        jsr print_str
