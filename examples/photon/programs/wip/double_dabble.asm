; Decimal conversion
; arg a: Value to convert
; out e: hundreds
; out f: tens
; out a: ones
.hundreds:
        sub a, 100
        add e, 1
        geq a, 100
        bt .hundreds
.tens:  sub a, 10
        add f, 1
        geq a, 10
        bt .tens
        
