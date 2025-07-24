# Qubit

Keyboard firmware with a variety of features, for different microcontrollers.

> [!NOTE]
> This project is still **work in progress**, those features are still being worked on :).
>
> Chips supported:
>
> - RP2040
> - STM32F411

> [!NOTE]
> RA version 2025-07-21 has an issue with reading env values from cargo config. This affects the import_device macro
> used in qubit's build.rs. Use version 2025-07-14 until that issue is fixed.

## Building the firmware

### Requirements

1. **Rust**. The recommended [install method](https://www.rust-lang.org/tools/install) comes with `cargo` out of the box.
2. Build tools. These can be installed by running `cargo install cargo-make flip-link`.

After installing all the required tools simply run `cargo make build --author <author name> --model <model name>`
to build for one of the supported devices. (building using a toml configuration file is not yet implemented)

## Flashing

The easiest way to flash the firmware is using a probe. This project uses [probe-rs](https://github.com/probe-rs/probe-rs) to help with that.
To install with cargo, run `cargo install probe-rs-tools --locked`.

Eg. flashing for STM32F411:

```zsh
probe-rs run --chip STM32F411CEUx target/thumbv7em-none-eabihf/debug/qubit`
```

### Flashing without a probe

For RP2040 specifically `elf2uf2-rs` can be used to turn the binary into a UF2 file.
Run 
```zsh
cargo install --git https://github.com/StripedMonkey/elf2uf2-rs
``` 

to install it and

```zsh
elf2uf2-rs target/thumbv6m-none-eabi/debug/qubit qubit.uf2
``` 

to convert it, then drag and drop it on the mounted device.

## TODO:

- finish writing instructions for building the firmware
- switch between NKRO and 6KRO at runtime
- add LED support
- add storage support
- add support for other components
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
