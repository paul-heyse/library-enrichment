# `object_store::integration`

Crate `object_store` · 16 public items · structured records in [`model/object_store.integration.json`](../model/object_store.integration.json)

## copy_if_not_exists

`function` · `object_store::integration::copy_if_not_exists`

```rust
async fn copy_if_not_exists(storage: &DynObjectStore)
```

Tests copy if not exists

---

## copy_rename_nonexistent_object

`function` · `object_store::integration::copy_rename_nonexistent_object`

```rust
async fn copy_rename_nonexistent_object(storage: &DynObjectStore)
```

Tests copy and renaming behaviour of non-existent objects

---

## get_nonexistent_object

`function` · `object_store::integration::get_nonexistent_object`

```rust
async fn get_nonexistent_object(storage: &DynObjectStore, location: Option<path::Path>) -> Result<bytes::Bytes>
```

Tests fetching a non-existent object returns a not found error

---

## get_opts

`function` · `object_store::integration::get_opts`

```rust
async fn get_opts(storage: &dyn ObjectStore)
```

Tests conditional read requests

---

## list_paginated

`function` · `object_store::integration::list_paginated`

```rust
async fn list_paginated(storage: &dyn ObjectStore, list: &dyn PaginatedListStore)
```

Tests [`PaginatedListStore`]

---

## list_uses_directories_correctly

`function` · `object_store::integration::list_uses_directories_correctly`

```rust
async fn list_uses_directories_correctly(storage: &DynObjectStore)
```

Tests that directories are transparent

---

## list_with_delimiter

`function` · `object_store::integration::list_with_delimiter`

```rust
async fn list_with_delimiter(storage: &DynObjectStore)
```

Tests listing with delimiter

---

## list_with_offset_exclusivity

`function` · `object_store::integration::list_with_offset_exclusivity`

```rust
async fn list_with_offset_exclusivity(storage: &DynObjectStore)
```

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

Tests [`MultipartStore`]

---

## multipart_out_of_order

`function` · `object_store::integration::multipart_out_of_order`

```rust
async fn multipart_out_of_order(storage: &dyn ObjectStore)
```

Tests performing out of order multipart uploads

---

## multipart_race_condition

`function` · `object_store::integration::multipart_race_condition`

```rust
async fn multipart_race_condition(storage: &dyn ObjectStore, last_writer_wins: bool)
```

Tests a race condition where 2 threads are performing multipart writes to the same path

---

## put_get_attributes

`function` · `object_store::integration::put_get_attributes`

```rust
async fn put_get_attributes(integration: &dyn ObjectStore)
```

Tests the ability to read and write [`Attributes`]

---

## put_get_delete_list

`function` · `object_store::integration::put_get_delete_list`

```rust
async fn put_get_delete_list(storage: &DynObjectStore)
```

Tests basic read/write and listing operations

---

## put_opts

`function` · `object_store::integration::put_opts`

```rust
async fn put_opts(storage: &dyn ObjectStore, supports_update: bool)
```

Tests conditional writes

---

## rename_and_copy

`function` · `object_store::integration::rename_and_copy`

```rust
async fn rename_and_copy(storage: &DynObjectStore)
```

Tests copying

---

## stream_get

`function` · `object_store::integration::stream_get`

```rust
async fn stream_get(storage: &DynObjectStore)
```

Tests the ability to perform multipart writes

---
