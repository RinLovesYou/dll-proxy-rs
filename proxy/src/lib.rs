//! A Macro for defining an entry point for a cdylib.
//!
//! On Windows, this macro will wrap your function in DllMain, and call it when the DLL attaches.
//! It will lookup exports of supported proxies, based on our own Module Name, and store them.
//! Effectively, creating a dynamic proxy that we could add any number of supported proxies to.
//!
//! # Supported Targets
//!
//! - Windows
//!     - `x86_64-pc-windows-msvc`
//!     - `i686-pc-windows-msvc`
//!
//!
//! # Example
//!
//! ```
//! use proxy_dll::proxy;
//!
//! #[proxy]
//! fn main() {
//!    println!("Hello, world!");
//! }
//! ```
//!
//! # Safety
//!
//! This crate is pretty unsafe

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

pub use proxy::proxy;

pub use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::winnt::DLL_PROCESS_ATTACH,
};
