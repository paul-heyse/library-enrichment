# DeletionVector

`buoyant_kernel::actions::deletion_vector_writer::DeletionVector`

```rust
trait DeletionVector: Sized
```

Also reachable as `delta_kernel::actions::deletion_vector_writer::DeletionVector`

Prose: [`api/buoyant_kernel.actions.deletion_vector_writer.md`](../api/buoyant_kernel.actions.deletion_vector_writer.md#deletionvector) · records: [`model/buoyant_kernel.actions.deletion_vector_writer.json`](../model/buoyant_kernel.actions.deletion_vector_writer.json)

## Required

Every implementation must supply these.

```rust
fn cardinality(&self) -> u64
fn into_iter(self) -> Self::IndexIterator
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn serialize(self) -> DeltaResult<Bytes>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector`

## Documentation

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
