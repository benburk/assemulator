.include "ports.asm"

arr:      .i8 8, 6, 4, 10, 36, 23, 48, 12, 42, 5


; 
;         mov c, 9
; 
; outer:  mov b, arr[x+1]
;         mov y, x
; 
; inner:  mov a, arr[y]
;         cmp b
;         bcc next
;         mov arr[y+1], a
;         dec y
;         bpl inner
; 
; next:   mov arr[y+1], b
;         inc x
;         dec c
;         bne outer




        mov c, 9

outer:  mov b, [arr+1+x]
        mov y, x

inner:  mov a, [arr+y]
        cmp b
        bcc next
        mov [arr+1+y], a
        dec y
        bpl inner

next:   mov [arr+1+y], b
        inc x
        dec c
        bne outer

        ; print the sorted array
        mov x, 0
print:  mov a, [arr+x]
        mov [ticker], a
        inc x
        mov a, x
        cmp a, 10
        bne print
