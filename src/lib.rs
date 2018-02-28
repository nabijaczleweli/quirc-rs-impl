//! Pure Rust QR code decoder,
//! translated from [Daniel Beer's C implementation](https://github.com/dlbeer/quirc),
//! the FFI to which is used by [the `quirc` crate](https://crates.io/crates/quirc).
//!
//! The crate also provides the C interface, which is A{B,P}I compatible with the original `quirc`
//! (see the `c-examples` directory for more).


extern crate libc;

pub mod constants;
pub mod util;
pub mod ffi;
pub mod ops;
