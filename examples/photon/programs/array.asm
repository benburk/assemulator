; ; Array max (signed)
; ; arg b: pointer to array
; ; arg c: length of array
; ; out a: max of array
; ; tmp d
; array_max:
;         add c, -1
;         ld a, b         ; initialize max
; loop:   pop d, b        ; d = [b++]
;         ges d, a
;         mvt a, d        ; a = max(a, d)
;         btd c, loop
;         ret


; ; 16 bit array add
; ; a/b: high/low
; ; c: pointer
; ; d: count
; ; e: clobber
; loop:
;         pop f, c
;         addx b, e
;         pop e, c
;         addx a, e
; start:  btd d, loop
; 
; arr:    .i8 -3, 3, -4, -2
; main:   mov b, arr
;         mov c, 4
;         jsr array_max
;         pst a, @ticker


; ; Array sum
; ; arg b: pointer to array
; ; arg c: length
; ; out a: total sum
; ; tmp d:
; loop:   pop d, b
        ; add a, d
; arrsum: btd c, loop
        ; ret
; 
; 
; arr:    .i8 1 2 3 4 5 6 7 8 9 10        ; sum=55
; 
; main:   mov b, arr
        ; mov c, 10
        ; jsr h, arrsum
        ; pst a, ticker
