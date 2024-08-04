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
        
