# `arrow_arith::numeric::sub`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.sub.json).

<a id="op-069446292f1a85dcf97e39c6"></a>
## sub

`function` · `arrow_arith::numeric::sub` · arrow-arith 59.3.0

```rust
fn sub(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs - rhs`, returning an error on overflow
