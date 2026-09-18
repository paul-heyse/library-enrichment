# `datafusion_expr_common::statistics::GenericDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.GenericDistribution.json).

<a id="op-a36921bb4183dbc4abf89add"></a>
## GenericDistribution

`struct` · `datafusion_expr_common::statistics::GenericDistribution` · datafusion-expr-common 55.1.0

```rust
struct GenericDistribution
```

Source: `src/statistics.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A generic distribution whose functional form is not available, which is
approximated via some summary statistics. For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Summary_statistics>

<a id="op-acca41bba6c718d316276f60"></a>
## clone

`function` · `datafusion_expr_common::statistics::GenericDistribution::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> GenericDistribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 10], "end": [305, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f4f9784110654455555b5f4"></a>
## data_type

`function` · `datafusion_expr_common::statistics::GenericDistribution::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [626, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a04f1c1d9cde463343de8529"></a>
## eq

`function` · `datafusion_expr_common::statistics::GenericDistribution::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &GenericDistribution) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 24], "end": [305, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09455bc490820eba5b148c5f"></a>
## fmt

`function` · `datafusion_expr_common::statistics::GenericDistribution::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 17], "end": [305, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-beefd20469e5a1c48a836085"></a>
## mean

`function` · `datafusion_expr_common::statistics::GenericDistribution::mean` · datafusion-expr-common 55.1.0

```rust
fn mean(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [626, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:611`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cedb82eab7e1c2887341a9b"></a>
## median

`function` · `datafusion_expr_common::statistics::GenericDistribution::median` · datafusion-expr-common 55.1.0

```rust
fn median(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [626, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:615`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6fd3d9fdeec78bda71c2877"></a>
## range

`function` · `datafusion_expr_common::statistics::GenericDistribution::range` · datafusion-expr-common 55.1.0

```rust
fn range(&self) -> &Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [626, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:623`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b59881f217ca02e77525725"></a>
## variance

`function` · `datafusion_expr_common::statistics::GenericDistribution::variance` · datafusion-expr-common 55.1.0

```rust
fn variance(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [626, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:619`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
