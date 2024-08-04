; multiply by 10
; arg a: initial value
        shl a, a    ; a = 2a
        mov b, a    ; b = 2a
        shl a, a    ; a = 4a
        shl a, a    ; b = 8a
        add a, b    ; a = 10a

        swap b, a
        shl a
        and b, 0x0f     ; b = 8a
        nop
        add a, b        ; a = 8a + 2a = 10a



; Average of two numbers
; computes a = (a+b)/2
        addx a, b
        shrc a


; Absolute value
        neg b, a
        geq a, 0
        mvf a, b        ; a = a >= 0 ? a : -a


; Swap two values if x is set
        mov tmp, a
        mov a, b
        mov b, tmp
        


; 16-bit addition
; out a: result low
; out b: result high
; arg a: a_lo
; arg b: a_hi
; arg c: b_lo
; arg d: b_hi
        addx a, c
        addc b, d



; Decimal conversion
; arg a: Value to convert
; out e: hundreds
; out f: tens
; out a: ones
-:      sub a, 100
        add e, 1
        geq a, 100
        bt -
-:      sub a, 10
        add f, 1
        geq a, 10
        bt -
        


; pi
; https://www.wikiwand.com/en/Spigot_algorithm
; https://stackoverflow.com/q/52348369/9518712





; cookie cutter
; Select specific bits of A into B
; write
        xor dest
        and 0b00111000  ; bits 3-5
        xor dest
        mov dest, a

; read (A is 0)
        xor src
        and 0b11000111  ; not bit 3-5
        xor src


; toggle x flag
        shlb a
        xor a, 1
        shrb a



; swap nibbles
; arg a: value to swap
        shl a
        addb a, 0x80
        shlc a
        addb a, 0x80
        shlc a



; compute parity
; http://forum.6502.org/viewtopic.php?p=4354&sid=d79dbf7bb9069cfa01723a0af4742667#p4354
        sta temp
        shl a
        eor temp
        and a, b10101010
        add a, b01100110
        and a, b10001000
        addx a, b01111000
        ; now the parity is in the sign bit



; increment lower 4 bits
; arg a: value
        add b, a, 1     ; b = a + 1
        and a, 0xf
        and b, 0xf      ; b = b & 0xf
        nop
        or a, b


; arithmetic shift right
        geq a, 0x80
        shrc a


; arithmetic shift right 4 bits at once
        swap a
        and a, 0xf0
        add a, 0xf8
        xor a, 0xf8



; Check if only one bit is set (power of 2)
; by computing x = a & (a - 1) != 0
; arg: a
; out: x=0 if true
        add b, a, -1
        tst a, b


; increment 16-bit counter in memory

counter:        .u16 0  ; initialize 16-bit counter to 0

        mov b, 1
        ld a, counter[0]
        addx a, b
        st a, counter[0]
        ld a, counter[1]
        addc a, c       ; c = 0
        st a, counter[1]



; Add two 4-bit vectors
; aka simd within a register https://en.wikipedia.org/wiki/SWAR
; https://news.ycombinator.com/item?id=25301039
; arg a, b

        and a, 0x77
        and b, 0x77
        add a, b        ; a = a + b