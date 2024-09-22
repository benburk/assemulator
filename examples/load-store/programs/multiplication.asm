
;       8x8=8 multiplication 
;       r7: 1 
;       r3: product 
;       r1: multiplicand 
;       r2: multiplier 
        
.doAdd: add r3, r1      ; r3 += A
.loop:  shl r1, r7      ; r1 << 1
start:  and r6, r2, r7  ; test lsb
        shr r2, r7 
        bt  r6, .doAdd  ; was B odd?
        bt  r2, .loop   ; loop while r2 != 0