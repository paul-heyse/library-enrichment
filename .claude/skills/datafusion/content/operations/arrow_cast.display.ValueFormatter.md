# `arrow_cast::display::ValueFormatter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.ValueFormatter.json).

<a id="op-9ae555388794c1910e97ad8a"></a>
## ValueFormatter

`struct` · `arrow_cast::display::ValueFormatter` · arrow-cast 59.3.0

```rust
struct ValueFormatter<'a>
```

Source: `src/display.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Implements [`Display`] for a specific array value

Unresolved upstream links (retained, not inferred): ``Display``.

<a id="op-5c3959a78608538e152d3c56"></a>
## fmt

`function` · `arrow_cast::display::ValueFormatter::fmt` · arrow-cast 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::display::ValueFormatter", "path": "ValueFormatter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 1], "end": [446, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/display.rs:437`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62debfe1429f9191313ac4ba"></a>
## try_to_string

`function` · `arrow_cast::display::ValueFormatter::try_to_string` · arrow-cast 59.3.0

```rust
fn try_to_string(&self) -> Result<String, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::display::ValueFormatter", "path": "ValueFormatter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [434, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:429`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Fallibly converts this to a string

<a id="op-670b0503c94f323521d1c48c"></a>
## write

`function` · `arrow_cast::display::ValueFormatter::write` · arrow-cast 59.3.0

```rust
fn write(&self, s: &mut dyn Write) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::display::ValueFormatter", "path": "ValueFormatter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [434, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:420`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Writes this value to the provided [`Write`]

Note: this ignores [`FormatOptions::with_display_error`](../operations/arrow_cast.display.FormatOptions.md#op-41375742483c8a65e4258294) and
will return an error on formatting issue

Unresolved upstream links (retained, not inferred): ``Write``.
