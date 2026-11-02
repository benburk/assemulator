        .include "ports.asm"

arr:    .i8 3, 7, 2, 4
len:    .set 4

; Array sum
; sum the values in an array into A
main:   mov a, 0
        mov x, len-1
.sum:   add [arr+x]
        dec x
        bpl .sum
        mov [ticker], a

; Max of unsigned array
; max value in A
        mov x, len-1
.new:   mov a, [arr+x]
.loop:  dec x
        bmi .end
        cmp [arr+x]
        bcs .loop
        jmp .new
.end:   mov [ticker], a
