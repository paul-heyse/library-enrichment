# `datafusion_expr_common::signature::TIMEZONE_WILDCARD`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.TIMEZONE_WILDCARD.json).

<a id="op-5c6a30f50b3ea94ff4230ef5"></a>
## TIMEZONE_WILDCARD

`constant` · `datafusion_expr_common::signature::TIMEZONE_WILDCARD` · datafusion-expr-common 55.1.0

```rust
const TIMEZONE_WILDCARD: &str = "+TZ"
```

Source: `src/signature.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constant that is used as a placeholder for any valid timezone.
This is used where a function can accept a timestamp type with any
valid timezone, it exists to avoid the need to enumerate all possible
timezones. See [`TypeSignature`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff) for more details.

Type coercion always ensures that functions will be executed using
timestamp arrays that have a valid time zone. Functions must never
return results with this timezone.
