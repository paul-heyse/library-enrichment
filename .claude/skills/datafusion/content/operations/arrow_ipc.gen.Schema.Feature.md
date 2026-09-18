# `arrow_ipc::gen::Schema::Feature`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Feature.json).

<a id="op-8dfee1d79919b32e9a457167"></a>
## Feature

`struct` · `arrow_ipc::gen::Schema::Feature` · arrow-ipc 59.3.0

```rust
struct Feature
```

Source: `src/gen/Schema.rs:181`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Represents Arrow Features that might not have full support
within implementations. This is intended to be used in
two scenarios:
 1.  A mechanism for readers of Arrow Streams
     and files to understand that the stream or file makes
     use of a feature that isn't supported or unknown to
     the implementation (and therefore can meet the Arrow
     forward compatibility guarantees).
 2.  A means of negotiating between a client and server
     what features a stream is allowed to use. The enums
     values here are intented to represent higher level
     features, additional details maybe negotiated
     with key-value pairs specific to the protocol.

Enums added to this list should be assigned power-of-two values
to facilitate exchanging and comparing bitmaps for supported
features.

<a id="op-63a3d2746adee96bc00359b0"></a>
## 0

`struct_field` · `arrow_ipc::gen::Schema::Feature::0` · arrow-ipc 59.3.0

```rust
0: i64
```

Source: `src/gen/Schema.rs:181`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb7f4178d0af46998233af00"></a>
## COMPRESSED_BODY

`assoc_const` · `arrow_ipc::gen::Schema::Feature::COMPRESSED_BODY` · arrow-ipc 59.3.0

```rust
COMPRESSED_BODY
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The stream makes use of compressed bodies as described
in Message.fbs.

<a id="op-c2bf864d92aaee6f3912a487"></a>
## DICTIONARY_REPLACEMENT

`assoc_const` · `arrow_ipc::gen::Schema::Feature::DICTIONARY_REPLACEMENT` · arrow-ipc 59.3.0

```rust
DICTIONARY_REPLACEMENT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The stream makes use of multiple full dictionaries with the
same ID and assumes clients implement dictionary replacement
correctly.

<a id="op-c0760e96c47c65603b7de3ce"></a>
## ENUM_MAX

`assoc_const` · `arrow_ipc::gen::Schema::Feature::ENUM_MAX` · arrow-ipc 59.3.0

```rust
ENUM_MAX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:195`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41b8e244573c453bcada953e"></a>
## ENUM_MIN

`assoc_const` · `arrow_ipc::gen::Schema::Feature::ENUM_MIN` · arrow-ipc 59.3.0

```rust
ENUM_MIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33ed897fba695223c0ba2722"></a>
## ENUM_VALUES

`assoc_const` · `arrow_ipc::gen::Schema::Feature::ENUM_VALUES` · arrow-ipc 59.3.0

```rust
ENUM_VALUES
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c16f7ab6564e78e424fbc3a"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::Feature::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [220, 1], "end": [227, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:221`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-573a8146348fbc71f6d00b8f"></a>
## Output

`assoc_type` · `arrow_ipc::gen::Schema::Feature::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [237, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51472a03ca0d6877cd070027"></a>
## Scalar

`assoc_type` · `arrow_ipc::gen::Schema::Feature::Scalar` · arrow-ipc 59.3.0

```rust
Scalar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [251, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Schema.rs:240`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b0a2a94076fb90308e71ec0"></a>
## UNUSED

`assoc_const` · `arrow_ipc::gen::Schema::Feature::UNUSED` · arrow-ipc 59.3.0

```rust
UNUSED
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Needed to make flatbuffers happy.

<a id="op-019e0294c0ea1c44837d5699"></a>
## clone

`function` · `arrow_ipc::gen::Schema::Feature::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Feature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 10], "end": [179, 15], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bf93c97f1f488a122f1840b"></a>
## cmp

`function` · `arrow_ipc::gen::Schema::Feature::cmp` · arrow-ipc 59.3.0

```rust
fn cmp(&self, other: &Feature) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 50], "end": [179, 53], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/gen/Schema.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6411ea6d7705e1907e9b8ac"></a>
## default

`function` · `arrow_ipc::gen::Schema::Feature::default` · arrow-ipc 59.3.0

```rust
fn default() -> Feature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 61], "end": [179, 68], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Schema.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1cf389e014b9787c2c6fac3"></a>
## eq

`function` · `arrow_ipc::gen::Schema::Feature::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Feature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 23], "end": [179, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cae3039ad51faef0c400cf91"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::Feature::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [219, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:212`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2094aa414eb43d4c3ac94b9b"></a>
## follow

`function` · `arrow_ipc::gen::Schema::Feature::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [220, 1], "end": [227, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77b0fd9f81865cd20371ce76"></a>
## from_little_endian

`function` · `arrow_ipc::gen::Schema::Feature::from_little_endian` · arrow-ipc 59.3.0

```rust
fn from_little_endian(v: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [251, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Schema.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1616e8555025d74e70154f97"></a>
## hash

`function` · `arrow_ipc::gen::Schema::Feature::hash` · arrow-ipc 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 55], "end": [179, 59], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/gen/Schema.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ecf78febcf59af74b8cc48d"></a>
## partial_cmp

`function` · `arrow_ipc::gen::Schema::Feature::partial_cmp` · arrow-ipc 59.3.0

```rust
fn partial_cmp(&self, other: &Feature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 38], "end": [179, 48], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/gen/Schema.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afb742ff72617e08e27d09e1"></a>
## push

`function` · `arrow_ipc::gen::Schema::Feature::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [237, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:232`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a0794e161e76f721ee7c8da"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::Feature::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [262, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb8d5880e46427f12a45a3e4"></a>
## to_little_endian

`function` · `arrow_ipc::gen::Schema::Feature::to_little_endian` · arrow-ipc 59.3.0

```rust
fn to_little_endian(self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [251, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Schema.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2945df32201b896e324d3033"></a>
## variant_name

`function` · `arrow_ipc::gen::Schema::Feature::variant_name` · arrow-ipc 59.3.0

```rust
fn variant_name(self) -> Option<&'static str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Feature", "path": "Feature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [210, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:202`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Returns the variant's name or "" if unknown.
