# `arrow_string::like::ilike`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.ilike.json).

<a id="op-1800677cddbc35bbb9968c19"></a>
## ilike

`function` · `arrow_string::like::ilike` · arrow-string 59.3.0

```rust
fn ilike(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `left ILIKE right`

# Notes
- This is a case-insensitive version of [`like`](../operations/arrow_string.like.like.md#op-4f5e15a014f56ccae12c54a6)
- See the documentation on [`like`](../operations/arrow_string.like.like.md#op-4f5e15a014f56ccae12c54a6) for more details
- Implements loose matching as defined by the Unicode standard. For example,
  the `ﬀ` ligature is not equivalent to `FF` and `ß` is not equivalent to `SS`
