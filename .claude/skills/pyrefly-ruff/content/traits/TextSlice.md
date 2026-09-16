# TextSlice

`ruff_text_size::traits::TextSlice`

```rust
trait TextSlice: Sealed
```

Also reachable as `ruff_text_size::TextSlice`

Prose: [`api/ruff_text_size.traits.md`](../api/ruff_text_size.traits.md#textslice) · records: [`model/ruff_text_size.traits.json`](../model/ruff_text_size.traits.json)

## Required

Every implementation must supply these.

```rust
fn slice(&self, range: impl Ranged) -> &str
```

## Documentation

A slice of the source text.
