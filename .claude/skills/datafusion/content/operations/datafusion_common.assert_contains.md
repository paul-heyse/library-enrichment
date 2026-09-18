# `datafusion_common::assert_contains`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_contains.json).

<a id="op-219e797bb45b75ea6dd6ada3"></a>
## assert_contains

`macro` · `datafusion_common::assert_contains` · datafusion-common 55.1.0

```rust
macro_rules! assert_contains
```

Source: `src/test_util.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A macro to assert that one string is contained within another with
a nice error message if they are not.

Usage: `assert_contains!(actual, expected)`

Is a macro so test error
messages are on the same line as the failure;

Both arguments must be convertible into Strings ([`Into`]<[`String`]>)

Unresolved upstream links (retained, not inferred): ``String``, ``Into``.
