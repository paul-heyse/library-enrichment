# `buoyant_kernel::actions::deletion_vector_writer`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.actions.deletion_vector_writer.json`](../model/buoyant_kernel.actions.deletion_vector_writer.json)

## DeletionVectorWriteResult

`struct` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult`

Also reachable as `delta_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult`

```rust
struct DeletionVectorWriteResult
```

**Fields**: `offset`, `size_in_bytes`, `cardinality`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn to_descriptor(self, path: &DeletionVectorPath) -> DeletionVectorDescriptor
```

Metadata about a written deletion vector, excluding the storage path.

This structure contains the information needed to construct a full
[`DeletionVectorDescriptor`]
after writing the DV to storage.

---

## KernelDeletionVector

`struct` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector`

Also reachable as `delta_kernel::actions::deletion_vector_writer::KernelDeletionVector`

```rust
struct KernelDeletionVector
```

**Implements**: `buoyant_kernel::actions::deletion_vector_writer::DeletionVector`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn add_deleted_row_indexes<I, T>(&mut self, iter: I) where I: IntoIterator<Item = T>, T: Borrow<u64>
fn cardinality(&self) -> u64
fn new() -> Self
```

**via `buoyant_kernel::actions::deletion_vector_writer::DeletionVector`**

```rust
fn cardinality(&self) -> u64
fn into_iter(self) -> Self::IndexIterator
fn serialize(self) -> DeltaResult<Bytes>
```

A Kernel-provided deletion vector implementation backed by [`RoaringTreemap`].

This is the default implementation that engines can use. It provides memory-efficient
storage of deleted row indexes using compressed bitmaps.

# Examples

```rust
# use buoyant_kernel as delta_kernel;
use delta_kernel::actions::deletion_vector_writer::KernelDeletionVector;

let mut dv = KernelDeletionVector::new();
dv.add_deleted_row_indexes([0, 5, 10]);
```

---

## StreamingDeletionVectorWriter

`struct` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter`

Also reachable as `delta_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter`

```rust
struct StreamingDeletionVectorWriter<'a, W: Write>
```

**Methods** (3)

```rust
fn finalize(self) -> DeltaResult<()>
fn new(writer: &'a mut W) -> Self
fn write_deletion_vector(&mut self, deletion_vector: impl DeletionVector) -> DeltaResult<DeletionVectorWriteResult>
```

A streaming writer for deletion vectors.

This writer allows for writing multiple deletion vectors to a single file in a streaming
fashion, which is memory-efficient for distributed workloads where deletion vectors are
generated on executors.

# Format

The writer produces deletion vector files in the Delta Lake format:
- The first byte of the file is a version byte (currently 1)
- Each DV is prefixed with a 4-byte size (big-endian) of the serialized data
- Followed by a 4-byte magic number (0x6439d3d1, little-endian)
- Followed by the serialized 64-bit Roaring Bitmap
- Followed by a 4-byte CRC32 checksum (big-endian) of the serialized data

# Examples

```rust
# use buoyant_kernel as delta_kernel;
use delta_kernel::actions::deletion_vector_writer::{StreamingDeletionVectorWriter, KernelDeletionVector};

let mut buffer = Vec::new();
let mut writer = StreamingDeletionVectorWriter::new(&mut buffer);

let mut dv = KernelDeletionVector::new();
dv.add_deleted_row_indexes([1, 5, 10]);

let descriptor = writer.write_deletion_vector(dv)?;
writer.finalize()?;
# Ok::<(), delta_kernel::Error>(())
```

---

## DeletionVector

`trait` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVector`

Also reachable as `delta_kernel::actions::deletion_vector_writer::DeletionVector`

```rust
trait DeletionVector: Sized
```

**Implementors** (1)

- `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector`

**Methods** (3)

```rust
fn cardinality(&self) -> u64
fn into_iter(self) -> Self::IndexIterator
fn serialize(self) -> DeltaResult<Bytes>
```

A trait that allows engines to provide deletion vectors in various formats.

Engines can implement this trait to provide their own deletion vector implementations,
or use the provided [`KernelDeletionVector`] implementation backed by RoaringTreemap.

# Examples

```rust
# use buoyant_kernel as delta_kernel;
use delta_kernel::actions::deletion_vector_writer::DeletionVector;

struct MyDeletionVector {
    deleted_indexes: Vec<u64>,
}

impl DeletionVector for MyDeletionVector {
    type IndexIterator = std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IndexIterator {
        self.deleted_indexes.into_iter()
    }

    fn cardinality(&self) -> u64 {
        self.deleted_indexes.len() as u64
    }
}
```

---
