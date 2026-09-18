# `object_store::integration`

Crate `object_store` · 16 public items · structured records in [`model/object_store.integration.json`](../model/object_store.integration.json)

## copy_if_not_exists

`function` · `object_store::integration::copy_if_not_exists`

```rust
async fn copy_if_not_exists(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.copy_if_not_exists.md).


Tests copy if not exists

---

## copy_rename_nonexistent_object

`function` · `object_store::integration::copy_rename_nonexistent_object`

```rust
async fn copy_rename_nonexistent_object(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.copy_rename_nonexistent_object.md).


Tests copy and renaming behaviour of non-existent objects

---

## get_nonexistent_object

`function` · `object_store::integration::get_nonexistent_object`

```rust
async fn get_nonexistent_object(storage: &DynObjectStore, location: Option<path::Path>) -> Result<bytes::Bytes>
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.get_nonexistent_object.md).


Tests fetching a non-existent object returns a not found error

---

## get_opts

`function` · `object_store::integration::get_opts`

```rust
async fn get_opts(storage: &dyn ObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.get_opts.md).


Tests conditional read requests

---

## list_paginated

`function` · `object_store::integration::list_paginated`

```rust
async fn list_paginated(storage: &dyn ObjectStore, list: &dyn PaginatedListStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.list_paginated.md).


Tests [`PaginatedListStore`]

---

## list_uses_directories_correctly

`function` · `object_store::integration::list_uses_directories_correctly`

```rust
async fn list_uses_directories_correctly(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.list_uses_directories_correctly.md).


Tests that directories are transparent

---

## list_with_delimiter

`function` · `object_store::integration::list_with_delimiter`

```rust
async fn list_with_delimiter(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.list_with_delimiter.md).


Tests listing with delimiter

---

## list_with_offset_exclusivity

`function` · `object_store::integration::list_with_offset_exclusivity`

```rust
async fn list_with_offset_exclusivity(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.list_with_offset_exclusivity.md).


Tests that [`ObjectStore::list_with_offset`] returns an exclusive list
that does not include the offset value itself.
This is needed because some object stores (i.e. Azure) return inclusive results,
while AWS S3 and GCP return exclusive results.

---

## multipart

`function` · `object_store::integration::multipart`

```rust
async fn multipart(storage: &dyn ObjectStore, multipart: &dyn MultipartStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.multipart.md).


Tests [`MultipartStore`]

---

## multipart_out_of_order

`function` · `object_store::integration::multipart_out_of_order`

```rust
async fn multipart_out_of_order(storage: &dyn ObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.multipart_out_of_order.md).


Tests performing out of order multipart uploads

---

## multipart_race_condition

`function` · `object_store::integration::multipart_race_condition`

```rust
async fn multipart_race_condition(storage: &dyn ObjectStore, last_writer_wins: bool)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.multipart_race_condition.md).


Tests a race condition where 2 threads are performing multipart writes to the same path

---

## put_get_attributes

`function` · `object_store::integration::put_get_attributes`

```rust
async fn put_get_attributes(integration: &dyn ObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.put_get_attributes.md).


Tests the ability to read and write [`Attributes`]

---

## put_get_delete_list

`function` · `object_store::integration::put_get_delete_list`

```rust
async fn put_get_delete_list(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.put_get_delete_list.md).


Tests basic read/write and listing operations

---

## put_opts

`function` · `object_store::integration::put_opts`

```rust
async fn put_opts(storage: &dyn ObjectStore, supports_update: bool)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.put_opts.md).


Tests conditional writes

---

## rename_and_copy

`function` · `object_store::integration::rename_and_copy`

```rust
async fn rename_and_copy(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.rename_and_copy.md).


Tests copying

---

## stream_get

`function` · `object_store::integration::stream_get`

```rust
async fn stream_get(storage: &DynObjectStore)
```

[Full member, field, variant and typed contracts](../operations/object_store.integration.stream_get.md).


Tests the ability to perform multipart writes

---
