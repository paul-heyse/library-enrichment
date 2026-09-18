# `object_store::tags`

Crate `object_store` · 1 public items · structured records in [`model/object_store.tags.json`](../model/object_store.tags.json)

## TagSet

`struct` · `object_store::tags::TagSet`

Also reachable as `datafusion::object_store::TagSet`, `object_store::TagSet`

```rust
struct TagSet
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn encoded(&self) -> &str
fn push(&mut self, key: &str, value: &str)
```

[Full member, field, variant and typed contracts](../operations/object_store.tags.TagSet.md).


A collection of key value pairs used to annotate objects

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-tagging.html>
<https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tags>

---
