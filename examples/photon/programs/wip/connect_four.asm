

        ; the height of each position
        ; 0 means empty
        ; 1, 2, 3, 4, 5, 6
height: .fill 0, 7



        ; draw X
        ; a: column to draw piece
draw_x: 

        ld b, a, height         ; get height for that column

        ; check if column is full
        eq b, 6
        bt end



end:    ret





        ; connect four board is 6x7
board:  .fill 0, 7
        .fill 0, 7
        .fill 0, 7
        .fill 0, 7
        .fill 0, 7
        .fill 0, 7




; b: current column selector
; f: current player (0: o, 1: x)
; h: return address


        ; returns move in a
        ; clobbers: b
getmove:

        pld a, u8       ; get player move as column index
        ld b, a, height ; get height for that column
        eq b, 6         ; check if column is full
        bt getmove      ; yes? -> illegal move
        ret


; place piece
        ; compute index
        ; for 4x4 sprites, there are 256 locations

        ; a = 0 to 5 (height)
        ; b = 0 to 6 (column)

        
        
        shl a
        shl a
        shl a

        pst a, small_sprite



sprite_table:   .u16 0b0000_1010_0100_1010      ; x-piece
                .u16 0b0000_1110_1010_1110      ; o-piece



; increment lower byte
        swap a
        add a, 0x10
        swap a 