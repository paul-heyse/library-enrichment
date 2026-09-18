# `datafusion_optimizer::decorrelate::UN_MATCHED_ROW_INDICATOR`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.decorrelate.UN_MATCHED_ROW_INDICATOR.json).

<a id="op-bcaca5411e42949d7f9f3eb1"></a>
## UN_MATCHED_ROW_INDICATOR

`constant` · `datafusion_optimizer::decorrelate::UN_MATCHED_ROW_INDICATOR` · datafusion-optimizer 55.1.0

```rust
const UN_MATCHED_ROW_INDICATOR: &str = "__always_true"
```

Source: `src/decorrelate.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Used to indicate the unmatched rows from the inner(subquery) table after the left out Join
This is used to handle [the Count bug]

[the Count bug]: https://github.com/apache/datafusion/issues/10553
