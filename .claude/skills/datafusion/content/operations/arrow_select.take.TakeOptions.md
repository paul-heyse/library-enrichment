# `arrow_select::take::TakeOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.take.TakeOptions.json).

<a id="op-87fdafaf7935d7c5843791a8"></a>
## TakeOptions

`struct` · `arrow_select::take::TakeOptions` · arrow-select 59.3.0

```rust
struct TakeOptions
```

Source: `src/take.rs:391`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Options that define how `take` should behave

<a id="op-5eeff4ec58b4ef4e8e70ed2b"></a>
## check_bounds

`struct_field` · `arrow_select::take::TakeOptions::check_bounds` · arrow-select 59.3.0

```rust
check_bounds: bool
```

Source: `src/take.rs:395`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Perform bounds check before taking indices from values.
If enabled, an `ArrowError` is returned if the indices are out of bounds.
If not enabled, and indices exceed bounds, the kernel will panic.

<a id="op-616a948cb1812d1d2ac2016b"></a>
## clone

`function` · `arrow_select::take::TakeOptions::clone` · arrow-select 59.3.0

```rust
fn clone(&self) -> TakeOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::take::TakeOptions", "path": "TakeOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [390, 10], "end": [390, 15], "filename": "src/take.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/take.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc813e5b174bb4d2f83db376"></a>
## default

`function` · `arrow_select::take::TakeOptions::default` · arrow-select 59.3.0

```rust
fn default() -> TakeOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::take::TakeOptions", "path": "TakeOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [390, 24], "end": [390, 31], "filename": "src/take.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/take.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-042a724df3e196627782ea4b"></a>
## fmt

`function` · `arrow_select::take::TakeOptions::fmt` · arrow-select 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::take::TakeOptions", "path": "TakeOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [390, 17], "end": [390, 22], "filename": "src/take.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/take.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
