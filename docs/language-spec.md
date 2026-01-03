# Language Specification

This document describes the language specification used for the simulated RAM as described [here](https://arbeitsplattform.bildung.hessen.de/fach/informatik/registermaschine.html).

<div style="display: grid; place-items: center;">
    <img alt="Railroad diagram of the RAM's language grammar" src="assets/language-grammar.svg">
</div>

## Table of Contents

- [Program](#program)
- [Instructions](#instructions)
    - [LOAD](#load)
    - [STORE](#store)
    - [ADD](#add)
    - [SUB](#sub)
    - [MUL](#mul)
    - [DIV](#div)
    - [GOTO](#goto)
    - [JZERO](#jzero)
    - [JNZERO](#jnzero)
    - [END](#end)
- [Operands](#operands)
    - [Immediate Values](#immediate-values)
    - [Direct Addressing](#direct-addressing)
    - [Indirect Addressing](#indirect-addressing)
- [Labels](#labels)
- [Comments](#comments)

## Program

- A program consists of a sequence of [instructions](#instructions) separated by new lines.
- Leading and trailing whitespace, empty lines, and [comments](#comments) are ignored.
- Instruction keywords are **case-insensitive**.
- If an instruction does not match the specification, an error is raised **before execution**.
- Program execution terminates when:
    - a division by `0` occurs
    - the [`end`](#end) instruction is executed
    - the end of the file is reached (no further instructions)

## Instructions

### LOAD

Syntax: `load <operand>`

Loads the value of the operand into the [accumulator](ram-spec.md#accumulator).

### STORE

Syntax: `store <operand>`

> [!IMPORTANT]
> The `store` instruction does **not** allow [immediate values](#immediate-values).

Stores the value of the accumulator into the specified register.

### ADD

Syntax: `add <operand>`

Adds the operand’s value to the accumulator and stores the result in the accumulator.

### SUB

Syntax: `sub <operand>`

Subtracts the operand’s value from the accumulator and stores the result in the accumulator.

### MUL

Syntax: `mul <operand>`

Multiplies the accumulator by the operand’s value and stores the product in the accumulator.

### DIV

Syntax: `div <operand>`

Divides the accumulator by the operand’s value and stores the result in the accumulator.

> [!WARNING]
> Division by zero immediately terminates the program.

### GOTO

Syntax: `goto <label>`

Unconditionally jumps to the instruction marked with the given [label](#labels).

### JZERO

Syntax: `jzero <label>`

Jumps to the given [label](#labels) if the accumulator equals `0`.

### JNZERO

Syntax: `jnzero <label>`

Jumps to the given [label](#labels) if the accumulator is **not** equal to (greater than) `0`.

### END

Syntax: `end`

Marks the end of the program by setting the [program counter](ram-spec.md#program-counter) to infinity.

## Operands

### Immediate Values

Syntax: `#<value>`

Uses a constant value directly.

### Direct Addressing

Syntax: `<register_address>`

Accesses the value stored in the given register.

### Indirect Addressing

Syntax: `*<register_address>`

Uses the value stored in the given register as a register address. This is equivalent to a pointer with a depth of 1.

## Labels

Syntax: `<label_name>:`

A label marks a position in the program that can be jumped to using
[`goto`](#goto), [`jzero`](#jzero), or [`jnzero`](#jnzero).

- Label names must not contain spaces.
- If a label is followed directly by a line break or a comment, it is still a valid jump target.

## Comments

Syntax: `// <comment>`

A comment starts with `//` and continues until the next line break. Comments are ignored during execution.
