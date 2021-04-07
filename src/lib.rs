//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # delayed-assert
//!
//! A simple library for delaying asserts.
//!
//! ## Sample Usage
//! ```rust
//! # use delayed_assert::DelayedAssert;
//! let mut da = DelayedAssert::new();
//! da.expect("something truthy");
//! da.expect_equal(42, 42);
//! da.assert_expectations();
//! ```
//!
//! [ci]: https://github.com/Elinvynia/delayed-assert/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/delayed-assert/Rust/master?style=flat-square
//! [docs]: https://docs.rs/delayed-assert
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/delayed-assert
//! [crate-version]: https://img.shields.io/crates/v/delayed-assert.svg?style=flat-square

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::fmt::Debug;
use truthy::Truthy;

/// A struct for keeping state of delayed asserts.
#[derive(Debug, Default)]
pub struct DelayedAssert {
    asserts: u32,
    messages: Vec<String>,
}

impl DelayedAssert {
    /// Creates a new holder for delayed asserts.
    pub fn new() -> Self {
        DelayedAssert {
            asserts: 0,
            messages: vec![],
        }
    }

    /// Use just like the `assert!` macro, adds it to the queue.
    pub fn expect<T: Truthy + Debug>(&mut self, value: T) {
        self.asserts += 1;
        if !value.truthy() {
            self.messages
                .push(format!("{}. Assertion Failed! {:?}", self.asserts, value));
        }
    }

    /// Use just like the `assert_eq!` macro, adds it to the queue.
    pub fn expect_equal<T: PartialEq + Debug>(&mut self, left: T, right: T) {
        self.asserts += 1;
        if !(left == right) {
            self.messages.push(format!(
                "{}. Assertion Failed! Values are not equal!\nLeft: {:?}\nRight: {:?}",
                self.asserts, left, right
            ));
        }
    }

    /// Run the asserts.
    pub fn assert_expectations(self) {
        let length = self.messages.len();

        for message in self.messages {
            println!("{}", message)
        }

        println!("In total {}/{} asserts failed.", length, self.asserts);
    }
}
