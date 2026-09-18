# `datafusion_common::spans::Location`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.spans.Location.json).

<a id="op-14296911d166f7886092e520"></a>
## Location

`struct` · `datafusion_common::spans::Location` · datafusion-common 55.1.0

```rust
struct Location
```

Source: `src/spans.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a location, determined by a line and a column number, in the
original SQL query.

<a id="op-e300d5d34d143b9590518bf5"></a>
## clone

`function` · `datafusion_common::spans::Location::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Location
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 31], "end": [24, 36], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/spans.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d925466e037136b7b0bf00d"></a>
## cmp

`function` · `datafusion_common::spans::Location::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &Location) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 44], "end": [24, 47], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/spans.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd4616dacd79d7d789ff6c21"></a>
## column

`struct_field` · `datafusion_common::spans::Location::column` · datafusion-common 55.1.0

```rust
column: u64
```

Source: `src/spans.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Line column, starting from 1.

Note: Column 0 is used for empty spans

<a id="op-797f660d7473f37803f78417"></a>
## eq

`function` · `datafusion_common::spans::Location::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Location) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 14], "end": [24, 23], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/spans.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43f70fbba0f649629f5bccbf"></a>
## fmt

`function` · `datafusion_common::spans::Location::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/spans.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a169b799b231e00f4c9339b"></a>
## from

`function` · `datafusion_common::spans::Location::from` · datafusion-common 55.1.0

```rust
fn from(value: sqlparser::tokenizer::Location) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [50, 2], "filename": "src/spans.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/spans.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d6670a6284db45932a8fb0b"></a>
## hash

`function` · `datafusion_common::spans::Location::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 25], "end": [24, 29], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/spans.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f52e05d76f80e13715e17a1a"></a>
## line

`struct_field` · `datafusion_common::spans::Location::line` · datafusion-common 55.1.0

```rust
line: u64
```

Source: `src/spans.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Line number, starting from 1.

Note: Line 0 is used for empty spans

<a id="op-575f610c5922370c496df36b"></a>
## partial_cmp

`function` · `datafusion_common::spans::Location::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Location) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 49], "end": [24, 59], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/spans.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
