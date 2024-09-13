//!
//! This crate provides an async Rust driver for the [ST25DV-I2C](https://www.st.com/en/nfc/st25dv-i2c-series-dynamic-nfc-tags.html)
//! series of NFC/RFID contactless interfaces, based on the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.
//!

#![cfg_attr(not(test), no_std)]

pub struct ST25DV {

}

impl ST25DV {
    pub fn new() -> Self {
        Self {

        }
    }
}
