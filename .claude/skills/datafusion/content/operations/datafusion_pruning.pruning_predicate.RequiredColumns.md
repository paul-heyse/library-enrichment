# `datafusion_pruning::pruning_predicate::RequiredColumns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.pruning_predicate.RequiredColumns.json).

<a id="op-5597779f5108f33de2116787"></a>
## RequiredColumns

`struct` · `datafusion_pruning::pruning_predicate::RequiredColumns` · datafusion-pruning 55.1.0

```rust
struct RequiredColumns
```

Source: `src/pruning_predicate.rs:818`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Describes which columns statistics are necessary to evaluate a
[`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068).

This structure permits reading and creating the minimum number statistics,
which is important since statistics may be non trivial to read (e.g. large
strings or when there are 1000s of columns).

Handles creating references to the min/max statistics
for columns as well as recording which statistics are needed

<a id="op-aaa70ac4403246331a276cf7"></a>
## clone

`function` · `datafusion_pruning::pruning_predicate::RequiredColumns::clone` · datafusion-pruning 55.1.0

```rust
fn clone(&self) -> RequiredColumns
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::RequiredColumns", "path": "RequiredColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [817, 26], "end": [817, 31], "filename": "src/pruning_predicate.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/pruning_predicate.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-462f39dd6e41fb92976a765c"></a>
## default

`function` · `datafusion_pruning::pruning_predicate::RequiredColumns::default` · datafusion-pruning 55.1.0

```rust
fn default() -> RequiredColumns
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::RequiredColumns", "path": "RequiredColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [817, 17], "end": [817, 24], "filename": "src/pruning_predicate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/pruning_predicate.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f827881906dcdb3398eacf73"></a>
## fmt

`function` · `datafusion_pruning::pruning_predicate::RequiredColumns::fmt` · datafusion-pruning 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::RequiredColumns", "path": "RequiredColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [817, 10], "end": [817, 15], "filename": "src/pruning_predicate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/pruning_predicate.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9db4501ddf19a864c058ed9e"></a>
## from

`function` · `datafusion_pruning::pruning_predicate::RequiredColumns::from` · datafusion-pruning 55.1.0

```rust
fn from(columns: Vec<(phys_expr::Column, StatisticsType, Field)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::RequiredColumns", "path": "RequiredColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [981, 1], "end": [985, 2], "filename": "src/pruning_predicate.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, {"resolved_path": {"args": null, "id": "unresolved", "path": "StatisticsType"}}, {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/pruning_predicate.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0606152c7f329da3b15c3616"></a>
## single_column

`function` · `datafusion_pruning::pruning_predicate::RequiredColumns::single_column` · datafusion-pruning 55.1.0

```rust
fn single_column(&self) -> Option<&phys_expr::Column>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::RequiredColumns", "path": "RequiredColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 1], "end": [979, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Returns Some(column) if this is a single column predicate.

Returns None if this is a multi-column predicate.

Examples:
* `a > 5 OR a < 10` returns `Some(a)`
* `a > 5 OR b < 10` returns `None`
* `true` returns None
