; computes the length of a null-terminated string
; out a: length of string
; arg a: pointer to first char of string
; clobbers b, c
str.strlen:
        add c, a, 1     ; store start to compute length
@:      pop b, a
        bt b, -         ; loop while char != null
        sub a, c        ; length=end-start+1
        ret


; strcmp - compares two strings
; out a: result of comparison
; arg b: ptr1 
; arg c: ptr2 
; clobbers b, c, d
str.strcmp:
@:      pop a, b        ; a = ram[b++]
        bf a, +         ; break if a is zero
        pop d, c        ; d = ram[c++]
        eq a, d
        bt -            ; loop while chars are equal
@:      sub a, d        ; compute char difference
        ret



; Prints a null-terminated string
; arg a: pointer to string
; clobbers b
@:      pst b, char     ; print char
str.print:
        pop b, a        ; load char
        bt b, -         ; loop while char != 0
        ret



; #################
; ## ARRAY LOGIC ##
; #################


; memcpy - copies n bytes from source to destination
; arg a: src pointer
; arg b: dst pointer
; arg c: number of bytes to copy
; clobbers: a, b, c, d
@:      pop d, a        ; read from a
        st d, b         ; write to b
        add b, 1
memcpy: btd c, -
        ret


; print array
; arg a: pointer to array
; arg b: number of elements
; clobbers: a, b, c
@:      pop c, a
        pst c, ticker
arr.print:
        btd b, -
