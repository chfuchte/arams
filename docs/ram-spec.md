# Random-Access Machine (RAM) Specification

## Accumulator

The accumulator is a special register used implicitly by all arithmetic and load/store instructions. It holds the current working value for computations and comparisons.

## Registers

The RAM provides an infinite number of registers. Registers are addressed by natural numbers starting at `1` and increasing without an upper bound.

## Program Counter

The program counter stores the line number of the instruction currently being executed. After each instruction, it is incremented to point to the next instruction unless modified by a [jump instruction](language-spec.md#goto).

## Values

All values processed by the RAM are **natural numbers**, including `0`. Negative values are not supported.
