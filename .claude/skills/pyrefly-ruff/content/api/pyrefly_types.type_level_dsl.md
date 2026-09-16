# `pyrefly_types::type_level_dsl`

Crate `pyrefly_types` · 29 public items · structured records in [`model/pyrefly_types.type_level_dsl.json`](../model/pyrefly_types.type_level_dsl.json)

## MAX_HELPER_CALL_DEPTH

`constant` · `pyrefly_types::type_level_dsl::MAX_HELPER_CALL_DEPTH`

```rust
const MAX_HELPER_CALL_DEPTH: usize = 32
```

---

## MAX_HELPER_GRAPH_EDGES

`constant` · `pyrefly_types::type_level_dsl::MAX_HELPER_GRAPH_EDGES`

```rust
const MAX_HELPER_GRAPH_EDGES: usize = 16384
```

---

## MAX_HELPER_GRAPH_NODES

`constant` · `pyrefly_types::type_level_dsl::MAX_HELPER_GRAPH_NODES`

```rust
const MAX_HELPER_GRAPH_NODES: usize = 4096
```

---

## DslIntegerDomain

`enum` · `pyrefly_types::type_level_dsl::DslIntegerDomain`

```rust
enum DslIntegerDomain
```

**Variants**: `Flag`, `Dimension`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

Which integer domain an integer-valued expression is evaluated in.

This is public because it is embedded in `TypeShapeDslExpressionKind`.

---

## TypeLevelDslFunction

`enum` · `pyrefly_types::type_level_dsl::TypeLevelDslFunction`

```rust
enum TypeLevelDslFunction
```

**Variants**: `IndexShape`, `UserDefined`, `MapIntTuples`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

The identity of a type-level DSL operation.

---

## TypeShapeDslArithmeticOp

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslArithmeticOp`

```rust
enum TypeShapeDslArithmeticOp
```

**Variants**: `Add`, `Subtract`, `Multiply`, `FloorDivide`, `Modulo`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

The arithmetic a structurally validated dimension or Flag expression applies. Reached through
`TypeShapeDslReturnKind` and `TypeShapeDslExpressionKind`, so it shares their identity
requirements.

---

## TypeShapeDslComparisonOp

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslComparisonOp`

```rust
enum TypeShapeDslComparisonOp
```

**Variants**: `Equal`, `NotEqual`, `LessThan`, `LessThanOrEqual`, `GreaterThan`, `GreaterThanOrEqual`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

The comparison a structurally validated DSL condition applies. `CmpOp` has no total order, so
the DSL records its own closed operator set and keeps evaluator matching exhaustive.

---

## TypeShapeDslConditionKind

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslConditionKind`

```rust
enum TypeShapeDslConditionKind
```

**Variants**: `Any`, `SlotCompare`, `IntegerCompare`, `DimensionEquality`, `GeneratorElementSelfCompare`, `IsConcreteInt`, `IsIntValue`, `IsNone`, `FlagIntCompare`, `BoolSlot`, `StringEquality`, `Membership`, `LengthEqualLiteral`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

What a structurally validated DSL condition tests. Like `TypeShapeDslReturnKind`, this depends
on intrinsic resolution, so it participates in `StructurallyValidatedTypeShapeDslFunction`
identity.

---

## TypeShapeDslDomain

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslDomain`

```rust
enum TypeShapeDslDomain
```

**Variants**: `Int`, `IntTuple`, `IntTuples`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(self) -> &'static str
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

---

## TypeShapeDslExpressionKind

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslExpressionKind`

```rust
enum TypeShapeDslExpressionKind
```

**Variants**: `IntTupleSlot`, `IntTupleSlice`, `IntTupleConcat`, `Einsum`, `GufuncBroadcast`, `DimensionSlot`, `IntegerSlot`, `DimensionLiteral`, `Gradual`, `IntTupleIndex`, `IntTuplesIndex`, `DimensionTuple`, `IntTupleConstructor`, `IntTuplesConstructor`, `IntTupleProduct`, `IntTupleSum`, `IntTupleLength`, `GeneratorSourceSlot`, `GeneratorElementAsDimension`, `GeneratorElementAsFlagInt`, `GeneratorElementAsIntTuple`, `GeneratorZip`, `Slot`, `FlagValueSlot`, `FlagIntLiteral`, `FlagStringLiteral`, `FlagBool`, `FlagNone`, `FlagTuple`, `Range`, `FlagSequenceLength`, `FlagSequenceCount`, `FlagSequenceIndex`, `FlagIntArithmetic`, `DimensionArithmetic`, `Conditional`, `DimensionGenerator`, `FlagGenerator`, `IntTuplesGenerator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

What a structurally validated DSL value expression computes. Like `TypeShapeDslReturnKind`,
this depends on intrinsic resolution, so it participates in
`StructurallyValidatedTypeShapeDslFunction` identity.

---

## TypeShapeDslFlagValueKind

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslFlagValueKind`

```rust
enum TypeShapeDslFlagValueKind
```

**Variants**: `Int`, `String`, `Sequence`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

The Flag value domain a validated operation requires of its operand. Reached through
`TypeShapeDslExpressionKind`, so it shares that type's identity requirements.

---

## TypeShapeDslHelperArgumentError

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslHelperArgumentError`

```rust
enum TypeShapeDslHelperArgumentError
```

**Variants**: `Arity`, `IncompatibleDomain`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## TypeShapeDslInputDomain

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslInputDomain`

```rust
enum TypeShapeDslInputDomain
```

**Variants**: `Value`, `OptionalInt`, `Flag`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn can_use_as(self, result: TypeShapeDslDomain, narrowing: TypeShapeDslParameterNarrowing) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

The type-system domain accepted by one DSL parameter.

`Value` represents a shape dimension or shape tuple. `Flag` represents literal-preserving
configuration values supplied by ordinary Python calls.

---

## TypeShapeDslIntrinsic

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslIntrinsic`

```rust
enum TypeShapeDslIntrinsic
```

**Variants**: `Any`, `Concat`, `Einsum`, `GufuncBroadcast`, `Gradual`, `IsConcreteInt`, `IsIntValue`, `IntTuple`, `IntTuples`, `Prod`, `Sum`, `Invalid`, `Len`, `Range`, `Tuple`, `Zip`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

A closed, canonical operation the DSL recognizes by callable identity rather than by spelling.

---

## TypeShapeDslParameterNarrowing

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslParameterNarrowing`

```rust
enum TypeShapeDslParameterNarrowing
```

**Variants**: `Unnarrowed`, `IsNone`, `NonNone`, `Integer`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn proves_not_none(self) -> bool
```

What control flow has established about a parameter at a particular use.

---

## TypeShapeDslProgramError

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslProgramError`

```rust
enum TypeShapeDslProgramError
```

**Variants**: `Cycle`, `Depth`, `NodeBudget`, `EdgeBudget`, `InconsistentDependency`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn message(self) -> &'static str
```

---

## TypeShapeDslReturnKind

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslReturnKind`

```rust
enum TypeShapeDslReturnKind
```

**Variants**: `Slot`, `Expression`, `Invalid`, `Gradual`, `HelperCall`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

The structurally validated source of a type-level shape DSL function's return value.
Resolving this depends on more than the AST, so it participates in
`StructurallyValidatedTypeShapeDslFunction` identity.

---

## TypeShapeDslSlotReturnKind

`enum` · `pyrefly_types::type_level_dsl::TypeShapeDslSlotReturnKind`

```rust
enum TypeShapeDslSlotReturnKind
```

**Variants**: `DirectParameter`, `ParameterAlias`, `KnownDomain`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

Validation information for a returned parameter or local slot.

---

## ParsedTypeShapeDslFunction

`struct` · `pyrefly_types::type_level_dsl::ParsedTypeShapeDslFunction`

```rust
struct ParsedTypeShapeDslFunction
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (9)

```rust
fn has_parameter_annotation(&self, index: usize) -> bool
fn has_return_annotation(&self) -> bool
fn name(&self) -> &Name
fn parameter_annotation_range(&self, index: usize) -> TextRange
fn parameter_count(&self) -> usize
fn parameter_name(&self, index: usize) -> &Name
fn return_annotation_range(&self) -> TextRange
fn try_new(definition: StmtFunctionDef, is_top_level: bool) -> Result<Self, TypeShapeDslDefinitionError>
fn validate(&self, intrinsic: impl Fn(&Expr) -> Option<TypeShapeDslIntrinsic>) -> Result<StructurallyValidatedTypeShapeDslFunction, TypeShapeDslDefinitionError>
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, _ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

A type-level shape DSL declaration whose envelope was validated during binding.

---

## ResolvedTypeShapeDslFunction

`struct` · `pyrefly_types::type_level_dsl::ResolvedTypeShapeDslFunction`

```rust
struct ResolvedTypeShapeDslFunction
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn contains_function(&self, id: &FuncDefId) -> bool
fn helper_graph_metrics(&self) -> (usize, usize, usize)
fn name(&self) -> &Name
fn parameter_domains(&self) -> &[TypeShapeDslInputDomain]
fn parameter_name(&self, index: usize) -> &Name
fn result_domain(&self) -> TypeShapeDslDomain
fn try_new(id: Arc<FuncDefId>, definition: Arc<StructurallyValidatedTypeShapeDslFunction>, parameter_domains: Vec<TypeShapeDslInputDomain>, result_domain: TypeShapeDslDomain, helpers: Vec<(Arc<FuncDefId>, Arc<Self>)>) -> Result<Self, TypeShapeDslProgramError>
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

A validated DSL definition paired with its resolved program.

---

## StructurallyValidatedTypeShapeDslFunction

`struct` · `pyrefly_types::type_level_dsl::StructurallyValidatedTypeShapeDslFunction`

```rust
struct StructurallyValidatedTypeShapeDslFunction
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn conditions(&self) -> impl Iterator<Item = TypeShapeDslCondition> + '_
fn expressions(&self) -> impl Iterator<Item = TypeShapeDslExpression> + '_
fn helper_calls(&self) -> impl Iterator<Item = &TypeShapeDslHelperCall>
fn name(&self) -> &Name
fn parameter_name(&self, index: usize) -> &Name
fn returns(&self) -> impl Iterator<Item = TypeShapeDslReturn> + '_
fn validate_with_resolved_domains(&self, intrinsic: impl Fn(&Expr) -> Option<TypeShapeDslIntrinsic>, parameter_domains: &[TypeShapeDslInputDomain], helper_argument_domains: Option<&[Vec<TypeShapeDslInputDomain>]>) -> Result<Self, TypeShapeDslDefinitionError>
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, _ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

An owned function AST whose restricted declaration syntax and body have been structurally
validated.
Future evaluation may interpret the definition relying on these invariants.

Identity is derived from the parsed program's pointer identity plus the resolved metadata. The
latter is required because resolving an intrinsic depends on imports outside this AST, so an
unedited declaration whose gradual constructor now resolves to a different domain is unequal.

---

## TypeLevelDslCall

`struct` · `pyrefly_types::type_level_dsl::TypeLevelDslCall`

```rust
struct TypeLevelDslCall
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
fn as_map_int_tuples(&self) -> Option<(&TypeLambda, &MapIntTuplesInterpretation, &Type)>
fn as_map_int_tuples_mut(&mut self) -> Option<&mut MapIntTuples>
fn evaluate(&self) -> Result<Type, ShapeError>
fn fallback(&self) -> Type
fn function_name(&self) -> &str
fn index_shape(shape: Type, index: Type) -> Self
fn map_int_tuples(mapper: TypeLambda, source: Type) -> Self
fn result_domain(&self) -> Option<TypeShapeDslDomain>
fn subst_parts_mut(&mut self, shadowed: &mut Vec<Quantified>, f: &mut dyn FnMut(&mut Type, &mut Vec<Quantified>))
fn type_for_generic_bound_check(&self) -> Type
fn user_defined(function: Arc<ResolvedTypeShapeDslFunction>, args: Vec<Type>) -> Self
fn visit_parts<'a>(&'a self, shadowed: &mut Vec<&'a Quantified>, f: &mut dyn FnMut(&'a Type, &mut Vec<&'a Quantified>))
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

A deferred type-level DSL invocation retained until the operation's result is consumed.

---

## TypeShapeDslComparisonOperand

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslComparisonOperand`

```rust
struct TypeShapeDslComparisonOperand
```

**Fields**: `parameter_uses`, `is_flag_operand`, `non_parameter_flag_domain`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

Information needed to validate one comparison operand after parameter domains are resolved.

---

## TypeShapeDslCondition

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslCondition`

```rust
struct TypeShapeDslCondition
```

**Implements**: `pyrefly_types::type_level_dsl::SourceRangeKey`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn kind(&self) -> &TypeShapeDslConditionKind
fn range(&self) -> TextRange
```

**via `pyrefly_types::type_level_dsl::SourceRangeKey`**

```rust
fn source_range_key(&self) -> TextRange
```

---

## TypeShapeDslDefinitionError

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslDefinitionError`

```rust
struct TypeShapeDslDefinitionError
```

**Fields**: `range`, `message`

**Derives**: Clone, Debug

---

## TypeShapeDslExpression

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslExpression`

```rust
struct TypeShapeDslExpression
```

**Implements**: `pyrefly_types::type_level_dsl::SourceRangeKey`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn kind(&self) -> &TypeShapeDslExpressionKind
fn range(&self) -> TextRange
```

**via `pyrefly_types::type_level_dsl::SourceRangeKey`**

```rust
fn source_range_key(&self) -> TextRange
```

---

## TypeShapeDslHelperCall

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslHelperCall`

```rust
struct TypeShapeDslHelperCall
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn argument_domains(&self, caller_domains: &[TypeShapeDslInputDomain], expected_domains: &[TypeShapeDslInputDomain], deferred_domains: &mut HashMap<usize, TypeShapeDslInputDomain>) -> Result<Vec<TypeShapeDslInputDomain>, TypeShapeDslHelperArgumentError>
fn callee(&self) -> &Expr
```

A syntactically valid helper call retained until ordinary name resolution is available.

DSL validation records the callee AST and each argument's shape-domain source. The solver then
resolves imports and aliases through normal function identity before attaching the resulting
helper program at the narrow boundary between Pyrefly's function model and the shape DSL.
Helper calls are valid only as return values, so evaluation follows a bounded chain of calls.

---

## TypeShapeDslParameterUse

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslParameterUse`

```rust
struct TypeShapeDslParameterUse
```

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn narrowing(self) -> TypeShapeDslParameterNarrowing
fn parameter(self) -> usize
```

A parameter that contributes to a DSL value, together with its narrowing at that use.

---

## TypeShapeDslReturn

`struct` · `pyrefly_types::type_level_dsl::TypeShapeDslReturn`

```rust
struct TypeShapeDslReturn
```

**Implements**: `pyrefly_types::type_level_dsl::SourceRangeKey`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn kind(&self) -> &TypeShapeDslReturnKind
fn range(&self) -> TextRange
```

**via `pyrefly_types::type_level_dsl::SourceRangeKey`**

```rust
fn source_range_key(&self) -> TextRange
```

---
