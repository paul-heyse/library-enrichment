# `arrow_cast::display::FormatError`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.FormatError.json).

<a id="op-f8cb8919f51aec9c78d65c10"></a>
## FormatError

`enum` · `arrow_cast::display::FormatError` · arrow-cast 59.3.0

```rust
enum FormatError
```

Source: `src/display.rs:573`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Either an [`ArrowError`](../operations/arrow_schema.error.ArrowError.md#op-0b9e83026812de4ef436507a) or [`std::fmt::Error`]

Unresolved upstream links (retained, not inferred): ``std::fmt::Error``.

<a id="op-1eafe5bf1a26aae30db604cd"></a>
## Arrow

`variant` · `arrow_cast::display::FormatError::Arrow` · arrow-cast 59.3.0

```rust
Arrow
```

Source: `src/display.rs:577`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

An Arrow error occurred while formatting the array.

<a id="op-8002274d60dacbc883d7d1cb"></a>
## Format

`variant` · `arrow_cast::display::FormatError::Format` · arrow-cast 59.3.0

```rust
Format
```

Source: `src/display.rs:575`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

An error occurred while formatting the array

<a id="op-875369cb90d08fe7fa2752da"></a>
## from

`function` · `arrow_cast::display::FormatError::from` · arrow-cast 59.3.0

```rust
fn from(value: std::fmt::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::display::FormatError", "path": "FormatError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [587, 2], "filename": "src/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::fmt::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/display.rs:584`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3bb034617a9015696f31d11"></a>
## from

`function` · `arrow_cast::display::FormatError::from` · arrow-cast 59.3.0

```rust
fn from(value: ArrowError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::display::FormatError", "path": "FormatError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 1], "end": [593, 2], "filename": "src/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/display.rs:590`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
