# `datafusion_expr::logical_plan::dml::MergeIntoClause`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.MergeIntoClause.json).

<a id="op-02464bca4a3d11cba00886d2"></a>
## MergeIntoClause

`struct` · `datafusion_expr::logical_plan::dml::MergeIntoClause` · datafusion-expr 55.1.0

```rust
struct MergeIntoClause
```

Source: `src/logical_plan/dml.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A single WHEN clause within a MERGE INTO statement.

<a id="op-ac096f10a5168c7c29b68f45"></a>
## action

`struct_field` · `datafusion_expr::logical_plan::dml::MergeIntoClause::action` · datafusion-expr 55.1.0

```rust
action: MergeIntoAction
```

Source: `src/logical_plan/dml.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The action to take.

<a id="op-239097d67f6e24dea2cab7e6"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClause::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> MergeIntoClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClause", "path": "MergeIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 17], "end": [411, 22], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86c108dc0267c4bd30e5af96"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClause::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &MergeIntoClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClause", "path": "MergeIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 24], "end": [411, 33], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0b0175e02d22b0435e08986"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClause::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClause", "path": "MergeIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 10], "end": [411, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2170ba5385e19befa6dd61e9"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClause::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClause", "path": "MergeIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 51], "end": [411, 55], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3108738472ed8fd002704da"></a>
## kind

`struct_field` · `datafusion_expr::logical_plan::dml::MergeIntoClause::kind` · datafusion-expr 55.1.0

```rust
kind: MergeIntoClauseKind
```

Source: `src/logical_plan/dml.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether this fires on matched or unmatched rows.

<a id="op-986239fc41ba74402c984240"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::MergeIntoClause::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &MergeIntoClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoClause", "path": "MergeIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 39], "end": [411, 49], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c44995d8090697288da0098f"></a>
## predicate

`struct_field` · `datafusion_expr::logical_plan::dml::MergeIntoClause::predicate` · datafusion-expr 55.1.0

```rust
predicate: Option<Expr>
```

Source: `src/logical_plan/dml.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional additional predicate (`AND <expr>`).
