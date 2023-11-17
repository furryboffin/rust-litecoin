// SPDX-License-Identifier: CC0-1.0

//! # Rust Litecoin Internal
//!
//! This crate is only meant to be used internally by crates in the
//! [rust-litecoin](https://github.com/rust-litecoin) ecosystem.
//!

#![no_std]
// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Coding conventions
#![warn(missing_docs)]
// Exclude clippy lints we don't think are valuable
#![allow(clippy::needless_question_mark)] // https://github.com/rust-litecoin/rust-litecoin/pull/2134

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod macros;
mod parse;
pub mod serde;
