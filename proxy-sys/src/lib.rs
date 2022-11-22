//! A Dynamic DLL Proxy written in Rust.
//! 
//! These are the internals of the Proxy macro.

#![feature(naked_functions)]
#![feature(asm_const)]

#![deny(
    missing_debug_implementations,
    missing_docs,
    unused_results,
    warnings,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion
)]
#![forbid(rust_2018_idioms)]
#![allow(clippy::inherent_to_string, clippy::type_complexity, improper_ctypes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod exports;