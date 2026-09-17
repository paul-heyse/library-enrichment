# `rustdoc_types`

Crate `rustdoc-types` · 58 public items · structured records in [`model/rustdoc_types.json`](../model/rustdoc_types.json)

## FORMAT_VERSION

`constant` · `rustdoc_types::FORMAT_VERSION`

```rust
const FORMAT_VERSION: u32 = 61
```

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as [`Crate::format_version`].
Consuming code should assert that this value matches the format version(s) that it supports.

---

## Abi

`enum` · `rustdoc_types::Abi`

```rust
enum Abi
```

**Variants**: `Rust`, `C`, `Cdecl`, `Stdcall`, `Fastcall`, `Aapcs`, `Win64`, `SysV64`, `System`, `Other`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

---

## AssocItemConstraintKind

`enum` · `rustdoc_types::AssocItemConstraintKind`

```rust
enum AssocItemConstraintKind
```

**Variants**: `Equality`, `Constraint`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The way in which an associate type/constant is bound.

---

## Attribute

`enum` · `rustdoc_types::Attribute`

```rust
enum Attribute
```

**Variants**: `NonExhaustive`, `MustUse`, `MacroExport`, `ExportName`, `LinkSection`, `AutomaticallyDerived`, `Repr`, `NoMangle`, `TargetFeature`, `Other`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An attribute, e.g. `#[repr(C)]`

This doesn't include:
- `#[doc = "Doc Comment"]` or `/// Doc comment`. These are in [`Item::docs`] instead.
- `#[deprecated]`. These are in [`Item::deprecation`] instead.
- `#[stable]` and `#[unstable]`. These are in [`Item::stability`] instead.
- `#[rustc_const_stable]` and `#[rustc_const_unstable]`. These are in
  [`Item::const_stability`] instead.
- `#[rustc_default_body_unstable]`. These are in the `default_unstable` field on the appropriate
  item kinds: [`Function::default_unstable`], [`ItemEnum::AssocConst::default_unstable`],
  and [`ItemEnum::AssocType::default_unstable`].

---

## GenericArg

`enum` · `rustdoc_types::GenericArg`

```rust
enum GenericArg
```

**Variants**: `Lifetime`, `Type`, `Const`, `Infer`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

---

## GenericArgs

`enum` · `rustdoc_types::GenericArgs`

```rust
enum GenericArgs
```

**Variants**: `AngleBracketed`, `Parenthesized`, `ReturnTypeNotation`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option<u32>
                   ^^^^^
```

---

## GenericBound

`enum` · `rustdoc_types::GenericBound`

```rust
enum GenericBound
```

**Variants**: `TraitBound`, `Outlives`, `Use`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Either a trait bound or a lifetime bound.

---

## GenericParamDefKind

`enum` · `rustdoc_types::GenericParamDefKind`

```rust
enum GenericParamDefKind
```

**Variants**: `Lifetime`, `Type`, `Const`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The kind of a [`GenericParamDef`].

---

## ItemEnum

`enum` · `rustdoc_types::ItemEnum`

```rust
enum ItemEnum
```

**Variants**: `Module`, `ExternCrate`, `Use`, `Union`, `Struct`, `StructField`, `Enum`, `Variant`, `Function`, `Trait`, `TraitAlias`, `Impl`, `TypeAlias`, `Constant`, `Static`, `ExternType`, `Macro`, `ProcMacro`, `Primitive`, `AssocConst`, `AssocType`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn item_kind(&self) -> ItemKind
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Specific fields of an item.

Part of [`Item`].

---

## ItemKind

`enum` · `rustdoc_types::ItemKind`

```rust
enum ItemKind
```

**Variants**: `Module`, `ExternCrate`, `Use`, `Struct`, `StructField`, `Union`, `Enum`, `Variant`, `Function`, `TypeAlias`, `Constant`, `Trait`, `TraitAlias`, `Impl`, `Static`, `ExternType`, `Macro`, `ProcAttribute`, `ProcDerive`, `AssocConst`, `AssocType`, `Primitive`, `Keyword`, `Attribute`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The fundamental kind of an item. Unlike [`ItemEnum`], this does not carry any additional info.

Part of [`ItemSummary`].

---

## MacroKind

`enum` · `rustdoc_types::MacroKind`

```rust
enum MacroKind
```

**Variants**: `Bang`, `Attr`, `Derive`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The way a [`ProcMacro`] is declared to be used.

---

## PreciseCapturingArg

`enum` · `rustdoc_types::PreciseCapturingArg`

```rust
enum PreciseCapturingArg
```

**Variants**: `Lifetime`, `Param`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

---

## ReprKind

`enum` · `rustdoc_types::ReprKind`

```rust
enum ReprKind
```

**Variants**: `Rust`, `C`, `Transparent`, `Simd`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The kind of `#[repr]`.

See [AttributeRepr::kind]`.

---

## StabilityLevel

`enum` · `rustdoc_types::StabilityLevel`

```rust
enum StabilityLevel
```

**Variants**: `Stable`, `Unstable`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## StructKind

`enum` · `rustdoc_types::StructKind`

```rust
enum StructKind
```

**Variants**: `Unit`, `Tuple`, `Plain`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The kind of a [`Struct`] and the data specific to it, i.e. fields.

---

## Term

`enum` · `rustdoc_types::Term`

```rust
enum Term
```

**Variants**: `Type`, `Constant`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`]

---

## TraitBoundModifier

`enum` · `rustdoc_types::TraitBoundModifier`

```rust
enum TraitBoundModifier
```

**Variants**: `None`, `Maybe`, `MaybeConst`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A set of modifiers applied to a trait.

---

## Type

`enum` · `rustdoc_types::Type`

```rust
enum Type
```

**Variants**: `ResolvedPath`, `DynTrait`, `Generic`, `Primitive`, `FunctionPointer`, `Tuple`, `Slice`, `Array`, `Pat`, `ImplTrait`, `Infer`, `RawPointer`, `BorrowedRef`, `QualifiedPath`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A type.

---

## VariantKind

`enum` · `rustdoc_types::VariantKind`

```rust
enum VariantKind
```

**Variants**: `Plain`, `Tuple`, `Struct`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

---

## Visibility

`enum` · `rustdoc_types::Visibility`

```rust
enum Visibility
```

**Variants**: `Public`, `Default`, `Crate`, `Restricted`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Visibility of an [`Item`].

---

## WherePredicate

`enum` · `rustdoc_types::WherePredicate`

```rust
enum WherePredicate
```

**Variants**: `BoundPredicate`, `LifetimePredicate`, `EqPredicate`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

---

## AssocItemConstraint

`struct` · `rustdoc_types::AssocItemConstraint`

```rust
struct AssocItemConstraint
```

**Fields**: `name`, `args`, `binding`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Describes a bound applied to an associated type/constant.

Example:
```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

---

## AttributeRepr

`struct` · `rustdoc_types::AttributeRepr`

```rust
struct AttributeRepr
```

**Fields**: `kind`, `align`, `packed`, `int`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The contents of a `#[repr(...)]` attribute.

Used in [`Attribute::Repr`].

---

## Constant

`struct` · `rustdoc_types::Constant`

```rust
struct Constant
```

**Fields**: `expr`, `value`, `is_literal`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A constant.

---

## Crate

`struct` · `rustdoc_types::Crate`

```rust
struct Crate
```

**Fields**: `root`, `crate_version`, `includes_private`, `index`, `paths`, `external_crates`, `target`, `format_version`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

---

## Deprecation

`struct` · `rustdoc_types::Deprecation`

```rust
struct Deprecation
```

**Fields**: `since`, `note`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about the deprecation of an [`Item`].

---

## Discriminant

`struct` · `rustdoc_types::Discriminant`

```rust
struct Discriminant
```

**Fields**: `expr`, `value`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The value that distinguishes a variant in an [`Enum`] from other variants.

---

## DynTrait

`struct` · `rustdoc_types::DynTrait`

```rust
struct DynTrait
```

**Fields**: `traits`, `lifetime`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Dynamic trait object type (`dyn Trait`).

---

## Enum

`struct` · `rustdoc_types::Enum`

```rust
struct Enum
```

**Fields**: `generics`, `has_stripped_variants`, `variants`, `impls`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An `enum`.

---

## ExternalCrate

`struct` · `rustdoc_types::ExternalCrate`

```rust
struct ExternalCrate
```

**Fields**: `name`, `html_root_url`, `path`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

---

## Function

`struct` · `rustdoc_types::Function`

```rust
struct Function
```

**Fields**: `sig`, `generics`, `header`, `has_body`, `default_unstable`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A function declaration (including methods and other associated functions).

---

## FunctionHeader

`struct` · `rustdoc_types::FunctionHeader`

```rust
struct FunctionHeader
```

**Fields**: `is_const`, `is_unsafe`, `is_async`, `abi`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A set of fundamental properties of a function.

---

## FunctionPointer

`struct` · `rustdoc_types::FunctionPointer`

```rust
struct FunctionPointer
```

**Fields**: `sig`, `generic_params`, `header`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A type that is a function pointer.

---

## FunctionSignature

`struct` · `rustdoc_types::FunctionSignature`

```rust
struct FunctionSignature
```

**Fields**: `inputs`, `output`, `is_c_variadic`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The signature of a function.

---

## GenericParamDef

`struct` · `rustdoc_types::GenericParamDef`

```rust
struct GenericParamDef
```

**Fields**: `name`, `kind`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One generic parameter accepted by an item.

---

## Generics

`struct` · `rustdoc_types::Generics`

```rust
struct Generics
```

**Fields**: `params`, `where_predicates`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

---

## Id

`struct` · `rustdoc_types::Id`

```rust
struct Id
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An opaque identifier for an item.

It can be used to lookup in [`Crate::index`] or [`Crate::paths`] to resolve it
to an [`Item`].

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

---

## Impl

`struct` · `rustdoc_types::Impl`

```rust
struct Impl
```

**Fields**: `is_unsafe`, `generics`, `provided_trait_methods`, `trait_`, `for_`, `items`, `is_negative`, `is_synthetic`, `blanket_impl`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An `impl` block.

---

## Item

`struct` · `rustdoc_types::Item`

```rust
struct Item
```

**Fields**: `id`, `crate_id`, `name`, `span`, `visibility`, `docs`, `links`, `attrs`, `deprecation`, `stability`, `const_stability`, `inner`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

---

## ItemSummary

`struct` · `rustdoc_types::ItemSummary`

```rust
struct ItemSummary
```

**Fields**: `crate_id`, `path`, `kind`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

---

## Module

`struct` · `rustdoc_types::Module`

```rust
struct Module
```

**Fields**: `is_crate`, `items`, `is_stripped`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A module declaration, e.g. `mod foo;` or `mod foo {}`.

---

## Path

`struct` · `rustdoc_types::Path`

```rust
struct Path
```

**Fields**: `path`, `id`, `args`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

---

## PolyTrait

`struct` · `rustdoc_types::PolyTrait`

```rust
struct PolyTrait
```

**Fields**: `trait_`, `generic_params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A trait and potential HRTBs

---

## Primitive

`struct` · `rustdoc_types::Primitive`

```rust
struct Primitive
```

**Fields**: `name`, `impls`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A primitive type declaration. Declarations of this kind can only come from the core library.

---

## ProcMacro

`struct` · `rustdoc_types::ProcMacro`

```rust
struct ProcMacro
```

**Fields**: `kind`, `helpers`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A procedural macro.

---

## ProvidedDefaultUnstable

`struct` · `rustdoc_types::ProvidedDefaultUnstable`

```rust
struct ProvidedDefaultUnstable
```

**Fields**: `feature`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about an unstable default provided by a trait item.

Example unstable defaults include:
- a stable trait function or method whose body is not stable
- a stable trait associated type or const whose default value is not stable

---

## Span

`struct` · `rustdoc_types::Span`

```rust
struct Span
```

**Fields**: `filename`, `begin`, `end`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A range of source code.

---

## Stability

`struct` · `rustdoc_types::Stability`

```rust
struct Stability
```

**Fields**: `feature`, `level`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Stability information for an item.

In [`Item::stability`], this refers to regular item stability: whether the item is
stable or unstable as represented by the `#[stable]` or `#[unstable]` attributes.
In [`Item::const_stability`], this refers to using the item in const contexts,
as represented by `#[rustc_const_stable]` or `#[rustc_const_unstable]`.

---

## Static

`struct` · `rustdoc_types::Static`

```rust
struct Static
```

**Fields**: `type_`, `is_mutable`, `expr`, `is_unsafe`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A `static` declaration.

---

## Struct

`struct` · `rustdoc_types::Struct`

```rust
struct Struct
```

**Fields**: `kind`, `generics`, `impls`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A `struct`.

---

## Target

`struct` · `rustdoc_types::Target`

```rust
struct Target
```

**Fields**: `triple`, `target_features`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about a target

---

## TargetFeature

`struct` · `rustdoc_types::TargetFeature`

```rust
struct TargetFeature
```

**Fields**: `name`, `implies_features`, `unstable_feature_gate`, `globally_enabled`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about a target feature.

Rust target features are used to influence code generation, especially around selecting
instructions which are not universally supported by the target architecture.

Target features are commonly enabled by the [`#[target_feature]` attribute][1] to influence code
generation for a particular function, and less commonly enabled by compiler options like
`-Ctarget-feature` or `-Ctarget-cpu`. Targets themselves automatically enable certain target
features by default, for example because the target's ABI specification requires saving specific
registers which only exist in an architectural extension.

Target features can imply other target features: for example, x86-64 `avx2` implies `avx`, and
aarch64 `sve2` implies `sve`, since both of these architectural extensions depend on their
predecessors.

Target features can be probed at compile time by [`#[cfg(target_feature)]`][2] or `cfg!(…)`
conditional compilation to determine whether a target feature is enabled in a particular
context.

[1]: https://doc.rust-lang.org/stable/reference/attributes/codegen.html#the-target_feature-attribute
[2]: https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature

---

## Trait

`struct` · `rustdoc_types::Trait`

```rust
struct Trait
```

**Fields**: `is_auto`, `is_unsafe`, `is_dyn_compatible`, `items`, `generics`, `bounds`, `implementations`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A `trait` declaration.

---

## TraitAlias

`struct` · `rustdoc_types::TraitAlias`

```rust
struct TraitAlias
```

**Fields**: `generics`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

---

## TypeAlias

`struct` · `rustdoc_types::TypeAlias`

```rust
struct TypeAlias
```

**Fields**: `type_`, `generics`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

---

## Union

`struct` · `rustdoc_types::Union`

```rust
struct Union
```

**Fields**: `generics`, `has_stripped_fields`, `fields`, `impls`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A `union`.

---

## Use

`struct` · `rustdoc_types::Use`

```rust
struct Use
```

**Fields**: `source`, `name`, `id`, `is_glob`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A `use` statement.

---

## Variant

`struct` · `rustdoc_types::Variant`

```rust
struct Variant
```

**Fields**: `kind`, `discriminant`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A variant of an enum.

---
