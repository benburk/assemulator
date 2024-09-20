

WORD:   .string "hello"
LEN: 5
TABLE:  0 * 26
lives: 7


initialize_word:
        ; read the secret word and initialize the
        ; counter hashmap
        mov x, 0
loop:   mov y, WORD[x]
        beq end         ; reached end of string
        inc TABLE[y]    ; increment the count for that letter
        inc x
        jmp loop
end:



check_guess:
    mov y, KEYBD        ; get user input
    mov a, TABLE[y]     ; check if the letter is set
    beq lose_life       ; if zero, lose a life




lose_life:
    dec lives





check_guess:
        mov a, KYBD         ; get users guess
        mov x, LEN          ; length of secret word
loop:   dec x
        bmi end
        cmp WORD[x]
        bne loop
        mov SCREEN[x], a    ; write the guess to the screen
        jmp loop
end:

        hey


KYBD:   I/O keyboard
TABLE:  boolean array
LIVES:  lives remaining



check_guess:
        mov x, KYBD             ; get guess from player
        mov a, TABLE[x-12]      ; is guess in secret word?
        beq lose_life           ; letter is not in secret word, lose a life


        ; reveal correct letter in secret word
        mov x, 0                ; first char of secret word
        mov b, KYBD             ; get player's guess
next_char:
        mov a, WORD[x]          ; load x'th char of secret word
        beq end                 ; done if null char
        cmp b                   ; compare with guess
        bne next_char           ; not equal? then next char
        mov SCREEN[x], b        ; write the letter to the screen
        inc x
        jmp next_char

end:    jmp 




lose_life:
        dec lives

