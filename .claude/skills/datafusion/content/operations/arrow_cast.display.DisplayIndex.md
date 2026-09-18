# `arrow_cast::display::DisplayIndex`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.DisplayIndex.json).

<a id="op-b45c112d21144439be9d19f8"></a>
## DisplayIndex

`trait` · `arrow_cast::display::DisplayIndex` · arrow-cast 59.3.0

```rust
trait DisplayIndex
```

Source: `src/display.rs:596`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

[`Display`] but accepting an index

Unresolved upstream links (retained, not inferred): ``Display``.

<a id="op-38df84f573de1fecce167bb1"></a>
## write

`function` · `arrow_cast::display::DisplayIndex::write` · arrow-cast 59.3.0

```rust
fn write(&self, idx: usize, f: &mut dyn Write) -> FormatResult
```

Source: `src/display.rs:598`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Write the value of the underlying array at `idx` to `f`.
