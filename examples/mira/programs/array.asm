; Array sum
; sum the values in an array
arr:    .i8 1,2,5,2

        mov x, 3    ; len of arr-1
loop:   add arr[x]
        dec x
        bpl loop


mov x, 3


; Max of unsigned array
; max value in A
arr:    .u8 3,7,2,4
        mov x, 3
new:	mov a, arr[x]
loop:	dec x
        bmi end
        cmp arr[x]
        bcs loop
        jmp new
end: