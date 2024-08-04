; Print null-terminated string
; arg a: pointer to string
; clobbers: b
loop:   pst b, @char    ; print char
print:  pop b, a        ; load char
        bt b, loop      ; loop while char != 0
        ret


; Number guessing game
; arg a: general arg-passing register
; arg b: guess from player
; arg c: upper bound
; arg d: lower bound

guess_str:      .strz "\nGuess a number in the range: "
lower_str:      .strz "Too high! Guess lower\n"
higher_str:     .strz "Too low! Guess higher\n"
game_over:      .strz "You lose! The number I was thinking of was: "

game:   mov a, guess_str
        jsr print
        pst d, @ticker
        pst c, @ticker
        pld b, @ticker  ; get guess from user

        ; check if 2 * guess - lower >= upper
        shl a, b
        sub a, d
        geq a, c
        bt guess_lower

        ; GUESS HIGHER
        add d, b, 1     ; lower = guess + 1
        mov a, higher_str
        jsr print
        jmp skip

        ; GUESS LOWER
guess_lower:
        add c, b, -1    ; upper = guess - 1
        mov a, lower_str
        jsr print

skip:   eq c, d         ; loop while upper != lower
        bf game

; GAME OVER
        mov a, game_over
        jsr print
        pst c, @ticker  ; you lose, i was thinking of x


; Run game with bounds [0, 10]
main:   mov c, 10       ; initialize upper bound
        mov d, 0        ; ... lower bound
        jmp game
