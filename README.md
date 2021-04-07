[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# delayed-assert

A simple library for delaying asserts.

## Sample Usage
```rust
    use delayed_assert::DelayedAssert;

    ...

    let mut da = DelayedAssert::new();
    da.expect("something truthy");
    da.expect_equal(42, 42);
    da.assert_expectations();
```

[ci]: https://github.com/Elinvynia/delayed-assert/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/delayed-assert/Rust/master?style=flat-square
[docs]: https://docs.rs/delayed-assert
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/delayed-assert
[crate-version]: https://img.shields.io/crates/v/delayed-assert.svg?style=flat-square
