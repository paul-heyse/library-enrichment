# `datafusion_expr::logical_plan::dml::MergeIntoClauseKind`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.MergeIntoClauseKind.json).

<a id="op-40dc40de4cec9047c84c036a"></a>
## MergeIntoClauseKind

`enum` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind` · datafusion-expr 55.1.0

```rust
enum MergeIntoClauseKind
```

Source: `src/logical_plan/dml.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Which rows a MERGE WHEN clause applies to.

Mirrors `sqlparser::ast::MergeClauseKind` so that the SQL spelling is
preserved through the logical plan.

**Note on `NotMatched` vs `NotMatchedByTarget`:** these two variants are
semantically identical — both describe a source row that has no matching
target row. `NotMatched` is the SQL standard short form (used by
Snowflake, Postgres, SQL Server); `NotMatchedByTarget` is BigQuery's
explicit form added for symmetry with `NotMatchedBySource`. Downstream
consumers (planners, table providers, optimizers) MUST treat the two
variants identically.

<a id="op-af6444bf78d0ea1ac1c6b0bd"></a>
## Matched

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::Matched` · datafusion-expr 55.1.0

```rust
Matched
```

Source: `src/logical_plan/dml.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`WHEN MATCHED`

<a id="op-bd8ac5f857c4f9b71168514c"></a>
## NotMatched

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::NotMatched` · datafusion-expr 55.1.0

```rust
NotMatched
```

Source: `src/logical_plan/dml.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`WHEN NOT MATCHED` — see type-level note for the equivalence with
[`NotMatchedByTarget`](Self::NotMatchedByTarget).

<a id="op-690584fb0e6685e501d0555d"></a>
## NotMatchedBySource

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::NotMatchedBySource` · datafusion-expr 55.1.0

```rust
NotMatchedBySource
```

Source: `src/logical_plan/dml.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`WHEN NOT MATCHED BY SOURCE`

<a id="op-cfe5d7f4b68c6dc2b38530af"></a>
## NotMatchedByTarget

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::NotMatchedByTarget` · datafusion-expr 55.1.0

```rust
NotMatchedByTarget
```

Source: `src/logical_plan/dml.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`WHEN NOT MATCHED BY TARGET` — see type-level note for the
equivalence with [`NotMatched`](Self::NotMatched).

<a id="op-599a47876345e57670e7a12b"></a>
## canonical

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::canonical` · datafusion-expr 55.1.0

```rust
fn canonical(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [447, 1], "end": [474, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Collapse the SQL-spelling variants into the canonical three semantic
categories: [`Matched`](Self::Matched),
[`NotMatchedByTarget`](Self::NotMatchedByTarget) (covering both
"NOT MATCHED" spellings), and
[`NotMatchedBySource`](Self::NotMatchedBySource).

Use this in downstream `match` expressions when the SQL spelling
distinction does not matter — e.g. in planners, optimizers, or
table-provider dispatch.

<a id="op-8467efaf1c5eb773ac57e8cb"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> MergeIntoClauseKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 17], "end": [433, 22], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e637e7c8c2b85e8425219e8b"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &MergeIntoClauseKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 30], "end": [433, 39], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0a748172f8af010928ace63"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 10], "end": [433, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d3eac81542e5d83475b0d1d"></a>
## from

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::from` · datafusion-expr 55.1.0

```rust
fn from(kind: protobuf::merge_into_clause_node::Kind) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "crate::dml::MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [162, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::merge_into_clause_node::Kind", "path": "Kind"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/proto.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e27e11ba760577c2a3a1db5"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 57], "end": [433, 61], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbb54af1b69445f4468578c0"></a>
## is_not_matched_by_target

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::is_not_matched_by_target` · datafusion-expr 55.1.0

```rust
fn is_not_matched_by_target(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [447, 1], "end": [474, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if this clause fires on a source row that has no matching target
row. Returns `true` for both [`NotMatched`](Self::NotMatched) and
[`NotMatchedByTarget`](Self::NotMatchedByTarget) (see the type-level
note explaining why those two variants are semantically identical).

Prefer this predicate over hand-written `matches!` arms so the
`NotMatched`/`NotMatchedByTarget` equivalence is enforced in one place.

<a id="op-64719ee09dbfcd3627387f9d"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &MergeIntoClauseKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClauseKind", "path": "MergeIntoClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 45], "end": [433, 55], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
