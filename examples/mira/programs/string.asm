; print
msg:    .strz "Hello, World!"
        
loop:   mov TERM, a     ; print char
start:  inc x           ; post-increment
        mov a, msg[x]
        bzc loop



; strlen
; result in x
str:    .strz "hello, world!"
loop:   inc x           ; post-increment
        mov a, str[x]   ; load the character
        bne loop        ; loop while non-zero
        

; strcmp
loop:   inc x
        mov a, str1[x]
        beq end
        mov b, str2[x]
        beq end
        sub b
        bne loop