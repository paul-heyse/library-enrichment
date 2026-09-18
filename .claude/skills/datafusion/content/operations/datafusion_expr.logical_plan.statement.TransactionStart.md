# `datafusion_expr::logical_plan::statement::TransactionStart`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.TransactionStart.json).

<a id="op-4b402d0c4394e77d1e6ac430"></a>
## TransactionStart

`struct` · `datafusion_expr::logical_plan::statement::TransactionStart` · datafusion-expr 55.1.0

```rust
struct TransactionStart
```

Source: `src/logical_plan/statement.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicator that the following statements should be committed or rolled back atomically

<a id="op-ec600c0513e91ce02dec352e"></a>
## access_mode

`struct_field` · `datafusion_expr::logical_plan::statement::TransactionStart::access_mode` · datafusion-expr 55.1.0

```rust
access_mode: TransactionAccessMode
```

Source: `src/logical_plan/statement.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

indicates if transaction is allowed to write

<a id="op-aabf03bc656a27975857663e"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::TransactionStart::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TransactionStart
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionStart", "path": "TransactionStart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 17], "end": [173, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c05a4d34038edbdd12150eb6"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::TransactionStart::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TransactionStart) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionStart", "path": "TransactionStart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 24], "end": [173, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e6791fb2caf57fb7d400a80"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::TransactionStart::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionStart", "path": "TransactionStart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 10], "end": [173, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69640f13e10b146dafc023f6"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::TransactionStart::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionStart", "path": "TransactionStart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 51], "end": [173, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cb17f0d480d98f3aea15eda"></a>
## isolation_level

`struct_field` · `datafusion_expr::logical_plan::statement::TransactionStart::isolation_level` · datafusion-expr 55.1.0

```rust
isolation_level: TransactionIsolationLevel
```

Source: `src/logical_plan/statement.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4353a2c0ba16fdd44cc997d"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::TransactionStart::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &TransactionStart) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionStart", "path": "TransactionStart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 35], "end": [173, 45], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
