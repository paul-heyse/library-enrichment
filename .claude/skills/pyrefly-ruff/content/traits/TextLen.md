# TextLen

`ruff_text_size::traits::TextLen`

```rust
trait TextLen: Copy + Sealed
```

Also reachable as `ruff_text_size::TextLen`

Prose: [`api/ruff_text_size.traits.md`](../api/ruff_text_size.traits.md#textlen) · records: [`model/ruff_text_size.traits.json`](../model/ruff_text_size.traits.json)

## Required

Every implementation must supply these.

```rust
fn text_len(self) -> TextSize
```

## Documentation

Primitives with a textual length that can be passed to [`TextSize::of`].
