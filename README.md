# ARAMS - Abstract Random-Access Machine (RAM) Simulation

ARAMS is a simulator for an Abstract Random-Access Machine (RAM) intended for educational use, with a particular focus on the Hessian Landesabitur (Informatik).

The simulator is based on the same instruction set and operational semantics as the [Remasp GUI](https://github.com/groehner/Remasp) and is fully compatible with the RAM/register machine model used in Hessian computer science education, as described [here](https://arbeitsplattform.bildung.hessen.de/fach/informatik/registermaschine.html).

## Documentation

The detailed specifications for the ARAMS simulator can be found in the `docs` directory:

- [RAM Specification](docs/ram-spec.md) Describes the abstract machine model and its structure.
- [Language Specification](docs/language-spec.md) Defines the RAM assembly language syntax, instructions and operands.
- [CLI](crates/arams-cli/README.md) Documentation for using the command-line interface to run and debug ARAMS programs.
- [WASM Package](crates/arams-wasm/README.md) Guide for the published NPM package, enabling ARAMS execution directly in your browser.

## License

This project is licensed under the [MIT License](https://opensource.org/license/mit).
