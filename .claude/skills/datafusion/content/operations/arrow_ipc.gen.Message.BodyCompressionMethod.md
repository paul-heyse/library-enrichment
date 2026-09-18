# `arrow_ipc::gen::Message::BodyCompressionMethod`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.BodyCompressionMethod.json).

<a id="op-f113110f2fc843b11c9b06f2"></a>
## BodyCompressionMethod

`struct` · `arrow_ipc::gen::Message::BodyCompressionMethod` · arrow-ipc 59.3.0

```rust
struct BodyCompressionMethod
```

Source: `src/gen/Message.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Provided for forward compatibility in case we need to support different
strategies for compressing the IPC message body (like whole-body
compression rather than buffer-level) in the future

<a id="op-25e4ff58ad5880d803138b81"></a>
## 0

`struct_field` · `arrow_ipc::gen::Message::BodyCompressionMethod::0` · arrow-ipc 59.3.0

```rust
0: i8
```

Source: `src/gen/Message.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79473aa4076dc392eff6698d"></a>
## BUFFER

`assoc_const` · `arrow_ipc::gen::Message::BodyCompressionMethod::BUFFER` · arrow-ipc 59.3.0

```rust
BUFFER
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [167, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Each constituent buffer is first compressed with the indicated
compressor, and then written with the uncompressed length in the first 8
bytes as a 64-bit little-endian signed integer followed by the compressed
buffer bytes (and then padding as required by the protocol). The
uncompressed length may be set to -1 to indicate that the data that
follows is not compressed, which can be useful for cases where
compression does not yield appreciable savings.

<a id="op-43bb34a19d3d520658683f9e"></a>
## ENUM_MAX

`assoc_const` · `arrow_ipc::gen::Message::BodyCompressionMethod::ENUM_MAX` · arrow-ipc 59.3.0

```rust
ENUM_MAX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [167, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:158`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2be648c1011de64caf6d805"></a>
## ENUM_MIN

`assoc_const` · `arrow_ipc::gen::Message::BodyCompressionMethod::ENUM_MIN` · arrow-ipc 59.3.0

```rust
ENUM_MIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [167, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9485f9c4bc946a9dbca708e0"></a>
## ENUM_VALUES

`assoc_const` · `arrow_ipc::gen::Message::BodyCompressionMethod::ENUM_VALUES` · arrow-ipc 59.3.0

```rust
ENUM_VALUES
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [167, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00effd00b462e8a9b389ed61"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Message::BodyCompressionMethod::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [184, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:178`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83a6cf8c2d0cba17a197f926"></a>
## Output

`assoc_type` · `arrow_ipc::gen::Message::BodyCompressionMethod::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [194, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4242aff546d4cba623fe712"></a>
## Scalar

`assoc_type` · `arrow_ipc::gen::Message::BodyCompressionMethod::Scalar` · arrow-ipc 59.3.0

```rust
Scalar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [208, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Message.rs:197`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c156765f8f2ece751ad69b31"></a>
## clone

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> BodyCompressionMethod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 10], "end": [143, 15], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Message.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5f62cbd608251f40568e92d"></a>
## cmp

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::cmp` · arrow-ipc 59.3.0

```rust
fn cmp(&self, other: &BodyCompressionMethod) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 50], "end": [143, 53], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/gen/Message.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c449d9bc178bbd12f90b6e4c"></a>
## default

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::default` · arrow-ipc 59.3.0

```rust
fn default() -> BodyCompressionMethod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 61], "end": [143, 68], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Message.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acc03900aef68f9aa9f4f3f3"></a>
## eq

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &BodyCompressionMethod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 23], "end": [143, 32], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Message.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a43c699d77f149d9d50031ba"></a>
## fmt

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [176, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Message.rs:169`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5540f0d519722d751d8f3e36"></a>
## follow

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [184, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ab27089220998718082745d"></a>
## from_little_endian

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::from_little_endian` · arrow-ipc 59.3.0

```rust
fn from_little_endian(v: i8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [208, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Message.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b43153af89ae6b04ae5470cc"></a>
## hash

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::hash` · arrow-ipc 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 55], "end": [143, 59], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/gen/Message.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95853b1069117667cce6b211"></a>
## partial_cmp

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::partial_cmp` · arrow-ipc 59.3.0

```rust
fn partial_cmp(&self, other: &BodyCompressionMethod) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 38], "end": [143, 48], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/gen/Message.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1e8f92d80d6f0f853cf9d1c"></a>
## push

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [194, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfe8f554775a62c7ec0ca661"></a>
## run_verifier

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [219, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Message.rs:212`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30f4fc9dd110ff19ac99305c"></a>
## to_little_endian

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::to_little_endian` · arrow-ipc 59.3.0

```rust
fn to_little_endian(self) -> i8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [208, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Message.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04b26246e7975dff6a878bf4"></a>
## variant_name

`function` · `arrow_ipc::gen::Message::BodyCompressionMethod::variant_name` · arrow-ipc 59.3.0

```rust
fn variant_name(self) -> Option<&'static str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::BodyCompressionMethod", "path": "BodyCompressionMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [167, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Returns the variant's name or "" if unknown.
