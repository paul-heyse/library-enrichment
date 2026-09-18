# `sqlparser::ast::Function`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Function.json).

<a id="op-7a817c4696b83e2dbacf9d59"></a>
## Function

`struct` · `sqlparser::ast::Function` · sqlparser 0.62.0

```rust
struct Function
```

Source: `src/ast/mod.rs:8049`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A function call

<a id="op-d7be3a95737fcdb0be40cee7"></a>
## args

`struct_field` · `sqlparser::ast::Function::args` · sqlparser 0.62.0

```rust
args: FunctionArguments
```

Source: `src/ast/mod.rs:8073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The arguments to the function, including any options specified within the
delimiting parentheses.

<a id="op-13d2f64be71d7588b2d94c89"></a>
## clone

`function` · `sqlparser::ast::Function::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Function
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8046, 17], "end": [8046, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d34f44b49d231c89526a251"></a>
## cmp

`function` · `sqlparser::ast::Function::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Function) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8046, 51], "end": [8046, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c26201a4476749c54fbf61af"></a>
## deserialize

`function` · `sqlparser::ast::Function::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8047, 49], "end": [8047, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecca7bc702dddecd53b6e291"></a>
## eq

`function` · `sqlparser::ast::Function::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Function) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8046, 24], "end": [8046, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b6174c30b1527f329ef3809"></a>
## filter

`struct_field` · `sqlparser::ast::Function::filter` · sqlparser 0.62.0

```rust
filter: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:8075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

e.g. `x > 5` in `COUNT(x) FILTER (WHERE x > 5)`

<a id="op-58e6ce7d3180e05a3aff0113"></a>
## fmt

`function` · `sqlparser::ast::Function::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8097, 1], "end": [8132, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a57204c24073cf4cc4521bda"></a>
## fmt

`function` · `sqlparser::ast::Function::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8046, 10], "end": [8046, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e01c9ef4c5631167b070d13c"></a>
## hash

`function` · `sqlparser::ast::Function::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8046, 56], "end": [8046, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04e1661d56fb465c3fc1786d"></a>
## name

`struct_field` · `sqlparser::ast::Function::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:8051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function name (may be qualified).

<a id="op-074bcadd00dcad76fa2e7dc4"></a>
## null_treatment

`struct_field` · `sqlparser::ast::Function::null_treatment` · sqlparser 0.62.0

```rust
null_treatment: Option<NullTreatment>
```

Source: `src/ast/mod.rs:8084`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates how `NULL`s should be handled in the calculation.

Example:
```plaintext
FIRST_VALUE( <expr> ) [ { IGNORE | RESPECT } NULLS ] OVER ...
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/functions/first_value)

<a id="op-00b2ebc792f3bd23594f31c9"></a>
## over

`struct_field` · `sqlparser::ast::Function::over` · sqlparser 0.62.0

```rust
over: Option<WindowType>
```

Source: `src/ast/mod.rs:8086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `OVER` clause, indicating a window function call.

<a id="op-44339cc9fe48cc7bcd38a0b8"></a>
## parameters

`struct_field` · `sqlparser::ast::Function::parameters` · sqlparser 0.62.0

```rust
parameters: FunctionArguments
```

Source: `src/ast/mod.rs:8070`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The parameters to the function, including any options specified within the
delimiting parentheses.

Example:
```plaintext
HISTOGRAM(0.5, 0.6)(x, y)
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/aggregate-functions/parametric-functions)

<a id="op-23af12a616801e4bb198272d"></a>
## partial_cmp

`function` · `sqlparser::ast::Function::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Function) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8046, 35], "end": [8046, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c87ef113e15074a043d9a9d8"></a>
## serialize

`function` · `sqlparser::ast::Function::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8047, 38], "end": [8047, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d98fccaff6a806a3cdf5cfe"></a>
## span

`function` · `sqlparser::ast::Function::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "super::Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1731, 1], "end": [1754, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7919e3b136a6bcfd77a2e49"></a>
## uses_odbc_syntax

`struct_field` · `sqlparser::ast::Function::uses_odbc_syntax` · sqlparser 0.62.0

```rust
uses_odbc_syntax: bool
```

Source: `src/ast/mod.rs:8060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flags whether this function call uses the [ODBC syntax].

Example:
```sql
SELECT {fn CONCAT('foo', 'bar')}
```

[ODBC syntax]: https://learn.microsoft.com/en-us/sql/odbc/reference/develop-app/scalar-function-calls?view=sql-server-2017

<a id="op-ccac2c1217e2df633e0fd371"></a>
## visit

`function` · `sqlparser::ast::Function::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8048, 47], "end": [8048, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de9021b0fd03c609f1bbca79"></a>
## visit

`function` · `sqlparser::ast::Function::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8048, 40], "end": [8048, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7e76fd9231b443b4588a76d"></a>
## within_group

`struct_field` · `sqlparser::ast::Function::within_group` · sqlparser 0.62.0

```rust
within_group: Vec<OrderByExpr>
```

Source: `src/ast/mod.rs:8094`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A clause used with certain aggregate functions to control the ordering
within grouped sets before the function is applied.

Syntax:
```plaintext
<aggregate_function>(expression) WITHIN GROUP (ORDER BY key [ASC | DESC], ...)
```
