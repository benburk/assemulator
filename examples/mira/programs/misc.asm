; swap the nibbles of A
shl a
adc 0x80
rol a
shl a
adc 0x80
rol a


; Test if a is a power of 2
; Technically popcnt() == 1, so doesn't work for 0
; Z=0 => yes
mov b, a
dec a
and b


; Toggle carry flag while preserving A
rol A
xor 1
ror A


; increment lower 4 bits of A
mov b, a    ; save a in b to restore upper nibble later
inc a       ; increment lower 4 bits
and b, 0xf0 ; mask upper bits of b
and a, 0x0f ; mask lower bits of a
ior a, b



; Copy bit 7 of a value to all of A
; a=0 if positive, -1 if negative
mov a, 0x7F
cmp value  # Carry clear if >= $80
adc 0x80   # 0x00 if carry set; 0xff if carry clear



; Cookie-cutter
; write bits 3-5 of a to memory
xor dest
and 0x38; bits 3-5
xor dest
mov dest, a


; read bits 3-5 of memory into A
; A must be 0
eor src
and #$C7 ; not bits 3-5
eor src



; arithmetic shift right
        cmp 0x80
        ror
        

; arithmetic shift right 4 bits at once
        shr
        shr
        shr
        shr
        add 0xf8
        xor 0xf8


; Average of two numbers (2 bytes)
; inputs: A, B
; result: A
add b
ror


; Absolute value
; input: A, with N correctly set
; result: A
    bnc abs
    xor 0xff
    inc a
abs:


; Collatz conjecture (14 bytes)
; starting value in A
loop:   mov TICKER, a   ; print value
        shr a           ; divide by 2
        bzs end         ; if we hit 0 we're done
        bcc loop        ; loop while even
        rol a           ; restore true odd value
        mov b, a        ; compute a = 3a+1
        shl a
        add b
        inc a           ; a = 3a+1 at this point
        jmp loop
end: