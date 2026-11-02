
ball_x:  .i8 0
ball_y:  .i8 0
ball_dx: .i8 1
ball_dy: .i8 1

paddle_y: .i8 0 ; paddle occupies cells from y to y+PADDLE_SIZE-1

WIDTH:       .set 8
HEIGHT:      .set 8
PADDLE_SIZE: .set 3


; update ball x position
; update dx/dy if it hits a wall
    mov a, [ball_x]     ; load ball x position
    beq negate          ; ball_x==0?
    cmp a, WIDTH-1      ; ball in front of paddle
    beq negate

negate:
    xor [ball_dx], 0xff ; negate ball_dx
    inc [ball_dx]



    add [ball_dx]
    cmp a, 

