# `arrow_ipc::gen::Schema::DictionaryKind`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.DictionaryKind.json).

<a id="op-50cc7e4fff55513f9b613f86"></a>
## DictionaryKind

`struct` · `arrow_ipc::gen::Schema::DictionaryKind` · arrow-ipc 59.3.0

```rust
struct DictionaryKind
```

Source: `src/gen/Schema.rs:970`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

----------------------------------------------------------------------
Dictionary encoding metadata
Maintained for forwards compatibility, in the future
Dictionaries might be explicit maps between integers and values
allowing for non-contiguous index values

<a id="op-db7d8b0441161ebf00a5f9ae"></a>
## 0

`struct_field` · `arrow_ipc::gen::Schema::DictionaryKind::0` · arrow-ipc 59.3.0

```rust
0: i16
```

Source: `src/gen/Schema.rs:970`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51a1e18e8bcd46d5c579e45b"></a>
## DenseArray

`assoc_const` · `arrow_ipc::gen::Schema::DictionaryKind::DenseArray` · arrow-ipc 59.3.0

```rust
DenseArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [985, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:973`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29e538b46d272368f65279a1"></a>
## ENUM_MAX

`assoc_const` · `arrow_ipc::gen::Schema::DictionaryKind::ENUM_MAX` · arrow-ipc 59.3.0

```rust
ENUM_MAX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [985, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:976`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dc2dfe4cafd94913ec57e14"></a>
## ENUM_MIN

`assoc_const` · `arrow_ipc::gen::Schema::DictionaryKind::ENUM_MIN` · arrow-ipc 59.3.0

```rust
ENUM_MIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [985, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:975`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-508f29d775db19ef3e9c5c76"></a>
## ENUM_VALUES

`assoc_const` · `arrow_ipc::gen::Schema::DictionaryKind::ENUM_VALUES` · arrow-ipc 59.3.0

```rust
ENUM_VALUES
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [985, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:977`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b918d9f1f523fc797fae2f38"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::DictionaryKind::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1002, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:996`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed106f33738616778f984404"></a>
## Output

`assoc_type` · `arrow_ipc::gen::Schema::DictionaryKind::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1004, 1], "end": [1012, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:1005`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea25d645c7cce5d1b9e8b0bf"></a>
## Scalar

`assoc_type` · `arrow_ipc::gen::Schema::DictionaryKind::Scalar` · arrow-ipc 59.3.0

```rust
Scalar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 1], "end": [1026, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Schema.rs:1015`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9611b7a67c88c075c298b130"></a>
## clone

`function` · `arrow_ipc::gen::Schema::DictionaryKind::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> DictionaryKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 10], "end": [968, 15], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-257f9294b108bd16aa4eb4ee"></a>
## cmp

`function` · `arrow_ipc::gen::Schema::DictionaryKind::cmp` · arrow-ipc 59.3.0

```rust
fn cmp(&self, other: &DictionaryKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 50], "end": [968, 53], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/gen/Schema.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5136899bfc9e0f4364c63a0"></a>
## default

`function` · `arrow_ipc::gen::Schema::DictionaryKind::default` · arrow-ipc 59.3.0

```rust
fn default() -> DictionaryKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 61], "end": [968, 68], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Schema.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbf160f141a406aa5bae6d76"></a>
## eq

`function` · `arrow_ipc::gen::Schema::DictionaryKind::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &DictionaryKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 23], "end": [968, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-810fac0b34b61cbf842efa0a"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::DictionaryKind::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [986, 1], "end": [994, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:987`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64572b6a90e3c7aff678bd2a"></a>
## follow

`function` · `arrow_ipc::gen::Schema::DictionaryKind::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [995, 1], "end": [1002, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:998`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ba8c523ab9034509a9f419a"></a>
## from_little_endian

`function` · `arrow_ipc::gen::Schema::DictionaryKind::from_little_endian` · arrow-ipc 59.3.0

```rust
fn from_little_endian(v: i16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 1], "end": [1026, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Schema.rs:1022`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b664253f1a24d37bb0b1201b"></a>
## hash

`function` · `arrow_ipc::gen::Schema::DictionaryKind::hash` · arrow-ipc 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 55], "end": [968, 59], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/gen/Schema.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5440bb1944433d2c781b045a"></a>
## partial_cmp

`function` · `arrow_ipc::gen::Schema::DictionaryKind::partial_cmp` · arrow-ipc 59.3.0

```rust
fn partial_cmp(&self, other: &DictionaryKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 38], "end": [968, 48], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/gen/Schema.rs:968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cc00b8057a91001ac5c2800"></a>
## push

`function` · `arrow_ipc::gen::Schema::DictionaryKind::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1004, 1], "end": [1012, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:1007`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcdefe377650a00e97bb16cc"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::DictionaryKind::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1028, 1], "end": [1037, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:1030`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f2ef758069ed45f63069ea0"></a>
## to_little_endian

`function` · `arrow_ipc::gen::Schema::DictionaryKind::to_little_endian` · arrow-ipc 59.3.0

```rust
fn to_little_endian(self) -> i16
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 1], "end": [1026, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Schema.rs:1017`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcc146795b45df3d9a6c0eca"></a>
## variant_name

`function` · `arrow_ipc::gen::Schema::DictionaryKind::variant_name` · arrow-ipc 59.3.0

```rust
fn variant_name(self) -> Option<&'static str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::DictionaryKind", "path": "DictionaryKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [985, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:979`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Returns the variant's name or "" if unknown.
