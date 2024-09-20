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

outer:  mov b,  ramX,arr+1
        mov y, x

inner:  mov a,  ramY,arr
        cmp b
        bcc next
        mov ramY,arr+1,  a
        dec y
        bpl inner

next:   mov ramY,arr+1,  b
        inc x
        dec c
        bne outer

