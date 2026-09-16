# `pyrefly_config::error_kind`

Crate `pyrefly_config` · 2 public items · structured records in [`model/pyrefly_config.error_kind.json`](../model/pyrefly_config.error_kind.json)

## ErrorKind

`enum` · `pyrefly_config::error_kind::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `AbstractMethodCall`, `AssertType`, `BadArgumentCount`, `BadArgumentType`, `BadAssignment`, `BadClassDefinition`, `BadContextManager`, `BadDataclassDescriptor`, `BadDunderAll`, `BadFunctionDefinition`, `BadIndex`, `BadInstantiation`, `BadKeywordArgument`, `BadMatch`, `BadOverride`, `BadOverrideMutableAttribute`, `BadOverrideParamName`, `BadParamNameOverride`, `BadRaise`, `BadReturn`, `BadSingledispatchRegister`, `BadSpecialization`, `BadTypedDict`, `BadTypedDictKey`, `BadUnpacking`, `ColumnSchemaMismatch`, `ColumnTypeMismatch`, `CoverageMissing`, `CoveragePartial`, `Deprecated`, `DirectAbstractBaseInstantiation`, `DivisionByZero`, `DuplicateColumn`, `EmptyBody`, `ExplicitAny`, `ImplicitAbstractClass`, `ImplicitAny`, `ImplicitAnyAttribute`, `ImplicitAnyEmptyContainer`, `ImplicitAnyLambda`, `ImplicitAnyParameter`, `ImplicitAnyTypeArgument`, `ImplicitBool`, `ImplicitImport`, `ImplicitReexport`, `ImplicitlyDefinedAttribute`, `IncompatibleComparison`, `IncompatibleOverloadResidual`, `InconsistentInheritance`, `InconsistentOverload`, `InconsistentOverloadDefault`, `InternalError`, `InvalidAbstractMethod`, `InvalidAnnotation`, `InvalidArgument`, `InvalidCast`, `InvalidDecorator`, `InvalidInheritance`, `InvalidLiteral`, `InvalidOverload`, `InvalidParamSpec`, `InvalidPattern`, `InvalidSelfType`, `InvalidSentinel`, `InvalidSuperCall`, `InvalidSyntax`, `InvalidTypeAlias`, `InvalidTypeCheckingConstant`, `InvalidTypeVar`, `InvalidTypeVarTuple`, `InvalidVariance`, `InvalidYield`, `MisplacedIgnore`, `MissingArgument`, `MissingAttribute`, `MissingAttributePatchTarget`, `MissingImport`, `MissingModuleAttribute`, `MissingOverrideDecorator`, `MissingSource`, `MissingSourceForStubs`, `MissingSuperCall`, `NameMismatch`, `NoAccess`, `NoAnyReturn`, `NoAnyReturnExplicit`, `NoAnyReturnImplicit`, `NoMatchingOverload`, `NonConvergentRecursion`, `NonExhaustiveMatch`, `NonExhaustiveMatchOpenType`, `NotAType`, `NotAsync`, `NotCallable`, `NotIterable`, `NotRequiredKeyAccess`, `OpenUnpacking`, `ParseError`, `PotentialBadKeywordArgument`, `ProtocolImplicitlyDefinedAttribute`, `PytorchEfficiencyLintCudaCall`, `PytorchEfficiencyLintItemCall`, `PytorchEfficiencyLintPrintTensor`, `PytorchEfficiencyLintRedundantToCall`, `PytorchEfficiencyLints`, `ReadOnly`, `Redefinition`, `RedundantCast`, `RedundantCondition`, `Regex`, `RevealType`, `StringAsIterable`, `UnannotatedAttribute`, `UnannotatedParameter`, `UnannotatedProtocolMember`, `UnannotatedReturn`, `UnboundName`, `UnexpectedKeyword`, `UnexpectedPositionalArgument`, `UnimportedDirective`, `UnknownArgumentType`, `UnknownAttributeType`, `UnknownColumn`, `UnknownName`, `UnknownVariableType`, `UnnecessaryComparison`, `UnnecessaryTypeConversion`, `Unreachable`, `UnreachableMatchCase`, `UnresolvableDunderAll`, `UnsafeOverlap`, `Unsupported`, `UnsupportedDelete`, `UnsupportedDynamicBase`, `UnsupportedOperation`, `UntypedClassDecorator`, `UntypedFunctionDecorator`, `UntypedImport`, `UnusedCallResult`, `UnusedCoroutine`, `UnusedIgnore`, `UnusedTypeIgnore`, `UselessOverloadBody`, `VarianceMismatch`

**Implements**: `clap_builder::derive::ValueEnum`, `core::fmt::Display`, `core::str::traits::FromStr`, `dupe::Dupe`, `enum_iterator::Sequence`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
fn default_severity(self) -> Severity
fn deprecated_alias(self) -> Option<ErrorKind>
fn docs_url(self) -> String
fn is_coverage(self) -> bool
fn is_directive(self) -> bool
fn is_soft(self) -> bool
fn is_suppressable(self) -> bool
fn is_unused_ignore(self) -> bool
fn parent_kind(self) -> Option<ErrorKind>
fn suppression_names(self) -> impl Iterator<Item = &'static str>
fn to_name(self) -> &'static str
```

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `enum_iterator::Sequence`**

```rust
fn first() -> ::core::option::Option<Self>
fn last() -> ::core::option::Option<Self>
fn next(&self) -> ::core::option::Option<Self>
fn previous(&self) -> ::core::option::Option<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

ErrorKind categorizes an error by the part of the spec the error is related to.
They are used in suppressions to identify which error should be suppressed.

---

## Severity

`enum` · `pyrefly_config::error_kind::Severity`

```rust
enum Severity
```

**Variants**: `Ignore`, `Info`, `Warn`, `Error`

**Implements**: `clap_builder::derive::ValueEnum`, `core::convert::From`, `dupe::Dupe`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn is_enabled(self) -> bool
fn label(self) -> &'static str
fn painted(self) -> Painted<&'static str>
```

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `core::convert::From`**

```rust
fn from(value: DiagnosticLevel) -> Self
fn from(value: DiagnosticLevelOrBool) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
