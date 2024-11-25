# DHDL

This repo contains a hardware description language that transpiles to [Digital](https://github.com/hneemann/Digital).

## Example

```
half_adder {
    @in(1) a, b

    @out sum = a ^ b
    @out carry = a & b
}

full_adder {
    @in(1) a, b, c

    ha_1 = half_adder(a: a, b: b)
    ha_2 = half_adder(a: c, b: ha_1.sum)

    @out sum = ha_2.sum
    @out carry = ha_1.carry | ha_2.carry
}

adder_8_bit {
    @in(8) a, b

    out_0 = full_adder(a: a.0, b: b.0, c: 0)
    out_1 = full_adder(a: a.1, b: b.1, c: out_0.carry)
    out_2 = full_adder(a: a.2, b: b.2, c: out_1.carry)
    out_3 = full_adder(a: a.3, b: b.3, c: out_2.carry)
    out_4 = full_adder(a: a.4, b: b.4, c: out_3.carry)
    out_5 = full_adder(a: a.5, b: b.5, c: out_4.carry)
    out_6 = full_adder(a: a.6, b: b.6, c: out_5.carry)
    out_7 = full_adder(a: a.7, b: b.7, c: out_6.carry)

    @out sum = [
        0: out_0.sum,
        1: out_1.sum,
        2: out_2.sum,
        3: out_3.sum,
        4: out_4.sum,
        5: out_5.sum,
        6: out_6.sum,
        7: out_7.sum,
    ]

    @out carry = out_7.carry
}

@in(8) a
@in(8) b

result = adder_8_bit(a: a, b: b)
@out sum = result.sum
@out carry = result.carry
```

## Usage

Currently, DHDL doesn't have a CLI. To use DHDL, you need to have rust installed. To run the example above, create a file called `adder.dhl` inside tests/, create an output/ directory and run the following command:

```
cargo r adder
```

This will create a file called `adder.dig` inside the output/ directory. You can then open this file in Digital.

## Features

### Comments

Comments are denoted by `//`. Everything after `//` on a line is ignored. Keep in mind that newline continuation is not supported, so comments must be on their own line.

### Wires

In DHDL, every variable assignment is a wire. Wires can be assigned to the result of a logic gate, a constant, another wire or a combination of wires. Wires can have different bit widths. The bit width of a wire is automatically inferred from the bit width of the assigned value.

For constants, the bit width of the assigned wire is the lowest
number of bits that can represent the constant. Since we always work with unsigned values, this doesn't cause any issues.

When using a wire in an expression, the width of the wire is automatically extended / reduced to the width of the expression. This is done by zero-extending the wire, or truncating the wire to its least-significant bits.

Note that a wire cannot be assigned multiple times. This disallows any kind of feedback loops, so if you want to create a flip-flop, you have to import it as an external module with the external module syntax.

A wire can be designated as an input or an output. To do this, use the `@in` and `@out` annotations. The bit width of the input or output is specified in the parentheses. If the bit width is omitted, it defaults to 1 for an input, and gets inferred from usage for the output.

### Logic gates

DHDL supports the following logic gates:

- `!` (NOT)
- `&` (AND)
- `|` (OR)
- `^` (XOR)
- `!&` (NAND)
- `!|` (NOR)
- `!^` (XNOR)

Note: The NOT gate, despite the `!` operator, is still a bitwise operator.

When using a logic gate, the bit width of the result is the maximum of the bit widths of the inputs. Gates are automatically repeated to match the bit width of the inputs.

### Wire Slicing

Sometimes, we might want to manually cast a wire to a different bit width. This can be done using the slicing syntax. The syntax is as follows:

```
wire_a = a.0..3
wire_b = a.3..7
wire_c = a.2
```

This will create a wire `wire_a` that is bits 0 to34 of `a`, a wire `wire_b` that is bits 3 to 7 of `a`, and a wire `wire_c` that is bit 2 of `a`.

If the slice is out of bounds, the rest of the bits are filled with zeros.

### Wire Concatenation

We might want to put multiple wires together to form a single wire. This can be done using the concatenation syntax. The syntax is as follows:

```
data = [
    0, 1, 2, 3: wire_a,
    4..6: wire_b,
    7: wire_c,
]
```

Here, bits 0 to 3 of `data` are the least significant bit of `wire_a`, bits 4 to 6 are the least significant bits of `wire_b`, and bit 7 is `wire_c`.

Using the range syntax in the concatenation syntax clones the appropriate amount of wire to the specified range, which is different from manually specifying the bits (in which case, only the lowest bit will be copied).

Wire concatenation can also specify names instead of numbers. This creates an object with the specified names as keys, and the corresponding wires as values.

```
data = [
    a: wire_a,
    b: wire_b,
    c: wire_c,
]
```

This creates an object with keys `a`, `b`, and `c`, and the corresponding wires as values. To access the wires, use the dot operator.

```
data.a
data.b
data.c
```

If an object only has a single value, the value can be accessed directly without using the key.

```
data
```

### Modules

Modules are a way to encapsulate logic. A module is defined using the following syntax:

```
module_name {
    // logic
}
```

A module can have inputs and outputs. Inputs and outputs are defined using the `@in` and `@out` annotations, just like the global context.

When using a module, the module usage syntax is used. The syntax is as follows:

```
module_data = module_name(input_name: input_value, ...)
```

If the module only has one input, the input name can be omitted.

```
module_data = module_name(input_value)
```

The module returns an object with the module's outputs as keys, and the corresponding wires as values. To access the wires, use the dot operator, as before. Keep in mind that if
the module only has one output, the wire can be accessed directly.

```
wire = module_name(input_value)
@out o = wire
```

### External Modules

DHDL doesn't implement every single component in Digital. To use components that aren't implemented in DHDL, you can import them as external modules. External modules are defined using the following syntax:

```
* SixteenSeg {
    @in(16) value   @ (40, 140)
    @in(1) dot      @ (60, 140)

    segSize = 5
    Color = rgba(255, 0, 0, 255)
}
```

Optionally, an external module can be renamed using the following syntax:

```
* MyCoolSixteenSegmentDisplay: SixteenSeg {
    @in(16) value   @ (40, 140)
    @in(1) dot      @ (60, 140)

    segSize = 5
    Color = rgba(255, 0, 0, 255)
}
```

After the rename, the external module can be used as `MyCoolSixteenSegmentDisplay`.

An external module can have inputs and outputs, just like a normal module. The inputs and outputs are defined using the `@in` and `@out` annotations, just like the global context, but the bit width is required. After each input and output, a position should be specified using the `@` symbol. The position is the position of the input or output on the component, relative to the component position. The position is specified as `(x, y)`, where `x` and `y` are the x and y coordinates of the input or output.

After the inputs and outputs, the external module can have any number of attributes. These variables are used to configure the component.

The supported attribute types are:

- `int` -> `attribute = 5`
- `long` -> `attribute = 5L` or `attribute = 5l`
- `string` -> `attribute = "string"`
- `bool` -> `attribute = true` or `attribute = false`
- `color` -> `attribute = rgba(255, 0, 0, 255)` or `attribute = rgb(255, 0, 0)`

These can be determined by first using the component in Digital, saving the file and checking the generated XML file.

The usage of an external module is no different from the usage of a normal module. The external module also returns an object.

## Macro Expansion

DHDL doesn't have a preprocessor (yet), so to expand macros, an external macro processor must be used. Such a preprocessor
will be especially useful to repeat certain logic multiple times, or to create a large number of similar components (see the 8-bit adder example).

## Things that aren't implemented yet

This project was created in a single day, from start to finish, so some features are unfortunately still missing. These include:

- Proper error messages
- A preprocessor
- Proper CLI
- Testing
- Digital .dig files -> DHDL for easy template editing

## Credits

This project wouldn't be possible without the amazing work of [Helmut Neemann](https://github.com/hneemann) on [Digital](https://github.com/hneemann/Digital).

The syntax and the semantics are loosely inspired by Verilog.
