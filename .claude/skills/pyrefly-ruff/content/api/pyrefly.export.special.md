# `pyrefly::export::special`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.export.special.json`](../model/pyrefly.export.special.json)

## SpecialExport

`enum` · `pyrefly::export::special::SpecialExport`

```rust
enum SpecialExport
```

**Variants**: `ClassMethod`, `AbstractClassMethod`, `TypeAlias`, `TypeAliasType`, `TypeVar`, `IntVar`, `Flag`, `Index`, `ParamSpec`, `TypeVarTuple`, `Annotated`, `Literal`, `Enum`, `StrEnum`, `IntEnum`, `TypedDict`, `CollectionsNamedTuple`, `TypingNamedTuple`, `AssertType`, `RevealType`, `NewType`, `Union`, `Optional`, `Cast`, `Super`, `Exit`, `Quit`, `OsExit`, `Len`, `Bool`, `BuiltinsType`, `TypingType`, `NoTypeCheck`, `NotImplemented`, `NotImplementedError`, `Overload`, `Override`, `AbstractMethod`, `Generic`, `Protocol`, `PydanticConfigDict`, `PydanticToCamel`, `PydanticToPascal`, `PydanticToSnake`, `HasAttr`, `GetAttr`, `Callable`, `BuiltinsDict`, `TypingDict`, `BuiltinsList`, `TypingList`, `BuiltinsTuple`, `TypingTuple`, `BuiltinsInt`, `BuiltinsStr`, `BuiltinsBytes`, `BuiltinsBytearray`, `BuiltinsSet`, `BuiltinsFrozenset`, `BuiltinsFloat`, `Deprecated`, `Final`, `TypingMapping`, `TypeForm`, `UsesShapeDsl`, `ShapeDslFunction`, `TypeShapeDslFunction`, `MapIntTuples`, `ShapedArray`, `ProxyMethod`, `Sentinel`, `BuiltinsSentinel`, `AttrsLegacyAttrib`, `AttrsNextGenField`, `AttrsNothing`

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn defined_in(self, m: ModuleName) -> bool
fn is_single_positional_slot_builtin(self) -> bool
fn is_static_type_subscript(self) -> bool
fn new(name: &Name) -> Option<Self>
```

These are names that are exported from the stdlib, but which take on
a more keyword-like quality. E.g. `x: TypeAlias = ...` meaningfully
changes the sense of the binding.

---
