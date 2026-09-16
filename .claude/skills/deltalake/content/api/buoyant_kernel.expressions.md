# `buoyant_kernel::expressions`

Crate `buoyant_kernel` · 27 public items · structured records in [`model/buoyant_kernel.expressions.json`](../model/buoyant_kernel.expressions.json)

## BinaryExpressionOp

`enum` · `buoyant_kernel::expressions::BinaryExpressionOp`

Also reachable as `delta_kernel::expressions::BinaryExpressionOp`

```rust
enum BinaryExpressionOp
```

**Variants**: `Plus`, `Minus`, `Multiply`, `Divide`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A binary expression operator.

---

## BinaryPredicateOp

`enum` · `buoyant_kernel::expressions::BinaryPredicateOp`

Also reachable as `delta_kernel::expressions::BinaryPredicateOp`

```rust
enum BinaryPredicateOp
```

**Variants**: `LessThan`, `GreaterThan`, `Equal`, `Distinct`, `In`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A binary predicate operator.

---

## Expression

`enum` · `buoyant_kernel::expressions::Expression`

Also reachable as `buoyant_kernel::Expression`, `delta_kernel::expressions::Expression`

```rust
enum Expression
```

**Variants**: `Literal`, `Column`, `Predicate`, `Struct`, `StructPatch`, `Unary`, `Binary`, `Variadic`, `Opaque`, `Unknown`, `ParseJson`, `MapToStruct`

**Implements**: `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression`, `core::convert::From`, `core::fmt::Display`, `core::ops::arith::Add`, `core::ops::arith::Div`, `core::ops::arith::Mul`, `core::ops::arith::Sub`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (26)

```rust
fn array(exprs: impl IntoIterator<Item = impl Into<Expression>>) -> Self
fn binary(op: BinaryExpressionOp, lhs: impl Into<Expression>, rhs: impl Into<Expression>) -> Self
fn coalesce(exprs: impl IntoIterator<Item = impl Into<Expression>>) -> Self
fn column(field_names: impl CollectInto<ColumnName>) -> Expression
fn distinct(self, other: impl Into<Self>) -> Predicate
fn eq(self, other: impl Into<Self>) -> Predicate
fn from_pred(value: Predicate) -> Self
fn ge(self, other: impl Into<Self>) -> Predicate
fn gt(self, other: impl Into<Self>) -> Predicate
fn is_not_null(self) -> Predicate
fn is_null(self) -> Predicate
fn le(self, other: impl Into<Self>) -> Predicate
fn literal(value: impl Into<Scalar>) -> Self
fn lt(self, other: impl Into<Self>) -> Predicate
fn map_to_struct(map_expr: impl Into<Expression>) -> Self
fn ne(self, other: impl Into<Self>) -> Predicate
const fn null_literal(data_type: DataType) -> Self
fn opaque(op: impl OpaqueExpressionOp, exprs: impl IntoIterator<Item = Expression>) -> Self
fn parse_json(json_expr: impl Into<Expression>, output_schema: SchemaRef) -> Self
fn references(&self) -> HashSet<&ColumnName>
fn struct_from(exprs: impl IntoIterator<Item = impl Into<Arc<Self>>>) -> Self
fn struct_patch<P>(patch: P) -> DeltaResult<Self> where P: TryInto<ExpressionStructPatch>, Error: From<P::Error>
fn struct_with_nullability_from(exprs: impl IntoIterator<Item = impl Into<Arc<Self>>>, nullability_predicate: impl Into<Arc<Self>>) -> Self
fn unary(op: UnaryExpressionOp, expr: impl Into<Expression>) -> Self
fn unknown(name: impl Into<String>) -> Self
fn variadic(op: VariadicExpressionOp, exprs: impl IntoIterator<Item = impl Into<Expression>>) -> Self
```

**via `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression`**

```rust
fn arrow_opaque(op: impl ArrowOpaqueExpressionOp, exprs: impl IntoIterator<Item = Expression>) -> Expression
```

**via `core::convert::From`**

```rust
fn from(value: ColumnName) -> Self
fn from(value: Predicate) -> Self
fn from(value: Scalar) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: R) -> Self::Output
```

**via `core::ops::arith::Div`**

```rust
fn div(self, rhs: R) -> Self
```

**via `core::ops::arith::Mul`**

```rust
fn mul(self, rhs: R) -> Self
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, rhs: R) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A SQL expression.

These expressions do not track or validate data types, other than the type
of literals. It is up to the expression evaluator to validate the
expression against a schema and add appropriate casts as required.

---

## JunctionPredicateOp

`enum` · `buoyant_kernel::expressions::JunctionPredicateOp`

Also reachable as `delta_kernel::expressions::JunctionPredicateOp`

```rust
enum JunctionPredicateOp
```

**Variants**: `And`, `Or`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A junction (AND/OR) predicate operator.

---

## Predicate

`enum` · `buoyant_kernel::expressions::Predicate`

Also reachable as `buoyant_kernel::Predicate`, `delta_kernel::expressions::Predicate`

```rust
enum Predicate
```

**Variants**: `BooleanExpression`, `Not`, `Unary`, `Binary`, `Junction`, `Opaque`, `Unknown`

**Implements**: `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate`, `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (24)

```rust
fn and(a: impl Into<Self>, b: impl Into<Self>) -> Self
fn and_from(preds: impl IntoIterator<Item = Self>) -> Self
fn binary(op: BinaryPredicateOp, lhs: impl Into<Expression>, rhs: impl Into<Expression>) -> Self
fn column(field_names: impl CollectInto<ColumnName>) -> Predicate
fn distinct(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
fn eq(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
fn from_expr(expr: impl Into<Expression>) -> Self
fn ge(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
fn gt(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
fn is_not_null(expr: impl Into<Expression>) -> Predicate
fn is_null(expr: impl Into<Expression>) -> Predicate
fn junction(op: JunctionPredicateOp, preds: impl IntoIterator<Item = Self>) -> Self
fn le(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
const fn literal(value: bool) -> Self
fn lt(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
fn ne(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
fn not(pred: impl Into<Self>) -> Self
const fn null_literal() -> Self
fn opaque(op: impl OpaquePredicateOp, exprs: impl IntoIterator<Item = Expression>) -> Self
fn or(a: impl Into<Self>, b: impl Into<Self>) -> Self
fn or_from(preds: impl IntoIterator<Item = Self>) -> Self
fn references(&self) -> HashSet<&ColumnName>
fn unary(op: UnaryPredicateOp, expr: impl Into<Expression>) -> Self
fn unknown(name: impl Into<String>) -> Self
```

**via `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate`**

```rust
fn arrow_opaque<T: ArrowOpaquePredicateOp>(op: T, exprs: impl IntoIterator<Item = Expression>) -> Predicate
```

**via `core::convert::From`**

```rust
fn from(value: ColumnName) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A SQL predicate.

These predicates do not track or validate data types, other than the type
of literals. It is up to the predicate evaluator to validate the
predicate against a schema and add appropriate casts as required.

---

## UnaryExpressionOp

`enum` · `buoyant_kernel::expressions::UnaryExpressionOp`

Also reachable as `delta_kernel::expressions::UnaryExpressionOp`

```rust
enum UnaryExpressionOp
```

**Variants**: `ToJson`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A unary expression operator.

---

## UnaryPredicateOp

`enum` · `buoyant_kernel::expressions::UnaryPredicateOp`

Also reachable as `delta_kernel::expressions::UnaryPredicateOp`

```rust
enum UnaryPredicateOp
```

**Variants**: `IsNull`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A unary predicate operator.

---

## VariadicExpressionOp

`enum` · `buoyant_kernel::expressions::VariadicExpressionOp`

Also reachable as `delta_kernel::expressions::VariadicExpressionOp`

```rust
enum VariadicExpressionOp
```

**Variants**: `Coalesce`, `Array`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A variadic expression operator.

---

## lit

`function` · `buoyant_kernel::expressions::lit`

Also reachable as `delta_kernel::expressions::lit`

```rust
fn lit(value: impl Into<Scalar>) -> Expression
```

Build an [`Expression::Literal`] from anything that converts into a [`Scalar`].

Concise alternative to [`Expression::literal`] for plan builders. Accepts the same value
types [`Scalar`] does (`i32`, `i64`, `&str`, `bool`, ...).

```
# use buoyant_kernel as delta_kernel;
use delta_kernel::expressions::lit;
let _zero = lit(0i64);
```

---

## BinaryExpression

`struct` · `buoyant_kernel::expressions::BinaryExpression`

Also reachable as `delta_kernel::expressions::BinaryExpression`

```rust
struct BinaryExpression
```

**Fields**: `op`, `left`, `right`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## BinaryPredicate

`struct` · `buoyant_kernel::expressions::BinaryPredicate`

Also reachable as `delta_kernel::expressions::BinaryPredicate`

```rust
struct BinaryPredicate
```

**Fields**: `op`, `left`, `right`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## JunctionPredicate

`struct` · `buoyant_kernel::expressions::JunctionPredicate`

Also reachable as `delta_kernel::expressions::JunctionPredicate`

```rust
struct JunctionPredicate
```

**Fields**: `op`, `preds`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MapToStructExpression

`struct` · `buoyant_kernel::expressions::MapToStructExpression`

Also reachable as `delta_kernel::expressions::MapToStructExpression`

```rust
struct MapToStructExpression
```

**Fields**: `map_expr`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Transforms a `Map<String, String>` column into a struct whose schema is provided by the
evaluator's output type (via `result_type`). Each row in the map column becomes one row in
the output struct column: a `key` -> `value` mapping in the map means the struct field named
`key` receives `value`, parsed into the field's target type via [`PrimitiveType::parse_scalar`].
An empty-string value is the exception (aligning with Spark): it casts to itself for string, to
empty bytes for binary, and to null for every other type.

- Missing keys produce null values
- Parse errors are propagated (indicating a broken table)
- Duplicate map keys are resolved by taking the rightmost entry

[`PrimitiveType::parse_scalar`]: crate::schema::PrimitiveType::parse_scalar

---

## OpaqueExpression

`struct` · `buoyant_kernel::expressions::OpaqueExpression`

Also reachable as `delta_kernel::expressions::OpaqueExpression`

```rust
struct OpaqueExpression
```

**Fields**: `op`, `exprs`

**Derives**: Clone, Debug, PartialEq

---

## OpaquePredicate

`struct` · `buoyant_kernel::expressions::OpaquePredicate`

Also reachable as `delta_kernel::expressions::OpaquePredicate`

```rust
struct OpaquePredicate
```

**Fields**: `op`, `exprs`

**Derives**: Clone, Debug, PartialEq

---

## ParseJsonExpression

`struct` · `buoyant_kernel::expressions::ParseJsonExpression`

Also reachable as `delta_kernel::expressions::ParseJsonExpression`

```rust
struct ParseJsonExpression
```

**Fields**: `json_expr`, `output_schema`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An expression that parses a JSON string into a struct with the given schema.
This is the inverse of `ToJson` - it converts a JSON-encoded string column into a
struct column.

---

## UnaryExpression

`struct` · `buoyant_kernel::expressions::UnaryExpression`

Also reachable as `delta_kernel::expressions::UnaryExpression`

```rust
struct UnaryExpression
```

**Fields**: `op`, `expr`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## UnaryPredicate

`struct` · `buoyant_kernel::expressions::UnaryPredicate`

Also reachable as `delta_kernel::expressions::UnaryPredicate`

```rust
struct UnaryPredicate
```

**Fields**: `op`, `expr`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## VariadicExpression

`struct` · `buoyant_kernel::expressions::VariadicExpression`

Also reachable as `delta_kernel::expressions::VariadicExpression`

```rust
struct VariadicExpression
```

**Fields**: `op`, `exprs`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## OpaqueExpressionOp

`trait` · `buoyant_kernel::expressions::OpaqueExpressionOp`

Also reachable as `delta_kernel::expressions::OpaqueExpressionOp`

```rust
trait OpaqueExpressionOp: DynPartialEq + std::fmt::Debug
```

**Implementors** (1)

- `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOpAdaptor`

**Methods** (2)

```rust
fn eval_expr_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, exprs: &[Expression]) -> DeltaResult<Scalar>
fn name(&self) -> &str
```

An opaque expression operation (ie defined and implemented by the engine).

---

## OpaquePredicateOp

`trait` · `buoyant_kernel::expressions::OpaquePredicateOp`

Also reachable as `delta_kernel::expressions::OpaquePredicateOp`

```rust
trait OpaquePredicateOp: DynPartialEq + std::fmt::Debug
```

**Implementors** (1)

- `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOpAdaptor`

**Methods** (4)

```rust
fn as_data_skipping_predicate(&self, evaluator: &IndirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<Predicate>
fn eval_as_data_skipping_predicate(&self, evaluator: &DirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<bool>
fn eval_pred_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, eval_pred: &DirectPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> DeltaResult<Option<bool>>
fn name(&self) -> &str
```

An opaque predicate operation (ie defined and implemented by the engine).

---

## ExpressionRef

`type_alias` · `buoyant_kernel::expressions::ExpressionRef`

Also reachable as `buoyant_kernel::ExpressionRef`, `delta_kernel::expressions::ExpressionRef`

```rust
type ExpressionRef = std::sync::Arc<Expression>
```

**Implements**: `buoyant_kernel::struct_patch::ExpressionItem`

**via `buoyant_kernel::struct_patch::ExpressionItem`**

```rust
fn expr(&self) -> &ExpressionRef
```

---

## ExpressionStructPatchBuilder

`type_alias` · `buoyant_kernel::expressions::ExpressionStructPatchBuilder`

Also reachable as `delta_kernel::expressions::ExpressionStructPatchBuilder`

```rust
type ExpressionStructPatchBuilder = struct_patch::StructPatchBuilder<ExpressionRef>
```

A [`StructPatchBuilder`](crate::struct_patch::StructPatchBuilder) whose emitted items are
expressions, lowered into an [`ExpressionStructPatch`] that can be embedded in an
[`Expression`].

---

## OpaqueExpressionOpRef

`type_alias` · `buoyant_kernel::expressions::OpaqueExpressionOpRef`

Also reachable as `delta_kernel::expressions::OpaqueExpressionOpRef`

```rust
type OpaqueExpressionOpRef = std::sync::Arc<dyn OpaqueExpressionOp>
```

A shared reference to an [`OpaqueExpressionOp`] instance.

---

## OpaquePredicateOpRef

`type_alias` · `buoyant_kernel::expressions::OpaquePredicateOpRef`

Also reachable as `delta_kernel::expressions::OpaquePredicateOpRef`

```rust
type OpaquePredicateOpRef = std::sync::Arc<dyn OpaquePredicateOp>
```

A shared reference to an [`OpaquePredicateOp`] instance.

---

## PredicateRef

`type_alias` · `buoyant_kernel::expressions::PredicateRef`

Also reachable as `buoyant_kernel::PredicateRef`, `delta_kernel::expressions::PredicateRef`

```rust
type PredicateRef = std::sync::Arc<Predicate>
```

---

## ScalarExpressionEvaluator

`type_alias` · `buoyant_kernel::expressions::ScalarExpressionEvaluator`

Also reachable as `delta_kernel::expressions::ScalarExpressionEvaluator`

```rust
type ScalarExpressionEvaluator<'a> = dyn Fn(&Expression) -> Option<Scalar> + 'a
```

A kernel-supplied scalar expression evaluator which in particular can convert column references
(i.e. [`Expression::Column`]) to [`Scalar`] values. [`OpaqueExpressionOp::eval_expr_scalar`] and
[`OpaquePredicateOp::eval_pred_scalar`] rely on this evaluator.

If the evaluator produces `None`, it means kernel was unable to evaluate
the input expression. Otherwise, `Some(Scalar)` is the result of that evaluation (possibly
`Scalar::Null` if the output was NULL).

---
