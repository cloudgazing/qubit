# Qubit

Keyboard firmware with a variety of features, for different microcontrollers.

> [!NOTE]
> This project is still **work in progress**, those features are still being worked on and currently
the only chip supported is the RP2040 :).

## Firmware

| Size(kB) | Profile | Model      |
| -------- | ------- | ---------- |
| 352.5    | dev     | MoonQuartz |
| 56.5     | release | MoonQuartz |

## Building the firmware

> [!TIP]
> Skip the last two steps if all you want is to upload the firmware on your board.

1. Install [rust](https://www.rust-lang.org)
2. Add the `thumbv6m-none-eabi` toolchain by running `rustup target add thumbv6m-none-eabi`
3. Install [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs) by running `cargo install elf2uf2-rs`
4. Run `cargo build -r` to build the firmware.
5. Convert the elf to uf2 by running `elf2uf2-rs target/thumbv6m-none-eabi/release/qubit qubit.uf2`.

## Uploading the firmware on your board

1. Install the required tools and toolchain using the steps above.
2. Put the pico into USB mass storage device mode. This is usually done by holding the `BOOTSEL` button and plugging the board in.
3. Run `cargo run -r` which should automatically find the board and upload the firmware.

> [!NOTE]
> In the case `elf2uf2-rs` cannot find your board, you can manually upload it by [building the firmware](#building-the-firmware) using all the steps and dragging the `.uf2` file on your device.

## Making a custom version of the firmware

See the instructions [here](./crates/keyboards/models/custom/README.md). (not finished)

## TODO:

- add LED support
- add N-Key rollover support
- add support for storing and updating data to non-volatile storage
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
