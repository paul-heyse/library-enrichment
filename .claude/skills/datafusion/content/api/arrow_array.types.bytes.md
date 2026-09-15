# `arrow_array::types::bytes`

Crate `arrow-array` · 2 public items · structured records in [`model/arrow_array.types.bytes.json`](../model/arrow_array.types.bytes.json)

## ByteArrayNativeType

`trait` · `arrow_array::types::bytes::ByteArrayNativeType`

```rust
trait ByteArrayNativeType: std::fmt::Debug + Send + Sync
```

**Methods** (2)

```rust
fn from_bytes_checked(b: &[u8]) -> Option<&Self>
unsafe fn from_bytes_unchecked(b: &[u8]) -> &Self
```

---

## ByteArrayTypeSealed

`trait` · `arrow_array::types::bytes::ByteArrayTypeSealed`

```rust
trait ByteArrayTypeSealed
```

---
