# `datafusion_proto::bytes::Serializeable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.Serializeable.json).

<a id="op-abef1839438df69a1a70cafa"></a>
## Serializeable

`trait` · `datafusion_proto::bytes::Serializeable` · datafusion-proto 55.1.0

```rust
trait Serializeable: Sized
```

Source: `src/bytes/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Encodes something (such as [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)) to/from a stream of
bytes.

```
use datafusion_expr::{col, lit, Expr};
use datafusion_proto::bytes::Serializeable;

// Create a new `Expr` a < 32
let expr = col("a").lt(lit(5i32));

// Convert it to an opaque form
let bytes = expr.to_bytes().unwrap();

// Decode bytes from somewhere (over network, etc.)
let decoded_expr = Expr::from_bytes(&bytes).unwrap();
assert_eq!(expr, decoded_expr);
```

<a id="op-aee9c9830edbff35103031b3"></a>
## from_bytes

`function` · `datafusion_proto::bytes::Serializeable::from_bytes` · datafusion-proto 55.1.0

```rust
fn from_bytes(bytes: &[u8]) -> Result<Self>
```

Source: `src/bytes/mod.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Convert `bytes` (the output of [`to_bytes`]) back into an object. This
will error if the serialized bytes contain any user defined functions,
in which case use [`from_bytes_with_ctx`]

[`to_bytes`]: Self::to_bytes
[`from_bytes_with_ctx`]: Self::from_bytes_with_ctx

<a id="op-9f2d6053d5c0f8711ad6c9bb"></a>
## from_bytes_with_ctx

`function` · `datafusion_proto::bytes::Serializeable::from_bytes_with_ctx` · datafusion-proto 55.1.0

```rust
fn from_bytes_with_ctx(bytes: &[u8], ctx: &TaskContext) -> Result<Self>
```

Source: `src/bytes/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Convert `bytes` (the output of [`to_bytes`]) back into an object,
resolving user defined functions with the specified `ctx`

[`to_bytes`]: Self::to_bytes

<a id="op-14032caadc8bc1885f1ead53"></a>
## to_bytes

`function` · `datafusion_proto::bytes::Serializeable::to_bytes` · datafusion-proto 55.1.0

```rust
fn to_bytes(&self) -> Result<Bytes>
```

Source: `src/bytes/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Convert `self` to an opaque byte stream
