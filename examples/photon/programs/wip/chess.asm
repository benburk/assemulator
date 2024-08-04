

        ; move chess piece
        ; read char at position in reg b
        pst b, screen_idx
        pld a, screen           ; read char at position b
        pst 0, screen           ; clear char
        pst c, screen_idx       ; set dest idx
        pst b, screen           ; write char
