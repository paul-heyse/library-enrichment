# `datafusion_spark::function::hash::expr_fn`

Crate `datafusion-spark` · 4 public items · structured records in [`model/datafusion_spark.function.hash.expr_fn.json`](../model/datafusion_spark.function.hash.expr_fn.json)

## crc32

`function` · `datafusion_spark::function::hash::expr_fn::crc32`

Also reachable as `datafusion_spark::expr_fn::crc32`

```rust
fn crc32(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.hash.expr_fn.crc32.md).


crc32(expr) - Returns a cyclic redundancy check value of the expr as a bigint.

---

## sha1

`function` · `datafusion_spark::function::hash::expr_fn::sha1`

Also reachable as `datafusion_spark::expr_fn::sha1`

```rust
fn sha1(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.hash.expr_fn.sha1.md).


sha1(expr) - Returns a SHA-1 hash value of the expr as a hex string.

---

## sha2

`function` · `datafusion_spark::function::hash::expr_fn::sha2`

Also reachable as `datafusion_spark::expr_fn::sha2`

```rust
fn sha2(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.hash.expr_fn.sha2.md).


sha2(expr, bitLength) - Returns a checksum of SHA-2 family as a hex string of expr. SHA-224, SHA-256, SHA-384, and SHA-512 are supported. Bit length of 0 is equivalent to 256.

---

## xxhash64

`function` · `datafusion_spark::function::hash::expr_fn::xxhash64`

Also reachable as `datafusion_spark::expr_fn::xxhash64`

```rust
fn xxhash64(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.hash.expr_fn.xxhash64.md).


xxhash64(expr1, expr2, ...) - Returns a 64-bit hash value of the arguments using xxHash.

---
