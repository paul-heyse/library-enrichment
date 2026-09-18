# `arrow_string::like::nlike`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.nlike.json).

<a id="op-8d6e7e0b14228e1331d254f2"></a>
## nlike

`function` · `arrow_string::like::nlike` · arrow-string 59.3.0

```rust
fn nlike(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `left NOT LIKE right`

# Notes
- This is a negative of [`like`](../operations/arrow_string.like.like.md#op-4f5e15a014f56ccae12c54a6)
- See the documentation on [`like`](../operations/arrow_string.like.like.md#op-4f5e15a014f56ccae12c54a6) for more details
