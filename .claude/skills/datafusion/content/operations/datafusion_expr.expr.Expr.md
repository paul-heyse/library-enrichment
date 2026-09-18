# `datafusion_expr::expr::Expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Expr.json).

<a id="op-230499d6f244cf7372db53bc"></a>
## Expr

`enum` · `datafusion_expr::expr::Expr` · datafusion-expr 55.1.0

```rust
enum Expr
```

Source: `src/expr.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

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
use the fluent APIs in [`crate::expr_fn`](../modules/datafusion_expr.expr_fn.md#op-e7001738a261bab88eecdffd) such as [`col`] and [`lit`], or
methods such as [`Expr::alias`](../operations/datafusion_expr.expr.Expr.md#op-b1bb2bd71918897567a64c11), [`Expr::cast_to`](../operations/datafusion_expr.expr.Expr.md#op-97b23e3232ac43bdf8143a83), and [`Expr::Like`](../operations/datafusion_expr.expr.Expr.md#op-d4d24ebc554451be18b29ca2)).

See also [`ExprFunctionExt`] for creating aggregate and window functions.

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

# Printing Expressions

You can print `Expr`s using the `Debug` trait, `Display` trait, or
[`Self::human_display`](../operations/datafusion_expr.expr.Expr.md#op-3d5b1a93f5851b5885d840ce). See the [examples](#examples-displaying-exprs) below.

If you need  SQL to pass to other systems, consider using [`Unparser`].

[`Unparser`]: https://docs.rs/datafusion/latest/datafusion/sql/unparser/struct.Unparser.html

# Schema Access

See [`ExprSchemable::get_type`](../operations/datafusion_expr.expr_schema.ExprSchemable.md#op-1fe6f24ccb408cc1caaba5aa) to access the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) and nullability
of an `Expr`.

# Visiting and Rewriting `Expr`s

The `Expr` struct implements the [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) trait for walking and
rewriting expressions. For example [`TreeNode::apply`](../operations/datafusion_common.tree_node.TreeNode.md#op-bd071500b276296c970d8bcd) recursively visits an
`Expr` and [`TreeNode::transform`](../operations/datafusion_common.tree_node.TreeNode.md#op-335537caf61ac03893f423ab) can be used to rewrite an expression. See
the examples below and [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) for more information.

# Examples: Creating and Using `Expr`s

## Column References and Literals

[`Expr::Column`](../operations/datafusion_expr.expr.Expr.md#op-711b028cea444e99ecdf33d0) refer to the values of columns and are often created with
the [`col`] function. For example to create an expression `c1` referring to
column named "c1":

[`col`]: crate::expr_fn::col

```
# use datafusion_common::Column;
# use datafusion_expr::{lit, col, Expr};
let expr = col("c1");
assert_eq!(expr, Expr::Column(Column::from_name("c1")));
```

[`Expr::Literal`](../operations/datafusion_expr.expr.Expr.md#op-c99af4213ce55a9ac2d007ab) refer to literal, or constant, values. These are created
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
[`Expr::Column`](../operations/datafusion_expr.expr.Expr.md#op-711b028cea444e99ecdf33d0) from a [`DFSchema`](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98)'s columns:

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

## Use [`Self::human_display`](../operations/datafusion_expr.expr.Expr.md#op-3d5b1a93f5851b5885d840ce) (human readable)

[`Self::human_display`](../operations/datafusion_expr.expr.Expr.md#op-3d5b1a93f5851b5885d840ce)  prints out the expression in a SQL-like form, optimized
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

<a id="op-98c26e3c882c5c14cacb82a2"></a>
## AggregateFunction

`variant` · `datafusion_expr::expr::Expr::AggregateFunction` · datafusion-expr 55.1.0

```rust
AggregateFunction
```

Source: `src/expr.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Calls an aggregate function with arguments, and optional
`ORDER BY`, `FILTER`, `DISTINCT` and `NULL TREATMENT`.

See also [`ExprFunctionExt`] to set these fields.

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

<a id="op-99ef95e1ad9855e695be1ebb"></a>
## Alias

`variant` · `datafusion_expr::expr::Expr::Alias` · datafusion-expr 55.1.0

```rust
Alias
```

Source: `src/expr.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An expression with a specific name.

<a id="op-c67f4fb5d000f8c263795bb0"></a>
## Between

`variant` · `datafusion_expr::expr::Expr::Between` · datafusion-expr 55.1.0

```rust
Between
```

Source: `src/expr.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether an expression is between a given range.

<a id="op-5fea7a03414f3ba72127e37d"></a>
## BinaryExpr

`variant` · `datafusion_expr::expr::Expr::BinaryExpr` · datafusion-expr 55.1.0

```rust
BinaryExpr
```

Source: `src/expr.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A binary expression such as "age > 21"

<a id="op-030741e7d78bd356286f24f2"></a>
## Case

`variant` · `datafusion_expr::expr::Expr::Case` · datafusion-expr 55.1.0

```rust
Case
```

Source: `src/expr.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A CASE expression (see docs on [`Case`](../operations/datafusion_expr.expr.Case.md#op-358933dad32fc3f4ce9d3573))

<a id="op-fe8aba365a7255f4059d5fc3"></a>
## Cast

`variant` · `datafusion_expr::expr::Expr::Cast` · datafusion-expr 55.1.0

```rust
Cast
```

Source: `src/expr.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Casts the expression to a given type and will return a runtime error if the expression cannot be cast.
This expression is guaranteed to have a fixed type.

<a id="op-711b028cea444e99ecdf33d0"></a>
## Column

`variant` · `datafusion_expr::expr::Expr::Column` · datafusion-expr 55.1.0

```rust
Column
```

Source: `src/expr.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A named reference to a qualified field in a schema.

<a id="op-51a6d7cbf08d0f1abb3ed2fe"></a>
## Exists

`variant` · `datafusion_expr::expr::Expr::Exists` · datafusion-expr 55.1.0

```rust
Exists
```

Source: `src/expr.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

EXISTS subquery

<a id="op-9aea79504686cdb750abed59"></a>
## GroupingSet

`variant` · `datafusion_expr::expr::Expr::GroupingSet` · datafusion-expr 55.1.0

```rust
GroupingSet
```

Source: `src/expr.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List of grouping set expressions. Only valid in the context of an aggregate
GROUP BY expression list

<a id="op-fa4f2502823ff39899f95029"></a>
## HigherOrderFunction

`variant` · `datafusion_expr::expr::Expr::HigherOrderFunction` · datafusion-expr 55.1.0

```rust
HigherOrderFunction
```

Source: `src/expr.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Call a higher order function with a set of arguments.

For example, `array_transform([1,2,3], v -> v+1)` would be equivalent to:

```text
HigherOrderFunction(array_transform)
├── args[0]: Literal([1,2,3])
└── args[1]: Lambda
    ├── params: ["v"]
    └── body: BinaryExpr(+)
        ├── LambdaVariable("v")
        └── Literal(1)
```

<a id="op-698859116d1a3c07dc061889"></a>
## InList

`variant` · `datafusion_expr::expr::Expr::InList` · datafusion-expr 55.1.0

```rust
InList
```

Source: `src/expr.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns whether the list contains the expr value.

<a id="op-b5bf55a5a24fb2c0856a925d"></a>
## InSubquery

`variant` · `datafusion_expr::expr::Expr::InSubquery` · datafusion-expr 55.1.0

```rust
InSubquery
```

Source: `src/expr.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

IN subquery

<a id="op-424d5dedf5a54ff6695a052b"></a>
## IsFalse

`variant` · `datafusion_expr::expr::Expr::IsFalse` · datafusion-expr 55.1.0

```rust
IsFalse
```

Source: `src/expr.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is  false, false otherwise. This expression itself is never NULL.

<a id="op-56f8f18d085f47128ef745c2"></a>
## IsNotFalse

`variant` · `datafusion_expr::expr::Expr::IsNotFalse` · datafusion-expr 55.1.0

```rust
IsNotFalse
```

Source: `src/expr.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is TRUE OR NULL, false otherwise. This expression itself is never NULL.

<a id="op-61b91e7ae8ad37efe1340ef2"></a>
## IsNotNull

`variant` · `datafusion_expr::expr::Expr::IsNotNull` · datafusion-expr 55.1.0

```rust
IsNotNull
```

Source: `src/expr.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is not NULL, false otherwise. This expression itself is never NULL.

<a id="op-2a86017548a3f0137bbc4a48"></a>
## IsNotTrue

`variant` · `datafusion_expr::expr::Expr::IsNotTrue` · datafusion-expr 55.1.0

```rust
IsNotTrue
```

Source: `src/expr.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is FALSE or NULL, false otherwise. This expression itself is never NULL.

<a id="op-53dfef289c2cf16bd5125a82"></a>
## IsNotUnknown

`variant` · `datafusion_expr::expr::Expr::IsNotUnknown` · datafusion-expr 55.1.0

```rust
IsNotUnknown
```

Source: `src/expr.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is TRUE or FALSE, false otherwise. This expression itself is never NULL.

<a id="op-bf6b8b00dd754e0b188fd007"></a>
## IsNull

`variant` · `datafusion_expr::expr::Expr::IsNull` · datafusion-expr 55.1.0

```rust
IsNull
```

Source: `src/expr.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is NULL, false otherwise. This expression itself is never NULL.

<a id="op-cf664bd648f28b69c30cfc3f"></a>
## IsTrue

`variant` · `datafusion_expr::expr::Expr::IsTrue` · datafusion-expr 55.1.0

```rust
IsTrue
```

Source: `src/expr.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is true, false otherwise. This expression itself is never NULL.

<a id="op-12e94b74e6637c39fc8dbff6"></a>
## IsUnknown

`variant` · `datafusion_expr::expr::Expr::IsUnknown` · datafusion-expr 55.1.0

```rust
IsUnknown
```

Source: `src/expr.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

True if argument is NULL, false otherwise. This expression itself is never NULL.

<a id="op-97df3c2dd6acdc9050f5ac00"></a>
## Lambda

`variant` · `datafusion_expr::expr::Expr::Lambda` · datafusion-expr 55.1.0

```rust
Lambda
```

Source: `src/expr.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A Lambda expression with a set of parameters names and a body

<a id="op-cc0cdf367b4568c7d809b291"></a>
## LambdaVariable

`variant` · `datafusion_expr::expr::Expr::LambdaVariable` · datafusion-expr 55.1.0

```rust
LambdaVariable
```

Source: `src/expr.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A named reference to a lambda parameter

<a id="op-d4d24ebc554451be18b29ca2"></a>
## Like

`variant` · `datafusion_expr::expr::Expr::Like` · datafusion-expr 55.1.0

```rust
Like
```

Source: `src/expr.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

LIKE expression

<a id="op-c99af4213ce55a9ac2d007ab"></a>
## Literal

`variant` · `datafusion_expr::expr::Expr::Literal` · datafusion-expr 55.1.0

```rust
Literal
```

Source: `src/expr.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A constant value along with associated [`FieldMetadata`](../operations/datafusion_common.metadata.FieldMetadata.md#op-8afcac4d754acb196812c528).

<a id="op-7c46c472bbec3f799a7ceff0"></a>
## Negative

`variant` · `datafusion_expr::expr::Expr::Negative` · datafusion-expr 55.1.0

```rust
Negative
```

Source: `src/expr.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

arithmetic negation of an expression, the operand must be of a signed numeric data type

<a id="op-7e3fc95735ddb314e9d80202"></a>
## Not

`variant` · `datafusion_expr::expr::Expr::Not` · datafusion-expr 55.1.0

```rust
Not
```

Source: `src/expr.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Negation of an expression. The expression's type must be a boolean to make sense.

<a id="op-888620a725a506630b03f8bf"></a>
## OuterReferenceColumn

`variant` · `datafusion_expr::expr::Expr::OuterReferenceColumn` · datafusion-expr 55.1.0

```rust
OuterReferenceColumn
```

Source: `src/expr.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A placeholder which holds a reference to a qualified field
in the outer query, used for correlated sub queries.

<a id="op-2186d72b9c826c94799572ff"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [104, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::Shl", "path": "Shl"}, "trait_path": "core::ops::bit::Shl"}`

Source: `src/operation.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fb7a30b6f77c83486d40c4b"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [113, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::Shr", "path": "Shr"}, "trait_path": "core::ops::bit::Shr"}`

Source: `src/operation.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33f90f69041dc28900e1c53f"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [41, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/operation.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e7494999372031689654506"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [95, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::BitXor", "path": "BitXor"}, "trait_path": "core::ops::bit::BitXor"}`

Source: `src/operation.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-412f2d1e339cfd6e8e018ef3"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [50, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/operation.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a791b58aad9ac0c9c4575c"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [68, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/operation.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-546c5764e21f0a6f1f4c6f6d"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [122, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/operation.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f15faf778e7439ec2444f7b"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [86, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::BitOr", "path": "BitOr"}, "trait_path": "core::ops::bit::BitOr"}`

Source: `src/operation.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a29c366b3fc0476176fa398"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [172, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::Not", "path": "Not"}, "trait_path": "core::ops::bit::Not"}`

Source: `src/operation.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0c602cdab85445d9cfa9735"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [59, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/operation.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b65094a1b59000dc06eb0f62"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [32, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/operation.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9407472e81b0628e8210d7b"></a>
## Output

`assoc_type` · `datafusion_expr::expr::Expr::Output` · datafusion-expr 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [77, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::BitAnd", "path": "BitAnd"}, "trait_path": "core::ops::bit::BitAnd"}`

Source: `src/operation.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-439f205dd6f1cd4001b2dc15"></a>
## Placeholder

`variant` · `datafusion_expr::expr::Expr::Placeholder` · datafusion-expr 55.1.0

```rust
Placeholder
```

Source: `src/expr.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A place holder for parameters in a prepared statement
(e.g. `$foo` or `$1`)

<a id="op-b60ee33d331f68f120313a58"></a>
## ScalarFunction

`variant` · `datafusion_expr::expr::Expr::ScalarFunction` · datafusion-expr 55.1.0

```rust
ScalarFunction
```

Source: `src/expr.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Call a scalar function with a set of arguments.

<a id="op-3d92059d3752418645a48ac8"></a>
## ScalarSubquery

`variant` · `datafusion_expr::expr::Expr::ScalarSubquery` · datafusion-expr 55.1.0

```rust
ScalarSubquery
```

Source: `src/expr.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Scalar subquery

<a id="op-9b5a5847402e3d0fa1f60243"></a>
## ScalarVariable

`variant` · `datafusion_expr::expr::Expr::ScalarVariable` · datafusion-expr 55.1.0

```rust
ScalarVariable
```

Source: `src/expr.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A named reference to a variable in a registry.

<a id="op-95fc7e189a2a6199ae7cee22"></a>
## SetComparison

`variant` · `datafusion_expr::expr::Expr::SetComparison` · datafusion-expr 55.1.0

```rust
SetComparison
```

Source: `src/expr.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set comparison subquery (e.g. `= ANY`, `> ALL`)

<a id="op-cf8e176c0a23dadb39411160"></a>
## SimilarTo

`variant` · `datafusion_expr::expr::Expr::SimilarTo` · datafusion-expr 55.1.0

```rust
SimilarTo
```

Source: `src/expr.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

LIKE expression that uses regular expressions

<a id="op-0251027e524caf4700df1901"></a>
## TryCast

`variant` · `datafusion_expr::expr::Expr::TryCast` · datafusion-expr 55.1.0

```rust
TryCast
```

Source: `src/expr.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Casts the expression to a given type and will return a null value if the expression cannot be cast.
This expression is guaranteed to have a fixed type.

<a id="op-3fe125763b5ea0320fac0299"></a>
## Unnest

`variant` · `datafusion_expr::expr::Expr::Unnest` · datafusion-expr 55.1.0

```rust
Unnest
```

Source: `src/expr.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Unnest expression

<a id="op-ea63f88ecb3f6e144471edcb"></a>
## Wildcard

`variant` · `datafusion_expr::expr::Expr::Wildcard` · datafusion-expr 55.1.0

```rust
Wildcard
```

Source: `src/expr.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents a reference to all available fields in a specific schema,
with an optional (schema) qualifier.

This expr has to be resolved to a list of columns before translating logical
plan into physical plan.

<a id="op-1ab47cd303f22d10dd7f4cec"></a>
## WindowFunction

`variant` · `datafusion_expr::expr::Expr::WindowFunction` · datafusion-expr 55.1.0

```rust
WindowFunction
```

Source: `src/expr.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Call a window function with a set of arguments.

<a id="op-f386b5dd0f967f3842e515ff"></a>
## add

`function` · `datafusion_expr::expr::Expr::add` · datafusion-expr 55.1.0

```rust
fn add(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [32, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/operation.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2a36735ecfdbc3e8cd4992f"></a>
## add_column_ref_counts

`function` · `datafusion_expr::expr::Expr::add_column_ref_counts` · datafusion-expr 55.1.0

```rust
fn add_column_ref_counts<'a>(&'a self, map: &mut HashMap<&'a Column, usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds references to all columns and their occurrence counts in the expression to
the map.

See [`Self::column_refs_counts`](../operations/datafusion_expr.expr.Expr.md#op-8ed374ff3159f149ff1ede2e) for details

<a id="op-2b02c2e196946528c6e2998e"></a>
## add_column_refs

`function` · `datafusion_expr::expr::Expr::add_column_refs` · datafusion-expr 55.1.0

```rust
fn add_column_refs<'a>(&'a self, set: &mut HashSet<&'a Column>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2091`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds references to all columns in this expression to the set

See [`Self::column_refs`](../operations/datafusion_expr.expr.Expr.md#op-300320e6a6eef529be8b394b) for details

<a id="op-b1bb2bd71918897567a64c11"></a>
## alias

`function` · `datafusion_expr::expr::Expr::alias` · datafusion-expr 55.1.0

```rust
fn alias(self, name: impl Into<String>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1807`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self AS name` alias expression

<a id="op-7473a20b1e4147dad6073a82"></a>
## alias_if_changed

`function` · `datafusion_expr::expr::Expr::alias_if_changed` · datafusion-expr 55.1.0

```rust
fn alias_if_changed(self, original_name: String) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1797`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Ensure `expr` has the name as `original_name` by adding an
alias if necessary.

<a id="op-7761b13247b0e22035285bcd"></a>
## alias_qualified

`function` · `datafusion_expr::expr::Expr::alias_qualified` · datafusion-expr 55.1.0

```rust
fn alias_qualified(self, relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1834`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self AS name` alias expression with a specific qualifier

<a id="op-683268a1bd9c0927020e4b3d"></a>
## alias_qualified_with_metadata

`function` · `datafusion_expr::expr::Expr::alias_qualified_with_metadata` · datafusion-expr 55.1.0

```rust
fn alias_qualified_with_metadata(self, relation: Option<impl Into<TableReference>>, name: impl Into<String>, metadata: Option<FieldMetadata>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1857`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self AS name` alias expression with a specific qualifier and metadata

The metadata will be attached to the Arrow Schema field when the expression
is converted to a field via `Expr.to_field()`.

# Example
```
# use datafusion_expr::col;
# use std::collections::HashMap;
# use datafusion_common::metadata::FieldMetadata;
let metadata = HashMap::from([("key".to_string(), "value".to_string())]);
let metadata = FieldMetadata::from(metadata);
let expr =
    col("foo").alias_qualified_with_metadata(Some("tbl"), "bar", Some(metadata));
```

<a id="op-8a4509eda5ab0f2fc66f8770"></a>
## alias_with_metadata

`function` · `datafusion_expr::expr::Expr::alias_with_metadata` · datafusion-expr 55.1.0

```rust
fn alias_with_metadata(self, name: impl Into<String>, metadata: Option<FieldMetadata>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1825`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self AS name` alias expression with metadata

The metadata will be attached to the Arrow Schema field when the expression
is converted to a field via `Expr.to_field()`.

# Example
```
# use datafusion_expr::col;
# use std::collections::HashMap;
# use datafusion_common::metadata::FieldMetadata;
let metadata = HashMap::from([("key".to_string(), "value".to_string())]);
let metadata = FieldMetadata::from(metadata);
let expr = col("foo").alias_with_metadata("bar", Some(metadata));
```

<a id="op-db49043f1d34ae62103d7f2b"></a>
## and

`function` · `datafusion_expr::expr::Expr::and` · datafusion-expr 55.1.0

```rust
fn and(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1743`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self && other`

<a id="op-adbea2d886420393acafb0a0"></a>
## any_column_refs

`function` · `datafusion_expr::expr::Expr::any_column_refs` · datafusion-expr 55.1.0

```rust
fn any_column_refs(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2137`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if there are any column references in this Expr

<a id="op-a68fc16e9ae439f9d76230e0"></a>
## apply_children

`function` · `datafusion_expr::expr::Expr::apply_children` · datafusion-expr 55.1.0

```rust
fn apply_children<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [351, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNode", "path": "TreeNode"}, "trait_path": "datafusion_common::tree_node::TreeNode"}`

Source: `src/tree_node.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Applies a function `f` to each child expression of `self`.

The function `f` determines whether to continue traversing the tree or to stop.
This method collects all child expressions and applies `f` to each.

<a id="op-3e7bfa058d8bfe96b0140d1f"></a>
## apply_elements

`function` · `datafusion_expr::expr::Expr::apply_elements` · datafusion-expr 55.1.0

```rust
fn apply_elements<F: FnMut(&'a Self) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 1], "end": [603, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/expr.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b634984d42a0d587955ca83"></a>
## as_literal

`function` · `datafusion_expr::expr::Expr::as_literal` · datafusion-expr 55.1.0

```rust
fn as_literal(&self) -> Option<&ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Check if the Expr is literal and get the literal value if it is.

<a id="op-b0eb2e4361d44a057137971e"></a>
## as_ref

`function` · `datafusion_expr::expr::Expr::as_ref` · datafusion-expr 55.1.0

```rust
fn as_ref(&self) -> &Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [555, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/expr.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cb40b2f788a68a4d66c6f4c"></a>
## between

`function` · `datafusion_expr::expr::Expr::between` · datafusion-expr 55.1.0

```rust
fn between(self, low: Expr, high: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2007`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

return `self BETWEEN low AND high`

<a id="op-de2acbdb636f7ebc988a829c"></a>
## bitand

`function` · `datafusion_expr::expr::Expr::bitand` · datafusion-expr 55.1.0

```rust
fn bitand(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [77, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::BitAnd", "path": "BitAnd"}, "trait_path": "core::ops::bit::BitAnd"}`

Source: `src/operation.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b191eecd8fa99bdd6f845f0f"></a>
## bitor

`function` · `datafusion_expr::expr::Expr::bitor` · datafusion-expr 55.1.0

```rust
fn bitor(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [86, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::BitOr", "path": "BitOr"}, "trait_path": "core::ops::bit::BitOr"}`

Source: `src/operation.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eed8655cfd8a01749d52bf87"></a>
## bitxor

`function` · `datafusion_expr::expr::Expr::bitxor` · datafusion-expr 55.1.0

```rust
fn bitxor(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [95, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::BitXor", "path": "BitXor"}, "trait_path": "core::ops::bit::BitXor"}`

Source: `src/operation.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3a6a9e524deb23d74dd2cf3"></a>
## can_normalize

`function` · `datafusion_expr::expr::Expr::can_normalize` · datafusion-expr 55.1.0

```rust
fn can_normalize(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2332, 1], "end": [2350, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "datafusion_common::cse::Normalizeable", "path": "Normalizeable"}, "trait_path": "datafusion_common::cse::Normalizeable"}`

Source: `src/expr.rs:2333`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b23e3232ac43bdf8143a83"></a>
## cast_to

`function` · `datafusion_expr::expr::Expr::cast_to` · datafusion-expr 55.1.0

```rust
fn cast_to(self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [749, 2], "filename": "src/expr_schema.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_schema::ExprSchemable", "path": "ExprSchemable"}, "trait_path": "datafusion_expr::expr_schema::ExprSchemable"}`

Source: `src/expr_schema.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Wraps this expression in a cast to a target [arrow::datatypes::DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).

# Errors

This function errors when it is impossible to cast the
expression to the target [arrow::datatypes::DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).

<a id="op-411583d028167c1fe948f461"></a>
## clone

`function` · `datafusion_expr::expr::Expr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 10], "end": [325, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-300320e6a6eef529be8b394b"></a>
## column_refs

`function` · `datafusion_expr::expr::Expr::column_refs` · datafusion-expr 55.1.0

```rust
fn column_refs(&self) -> HashSet<&Column>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2082`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all references to columns in this expression.

# Example
```
# use std::collections::HashSet;
# use datafusion_common::Column;
# use datafusion_expr::col;
// For an expression `a + (b * a)`
let expr = col("a") + (col("b") * col("a"));
let refs = expr.column_refs();
// refs contains "a" and "b"
assert_eq!(refs.len(), 2);
assert!(refs.contains(&Column::new_unqualified("a")));
assert!(refs.contains(&Column::new_unqualified("b")));
```

<a id="op-8ed374ff3159f149ff1ede2e"></a>
## column_refs_counts

`function` · `datafusion_expr::expr::Expr::column_refs_counts` · datafusion-expr 55.1.0

```rust
fn column_refs_counts(&self) -> HashMap<&Column, usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2116`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all references to columns and their occurrence counts in the expression.

# Example
```
# use std::collections::HashMap;
# use datafusion_common::Column;
# use datafusion_expr::col;
// For an expression `a + (b * a)`
let expr = col("a") + (col("b") * col("a"));
let mut refs = expr.column_refs_counts();
// refs contains "a" and "b"
assert_eq!(refs.len(), 2);
assert_eq!(*refs.get(&Column::new_unqualified("a")).unwrap(), 2);
assert_eq!(*refs.get(&Column::new_unqualified("b")).unwrap(), 1);
```

<a id="op-0ba6cce5388fe6fbdcec5520"></a>
## contains_outer

`function` · `datafusion_expr::expr::Expr::contains_outer` · datafusion-expr 55.1.0

```rust
fn contains_outer(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2143`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return true if the expression contains out reference(correlated) expressions.

<a id="op-9ea335d073afec36b262cf60"></a>
## contains_scalar_subquery

`function` · `datafusion_expr::expr::Expr::contains_scalar_subquery` · datafusion-expr 55.1.0

```rust
fn contains_scalar_subquery(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the expression contains a scalar subquery.

<a id="op-371e94f3dafa221cbcf82be7"></a>
## data_type_and_nullable

`function` · `datafusion_expr::expr::Expr::data_type_and_nullable` · datafusion-expr 55.1.0

```rust
fn data_type_and_nullable(&self, schema: &dyn ExprSchema) -> Result<(DataType, bool)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [749, 2], "filename": "src/expr_schema.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_schema::ExprSchemable", "path": "ExprSchemable"}, "trait_path": "datafusion_expr::expr_schema::ExprSchemable"}`

Source: `src/expr_schema.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the datatype and nullability of the expression based on [ExprSchema](../operations/datafusion_common.dfschema.ExprSchema.md#op-ffb72162f5de4ff12fd93126).

Note: [`DFSchema`] implements [ExprSchema](../operations/datafusion_common.dfschema.ExprSchema.md#op-ffb72162f5de4ff12fd93126).

[`DFSchema`]: datafusion_common::DFSchema

# Errors

This function errors when it is not possible to compute its
datatype or nullability.

<a id="op-62950bd3a23f84852f6af0b8"></a>
## default

`function` · `datafusion_expr::expr::Expr::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [549, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/expr.rs:546`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0920bc007acf831ab1ac7fe5"></a>
## distinct

`function` · `datafusion_expr::expr::Expr::distinct` · datafusion-expr 55.1.0

```rust
fn distinct(self) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [923, 1], "end": [1001, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:950`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7310fc1e7330d3d8c2fa448e"></a>
## div

`function` · `datafusion_expr::expr::Expr::div` · datafusion-expr 55.1.0

```rust
fn div(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [59, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/operation.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59a45dada06a89e58d6e83d7"></a>
## eq

`function` · `datafusion_expr::expr::Expr::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Expr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 17], "end": [325, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82bf82195dba24618320c392"></a>
## eq

`function` · `datafusion_expr::expr::Expr::eq` · datafusion-expr 55.1.0

```rust
fn eq(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1713`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self == other`

<a id="op-de6041fbb383e5f99a9eb485"></a>
## filter

`function` · `datafusion_expr::expr::Expr::filter` · datafusion-expr 55.1.0

```rust
fn filter(self, filter: Expr) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [923, 1], "end": [1001, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:939`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2240fdfa715fbcf8f8400534"></a>
## fmt

`function` · `datafusion_expr::expr::Expr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 44], "end": [325, 49], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2af58ee175b0dcacb0255db6"></a>
## fmt

`function` · `datafusion_expr::expr::Expr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3562, 1], "end": [3800, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:3563`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f62010fc07111903f9005e8"></a>
## from

`function` · `datafusion_expr::expr::Expr::from` · datafusion-expr 55.1.0

```rust
fn from(value: Column) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [558, 1], "end": [562, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:559`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4918db1f15e3618c0f9ad05"></a>
## from

`function` · `datafusion_expr::expr::Expr::from` · datafusion-expr 55.1.0

```rust
fn from(value: WindowFunction) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [569, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa84a1c69ec2143596578f75"></a>
## from

`function` · `datafusion_expr::expr::Expr::from` · datafusion-expr 55.1.0

```rust
fn from(value: ScalarAndMetadata) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [577, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0aa3775f47f0025b61b5313"></a>
## from

`function` · `datafusion_expr::expr::Expr::from` · datafusion-expr 55.1.0

```rust
fn from(value: (Option<&'a TableReference>, &'a FieldRef)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [587, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:584`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f413e8dfa1bb8f8980aeb7d"></a>
## get_as_join_column

`function` · `datafusion_expr::expr::Expr::get_as_join_column` · datafusion-expr 55.1.0

```rust
fn get_as_join_column(&self) -> Option<&Column>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2056`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the inner `Column` if any. This is a specialized version of
[`Self::try_as_col`](../operations/datafusion_expr.expr.Expr.md#op-c1bbc96c9fd900534493361e) that take Cast expressions into account when the
expression is as on condition for joins.

Called this method when you are sure that the expression is a `Column`
or a `Cast` expression that wraps a `Column`.

<a id="op-0bdfc2af21619f8ba58d0b83"></a>
## get_type

`function` · `datafusion_expr::expr::Expr::get_type` · datafusion-expr 55.1.0

```rust
fn get_type(&self, schema: &dyn ExprSchema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [749, 2], "filename": "src/expr_schema.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_schema::ExprSchemable", "path": "ExprSchemable"}, "trait_path": "datafusion_expr::expr_schema::ExprSchemable"}`

Source: `src/expr_schema.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the [arrow::datatypes::DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of the expression
based on [ExprSchema](../operations/datafusion_common.dfschema.ExprSchema.md#op-ffb72162f5de4ff12fd93126)

Note: [`DFSchema`] implements [ExprSchema](../operations/datafusion_common.dfschema.ExprSchema.md#op-ffb72162f5de4ff12fd93126).

[`DFSchema`]: datafusion_common::DFSchema

# Examples

Get the type of an expression that adds 2 columns. Adding an Int32
and Float32 results in Float32 type

```
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::DFSchema;
# use datafusion_expr::{col, ExprSchemable};
# use std::collections::HashMap;

fn main() {
    let expr = col("c1") + col("c2");
    let schema = DFSchema::from_unqualified_fields(
        vec![
            Field::new("c1", DataType::Int32, true),
            Field::new("c2", DataType::Float32, true),
        ]
        .into(),
        HashMap::new(),
    )
    .unwrap();
    assert_eq!("Float32", format!("{}", expr.get_type(&schema).unwrap()));
}
```

# Errors

This function errors when it is not possible to compute its
[arrow::datatypes::DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).  This happens when e.g. the
expression refers to a column that does not exist in the
schema, or when the expression is incorrectly typed
(e.g. `[utf8] + [bool]`).

<a id="op-859f6bc1fd827eab65e695ac"></a>
## gt

`function` · `datafusion_expr::expr::Expr::gt` · datafusion-expr 55.1.0

```rust
fn gt(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1723`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self > other`

<a id="op-cae4b981698630da991b1ba8"></a>
## gt_eq

`function` · `datafusion_expr::expr::Expr::gt_eq` · datafusion-expr 55.1.0

```rust
fn gt_eq(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1728`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self >= other`

<a id="op-543a651c65e54792fd778ad6"></a>
## hash

`function` · `datafusion_expr::expr::Expr::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 51], "end": [325, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08585e817376f15461b10e92"></a>
## hash_node

`function` · `datafusion_expr::expr::Expr::hash_node` · datafusion-expr 55.1.0

```rust
fn hash_node<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2748, 1], "end": [2934, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "datafusion_common::cse::HashNode", "path": "HashNode"}, "trait_path": "datafusion_common::cse::HashNode"}`

Source: `src/expr.rs:2752`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

As it is pretty easy to forget changing this method when `Expr` changes the
implementation doesn't use wildcard patterns (`..`, `_`) to catch changes
compile time.

<a id="op-3d5b1a93f5851b5885d840ce"></a>
## human_display

`function` · `datafusion_expr::expr::Expr::human_display` · datafusion-expr 55.1.0

```rust
fn human_display(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1628`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Human readable display formatting for this expression.

This function is primarily used in printing the explain tree output,
(e.g. `EXPLAIN FORMAT TREE <query>`), providing a readable format to
show how expressions are used in physical and logical plans. See the
[`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) for other ways to format expressions

Note this format is intended for human consumption rather than SQL for
other systems. If you need  SQL to pass to other systems, consider using
[`Unparser`].

[`Unparser`]: https://docs.rs/datafusion/latest/datafusion/sql/unparser/struct.Unparser.html

# Example
```
# use datafusion_expr::{col, lit};
let expr = col("foo") + lit(42);
// For EXPLAIN output:
// "foo + 42"
println!("{}", expr.human_display());
```

<a id="op-92b1dbef48157cc56fa3c8a0"></a>
## ilike

`function` · `datafusion_expr::expr::Expr::ilike` · datafusion-expr 55.1.0

```rust
fn ilike(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1775`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self ILIKE other`

<a id="op-a47c29e065eef72952c50485"></a>
## in_list

`function` · `datafusion_expr::expr::Expr::in_list` · datafusion-expr 55.1.0

```rust
fn in_list(self, list: Vec<Expr>, negated: bool) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1952`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self IN <list>` if `negated` is false, otherwise
return `self NOT IN <list>`.a

<a id="op-db4bc32e974f4349ede47fce"></a>
## infer_placeholder_types

`function` · `datafusion_expr::expr::Expr::infer_placeholder_types` · datafusion-expr 55.1.0

```rust
fn infer_placeholder_types(self, schema: &DFSchema) -> Result<(Expr, bool)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2183`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively find all [`Expr::Placeholder`](../operations/datafusion_expr.expr.Expr.md#op-439f205dd6f1cd4001b2dc15) expressions, and
to infer their [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) from the context of their use.

For example, given an expression like `<int32> = $0` will infer `$0` to
have type `int32`.

Returns transformed expression and flag that is true if expression contains
at least one placeholder.

<a id="op-4c9694e617afb9f8f1ba35a7"></a>
## is_false

`function` · `datafusion_expr::expr::Expr::is_false` · datafusion-expr 55.1.0

```rust
fn is_false(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1987`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsFalse(Box(self))`

<a id="op-644d2726ee7cab53b2d707eb"></a>
## is_not_false

`function` · `datafusion_expr::expr::Expr::is_not_false` · datafusion-expr 55.1.0

```rust
fn is_not_false(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1992`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsNotFalse(Box(self))`

<a id="op-581a43ea44199825883746dd"></a>
## is_not_null

`function` · `datafusion_expr::expr::Expr::is_not_null` · datafusion-expr 55.1.0

```rust
fn is_not_null(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1962`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsNotNull(Box(self))

<a id="op-85941c6b1b6a7c5e90823ff4"></a>
## is_not_true

`function` · `datafusion_expr::expr::Expr::is_not_true` · datafusion-expr 55.1.0

```rust
fn is_not_true(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1982`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsNotTrue(Box(self))`

<a id="op-202b2e6a4227f7da45ce36a0"></a>
## is_not_unknown

`function` · `datafusion_expr::expr::Expr::is_not_unknown` · datafusion-expr 55.1.0

```rust
fn is_not_unknown(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2002`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsNotUnknown(Box(self))`

<a id="op-65d6f8425fa1925962776059"></a>
## is_null

`function` · `datafusion_expr::expr::Expr::is_null` · datafusion-expr 55.1.0

```rust
fn is_null(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1957`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsNull(Box(self))

<a id="op-f3079452c03f1539ddbfa820"></a>
## is_true

`function` · `datafusion_expr::expr::Expr::is_true` · datafusion-expr 55.1.0

```rust
fn is_true(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1977`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsTrue(Box(self))`

<a id="op-aff7224621949f168202682a"></a>
## is_unknown

`function` · `datafusion_expr::expr::Expr::is_unknown` · datafusion-expr 55.1.0

```rust
fn is_unknown(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1997`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `IsUnknown(Box(self))`

<a id="op-1afe572f3af577b831f4db88"></a>
## is_volatile

`function` · `datafusion_expr::expr::Expr::is_volatile` · datafusion-expr 55.1.0

```rust
fn is_volatile(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2170`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the expression is volatile, i.e. whether it can return different
results when evaluated multiple times with the same input.

For example the function call `RANDOM()` is volatile as each call will
return a different value.

See [`Volatility`](../operations/datafusion_expr_common.signature.Volatility.md#op-2e27042945dba7db012302a0) for more information.

<a id="op-a0ce18985cdaf9d7baaffbe5"></a>
## is_volatile_node

`function` · `datafusion_expr::expr::Expr::is_volatile_node` · datafusion-expr 55.1.0

```rust
fn is_volatile_node(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2159`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the expression node is volatile, i.e. whether it can return
different results when evaluated multiple times with the same input.
Note: unlike [`Self::is_volatile`](../operations/datafusion_expr.expr.Expr.md#op-1afe572f3af577b831f4db88), this function does not consider inputs:
- `rand()` returns `true`,
- `a + rand()` returns `false`

<a id="op-234c4f5ebbea68082df4399e"></a>
## like

`function` · `datafusion_expr::expr::Expr::like` · datafusion-expr 55.1.0

```rust
fn like(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1753`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self LIKE other`

<a id="op-0297ef7de51f1fb3c87e5253"></a>
## lt

`function` · `datafusion_expr::expr::Expr::lt` · datafusion-expr 55.1.0

```rust
fn lt(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1733`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self < other`

<a id="op-b270e85adcefe4305da10f65"></a>
## lt_eq

`function` · `datafusion_expr::expr::Expr::lt_eq` · datafusion-expr 55.1.0

```rust
fn lt_eq(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1738`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self <= other`

<a id="op-82366d02b9182d414113f02c"></a>
## map_children

`function` · `datafusion_expr::expr::Expr::map_children` · datafusion-expr 55.1.0

```rust
fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [351, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNode", "path": "TreeNode"}, "trait_path": "datafusion_common::tree_node::TreeNode"}`

Source: `src/tree_node.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Maps each child of `self` using the provided closure `f`.

The closure `f` takes ownership of an expression and returns a `Transformed` result,
indicating whether the expression was transformed or left unchanged.

<a id="op-d071d0996b660dbb83026aa3"></a>
## map_elements

`function` · `datafusion_expr::expr::Expr::map_elements` · datafusion-expr 55.1.0

```rust
fn map_elements<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 1], "end": [603, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/expr.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1aa3068f0edd768a1af9c1e"></a>
## metadata

`function` · `datafusion_expr::expr::Expr::metadata` · datafusion-expr 55.1.0

```rust
fn metadata(&self, schema: &dyn ExprSchema) -> Result<FieldMetadata>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [749, 2], "filename": "src/expr_schema.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_schema::ExprSchemable", "path": "ExprSchemable"}, "trait_path": "datafusion_expr::expr_schema::ExprSchemable"}`

Source: `src/expr_schema.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f61c4e6b9fcbf8b9f41e42a8"></a>
## mul

`function` · `datafusion_expr::expr::Expr::mul` · datafusion-expr 55.1.0

```rust
fn mul(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [50, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/operation.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee0a09de51e10892adfcf95b"></a>
## name_for_alias

`function` · `datafusion_expr::expr::Expr::name_for_alias` · datafusion-expr 55.1.0

```rust
fn name_for_alias(&self) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1791`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the name to use for the specific Expr

<a id="op-f838a628e1331abdc67886a6"></a>
## neg

`function` · `datafusion_expr::expr::Expr::neg` · datafusion-expr 55.1.0

```rust
fn neg(self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [122, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/operation.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a15161cacdf872473dd3e833"></a>
## normalize_eq

`function` · `datafusion_expr::expr::Expr::normalize_eq` · datafusion-expr 55.1.0

```rust
fn normalize_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2352, 1], "end": [2746, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "datafusion_common::cse::NormalizeEq", "path": "NormalizeEq"}, "trait_path": "datafusion_common::cse::NormalizeEq"}`

Source: `src/expr.rs:2353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4837a70fc8119db69d6a0043"></a>
## not

`function` · `datafusion_expr::expr::Expr::not` · datafusion-expr 55.1.0

```rust
fn not(self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [172, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::Not", "path": "Not"}, "trait_path": "core::ops::bit::Not"}`

Source: `src/operation.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-849275fb33e0962487984a9a"></a>
## not_between

`function` · `datafusion_expr::expr::Expr::not_between` · datafusion-expr 55.1.0

```rust
fn not_between(self, low: Expr, high: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2017`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self NOT BETWEEN low AND high`

<a id="op-c2a2e8d601fb7362bf56066d"></a>
## not_eq

`function` · `datafusion_expr::expr::Expr::not_eq` · datafusion-expr 55.1.0

```rust
fn not_eq(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1718`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self != other`

<a id="op-2c3194b5b4b106448d124956"></a>
## not_ilike

`function` · `datafusion_expr::expr::Expr::not_ilike` · datafusion-expr 55.1.0

```rust
fn not_ilike(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1786`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self NOT ILIKE other`

<a id="op-057cc1f7b31c996413641254"></a>
## not_like

`function` · `datafusion_expr::expr::Expr::not_like` · datafusion-expr 55.1.0

```rust
fn not_like(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1764`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self NOT LIKE other`

<a id="op-076d7c6e577503ded403b551"></a>
## null_treatment

`function` · `datafusion_expr::expr::Expr::null_treatment` · datafusion-expr 55.1.0

```rust
fn null_treatment(self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [923, 1], "end": [1001, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:961`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a16d4502541b7797f669d2eb"></a>
## nullable

`function` · `datafusion_expr::expr::Expr::nullable` · datafusion-expr 55.1.0

```rust
fn nullable(&self, input_schema: &dyn ExprSchema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [749, 2], "filename": "src/expr_schema.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_schema::ExprSchemable", "path": "ExprSchemable"}, "trait_path": "datafusion_expr::expr_schema::ExprSchemable"}`

Source: `src/expr_schema.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the nullability of the expression based on [ExprSchema](../operations/datafusion_common.dfschema.ExprSchema.md#op-ffb72162f5de4ff12fd93126).

Note: [`DFSchema`] implements [ExprSchema](../operations/datafusion_common.dfschema.ExprSchema.md#op-ffb72162f5de4ff12fd93126).

[`DFSchema`]: datafusion_common::DFSchema

# Errors

This function errors when it is not possible to compute its
nullability.  This happens when the expression refers to a
column that does not exist in the schema.

<a id="op-38bca188f8c7ab4d2fd89670"></a>
## or

`function` · `datafusion_expr::expr::Expr::or` · datafusion-expr 55.1.0

```rust
fn or(self, other: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1748`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return `self || other`

<a id="op-8c1c4eed32bbbd6ed6ae7073"></a>
## order_by

`function` · `datafusion_expr::expr::Expr::order_by` · datafusion-expr 55.1.0

```rust
fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [923, 1], "end": [1001, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:924`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c202e7e09ed0972732d0dcd"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Expr::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Expr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 28], "end": [325, 38], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b52537419bf897fd109eb58b"></a>
## partition_by

`function` · `datafusion_expr::expr::Expr::partition_by` · datafusion-expr 55.1.0

```rust
fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [923, 1], "end": [1001, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:980`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-175adbd923e563b2ddad9a89"></a>
## placement

`function` · `datafusion_expr::expr::Expr::placement` · datafusion-expr 55.1.0

```rust
fn placement(&self) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1653`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns placement information for this expression.

This is used by optimizers to make decisions about expression placement,
such as whether to push expressions down through projections.

<a id="op-b60974dbd570a8be83ce1cb6"></a>
## qualified_name

`function` · `datafusion_expr::expr::Expr::qualified_name` · datafusion-expr 55.1.0

```rust
fn qualified_name(&self) -> (Option<TableReference>, String)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1637`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the qualifier and the schema name of this expression.

Used when the expression forms the output field of a certain plan.
The result is the field's qualifier and field name in the plan's
output schema. We can use this qualified name to reference the field.

<a id="op-85550de293940de92981759d"></a>
## rem

`function` · `datafusion_expr::expr::Expr::rem` · datafusion-expr 55.1.0

```rust
fn rem(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [68, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/operation.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a84a19ab1b8cbaf85e0d80f0"></a>
## resolve_lambda_variables

`function` · `datafusion_expr::expr::Expr::resolve_lambda_variables` · datafusion-expr 55.1.0

```rust
fn resolve_lambda_variables(self, schema: &DFSchema) -> Result<Transformed<Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2324`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `Expr` with all [`LambdaVariable`](../operations/datafusion_expr.expr.LambdaVariable.md#op-b6c77ba2103aa239605d5e16) resolved only if all of them
are contained in the subtree of the [`HigherOrderFunction`](../operations/datafusion_expr.expr.HigherOrderFunction.md#op-116039d54aad245a60edcd39) it originates from,
otherwise returns an error

<a id="op-82ef8179c0d4e6c2ae471137"></a>
## schema_name

`function` · `datafusion_expr::expr::Expr::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1603`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the column (field) that this `Expr` will produce.

For example, for a projection (e.g. `SELECT <expr>`) the resulting arrow
[`Schema`] will have a field with this name.

Note that the resulting string is subtlety different from the `Display`
representation for certain `Expr`. Some differences:

1. [`Expr::Alias`](../operations/datafusion_expr.expr.Expr.md#op-99ef95e1ad9855e695be1ebb), which shows only the alias itself
2. [`Expr::Cast`](../operations/datafusion_expr.expr.Expr.md#op-fe8aba365a7255f4059d5fc3) / [`Expr::TryCast`](../operations/datafusion_expr.expr.Expr.md#op-0251027e524caf4700df1901), which only displays the expression

# Example
```
# use datafusion_expr::{col, lit};
let expr = col("foo").eq(lit(42));
assert_eq!("foo = Int32(42)", expr.schema_name().to_string());

let expr = col("foo").alias("bar").eq(lit(11));
assert_eq!("bar = Int32(11)", expr.schema_name().to_string());
```

[`Schema`]: arrow::datatypes::Schema

<a id="op-8f863d39f6034d437985513d"></a>
## shl

`function` · `datafusion_expr::expr::Expr::shl` · datafusion-expr 55.1.0

```rust
fn shl(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [104, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::Shl", "path": "Shl"}, "trait_path": "core::ops::bit::Shl"}`

Source: `src/operation.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64def48cd426cd951283a673"></a>
## short_circuits

`function` · `datafusion_expr::expr::Expr::short_circuits` · datafusion-expr 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2250`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if some of this `exprs` subexpressions may not be evaluated
and thus any side effects (like divide by zero) may not be encountered

<a id="op-1f1ac847da9242e1944956d9"></a>
## shr

`function` · `datafusion_expr::expr::Expr::shr` · datafusion-expr 55.1.0

```rust
fn shr(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [113, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::bit::Shr", "path": "Shr"}, "trait_path": "core::ops::bit::Shr"}`

Source: `src/operation.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b348ae9f1bd1d0467164060"></a>
## sort

`function` · `datafusion_expr::expr::Expr::sort` · datafusion-expr 55.1.0

```rust
fn sort(self, asc: bool, nulls_first: bool) -> Sort
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1972`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a sort configuration from an existing expression.

```
# use datafusion_expr::col;
let sort_expr = col("foo").sort(true, true); // SORT ASC NULLS_FIRST
```

<a id="op-11f5088ba601bc8c0838cbf4"></a>
## spans

`function` · `datafusion_expr::expr::Expr::spans` · datafusion-expr 55.1.0

```rust
fn spans(&self) -> Option<&Spans>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2304`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a reference to the set of locations in the SQL query where this
expression appears, if known. [`None`] is returned if the expression
type doesn't support tracking locations yet.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-17444373900f8d0b8cbbb995"></a>
## sub

`function` · `datafusion_expr::expr::Expr::sub` · datafusion-expr 55.1.0

```rust
fn sub(self, rhs: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::expr::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [41, 2], "filename": "src/operation.rs"}, "trait": {"args": null, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/operation.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77306878ad632fdf80f4fb7d"></a>
## to_field

`function` · `datafusion_expr::expr::Expr::to_field` · datafusion-expr 55.1.0

```rust
fn to_field(&self, schema: &dyn ExprSchema) -> Result<(Option<TableReference>, Arc<Field>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [749, 2], "filename": "src/expr_schema.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_schema::ExprSchemable", "path": "ExprSchemable"}, "trait_path": "datafusion_expr::expr_schema::ExprSchemable"}`

Source: `src/expr_schema.rs:511`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a [arrow::datatypes::Field](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) compatible with this expression.

This function converts an expression into a field with appropriate metadata
and nullability based on the expression type and context. It is the primary
mechanism for determining field-level schemas.

# Field Property Resolution

For each expression, the following properties are determined:

## Data Type Resolution
- **Column references**: Data type from input schema field
- **Literals**: Data type inferred from literal value
- **Aliases**: Data type inherited from the underlying expression (the aliased expression)
- **Binary expressions**: Result type from type coercion rules
- **Boolean expressions**: Always a boolean type
- **Cast expressions**: Target data type from cast operation
- **Function calls**: Return type based on function signature and argument types

## Nullability Determination
- **Column references**: Inherit nullability from input schema field
- **Literals**: Nullable only if literal value is NULL
- **Aliases**: Inherit nullability from the underlying expression (the aliased expression)
- **Binary expressions**: Nullable if either operand is nullable
- **Boolean expressions**: Always non-nullable (IS NULL, EXISTS, etc.)
- **Cast expressions**: determined by the input expression's nullability rules
- **Function calls**: Based on function nullability rules and input nullability

## Metadata Handling
- **Column references**: Preserve original field metadata from input schema
- **Literals**: Use explicitly provided metadata, otherwise empty
- **Aliases**: Merge underlying expr metadata with alias-specific metadata, preferring the alias metadata
- **Binary expressions**: field metadata is empty
- **Boolean expressions**: field metadata is empty
- **Cast expressions**: Type-only casts pass through source metadata (stripping extension
  type keys); casts with explicit target fields use target metadata exactly
- **Scalar functions**: Generate metadata via function's [`return_field_from_args`] method,
  with the default implementation returning empty field metadata
- **Aggregate functions**: Generate metadata via function's [`return_field`] method,
  with the default implementation returning empty field metadata
- **Window functions**: field metadata follows the function's return field

## Table Reference Scoping
- Establishes proper qualified field references when columns belong to specific tables
- Maintains table context for accurate field resolution in multi-table scenarios

So for example, a projected expression `col(c1) + col(c2)` is
placed in an output field **named** col("c1 + c2")

[`return_field_from_args`]: crate::ScalarUDF::return_field_from_args
[`return_field`]: crate::AggregateUDF::return_field

<a id="op-c1bbc96c9fd900534493361e"></a>
## try_as_col

`function` · `datafusion_expr::expr::Expr::try_as_col` · datafusion-expr 55.1.0

```rust
fn try_as_col(&self) -> Option<&Column>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:2042`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a reference to the inner `Column` if any

returns `None` if the expression is not a `Column`

Note: None may be returned for expressions that are not `Column` but
are convertible to `Column` such as `Cast` expressions.

Example
```
# use datafusion_common::Column;
use datafusion_expr::{col, Expr};
let expr = col("foo");
assert_eq!(expr.try_as_col(), Some(&Column::from("foo")));

let expr = col("foo").alias("bar");
assert_eq!(expr.try_as_col(), None);
```

<a id="op-75604470c74ffd9170dc5f08"></a>
## unalias

`function` · `datafusion_expr::expr::Expr::unalias` · datafusion-expr 55.1.0

```rust
fn unalias(self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1886`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Remove an alias from an expression if one exists.

If the expression is not an alias, the expression is returned unchanged.
This method does not remove aliases from nested expressions.

# Example
```
# use datafusion_expr::col;
// `foo as "bar"` is unaliased to `foo`
let expr = col("foo").alias("bar");
assert_eq!(expr.unalias(), col("foo"));

// `foo as "bar" + baz` is not unaliased
let expr = col("foo").alias("bar") + col("baz");
assert_eq!(expr.clone().unalias(), expr);

// `foo as "bar" as "baz" is unaliased to foo as "bar"
let expr = col("foo").alias("bar").alias("baz");
assert_eq!(expr.unalias(), col("foo").alias("bar"));
```

<a id="op-bee24fb84b40197b9068116d"></a>
## unalias_nested

`function` · `datafusion_expr::expr::Expr::unalias_nested` · datafusion-expr 55.1.0

```rust
fn unalias_nested(self) -> Transformed<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1913`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively removed potentially multiple aliases from an expression.

This method removes nested aliases and returns [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21)
to signal if the expression was changed.

# Example
```
# use datafusion_expr::col;
// `foo as "bar"` is unaliased to `foo`
let expr = col("foo").alias("bar");
assert_eq!(expr.unalias_nested().data, col("foo"));

// `foo as "bar" + baz` is  unaliased
let expr = col("foo").alias("bar") + col("baz");
assert_eq!(expr.clone().unalias_nested().data, col("foo") + col("baz"));

// `foo as "bar" as "baz" is unalaised to foo
let expr = col("foo").alias("bar").alias("baz");
assert_eq!(expr.unalias_nested().data, col("foo"));
```

<a id="op-8216a0456f01ca0d4012d56e"></a>
## variant_name

`function` · `datafusion_expr::expr::Expr::variant_name` · datafusion-expr 55.1.0

```rust
fn variant_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [2330, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1669`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return String representation of the variant represented by `self`
Useful for non-rust based bindings

<a id="op-148ce49863365d661dc1e0b2"></a>
## window_frame

`function` · `datafusion_expr::expr::Expr::window_frame` · datafusion-expr 55.1.0

```rust
fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "crate::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [923, 1], "end": [1001, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:991`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
