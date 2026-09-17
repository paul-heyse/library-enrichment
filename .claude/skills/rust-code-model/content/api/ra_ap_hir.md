# `ra_ap_hir`

Crate `ra_ap_hir` · 88 public items · structured records in [`model/ra_ap_hir.json`](../model/ra_ap_hir.json)

## Access

`enum` · `ra_ap_hir::Access`

```rust
enum Access
```

**Variants**: `Shared`, `Exclusive`, `Owned`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(mutability: hir_ty::next_solver::Mutability) -> Access
```

---

## Adjust

`enum` · `ra_ap_hir::Adjust`

```rust
enum Adjust
```

**Variants**: `NeverToAny`, `Deref`, `Borrow`, `Pointer`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Adt

`enum` · `ra_ap_hir::Adt`

```rust
enum Adt
```

**Variants**: `Struct`, `Union`, `Enum`

**Implements**: `core::convert::From`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn as_enum(&self) -> Option<Enum>
fn as_struct(&self) -> Option<Struct>
fn has_non_default_type_params(self, db: &dyn HirDatabase) -> bool
fn layout<'db>(self, db: &'db dyn HirDatabase) -> Result<Layout<'db>, LayoutError>
fn lifetime(&self, db: &dyn HirDatabase) -> Option<LifetimeParamData>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(it: Enum) -> Adt
fn from(value: AdtId) -> Self
fn from(it: Struct) -> Adt
fn from(it: Union) -> Adt
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

A Data Type

---

## AssocItem

`enum` · `ra_ap_hir::AssocItem`

```rust
enum AssocItem
```

**Variants**: `Function`, `Const`, `TypeAlias`

**Implements**: `core::convert::From`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn as_const(self) -> Option<Const>
fn as_function(self) -> Option<Function>
fn as_type_alias(self) -> Option<TypeAlias>
fn container(self, db: &dyn HirDatabase) -> AssocItemContainer
fn container_or_implemented_trait(self, db: &dyn HirDatabase) -> Option<Trait>
fn container_trait(self, db: &dyn HirDatabase) -> Option<Trait>
fn implemented_trait(self, db: &dyn HirDatabase) -> Option<Trait>
fn implementing_ty(self, db: &dyn HirDatabase) -> Option<Type<'_>>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Option<Name>
```

**via `core::convert::From`**

```rust
fn from(value: method_resolution::CandidateId) -> Self
fn from(value: AssocItemId) -> Self
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

Invariant: `inner.as_assoc_item(db).is_some()`
We do not actively enforce this invariant.

---

## AssocItemContainer

`enum` · `ra_ap_hir::AssocItemContainer`

```rust
enum AssocItemContainer
```

**Variants**: `Trait`, `Impl`

**Derives**: Clone, Debug

---

## AutoBorrow

`enum` · `ra_ap_hir::AutoBorrow`

```rust
enum AutoBorrow
```

**Variants**: `Ref`, `RawPtr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## BindingMode

`enum` · `ra_ap_hir::BindingMode`

```rust
enum BindingMode
```

**Variants**: `Move`, `Ref`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## CallableKind

`enum` · `ra_ap_hir::CallableKind`

```rust
enum CallableKind<'db>
```

**Variants**: `Function`, `TupleStruct`, `TupleEnumVariant`, `Closure`, `FnPtr`, `FnImpl`

---

## CaptureKind

`enum` · `ra_ap_hir::CaptureKind`

```rust
enum CaptureKind
```

**Variants**: `SharedRef`, `UniqueSharedRef`, `MutableRef`, `Move`

**Derives**: Clone, Copy, Eq, PartialEq, StructuralPartialEq

---

## DefWithBody

`enum` · `ra_ap_hir::DefWithBody`

```rust
enum DefWithBody
```

**Variants**: `Function`, `Static`, `Const`, `EnumVariant`

**Implements**: `core::convert::From`, `ra_ap_hir::AsAssocItem`, `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn binding_types<'db>(self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Type<'db>>
fn body_type(self, db: &dyn HirDatabase) -> Type<'_>
fn debug_hir(self, db: &dyn HirDatabase) -> String
fn debug_mir(self, db: &dyn HirDatabase) -> String
fn expression_types<'db>(self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Type<'db>>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Option<Name>
fn pattern_types<'db>(self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Type<'db>>
fn run_mir_body(self, db: &dyn HirDatabase) -> Result<(), MirLowerError<'_>>
```

**via `core::convert::From`**

```rust
fn from(it: Const) -> DefWithBody
fn from(it: Function) -> DefWithBody
fn from(it: Static) -> DefWithBody
fn from(it: EnumVariant) -> DefWithBody
fn from(value: DefWithBodyId) -> Self
```

**via `ra_ap_hir::AsAssocItem`**

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

The defs which have a body.

---

## DocLinkDef

`enum` · `ra_ap_hir::DocLinkDef`

```rust
enum DocLinkDef
```

**Variants**: `ModuleDef`, `Field`, `SelfType`

Subset of `ide_db::Definition` that doc links can resolve to.

---

## ExpressionStoreOwner

`enum` · `ra_ap_hir::ExpressionStoreOwner`

```rust
enum ExpressionStoreOwner
```

**Variants**: `Body`, `Signature`, `VariantFields`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn module(self, db: &dyn HirDatabase) -> Module
```

**via `core::convert::From`**

```rust
fn from(value: ExpressionStoreOwnerId) -> Self
fn from(v: DefWithBody) -> Self
fn from(v: GenericDef) -> Self
```

---

## ExternAssocItem

`enum` · `ra_ap_hir::ExternAssocItem`

```rust
enum ExternAssocItem
```

**Variants**: `Function`, `Static`, `TypeAlias`

**Implements**: `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn as_function(self) -> Option<Function>
fn as_static(self) -> Option<Static>
fn as_type_alias(self) -> Option<TypeAlias>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

Invariant: `inner.as_extern_assoc_item(db).is_some()`
We do not actively enforce this invariant.

---

## FieldSource

`enum` · `ra_ap_hir::FieldSource`

```rust
enum FieldSource
```

**Variants**: `Named`, `Pos`

**Implements**: `ra_ap_syntax::ast::AstNode`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: syntax::SyntaxKind) -> bool where Self: Sized
fn cast(syntax: SyntaxNode) -> Option<Self> where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## FnTrait

`enum` · `ra_ap_hir::FnTrait`

```rust
enum FnTrait
```

**Variants**: `FnOnce`, `FnMut`, `Fn`, `AsyncFnOnce`, `AsyncFnMut`, `AsyncFn`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn function_name(&self) -> &'static str
fn get_id(self, db: &dyn HirDatabase, krate: Crate) -> Option<Trait>
fn lang_item(self) -> LangItem
```

**via `core::convert::From`**

```rust
fn from(value: traits::FnTrait) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## GenericDef

`enum` · `ra_ap_hir::GenericDef`

```rust
enum GenericDef
```

**Variants**: `Function`, `Adt`, `Trait`, `TypeAlias`, `Impl`, `Const`, `Static`

**Implements**: `core::convert::From`, `ra_ap_hir::AsAssocItem`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn description(self) -> &'static str
fn lifetime_params(self, db: &dyn HirDatabase) -> Vec<LifetimeParam>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Option<Name>
fn params(self, db: &dyn HirDatabase) -> Vec<GenericParam>
fn type_or_const_params(self, db: &dyn HirDatabase) -> Vec<TypeOrConstParam>
```

**via `core::convert::From`**

```rust
fn from(it: Struct) -> GenericDef
fn from(it: Enum) -> GenericDef
fn from(value: GenericDefId) -> Self
fn from(it: TypeAlias) -> GenericDef
fn from(it: Union) -> GenericDef
fn from(it: Impl) -> GenericDef
fn from(it: Trait) -> GenericDef
fn from(it: Const) -> GenericDef
fn from(it: Function) -> GenericDef
fn from(it: Static) -> GenericDef
fn from(it: Adt) -> GenericDef
```

**via `ra_ap_hir::AsAssocItem`**

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

---

## GenericParam

`enum` · `ra_ap_hir::GenericParam`

```rust
enum GenericParam
```

**Variants**: `TypeParam`, `ConstParam`, `LifetimeParam`

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self) -> GenericDef
fn variance(self, db: &dyn HirDatabase) -> Option<Variance>
```

**via `core::convert::From`**

```rust
fn from(it: TypeParam) -> GenericParam
fn from(value: GenericParamId) -> Self
fn from(it: LifetimeParam) -> GenericParam
fn from(it: ConstParam) -> GenericParam
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## InferBody

`enum` · `ra_ap_hir::InferBody`

```rust
enum InferBody<'db>
```

**Variants**: `Body`, `AnonConst`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ItemContainer

`enum` · `ra_ap_hir::ItemContainer`

```rust
enum ItemContainer
```

**Variants**: `Trait`, `Impl`, `Module`, `ExternBlock`, `Crate`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ItemInNs

`enum` · `ra_ap_hir::ItemInNs`

```rust
enum ItemInNs
```

**Variants**: `Types`, `Values`, `Macros`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn attrs(&self, db: &dyn HirDatabase) -> Option<AttrsWithOwner>
fn into_module_def(self) -> ModuleDef
fn krate(&self, db: &dyn HirDatabase) -> Option<Crate>
```

**via `core::convert::From`**

```rust
fn from(value: ModuleDef) -> Self
fn from(value: ItemInNsId) -> Self
fn from(it: Macro) -> Self
```

---

## MacroBraces

`enum` · `ra_ap_hir::MacroBraces`

```rust
enum MacroBraces
```

**Variants**: `Braces`, `Brackets`, `Parentheses`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ModuleDef

`enum` · `ra_ap_hir::ModuleDef`

```rust
enum ModuleDef
```

**Variants**: `Module`, `Function`, `Adt`, `EnumVariant`, `Const`, `Static`, `Trait`, `TypeAlias`, `BuiltinType`, `Macro`

**Implements**: `core::convert::From`, `ra_ap_hir::AsAssocItem`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn as_def_with_body(self) -> Option<DefWithBody>
fn as_generic_def(self) -> Option<GenericDef>
fn as_self_generic_def(self) -> Option<GenericDef>
fn attrs(&self, db: &dyn HirDatabase) -> Option<AttrsWithOwner>
fn canonical_module_path(&self, db: &dyn HirDatabase) -> Option<impl Iterator<Item = Module>>
fn canonical_path(&self, db: &dyn HirDatabase, edition: Edition) -> Option<String>
fn module(self, db: &dyn HirDatabase) -> Option<Module>
fn name(self, db: &dyn HirDatabase) -> Option<Name>
```

**via `core::convert::From`**

```rust
fn from(it: Module) -> ModuleDef
fn from(it: BuiltinType) -> ModuleDef
fn from(value: AssocItem) -> Self
fn from(it: Function) -> ModuleDef
fn from(it: Enum) -> ModuleDef
fn from(it: Const) -> ModuleDef
fn from(it: TypeAlias) -> ModuleDef
fn from(value: Variant) -> Self
fn from(it: Struct) -> ModuleDef
fn from(it: EnumVariant) -> ModuleDef
fn from(it: Trait) -> ModuleDef
fn from(value: ModuleDefId) -> Self
fn from(it: Macro) -> ModuleDef
fn from(it: Adt) -> ModuleDef
fn from(it: Union) -> ModuleDef
fn from(it: Static) -> ModuleDef
```

**via `ra_ap_hir::AsAssocItem`**

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

The defs which can be visible in the module.

---

## PredicateEvaluationStatus

`enum` · `ra_ap_hir::PredicateEvaluationStatus`

Also reachable as `ra_ap_ide::PredicateEvaluationStatus`

```rust
enum PredicateEvaluationStatus
```

**Variants**: `Holds`, `NotProven`, `Invalid`, `Unsupported`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## PredicatePolarity

`enum` · `ra_ap_hir::PredicatePolarity`

```rust
enum PredicatePolarity
```

**Variants**: `Positive`, `Negative`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ScopeDef

`enum` · `ra_ap_hir::ScopeDef`

```rust
enum ScopeDef<'db>
```

**Variants**: `ModuleDef`, `GenericParam`, `ImplSelfType`, `AdtSelfType`, `Local`, `Label`, `Unknown`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn all_items(def: PerNs) -> ArrayVec<Self, 3>
fn attrs(&self, db: &dyn HirDatabase) -> Option<AttrsWithOwner>
fn krate(&self, db: &dyn HirDatabase) -> Option<Crate>
```

**via `core::convert::From`**

```rust
fn from(value: ItemInNs) -> Self
```

For IDE only

---

## StructKind

`enum` · `ra_ap_hir::StructKind`

```rust
enum StructKind
```

**Variants**: `Record`, `Tuple`, `Unit`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Variance

`enum` · `ra_ap_hir::Variance`

```rust
enum Variance
```

**Variants**: `Bivariant`, `Covariant`, `Contravariant`, `Invariant`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: rustc_type_ir::Variance) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## Variant

`enum` · `ra_ap_hir::Variant`

```rust
enum Variant
```

**Variants**: `Struct`, `Union`, `EnumVariant`

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::has_source::HasSource`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn adt(&self, db: &dyn HirDatabase) -> Adt
fn fields(self, db: &dyn HirDatabase) -> Vec<Field>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(&self, db: &dyn HirDatabase) -> Name
```

**via `core::convert::From`**

```rust
fn from(value: VariantId) -> Self
fn from(it: EnumVariant) -> Variant
fn from(it: Struct) -> Variant
fn from(it: Union) -> Variant
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

---

## resolve_absolute_path

`function` · `ra_ap_hir::resolve_absolute_path`

```rust
fn resolve_absolute_path<'a, I: Iterator<Item = Symbol> + Clone + 'a>(db: &'a dyn HirDatabase, segments: I) -> impl Iterator<Item = ItemInNs> + use<{'lifetime': "'a"}, {'param': 'I'}>
```

---

## Adjustment

`struct` · `ra_ap_hir::Adjustment`

```rust
struct Adjustment<'db>
```

**Fields**: `source`, `target`, `kind`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## AnonConst

`struct` · `ra_ap_hir::AnonConst`

```rust
struct AnonConst<'db>
```

**Implements**: `ra_ap_hir::HasCrate`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn eval(self, db: &'db dyn HirDatabase) -> Result<EvaluatedConst<'db>, ConstEvalError<'db>>
fn owner(self, db: &dyn HirDatabase) -> ExpressionStoreOwner
fn ty(self, db: &'db dyn HirDatabase) -> Type<'db>
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

---

## BuiltinAttr

`struct` · `ra_ap_hir::BuiltinAttr`

```rust
struct BuiltinAttr
```

**Implements**: `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> Name
fn template(&self) -> Option<AttributeTemplate>
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, _db: &dyn HirDatabase) -> Option<Name>
```

---

## BuiltinDeriveMacroKind

`struct` · `ra_ap_hir::BuiltinDeriveMacroKind`

```rust
struct BuiltinDeriveMacroKind
```

**Derives**: Clone, Copy, Eq, Hash, PartialEq, StructuralPartialEq

---

## BuiltinType

`struct` · `ra_ap_hir::BuiltinType`

```rust
struct BuiltinType
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn bool() -> BuiltinType
fn i32() -> BuiltinType
fn is_bool(&self) -> bool
fn is_char(&self) -> bool
fn is_f128(&self) -> bool
fn is_f16(&self) -> bool
fn is_f32(&self) -> bool
fn is_f64(&self) -> bool
fn is_float(&self) -> bool
fn is_int(&self) -> bool
fn is_str(&self) -> bool
fn is_uint(&self) -> bool
fn name(self) -> Name
fn str() -> BuiltinType
fn ty<'db>(self, db: &'db dyn HirDatabase) -> Type<'db>
```

**via `core::convert::From`**

```rust
fn from(inner: hir_def::builtin_type::BuiltinType) -> Self
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, _db: &dyn HirDatabase) -> Option<Name>
```

---

## Callable

`struct` · `ra_ap_hir::Callable`

```rust
struct Callable<'db>
```

**Derives**: Debug

**Methods** (7)

```rust
fn kind(&self) -> CallableKind<'db>
fn n_params(&self) -> usize
fn params(&self) -> Vec<Param<'db>>
fn receiver_param(&self, db: &'db dyn HirDatabase) -> Option<(SelfParam, Type<'db>)>
fn return_type(&self) -> Type<'db>
fn sig(&self) -> impl Eq
fn ty(&self) -> &Type<'db>
```

---

## CaptureUsageSource

`struct` · `ra_ap_hir::CaptureUsageSource`

```rust
struct CaptureUsageSource
```

**Derives**: Debug

**Methods** (3)

```rust
fn file_id(&self) -> HirFileId
fn is_ref(&self) -> bool
fn source(&self) -> AstPtr<Either<ast::Expr, ast::Pat>>
```

---

## CaptureUsages

`struct` · `ra_ap_hir::CaptureUsages`

```rust
struct CaptureUsages<'db>
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn sources(&self, db: &dyn HirDatabase) -> Vec<CaptureUsageSource>
```

---

## Closure

`struct` · `ra_ap_hir::Closure`

```rust
struct Closure<'db>
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn captured_items(&self, db: &'db dyn HirDatabase) -> Vec<ClosureCapture<'db>>
fn display_with_id(&self, db: &dyn HirDatabase, display_target: DisplayTarget) -> String
fn display_with_impl(&self, db: &dyn HirDatabase, display_target: DisplayTarget) -> String
fn fn_trait(&self, _db: &dyn HirDatabase) -> FnTrait
```

---

## ClosureCapture

`struct` · `ra_ap_hir::ClosureCapture`

```rust
struct ClosureCapture<'db>
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn captured_ty(&self, db: &'db dyn HirDatabase) -> Type<'db>
fn display_place_source_code(&self, db: &dyn HirDatabase, edition: Edition) -> String
fn has_field_projections(&self) -> bool
fn kind(&self) -> CaptureKind
fn local(&self) -> Local<'db>
fn place_to_name(&self, db: &dyn HirDatabase, edition: Edition) -> String
fn ty(&self, db: &'db dyn HirDatabase) -> Type<'db>
fn usages(&self) -> CaptureUsages<'db>
```

---

## Const

`struct` · `ra_ap_hir::Const`

```rust
struct Const
```

**Implements**: `core::convert::From`, `ra_ap_hir::AsAssocItem`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn eval(self, db: &dyn HirDatabase) -> Result<EvaluatedConst<'_>, ConstEvalError<'_>>
fn has_body(self, db: &dyn HirDatabase) -> bool
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Option<Name>
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
fn value(self, db: &dyn HirDatabase) -> Option<ast::Expr>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::ConstId) -> Const
```

**via `ra_ap_hir::AsAssocItem`**

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## ConstParam

`struct` · `ra_ap_hir::ConstParam`

```rust
struct ConstParam
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn default(self, db: &dyn HirDatabase, display_target: DisplayTarget) -> Option<String>
fn default_source_code(self, db: &dyn HirDatabase, target_module: Module) -> Option<ast::ConstArg>
fn merge(self) -> TypeOrConstParam
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self, _db: &dyn HirDatabase) -> GenericDef
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::ConstParamId) -> ConstParam
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Coroutine

`struct` · `ra_ap_hir::Coroutine`

```rust
struct Coroutine<'db>
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn captured_items(&self, db: &'db dyn HirDatabase) -> Vec<ClosureCapture<'db>>
```

A coroutine expression, including async, generator, and async-generator coroutines.

---

## Crate

`struct` · `ra_ap_hir::Crate`

```rust
struct Crate
```

**Implements**: `core::convert::From`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (20)

```rust
fn all(db: &dyn HirDatabase) -> Vec<Crate>
fn base(self) -> base_db::Crate
fn cfg<'db>(&self, db: &'db dyn HirDatabase) -> &'db CfgOptions
fn dependencies(self, db: &dyn HirDatabase) -> Vec<CrateDependency>
fn display_name(self, db: &dyn HirDatabase) -> Option<CrateDisplayName>
fn edition(self, db: &dyn HirDatabase) -> Edition
fn get_html_root_url(self, db: &dyn HirDatabase) -> Option<String>
fn is_builtin(self, db: &dyn HirDatabase) -> bool
fn is_unstable_feature_enabled(self, db: &dyn HirDatabase, feature: &Symbol) -> bool
fn modules(self, db: &dyn HirDatabase) -> Vec<Module>
fn notable_traits_in_deps(self, db: &dyn HirDatabase) -> impl Iterator<Item = &TraitId>
fn origin(self, db: &dyn HirDatabase) -> CrateOrigin
fn potential_cfg<'db>(&self, db: &'db dyn HirDatabase) -> &'db CfgOptions
fn query_external_importables(self, db: &dyn SourceDatabase, query: import_map::Query) -> impl Iterator<Item = (Either<ModuleDef, Macro>, Complete)>
fn reverse_dependencies(self, db: &dyn HirDatabase) -> Vec<Crate>
fn root_file(self, db: &dyn HirDatabase) -> FileId
fn root_module(self, db: &dyn HirDatabase) -> Module
fn to_display_target(self, db: &dyn HirDatabase) -> DisplayTarget
fn transitive_reverse_dependencies(self, db: &dyn HirDatabase) -> impl Iterator<Item = Crate>
fn version(self, db: &dyn HirDatabase) -> Option<String>
```

**via `core::convert::From`**

```rust
fn from(id: base_db::Crate) -> Crate
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

hir::Crate describes a single crate. It's the main interface with which
a crate's dependencies interact. Mostly, it should be just a proxy for the
root module.

---

## CrateDependency

`struct` · `ra_ap_hir::CrateDependency`

```rust
struct CrateDependency
```

**Fields**: `krate`, `name`

**Derives**: Debug

---

## DeriveHelper

`struct` · `ra_ap_hir::DeriveHelper`

```rust
struct DeriveHelper
```

**Implements**: `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn derive(&self) -> Macro
fn name(&self, db: &dyn HirDatabase) -> Name
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

---

## Enum

`struct` · `ra_ap_hir::Enum`

```rust
struct Enum
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn is_data_carrying(self, db: &dyn HirDatabase) -> bool
fn is_unstable(self, db: &dyn HirDatabase) -> bool
fn layout<'db>(self, db: &'db dyn HirDatabase) -> Result<Layout<'db>, LayoutError>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn num_variants(self, db: &dyn HirDatabase) -> usize
fn repr(self, db: &dyn HirDatabase) -> Option<ReprOptions>
fn ty<'db>(self, db: &'db dyn HirDatabase) -> Type<'db>
fn variant_body_ty<'db>(self, db: &'db dyn HirDatabase) -> Type<'db>
fn variants(self, db: &dyn HirDatabase) -> Vec<EnumVariant>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::EnumId) -> Enum
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## EnumVariant

`struct` · `ra_ap_hir::EnumVariant`

```rust
struct EnumVariant
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn constructor_ty(self, db: &dyn HirDatabase) -> Type<'_>
fn eval(self, db: &dyn HirDatabase) -> Result<i128, ConstEvalError<'_>>
fn fields(self, db: &dyn HirDatabase) -> Vec<Field>
fn is_unstable(self, db: &dyn HirDatabase) -> bool
fn kind(self, db: &dyn HirDatabase) -> StructKind
fn layout<'db>(&self, db: &'db dyn HirDatabase) -> Result<Layout<'db>, LayoutError>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent_enum(self, db: &dyn HirDatabase) -> Enum
fn value(self, db: &dyn HirDatabase) -> Option<ast::Expr>
```

**via `core::convert::From`**

```rust
fn from(id: EnumVariantId) -> Self
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Variant>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## EvaluatedConst

`struct` · `ra_ap_hir::EvaluatedConst`

```rust
struct EvaluatedConst<'db>
```

**Methods** (2)

```rust
fn render(&self, db: &dyn HirDatabase, display_target: DisplayTarget) -> String
fn render_debug(&self, db: &'db dyn HirDatabase) -> Result<String, MirEvalError<'db>>
```

---

## ExternBlock

`struct` · `ra_ap_hir::ExternBlock`

```rust
struct ExternBlock
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn module(self, db: &dyn HirDatabase) -> Module
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::ExternBlockId) -> ExternBlock
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

---

## ExternCrateDecl

`struct` · `ra_ap_hir::ExternCrateDecl`

```rust
struct ExternCrateDecl
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn alias(self, db: &dyn HirDatabase) -> Option<ImportAlias>
fn alias_or_name(self, db: &dyn HirDatabase) -> Option<Name>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn resolved_crate(self, db: &dyn HirDatabase) -> Option<Crate>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::ExternCrateId) -> ExternCrateDecl
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Field

`struct` · `ra_ap_hir::Field`

```rust
struct Field
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn index(&self) -> usize
fn layout<'db>(&self, db: &'db dyn HirDatabase) -> Result<Layout<'db>, LayoutError>
fn name(&self, db: &dyn HirDatabase) -> Name
fn parent_def(&self, _db: &dyn HirDatabase) -> Variant
fn ty<'db>(&self, db: &'db dyn HirDatabase) -> Type<'db>
```

**via `core::convert::From`**

```rust
fn from(def: FieldId) -> Self
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Function

`struct` · `ra_ap_hir::Function`

```rust
struct Function
```

**Implements**: `core::convert::From`, `ra_ap_hir::AsAssocItem`, `ra_ap_hir::AsExternAssocItem`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (29)

```rust
fn as_proc_macro(self, db: &dyn HirDatabase) -> Option<Macro>
fn assoc_fn_params(self, db: &dyn HirDatabase) -> Vec<Param<'_>>
fn async_ret_type<'db>(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>
fn eval(self, db: &dyn HirDatabase, span_formatter: impl Fn(FileId, TextRange) -> String) -> Result<String, ConstEvalError<'_>>
fn exported_main(self, db: &dyn HirDatabase) -> bool
fn extern_block(self, db: &dyn HirDatabase) -> Option<ExternBlock>
fn fn_ptr_type(self, db: &dyn HirDatabase) -> Type<'_>
fn has_body(self, db: &dyn HirDatabase) -> bool
fn has_self_param(self, db: &dyn HirDatabase) -> bool
fn is_async(self, db: &dyn HirDatabase) -> bool
fn is_bench(self, db: &dyn HirDatabase) -> bool
fn is_const(self, db: &dyn HirDatabase) -> bool
fn is_ignore(self, db: &dyn HirDatabase) -> bool
fn is_main(self, db: &dyn HirDatabase) -> bool
fn is_test(self, db: &dyn HirDatabase) -> bool
fn is_unsafe(self, db: &dyn HirDatabase) -> bool
fn is_unsafe_to_call(self, db: &dyn HirDatabase, caller: Option<Function>, call_edition: Edition) -> bool
fn is_unstable(self, db: &dyn HirDatabase) -> bool
fn is_varargs(self, db: &dyn HirDatabase) -> bool
fn lang(db: &dyn HirDatabase, krate: Crate, lang_item: LangItem) -> Option<Function>
fn method_params(self, db: &dyn HirDatabase) -> Option<Vec<Param<'_>>>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn num_params(self, db: &dyn HirDatabase) -> usize
fn params_without_self(self, db: &dyn HirDatabase) -> Vec<Param<'_>>
fn ret_type(self, db: &dyn HirDatabase) -> Type<'_>
fn returns_impl_future(self, db: &dyn HirDatabase) -> bool
fn self_param(self, db: &dyn HirDatabase) -> Option<SelfParam>
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(value: hir_def::FunctionId) -> Self
```

**via `ra_ap_hir::AsAssocItem`**

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

**via `ra_ap_hir::AsExternAssocItem`**

```rust
fn as_extern_assoc_item(self, db: &dyn HirDatabase) -> Option<ExternAssocItem>
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
fn source_with_range(self, db: &dyn HirDatabase) -> Option<InFile<(TextRange, Option<Self::Ast>)>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## GenericSubstitution

`struct` · `ra_ap_hir::GenericSubstitution`

```rust
struct GenericSubstitution<'db>
```

**Derives**: Debug

**Methods** (1)

```rust
fn types(&self, db: &'db dyn HirDatabase) -> Vec<(Symbol, Type<'db>)>
```

---

## Impl

`struct` · `ra_ap_hir::Impl`

```rust
struct Impl
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasCrate`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn all_for_trait(db: &dyn HirDatabase, trait_: Trait) -> Vec<Impl>
fn all_for_type<'db>(db: &'db dyn HirDatabase, ty: Type<'db>) -> Vec<Impl>
fn all_in_crate(db: &dyn HirDatabase, krate: Crate) -> Vec<Impl>
fn all_in_module(db: &dyn HirDatabase, module: Module) -> Vec<Impl>
fn check_orphan_rules(self, db: &dyn HirDatabase) -> bool
fn is_negative(self, db: &dyn HirDatabase) -> bool
fn is_unsafe(self, db: &dyn HirDatabase) -> bool
fn items(self, db: &dyn HirDatabase) -> Vec<AssocItem>
fn module(self, db: &dyn HirDatabase) -> Module
fn self_ty(self, db: &dyn HirDatabase) -> Type<'_>
fn trait_(self, db: &dyn HirDatabase) -> Option<Trait>
fn trait_ref(self, db: &dyn HirDatabase) -> Option<TraitRef<'_>>
```

**via `core::convert::From`**

```rust
fn from(value: hir_def::ImplId) -> Self
fn from(value: BuiltinDeriveImplId) -> Self
fn from(id: hir_ty::next_solver::AnyImplId) -> Impl
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
fn source_with_range(self, db: &dyn HirDatabase) -> Option<InFile<(TextRange, Option<Self::Ast>)>>
```

---

## InlineAsmOperand

`struct` · `ra_ap_hir::InlineAsmOperand`

```rust
struct InlineAsmOperand
```

**Implements**: `ra_ap_hir::HasName`, `ra_ap_hir::has_source::HasSource`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
fn parent(self, _db: &dyn HirDatabase) -> ExpressionStoreOwner
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

---

## Label

`struct` · `ra_ap_hir::Label`

```rust
struct Label
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::has_source::HasSource`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self, _db: &dyn HirDatabase) -> ExpressionStoreOwner
```

**via `core::convert::From`**

```rust
fn from((parent, label_id): (ExpressionStoreOwnerId, LabelId)) -> Self
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

---

## Layout

`struct` · `ra_ap_hir::Layout`

```rust
struct Layout<'db>
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn align(&self) -> u64
fn enum_tag_size(&self) -> Option<usize>
fn field_offset(&self, field: Field) -> Option<u64>
fn largest_padding(&self, field_size: &mut impl FnMut(usize) -> Option<u64>) -> Option<u64>
fn niches(&self) -> Option<u128>
fn size(&self) -> u64
fn tail_padding(&self, field_size: &mut impl FnMut(usize) -> Option<u64>) -> Option<u64>
fn tuple_field_offset(&self, field: usize) -> Option<u64>
```

---

## LifetimeParam

`struct` · `ra_ap_hir::LifetimeParam`

```rust
struct LifetimeParam
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self, _db: &dyn HirDatabase) -> GenericDef
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::LifetimeParamId) -> LifetimeParam
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Local

`struct` · `ra_ap_hir::Local`

```rust
struct Local<'db>
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
fn as_id(self) -> u32
fn as_self_param(self, db: &dyn HirDatabase) -> Option<SelfParam>
fn is_mut(self, db: &dyn HirDatabase) -> bool
fn is_param(self, db: &dyn HirDatabase) -> bool
fn is_ref(self, db: &dyn HirDatabase) -> bool
fn is_self(self, db: &dyn HirDatabase) -> bool
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self, _db: &dyn HirDatabase) -> ExpressionStoreOwner
fn primary_source(self, db: &dyn HirDatabase) -> LocalSource<'db>
fn sources(self, db: &dyn HirDatabase) -> Vec<LocalSource<'db>>
fn ty(self, db: &'db dyn HirDatabase) -> Type<'db>
```

**via `core::convert::From`**

```rust
fn from((parent, binding_id): (DefWithBodyId, BindingId)) -> Self
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

A single local definition.

---

## LocalSource

`struct` · `ra_ap_hir::LocalSource`

```rust
struct LocalSource<'db>
```

**Fields**: `local`, `source`

**Implements**: `ra_ap_hir::has_source::HasSource`

**Methods** (7)

```rust
fn as_ident_pat(&self) -> Option<&ast::IdentPat>
fn file(&self) -> HirFileId
fn into_ident_pat(self) -> Option<ast::IdentPat>
fn name(&self) -> Option<InFile<ast::Name>>
fn original_file(&self, db: &dyn HirDatabase) -> EditionedFileId
fn syntax(&self) -> &SyntaxNode
fn syntax_ptr(self) -> InFile<SyntaxNodePtr>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, _: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

---

## Macro

`struct` · `ra_ap_hir::Macro`

```rust
struct Macro
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn builtin_derive_kind(&self, db: &dyn HirDatabase) -> Option<BuiltinDeriveMacroKind>
fn is_asm_like(&self, db: &dyn HirDatabase) -> bool
fn is_attr(&self, db: &dyn HirDatabase) -> bool
fn is_derive(&self, db: &dyn HirDatabase) -> bool
fn is_env_or_option_env(&self, db: &dyn HirDatabase) -> bool
fn is_fn_like(&self, db: &dyn HirDatabase) -> bool
fn is_proc_macro(self) -> bool
fn kind(&self, db: &dyn HirDatabase) -> MacroKind
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn preferred_brace_style(&self, db: &dyn HirDatabase) -> Option<MacroBraces>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::MacroId) -> Macro
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Module

`struct` · `ra_ap_hir::Module`

```rust
struct Module
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (28)

```rust
fn as_source_file_id(self, db: &dyn HirDatabase) -> Option<EditionedFileId>
fn children(self, db: &dyn HirDatabase) -> impl Iterator<Item = Module>
fn crate_root(self, db: &dyn HirDatabase) -> Module
fn declaration_source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Module>>
fn declaration_source_range(self, db: &dyn HirDatabase) -> Option<InFile<TextRange>>
fn declarations(self, db: &dyn HirDatabase) -> Vec<ModuleDef>
fn definition_source(self, db: &dyn HirDatabase) -> InFile<ModuleSource>
fn definition_source_file_id(self, db: &dyn HirDatabase) -> HirFileId
fn definition_source_range(self, db: &dyn HirDatabase) -> InFile<TextRange>
fn diagnostics<'db>(self, db: &'db dyn HirDatabase, acc: &mut Vec<AnyDiagnostic<'db>>, style_lints: bool)
fn doc_keyword(self, db: &dyn HirDatabase) -> Option<Symbol>
fn find_path(self, db: &dyn SourceDatabase, item: impl Into<ItemInNs>, cfg: FindPathConfig) -> Option<ModPath>
fn find_use_path(self, db: &dyn SourceDatabase, item: impl Into<ItemInNs>, prefix_kind: PrefixKind, cfg: FindPathConfig) -> Option<ModPath>
fn has_path(&self, db: &dyn HirDatabase) -> bool
fn impl_defs(self, db: &dyn HirDatabase) -> Vec<Impl>
fn is_crate_root(self, db: &dyn HirDatabase) -> bool
fn is_inline(self, db: &dyn HirDatabase) -> bool
fn is_mod_rs(self, db: &dyn HirDatabase) -> bool
fn krate(self, db: &dyn HirDatabase) -> Crate
fn legacy_macros(self, db: &dyn HirDatabase) -> Vec<Macro>
fn modules_in_scope(&self, db: &dyn HirDatabase, pub_only: bool) -> Vec<(Name, Module)>
fn name(self, db: &dyn HirDatabase) -> Option<Name>
fn nearest_non_block_module(self, db: &dyn HirDatabase) -> Module
fn parent(self, db: &dyn HirDatabase) -> Option<Module>
fn path_segments(self, db: &dyn HirDatabase) -> impl Iterator<Item = Name>
fn path_to_root(self, db: &dyn HirDatabase) -> Vec<Module>
fn resolve_mod_path(&self, db: &dyn HirDatabase, segments: impl IntoIterator<Item = Name>) -> Option<impl Iterator<Item = ItemInNs>>
fn scope(self, db: &dyn HirDatabase, visible_from: Option<Module>) -> Vec<(Name, ScopeDef<'_>)>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::ModuleId) -> Module
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## OverloadedDeref

`struct` · `ra_ap_hir::OverloadedDeref`

```rust
struct OverloadedDeref
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Param

`struct` · `ra_ap_hir::Param`

```rust
struct Param<'db>
```

**Implements**: `ra_ap_hir::HasName`, `ra_ap_hir::has_source::HasSource`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_local(&self, db: &'db dyn HirDatabase) -> Option<Local<'db>>
fn index(&self) -> usize
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
fn parent_fn(&self) -> Option<Function>
fn pattern_source(self, db: &dyn HirDatabase) -> Option<ast::Pat>
fn ty(&self) -> &Type<'db>
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

---

## PredicateEvaluationResult

`struct` · `ra_ap_hir::PredicateEvaluationResult`

Also reachable as `ra_ap_ide::PredicateEvaluationResult`

```rust
struct PredicateEvaluationResult
```

**Fields**: `status`, `message`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn holds(message: impl Into<String>) -> Self
fn invalid(message: impl Into<String>) -> Self
fn not_proven(message: impl Into<String>) -> Self
fn unsupported(message: impl Into<String>) -> Self
```

---

## SelfParam

`struct` · `ra_ap_hir::SelfParam`

```rust
struct SelfParam
```

**Implements**: `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn access(self, db: &dyn HirDatabase) -> Access
fn parent_fn(&self) -> Function
fn ty<'db>(&self, db: &'db dyn HirDatabase) -> Type<'db>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Static

`struct` · `ra_ap_hir::Static`

```rust
struct Static
```

**Implements**: `core::convert::From`, `ra_ap_hir::AsExternAssocItem`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn eval(self, db: &dyn HirDatabase) -> Result<EvaluatedConst<'_>, ConstEvalError<'_>>
fn extern_block(self, db: &dyn HirDatabase) -> Option<ExternBlock>
fn is_mut(self, db: &dyn HirDatabase) -> bool
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
fn value(self, db: &dyn HirDatabase) -> Option<ast::Expr>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::StaticId) -> Static
```

**via `ra_ap_hir::AsExternAssocItem`**

```rust
fn as_extern_assoc_item(self, db: &dyn HirDatabase) -> Option<ExternAssocItem>
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## StaticLifetime

`struct` · `ra_ap_hir::StaticLifetime`

```rust
struct StaticLifetime
```

**Implements**: `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn name(self) -> Name
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, _db: &dyn HirDatabase) -> Option<Name>
```

---

## Struct

`struct` · `ra_ap_hir::Struct`

```rust
struct Struct
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn constructor_ty(self, db: &dyn HirDatabase) -> Type<'_>
fn fields(self, db: &dyn HirDatabase) -> Vec<Field>
fn is_unstable(self, db: &dyn HirDatabase) -> bool
fn kind(self, db: &dyn HirDatabase) -> StructKind
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn repr(self, db: &dyn HirDatabase) -> Option<ReprOptions>
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::StructId) -> Struct
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## ToolModule

`struct` · `ra_ap_hir::ToolModule`

```rust
struct ToolModule
```

**Implements**: `ra_ap_hir::HasName`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn krate(&self) -> Crate
fn name(&self, db: &dyn HirDatabase) -> Name
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

---

## Trait

`struct` · `ra_ap_hir::Trait`

```rust
struct Trait
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn all_supertraits(self, db: &dyn HirDatabase) -> Vec<Trait>
fn complete(self, db: &dyn HirDatabase) -> Complete
fn direct_supertraits(self, db: &dyn HirDatabase) -> Vec<Trait>
fn dyn_compatibility(&self, db: &dyn HirDatabase) -> Option<DynCompatibilityViolation>
fn dyn_compatibility_all_violations(&self, db: &dyn HirDatabase) -> Option<Vec<DynCompatibilityViolation>>
fn function(self, db: &dyn HirDatabase, name: impl PartialEq<Name>) -> Option<Function>
fn is_auto(self, db: &dyn HirDatabase) -> bool
fn is_unsafe(&self, db: &dyn HirDatabase) -> bool
fn items(self, db: &dyn HirDatabase) -> Vec<AssocItem>
fn items_with_supertraits(self, db: &dyn HirDatabase) -> Vec<AssocItem>
fn lang(db: &dyn HirDatabase, krate: Crate, lang_item: LangItem) -> Option<Trait>
fn module(self, db: &dyn HirDatabase) -> Module
fn must_implement_one_of(self, db: &dyn HirDatabase) -> Option<&[Name]>
fn name(self, db: &dyn HirDatabase) -> Name
fn prefer_underscore_import(self, db: &dyn HirDatabase) -> bool
fn type_or_const_param_count(&self, db: &dyn HirDatabase, count_required_only: bool) -> usize
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::TraitId) -> Trait
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## TraitPredicate

`struct` · `ra_ap_hir::TraitPredicate`

```rust
struct TraitPredicate<'db>
```

**Implements**: `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn polarity(&self) -> PredicatePolarity
fn trait_ref(&self) -> TraitRef<'db>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## TraitRef

`struct` · `ra_ap_hir::TraitRef`

```rust
struct TraitRef<'db>
```

**Implements**: `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn get_type_argument(&self, idx: usize) -> Option<Type<'db>>
fn self_ty(&self) -> Type<'_>
fn trait_(&self) -> Trait
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## TupleField

`struct` · `ra_ap_hir::TupleField`

```rust
struct TupleField<'db>
```

**Fields**: `owner`, `tuple`, `index`

**Implements**: `ra_ap_hir::HasName`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> Name
fn ty(&self, db: &'db dyn HirDatabase) -> Type<'db>
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, _db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Type

`struct` · `ra_ap_hir::Type`

```rust
struct Type<'db>
```

**Implements**: `ra_ap_hir::HasCrate`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (83)

```rust
fn add_reference(&self, db: &'db dyn HirDatabase, mutability: Mutability) -> Self
fn applicable_inherent_traits(&self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Trait>
fn as_adt(&self) -> Option<Adt>
fn as_adt_with_args(&self) -> Option<(Adt, Vec<Option<Type<'db>>>)>
fn as_array(&self, db: &'db dyn HirDatabase) -> Option<(Self, usize)>
fn as_associated_type_parent_trait(&self, db: &'db dyn HirDatabase) -> Option<Trait>
fn as_builtin(&self) -> Option<BuiltinType>
fn as_callable(&self, db: &'db dyn HirDatabase) -> Option<Callable<'db>>
fn as_closure(&self) -> Option<Closure<'db>>
fn as_coroutine(&self) -> Option<Coroutine<'db>>
fn as_dyn_trait(&self) -> Option<Trait>
fn as_impl_traits(&self, db: &'db dyn HirDatabase) -> Option<impl Iterator<Item = Trait>>
fn as_raw_ptr(&self) -> Option<(Type<'db>, Mutability)>
fn as_reference(&self) -> Option<(Type<'db>, Mutability)>
fn as_reference_inner(&self) -> Option<Type<'db>>
fn as_slice(&self) -> Option<Type<'db>>
fn as_type_param(&self, _db: &'db dyn HirDatabase) -> Option<TypeParam>
fn autoderef(&self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Type<'db>> + use<{'lifetime': "'_"}, {'lifetime': "'db"}>
fn contains_reference(&self, db: &'db dyn HirDatabase) -> bool
fn contains_unknown(&self) -> bool
fn could_coerce_to(&self, db: &'db dyn HirDatabase, to: &Type<'db>) -> bool
fn could_unify_with(&self, db: &'db dyn HirDatabase, other: &Type<'db>) -> bool
fn could_unify_with_deeply(&self, db: &'db dyn HirDatabase, other: &Type<'db>) -> bool
fn drop_glue(&self, db: &'db dyn HirDatabase) -> DropGlue
fn env_traits(&self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Trait>
fn fields(&self, db: &'db dyn HirDatabase) -> Vec<(Field, Self)>
fn fingerprint_for_trait_impl(&self, db: &'db dyn HirDatabase) -> Option<SimplifiedType<'db>>
fn future_output(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>
fn generic_parameters<'a>(&'a self, db: &'a dyn HirDatabase, display_target: DisplayTarget) -> impl Iterator<Item = SmolStr> + 'a
fn generic_params(&self, db: &'db dyn HirDatabase) -> FxHashSet<GenericParam>
fn has_any_impl(&self, db: &'db dyn HirDatabase, trait_: Trait, args: &[Type<'db>]) -> bool
fn impls_fnonce(&self, db: &'db dyn HirDatabase) -> bool
fn impls_iterator(self, db: &'db dyn HirDatabase) -> bool
fn impls_trait(&self, db: &'db dyn HirDatabase, trait_: Trait, args: &[Type<'db>]) -> bool
fn instantiate(&self, args: impl IntoIterator<Item: Borrow<Type<'db>>>) -> Type<'db>
fn instantiate_with_errors(&self) -> Self
fn into_future_output(&self, db: &'db dyn HirDatabase) -> Option<Type<'db>>
fn into_iterator_iter(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>
fn is_array(&self) -> bool
fn is_bool(&self) -> bool
fn is_char(&self) -> bool
fn is_closure(&self) -> bool
fn is_copy(&self, db: &'db dyn HirDatabase) -> bool
fn is_float(&self) -> bool
fn is_fn(&self) -> bool
fn is_int_or_uint(&self) -> bool
fn is_mutable_raw_ptr(&self) -> bool
fn is_mutable_reference(&self) -> bool
fn is_never(&self) -> bool
fn is_packed(&self, _db: &'db dyn HirDatabase) -> bool
fn is_raw_ptr(&self) -> bool
fn is_reference(&self) -> bool
fn is_scalar(&self) -> bool
fn is_slice(&self) -> bool
fn is_str(&self) -> bool
fn is_tuple(&self) -> bool
fn is_unit(&self) -> bool
fn is_unknown(&self) -> bool
fn is_usize(&self) -> bool
fn iterate_assoc_items<T>(&self, db: &'db dyn HirDatabase, callback: impl FnMut(AssocItem) -> Option<T>) -> Option<T>
fn iterate_method_candidates<T>(&self, db: &'db dyn HirDatabase, scope: &SemanticsScope<'_>, name: Option<&Name>, callback: impl FnMut(Function) -> Option<T>) -> Option<T>
fn iterate_method_candidates_split_inherent(&self, db: &'db dyn HirDatabase, scope: &SemanticsScope<'_>, traits_in_scope: &FxHashSet<TraitId>, name: Option<&Name>, callback: impl MethodCandidateCallback)
fn iterate_method_candidates_with_traits<T>(&self, db: &'db dyn HirDatabase, scope: &SemanticsScope<'_>, traits_in_scope: &FxHashSet<TraitId>, name: Option<&Name>, callback: impl FnMut(Function) -> Option<T>) -> Option<T>
fn iterate_path_candidates<T>(&self, db: &'db dyn HirDatabase, scope: &SemanticsScope<'_>, traits_in_scope: &FxHashSet<TraitId>, name: Option<&Name>, callback: impl FnMut(AssocItem) -> Option<T>) -> Option<T>
fn iterate_path_candidates_split_inherent(&self, db: &'db dyn HirDatabase, scope: &SemanticsScope<'_>, traits_in_scope: &FxHashSet<TraitId>, name: Option<&Name>, callback: impl PathCandidateCallback)
fn iterator_item(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>
fn layout(&self, db: &'db dyn HirDatabase) -> Result<Layout<'db>, LayoutError>
fn new_slice(db: &'db dyn HirDatabase, ty: Self) -> Self
fn new_tuple(db: &'db dyn HirDatabase, tys: impl IntoIterator<Item: Borrow<Type<'db>>>) -> Self
fn new_unit() -> Self
fn normalize_trait_assoc_type(&self, db: &'db dyn HirDatabase, args: &[Type<'db>], alias: TypeAlias) -> Option<Type<'db>>
fn rebase_into_or_error(&self, db: &'db dyn HirDatabase, rebase_into: &Type<'db>) -> Type<'db>
fn rebase_into_owner_or_error(&self, db: &'db dyn HirDatabase, new_owner: GenericDef) -> Self
fn remove_raw_ptr(&self) -> Option<Type<'db>>
fn strip_reference(&self) -> Self
fn strip_references(&self) -> Self
fn try_rebase_into(&self, db: &'db dyn HirDatabase, rebase_into: &Type<'db>) -> Option<Self>
fn try_rebase_into_owner(&self, db: &'db dyn HirDatabase, new_owner: GenericDef) -> Option<Self>
fn tuple_fields(&self, _db: &'db dyn HirDatabase) -> Vec<Self>
fn type_and_const_arguments<'a>(&'a self, db: &'a dyn HirDatabase, display_target: DisplayTarget) -> impl Iterator<Item = SmolStr> + 'a
fn type_arguments(&self) -> impl Iterator<Item = Type<'db>> + '_
fn unknown() -> Self
fn walk(&self, db: &'db dyn HirDatabase, callback: impl FnMut(Type<'db>))
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

Note: A [`Type`] remembers its origin. Trying to do anything (except comparing)
with types of different origins will cause errors or panics. Instead, use the `instantiate` methods.

---

## TypeAlias

`struct` · `ra_ap_hir::TypeAlias`

```rust
struct TypeAlias
```

**Implements**: `core::convert::From`, `ra_ap_hir::AsAssocItem`, `ra_ap_hir::AsExternAssocItem`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn has_non_default_type_params(self, db: &dyn HirDatabase) -> bool
fn has_type(self, db: &dyn HirDatabase) -> bool
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::TypeAliasId) -> TypeAlias
```

**via `ra_ap_hir::AsAssocItem`**

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

**via `ra_ap_hir::AsExternAssocItem`**

```rust
fn as_extern_assoc_item(self, db: &dyn HirDatabase) -> Option<ExternAssocItem>
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## TypeOrConstParam

`struct` · `ra_ap_hir::TypeOrConstParam`

```rust
struct TypeOrConstParam
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn as_const_param(self, db: &dyn HirDatabase) -> Option<ConstParam>
fn as_type_param(self, db: &dyn HirDatabase) -> Option<TypeParam>
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self, _db: &dyn HirDatabase) -> GenericDef
fn split(self, db: &dyn HirDatabase) -> Either<ConstParam, TypeParam>
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::TypeOrConstParamId) -> TypeOrConstParam
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## TypeParam

`struct` · `ra_ap_hir::TypeParam`

```rust
struct TypeParam
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasName`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir_ty::display::HirDisplay`, `ra_ap_ide::navigation_target::TryToNav`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn default(self, db: &dyn HirDatabase) -> Option<Type<'_>>
fn is_implicit(self, db: &dyn HirDatabase) -> bool
fn is_unstable(self, db: &dyn HirDatabase) -> bool
fn merge(self) -> TypeOrConstParam
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn parent(self, _db: &dyn HirDatabase) -> GenericDef
fn trait_bounds(self, db: &dyn HirDatabase) -> Vec<Trait>
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::TypeParamId) -> TypeParam
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## Union

`struct` · `ra_ap_hir::Union`

```rust
struct Union
```

**Implements**: `core::convert::From`, `ra_ap_hir::HasContainer`, `ra_ap_hir::HasCrate`, `ra_ap_hir::HasName`, `ra_ap_hir::HasVisibility`, `ra_ap_hir::attrs::HasAttrs`, `ra_ap_hir::has_source::HasSource`, `ra_ap_hir_ty::display::HirDisplay`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn constructor_ty(self, db: &dyn HirDatabase) -> Type<'_>
fn fields(self, db: &dyn HirDatabase) -> Vec<Field>
fn is_unstable(self, db: &dyn HirDatabase) -> bool
fn kind(self, db: &dyn HirDatabase) -> StructKind
fn module(self, db: &dyn HirDatabase) -> Module
fn name(self, db: &dyn HirDatabase) -> Name
fn ty(self, db: &dyn HirDatabase) -> Type<'_>
```

**via `core::convert::From`**

```rust
fn from(id: hir_def::UnionId) -> Union
```

**via `ra_ap_hir::HasContainer`**

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

**via `ra_ap_hir::HasCrate`**

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

**via `ra_ap_hir::HasName`**

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

**via `ra_ap_hir::HasVisibility`**

```rust
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

**via `ra_ap_hir::has_source::HasSource`**

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
```

**via `ra_ap_hir_ty::display::HirDisplay`**

```rust
fn hir_fmt(&self, f: &mut HirFormatter<'_, 'db>) -> Result
```

---

## AsAssocItem

`trait` · `ra_ap_hir::AsAssocItem`

```rust
trait AsAssocItem
```

**Implementors** (6)

- `ra_ap_hir::Const`
- `ra_ap_hir::DefWithBody`
- `ra_ap_hir::Function`
- `ra_ap_hir::GenericDef`
- `ra_ap_hir::ModuleDef`
- `ra_ap_hir::TypeAlias`

**Methods** (1)

```rust
fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>
```

---

## AsExternAssocItem

`trait` · `ra_ap_hir::AsExternAssocItem`

```rust
trait AsExternAssocItem
```

**Implementors** (3)

- `ra_ap_hir::Function`
- `ra_ap_hir::Static`
- `ra_ap_hir::TypeAlias`

**Methods** (1)

```rust
fn as_extern_assoc_item(self, db: &dyn HirDatabase) -> Option<ExternAssocItem>
```

---

## HasContainer

`trait` · `ra_ap_hir::HasContainer`

```rust
trait HasContainer
```

**Implementors** (11)

- `ra_ap_hir::Const`
- `ra_ap_hir::Enum`
- `ra_ap_hir::ExternBlock`
- `ra_ap_hir::ExternCrateDecl`
- `ra_ap_hir::Function`
- `ra_ap_hir::Module`
- `ra_ap_hir::Static`
- `ra_ap_hir::Struct`
- `ra_ap_hir::Trait`
- `ra_ap_hir::TypeAlias`
- `ra_ap_hir::Union`

**Methods** (1)

```rust
fn container(&self, db: &dyn HirDatabase) -> ItemContainer
```

---

## HasCrate

`trait` · `ra_ap_hir::HasCrate`

```rust
trait HasCrate
```

**Implementors** (18)

- `ra_ap_hir::Adt`
- `ra_ap_hir::AnonConst`
- `ra_ap_hir::AssocItem`
- `ra_ap_hir::Const`
- `ra_ap_hir::Enum`
- `ra_ap_hir::EnumVariant`
- `ra_ap_hir::Field`
- `ra_ap_hir::Function`
- `ra_ap_hir::Impl`
- `ra_ap_hir::Macro`
- `ra_ap_hir::Module`
- `ra_ap_hir::ModuleDef`
- `ra_ap_hir::Static`
- `ra_ap_hir::Struct`
- `ra_ap_hir::Trait`
- `ra_ap_hir::Type`
- `ra_ap_hir::TypeAlias`
- `ra_ap_hir::Union`

**Methods** (1)

```rust
fn krate(&self, db: &dyn HirDatabase) -> Crate
```

Trait for obtaining the defining crate of an item.

---

## HasName

`trait` · `ra_ap_hir::HasName`

```rust
trait HasName
```

**Implementors** (34)

- `ra_ap_hir::Adt`
- `ra_ap_hir::AssocItem`
- `ra_ap_hir::BuiltinAttr`
- `ra_ap_hir::BuiltinType`
- `ra_ap_hir::Const`
- `ra_ap_hir::ConstParam`
- `ra_ap_hir::DefWithBody`
- `ra_ap_hir::DeriveHelper`
- `ra_ap_hir::Enum`
- `ra_ap_hir::EnumVariant`
- `ra_ap_hir::ExternAssocItem`
- `ra_ap_hir::ExternCrateDecl`
- `ra_ap_hir::Field`
- `ra_ap_hir::Function`
- `ra_ap_hir::GenericParam`
- `ra_ap_hir::InlineAsmOperand`
- `ra_ap_hir::Label`
- `ra_ap_hir::LifetimeParam`
- `ra_ap_hir::Local`
- `ra_ap_hir::Macro`
- `ra_ap_hir::Module`
- `ra_ap_hir::ModuleDef`
- `ra_ap_hir::Param`
- `ra_ap_hir::Static`
- `ra_ap_hir::StaticLifetime`
- `ra_ap_hir::Struct`
- `ra_ap_hir::ToolModule`
- `ra_ap_hir::Trait`
- `ra_ap_hir::TupleField`
- `ra_ap_hir::TypeAlias`
- `ra_ap_hir::TypeOrConstParam`
- `ra_ap_hir::TypeParam`
- `ra_ap_hir::Union`
- `ra_ap_hir::Variant`

**Methods** (1)

```rust
fn name(&self, db: &dyn HirDatabase) -> Option<Name>
```

---

## HasVisibility

`trait` · `ra_ap_hir::HasVisibility`

```rust
trait HasVisibility
```

**Implementors** (16)

- `ra_ap_hir::Adt`
- `ra_ap_hir::AssocItem`
- `ra_ap_hir::Const`
- `ra_ap_hir::Enum`
- `ra_ap_hir::EnumVariant`
- `ra_ap_hir::ExternCrateDecl`
- `ra_ap_hir::Field`
- `ra_ap_hir::Function`
- `ra_ap_hir::Macro`
- `ra_ap_hir::Module`
- `ra_ap_hir::ModuleDef`
- `ra_ap_hir::Static`
- `ra_ap_hir::Struct`
- `ra_ap_hir::Trait`
- `ra_ap_hir::TypeAlias`
- `ra_ap_hir::Union`

**Methods** (2)

```rust
fn is_visible_from(&self, db: &dyn HirDatabase, module: Module) -> bool
fn visibility(&self, db: &dyn HirDatabase) -> Visibility
```

---

## MacroCallIdExt

`trait` · `ra_ap_hir::MacroCallIdExt`

```rust
trait MacroCallIdExt
```

**Implementors** (1)

- `ra_ap_span::MacroCallId`

**Methods** (1)

```rust
fn loc(self, db: &dyn HirDatabase) -> &hir_expand::MacroCallLoc
```

---

## MethodCandidateCallback

`trait` · `ra_ap_hir::MethodCandidateCallback`

```rust
trait MethodCandidateCallback
```

**Methods** (2)

```rust
fn on_inherent_method(&mut self, f: Function) -> ControlFlow<()>
fn on_trait_method(&mut self, f: Function) -> ControlFlow<()>
```

---

## PathCandidateCallback

`trait` · `ra_ap_hir::PathCandidateCallback`

```rust
trait PathCandidateCallback
```

**Methods** (2)

```rust
fn on_inherent_item(&mut self, item: AssocItem) -> ControlFlow<()>
fn on_trait_item(&mut self, item: AssocItem) -> ControlFlow<()>
```

---
