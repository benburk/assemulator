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
```
