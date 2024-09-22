        .include "ports.asm"

        mov a, 19

; Collatz conjecture (14 bytes)
; starting value in A
.loop:  mov ticker, a   ; print value
        shr a           ; divide by 2
        beq end         ; if we hit 0 we're done
        bcc .loop       ; loop while even
        rol a           ; restore true odd value
        mov b, a        ; compute a = 3a+1
        shl a
        add b
        inc a           ; a = 3a+1 at this point
        jmp .loop
end: