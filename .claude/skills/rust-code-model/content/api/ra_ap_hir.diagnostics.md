# `ra_ap_hir::diagnostics`

Crate `ra_ap_hir` · 83 public items · structured records in [`model/ra_ap_hir.diagnostics.json`](../model/ra_ap_hir.diagnostics.json)

## AnyDiagnostic

`enum` · `ra_ap_hir::diagnostics::AnyDiagnostic`

Also reachable as `ra_ap_hir::AnyDiagnostic`

```rust
enum AnyDiagnostic<'db>
```

**Variants**: `ArrayPatternWithoutFixedLength`, `AwaitOutsideOfAsync`, `BreakOutsideOfLoop`, `CannotBeDereferenced`, `UnaryOperatorCannotBeApplied`, `CannotImplicitlyDerefTraitObject`, `CannotIndexInto`, `CastToUnsized`, `ExpectedArrayOrSlicePat`, `ExpectedFunction`, `ExplicitDropMethodUse`, `FruInDestructuringAssignment`, `MissingBody`, `FunctionalRecordUpdateOnNonStruct`, `GenericDefaultRefersToSelf`, `InactiveCode`, `IncoherentImpl`, `IncorrectCase`, `IncorrectGenericsLen`, `IncorrectGenericsOrder`, `InferVarsNotAllowed`, `InvalidCast`, `InvalidDeriveTarget`, `InvalidLhsOfAssignment`, `InvalidRangePatType`, `MacroDefError`, `MacroError`, `MacroExpansionParseError`, `MalformedDerive`, `MethodCallIllegalSizedBound`, `MismatchedArgCount`, `MismatchedTupleStructPatArgCount`, `MissingFields`, `MissingMatchArms`, `MissingUnsafe`, `MutRefInImmRefPat`, `MutableRefBinding`, `NonExhaustiveLet`, `NonExhaustiveRecordExpr`, `NonExhaustiveRecordPat`, `NoSuchField`, `MismatchedArrayPatLen`, `DuplicateField`, `PatternArgInExternFn`, `PrivateAssocItem`, `PrivateField`, `RemoveTrailingReturn`, `RemoveUnnecessaryElse`, `UnusedMustUse`, `ReplaceFilterMapNextWithFindMap`, `TraitImplIncorrectSafety`, `TraitImplMissingAssocItems`, `TraitImplOrphan`, `TraitImplRedundantAssocItems`, `TypedHole`, `TypeMismatch`, `UndeclaredLabel`, `UnimplementedBuiltinMacro`, `UnreachableLabel`, `UnresolvedAssocItem`, `UnresolvedExternCrate`, `UnresolvedField`, `UnresolvedImport`, `UnresolvedMacroCall`, `UnresolvedMethodCall`, `UnresolvedModule`, `UnresolvedIdent`, `GenericArgsProhibited`, `ParenthesizedGenericArgsWithoutFnTrait`, `BadRtn`, `MissingLifetime`, `ElidedLifetimesInPath`, `TypeMustBeKnown`, `UnionExprMustHaveExactlyOneField`, `UnionPatMustHaveExactlyOneField`, `UnionPatHasRest`, `UnimplementedTrait`, `YieldOutsideCoroutine`, `ReturnOutsideFunction`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(d: MissingMatchArms) -> AnyDiagnostic<'db>
fn from(d: TraitImplOrphan) -> AnyDiagnostic<'db>
fn from(d: ElidedLifetimesInPath) -> AnyDiagnostic<'db>
fn from(d: IncorrectCase) -> AnyDiagnostic<'db>
fn from(d: MutableRefBinding) -> AnyDiagnostic<'db>
fn from(d: TypeMismatch<'db>) -> AnyDiagnostic<'db>
fn from(d: AwaitOutsideOfAsync) -> AnyDiagnostic<'db>
fn from(d: UnionPatMustHaveExactlyOneField) -> AnyDiagnostic<'db>
fn from(d: InferVarsNotAllowed) -> AnyDiagnostic<'db>
fn from(d: NonExhaustiveRecordPat) -> AnyDiagnostic<'db>
fn from(d: UnreachableLabel) -> AnyDiagnostic<'db>
fn from(d: UnaryOperatorCannotBeApplied<'db>) -> AnyDiagnostic<'db>
fn from(d: YieldOutsideCoroutine) -> AnyDiagnostic<'db>
fn from(d: InvalidLhsOfAssignment) -> AnyDiagnostic<'db>
fn from(d: DuplicateField) -> AnyDiagnostic<'db>
fn from(d: UnresolvedField<'db>) -> AnyDiagnostic<'db>
fn from(d: CastToUnsized<'db>) -> AnyDiagnostic<'db>
fn from(d: MacroError) -> AnyDiagnostic<'db>
fn from(d: PrivateField) -> AnyDiagnostic<'db>
fn from(d: UnresolvedMethodCall<'db>) -> AnyDiagnostic<'db>
fn from(d: ExplicitDropMethodUse) -> AnyDiagnostic<'db>
fn from(d: MethodCallIllegalSizedBound) -> AnyDiagnostic<'db>
fn from(d: UnusedMustUse<'db>) -> AnyDiagnostic<'db>
fn from(d: GenericArgsProhibited) -> AnyDiagnostic<'db>
fn from(d: FunctionalRecordUpdateOnNonStruct) -> AnyDiagnostic<'db>
fn from(d: MissingFields) -> AnyDiagnostic<'db>
fn from(d: TraitImplMissingAssocItems) -> AnyDiagnostic<'db>
fn from(d: MissingLifetime) -> AnyDiagnostic<'db>
fn from(d: IncoherentImpl) -> AnyDiagnostic<'db>
fn from(d: MutRefInImmRefPat) -> AnyDiagnostic<'db>
fn from(d: TypedHole<'db>) -> AnyDiagnostic<'db>
fn from(d: ArrayPatternWithoutFixedLength) -> AnyDiagnostic<'db>
fn from(d: UnionExprMustHaveExactlyOneField) -> AnyDiagnostic<'db>
fn from(d: IncorrectGenericsOrder) -> AnyDiagnostic<'db>
fn from(d: NonExhaustiveRecordExpr) -> AnyDiagnostic<'db>
fn from(d: UnimplementedBuiltinMacro) -> AnyDiagnostic<'db>
fn from(d: CannotBeDereferenced<'db>) -> AnyDiagnostic<'db>
fn from(d: UnimplementedTrait<'db>) -> AnyDiagnostic<'db>
fn from(d: InvalidDeriveTarget) -> AnyDiagnostic<'db>
fn from(d: MismatchedArrayPatLen) -> AnyDiagnostic<'db>
fn from(d: UnresolvedExternCrate) -> AnyDiagnostic<'db>
fn from(d: CannotIndexInto<'db>) -> AnyDiagnostic<'db>
fn from(d: MacroDefError) -> AnyDiagnostic<'db>
fn from(d: PrivateAssocItem) -> AnyDiagnostic<'db>
fn from(d: UnresolvedMacroCall) -> AnyDiagnostic<'db>
fn from(d: ExpectedFunction<'db>) -> AnyDiagnostic<'db>
fn from(d: MalformedDerive) -> AnyDiagnostic<'db>
fn from(d: RemoveUnnecessaryElse) -> AnyDiagnostic<'db>
fn from(d: UnresolvedIdent) -> AnyDiagnostic<'db>
fn from(d: MissingBody) -> AnyDiagnostic<'db>
fn from(d: MismatchedTupleStructPatArgCount) -> AnyDiagnostic<'db>
fn from(d: TraitImplIncorrectSafety) -> AnyDiagnostic<'db>
fn from(d: BadRtn) -> AnyDiagnostic<'db>
fn from(d: InactiveCode) -> AnyDiagnostic<'db>
fn from(d: MissingUnsafe) -> AnyDiagnostic<'db>
fn from(d: TraitImplRedundantAssocItems) -> AnyDiagnostic<'db>
fn from(d: TypeMustBeKnown<'db>) -> AnyDiagnostic<'db>
fn from(d: IncorrectGenericsLen) -> AnyDiagnostic<'db>
fn from(d: NonExhaustiveLet) -> AnyDiagnostic<'db>
fn from(d: UndeclaredLabel) -> AnyDiagnostic<'db>
fn from(d: BreakOutsideOfLoop) -> AnyDiagnostic<'db>
fn from(d: UnionPatHasRest) -> AnyDiagnostic<'db>
fn from(d: InvalidCast<'db>) -> AnyDiagnostic<'db>
fn from(d: NoSuchField) -> AnyDiagnostic<'db>
fn from(d: UnresolvedAssocItem) -> AnyDiagnostic<'db>
fn from(d: CannotImplicitlyDerefTraitObject<'db>) -> AnyDiagnostic<'db>
fn from(d: ReturnOutsideFunction) -> AnyDiagnostic<'db>
fn from(d: InvalidRangePatType) -> AnyDiagnostic<'db>
fn from(d: PatternArgInExternFn) -> AnyDiagnostic<'db>
fn from(d: UnresolvedImport) -> AnyDiagnostic<'db>
fn from(d: ExpectedArrayOrSlicePat<'db>) -> AnyDiagnostic<'db>
fn from(d: MacroExpansionParseError) -> AnyDiagnostic<'db>
fn from(d: RemoveTrailingReturn) -> AnyDiagnostic<'db>
fn from(d: UnresolvedModule) -> AnyDiagnostic<'db>
fn from(d: FruInDestructuringAssignment) -> AnyDiagnostic<'db>
fn from(d: MismatchedArgCount) -> AnyDiagnostic<'db>
fn from(d: ReplaceFilterMapNextWithFindMap) -> AnyDiagnostic<'db>
fn from(d: ParenthesizedGenericArgsWithoutFnTrait) -> AnyDiagnostic<'db>
fn from(d: GenericDefaultRefersToSelf) -> AnyDiagnostic<'db>
```

---

## GenericArgKind

`enum` · `ra_ap_hir::diagnostics::GenericArgKind`

Also reachable as `ra_ap_hir::GenericArgKind`

```rust
enum GenericArgKind
```

**Variants**: `Lifetime`, `Type`, `Const`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## SpanAst

`enum` · `ra_ap_hir::diagnostics::SpanAst`

Also reachable as `ra_ap_hir::SpanAst`

```rust
enum SpanAst
```

**Variants**: `Expr`, `Pat`, `Type`

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(it: Pat) -> SpanAst
fn from(it: Expr) -> SpanAst
fn from(value: Either<ast::Expr, ast::Pat>) -> Self
fn from(it: Type) -> SpanAst
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: syntax::SyntaxKind) -> bool
fn cast(syntax: syntax::SyntaxNode) -> Option<Self>
fn syntax(&self) -> &syntax::SyntaxNode
```

---

## UnsafeLint

`enum` · `ra_ap_hir::diagnostics::UnsafeLint`

Also reachable as `ra_ap_hir::UnsafeLint`

```rust
enum UnsafeLint
```

**Variants**: `HardError`, `UnsafeOpInUnsafeFn`, `DeprecatedSafe2024`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## ArrayPatternWithoutFixedLength

`struct` · `ra_ap_hir::diagnostics::ArrayPatternWithoutFixedLength`

Also reachable as `ra_ap_hir::ArrayPatternWithoutFixedLength`

```rust
struct ArrayPatternWithoutFixedLength
```

**Fields**: `pat`

**Derives**: Debug

---

## AwaitOutsideOfAsync

`struct` · `ra_ap_hir::diagnostics::AwaitOutsideOfAsync`

Also reachable as `ra_ap_hir::AwaitOutsideOfAsync`

```rust
struct AwaitOutsideOfAsync
```

**Fields**: `node`, `location`

**Derives**: Debug

---

## BadRtn

`struct` · `ra_ap_hir::diagnostics::BadRtn`

Also reachable as `ra_ap_hir::BadRtn`

```rust
struct BadRtn
```

**Fields**: `rtn`

**Derives**: Debug

---

## BreakOutsideOfLoop

`struct` · `ra_ap_hir::diagnostics::BreakOutsideOfLoop`

Also reachable as `ra_ap_hir::BreakOutsideOfLoop`

```rust
struct BreakOutsideOfLoop
```

**Fields**: `expr`, `is_break`, `bad_value_break`

**Derives**: Debug

---

## CannotBeDereferenced

`struct` · `ra_ap_hir::diagnostics::CannotBeDereferenced`

Also reachable as `ra_ap_hir::CannotBeDereferenced`

```rust
struct CannotBeDereferenced<'db>
```

**Fields**: `expr`, `found`

**Derives**: Debug

---

## CannotImplicitlyDerefTraitObject

`struct` · `ra_ap_hir::diagnostics::CannotImplicitlyDerefTraitObject`

Also reachable as `ra_ap_hir::CannotImplicitlyDerefTraitObject`

```rust
struct CannotImplicitlyDerefTraitObject<'db>
```

**Fields**: `pat`, `found`

**Derives**: Debug

---

## CannotIndexInto

`struct` · `ra_ap_hir::diagnostics::CannotIndexInto`

Also reachable as `ra_ap_hir::CannotIndexInto`

```rust
struct CannotIndexInto<'db>
```

**Fields**: `expr`, `found`

**Derives**: Debug

---

## CastToUnsized

`struct` · `ra_ap_hir::diagnostics::CastToUnsized`

Also reachable as `ra_ap_hir::CastToUnsized`

```rust
struct CastToUnsized<'db>
```

**Fields**: `expr`, `cast_ty`

**Derives**: Debug

---

## DuplicateField

`struct` · `ra_ap_hir::diagnostics::DuplicateField`

Also reachable as `ra_ap_hir::DuplicateField`

```rust
struct DuplicateField
```

**Fields**: `field`, `variant`

**Derives**: Debug

---

## ElidedLifetimesInPath

`struct` · `ra_ap_hir::diagnostics::ElidedLifetimesInPath`

Also reachable as `ra_ap_hir::ElidedLifetimesInPath`

```rust
struct ElidedLifetimesInPath
```

**Fields**: `generics_or_segment`, `expected`, `def`, `hard_error`

**Derives**: Debug

---

## ExpectedArrayOrSlicePat

`struct` · `ra_ap_hir::diagnostics::ExpectedArrayOrSlicePat`

Also reachable as `ra_ap_hir::ExpectedArrayOrSlicePat`

```rust
struct ExpectedArrayOrSlicePat<'db>
```

**Fields**: `pat`, `found`

**Derives**: Debug

---

## ExpectedFunction

`struct` · `ra_ap_hir::diagnostics::ExpectedFunction`

Also reachable as `ra_ap_hir::ExpectedFunction`

```rust
struct ExpectedFunction<'db>
```

**Fields**: `call`, `found`

**Derives**: Debug

---

## ExplicitDropMethodUse

`struct` · `ra_ap_hir::diagnostics::ExplicitDropMethodUse`

Also reachable as `ra_ap_hir::ExplicitDropMethodUse`

```rust
struct ExplicitDropMethodUse
```

**Fields**: `expr_or_path`

**Derives**: Debug

---

## FruInDestructuringAssignment

`struct` · `ra_ap_hir::diagnostics::FruInDestructuringAssignment`

Also reachable as `ra_ap_hir::FruInDestructuringAssignment`

```rust
struct FruInDestructuringAssignment
```

**Fields**: `node`

**Derives**: Debug

---

## FunctionalRecordUpdateOnNonStruct

`struct` · `ra_ap_hir::diagnostics::FunctionalRecordUpdateOnNonStruct`

Also reachable as `ra_ap_hir::FunctionalRecordUpdateOnNonStruct`

```rust
struct FunctionalRecordUpdateOnNonStruct
```

**Fields**: `base_expr`

**Derives**: Debug

---

## GenericArgsProhibited

`struct` · `ra_ap_hir::diagnostics::GenericArgsProhibited`

Also reachable as `ra_ap_hir::GenericArgsProhibited`

```rust
struct GenericArgsProhibited
```

**Fields**: `args`, `reason`

**Derives**: Debug

---

## GenericDefaultRefersToSelf

`struct` · `ra_ap_hir::diagnostics::GenericDefaultRefersToSelf`

Also reachable as `ra_ap_hir::GenericDefaultRefersToSelf`

```rust
struct GenericDefaultRefersToSelf
```

**Fields**: `segment`

**Derives**: Debug

---

## InactiveCode

`struct` · `ra_ap_hir::diagnostics::InactiveCode`

Also reachable as `ra_ap_hir::InactiveCode`

```rust
struct InactiveCode
```

**Fields**: `node`, `cfg`, `opts`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## IncoherentImpl

`struct` · `ra_ap_hir::diagnostics::IncoherentImpl`

Also reachable as `ra_ap_hir::IncoherentImpl`

```rust
struct IncoherentImpl
```

**Fields**: `file_id`, `impl_`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

---

## IncorrectGenericsLen

`struct` · `ra_ap_hir::diagnostics::IncorrectGenericsLen`

Also reachable as `ra_ap_hir::IncorrectGenericsLen`

```rust
struct IncorrectGenericsLen
```

**Fields**: `generics_or_segment`, `kind`, `provided`, `expected`, `def`

**Derives**: Debug

---

## IncorrectGenericsOrder

`struct` · `ra_ap_hir::diagnostics::IncorrectGenericsOrder`

Also reachable as `ra_ap_hir::IncorrectGenericsOrder`

```rust
struct IncorrectGenericsOrder
```

**Fields**: `provided_arg`, `expected_kind`

**Derives**: Debug

---

## InferVarsNotAllowed

`struct` · `ra_ap_hir::diagnostics::InferVarsNotAllowed`

Also reachable as `ra_ap_hir::InferVarsNotAllowed`

```rust
struct InferVarsNotAllowed
```

**Fields**: `node`

**Derives**: Debug

---

## InvalidCast

`struct` · `ra_ap_hir::diagnostics::InvalidCast`

Also reachable as `ra_ap_hir::InvalidCast`

```rust
struct InvalidCast<'db>
```

**Fields**: `expr`, `error`, `expr_ty`, `cast_ty`

**Derives**: Debug

---

## InvalidDeriveTarget

`struct` · `ra_ap_hir::diagnostics::InvalidDeriveTarget`

Also reachable as `ra_ap_hir::InvalidDeriveTarget`

```rust
struct InvalidDeriveTarget
```

**Fields**: `range`

**Derives**: Debug

---

## InvalidLhsOfAssignment

`struct` · `ra_ap_hir::diagnostics::InvalidLhsOfAssignment`

Also reachable as `ra_ap_hir::InvalidLhsOfAssignment`

```rust
struct InvalidLhsOfAssignment
```

**Fields**: `lhs`

**Derives**: Debug

---

## InvalidRangePatType

`struct` · `ra_ap_hir::diagnostics::InvalidRangePatType`

Also reachable as `ra_ap_hir::InvalidRangePatType`

```rust
struct InvalidRangePatType
```

**Fields**: `pat`

**Derives**: Debug

---

## MacroDefError

`struct` · `ra_ap_hir::diagnostics::MacroDefError`

Also reachable as `ra_ap_hir::MacroDefError`

```rust
struct MacroDefError
```

**Fields**: `node`, `message`, `name`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## MacroError

`struct` · `ra_ap_hir::diagnostics::MacroError`

Also reachable as `ra_ap_hir::MacroError`

```rust
struct MacroError
```

**Fields**: `range`, `message`, `error`, `kind`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## MacroExpansionParseError

`struct` · `ra_ap_hir::diagnostics::MacroExpansionParseError`

Also reachable as `ra_ap_hir::MacroExpansionParseError`

```rust
struct MacroExpansionParseError
```

**Fields**: `range`, `errors`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## MalformedDerive

`struct` · `ra_ap_hir::diagnostics::MalformedDerive`

Also reachable as `ra_ap_hir::MalformedDerive`

```rust
struct MalformedDerive
```

**Fields**: `range`

**Derives**: Debug

---

## MethodCallIllegalSizedBound

`struct` · `ra_ap_hir::diagnostics::MethodCallIllegalSizedBound`

Also reachable as `ra_ap_hir::MethodCallIllegalSizedBound`

```rust
struct MethodCallIllegalSizedBound
```

**Fields**: `call_expr`

**Derives**: Debug

---

## MismatchedArgCount

`struct` · `ra_ap_hir::diagnostics::MismatchedArgCount`

Also reachable as `ra_ap_hir::MismatchedArgCount`

```rust
struct MismatchedArgCount
```

**Fields**: `call_expr`, `expected`, `found`, `is_fn_trait_call`

**Derives**: Debug

---

## MismatchedArrayPatLen

`struct` · `ra_ap_hir::diagnostics::MismatchedArrayPatLen`

Also reachable as `ra_ap_hir::MismatchedArrayPatLen`

```rust
struct MismatchedArrayPatLen
```

**Fields**: `pat`, `expected`, `found`, `has_rest`

**Derives**: Debug

---

## MismatchedTupleStructPatArgCount

`struct` · `ra_ap_hir::diagnostics::MismatchedTupleStructPatArgCount`

Also reachable as `ra_ap_hir::MismatchedTupleStructPatArgCount`

```rust
struct MismatchedTupleStructPatArgCount
```

**Fields**: `expr_or_pat`, `expected`, `found`

**Derives**: Debug

---

## MissingBody

`struct` · `ra_ap_hir::diagnostics::MissingBody`

Also reachable as `ra_ap_hir::MissingBody`

```rust
struct MissingBody
```

**Fields**: `node`, `kind`

**Derives**: Debug

---

## MissingFields

`struct` · `ra_ap_hir::diagnostics::MissingFields`

Also reachable as `ra_ap_hir::MissingFields`

```rust
struct MissingFields
```

**Fields**: `file`, `field_list_parent`, `field_list_parent_path`, `missed_fields`

**Derives**: Debug

---

## MissingLifetime

`struct` · `ra_ap_hir::diagnostics::MissingLifetime`

Also reachable as `ra_ap_hir::MissingLifetime`

```rust
struct MissingLifetime
```

**Fields**: `generics_or_segment`, `expected`, `def`

**Derives**: Debug

---

## MissingMatchArms

`struct` · `ra_ap_hir::diagnostics::MissingMatchArms`

Also reachable as `ra_ap_hir::MissingMatchArms`

```rust
struct MissingMatchArms
```

**Fields**: `scrutinee_expr`, `uncovered_patterns`

**Derives**: Debug

---

## MissingUnsafe

`struct` · `ra_ap_hir::diagnostics::MissingUnsafe`

Also reachable as `ra_ap_hir::MissingUnsafe`

```rust
struct MissingUnsafe
```

**Fields**: `node`, `lint`, `reason`

**Derives**: Debug

---

## MutRefInImmRefPat

`struct` · `ra_ap_hir::diagnostics::MutRefInImmRefPat`

Also reachable as `ra_ap_hir::MutRefInImmRefPat`

```rust
struct MutRefInImmRefPat
```

**Fields**: `pat`

**Derives**: Debug

---

## MutableRefBinding

`struct` · `ra_ap_hir::diagnostics::MutableRefBinding`

Also reachable as `ra_ap_hir::MutableRefBinding`

```rust
struct MutableRefBinding
```

**Fields**: `pat`

**Derives**: Debug

---

## NoSuchField

`struct` · `ra_ap_hir::diagnostics::NoSuchField`

Also reachable as `ra_ap_hir::NoSuchField`

```rust
struct NoSuchField
```

**Fields**: `field`, `private`, `variant`

**Derives**: Debug

---

## NonExhaustiveLet

`struct` · `ra_ap_hir::diagnostics::NonExhaustiveLet`

Also reachable as `ra_ap_hir::NonExhaustiveLet`

```rust
struct NonExhaustiveLet
```

**Fields**: `pat`, `uncovered_patterns`

**Derives**: Debug

---

## NonExhaustiveRecordExpr

`struct` · `ra_ap_hir::diagnostics::NonExhaustiveRecordExpr`

Also reachable as `ra_ap_hir::NonExhaustiveRecordExpr`

```rust
struct NonExhaustiveRecordExpr
```

**Fields**: `expr`

**Derives**: Debug

---

## NonExhaustiveRecordPat

`struct` · `ra_ap_hir::diagnostics::NonExhaustiveRecordPat`

Also reachable as `ra_ap_hir::NonExhaustiveRecordPat`

```rust
struct NonExhaustiveRecordPat
```

**Fields**: `pat`, `variant`

**Derives**: Debug

---

## ParenthesizedGenericArgsWithoutFnTrait

`struct` · `ra_ap_hir::diagnostics::ParenthesizedGenericArgsWithoutFnTrait`

Also reachable as `ra_ap_hir::ParenthesizedGenericArgsWithoutFnTrait`

```rust
struct ParenthesizedGenericArgsWithoutFnTrait
```

**Fields**: `args`

**Derives**: Debug

---

## PatternArgInExternFn

`struct` · `ra_ap_hir::diagnostics::PatternArgInExternFn`

Also reachable as `ra_ap_hir::PatternArgInExternFn`

```rust
struct PatternArgInExternFn
```

**Fields**: `node`

**Derives**: Debug

---

## PrivateAssocItem

`struct` · `ra_ap_hir::diagnostics::PrivateAssocItem`

Also reachable as `ra_ap_hir::PrivateAssocItem`

```rust
struct PrivateAssocItem
```

**Fields**: `expr_or_pat`, `item`

**Derives**: Debug

---

## PrivateField

`struct` · `ra_ap_hir::diagnostics::PrivateField`

Also reachable as `ra_ap_hir::PrivateField`

```rust
struct PrivateField
```

**Fields**: `expr`, `field`

**Derives**: Debug

---

## RemoveTrailingReturn

`struct` · `ra_ap_hir::diagnostics::RemoveTrailingReturn`

Also reachable as `ra_ap_hir::RemoveTrailingReturn`

```rust
struct RemoveTrailingReturn
```

**Fields**: `return_expr`

**Derives**: Debug

---

## RemoveUnnecessaryElse

`struct` · `ra_ap_hir::diagnostics::RemoveUnnecessaryElse`

Also reachable as `ra_ap_hir::RemoveUnnecessaryElse`

```rust
struct RemoveUnnecessaryElse
```

**Fields**: `if_expr`

**Derives**: Debug

---

## ReplaceFilterMapNextWithFindMap

`struct` · `ra_ap_hir::diagnostics::ReplaceFilterMapNextWithFindMap`

Also reachable as `ra_ap_hir::ReplaceFilterMapNextWithFindMap`

```rust
struct ReplaceFilterMapNextWithFindMap
```

**Fields**: `file`, `next_expr`

**Derives**: Debug

---

## ReturnOutsideFunction

`struct` · `ra_ap_hir::diagnostics::ReturnOutsideFunction`

Also reachable as `ra_ap_hir::ReturnOutsideFunction`

```rust
struct ReturnOutsideFunction
```

**Fields**: `expr`, `kind`

**Derives**: Debug

---

## TraitImplIncorrectSafety

`struct` · `ra_ap_hir::diagnostics::TraitImplIncorrectSafety`

Also reachable as `ra_ap_hir::TraitImplIncorrectSafety`

```rust
struct TraitImplIncorrectSafety
```

**Fields**: `file_id`, `impl_`, `should_be_safe`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

---

## TraitImplMissingAssocItems

`struct` · `ra_ap_hir::diagnostics::TraitImplMissingAssocItems`

Also reachable as `ra_ap_hir::TraitImplMissingAssocItems`

```rust
struct TraitImplMissingAssocItems
```

**Fields**: `file_id`, `impl_`, `missing`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

---

## TraitImplOrphan

`struct` · `ra_ap_hir::diagnostics::TraitImplOrphan`

Also reachable as `ra_ap_hir::TraitImplOrphan`

```rust
struct TraitImplOrphan
```

**Fields**: `file_id`, `impl_`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

---

## TraitImplRedundantAssocItems

`struct` · `ra_ap_hir::diagnostics::TraitImplRedundantAssocItems`

Also reachable as `ra_ap_hir::TraitImplRedundantAssocItems`

```rust
struct TraitImplRedundantAssocItems
```

**Fields**: `file_id`, `trait_`, `impl_`, `assoc_item`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

---

## TypeMismatch

`struct` · `ra_ap_hir::diagnostics::TypeMismatch`

Also reachable as `ra_ap_hir::TypeMismatch`

```rust
struct TypeMismatch<'db>
```

**Fields**: `expr_or_pat`, `expected`, `actual`

**Derives**: Debug

---

## TypeMustBeKnown

`struct` · `ra_ap_hir::diagnostics::TypeMustBeKnown`

Also reachable as `ra_ap_hir::TypeMustBeKnown`

```rust
struct TypeMustBeKnown<'db>
```

**Fields**: `at_point`, `top_term`

**Derives**: Debug

---

## TypedHole

`struct` · `ra_ap_hir::diagnostics::TypedHole`

Also reachable as `ra_ap_hir::TypedHole`

```rust
struct TypedHole<'db>
```

**Fields**: `expr`, `expected`

**Derives**: Debug

---

## UnaryOperatorCannotBeApplied

`struct` · `ra_ap_hir::diagnostics::UnaryOperatorCannotBeApplied`

Also reachable as `ra_ap_hir::UnaryOperatorCannotBeApplied`

```rust
struct UnaryOperatorCannotBeApplied<'db>
```

**Fields**: `expr`, `op`, `found`

**Derives**: Debug

---

## UndeclaredLabel

`struct` · `ra_ap_hir::diagnostics::UndeclaredLabel`

Also reachable as `ra_ap_hir::UndeclaredLabel`

```rust
struct UndeclaredLabel
```

**Fields**: `node`, `name`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## UnimplementedBuiltinMacro

`struct` · `ra_ap_hir::diagnostics::UnimplementedBuiltinMacro`

Also reachable as `ra_ap_hir::UnimplementedBuiltinMacro`

```rust
struct UnimplementedBuiltinMacro
```

**Fields**: `node`

**Derives**: Debug

---

## UnimplementedTrait

`struct` · `ra_ap_hir::diagnostics::UnimplementedTrait`

Also reachable as `ra_ap_hir::UnimplementedTrait`

```rust
struct UnimplementedTrait<'db>
```

**Fields**: `span`, `trait_predicate`, `parent_trait_predicates`

**Derives**: Debug

---

## UnionExprMustHaveExactlyOneField

`struct` · `ra_ap_hir::diagnostics::UnionExprMustHaveExactlyOneField`

Also reachable as `ra_ap_hir::UnionExprMustHaveExactlyOneField`

```rust
struct UnionExprMustHaveExactlyOneField
```

**Fields**: `expr`

**Derives**: Debug

---

## UnionPatHasRest

`struct` · `ra_ap_hir::diagnostics::UnionPatHasRest`

Also reachable as `ra_ap_hir::UnionPatHasRest`

```rust
struct UnionPatHasRest
```

**Fields**: `pat`

**Derives**: Debug

---

## UnionPatMustHaveExactlyOneField

`struct` · `ra_ap_hir::diagnostics::UnionPatMustHaveExactlyOneField`

Also reachable as `ra_ap_hir::UnionPatMustHaveExactlyOneField`

```rust
struct UnionPatMustHaveExactlyOneField
```

**Fields**: `pat`

**Derives**: Debug

---

## UnreachableLabel

`struct` · `ra_ap_hir::diagnostics::UnreachableLabel`

Also reachable as `ra_ap_hir::UnreachableLabel`

```rust
struct UnreachableLabel
```

**Fields**: `node`, `name`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## UnresolvedAssocItem

`struct` · `ra_ap_hir::diagnostics::UnresolvedAssocItem`

Also reachable as `ra_ap_hir::UnresolvedAssocItem`

```rust
struct UnresolvedAssocItem
```

**Fields**: `expr_or_pat`

**Derives**: Debug

---

## UnresolvedExternCrate

`struct` · `ra_ap_hir::diagnostics::UnresolvedExternCrate`

Also reachable as `ra_ap_hir::UnresolvedExternCrate`

```rust
struct UnresolvedExternCrate
```

**Fields**: `decl`

**Derives**: Debug

---

## UnresolvedField

`struct` · `ra_ap_hir::diagnostics::UnresolvedField`

Also reachable as `ra_ap_hir::UnresolvedField`

```rust
struct UnresolvedField<'db>
```

**Fields**: `expr`, `receiver`, `name`, `method_with_same_name_exists`

**Derives**: Debug

---

## UnresolvedIdent

`struct` · `ra_ap_hir::diagnostics::UnresolvedIdent`

Also reachable as `ra_ap_hir::UnresolvedIdent`

```rust
struct UnresolvedIdent
```

**Fields**: `node`

**Derives**: Debug

---

## UnresolvedImport

`struct` · `ra_ap_hir::diagnostics::UnresolvedImport`

Also reachable as `ra_ap_hir::UnresolvedImport`

```rust
struct UnresolvedImport
```

**Fields**: `decl`

**Derives**: Debug

---

## UnresolvedMacroCall

`struct` · `ra_ap_hir::diagnostics::UnresolvedMacroCall`

Also reachable as `ra_ap_hir::UnresolvedMacroCall`

```rust
struct UnresolvedMacroCall
```

**Fields**: `range`, `path`, `is_bang`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## UnresolvedMethodCall

`struct` · `ra_ap_hir::diagnostics::UnresolvedMethodCall`

Also reachable as `ra_ap_hir::UnresolvedMethodCall`

```rust
struct UnresolvedMethodCall<'db>
```

**Fields**: `expr`, `receiver`, `name`, `field_with_same_name`, `assoc_func_with_same_name`

**Derives**: Debug

---

## UnresolvedModule

`struct` · `ra_ap_hir::diagnostics::UnresolvedModule`

Also reachable as `ra_ap_hir::UnresolvedModule`

```rust
struct UnresolvedModule
```

**Fields**: `decl`, `candidates`

**Derives**: Debug

---

## UnusedMustUse

`struct` · `ra_ap_hir::diagnostics::UnusedMustUse`

Also reachable as `ra_ap_hir::UnusedMustUse`

```rust
struct UnusedMustUse<'db>
```

**Fields**: `expr`, `message`

**Derives**: Debug

---

## YieldOutsideCoroutine

`struct` · `ra_ap_hir::diagnostics::YieldOutsideCoroutine`

Also reachable as `ra_ap_hir::YieldOutsideCoroutine`

```rust
struct YieldOutsideCoroutine
```

**Fields**: `expr`

**Derives**: Debug

---

## SpanSyntax

`type_alias` · `ra_ap_hir::diagnostics::SpanSyntax`

Also reachable as `ra_ap_hir::SpanSyntax`

```rust
type SpanSyntax = hir_expand::InFile<syntax::AstPtr<SpanAst>>
```

---
