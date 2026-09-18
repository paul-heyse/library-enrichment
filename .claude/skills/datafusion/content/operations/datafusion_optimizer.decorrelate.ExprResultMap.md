# `datafusion_optimizer::decorrelate::ExprResultMap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.decorrelate.ExprResultMap.json).

<a id="op-0ff482b56065bb274770e533"></a>
## ExprResultMap

`type_alias` · `datafusion_optimizer::decorrelate::ExprResultMap` · datafusion-optimizer 55.1.0

```rust
type ExprResultMap = datafusion_common::HashMap<String, datafusion_expr::Expr>
```

Source: `src/decorrelate.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Mapping from expr display name to its evaluation result on empty record
batch (for example: 'count(*)' is 'ScalarValue(0)', 'count(*) + 2' is
'ScalarValue(2)')
