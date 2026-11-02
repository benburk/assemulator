; number guessing game
; prints 2 for "guess higher" and 1 for "guess lower"
; then the number it was thinking of once it runs out of guesses
;       b: upper
;       c: lower
;       x: guess
;       y: hint
        .include "ports.asm"

        ; set initial guess boundaries
main:   mov b, 10

loop:
        ; get guess
        mov x, [ticker]

        ; A = 2 * guess - upper - lower
        mov a, x
        add x
        sub b
        sub c
        bpl guess_lower

; GUESS HIGHER
        ; lower = guess + 1
        mov c, x
        inc c
        mov y, 2        ; 2 => guess higher
        jmp skip

; GUESS LOWER
guess_lower:
        ; upper = guess - 1
        mov b, x
        dec b
        mov y, 1        ; 1 => guess lower

skip:
        mov [ticker], y

        ; calculate upper - lower
        mov a, b
        sub c
        dec a           ; a = upper - lower - 1
        bpl loop        ; loop while upper - lower >= 0

        ; GAME OVER
        mov [ticker], b ; you lose, i was thinking of b
