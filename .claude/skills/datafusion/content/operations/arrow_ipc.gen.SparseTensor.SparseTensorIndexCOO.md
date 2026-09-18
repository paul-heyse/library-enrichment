# `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.SparseTensorIndexCOO.json).

<a id="op-a575f676a2cd247579c0f5eb"></a>
## SparseTensorIndexCOO

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO` · arrow-ipc 59.3.0

```rust
struct SparseTensorIndexCOO<'a>
```

Source: `src/gen/SparseTensor.rs:265`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

----------------------------------------------------------------------
EXPERIMENTAL: Data structures for sparse tensors
Coordinate (COO) format of sparse tensor index.

COO's index list are represented as a NxM matrix,
where N is the number of non-zero values,
and M is the number of dimensions of a sparse tensor.

indicesBuffer stores the location and size of the data of this indices
matrix.  The value type and the stride of the indices matrix is
specified in indicesType and indicesStrides fields.

For example, let X be a 2x3x4x5 tensor, and it has the following
6 non-zero values:
```text
  X[0, 1, 2, 0] := 1
  X[1, 1, 2, 3] := 2
  X[0, 2, 1, 0] := 3
  X[0, 1, 3, 0] := 4
  X[0, 1, 2, 1] := 5
  X[1, 2, 0, 4] := 6
```
In COO format, the index matrix of X is the following 4x6 matrix:
```text
  [[0, 0, 0, 0, 1, 1],
   [1, 1, 1, 2, 1, 2],
   [2, 2, 3, 1, 2, 0],
   [0, 1, 0, 0, 3, 4]]
```
When isCanonical is true, the indices is sorted in lexicographical order
(row-major order), and it does not have duplicated entries.  Otherwise,
the indices may not be sorted, or may have duplicated entries.

<a id="op-68f4a04d8358268c3804403f"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [277, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/SparseTensor.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea52debde43e682c2f34e969"></a>
## VT_INDICESBUFFER

`assoc_const` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::VT_INDICESBUFFER` · arrow-ipc 59.3.0

```rust
VT_INDICESBUFFER
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20f793972c0cdd11ed466157"></a>
## VT_INDICESSTRIDES

`assoc_const` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::VT_INDICESSTRIDES` · arrow-ipc 59.3.0

```rust
VT_INDICESSTRIDES
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d39e94c4ff10741759dfac1"></a>
## VT_INDICESTYPE

`assoc_const` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::VT_INDICESTYPE` · arrow-ipc 59.3.0

```rust
VT_INDICESTYPE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a547ffc15d9ba82ce7dcf2ba"></a>
## VT_ISCANONICAL

`assoc_const` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::VT_ISCANONICAL` · arrow-ipc 59.3.0

```rust
VT_ISCANONICAL
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6963a37f8a29572d24b5e5f"></a>
## _tab

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/SparseTensor.rs:266`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a48e28dfdecf8969702770d"></a>
## clone

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> SparseTensorIndexCOO<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 16], "end": [231, 21], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/SparseTensor.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86aed6cad292a997c2e64ecc"></a>
## create

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args SparseTensorIndexCOOArgs<'args>) -> flatbuffers::WIPOffset<SparseTensorIndexCOO<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03bf8835b9057fbe38e4417a"></a>
## eq

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &SparseTensorIndexCOO<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 23], "end": [231, 32], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/SparseTensor.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1985dfcafedb72c9c37e98"></a>
## fmt

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [472, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/SparseTensor.rs:464`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55ffe367366dbbc5a8b90596"></a>
## follow

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [277, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/SparseTensor.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ccf985e9205756c5e55b9ab"></a>
## indicesBuffer

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::indicesBuffer` · arrow-ipc 59.3.0

```rust
fn indicesBuffer(&self) -> &'a Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:340`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The location and size of the indices matrix's data

<a id="op-4b657a367f35e9d1ed5107d1"></a>
## indicesStrides

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::indicesStrides` · arrow-ipc 59.3.0

```rust
fn indicesStrides(&self) -> Option<flatbuffers::Vector<'a, i64>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:326`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Non-negative byte offsets to advance one value cell along each dimension
If omitted, default to row-major order (C-like).

<a id="op-c3a7797b807473c81f8fb2b8"></a>
## indicesType

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::indicesType` · arrow-ipc 59.3.0

```rust
fn indicesType(&self) -> Int<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The type of values in indicesBuffer

<a id="op-5a02097fdb0439dcc1b001b0"></a>
## init_from_table

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9e120001f4e1585dd0ecfc1"></a>
## isCanonical

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::isCanonical` · arrow-ipc 59.3.0

```rust
fn isCanonical(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [366, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

This flag is true if and only if the indices matrix is sorted in
row-major order, and does not have duplicated entries.
This sort order is the same as of Tensorflow's SparseTensor,
but it is inverse order of SciPy's canonical coo_matrix
(SciPy employs column-major order for its coo_matrix).

<a id="op-ac2c2cb35a5688915c6fe582"></a>
## run_verifier

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO", "path": "SparseTensorIndexCOO"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 1], "end": [391, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/SparseTensor.rs:370`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
