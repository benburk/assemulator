        .include "ports.asm"

; computes the length of a null-terminated string
; out a: length of string
; arg a: pointer to first char of string
; clobbers b, c
strlen: add c, a, 1     ; store start to compute length
.loop:  pop b, a
        bt b, .loop     ; loop while char != null
        sub a, c        ; length=end-start+1
        ret


; strcmp - compares two strings
; out a: difference of the first mismatched chars, 0 if equal
; arg b: ptr1
; arg c: ptr2
; clobbers b, c, d
strcmp:
.loop:  pop a, b        ; a = ram[b++]
        pop d, c        ; d = ram[c++]
        bf a, .end      ; break at end of string
        eq a, d
        bt .loop        ; loop while chars are equal
.end:   sub a, d        ; compute char difference
        ret


; Prints a null-terminated string
; arg a: pointer to string
; clobbers b
print:  jmp .start
.loop:  pst b, char     ; print char
.start: pop b, a        ; load char
        bt b, .loop     ; loop while char != 0
        ret


; #################
; ## ARRAY LOGIC ##
; #################


; memcpy - copies n bytes from source to destination
; arg a: src pointer
; arg b: dst pointer
; arg c: number of bytes to copy
; clobbers: a, b, c, d
memcpy: jmp .start
.loop:  pop d, a        ; read from a
        st d, [b]       ; write to b
        add b, 1
.start: btd c, .loop
        ret


; print array
; arg a: pointer to array
; arg b: number of elements
; clobbers: a, b, c
print_array:
        jmp .start
.loop:  pop c, a
        pst c, ticker
.start: btd b, .loop
        ret


hello:  .strz "hello"
world:  .strz "world"
copy:   .zero 6
arr:    .i8 3, 1, 4

main:   mov a, hello    ; copy "hello" and print the copy
        mov b, copy
        mov c, 6
        jsr memcpy
        mov a, copy
        jsr print
        mov a, '\n'
        pst a, char

        mov a, hello
        jsr strlen
        pst a, ticker   ; 5

        mov b, hello
        mov c, world
        jsr strcmp
        pst a, ticker   ; 'h' - 'w' = 241

        mov b, hello
        mov c, copy
        jsr strcmp
        pst a, ticker   ; 0

        mov a, arr
        mov b, 3
        jsr print_array ; 3 1 4
