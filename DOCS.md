# Assemulator reference manual

## About

This is the technical documentation and user guide for the Assemulator framework.

```
Assemulator

Usage: <CPU> <FILE> <COMMAND>

Commands:
  assemble  Assemble the program
  run       Run the program
  help      Print this message or the help of the given subcommand(s)

Arguments:
  <FILE>  Input file

Options:
  -h, --help  Print hel
```

## Syntax

Tried to keep consistent with GNU assembly syntax. Piggyback off syntax highlighting infra.

`(label:)?  (opcode arg_list?)?  (; comment)?`

## Labels

Labels are names given to values. Labels can represent addresses in memory, or constants used by the program. Assemulator supports global and local labels. Local labels are prefixed with a `.`.

When resolving local labels, assemulator will go up until the next global label, and then down to the next global label. You can define multiple local labels with the same name.

```
.local_label:  add r1, r2, r3
global_label:  jmp .local_label
```

## Directives

Directives are special instructions for the assembler. They are prefixed with a `.`.

## Directives

```
.i8     <expr>[, <expr>, ...]
        List of bytes, also supports .i16, etc.

.zero   <count>
        fills memory with n zeros

.strz   "<string1>"[, "<string2>"...]
        array of null-terminated strings

.set    <expression>
        Set the value of the label to <expression>
        These need to be forward declared

.macro  [<argname1>[,<argname2>...]]
        Defines a macro

.endm
        ends a macro definition

.if     <expr>
        If the expression is true, the following code is assembled
        until .endif is reached.

.include <file>
        Includes the contents of <file> at the current position
        in the assembly file. If label is present, all labels in
        <file> are prefixed with it

.org    <exp>[,<fill>]
        Sets the address of the current code to exp
```



## Ports


- integer
- character
- screen x
- screen y
- screen color
- rng: set seed / read value
- flip
- draw


## Graphics

- 64x64 bitmap display
- 8x8 sprites, can fit 64 on a screen
- each sprite has a 6-bit id. The upper 2 bits represent whether to flip the sprite horizontally and vertically

screen only gets updated with call to draw or flip.
draws happen to framebuffer
would be nice to be able to plot pixels without writing to `color`
how do we read a pixel though
we could do this using two buffers, how to implement in minecraft though?


x
    write: x coordinate
    read: x coordinate
y
    write: y coordinate
    read: y coordinate

color
    write: set color pallette
    read: get color pallette
    notes: 4-bit color pallette https://romanzolotarev.com/pico-8-color-palette/

draw
    write: draw current frame buffer, don't clear, stall for next frame

flip
    write: wait for next frame, draw and clear frame buffer

sprite
    write: draw sprite at (x,y)
    read: sprite id


how to implement scrolling without redrawing screen the whole time?

links
- https://www.youtube.com/@docrobs/videos

