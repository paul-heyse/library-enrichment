# `datafusion_expr::preimage::PreimageResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.preimage.PreimageResult.json).

<a id="op-3bcf3c562bde118bfbfc632f"></a>
## PreimageResult

`enum` · `datafusion_expr::preimage::PreimageResult` · datafusion-expr 55.1.0

```rust
enum PreimageResult
```

Source: `src/preimage.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return from [`crate::ScalarUDFImpl::preimage`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-0f3b7cc7d7d1203c14fd2c9f)

<a id="op-4ac493d0288db7aae2c1fcf9"></a>
## None

`variant` · `datafusion_expr::preimage::PreimageResult::None` · datafusion-expr 55.1.0

```rust
None
```

Source: `src/preimage.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No preimage exists for the specified value

<a id="op-15c06605de855e48d55fd709"></a>
## Range

`variant` · `datafusion_expr::preimage::PreimageResult::Range` · datafusion-expr 55.1.0

```rust
Range
```

Source: `src/preimage.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression always evaluates to the specified constant
given that `expr` is within the interval
