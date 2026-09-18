# `datafusion_common::unnest::RecursionUnnestOption`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.unnest.RecursionUnnestOption.json).

<a id="op-664111a0cce383aba6113e94"></a>
## RecursionUnnestOption

`struct` · `datafusion_common::unnest::RecursionUnnestOption` · datafusion-common 55.1.0

```rust
struct RecursionUnnestOption
```

Source: `src/unnest.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Instruction on how to unnest a column (mostly with a list type)
such as how to name the output, and how many level it should be unnested

<a id="op-c39b0875fbe1f9bb3bbf7919"></a>
## clone

`function` · `datafusion_common::unnest::RecursionUnnestOption::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> RecursionUnnestOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::RecursionUnnestOption", "path": "RecursionUnnestOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 17], "end": [113, 22], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unnest.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e421059b31336644f463cd25"></a>
## depth

`struct_field` · `datafusion_common::unnest::RecursionUnnestOption::depth` · datafusion-common 55.1.0

```rust
depth: usize
```

Source: `src/unnest.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5086d82e93534575638fd28"></a>
## eq

`function` · `datafusion_common::unnest::RecursionUnnestOption::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &RecursionUnnestOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::RecursionUnnestOption", "path": "RecursionUnnestOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 24], "end": [113, 33], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unnest.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86764e3d116940381ae482ec"></a>
## fmt

`function` · `datafusion_common::unnest::RecursionUnnestOption::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::RecursionUnnestOption", "path": "RecursionUnnestOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 10], "end": [113, 15], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unnest.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a872fd898baef4cf11c6911"></a>
## hash

`function` · `datafusion_common::unnest::RecursionUnnestOption::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::RecursionUnnestOption", "path": "RecursionUnnestOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 39], "end": [113, 43], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/unnest.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-167d032e5917a647e357a0d7"></a>
## input_column

`struct_field` · `datafusion_common::unnest::RecursionUnnestOption::input_column` · datafusion-common 55.1.0

```rust
input_column: Column
```

Source: `src/unnest.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab71f0d5e0c45eddbf0dcb57"></a>
## output_column

`struct_field` · `datafusion_common::unnest::RecursionUnnestOption::output_column` · datafusion-common 55.1.0

```rust
output_column: Column
```

Source: `src/unnest.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23ac6302b48d4d1dd54418ac"></a>
## partial_cmp

`function` · `datafusion_common::unnest::RecursionUnnestOption::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &RecursionUnnestOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::RecursionUnnestOption", "path": "RecursionUnnestOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 45], "end": [113, 55], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/unnest.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
