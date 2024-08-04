.include "ports.asm"

; print array
; arg a: pointer to array
; arg b: number of elements
; clobbers: a, b, c
arr_print:
        jmp .start
.loop:  pop c, a
        pst c, ticker
.start: btd b, .loop
        ret


; Insertion sort
; Loop directions are reversed compared to canonical insertion sort
; allowing use of the pop instruction
; 
; arg a: pointer
; a: outer loop pointer
; j: inner loop pointer
; tmp a: unused
; arg b: array size
; tmp c: current outer item
; tmp d: current inner item
; tmp e: unused
; tmp f: inner loop counter
; tmp g: outer loop pointer/counter
; tmp h: inner loop pointer

arr:    .i8 4, 1, 3, 2, 5

main:   mov b, 4        ; length of array minus one
        add g, b, -1    ; g = b - 2

.outer: ld c, g         ; c = [g]
        mov f, b
        sub f, g
        add h, g, 1     ; h = g + 1

.inner: pop d, h        ; d = [h++]
        geq d, c        ; d >= c?
        bt .next        ; goto next
        st d, h, -2     ; [h+2] = d
        btd f, .inner

.next:  st c, h, -2
        btd g, .outer

        ; print array
        mov a, arr
        mov b, 5
        jsr arr_print
