# `datafusion_expr::select_expr::SelectExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.select_expr.SelectExpr.json).

<a id="op-6ce0960e86cb5a2473109641"></a>
## SelectExpr

`enum` · `datafusion_expr::select_expr::SelectExpr` · datafusion-expr 55.1.0

```rust
enum SelectExpr
```

Source: `src/select_expr.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents a SELECT expression in a SQL query.

`SelectExpr` supports three types of expressions commonly found in the SELECT clause:

* Wildcard (`*`) - Selects all columns
* Qualified wildcard (`table.*`) - Selects all columns from a specific table
* Regular expression - Any other expression like columns, functions, literals etc.

This enum is typically used when you need to handle wildcards. After expanding `*` in the query,
you can use `Expr` for all other expressions.

# Examples

```
use datafusion_expr::col;
use datafusion_expr::expr::WildcardOptions;
use datafusion_expr::select_expr::SelectExpr;

// SELECT *
let wildcard = SelectExpr::Wildcard(WildcardOptions::default());

// SELECT mytable.*
let qualified =
    SelectExpr::QualifiedWildcard("mytable".into(), WildcardOptions::default());

// SELECT col1
let expr = SelectExpr::Expression(col("col1").into());
```

<a id="op-ef68d22430e61714523042ea"></a>
## Expression

`variant` · `datafusion_expr::select_expr::SelectExpr::Expression` · datafusion-expr 55.1.0

```rust
Expression
```

Source: `src/select_expr.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents any other valid SELECT expression like column references,
function calls, literals, etc.

<a id="op-750a7ee311604269943983f9"></a>
## QualifiedWildcard

`variant` · `datafusion_expr::select_expr::SelectExpr::QualifiedWildcard` · datafusion-expr 55.1.0

```rust
QualifiedWildcard
```

Source: `src/select_expr.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents a qualified wildcard (`table.*`) that selects all columns from a specific table.
The `TableReference` specifies the table and `WildcardOptions` control additional behavior.

<a id="op-56fc832061ec26de1cf133b2"></a>
## Wildcard

`variant` · `datafusion_expr::select_expr::SelectExpr::Wildcard` · datafusion-expr 55.1.0

```rust
Wildcard
```

Source: `src/select_expr.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents a wildcard (`*`) that selects all columns from all tables.
The `WildcardOptions` control additional behavior like exclusions.

<a id="op-256a2b1deccaf780c75307e7"></a>
## clone

`function` · `datafusion_expr::select_expr::SelectExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SelectExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::select_expr::SelectExpr", "path": "SelectExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 10], "end": [53, 15], "filename": "src/select_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/select_expr.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb24484ae4cf48c2bd59ae42"></a>
## fmt

`function` · `datafusion_expr::select_expr::SelectExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::select_expr::SelectExpr", "path": "SelectExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [76, 2], "filename": "src/select_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/select_expr.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2f216dd14cb3600739f210d"></a>
## fmt

`function` · `datafusion_expr::select_expr::SelectExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::select_expr::SelectExpr", "path": "SelectExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 17], "end": [53, 22], "filename": "src/select_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/select_expr.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1371ac70478864445268a29e"></a>
## from

`function` · `datafusion_expr::select_expr::SelectExpr::from` · datafusion-expr 55.1.0

```rust
fn from(expr: Expr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::select_expr::SelectExpr", "path": "SelectExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [82, 2], "filename": "src/select_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/select_expr.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-823f5113f61d455998447a95"></a>
## from

`function` · `datafusion_expr::select_expr::SelectExpr::from` · datafusion-expr 55.1.0

```rust
fn from(value: (Option<&'a TableReference>, &'a FieldRef)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::select_expr::SelectExpr", "path": "SelectExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [99, 2], "filename": "src/select_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/select_expr.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f119d5e079d729af78c46d29"></a>
## from

`function` · `datafusion_expr::select_expr::SelectExpr::from` · datafusion-expr 55.1.0

```rust
fn from(value: Column) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::select_expr::SelectExpr", "path": "SelectExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [89, 2], "filename": "src/select_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/select_expr.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
