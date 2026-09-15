# `object_store::aws::precondition`

Crate `object_store` · 2 public items · structured records in [`model/object_store.aws.precondition.json`](../model/object_store.aws.precondition.json)

## S3ConditionalPut

`enum` · `object_store::aws::precondition::S3ConditionalPut`

Also reachable as `object_store::aws::S3ConditionalPut`

```rust
enum S3ConditionalPut
```

**Variants**: `ETagMatch`, `Disabled`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Configure how to provide conditional put support for [`AmazonS3`].

[`AmazonS3`]: super::AmazonS3

---

## S3CopyIfNotExists

`enum` · `object_store::aws::precondition::S3CopyIfNotExists`

Also reachable as `object_store::aws::S3CopyIfNotExists`

```rust
enum S3CopyIfNotExists
```

**Variants**: `Header`, `HeaderWithStatus`, `Multipart`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Configure how to provide [`CopyMode::Create`] for [`AmazonS3`].

[`CopyMode::Create`]: crate::CopyMode::Create
[`AmazonS3`]: super::AmazonS3

---
