[![github](https://img.shields.io/github/actions/workflow/status/kkdh/st25dv-rs/build.yaml?branch=main&style=for-the-badge&logo=githubactions&label=build)](https://github.com/kkdh/st25dv-rs/actions?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/st25dv?style=for-the-badge&logo=rust)](https://docs.rs/st25dv)
[![crates.io](https://img.shields.io/crates/v/st25dv?style=for-the-badge&logo=rust)](https://crates.io/crates/st25dv)

# ST25DV

This crate provides an async Rust driver for the [ST25DV-I2C](https://www.st.com/en/nfc/st25dv-i2c-series-dynamic-nfc-tags.html) series of NFC/RFID contactless interfaces, based on the [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits.

The [ST25DV04KC](https://www.st.com/en/nfc/st25dv04kc.html), [ST25DV16KC](https://www.st.com/en/nfc/st25dv16kc.html) and [ST25DV64KC](https://www.st.com/en/nfc/st25dv64kc.html) are NFC RFID tags offering, respectively, 4, 16, and 64‑Kbit of electrically erasable programmable memory (EEPROM). These devices feature two interfaces: the first one is an I²C serial link that can be operated from a DC power supply, the second one is an RF link activated when the device acts as a contactless memory powered by the received carrier electromagnetic wave.

## Appetizer
```rust
tbd.
```

## [Documentation](https://docs.rs/st25dv)

## License
Licensed using the [Apache License Version 2.0](LICENSE).

## Contributing
All contributions are welcome. Any contribution intentionally submitted for inclusion in this crate by you, as defined in the [Apache-2.0 license](LICENSE), shall be licensed without any additional terms or conditions.
