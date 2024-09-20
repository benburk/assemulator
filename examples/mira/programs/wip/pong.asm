
ball_x:  .i8 0
ball_y:  .i8 0
ball_dx: .i8 1
ball_dy: .i8 1

paddle_y .i8 0  ; paddle occupies cells from y to y+PADDLE_SIZE-1

WIDTH = 8
HEIGHT = 8
PADDLE_SIZE = 3


; update ball x position
; update dx/dy if it hits a wall
    mov a, ball_x       ; load ball x position
    bzs negate          ; ball_x==0?
    cmp a, WIDTH - 1    ; ball in front of paddle
    bzs negate

negate:
    xor ball_dx, 0xff   ; negate ball_dx
    inc ball_dx



    add a, ball_dx
    cmp a, 

