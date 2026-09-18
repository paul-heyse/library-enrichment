# `datafusion_common::assert_not_contains`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_not_contains.json).

<a id="op-5fedc3d3818ddfdce321d548"></a>
## assert_not_contains

`macro` · `datafusion_common::assert_not_contains` · datafusion-common 55.1.0

```rust
macro_rules! assert_not_contains
```

Source: `src/test_util.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A macro to assert that one string is NOT contained within another with
a nice error message if they are.

Usage: `assert_not_contains!(actual, unexpected)`

Is a macro so test error
messages are on the same line as the failure;

Both arguments must be convertible into Strings ([`Into`]<[`String`]>)

Unresolved upstream links (retained, not inferred): ``String``, ``Into``.
