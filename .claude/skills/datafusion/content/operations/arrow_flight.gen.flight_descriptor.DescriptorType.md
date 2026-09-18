# `arrow_flight::gen::flight_descriptor::DescriptorType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.flight_descriptor.DescriptorType.json).

<a id="op-a7bb77542acff3d10643266b"></a>
## DescriptorType

`enum` · `arrow_flight::gen::flight_descriptor::DescriptorType` · arrow-flight 59.3.0

```rust
enum DescriptorType
```

Source: `src/arrow.flight.protocol.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Describes what type of descriptor is defined.

<a id="op-5acf296c5cdbf97fae9d09ec"></a>
## Cmd

`variant` · `arrow_flight::gen::flight_descriptor::DescriptorType::Cmd` · arrow-flight 59.3.0

```rust
Cmd
```

Source: `src/arrow.flight.protocol.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


An opaque command to generate a dataset.

<a id="op-6d6622f62e349fe700676093"></a>
## Error

`assoc_type` · `arrow_flight::gen::flight_descriptor::DescriptorType::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 9], "end": [142, 29], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow.flight.protocol.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05fc7ea169d75d5bb03d02f0"></a>
## Path

`variant` · `arrow_flight::gen::flight_descriptor::DescriptorType::Path` · arrow-flight 59.3.0

```rust
Path
```

Source: `src/arrow.flight.protocol.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A named path that identifies a dataset. A path is composed of a string
or list of strings describing a particular dataset. This is conceptually
  similar to a path inside a filesystem.

<a id="op-ddb6b51e7f2b5679e077b9d0"></a>
## Unknown

`variant` · `arrow_flight::gen::flight_descriptor::DescriptorType::Unknown` · arrow-flight 59.3.0

```rust
Unknown
```

Source: `src/arrow.flight.protocol.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Protobuf pattern, not used.

<a id="op-c11224aea5c373f67f058c83"></a>
## as_str_name

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 5], "end": [178, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-30502917da76bb145d532afc"></a>
## clone

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> DescriptorType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 9], "end": [134, 14], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:134`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbb50a37138c565a7cd611e3"></a>
## cmp

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &DescriptorType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 9], "end": [141, 12], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/arrow.flight.protocol.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acc2afc703cbfa903c5ed6eb"></a>
## default

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::default` · arrow-flight 59.3.0

```rust
fn default() -> DescriptorType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 9], "end": [142, 29], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13cbb20c9b9a2697454edf67"></a>
## eq

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &DescriptorType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 9], "end": [137, 18], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-633094d8664b36ba8c2b25c5"></a>
## fmt

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 9], "end": [136, 14], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9b26888d409f5efbe27419d"></a>
## from_i32

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<DescriptorType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 9], "end": [142, 29], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `DescriptorType`, or `None` if `value` is not a valid variant.

<a id="op-a52715cb711af70c6408771f"></a>
## from_str_name

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 5], "end": [178, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:170`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-26c5eb39cf59c80886a247a3"></a>
## hash

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 9], "end": [139, 13], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f065c6b846108b5d90edc6c"></a>
## is_valid

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 9], "end": [142, 29], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `DescriptorType`.

<a id="op-61c368a1e1b05c3bf0afeed1"></a>
## partial_cmp

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &DescriptorType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 9], "end": [140, 19], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/arrow.flight.protocol.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c09b48a703ef3e10a4fd1504"></a>
## try_from

`function` · `arrow_flight::gen::flight_descriptor::DescriptorType::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<DescriptorType, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::flight_descriptor::DescriptorType", "path": "DescriptorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 9], "end": [142, 29], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow.flight.protocol.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
