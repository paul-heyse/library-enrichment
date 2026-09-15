# `object_store::aws::checksum`

Crate `object_store` · 1 public items · structured records in [`model/object_store.aws.checksum.json`](../model/object_store.aws.checksum.json)

## Checksum

`enum` · `object_store::aws::checksum::Checksum`

Also reachable as `object_store::aws::Checksum`

```rust
enum Checksum
```

**Variants**: `SHA256`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &String) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Enum representing checksum algorithm supported by S3.

---
