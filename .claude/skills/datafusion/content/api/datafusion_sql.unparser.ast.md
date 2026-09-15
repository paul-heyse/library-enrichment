# `datafusion_sql::unparser::ast`

Crate `datafusion-sql` · 10 public items · structured records in [`model/datafusion_sql.unparser.ast.json`](../model/datafusion_sql.unparser.ast.json)

## BuilderError

`enum` · `datafusion_sql::unparser::ast::BuilderError`

```rust
enum BuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: String) -> Self
fn from(s: UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## DerivedRelationBuilder

`struct` · `datafusion_sql::unparser::ast::DerivedRelationBuilder`

```rust
struct DerivedRelationBuilder
```

**Derives**: Clone, Default

**Methods** (3)

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
fn lateral(&mut self, value: bool) -> &mut Self
fn subquery(&mut self, value: Box<ast::Query>) -> &mut Self
```

---

## FlattenRelationBuilder

`struct` · `datafusion_sql::unparser::ast::FlattenRelationBuilder`

```rust
struct FlattenRelationBuilder
```

**Fields**: `alias`, `input_expr`, `outer`

**Derives**: Clone, Default

**Methods** (4)

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
fn build(&self) -> Result<ast::TableFactor, BuilderError>
fn input_expr(&mut self, value: ast::Expr) -> &mut Self
fn outer(&mut self, value: bool) -> &mut Self
```

Builds a `LATERAL FLATTEN(INPUT => expr, OUTER => bool)` table factor
for Snowflake-style unnesting.

---

## QueryBuilder

`struct` · `datafusion_sql::unparser::ast::QueryBuilder`

```rust
struct QueryBuilder
```

**Derives**: Clone, Default

**Methods** (13)

```rust
fn body(&mut self, value: Box<ast::SetExpr>) -> &mut Self
fn build(&self) -> Result<ast::Query, BuilderError>
fn distinct_union(&mut self) -> &mut Self
fn fetch(&mut self, value: Option<ast::Fetch>) -> &mut Self
fn for_clause(&mut self, value: Option<ast::ForClause>) -> &mut Self
fn is_distinct_union(&self) -> bool
fn limit(&mut self, value: Option<ast::Expr>) -> &mut Self
fn limit_by(&mut self, value: Vec<ast::Expr>) -> &mut Self
fn locks(&mut self, value: Vec<ast::LockClause>) -> &mut Self
fn offset(&mut self, value: Option<ast::Offset>) -> &mut Self
fn order_by(&mut self, value: OrderByKind) -> &mut Self
fn take_body(&mut self) -> Option<Box<ast::SetExpr>>
fn with(&mut self, value: Option<ast::With>) -> &mut Self
```

---

## RelationBuilder

`struct` · `datafusion_sql::unparser::ast::RelationBuilder`

```rust
struct RelationBuilder
```

**Derives**: Clone, Default

**Methods** (9)

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
fn build(&self) -> Result<Option<ast::TableFactor>, BuilderError>
fn derived(&mut self, value: DerivedRelationBuilder) -> &mut Self
fn empty(&mut self) -> &mut Self
fn flatten(&mut self, value: FlattenRelationBuilder) -> &mut Self
fn has_relation(&self) -> bool
fn nested_join(&mut self, value: ast::TableWithJoins, alias: Option<ast::TableAlias>) -> &mut Self
fn table(&mut self, value: TableRelationBuilder) -> &mut Self
fn unnest(&mut self, value: UnnestRelationBuilder) -> &mut Self
```

---

## SelectBuilder

`struct` · `datafusion_sql::unparser::ast::SelectBuilder`

```rust
struct SelectBuilder
```

**Derives**: Clone, Default

**Methods** (27)

```rust
fn add_flatten_table_alias(&mut self, alias: String)
fn already_projected(&self) -> bool
fn build(&self) -> Result<ast::Select, BuilderError>
fn cluster_by(&mut self, value: Vec<ast::Expr>) -> &mut Self
fn current_flatten_alias(&self) -> Option<String>
fn distinct(&mut self, value: Option<ast::Distinct>) -> &mut Self
fn distribute_by(&mut self, value: Vec<ast::Expr>) -> &mut Self
fn flatten_table_aliases_empty(&self) -> bool
fn from(&mut self, value: Vec<TableWithJoinsBuilder>) -> &mut Self
fn group_by(&mut self, value: ast::GroupByExpr) -> &mut Self
fn has_selection(&self) -> bool
fn having(&mut self, value: Option<ast::Expr>) -> &mut Self
fn into(&mut self, value: Option<ast::SelectInto>) -> &mut Self
fn is_flatten_table_alias(&self, alias: &str) -> bool
fn lateral_views(&mut self, value: Vec<ast::LateralView>) -> &mut Self
fn named_window(&mut self, value: Vec<ast::NamedWindowDefinition>) -> &mut Self
fn next_flatten_alias(&mut self) -> String
fn pop_from(&mut self) -> Option<TableWithJoinsBuilder>
fn pop_projections(&mut self) -> Vec<ast::SelectItem>
fn projection(&mut self, value: Vec<ast::SelectItem>) -> &mut Self
fn push_from(&mut self, value: TableWithJoinsBuilder) -> &mut Self
fn qualify(&mut self, value: Option<ast::Expr>) -> &mut Self
fn replace_mark(&mut self, existing_expr: &ast::Expr, value: &ast::Expr) -> &mut Self
fn selection(&mut self, value: Option<ast::Expr>) -> &mut Self
fn sort_by(&mut self, value: Vec<ast::OrderByExpr>) -> &mut Self
fn top(&mut self, value: Option<ast::Top>) -> &mut Self
fn value_table_mode(&mut self, value: Option<ast::ValueTableMode>) -> &mut Self
```

---

## TableRelationBuilder

`struct` · `datafusion_sql::unparser::ast::TableRelationBuilder`

```rust
struct TableRelationBuilder
```

**Derives**: Clone, Default

**Methods** (8)

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
fn args(&mut self, value: Option<Vec<ast::FunctionArg>>) -> &mut Self
fn build(&self) -> Result<ast::TableFactor, BuilderError>
fn index_hints(&mut self, value: Vec<ast::TableIndexHints>) -> &mut Self
fn name(&mut self, value: ast::ObjectName) -> &mut Self
fn partitions(&mut self, value: Vec<ast::Ident>) -> &mut Self
fn version(&mut self, value: Option<ast::TableVersion>) -> &mut Self
fn with_hints(&mut self, value: Vec<ast::Expr>) -> &mut Self
```

---

## TableWithJoinsBuilder

`struct` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder`

```rust
struct TableWithJoinsBuilder
```

**Derives**: Clone, Default

**Methods** (4)

```rust
fn build(&self) -> Result<Option<ast::TableWithJoins>, BuilderError>
fn joins(&mut self, value: Vec<ast::Join>) -> &mut Self
fn push_join(&mut self, value: ast::Join) -> &mut Self
fn relation(&mut self, value: RelationBuilder) -> &mut Self
```

---

## UninitializedFieldError

`struct` · `datafusion_sql::unparser::ast::UninitializedFieldError`

```rust
struct UninitializedFieldError
```

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn field_name(&self) -> &'static str
fn new(field_name: &'static str) -> Self
```

**via `core::convert::From`**

```rust
fn from(field_name: &'static str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Runtime error when a `build()` method is called and one or more required fields
do not have a value.

---

## UnnestRelationBuilder

`struct` · `datafusion_sql::unparser::ast::UnnestRelationBuilder`

```rust
struct UnnestRelationBuilder
```

**Fields**: `alias`, `array_exprs`

**Derives**: Clone, Default

**Methods** (6)

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
fn array_exprs(&mut self, value: Vec<ast::Expr>) -> &mut Self
fn build(&self) -> Result<ast::TableFactor, BuilderError>
fn with_offset(&mut self, value: bool) -> &mut Self
fn with_offset_alias(&mut self, value: Option<ast::Ident>) -> &mut Self
fn with_ordinality(&mut self, value: bool) -> &mut Self
```

---
