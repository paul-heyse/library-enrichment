# `datafusion_substrait::serializer`

Crate `datafusion-substrait` · 4 public items · structured records in [`model/datafusion_substrait.serializer.json`](../model/datafusion_substrait.serializer.json)

## deserialize

`function` · `datafusion_substrait::serializer::deserialize`

```rust
async fn deserialize(path: impl AsRef<std::path::Path>) -> datafusion::error::Result<Box<substrait::proto::Plan>>
```

Reads the file at `path` and deserializes a plan from the bytes.

---

## deserialize_bytes

`function` · `datafusion_substrait::serializer::deserialize_bytes`

```rust
fn deserialize_bytes(proto_bytes: &[u8]) -> datafusion::error::Result<Box<substrait::proto::Plan>>
```

Deserializes a plan from the bytes.

---

## serialize

`function` · `datafusion_substrait::serializer::serialize`

```rust
async fn serialize(sql: &str, ctx: &SessionContext, path: impl AsRef<std::path::Path>) -> datafusion::error::Result<()>
```

Plans a sql and serializes the generated logical plan to bytes.
The bytes are then written into a file at `path`.

Returns an error if the file already exists.

---

## serialize_bytes

`function` · `datafusion_substrait::serializer::serialize_bytes`

```rust
async fn serialize_bytes(sql: &str, ctx: &SessionContext) -> datafusion::error::Result<Vec<u8>>
```

Plans a sql and serializes the generated logical plan to bytes.

---
