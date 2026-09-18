# `datafusion_expr::logical_plan::statement::Statement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.Statement.json).

<a id="op-fb6e5de33854cdf98894fa4f"></a>
## Statement

`enum` · `datafusion_expr::logical_plan::statement::Statement` · datafusion-expr 55.1.0

```rust
enum Statement
```

Source: `src/logical_plan/statement.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Various types of Statements.

# Transactions:

While DataFusion does not offer support transactions, it provides
[`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) support to assist building database systems
using DataFusion

<a id="op-1ff855ad558e294518a97107"></a>
## Deallocate

`variant` · `datafusion_expr::logical_plan::statement::Statement::Deallocate` · datafusion-expr 55.1.0

```rust
Deallocate
```

Source: `src/logical_plan/statement.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deallocate a prepared statement.
This is used to implement SQL 'DEALLOCATE'.

<a id="op-168a7a001e6d0d019581e51f"></a>
## Execute

`variant` · `datafusion_expr::logical_plan::statement::Statement::Execute` · datafusion-expr 55.1.0

```rust
Execute
```

Source: `src/logical_plan/statement.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Execute a prepared statement. This is used to implement SQL 'EXECUTE'.

<a id="op-24d99356d911fcf184b8f559"></a>
## Prepare

`variant` · `datafusion_expr::logical_plan::statement::Statement::Prepare` · datafusion-expr 55.1.0

```rust
Prepare
```

Source: `src/logical_plan/statement.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Prepare a statement and find any bind parameters
(e.g. `?`). This is used to implement SQL-prepared statements.

<a id="op-afebb13a855cf98d1a6661f8"></a>
## ResetVariable

`variant` · `datafusion_expr::logical_plan::statement::Statement::ResetVariable` · datafusion-expr 55.1.0

```rust
ResetVariable
```

Source: `src/logical_plan/statement.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Reset a Variable

<a id="op-53e5be5209bf691707882e64"></a>
## SetVariable

`variant` · `datafusion_expr::logical_plan::statement::Statement::SetVariable` · datafusion-expr 55.1.0

```rust
SetVariable
```

Source: `src/logical_plan/statement.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set a Variable

<a id="op-88b7cbe56fa5d2a35855c0e8"></a>
## TransactionEnd

`variant` · `datafusion_expr::logical_plan::statement::Statement::TransactionEnd` · datafusion-expr 55.1.0

```rust
TransactionEnd
```

Source: `src/logical_plan/statement.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b66002191b8a08252141620f"></a>
## TransactionStart

`variant` · `datafusion_expr::logical_plan::statement::Statement::TransactionStart` · datafusion-expr 55.1.0

```rust
TransactionStart
```

Source: `src/logical_plan/statement.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-322e5d06d164da206641615a"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::Statement::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Statement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab21f715ee06145258898af5"></a>
## display

`function` · `datafusion_expr::logical_plan::statement::Statement::display` · datafusion-expr 55.1.0

```rust
fn display(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [146, 2], "filename": "src/logical_plan/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/statement.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `format`able structure with the a human readable
description of this LogicalPlan node per node, not including
children.

See [crate::LogicalPlan::display](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-a74af8650ebcf89cbff8609b) for an example

<a id="op-3a436b3f4d892f61419ab356"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::Statement::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Statement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 24], "end": [34, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a538ffe0f1c991399e4abee7"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::Statement::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff736f811c286f956c610418"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::Statement::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 51], "end": [34, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8fa7fd7d26c16854500beea"></a>
## name

`function` · `datafusion_expr::logical_plan::statement::Statement::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [146, 2], "filename": "src/logical_plan/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/statement.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a descriptive string describing the type of this
[`Statement`](../operations/datafusion_expr.logical_plan.statement.Statement.md#op-fb6e5de33854cdf98894fa4f)

<a id="op-88b0505013c566f5c8cbcc0f"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::Statement::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Statement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 39], "end": [34, 49], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d881594ab6a0bd810d90ed9"></a>
## schema

`function` · `datafusion_expr::logical_plan::statement::Statement::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [146, 2], "filename": "src/logical_plan/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/statement.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get a reference to the logical plan's schema
