# `datafusion_expr::expr`

Crate `datafusion-expr` · 39 public items · structured records in [`model/datafusion_expr.expr.json`](../model/datafusion_expr.expr.json)

## OUTER_REFERENCE_COLUMN_PREFIX

`constant` · `datafusion_expr::expr::OUTER_REFERENCE_COLUMN_PREFIX`

```rust
const OUTER_REFERENCE_COLUMN_PREFIX: &str = "outer_ref"
```

---

## UNNEST_COLUMN_PREFIX

`constant` · `datafusion_expr::expr::UNNEST_COLUMN_PREFIX`

```rust
const UNNEST_COLUMN_PREFIX: &str = "UNNEST"
```

---

## Expr

`enum` · `datafusion_expr::expr::Expr`

Also reachable as `datafusion::logical_expr::Expr`, `datafusion::prelude::Expr`, `datafusion_expr::Expr`

```rust
enum Expr
```

**Variants**: `Alias`, `Column`, `ScalarVariable`, `Literal`, `BinaryExpr`, `Like`, `SimilarTo`, `Not`, `IsNotNull`, `IsNull`, `IsTrue`, `IsFalse`, `IsUnknown`, `IsNotTrue`, `IsNotFalse`, `IsNotUnknown`, `Negative`, `Between`, `Case`, `Cast`, `TryCast`, `ScalarFunction`, `AggregateFunction`, `WindowFunction`, `InList`, `Exists`, `InSubquery`, `SetComparison`, `ScalarSubquery`, `Wildcard`, `GroupingSet`, `Placeholder`, `OuterReferenceColumn`, `Unnest`, `HigherOrderFunction`, `Lambda`, `LambdaVariable`

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::ops::arith::Add`, `core::ops::arith::Div`, `core::ops::arith::Mul`, `core::ops::arith::Neg`, `core::ops::arith::Rem`, `core::ops::arith::Sub`, `core::ops::bit::BitAnd`, `core::ops::bit::BitOr`, `core::ops::bit::BitXor`, `core::ops::bit::Not`, `core::ops::bit::Shl`, `core::ops::bit::Shr`, `datafusion_common::cse::HashNode`, `datafusion_common::cse::NormalizeEq`, `datafusion_common::cse::Normalizeable`, `datafusion_common::tree_node::TreeNode`, `datafusion_common::tree_node::TreeNodeContainer`, `datafusion_expr::expr_fn::ExprFunctionExt`, `datafusion_expr::expr_schema::ExprSchemable`, `datafusion_functions::core::expr_ext::FieldAccessor`, `datafusion_functions_nested::expr_ext::IndexAccessor`, `datafusion_functions_nested::expr_ext::SliceAccessor`, `datafusion_proto::bytes::Serializeable`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (53)

```rust
fn add_column_ref_counts<'a>(&'a self, map: &mut HashMap<&'a Column, usize>)
fn add_column_refs<'a>(&'a self, set: &mut HashSet<&'a Column>)
fn alias(self, name: impl Into<String>) -> Expr
fn alias_if_changed(self, original_name: String) -> Result<Expr>
fn alias_qualified(self, relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Expr
fn alias_qualified_with_metadata(self, relation: Option<impl Into<TableReference>>, name: impl Into<String>, metadata: Option<FieldMetadata>) -> Expr
fn alias_with_metadata(self, name: impl Into<String>, metadata: Option<FieldMetadata>) -> Expr
fn and(self, other: Expr) -> Expr
fn any_column_refs(&self) -> bool
fn as_literal(&self) -> Option<&ScalarValue>
fn between(self, low: Expr, high: Expr) -> Expr
fn column_refs(&self) -> HashSet<&Column>
fn column_refs_counts(&self) -> HashMap<&Column, usize>
fn contains_outer(&self) -> bool
fn contains_scalar_subquery(&self) -> bool
fn eq(self, other: Expr) -> Expr
fn get_as_join_column(&self) -> Option<&Column>
fn gt(self, other: Expr) -> Expr
fn gt_eq(self, other: Expr) -> Expr
fn human_display(&self) -> impl Display + '_
fn ilike(self, other: Expr) -> Expr
fn in_list(self, list: Vec<Expr>, negated: bool) -> Expr
fn infer_placeholder_types(self, schema: &DFSchema) -> Result<(Expr, bool)>
fn is_false(self) -> Expr
fn is_not_false(self) -> Expr
fn is_not_null(self) -> Expr
fn is_not_true(self) -> Expr
fn is_not_unknown(self) -> Expr
fn is_null(self) -> Expr
fn is_true(self) -> Expr
fn is_unknown(self) -> Expr
fn is_volatile(&self) -> bool
fn is_volatile_node(&self) -> bool
fn like(self, other: Expr) -> Expr
fn lt(self, other: Expr) -> Expr
fn lt_eq(self, other: Expr) -> Expr
fn name_for_alias(&self) -> Result<String>
fn not_between(self, low: Expr, high: Expr) -> Expr
fn not_eq(self, other: Expr) -> Expr
fn not_ilike(self, other: Expr) -> Expr
fn not_like(self, other: Expr) -> Expr
fn or(self, other: Expr) -> Expr
fn placement(&self) -> ExpressionPlacement
fn qualified_name(&self) -> (Option<TableReference>, String)
fn resolve_lambda_variables(self, schema: &DFSchema) -> Result<Transformed<Expr>>
fn schema_name(&self) -> impl Display + '_
fn short_circuits(&self) -> bool
fn sort(self, asc: bool, nulls_first: bool) -> Sort
fn spans(&self) -> Option<&Spans>
fn try_as_col(&self) -> Option<&Column>
fn unalias(self) -> Expr
fn unalias_nested(self) -> Transformed<Expr>
fn variant_name(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Expr
```

**via `core::convert::From`**

```rust
fn from(value: (Option<&'a TableReference>, &'a FieldRef)) -> Self
fn from(value: WindowFunction) -> Self
fn from(value: ScalarAndMetadata) -> Self
fn from(value: Column) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: Self) -> Self
```

**via `core::ops::arith::Div`**

```rust
fn div(self, rhs: Self) -> Self
```

**via `core::ops::arith::Mul`**

```rust
fn mul(self, rhs: Self) -> Self
```

**via `core::ops::arith::Neg`**

```rust
fn neg(self) -> Self::Output
```

**via `core::ops::arith::Rem`**

```rust
fn rem(self, rhs: Self) -> Self
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, rhs: Self) -> Self
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, rhs: Self) -> Self
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: Self) -> Self
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, rhs: Self) -> Self
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self::Output
```

**via `core::ops::bit::Shl`**

```rust
fn shl(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::Shr`**

```rust
fn shr(self, rhs: Self) -> Self::Output
```

**via `datafusion_common::cse::HashNode`**

```rust
fn hash_node<H: Hasher>(&self, state: &mut H)
```

**via `datafusion_common::cse::NormalizeEq`**

```rust
fn normalize_eq(&self, other: &Self) -> bool
```

**via `datafusion_common::cse::Normalizeable`**

```rust
fn can_normalize(&self) -> bool
```

**via `datafusion_common::tree_node::TreeNode`**

```rust
fn apply_children<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

**via `datafusion_common::tree_node::TreeNodeContainer`**

```rust
fn apply_elements<F: FnMut(&'a Self) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
fn map_elements<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

**via `datafusion_expr::expr_fn::ExprFunctionExt`**

```rust
fn distinct(self) -> ExprFuncBuilder
fn filter(self, filter: Expr) -> ExprFuncBuilder
fn null_treatment(self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder
fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder
fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder
fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder
```

**via `datafusion_expr::expr_schema::ExprSchemable`**

```rust
fn cast_to(self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr>
fn data_type_and_nullable(&self, schema: &dyn ExprSchema) -> Result<(DataType, bool)>
fn get_type(&self, schema: &dyn ExprSchema) -> Result<DataType>
fn metadata(&self, schema: &dyn ExprSchema) -> Result<FieldMetadata>
fn nullable(&self, input_schema: &dyn ExprSchema) -> Result<bool>
fn to_field(&self, schema: &dyn ExprSchema) -> Result<(Option<TableReference>, Arc<Field>)>
```

Represents logical expressions such as `A + 1`, or `CAST(c1 AS int)`.

For example the expression `A + 1` will be represented as

```text
 BinaryExpr {
   left: Expr::Column("A"),
   op: Operator::Plus,
   right: Expr::Literal(ScalarValue::Int32(Some(1)), None)
}
```

# Creating Expressions

`Expr`s can be created directly, but it is often easier and less verbose to
use the fluent APIs in [`crate::expr_fn`] such as [`col`] and [`lit`], or
methods such as [`Expr::alias`], [`Expr::cast_to`], and [`Expr::Like`]).

See also [`ExprFunctionExt`] for creating aggregate and window functions.

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

# Printing Expressions

You can print `Expr`s using the `Debug` trait, `Display` trait, or
[`Self::human_display`]. See the [examples](#examples-displaying-exprs) below.

If you need  SQL to pass to other systems, consider using [`Unparser`].

[`Unparser`]: https://docs.rs/datafusion/latest/datafusion/sql/unparser/struct.Unparser.html

# Schema Access

See [`ExprSchemable::get_type`] to access the [`DataType`] and nullability
of an `Expr`.

# Visiting and Rewriting `Expr`s

The `Expr` struct implements the [`TreeNode`] trait for walking and
rewriting expressions. For example [`TreeNode::apply`] recursively visits an
`Expr` and [`TreeNode::transform`] can be used to rewrite an expression. See
the examples below and [`TreeNode`] for more information.

# Examples: Creating and Using `Expr`s

## Column References and Literals

[`Expr::Column`] refer to the values of columns and are often created with
the [`col`] function. For example to create an expression `c1` referring to
column named "c1":

[`col`]: crate::expr_fn::col

```
# use datafusion_common::Column;
# use datafusion_expr::{lit, col, Expr};
let expr = col("c1");
assert_eq!(expr, Expr::Column(Column::from_name("c1")));
```

[`Expr::Literal`] refer to literal, or constant, values. These are created
with the [`lit`] function. For example to create an expression `42`:

[`lit`]: crate::lit

```
# use datafusion_common::{Column, ScalarValue};
# use datafusion_expr::{lit, col, Expr};
// All literals are strongly typed in DataFusion. To make an `i64` 42:
let expr = lit(42i64);
assert_eq!(expr, Expr::Literal(ScalarValue::Int64(Some(42)), None));
assert_eq!(expr, Expr::Literal(ScalarValue::Int64(Some(42)), None));
// To make a (typed) NULL:
let expr = Expr::Literal(ScalarValue::Int64(None), None);
// to make an (untyped) NULL (the optimizer will coerce this to the correct type):
let expr = lit(ScalarValue::Null);
```

## Binary Expressions

Exprs implement traits that allow easy to understand construction of more
complex expressions. For example, to create `c1 + c2` to add columns "c1" and
"c2" together

```
# use datafusion_expr::{lit, col, Operator, Expr};
// Use the `+` operator to add two columns together
let expr = col("c1") + col("c2");
assert!(matches!(expr, Expr::BinaryExpr { .. }));
if let Expr::BinaryExpr(binary_expr) = expr {
    assert_eq!(*binary_expr.left, col("c1"));
    assert_eq!(*binary_expr.right, col("c2"));
    assert_eq!(binary_expr.op, Operator::Plus);
}
```

The expression `c1 = 42` to compares the value in column "c1" to the
literal value `42`:

```
# use datafusion_common::ScalarValue;
# use datafusion_expr::{lit, col, Operator, Expr};
let expr = col("c1").eq(lit(42_i32));
assert!(matches!(expr, Expr::BinaryExpr { .. }));
if let Expr::BinaryExpr(binary_expr) = expr {
    assert_eq!(*binary_expr.left, col("c1"));
    let scalar = ScalarValue::Int32(Some(42));
    assert_eq!(*binary_expr.right, Expr::Literal(scalar, None));
    assert_eq!(binary_expr.op, Operator::Eq);
}
```

Here is how to implement the equivalent of `SELECT *` to select all
[`Expr::Column`] from a [`DFSchema`]'s columns:

```
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_common::{DFSchema, Column};
# use datafusion_expr::Expr;
// Create a schema c1(int, c2 float)
let arrow_schema = Schema::new(vec![
    Field::new("c1", DataType::Int32, false),
    Field::new("c2", DataType::Float64, false),
]);
// DFSchema is a an Arrow schema with optional relation name
let df_schema = DFSchema::try_from_qualified_schema("t1", &arrow_schema).unwrap();

// Form Vec<Expr> with an expression for each column in the schema
let exprs: Vec<_> = df_schema.iter().map(Expr::from).collect();

assert_eq!(
    exprs,
    vec![
        Expr::from(Column::from_qualified_name("t1.c1")),
        Expr::from(Column::from_qualified_name("t1.c2")),
    ]
);
```

# Examples: Displaying `Exprs`

There are three ways to print an `Expr` depending on the usecase.

## Use `Debug` trait

Following Rust conventions, the `Debug` implementation prints out the
internal structure of the expression, which is useful for debugging.

```
# use datafusion_expr::{lit, col};
let expr = col("c1") + lit(42);
assert_eq!(format!("{expr:?}"), "BinaryExpr(BinaryExpr { left: Column(Column { relation: None, name: \"c1\" }), op: Plus, right: Literal(Int32(42), None) })");
```

## Use the `Display` trait  (detailed expression)

The `Display` implementation prints out the expression in a SQL-like form,
but has additional details such as the data type of literals. This is useful
for understanding the expression in more detail and is used for the low level
[`ExplainFormat::Indent`] explain plan format.

[`ExplainFormat::Indent`]: crate::logical_plan::ExplainFormat::Indent

```
# use datafusion_expr::{lit, col};
let expr = col("c1") + lit(42);
assert_eq!(format!("{expr}"), "c1 + Int32(42)");
```

## Use [`Self::human_display`] (human readable)

[`Self::human_display`]  prints out the expression in a SQL-like form, optimized
for human consumption by end users. It is used for the
[`ExplainFormat::Tree`] explain plan format.

[`ExplainFormat::Tree`]: crate::logical_plan::ExplainFormat::Tree

```
# use datafusion_expr::{lit, col};
let expr = col("c1") + lit(42);
assert_eq!(format!("{}", expr.human_display()), "c1 + 42");
```

# Examples: Visiting and Rewriting `Expr`s

Here is an example that finds all literals in an `Expr` tree:
```
# use std::collections::{HashSet};
use datafusion_common::ScalarValue;
# use datafusion_expr::{col, Expr, lit};
use datafusion_common::tree_node::{TreeNode, TreeNodeRecursion};
// Expression a = 5 AND b = 6
let expr = col("a").eq(lit(5)) & col("b").eq(lit(6));
// find all literals in a HashMap
let mut scalars = HashSet::new();
// apply recursively visits all nodes in the expression tree
expr.apply(|e| {
    if let Expr::Literal(scalar, _) = e {
        scalars.insert(scalar);
    }
    // The return value controls whether to continue visiting the tree
    Ok(TreeNodeRecursion::Continue)
})
.unwrap();
// All subtrees have been visited and literals found
assert_eq!(scalars.len(), 2);
assert!(scalars.contains(&ScalarValue::Int32(Some(5))));
assert!(scalars.contains(&ScalarValue::Int32(Some(6))));
```

Rewrite an expression, replacing references to column "a" in an
to the literal `42`:

```
# use datafusion_common::tree_node::{Transformed, TreeNode};
# use datafusion_expr::{col, Expr, lit};
// expression a = 5 AND b = 6
let expr = col("a").eq(lit(5)).and(col("b").eq(lit(6)));
// rewrite all references to column "a" to the literal 42
let rewritten = expr.transform(|e| {
  if let Expr::Column(c) = &e {
    if &c.name == "a" {
      // return Transformed::yes to indicate the node was changed
      return Ok(Transformed::yes(lit(42)))
    }
  }
  // return Transformed::no to indicate the node was not changed
  Ok(Transformed::no(e))
}).unwrap();
// The expression has been rewritten
assert!(rewritten.transformed);
// to 42 = 5 AND b = 6
assert_eq!(rewritten.data, lit(42).eq(lit(5)).and(col("b").eq(lit(6))));
```

---

## GetFieldAccess

`enum` · `datafusion_expr::expr::GetFieldAccess`

Also reachable as `datafusion::logical_expr::GetFieldAccess`, `datafusion_expr::GetFieldAccess`

```rust
enum GetFieldAccess
```

**Variants**: `NamedStructField`, `ListIndex`, `ListRange`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Access a sub field of a nested type, such as `Field` or `List`

---

## GroupingSet

`enum` · `datafusion_expr::expr::GroupingSet`

Also reachable as `datafusion::logical_expr::GroupingSet`, `datafusion_expr::GroupingSet`

```rust
enum GroupingSet
```

**Variants**: `Rollup`, `Cube`, `GroupingSets`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn distinct_expr(&self) -> Vec<&Expr>
```

Grouping sets

See <https://www.postgresql.org/docs/current/queries-table-expressions.html#QUERIES-GROUPING-SETS>
for Postgres definition.
See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-groupby.html>
for Apache Spark definition.

---

## NullTreatment

`enum` · `datafusion_expr::expr::NullTreatment`

```rust
enum NullTreatment
```

**Variants**: `IgnoreNulls`, `RespectNulls`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: sqlparser::ast::NullTreatment) -> Self
fn from(t: protobuf::NullTreatment) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

---

## SetQuantifier

`enum` · `datafusion_expr::expr::SetQuantifier`

```rust
enum SetQuantifier
```

**Variants**: `Any`, `All`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Whether the set comparison uses `ANY`/`SOME` or `ALL`

---

## WindowFunctionDefinition

`enum` · `datafusion_expr::expr::WindowFunctionDefinition`

Also reachable as `datafusion::logical_expr::WindowFunctionDefinition`, `datafusion_expr::WindowFunctionDefinition`

```rust
enum WindowFunctionDefinition
```

**Variants**: `AggregateUDF`, `WindowUDF`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn name(&self) -> &str
fn return_field(&self, input_expr_fields: &[FieldRef], display_name: &str) -> Result<FieldRef>
fn signature(&self) -> Signature
fn simplify(&self) -> Option<WindowFunctionSimplification>
```

**via `core::convert::From`**

```rust
fn from(value: Arc<WindowUDF>) -> Self
fn from(value: Arc<AggregateUDF>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

A function used as a SQL window function

In SQL, you can use:
- Actual window functions ([`WindowUDF`])
- Normal aggregate functions ([`AggregateUDF`])

---

## display_comma_separated

`function` · `datafusion_expr::expr::display_comma_separated`

```rust
fn display_comma_separated<T>(slice: &[T]) -> String where T: Display
```

---

## intersect_metadata_for_union

`function` · `datafusion_expr::expr::intersect_metadata_for_union`

```rust
fn intersect_metadata_for_union<'a>(metadatas: impl IntoIterator<Item = &'a SchemaFieldMetadata>) -> SchemaFieldMetadata
```

Intersects multiple metadata instances for UNION operations.

This function implements the intersection strategy used by UNION operations,
where only metadata keys that exist in ALL inputs with identical values
are preserved in the result.

# Union Metadata Behavior

Union operations require consistent metadata across all branches:
- Only metadata keys present in ALL union branches are kept
- For each kept key, the value must be identical across all branches
- If a key has different values across branches, it is excluded from the result
- If any input has no metadata, the result will be empty

# Arguments

* `metadatas` - An iterator of `SchemaFieldMetadata` instances to intersect

# Returns

A new `SchemaFieldMetadata` containing only the intersected metadata

---

## physical_name

`function` · `datafusion_expr::expr::physical_name`

```rust
fn physical_name(expr: &Expr) -> datafusion_common::Result<String>
```

The name of the column (field) that this `Expr` will produce in the physical plan.
The difference from [Expr::schema_name] is that top-level columns are unqualified.

---

## schema_name_from_exprs

`function` · `datafusion_expr::expr::schema_name_from_exprs`

```rust
fn schema_name_from_exprs(exprs: &[Expr]) -> datafusion_common::Result<String, fmt::Error>
```

Get schema_name for Vector of expressions

---

## schema_name_from_sorts

`function` · `datafusion_expr::expr::schema_name_from_sorts`

```rust
fn schema_name_from_sorts(sorts: &[Sort]) -> datafusion_common::Result<String, fmt::Error>
```

---

## AggregateFunction

`struct` · `datafusion_expr::expr::AggregateFunction`

```rust
struct AggregateFunction
```

**Fields**: `func`, `params`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new_udf(func: Arc<AggregateUDF>, args: Vec<Expr>, distinct: bool, filter: Option<Box<Expr>>, order_by: Vec<Sort>, null_treatment: Option<NullTreatment>) -> Self
```

Aggregate function

See also  [`ExprFunctionExt`] to set these fields on `Expr`

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

---

## AggregateFunctionParams

`struct` · `datafusion_expr::expr::AggregateFunctionParams`

```rust
struct AggregateFunctionParams
```

**Fields**: `args`, `distinct`, `filter`, `order_by`, `null_treatment`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

---

## Alias

`struct` · `datafusion_expr::expr::Alias`

```rust
struct Alias
```

**Fields**: `expr`, `relation`, `name`, `metadata`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(expr: Expr, relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Self
fn with_metadata(self, metadata: Option<FieldMetadata>) -> Self
```

Alias expression

---

## Between

`struct` · `datafusion_expr::expr::Between`

Also reachable as `datafusion::logical_expr::Between`, `datafusion_expr::Between`

```rust
struct Between
```

**Fields**: `expr`, `negated`, `low`, `high`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(expr: Box<Expr>, negated: bool, low: Box<Expr>, high: Box<Expr>) -> Self
```

BETWEEN expression

---

## BinaryExpr

`struct` · `datafusion_expr::expr::BinaryExpr`

Also reachable as `datafusion::logical_expr::BinaryExpr`, `datafusion_expr::BinaryExpr`

```rust
struct BinaryExpr
```

**Fields**: `left`, `op`, `right`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(left: Box<Expr>, op: Operator, right: Box<Expr>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Binary expression for [`Expr::BinaryExpr`]

---

## Case

`struct` · `datafusion_expr::expr::Case`

Also reachable as `datafusion::logical_expr::Case`, `datafusion_expr::Case`

```rust
struct Case
```

**Fields**: `expr`, `when_then_expr`, `else_expr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(expr: Option<Box<Expr>>, when_then_expr: Vec<(Box<Expr>, Box<Expr>)>, else_expr: Option<Box<Expr>>) -> Self
```

CASE expression

The CASE expression is similar to a series of nested if/else and there are two forms that
can be used. The first form consists of a series of boolean "when" expressions with
corresponding "then" expressions, and an optional "else" expression.

```text
CASE WHEN condition THEN result
     [WHEN ...]
     [ELSE result]
END
```

The second form uses a base expression and then a series of "when" clauses that match on a
literal value.

```text
CASE expression
    WHEN value THEN result
    [WHEN ...]
    [ELSE result]
END
```

---

## Cast

`struct` · `datafusion_expr::expr::Cast`

Also reachable as `datafusion::logical_expr::Cast`, `datafusion_expr::Cast`

```rust
struct Cast
```

**Fields**: `expr`, `field`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(expr: Box<Expr>, data_type: DataType) -> Self
fn new_from_field(expr: Box<Expr>, field: FieldRef) -> Self
```

Cast expression

---

## Exists

`struct` · `datafusion_expr::expr::Exists`

```rust
struct Exists
```

**Fields**: `subquery`, `negated`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(subquery: Subquery, negated: bool) -> Self
```

EXISTS expression

---

## ExprListDisplay

`struct` · `datafusion_expr::expr::ExprListDisplay`

```rust
struct ExprListDisplay<'a>
```

**Implements**: `core::fmt::Display`

**Methods** (2)

```rust
fn comma_separated(exprs: &'a [Expr]) -> Self
fn new(exprs: &'a [Expr], sep: &'a str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Formats a list of `&Expr` with a custom separator using SQL display format

---

## HigherOrderFunction

`struct` · `datafusion_expr::expr::HigherOrderFunction`

```rust
struct HigherOrderFunction
```

**Fields**: `func`, `args`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (3)

```rust
fn lambda_parameters(&self, schema: &dyn ExprSchema) -> Result<Vec<Vec<FieldRef>>>
fn name(&self) -> &str
fn new(func: Arc<HigherOrderUDF>, args: Vec<Expr>) -> Self
```

Invoke a [`HigherOrderUDF`] with a set of arguments

---

## InList

`struct` · `datafusion_expr::expr::InList`

```rust
struct InList
```

**Fields**: `expr`, `list`, `negated`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(expr: Box<Expr>, list: Vec<Expr>, negated: bool) -> Self
```

InList expression

---

## InSubquery

`struct` · `datafusion_expr::expr::InSubquery`

```rust
struct InSubquery
```

**Fields**: `expr`, `subquery`, `negated`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(expr: Box<Expr>, subquery: Subquery, negated: bool) -> Self
```

IN subquery

---

## Lambda

`struct` · `datafusion_expr::expr::Lambda`

```rust
struct Lambda
```

**Fields**: `params`, `body`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(params: Vec<String>, body: Expr) -> Self
```

A Lambda expression with a set of parameters names and a body

---

## LambdaVariable

`struct` · `datafusion_expr::expr::LambdaVariable`

```rust
struct LambdaVariable
```

**Fields**: `name`, `field`, `spans`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(name: String, field: Option<FieldRef>) -> Self
fn spans_mut(&mut self) -> &mut Spans
```

A named reference to a lambda parameter which includes it's own [`FieldRef`],
which is used to implement [`ExprSchemable`], for example. It is an option only to make
easier for `expr_api` users to construct lambda variables, but any expression
tree or [`LogicalPlan`] containing unresolved variables must be resolved before
usage with either [`Expr::resolve_lambda_variables`] or
[`LogicalPlan::resolve_lambda_variables`]. The default SQL planner produces
already resolved variables and no further resolving is required.

After resolving, if any argument from the lambda function which this
variables originates from have it's field changed (type, nullability,
metadata, etc), the resolved variable may became outdated and must be
resolved again.

[`LogicalPlan`]: crate::LogicalPlan
[`LogicalPlan::resolve_lambda_variables`]: crate::LogicalPlan::resolve_lambda_variables

---

## Like

`struct` · `datafusion_expr::expr::Like`

Also reachable as `datafusion::logical_expr::Like`, `datafusion_expr::Like`

```rust
struct Like
```

**Fields**: `negated`, `expr`, `pattern`, `escape_char`, `case_insensitive`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(negated: bool, expr: Box<Expr>, pattern: Box<Expr>, escape_char: Option<char>, case_insensitive: bool) -> Self
```

LIKE expression

---

## Placeholder

`struct` · `datafusion_expr::expr::Placeholder`

```rust
struct Placeholder
```

**Fields**: `id`, `field`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(id: String, data_type: Option<DataType>) -> Self
fn new_with_field(id: String, field: Option<FieldRef>) -> Self
```

Placeholder, representing bind parameter values such as `$1` or `$name`.

The type of these parameters is inferred using [`Expr::infer_placeholder_types`]
or can be specified directly using `PREPARE` statements.

---

## PlannedReplaceSelectItem

`struct` · `datafusion_expr::expr::PlannedReplaceSelectItem`

```rust
struct PlannedReplaceSelectItem
```

**Fields**: `items`, `planned_expressions`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn expressions(&self) -> &[Expr]
fn items(&self) -> &[ReplaceSelectElement]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

The planned expressions for `REPLACE`

---

## ScalarFunction

`struct` · `datafusion_expr::expr::ScalarFunction`

```rust
struct ScalarFunction
```

**Fields**: `func`, `args`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> &str
fn new_udf(udf: Arc<ScalarUDF>, args: Vec<Expr>) -> Self
```

Invoke a [`ScalarUDF`] with a set of arguments

[`ScalarUDF`]: crate::ScalarUDF

---

## SetComparison

`struct` · `datafusion_expr::expr::SetComparison`

```rust
struct SetComparison
```

**Fields**: `expr`, `subquery`, `op`, `quantifier`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(expr: Box<Expr>, subquery: Subquery, op: Operator, quantifier: SetQuantifier) -> Self
```

Set comparison subquery (e.g. `= ANY`, `> ALL`)

---

## Sort

`struct` · `datafusion_expr::expr::Sort`

Also reachable as `datafusion::logical_expr::Sort`, `datafusion_expr::SortExpr`

```rust
struct Sort
```

**Fields**: `expr`, `asc`, `nulls_first`

**Implements**: `core::fmt::Display`, `datafusion_common::tree_node::TreeNodeContainer`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn new(expr: Expr, asc: bool, nulls_first: bool) -> Self
fn reverse(&self) -> Self
fn with_expr(&self, expr: Expr) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_common::tree_node::TreeNodeContainer`**

```rust
fn apply_elements<F: FnMut(&'a Expr) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

SORT expression

---

## TryCast

`struct` · `datafusion_expr::expr::TryCast`

Also reachable as `datafusion::logical_expr::TryCast`, `datafusion_expr::TryCast`

```rust
struct TryCast
```

**Fields**: `expr`, `field`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(expr: Box<Expr>, data_type: DataType) -> Self
fn new_from_field(expr: Box<Expr>, field: FieldRef) -> Self
```

TryCast Expression

---

## Unnest

`struct` · `datafusion_expr::expr::Unnest`

```rust
struct Unnest
```

**Fields**: `expr`, `outer`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn new(expr: Expr) -> Self
fn new_boxed(boxed: Box<Expr>) -> Self
fn new_outer(expr: Expr) -> Self
```

UNNEST expression.

When `outer` is `true`, the unnest should preserve `NULL` and empty input
lists by emitting a single `NULL` output row for each. When `false` (the
historical default), the behavior is identical to the plain `UNNEST(col)`
SQL form: `NULL` and empty input lists are dropped from the output.

---

## WildcardOptions

`struct` · `datafusion_expr::expr::WildcardOptions`

```rust
struct WildcardOptions
```

**Fields**: `ilike`, `exclude`, `except`, `replace`, `rename`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn with_replace(self, replace: PlannedReplaceSelectItem) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Additional options for wildcards, e.g. Snowflake `EXCLUDE`/`RENAME` and Bigquery `EXCEPT`.

---

## WindowFunction

`struct` · `datafusion_expr::expr::WindowFunction`

```rust
struct WindowFunction
```

**Fields**: `fun`, `params`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(fun: impl Into<WindowFunctionDefinition>, args: Vec<Expr>) -> Self
fn simplify(&self) -> Option<WindowFunctionSimplification>
```

Window function

Holds the actual function to call [`WindowFunction`] as well as its
arguments (`args`) and the contents of the `OVER` clause:

1. `PARTITION BY`
2. `ORDER BY`
3. Window frame (e.g. `ROWS 1 PRECEDING AND 1 FOLLOWING`)

See [`ExprFunctionExt`] for examples of how to create a `WindowFunction`.

[`ExprFunctionExt`]: crate::ExprFunctionExt

---

## WindowFunctionParams

`struct` · `datafusion_expr::expr::WindowFunctionParams`

```rust
struct WindowFunctionParams
```

**Fields**: `args`, `partition_by`, `order_by`, `window_frame`, `filter`, `null_treatment`, `distinct`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

---

## SchemaFieldMetadata

`type_alias` · `datafusion_expr::expr::SchemaFieldMetadata`

```rust
type SchemaFieldMetadata = std::collections::HashMap<String, String>
```

The metadata used in [`Field::metadata`].

This represents the metadata associated with an Arrow [`Field`]. The metadata consists of key-value pairs.

# Common Use Cases

Field metadata is commonly used to store:
- Default values for columns when data is missing
- Column descriptions or documentation
- Data lineage information
- Custom application-specific annotations
- Encoding hints or display formatting preferences

# Example: Storing Default Values

A practical example of using field metadata is storing default values for columns
that may be missing in the physical data but present in the logical schema.
See the [default_column_values.rs] example implementation.

[default_column_values.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/custom_data_source/default_column_values.rs

---
