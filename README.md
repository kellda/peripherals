# peripherals

Define and access your microcontroller peripherals

# Features

- A macro to generate code for peripherals, registers, and register fields
- Zero-sized structs to represent peripherals and registers
- Enums or newtype struct to represent fields values
- Combine values to write to a regiser with `|` (binary or)
- Read/write access at the regiser level
- Strong typing ensures you don't mix up registers
- Generic over the peripheral instance
- See the example module for an example of usage and generated API

# Inspiration

This is inspired primarly from

- [My Adventures in MMIO Abstraction](https://gist.github.com/Measter/2108508ba25ebe3978a6c10a1e01b9ad)
- [MMIO Abstraction Example](https://gist.github.com/Measter/393f402997520bf2ea213eef34d78e86)
- [rumio](https://crates.io/crates/rumio)

and likely by every other regiser access crate out there

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
