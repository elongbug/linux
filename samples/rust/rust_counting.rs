// SPDX-License-Identifier: GPL-2.0

#![allow(dead_code)]
//#![allow(unused_imports)]
//! Rust counting example for Kangrejos

use kernel::prelude::*;

module! {
    type: RustCounting,
    name: "rust_counting",
    author: "Benno Lossin",
    description: "Rust counting sample for Kangrejos",
    license: "GPL",
}

struct RustCounting;

impl kernel::Module for RustCounting {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        Ok(Self)
    }
}