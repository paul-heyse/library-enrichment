# `datafusion_expr::expr::WildcardOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.WildcardOptions.json).

<a id="op-6b0bc2b091308f3f94b87432"></a>
## WildcardOptions

`struct` · `datafusion_expr::expr::WildcardOptions` · datafusion-expr 55.1.0

```rust
struct WildcardOptions
```

Source: `src/expr.rs:1499`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Additional options for wildcards, e.g. Snowflake `EXCLUDE`/`RENAME` and Bigquery `EXCEPT`.

<a id="op-d32510dc3707fa7ca9481de0"></a>
## clone

`function` · `datafusion_expr::expr::WildcardOptions::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WildcardOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 10], "end": [1498, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ef2da0926a38d1b1cd49670"></a>
## default

`function` · `datafusion_expr::expr::WildcardOptions::default` · datafusion-expr 55.1.0

```rust
fn default() -> WildcardOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 57], "end": [1498, 64], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/expr.rs:1498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-638587afa5a3086463a60e97"></a>
## eq

`function` · `datafusion_expr::expr::WildcardOptions::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WildcardOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 17], "end": [1498, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df8c4087d1315a1747da9ab8"></a>
## except

`struct_field` · `datafusion_expr::expr::WildcardOptions::except` · datafusion-expr 55.1.0

```rust
except: Option<ExceptSelectItem>
```

Source: `src/expr.rs:1509`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`[EXCEPT...]`.
 BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_except>
 Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#except>

<a id="op-fea11b5331cb1e9ac8709ed2"></a>
## exclude

`struct_field` · `datafusion_expr::expr::WildcardOptions::exclude` · datafusion-expr 55.1.0

```rust
exclude: Option<ExcludeSelectItem>
```

Source: `src/expr.rs:1505`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`[EXCLUDE...]`.
 Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-7c7ca7c916b10e78db203caf"></a>
## fmt

`function` · `datafusion_expr::expr::WildcardOptions::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1532, 1], "end": [1551, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:1533`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5eba5376b9bd13298ea370d"></a>
## fmt

`function` · `datafusion_expr::expr::WildcardOptions::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 50], "end": [1498, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44de618ecebd5ea164bb6a7f"></a>
## hash

`function` · `datafusion_expr::expr::WildcardOptions::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 44], "end": [1498, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a28352c07c33bde91c2fa633"></a>
## ilike

`struct_field` · `datafusion_expr::expr::WildcardOptions::ilike` · datafusion-expr 55.1.0

```rust
ilike: Option<IlikeSelectItem>
```

Source: `src/expr.rs:1502`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`[ILIKE...]`.
 Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-a9f989dfb897ce4b68a502c2"></a>
## partial_cmp

`function` · `datafusion_expr::expr::WildcardOptions::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WildcardOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 32], "end": [1498, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9135b1b4f22547925ef269c"></a>
## rename

`struct_field` · `datafusion_expr::expr::WildcardOptions::rename` · datafusion-expr 55.1.0

```rust
rename: Option<RenameSelectItem>
```

Source: `src/expr.rs:1517`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`[RENAME ...]`.
 Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-9ef7417b0192c7a576c16c0b"></a>
## replace

`struct_field` · `datafusion_expr::expr::WildcardOptions::replace` · datafusion-expr 55.1.0

```rust
replace: Option<PlannedReplaceSelectItem>
```

Source: `src/expr.rs:1514`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`[REPLACE]`
 BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_replace>
 Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#replace>
 Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-a702f220762d828dce37fe29"></a>
## with_replace

`function` · `datafusion_expr::expr::WildcardOptions::with_replace` · datafusion-expr 55.1.0

```rust
fn with_replace(self, replace: PlannedReplaceSelectItem) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WildcardOptions", "path": "WildcardOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1520, 1], "end": [1530, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1521`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
