; for snake
; queue, 32 elements. queue stores dx, dy


KEY_UP:         .set 1
KEY_DOWN:       .set 2
KEY_LEFT:       .set 4
KEY_RIGHT:      .set 8

; stores the keys pressed so we can undo movements
queue:          .fill 64, 0
; pointer to front of queue
front:          .u8 0
back:           .u8 0

tail_x:         .u8 0
tail_y:         .u8 0
head_x:         .u8 0
head_y:         .u8 0


        ; read buttons
        pld e, buttonsp

        ; write latest input
        ld a, front
        psh e, a

        tst e, KEY_UP
        bt handle_key_up

        ld a, back



        ; a,b: the coordinate
        ; c: the key

        tst c, KEY_UP
        cad b, -1
        tst c, KEY_DOWN
        


compute_next_position:

handle_key_up:
        mov a, 0
        mov b, -1
handle_key_down:
        mov a, 0
        mov b, 1
handle_key_left:
        mov a, -1
        mov b, 0
handle_key_right:
        mov a, 1
        mov b, 0



check_collision:


; registers
; a, b: x, y position
; c, d: dx, dy
; e: temporary


; get user input

        ; init
        mov a, 15       ; head x pos
        mov b, 15       ; head y pos
        mov c, 1        ; dx
        mov d, -1       ; dy
        mov e, 15       ; tail x pos
        mov f, 15       ; tail y pos

loop:
        ; read buttons
        pld e, buttonsp

        ; handle controls
        tst e, KEY_UP
        cad b, d
        tst e, KEY_DOWN
        cad b, c
        tst e, KEY_LEFT
        cad a, d
        tst e, KEY_RIGHT
        cad a, c

        ; (a, b) draw the new position
        pst a, xpos
        pst b, ypos
        
        ; check collision
        pld e, color    ; read color
        pst e, ticker
        eq e, 1         ; check if color is 1
        bt game_over    ; if so, game over

        pst draw
        jmp loop

game_over:
        mov e, -1
        pst e, ticker

; ; print apple
; 
;         pld a, RNG
;         and a, 0b11111
;         pst a, X
;         
;         pld a, RNG
;         and a, 0b11111
;         pst a, Y
; 
;         mov a, color
;         pst a, COLOR