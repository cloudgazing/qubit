# Qubit

Keyboard firmware with a variety of features, for different microcontrollers.

> [!NOTE]
> This project is still **work in progress**, those features are still being worked on and currently
the only chip supported is the RP2040 :).

## Firmware

## Requirements

1. [Rust](https://www.rust-lang.org) and `cargo`. The recommended [install method](https://www.rust-lang.org/tools/install) comes with `cargo` out of the box.
2. `cargo-make`. Run `cargo install cargo-make`.
3. `flip-link`. Run `cargo install flip-link`.
4. `elf2uf2-rs`. Run `cargo install --git https://github.com/StripedMonkey/elf2uf2-rs`.

## Building the firmware

Instructions are incomplete and need to be rewritten/finished.

## TODO:

- add LED support
- add N-Key rollover support
- add support for storing and updating data to non-volatile storage
- extend support different type of devices
- other stuff

---

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Dependency Licenses

Some dependencies of this project are licensed under different terms.

See [COPYRIGHT](COPYRIGHT) for details.
