# `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector_writer.StreamingDeletionVectorWriter.json).

<a id="op-d2fbca68a34f237da6ee56cd"></a>
## StreamingDeletionVectorWriter

`struct` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StreamingDeletionVectorWriter<'a, W: Write>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L209).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:209`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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

<a id="op-8bf7c71ce0ca670f4a172edd"></a>
## finalize

`function` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter::finalize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finalize(self) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L360).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter", "path": "StreamingDeletionVectorWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [371, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:360`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Finalize all writes and flush the underlying writer.

This method should be called after all deletion vectors have been written.
After calling this method, the writer should not be used anymore.

# Errors

Returns an error if flushing the writer fails.

# Examples

```rust,ignore
writer.write_deletion_vector(dv1)?;
writer.write_deletion_vector(dv2)?;
writer.finalize()?;
# Ok::<(), delta_kernel::Error>(())
```

<a id="op-5278d7c7c4bad9a2495ebc72"></a>
## new

`function` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(writer: &'a mut W) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L227).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter", "path": "StreamingDeletionVectorWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [371, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:227`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new streaming deletion vector writer.

# Arguments

* `writer` - A mutable reference to any type implementing [`std::io::Write`].

# Examples

```rust,ignore
let mut buffer = Vec::new();
let writer = StreamingDeletionVectorWriter::new(&mut buffer);
```

Unresolved upstream links (retained, not inferred): ``std::io::Write``.

<a id="op-775402ac476e0ff484689204"></a>
## write_deletion_vector

`function` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter::write_deletion_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_deletion_vector(&mut self, deletion_vector: impl DeletionVector) -> DeltaResult<DeletionVectorWriteResult>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L266).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter", "path": "StreamingDeletionVectorWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [371, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:266`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write a deletion vector to the underlying writer.

This method can be called multiple times to write multiple deletion vectors to the same
writer. The caller is responsible for keeping track of which deletion vector corresponds to
which data file.

# Arguments

* `deletion_vector` - The deletion vector to write

# Returns

A [`DeletionVectorWriteResult`](../operations/buoyant_kernel.actions.deletion_vector_writer.DeletionVectorWriteResult.md#op-2c478f1fbf9c8c54eabc09d8) containing the offset, size, and cardinality
of the written deletion vector.

# Errors

Returns an error if:
- The writer fails to write data
- The deletion vector cannot be serialized
- The offset or size would overflow an i32

# Examples

```rust,ignore
let mut dv = KernelDeletionVector::new();
dv.add_deleted_row_indexes([1, 5, 10]);

let descriptor = writer.write_deletion_vector(dv)?;
println!("Written DV at offset {} with size {}", descriptor.offset, descriptor.size_in_bytes);
# Ok::<(), delta_kernel::Error>(())
```

<a id="op-ad6ea419edb3cad8e12727ed"></a>
## current_offset

`struct_field` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter::current_offset` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
current_offset: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L211).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:211`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fb57025252f80239e8f457a"></a>
## writer

`struct_field` · `buoyant_kernel::actions::deletion_vector_writer::StreamingDeletionVectorWriter::writer` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer: &'a mut W
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L210).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:210`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
