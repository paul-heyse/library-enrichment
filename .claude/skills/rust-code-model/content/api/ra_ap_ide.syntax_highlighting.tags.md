# `ra_ap_ide::syntax_highlighting::tags`

Crate `ra_ap_ide` · 6 public items · structured records in [`model/ra_ap_ide.syntax_highlighting.tags.json`](../model/ra_ap_ide.syntax_highlighting.tags.json)

## HlMod

`enum` · `ra_ap_ide::syntax_highlighting::tags::HlMod`

Also reachable as `ra_ap_ide::HlMod`

```rust
enum HlMod
```

**Variants**: `Associated`, `Async`, `Attribute`, `Callable`, `Const`, `Consuming`, `ControlFlow`, `CrateRoot`, `DefaultLibrary`, `Definition`, `Deprecated`, `Documentation`, `Injected`, `IntraDocLink`, `Library`, `Macro`, `ProcMacro`, `Mutable`, `Public`, `Reference`, `Static`, `Trait`, `Unsafe`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## HlOperator

`enum` · `ra_ap_ide::syntax_highlighting::tags::HlOperator`

Also reachable as `ra_ap_ide::HlOperator`

```rust
enum HlOperator
```

**Variants**: `Bitwise`, `Arithmetic`, `Logical`, `Negation`, `Comparison`, `Other`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## HlPunct

`enum` · `ra_ap_ide::syntax_highlighting::tags::HlPunct`

Also reachable as `ra_ap_ide::HlPunct`

```rust
enum HlPunct
```

**Variants**: `Bracket`, `Brace`, `Parenthesis`, `Angle`, `Comma`, `Dot`, `Colon`, `Semi`, `MacroBang`, `Other`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## HlTag

`enum` · `ra_ap_ide::syntax_highlighting::tags::HlTag`

Also reachable as `ra_ap_ide::HlTag`

```rust
enum HlTag
```

**Variants**: `Symbol`, `AttributeBracket`, `BoolLiteral`, `BuiltinType`, `ByteLiteral`, `CharLiteral`, `Comment`, `EscapeSequence`, `FormatSpecifier`, `InvalidEscapeSequence`, `Keyword`, `NumericLiteral`, `Operator`, `Punctuation`, `StringLiteral`, `UnresolvedReference`, `None`

**Implements**: `core::fmt::Display`, `core::ops::bit::BitOr`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: HlMod) -> Highlight
```

---

## Highlight

`struct` · `ra_ap_ide::syntax_highlighting::tags::Highlight`

Also reachable as `ra_ap_ide::Highlight`

```rust
struct Highlight
```

**Fields**: `tag`, `mods`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn is_empty(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(punct: HlPunct) -> Highlight
fn from(sym: SymbolKind) -> Highlight
fn from(tag: HlTag) -> Highlight
fn from(op: HlOperator) -> Highlight
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: HlMod) -> Highlight
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, rhs: HlMod)
```

---

## HlMods

`struct` · `ra_ap_ide::syntax_highlighting::tags::HlMods`

Also reachable as `ra_ap_ide::HlMods`

```rust
struct HlMods
```

**Implements**: `core::ops::bit::BitOrAssign`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn contains(self, m: HlMod) -> bool
fn is_empty(&self) -> bool
fn iter(self) -> impl Iterator<Item = HlMod>
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, rhs: HlMod)
```

---
