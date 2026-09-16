# `pyrefly::binding::binding`

Crate `pyrefly` · 105 public items · structured records in [`model/pyrefly.binding.binding.json`](../model/pyrefly.binding.binding.json)

## AnnAssignHasValue

`enum` · `pyrefly::binding::binding::AnnAssignHasValue`

```rust
enum AnnAssignHasValue
```

**Variants**: `Yes`, `No`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

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

Does an AnnAssign defining an Annotation have a value? Used to validate
some qualifiers like `Final` that require an initial value.

---

## AnnotationStyle

`enum` · `pyrefly::binding::binding::AnnotationStyle`

```rust
enum AnnotationStyle
```

**Variants**: `Direct`, `ForwardedInitial`, `Forwarded`

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## AnnotationTarget

`enum` · `pyrefly::binding::binding::AnnotationTarget`

```rust
enum AnnotationTarget
```

**Variants**: `Param`, `ArgsParam`, `KwargsParam`, `Return`, `Assign`, `AttrAssign`, `ClassMember`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn type_form_context(&self) -> TypeFormContext<'static>
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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## AnyExportedKey

`enum` · `pyrefly::binding::binding::AnyExportedKey`

```rust
enum AnyExportedKey
```

**Variants**: `KeyTParams`, `KeyClassBaseType`, `KeyClassField`, `KeyClassSynthesizedFields`, `KeyVariance`, `KeyExport`, `KeyClassMetadata`, `KeyDjangoRelations`, `KeyClassMro`, `KeyClassDisjointBase`, `KeyAbstractClassCheck`, `KeyClassSubscriptSymmetry`, `KeyTypeAlias`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

A type-erased exported key, used for fine-grained dependency tracking.
Unlike `AnyIdx`, this stores the key itself rather than an index into a bindings table.

---

## AnyIdx

`enum` · `pyrefly::binding::binding::AnyIdx`

```rust
enum AnyIdx
```

**Variants**: `Key`, `KeyExpect`, `KeyTypeAlias`, `KeyClass`, `KeyTParams`, `KeyClassBaseType`, `KeyClassField`, `KeyVariance`, `KeyClassChecks`, `KeyClassSynthesizedFields`, `KeyExport`, `KeyDecorator`, `KeyDecoratedFunction`, `KeyUndecoratedFunction`, `KeyUndecoratedFunctionRange`, `KeyAnnotation`, `KeyClassMetadata`, `KeyDjangoRelations`, `KeyClassMro`, `KeyClassDisjointBase`, `KeyAbstractClassCheck`, `KeyClassSubscriptSymmetry`, `KeyLegacyTypeParam`, `KeyYield`, `KeyYieldFrom`

**Implements**: `dupe::Dupe`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## Binding

`enum` · `pyrefly::binding::binding::Binding`

```rust
enum Binding
```

**Variants**: `Expr`, `StmtExpr`, `MultiTargetAssign`, `TypeVar`, `ParamSpec`, `TypeVarTuple`, `ReturnExplicit`, `ReturnImplicit`, `ReturnType`, `IterableValueComprehension`, `IterableValueLoop`, `ContextValue`, `UnpackedValue`, `AnnotatedType`, `AugAssign`, `None`, `Any`, `Global`, `TypeParameter`, `OuterClassTypeParameter`, `Function`, `Import`, `ClassDef`, `Forward`, `PatternCapture`, `PromoteForward`, `ForwardToFirstUse`, `Phi`, `LoopPhi`, `Narrow`, `Module`, `PossibleLegacyTParam`, `NameAssign`, `TypeAlias`, `TypeAliasRef`, `PatternMatchMapping`, `PatternMatchClassPositional`, `PatternMatchClassKeyword`, `ExceptionHandler`, `LambdaParameter`, `TypeLevelLambdaParameter`, `FunctionParameter`, `SuperInstance`, `AssignToAttribute`, `AssignToSubscript`, `UsageLink`, `Delete`, `ClassBodyUnknownName`, `Exhaustive`, `SuppressedException`, `Sentinel`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn symbol_kind(&self) -> Option<SymbolKind>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingAnnotation

`enum` · `pyrefly::binding::binding::BindingAnnotation`

```rust
enum BindingAnnotation
```

**Variants**: `AnnotateExpr`, `SpecialForm`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Values that return an annotation.

---

## BindingClass

`enum` · `pyrefly::binding::binding::BindingClass`

```rust
enum BindingClass
```

**Variants**: `ClassDef`, `FunctionalClassDef`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &Bindings) -> fmt::Result
```

Binding for a class.

---

## BindingExpect

`enum` · `pyrefly::binding::binding::BindingExpect`

```rust
enum BindingExpect
```

**Variants**: `TypeCheckExpr`, `TypeCheckBaseClassExpr`, `UnpackedLength`, `CheckRaisedException`, `Redefinition`, `ValidateImplicitReturn`, `Bool`, `MatchExhaustiveness`, `MatchCaseReachability`, `PrivateAttributeAccess`, `UninitializedCheck`, `ForwardRefUnion`, `ImplicitAliasCheck`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingExport

`enum` · `pyrefly::binding::binding::BindingExport`

```rust
enum BindingExport
```

**Variants**: `Forward`, `PromoteForward`, `AnnotatedForward`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn forward_maybe_promote(idx: Idx<Key>, name: &Name) -> Self
fn key_idx(&self) -> Idx<Key>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

An export binding. Every export is a forward reference to a `Key`, optionally
with an annotation.

---

## BindingLegacyTypeParam

`enum` · `pyrefly::binding::binding::BindingLegacyTypeParam`

```rust
enum BindingLegacyTypeParam
```

**Variants**: `ParamKeyed`, `ModuleKeyed`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn idx(&self) -> Idx<Key>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

A legacy type parameter (`T = typing.TypeVar("T")`).

---

## BindingTypeAlias

`enum` · `pyrefly::binding::binding::BindingTypeAlias`

```rust
enum BindingTypeAlias
```

**Variants**: `Legacy`, `Scoped`, `TypeAliasType`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &Bindings) -> fmt::Result
```

---

## BindingYield

`enum` · `pyrefly::binding::binding::BindingYield`

```rust
enum BindingYield
```

**Variants**: `Yield`, `Invalid`, `Unreachable`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingYieldFrom

`enum` · `pyrefly::binding::binding::BindingYieldFrom`

```rust
enum BindingYieldFrom
```

**Variants**: `YieldFrom`, `Invalid`, `Unreachable`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## ClassFieldDefinition

`enum` · `pyrefly::binding::binding::ClassFieldDefinition`

```rust
enum ClassFieldDefinition
```

**Variants**: `DeclaredByAnnotation`, `DeclaredWithoutAnnotation`, `AssignedInBody`, `MethodLike`, `NestedClass`, `DefinedWithoutAssign`, `DefinedInMethod`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Represents everything we know about a class field definition at binding time.

---

## ExhaustivenessKind

`enum` · `pyrefly::binding::binding::ExhaustivenessKind`

```rust
enum ExhaustivenessKind
```

**Variants**: `Match`, `IfElif`, `ClassPatternCoverage`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Distinguishes between different kinds of exhaustiveness checking.

---

## ExprOrBinding

`enum` · `pyrefly::binding::binding::ExprOrBinding`

```rust
enum ExprOrBinding
```

**Variants**: `Expr`, `Binding`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## FirstUse

`enum` · `pyrefly::binding::binding::FirstUse`

```rust
enum FirstUse
```

**Variants**: `Undetermined`, `DoesNotPin`, `UsedBy`

**Derives**: Clone, Debug

---

## FunctionParameter

`enum` · `pyrefly::binding::binding::FunctionParameter`

```rust
enum FunctionParameter
```

**Variants**: `Annotated`, `Unannotated`

**Derives**: Clone, Debug

A function parameter, either annotated or unannotated.
Unannotated function params must be resolved to a type before they are used, when
solving UndecoratedFunction, and will never resolve to a type based on their use.

---

## IsAsync

`enum` · `pyrefly::binding::binding::IsAsync`

```rust
enum IsAsync
```

**Variants**: `Sync`, `Async`

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn context_exit_dunder(self) -> Name
fn is_async(self) -> bool
fn new(is_async: bool) -> Self
```

---

## Key

`enum` · `pyrefly::binding::binding::Key`

```rust
enum Key
```

**Variants**: `Import`, `ImplicitGlobal`, `Definition`, `MutableCapture`, `FacetAssign`, `ReturnExplicit`, `ReturnImplicit`, `ReturnType`, `BoundName`, `Anon`, `InvalidTarget`, `PatternNarrow`, `StmtExpr`, `ContextExpr`, `ContextValue`, `MatchSubject`, `Phi`, `Narrow`, `Anywhere`, `SuperInstance`, `Unpack`, `UsageLink`, `YieldLink`, `PossibleLegacyTParam`, `Delete`, `Exhaustive`, `SuppressedException`

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn check_shortcut(answers: &AnswersSolver<'_, '_, Ans>, binding: &Binding) -> Option<TypeInfo>
fn create_recursive(answers: &AnswersSolver<'_, '_, Ans>, binding: &Self::Value) -> Var
fn promote_recursive(_heap: &TypeHeap, x: Var) -> Self::Answer
fn record_recursive(answers: &AnswersSolver<'_, '_, Ans>, answer: TypeInfo, recursive: Var) -> TypeInfo
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &Binding, range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Keys that refer to a `Type`.

---

## KeyAnnotation

`enum` · `pyrefly::binding::binding::KeyAnnotation`

```rust
enum KeyAnnotation
```

**Variants**: `Annotation`, `ReturnAnnotation`, `AttrAnnotation`

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingAnnotation, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Keys that refer to an `Annotation`.

---

## KeyExpect

`enum` · `pyrefly::binding::binding::KeyExpect`

```rust
enum KeyExpect
```

**Variants**: `TypeCheckExpr`, `TypeCheckBaseClassExpr`, `UnpackedLength`, `CheckRaisedException`, `Redefinition`, `Bool`, `MatchExhaustiveness`, `MatchCaseReachability`, `PrivateAttributeAccess`, `UninitializedCheck`, `ForwardRefUnion`, `ImplicitAliasCheck`, `ValidateImplicitReturn`

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingExpect, range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

An expectation to be checked. For example, that a sequence is of an expected length.

This is an enum to ensure that different kinds of expectations at the same source
location don't collide. Each variant represents a distinct category of expectation.

---

## LambdaKind

`enum` · `pyrefly::binding::binding::LambdaKind`

```rust
enum LambdaKind
```

**Variants**: `Ordinary`, `TypeLevel`

**Derives**: Clone, Copy, Debug

Which rule a lambda's parameters follow, decided while its enclosing expression is bound.

---

## LastStmt

`enum` · `pyrefly::binding::binding::LastStmt`

```rust
enum LastStmt
```

**Variants**: `Expr`, `With`, `Exhaustive`

**Derives**: Clone, Debug

---

## LinkedKey

`enum` · `pyrefly::binding::binding::LinkedKey`

```rust
enum LinkedKey
```

**Variants**: `Yield`, `YieldFrom`, `Expect`

**Derives**: Clone, Debug

Represents an `Idx<K>` for some `K: Keyed` other than `Key`
that we want to track for first-usage type inference.

---

## MethodSelfKind

`enum` · `pyrefly::binding::binding::MethodSelfKind`

```rust
enum MethodSelfKind
```

**Variants**: `Instance`, `Class`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## NarrowUseLocation

`enum` · `pyrefly::binding::binding::NarrowUseLocation`

```rust
enum NarrowUseLocation
```

**Variants**: `Span`, `Start`, `End`

**Implements**: `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Location at which a narrowing operation is used. We've seen the same narrowing operation be
used at the same text range up to three times, so we use this enum to mark those three uses
as distinct locations to avoid generating duplicate keys. It doesn't really matter whether a
particular location is marked as Span, Start, or End as long as we never have duplicates, but
generally, Start is used for an operation that happens before the main operation (e.g.,
negating the narrows from one branch of an if/else at the start of the next), Span is used
for the main operation, and End is used for an operation that happens afterwards (e.g.,
merging flow at the end of a fork).

---

## RaisedException

`enum` · `pyrefly::binding::binding::RaisedException`

```rust
enum RaisedException
```

**Variants**: `WithoutCause`, `WithCause`

**Derives**: Clone, Debug

---

## ReturnTypeKind

`enum` · `pyrefly::binding::binding::ReturnTypeKind`

```rust
enum ReturnTypeKind
```

**Variants**: `ShouldTrustAnnotation`, `ShouldReturnAny`, `ShouldInferType`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn has_return_annotation(&self) -> bool
fn should_infer_return(&self) -> bool
```

---

## SizeExpectation

`enum` · `pyrefly::binding::binding::SizeExpectation`

```rust
enum SizeExpectation
```

**Variants**: `Eq`, `Ge`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn message(&self) -> String
```

---

## SuperStyle

`enum` · `pyrefly::binding::binding::SuperStyle`

```rust
enum SuperStyle
```

**Variants**: `ExplicitArgs`, `ImplicitArgs`, `Any`

**Derives**: Clone, Debug

---

## TypeAliasParams

`enum` · `pyrefly::binding::binding::TypeAliasParams`

```rust
enum TypeAliasParams
```

**Variants**: `Legacy`, `Scoped`, `TypeAliasType`

**Derives**: Clone, Debug

---

## UnpackedPosition

`enum` · `pyrefly::binding::binding::UnpackedPosition`

```rust
enum UnpackedPosition
```

**Variants**: `ExactIndex`, `Index`, `ReverseIndex`, `Slice`

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug

A target position within an unpacking assignment or sequence pattern. The second
`usize` of each index variant is the source length the target list requires: the
exact length for `ExactIndex`, or the minimum for `Index`/`ReverseIndex`.

---

## AnnotationWithTarget

`struct` · `pyrefly::binding::binding::AnnotationWithTarget`

```rust
struct AnnotationWithTarget
```

**Fields**: `target`, `annotation`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn ty(&self, heap: &TypeHeap, stdlib: &Stdlib) -> Option<Type>
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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## AssignToAttribute

`struct` · `pyrefly::binding::binding::AssignToAttribute`

```rust
struct AssignToAttribute
```

**Fields**: `attr`, `value`, `allow_assign_to_final`

**Derives**: Clone, Debug

Data for assigning to an attribute.

---

## AttrsSpecifier

`struct` · `pyrefly::binding::binding::AttrsSpecifier`

```rust
struct AttrsSpecifier
```

**Fields**: `kind`, `class_def_index`

**Derives**: Clone, Copy, Debug

attrs specifier info for a class-body assignment whose RHS is a `field()` / `attr.ib()` call.
The in-body value is typed `Any` so `@<field>.default`/`.validator`/`.converter` accesses resolve.

---

## BindingAbstractClassCheck

`struct` · `pyrefly::binding::binding::BindingAbstractClassCheck`

```rust
struct BindingAbstractClassCheck
```

**Fields**: `class_idx`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingClassBaseType

`struct` · `pyrefly::binding::binding::BindingClassBaseType`

```rust
struct BindingClassBaseType
```

**Fields**: `class_idx`, `bases`, `is_new_type`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, bindings: &Bindings) -> fmt::Result
```

Binding for the base class types of a class.

---

## BindingClassChecks

`struct` · `pyrefly::binding::binding::BindingClassChecks`

```rust
struct BindingClassChecks
```

**Fields**: `class_idx`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Class-level diagnostics grouped behind one `EmptyAnswer` key.

Checks that produce answers used by downstream lookups should remain separate keys.

---

## BindingClassDisjointBase

`struct` · `pyrefly::binding::binding::BindingClassDisjointBase`

```rust
struct BindingClassDisjointBase
```

**Fields**: `class_idx`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Binding for the class's disjoint-base representative. The solver reads
both metadata and MRO for `class_idx`; both must be available, so this
binding must be inserted everywhere `BindingClassMro` is.

---

## BindingClassField

`struct` · `pyrefly::binding::binding::BindingClassField`

```rust
struct BindingClassField
```

**Fields**: `class_idx`, `name`, `range`, `definition`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Binding for a class field, which is any attribute (including methods) of a class defined in
either the class body or in method (like `__init__`) that we recognize as
defining instance attributes.

---

## BindingClassMetadata

`struct` · `pyrefly::binding::binding::BindingClassMetadata`

```rust
struct BindingClassMetadata
```

**Fields**: `class_idx`, `bases`, `keywords`, `decorators`, `is_new_type`, `pydantic_config_dict`, `pydantic_before_validator_fields`, `django_field_info`, `capture_init`, `shaped_array_metadata`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Binding for the class's metadata (anything obtained directly from base classes,
except for the MRO which is kept separate to avoid cycles).

---

## BindingClassMro

`struct` · `pyrefly::binding::binding::BindingClassMro`

```rust
struct BindingClassMro
```

**Fields**: `class_idx`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Binding for the class's MRO
This requires base classes; these should match what `BindingClassMetadata` has.

---

## BindingClassSubscriptSymmetry

`struct` · `pyrefly::binding::binding::BindingClassSubscriptSymmetry`

```rust
struct BindingClassSubscriptSymmetry
```

**Fields**: `class_idx`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingClassSynthesizedFields

`struct` · `pyrefly::binding::binding::BindingClassSynthesizedFields`

```rust
struct BindingClassSynthesizedFields
```

**Fields**: `class_idx`, `nn_module_registrations`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Bindings for fields synthesized by a class, such as a dataclass's `__init__` method. This
has to be its own key/binding type because of the dependencies between the various pieces of
information about a class: ClassDef -> ClassMetadata -> ClassField -> ClassSynthesizedFields.

---

## BindingDecoratedFunction

`struct` · `pyrefly::binding::binding::BindingDecoratedFunction`

```rust
struct BindingDecoratedFunction
```

**Fields**: `undecorated_idx`, `successor`, `docstring_range`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingDecorator

`struct` · `pyrefly::binding::binding::BindingDecorator`

```rust
struct BindingDecorator
```

**Fields**: `expr`, `trailing_name`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BindingDjangoRelations

`struct` · `pyrefly::binding::binding::BindingDjangoRelations`

```rust
struct BindingDjangoRelations
```

**Fields**: `classes`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &Bindings) -> fmt::Result
```

Binding for Django relations in a module, used to synthesize reverse relationships.

---

## BindingTParams

`struct` · `pyrefly::binding::binding::BindingTParams`

```rust
struct BindingTParams
```

**Fields**: `name`, `scoped_type_params`, `generic_bases`, `legacy_tparams`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &Bindings) -> fmt::Result
```

Binding for a class.

---

## BindingUndecoratedFunction

`struct` · `pyrefly::binding::binding::BindingUndecoratedFunction`

```rust
struct BindingUndecoratedFunction
```

**Fields**: `def_index`, `def`, `is_in_type_checking_block`, `body_kind`, `is_return_inferred`, `calls_super_method`, `class_key`, `legacy_tparams`, `decorators`, `parent`, `shape_dsl_def`, `type_shape_dsl_def`, `uses_shape_dsl_ir_name`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &Bindings) -> fmt::Result
```

---

## BindingUndecoratedFunctionRange

`struct` · `pyrefly::binding::binding::BindingUndecoratedFunctionRange`

```rust
struct BindingUndecoratedFunctionRange
```

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

Binding value for KeyUndecoratedFunctionRange.

---

## BindingVariance

`struct` · `pyrefly::binding::binding::BindingVariance`

```rust
struct BindingVariance
```

**Fields**: `class_key`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Bindings) -> fmt::Result
```

---

## BranchInfo

`struct` · `pyrefly::binding::binding::BranchInfo`

```rust
struct BranchInfo
```

**Fields**: `value_key`, `termination_key`

**Derives**: Clone, Debug

Information about a branch in a Phi node.

---

## ClassBinding

`struct` · `pyrefly::binding::binding::ClassBinding`

```rust
struct ClassBinding
```

**Fields**: `def`, `def_index`, `parent`, `is_protocol`, `tparams_require_binding`, `docstring_range`

**Derives**: Clone, Debug

---

## ClassBodyUnknownName

`struct` · `pyrefly::binding::binding::ClassBodyUnknownName`

```rust
struct ClassBodyUnknownName
```

**Fields**: `class_key`, `name`, `suggestion`, `allow_class_body_forward_reference`

**Derives**: Clone, Debug

Data for a name in a class body that wasn't found in the static scope.

---

## ClassDefData

`struct` · `pyrefly::binding::binding::ClassDefData`

```rust
struct ClassDefData
```

**Fields**: `name`, `type_params`, `range`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(def: StmtClassDef) -> Self
```

Stores fields from `StmtClassDef` that are needed during solving.

---

## DjangoRelationClass

`struct` · `pyrefly::binding::binding::DjangoRelationClass`

```rust
struct DjangoRelationClass
```

**Fields**: `class_idx`, `fields`

**Derives**: Clone, Debug

---

## EmptyAnswer

`struct` · `pyrefly::binding::binding::EmptyAnswer`

```rust
struct EmptyAnswer
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## ExhaustiveBinding

`struct` · `pyrefly::binding::binding::ExhaustiveBinding`

```rust
struct ExhaustiveBinding
```

**Fields**: `kind`, `narrow_entries`

**Derives**: Clone, Debug

Data for an exhaustiveness check binding.

Contains multiple narrow entries `(Idx<Key>, Box<NarrowOp>, TextRange)`. At solve time,
if ANY entry narrows to `Never`, the construct is exhaustive. This enables multi-subject
and isinstance-based exhaustiveness without complex subject-extraction logic.

---

## FunctionDefData

`struct` · `pyrefly::binding::binding::FunctionDefData`

```rust
struct FunctionDefData
```

**Fields**: `name`, `parameters`, `type_params`, `is_async`, `range`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(def: StmtFunctionDef) -> Self
```

Stores fields from `StmtFunctionDef` that are needed during solving.

---

## ImportBinding

`struct` · `pyrefly::binding::binding::ImportBinding`

```rust
struct ImportBinding
```

**Fields**: `module`, `name`, `original_name_range`, `check_deprecated`, `fallback`

**Derives**: Clone, Debug

Data for `Binding::Import`. Carries the `(module, name)` of an imported
symbol, plus metadata for downstream consumers.

---

## ImportFallback

`struct` · `pyrefly::binding::binding::ImportFallback`

```rust
struct ImportFallback
```

**Fields**: `stmt_range`, `is_unreachable`

**Derives**: Clone, Debug

Information required by the solve-time fallback cascade for
[`ImportBinding::fallback`].

---

## KeyAbstractClassCheck

`struct` · `pyrefly::binding::binding::KeyAbstractClassCheck`

```rust
struct KeyAbstractClassCheck
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingAbstractClassCheck, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## KeyClass

`struct` · `pyrefly::binding::binding::KeyClass`

```rust
struct KeyClass
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClass, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A reference to a class.

---

## KeyClassBaseType

`struct` · `pyrefly::binding::binding::KeyClassBaseType`

```rust
struct KeyClassBaseType
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassBaseType, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &pyrefly_python::module::Module) -> fmt::Result
```

A reference to a class.

---

## KeyClassChecks

`struct` · `pyrefly::binding::binding::KeyClassChecks`

```rust
struct KeyClassChecks
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassChecks, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## KeyClassDisjointBase

`struct` · `pyrefly::binding::binding::KeyClassDisjointBase`

```rust
struct KeyClassDisjointBase
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassDisjointBase, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

Disjoint-base representative for a class, used for PEP 800 narrowing and
downstream inheritance checks. Owned by `KeyClassDisjointBase`, not
`KeyClassMro`, so generated dataclass-slot promotion can read the
already-solved MRO without a binding cycle.

---

## KeyClassField

`struct` · `pyrefly::binding::binding::KeyClassField`

```rust
struct KeyClassField
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassField, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

A reference to a field in a class.

---

## KeyClassMetadata

`struct` · `pyrefly::binding::binding::KeyClassMetadata`

```rust
struct KeyClassMetadata
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassMetadata, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

Keys that refer to a class's `Mro` (which tracks its ancestors, in method
resolution order).

---

## KeyClassMro

`struct` · `pyrefly::binding::binding::KeyClassMro`

```rust
struct KeyClassMro
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassMro, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

Keys that refer to a class's `Mro` (which tracks its ancestors, in method
resolution order).

---

## KeyClassSubscriptSymmetry

`struct` · `pyrefly::binding::binding::KeyClassSubscriptSymmetry`

```rust
struct KeyClassSubscriptSymmetry
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassSubscriptSymmetry, _range: TextRange, _errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## KeyClassSynthesizedFields

`struct` · `pyrefly::binding::binding::KeyClassSynthesizedFields`

```rust
struct KeyClassSynthesizedFields
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingClassSynthesizedFields, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

Keys that refer to fields synthesized by a class, such as a dataclass's `__init__` method. This
has to be its own key/binding type because of the dependencies between the various pieces of
information about a class: ClassDef -> ClassMetadata -> ClassField -> ClassSynthesizedFields.

---

## KeyDecoratedFunction

`struct` · `pyrefly::binding::binding::KeyDecoratedFunction`

```rust
struct KeyDecoratedFunction
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingDecoratedFunction, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## KeyDecorator

`struct` · `pyrefly::binding::binding::KeyDecorator`

```rust
struct KeyDecorator
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingDecorator, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## KeyDjangoRelations

`struct` · `pyrefly::binding::binding::KeyDjangoRelations`

```rust
struct KeyDjangoRelations
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingDjangoRelations, range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(_idx: Idx<Self>, _bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Key for Django reverse relationship metadata within a module.

---

## KeyExport

`struct` · `pyrefly::binding::binding::KeyExport`

```rust
struct KeyExport
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingExport, range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &pyrefly_python::module::Module) -> fmt::Result
```

The binding definition site, at the end of the module (used for export).
If it has an annotation, only the annotation will be returned.

---

## KeyLegacyTypeParam

`struct` · `pyrefly::binding::binding::KeyLegacyTypeParam`

```rust
struct KeyLegacyTypeParam
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingLegacyTypeParam, range: TextRange, _errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## KeyTParams

`struct` · `pyrefly::binding::binding::KeyTParams`

```rust
struct KeyTParams
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingTParams, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &pyrefly_python::module::Module) -> fmt::Result
```

A reference to a class.

---

## KeyTypeAlias

`struct` · `pyrefly::binding::binding::KeyTypeAlias`

```rust
struct KeyTypeAlias
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingTypeAlias, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## KeyUndecoratedFunction

`struct` · `pyrefly::binding::binding::KeyUndecoratedFunction`

```rust
struct KeyUndecoratedFunction
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingUndecoratedFunction, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## KeyUndecoratedFunctionRange

`struct` · `pyrefly::binding::binding::KeyUndecoratedFunctionRange`

```rust
struct KeyUndecoratedFunctionRange
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(_answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingUndecoratedFunctionRange, _range: TextRange, _errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &pyrefly_python::module::Module) -> fmt::Result
```

Maps a FuncDefIndex to the function's ShortIdentifier, enabling lookup of
the corresponding KeyUndecoratedFunction.

---

## KeyVariance

`struct` · `pyrefly::binding::binding::KeyVariance`

```rust
struct KeyVariance
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Exported`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(_heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingVariance, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Exported`**

```rust
fn to_anykey(&self) -> AnyExportedKey
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, _ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## KeyYield

`struct` · `pyrefly::binding::binding::KeyYield`

```rust
struct KeyYield
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingYield, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## KeyYieldFrom

`struct` · `pyrefly::binding::binding::KeyYieldFrom`

```rust
struct KeyYieldFrom
```

**Implements**: `pyrefly::alt::traits::Solve`, `pyrefly::binding::binding::Keyed`, `pyrefly_util::display::DisplayWith`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly::alt::traits::Solve`**

```rust
fn promote_recursive(heap: &TypeHeap, _: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &BindingYieldFrom, _range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

**via `pyrefly::binding::binding::Keyed`**

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## LambdaParamId

`struct` · `pyrefly::binding::binding::LambdaParamId`

```rust
struct LambdaParamId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## LegacyTypeParamModule

`struct` · `pyrefly::binding::binding::LegacyTypeParamModule`

```rust
struct LegacyTypeParamModule
```

**Fields**: `base`, `attrs`

**Derives**: Clone, Debug

A module attribute that may be a legacy type parameter. For `pkg.mod.T`, `base` is
the binding for the local name `pkg`, and `attrs` contains `mod` and `T`.

---

## MethodThatSetsAttr

`struct` · `pyrefly::binding::binding::MethodThatSetsAttr`

```rust
struct MethodThatSetsAttr
```

**Fields**: `method_name`, `recognized_attribute_defining_method`, `instance_or_class`

**Derives**: Clone, Debug

The method where an attribute was defined implicitly by assignment to `self.<attr_name>`

We track whether this method is recognized as a valid attribute-defining
method (e.g. a constructor); if an attribute is inferred only from assignments
in non-recognized methods, we will infer its type but also produce a type error.

---

## MultiTargetReceiver

`struct` · `pyrefly::binding::binding::MultiTargetReceiver`

```rust
struct MultiTargetReceiver
```

**Fields**: `name`, `idx`

**Derives**: Clone, Debug

Carries the canonical class identity for a receiver-constrained class
rebind through `MultiTargetAssign` and `UnpackedValue` bindings. The `name`
is the LHS being assigned to, and `idx` points at the canonical class-object
binding of the original `class` definition. Invariants and semantics match
`NameAssign::receiver_idx`.

---

## NameAssign

`struct` · `pyrefly::binding::binding::NameAssign`

```rust
struct NameAssign
```

**Fields**: `name`, `annotation`, `expr`, `legacy_tparams`, `is_in_function_scope`, `is_class_body_assignment`, `first_use`, `def_idx`, `receiver_idx`, `attrs_field_specifier`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn is_pinned(&self) -> bool
```

Data for a name assignment binding.

---

## NoneIfRecursive

`struct` · `pyrefly::binding::binding::NoneIfRecursive`

```rust
struct NoneIfRecursive<T>
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## PrivateAttributeAccessCheck

`struct` · `pyrefly::binding::binding::PrivateAttributeAccessCheck`

```rust
struct PrivateAttributeAccessCheck
```

**Fields**: `value`, `attr`, `class_idx`

**Derives**: Clone, Debug

---

## ReturnExplicit

`struct` · `pyrefly::binding::binding::ReturnExplicit`

```rust
struct ReturnExplicit
```

**Fields**: `annot`, `expr`, `is_generator`, `is_async`, `range`, `is_unreachable`

**Derives**: Clone, Debug

---

## ReturnImplicit

`struct` · `pyrefly::binding::binding::ReturnImplicit`

```rust
struct ReturnImplicit
```

**Fields**: `last_exprs`

**Derives**: Clone, Debug

---

## ReturnType

`struct` · `pyrefly::binding::binding::ReturnType`

```rust
struct ReturnType
```

**Fields**: `kind`, `is_async`, `implicit_dunder_new_self`

**Derives**: Clone, Debug

---

## ShapedArrayMetadata

`struct` · `pyrefly::binding::binding::ShapedArrayMetadata`

```rust
struct ShapedArrayMetadata
```

**Fields**: `shape_name`, `range`, `builtin_indexing`

**Derives**: Clone, Debug

Information about a class that is marked as using array shapes (for shape typing).

---

## SuppressedException

`struct` · `pyrefly::binding::binding::SuppressedException`

```rust
struct SuppressedException
```

**Fields**: `contexts`, `kind`, `body`

**Derives**: Clone, Debug

Data for the reachability of the code following a `with` statement whose body
terminated with a `raise`

---

## TypeAliasBinding

`struct` · `pyrefly::binding::binding::TypeAliasBinding`

```rust
struct TypeAliasBinding
```

**Fields**: `name`, `tparams`, `key_type_alias`, `range`

**Derives**: Clone, Debug

Data for a type alias binding.

---

## TypeAliasRefBinding

`struct` · `pyrefly::binding::binding::TypeAliasRefBinding`

```rust
struct TypeAliasRefBinding
```

**Fields**: `name`, `key_type_alias`, `tparams`

**Derives**: Clone, Debug

Data for a type alias reference binding.

---

## TypeLevelLambdaParameter

`struct` · `pyrefly::binding::binding::TypeLevelLambdaParameter`

```rust
struct TypeLevelLambdaParameter
```

**Fields**: `id`, `identifier`

**Derives**: Clone, Debug

Binding data for a parameter introduced by a type-level lambda.

---

## TypeParameter

`struct` · `pyrefly::binding::binding::TypeParameter`

```rust
struct TypeParameter
```

**Fields**: `name`, `identity`, `kind`, `bound`, `default`, `constraints`, `owner`

**Derives**: Clone, Debug

---

## UndecoratedFunctionRangeAnswer

`struct` · `pyrefly::binding::binding::UndecoratedFunctionRangeAnswer`

```rust
struct UndecoratedFunctionRangeAnswer
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Trivial answer type for KeyUndecoratedFunctionRange — just a copy of the
binding value (the function's ShortIdentifier).

---

## UnpackedValue

`struct` · `pyrefly::binding::binding::UnpackedValue`

```rust
struct UnpackedValue
```

**Fields**: `annotation`, `source`, `range`, `position`, `receiver`

**Derives**: Clone, Debug

Data for a value at a specific position in an unpacked iterable expression.

---

## Exported

`trait` · `pyrefly::binding::binding::Exported`

```rust
trait Exported: Keyed
```

**Implementors** (13)

- `pyrefly::binding::binding::KeyAbstractClassCheck`
- `pyrefly::binding::binding::KeyClassBaseType`
- `pyrefly::binding::binding::KeyClassDisjointBase`
- `pyrefly::binding::binding::KeyClassField`
- `pyrefly::binding::binding::KeyClassMetadata`
- `pyrefly::binding::binding::KeyClassMro`
- `pyrefly::binding::binding::KeyClassSubscriptSymmetry`
- `pyrefly::binding::binding::KeyClassSynthesizedFields`
- `pyrefly::binding::binding::KeyDjangoRelations`
- `pyrefly::binding::binding::KeyExport`
- `pyrefly::binding::binding::KeyTParams`
- `pyrefly::binding::binding::KeyTypeAlias`
- `pyrefly::binding::binding::KeyVariance`

**Methods** (1)

```rust
fn to_anykey(&self) -> AnyExportedKey
```

Should be equivalent to Keyed<EXPORTED=true>.
Once `associated_const_equality` is stabilised, can switch to that.

---

## Keyed

`trait` · `pyrefly::binding::binding::Keyed`

```rust
trait Keyed: Hash + Eq + Clone + DisplayWith<pyrefly_python::module::Module> + Debug + 'static
```

**Implementors** (25)

- `pyrefly::binding::binding::Key`
- `pyrefly::binding::binding::KeyAbstractClassCheck`
- `pyrefly::binding::binding::KeyAnnotation`
- `pyrefly::binding::binding::KeyClass`
- `pyrefly::binding::binding::KeyClassBaseType`
- `pyrefly::binding::binding::KeyClassChecks`
- `pyrefly::binding::binding::KeyClassDisjointBase`
- `pyrefly::binding::binding::KeyClassField`
- `pyrefly::binding::binding::KeyClassMetadata`
- `pyrefly::binding::binding::KeyClassMro`
- `pyrefly::binding::binding::KeyClassSubscriptSymmetry`
- `pyrefly::binding::binding::KeyClassSynthesizedFields`
- `pyrefly::binding::binding::KeyDecoratedFunction`
- `pyrefly::binding::binding::KeyDecorator`
- `pyrefly::binding::binding::KeyDjangoRelations`
- `pyrefly::binding::binding::KeyExpect`
- `pyrefly::binding::binding::KeyExport`
- `pyrefly::binding::binding::KeyLegacyTypeParam`
- `pyrefly::binding::binding::KeyTParams`
- `pyrefly::binding::binding::KeyTypeAlias`
- `pyrefly::binding::binding::KeyUndecoratedFunction`
- `pyrefly::binding::binding::KeyUndecoratedFunctionRange`
- `pyrefly::binding::binding::KeyVariance`
- `pyrefly::binding::binding::KeyYield`
- `pyrefly::binding::binding::KeyYieldFrom`

**Methods** (3)

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

Any key that sets `EXPORTED` to `true` should not include positions
Incremental updates depend on knowing when a file's exports changed, which uses equality between exported keys
Moving code around should not cause all dependencies to be re-checked

---
