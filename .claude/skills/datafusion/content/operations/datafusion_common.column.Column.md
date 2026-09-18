# `datafusion_common::column::Column`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.column.Column.json).

<a id="op-099cc6d1a52c20c065bf8bc6"></a>
## Column

`struct` · `datafusion_common::column::Column` · datafusion-common 55.1.0

```rust
struct Column
```

Source: `src/column.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A named reference to a qualified field in a schema.

<a id="op-83f94d3b601c6513ce4e2a97"></a>
## Err

`assoc_type` · `datafusion_common::column::Column::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [365, 1], "end": [371, 2], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/column.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-689fb35e775db83aca19ce14"></a>
## clone

`function` · `datafusion_common::column::Column::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Column
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/column.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38f7fa590073296775283107"></a>
## cmp

`function` · `datafusion_common::column::Column::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &Column) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 50], "end": [29, 53], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/column.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e75e96d5f4241caf831f3fdb"></a>
## eq

`function` · `datafusion_common::column::Column::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Column) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 26], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/column.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79c8962c817ae13c339ee71c"></a>
## flat_name

`function` · `datafusion_common::column::Column::flat_name` · datafusion-common 55.1.0

```rust
fn flat_name(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Serialize column into a flat name string

<a id="op-1954c39e562996415d981209"></a>
## fmt

`function` · `datafusion_common::column::Column::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [373, 1], "end": [377, 2], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/column.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9f246a9ab039b449dddcd7b"></a>
## fmt

`function` · `datafusion_common::column::Column::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [46, 2], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/column.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1313ffeca4e3cfcd0b6003c0"></a>
## from

`function` · `datafusion_common::column::Column::from` · datafusion-common 55.1.0

```rust
fn from((relation, field): (Option<&TableReference>, &FieldRef)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [362, 2], "filename": "src/column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/column.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24e0a1a347d033828692c6b2"></a>
## from

`function` · `datafusion_common::column::Column::from` · datafusion-common 55.1.0

```rust
fn from((relation, field): (Option<&TableReference>, &Field)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [355, 2], "filename": "src/column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/column.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56d0f1239cbd62081607db1d"></a>
## from

`function` · `datafusion_common::column::Column::from` · datafusion-common 55.1.0

```rust
fn from(c: &String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [341, 2], "filename": "src/column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/column.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-780b3babbb3a00b30a24d76d"></a>
## from

`function` · `datafusion_common::column::Column::from` · datafusion-common 55.1.0

```rust
fn from(c: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [344, 1], "end": [348, 2], "filename": "src/column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/column.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d16c740de10fd68504317d5a"></a>
## from

`function` · `datafusion_common::column::Column::from` · datafusion-common 55.1.0

```rust
fn from(c: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [334, 2], "filename": "src/column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/column.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f44f1e6312636cbb08fa0b1d"></a>
## from_name

`function` · `datafusion_common::column::Column::from_name` · datafusion-common 55.1.0

```rust
fn from_name(name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create Column from unqualified name.

Alias for `Column::new_unqualified`

<a id="op-6fd4973bc10def72257cbdde"></a>
## from_qualified_name

`function` · `datafusion_common::column::Column::from_qualified_name` · datafusion-common 55.1.0

```rust
fn from_qualified_name(flat_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Deserialize a fully qualified name string into a column

Treats the name as a SQL identifier. For example
`foo.BAR` would be parsed to a reference to relation `foo`, column name `bar` (lower case)
where `"foo.BAR"` would be parsed to a reference to column named `foo.BAR`

<a id="op-10f244b2ae62a3db384073a8"></a>
## from_qualified_name_ignore_case

`function` · `datafusion_common::column::Column::from_qualified_name_ignore_case` · datafusion-common 55.1.0

```rust
fn from_qualified_name_ignore_case(flat_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Deserialize a fully qualified name string into a column preserving column text case

<a id="op-0bb5ffa3b5a10ea7de8e9920"></a>
## from_str

`function` · `datafusion_common::column::Column::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [365, 1], "end": [371, 2], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/column.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bce0709792215542a51186b2"></a>
## hash

`function` · `datafusion_common::column::Column::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 32], "end": [29, 36], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/column.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b7ac386eb77e6dc12a644b2"></a>
## name

`struct_field` · `datafusion_common::column::Column::name` · datafusion-common 55.1.0

```rust
name: String
```

Source: `src/column.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

field/column name.

<a id="op-4a0fa9d7f1adf4f2b9a7f9f8"></a>
## name

`function` · `datafusion_common::column::Column::name` · datafusion-common 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

return the column's name.

Note: This ignores the relation and returns the column name only.

<a id="op-c83a329caa4d4974a34e443a"></a>
## new

`function` · `datafusion_common::column::Column::new` · datafusion-common 55.1.0

```rust
fn new(relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create Column from optional qualifier and name. The optional qualifier, if present,
will be parsed and normalized by default.

See full details on [`TableReference::parse_str`]

[`TableReference::parse_str`]: crate::TableReference::parse_str

<a id="op-6d9a9a44b2bec0445916d35e"></a>
## new_unqualified

`function` · `datafusion_common::column::Column::new_unqualified` · datafusion-common 55.1.0

```rust
fn new_unqualified(name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience method for when there is no qualifier

<a id="op-90d085489b7639ee10100841"></a>
## normalize_with_schemas_and_ambiguity_check

`function` · `datafusion_common::column::Column::normalize_with_schemas_and_ambiguity_check` · datafusion-common 55.1.0

```rust
fn normalize_with_schemas_and_ambiguity_check(self, schemas: &[&[&DFSchema]], using_columns: &[HashSet<Column>]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Qualify column if not done yet.

If this column already has a [relation](Self::relation), it will be returned as is and the given parameters are
ignored. Otherwise this will search through the given schemas to find the column.

Will check for ambiguity at each level of `schemas`.

A schema matches if there is a single column that -- when unqualified -- matches this column. There is an
exception for `USING` statements, see below.

# Using columns
Take the following SQL statement:

```sql
SELECT id FROM t1 JOIN t2 USING(id)
```

In this case, both `t1.id` and `t2.id` will match unqualified column `id`. To express this possibility, use
`using_columns`. Each entry in this array is a set of columns that are bound together via a `USING` clause. So
in this example this would be `[{t1.id, t2.id}]`.

Regarding ambiguity check, `schemas` is structured to allow levels of schemas to be passed in.
For example:

```text
schemas = &[
   &[schema1, schema2], // first level
   &[schema3, schema4], // second level
]
```

Will search for a matching field in all schemas in the first level. If a matching field according to above
mentioned conditions is not found, then will check the next level. If found more than one matching column across
all schemas in a level, that isn't a USING column, will return an error due to ambiguous column.

If checked all levels and couldn't find field, will return field not found error.

<a id="op-6f1d128cda6a1f0fcd52fd6e"></a>
## partial_cmp

`function` · `datafusion_common::column::Column::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Column) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 38], "end": [29, 48], "filename": "src/column.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/column.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a9728b1abfcdd56b2ae2adc"></a>
## quoted_flat_name

`function` · `datafusion_common::column::Column::quoted_flat_name` · datafusion-common 55.1.0

```rust
fn quoted_flat_name(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Serialize column into a quoted flat name string

<a id="op-c509be02ad411c9608d97a4f"></a>
## relation

`struct_field` · `datafusion_common::column::Column::relation` · datafusion-common 55.1.0

```rust
relation: Option<TableReference>
```

Source: `src/column.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

relation/table reference.

<a id="op-a514c017448ba3fc689a5aa6"></a>
## spans

`struct_field` · `datafusion_common::column::Column::spans` · datafusion-common 55.1.0

```rust
spans: Spans
```

Source: `src/column.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Original source code location, if known

<a id="op-b4512efb2c94a9d23c47878c"></a>
## spans

`function` · `datafusion_common::column::Column::spans` · datafusion-common 55.1.0

```rust
fn spans(&self) -> &Spans
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a reference to the set of locations in the SQL query where this
column appears, if known.

<a id="op-071e3c2cd474eaca6ea28fd0"></a>
## spans_mut

`function` · `datafusion_common::column::Column::spans_mut` · datafusion-common 55.1.0

```rust
fn spans_mut(&mut self) -> &mut Spans
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a mutable reference to the set of locations in the SQL query
where this column appears, if known.

<a id="op-91595dc0b1ba7f461b7425cd"></a>
## with_relation

`function` · `datafusion_common::column::Column::with_relation` · datafusion-common 55.1.0

```rust
fn with_relation(&self, relation: TableReference) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Qualifies the column with the given table reference.

<a id="op-f2d20dd83390c9180ecf1123"></a>
## with_spans

`function` · `datafusion_common::column::Column::with_spans` · datafusion-common 55.1.0

```rust
fn with_spans(self, spans: Spans) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [328, 2], "filename": "src/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/column.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Replaces the set of locations in the SQL query where this column
appears, if known.
