# `datafusion_expr_common::signature::FIXED_SIZE_LIST_WILDCARD`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.FIXED_SIZE_LIST_WILDCARD.json).

<a id="op-8ccbc9e7ef1bab745b80519c"></a>
## FIXED_SIZE_LIST_WILDCARD

`constant` · `datafusion_expr_common::signature::FIXED_SIZE_LIST_WILDCARD` · datafusion-expr-common 55.1.0

```rust
const FIXED_SIZE_LIST_WILDCARD: i32 = i32::MIN
```

Source: `src/signature.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constant that is used as a placeholder for any valid fixed size list.
This is used where a function can accept a fixed size list type with any
valid length. It exists to avoid the need to enumerate all possible fixed size list lengths.
