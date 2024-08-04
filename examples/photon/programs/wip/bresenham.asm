        ; swap two registers using xor trick
swap:   .macro a, b
        xor a, b
        xor b, a
        xor a, b
        .endm

        ; compute the absolute value through a temporary
abs:    .macro a, tmp
        neg tmp, a
        geq a, 0
        mvf a, tmp

        ; plot a point
plot:   .macro x, y
        pst x, xpos
        pst y, ypos
        pst x, draw



        ; computes
        ; e = abs(c - a)
        ; f = abs(d - b)
        ; a, b, c, d remain untouched
        ; clobbers g
dxdy:   mov e, c
        mov f, d
        sub e, a        ; compute x2 - x1
        sub f, b        ; compute y2 - y1

        ; abs(e)
        neg g, e
        geq e, 0
        mvf e, g

        ; abs(f)
        neg g, f
        geq f, 0
        mvf f, g
        ret

main:   mov a, 5
        mov b, 10
        mov c, 56
        mov d, 44


        ; https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
        ; https://rosettacode.org/wiki/Bitmap/Bresenham%27s_line_algorithm#C++
        ; a: x1
        ; b: y1
        ; c: x2 / iteration count
        ; d: y2 / ystep
        ; e: dx
        ; f: dy
        ; g: error
        jsr dxdy
        geq e, f        ; abs(x2 - x1) >= abs(y2 - y1)
        bt skip

        ; swap x1, y1
        mov g, a
        mov a, b
        mov b, g

        ; swap x2, y2
        mov g, c
        mov c, d
        mov d, g

skip:   geq c, a        ; x2 >= x1
        bt skip2

        ; swap x1, x2
        mov g, a
        mov a, c
        mov c, g

        ; swap y1, y2
        mov g, b
        mov b, d
        mov d, g

skip2:  jsr dxdy

        ; compute d(ystep) = (y1 >= y2) ? -1 : 1
        geq b, d
        mov d, 1
        mov g, -1
        mvt d, g

        shr g, e        ; g (error) = dx / 2
        mov c, e        ; c (count) = dx
        neg g

        ; pst a, ticker   ; x1
        ; pst b, ticker   ; y1
        ; pst c, ticker   ; count
        ; pst d, ticker   ; ystep
        ; pst e, ticker   ; dx
        ; pst f, ticker   ; dy
        ; pst g, ticker   ; error

loop:   pst a, xpos
        pst b, ypos
        pst a, draw
        add g, f        ; error += dy
        ges g, 1        ; error >= 1
        cad b, d        ; y1 += ystep
        csb g, e        ; error -= dx
        add a, 1        ; x1 += 1
        btd c, loop