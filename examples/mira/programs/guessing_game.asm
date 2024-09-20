; number guessing game
;       b: upper
;       c: lower
;       x: guess
;       y: temp

        ; set initial guess boundaries
        mov b, 10

loop:
        ; get guess
        mov x, $0[y]

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
        ; calculate upper - lower
        mov a, b
        sub c
        dec a           ; a = upper - lower - 1
        bpl loop        ; loop while upper - lower >= 0

        ; GAME OVER
        mov x, b
        mov a, $0[x]    ; you lose, i was thinking of x

