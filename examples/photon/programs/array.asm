        .include "ports.asm"

; Array max (signed)
; arg b: pointer to array
; arg c: length of array
; out a: max of array
; tmp d
array_max:
        add c, -1
        ld a, [b]       ; initialize max
.loop:  pop d, b        ; d = [b++]
        ges d, a
        mvt a, d        ; a = max(a, d)
        btd c, .loop
        ret


; Array sum
; arg b: pointer to array
; arg c: length
; out a: total sum
; tmp d
array_sum:
        mov a, 0
        jmp .start
.loop:  pop d, b
        add a, d
.start: btd c, .loop
        ret


signed: .i8 -3, 3, -4, -2
numbers: .i8 1, 2, 3, 4, 5, 6, 7, 8, 9, 10

main:   mov b, signed
        mov c, 4
        jsr array_max
        pst a, ticker   ; 3

        mov b, numbers
        mov c, 10
        jsr array_sum
        pst a, ticker   ; 55
