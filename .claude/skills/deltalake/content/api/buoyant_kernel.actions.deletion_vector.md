# `buoyant_kernel::actions::deletion_vector`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.actions.deletion_vector.json`](../model/buoyant_kernel.actions.deletion_vector.json)

## DeletionVectorStorageType

`enum` · `buoyant_kernel::actions::deletion_vector::DeletionVectorStorageType`

Also reachable as `delta_kernel::actions::deletion_vector::DeletionVectorStorageType`

```rust
enum DeletionVectorStorageType
```

**Variants**: `PersistedRelative`, `Inline`, `PersistedAbsolute`

**Implements**: `buoyant_kernel::schema::derive_macro_utils::ToDataType`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::derive_macro_utils::ToDataType`**

```rust
fn to_data_type() -> DataType
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

---

## split_vector

`function` · `buoyant_kernel::actions::deletion_vector::split_vector`

Also reachable as `delta_kernel::actions::deletion_vector::split_vector`

```rust
fn split_vector(vector: Option<&mut Vec<bool>>, split_index: usize, extend: Option<bool>) -> Option<Vec<bool>>
```

helper function to split an `Option<Vec<bool>>`. Because deletion vectors apply to a whole file,
but parquet readers can chunk the file, there is a need to split the vector up.
If the passed vector is Some(vector):
  - If `split_index < vector.len()`, split `vector` at `split_index`. The passed vector is
    modified in place, and the split off component is returned.
  - If `split_index` >= vector.len()` will return None. If `extend` is Some(b), the passed
    vector will be extended with `b` to have a length of `split_index`. If `extend` is `None`,
    do nothing and return `None`
If the passed `vector` is `None`, do nothing and return None

---

## DeletionVectorDescriptor

`struct` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor`

Also reachable as `delta_kernel::actions::deletion_vector::DeletionVectorDescriptor`

```rust
struct DeletionVectorDescriptor
```

**Fields**: `storage_type`, `path_or_inline_dv`, `offset`, `size_in_bytes`, `cardinality`

**Implements**: `buoyant_kernel::schema::ToSchema`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn absolute_path(&self, parent: &Url) -> DeltaResult<Option<Url>>
fn read(&self, storage: Arc<dyn StorageHandler>, parent: &Url) -> DeltaResult<RoaringTreemap>
fn row_indexes(&self, storage: Arc<dyn StorageHandler>, parent: &Url) -> DeltaResult<Vec<u64>>
fn try_new(storage_type: DeletionVectorStorageType, path_or_inline_dv: impl Into<String>, offset: Option<i32>, size_in_bytes: i32, cardinality: i64) -> DeltaResult<Self>
fn unique_id(&self) -> String
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

---

## DeletionVectorPath

`struct` · `buoyant_kernel::actions::deletion_vector::DeletionVectorPath`

Also reachable as `delta_kernel::actions::deletion_vector::DeletionVectorPath`

```rust
struct DeletionVectorPath
```

**Methods** (1)

```rust
fn absolute_path(&self) -> DeltaResult<Url>
```

Represents an abstract path to a deletion vector file.

This is used in the public API to construct the path to a deletion vector file and
has logic to convert [`crate::actions::deletion_vector_writer::DeletionVectorWriteResult`]
to a [`DeletionVectorDescriptor`] with appropriate storage type and path.

---
