; swap the nibbles of A
shl a
adc a, 0x80
rol a
shl a
adc a, 0x80
rol a


; Test if a is a power of 2
; Technically popcnt() == 1, so doesn't work for 0
; Z=0 => yes
mov b, a
dec a
and b


; Toggle carry flag while preserving A
rol a
xor a, 1
ror a


; increment lower 4 bits of A
mov b, a    ; save a in b to restore upper nibble later
inc a       ; increment lower 4 bits
and b, 0xf0 ; mask upper bits of b
and a, 0x0f ; mask lower bits of a
ior b



; Copy bit 7 of a value to all of A
; a=0 if positive, -1 if negative
mov a, 0x7F
cmp [value]    ; carry clear if >= 0x80
adc a, 0x80    ; 0x00 if carry set; 0xff if carry clear



; Cookie-cutter
; write bits 3-5 of a to memory
xor [dest]
and a, 0x38 ; bits 3-5
xor [dest]
mov [dest], a


; read bits 3-5 of memory into A
; A must be 0
xor [src]
and a, 0xc7 ; not bits 3-5
xor [src]



; arithmetic shift right
        cmp a, 0x80
        ror a
        

; arithmetic shift right 4 bits at once
        shr a
        shr a
        shr a
        shr a
        add a, 0xf8
        xor a, 0xf8


; Average of two numbers (2 bytes)
; inputs: A, B
; result: A
add b
ror a


; Absolute value
; input: A, with N correctly set
; result: A
    bcc abs
    xor a, 0xff
    inc a
abs:


