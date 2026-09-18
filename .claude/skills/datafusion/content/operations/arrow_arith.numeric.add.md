# `arrow_arith::numeric::add`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.add.json).

<a id="op-69744f7167daf77071db32c9"></a>
## add

`function` · `arrow_arith::numeric::add` · arrow-arith 59.3.0

```rust
fn add(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs + rhs`, returning an error on overflow
