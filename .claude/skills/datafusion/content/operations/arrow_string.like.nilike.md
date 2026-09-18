# `arrow_string::like::nilike`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.nilike.json).

<a id="op-4064d4ffa47ab82ccdb2783e"></a>
## nilike

`function` · `arrow_string::like::nilike` · arrow-string 59.3.0

```rust
fn nilike(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `left NOT ILIKE right`

# Notes
- This is a negative of [`like`](../operations/arrow_string.like.like.md#op-4f5e15a014f56ccae12c54a6)
- See the documentation on [`ilike`](../operations/arrow_string.like.ilike.md#op-1800677cddbc35bbb9968c19) for more details
