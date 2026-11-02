        .include "../ports.asm"

; Fancy hash table idea
; 
; table[26] contains 0 if letter not present, or starting index
; 
; jump table {p: 0, e: 1, n: 2, l: 4, o: 5}}
; penelope -> [6, 3, 0, 7, 0, 0, 0, 0]


; Prints a null-terminated string
; arg a: pointer to string
; clobbers b
l1:      pst b, char   ; print char
print:
        pop b, a        ; load char
        bt b, l1        ; loop while char != 0
        ret


secret: .strz "penelope"
len:    .set 8
word2:  .strz "********"

main:  ; print word and ask for guess
        mov a, word2
        jsr print
        mov a, '\n'
        pst a, char
        pld d, char            ; get guess

        ; setup loop variables
        mov a, secret           ; load secret
        mov b, word2            ; load guesses
        mov c, len              ; length of secret
        jmp start

        ; substitute guess into word2
loop:   add b, 1
        pop e, a                ; load char
        eq d, e                 ; compare guess to char
        bf start
        st e, [b-1]             ; if match, store guess in word2
start:  btd c, loop

        jmp main
