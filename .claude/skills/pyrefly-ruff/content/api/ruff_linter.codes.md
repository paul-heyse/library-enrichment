# `ruff_linter::codes`

Crate `ruff_linter` · 126 public items · structured records in [`model/ruff_linter.codes.json`](../model/ruff_linter.codes.json)

## Airflow

`enum` · `ruff_linter::codes::Airflow`

```rust
enum Airflow
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_2`, `_20`, `_201`, `_202`, `_3`, `_30`, `_301`, `_302`, `_303`, `_304`, `_31`, `_311`, `_312`, `_32`, `_321`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> AirflowIter
```

---

## Category

`enum` · `ruff_linter::codes::Category`

```rust
enum Category
```

**Variants**: `Correctness`, `Suspicious`, `Complexity`, `Performance`, `Style`, `Security`, `Formatting`, `Pedantic`, `Restriction`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::ser::Serialize`, `strum::EnumMessage`, `strum::IntoEnumIterator`, `strum::VariantArray`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn default_categories() -> [Category; 5]
const fn into_str(&self) -> &'static str
```

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<Category, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<Category, <Self as ::core::str::FromStr>::Err>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `strum::EnumMessage`**

```rust
fn get_detailed_message(&self) -> ::core::option::Option<&'static str>
fn get_documentation(&self) -> ::core::option::Option<&'static str>
fn get_message(&self) -> ::core::option::Option<&'static str>
fn get_serializations(&self) -> &'static [&'static str]
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> CategoryIter
```

The category assigned to a lint rule.

These categories are similar to those found in [Clippy] and form much broader groupings than the
linter-based groups. Categories are intended to be our primary classification mechanism for
rules going forward, with the linter groups eventually being deprecated and removed, albeit in
the relatively distant future. The categorization of a rule determines two important properties:
- its default status, `style` and above are currently enabled by default
- its default severity, in a future where we have multiple diagnostic severities

Assuming we continue to follow Clippy, `correctness` lints will have a severity of `error` by
default, while the other on-by-default categories will have a severity of `warn` by default.

Secondary groups like the legacy linter groups are orthogonal selection mechanisms that have no
impact on severity or default status, and may, and usually do, include rules from multiple
categories. For example, many `F` rules are `correctness` lints, but `F` includes `suspicious`
and even `pedantic` rules too. At some point in the future, we may support additional secondary
groups that are not legacy linter groups as well.

The precedence between categories, linter groups, linter prefixes, and rules is determined by
the [`crate::rule_selector::Specificity`] returned by
[`crate::rule_selector::RuleSelector::specificity`], and currently follows this ordering:

```text
ALL < category < linter group < linter prefix < rule
```

The ordering of variants isn't currently used anywhere, but they should be kept in descending
order of severity, with error categories first, followed by warning, and then by off-by-default
categories.

See our [rule categorization guidelines] for more information on assigning categories.

[Clippy]: https://doc.rust-lang.org/clippy/lints.html
[rule categorization guidelines]: https://docs.astral.sh/ruff/rule-proposals/#rule-categorization-guidelines

---

## Eradicate

`enum` · `ruff_linter::codes::Eradicate`

```rust
enum Eradicate
```

**Variants**: `_0`, `_00`, `_001`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> EradicateIter
```

---

## FastApi

`enum` · `ruff_linter::codes::FastApi`

```rust
enum FastApi
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> FastApiIter
```

---

## Flake82020

`enum` · `ruff_linter::codes::Flake82020`

```rust
enum Flake82020
```

**Variants**: `_1`, `_10`, `_101`, `_102`, `_103`, `_2`, `_20`, `_201`, `_202`, `_203`, `_204`, `_3`, `_30`, `_301`, `_302`, `_303`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake82020Iter
```

---

## Flake8Annotations

`enum` · `ruff_linter::codes::Flake8Annotations`

```rust
enum Flake8Annotations
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_1`, `_10`, `_101`, `_102`, `_2`, `_20`, `_201`, `_202`, `_204`, `_205`, `_206`, `_4`, `_40`, `_401`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8AnnotationsIter
```

---

## Flake8Async

`enum` · `ruff_linter::codes::Flake8Async`

```rust
enum Flake8Async
```

**Variants**: `_1`, `_10`, `_100`, `_105`, `_109`, `_11`, `_110`, `_115`, `_116`, `_119`, `_2`, `_21`, `_210`, `_212`, `_22`, `_220`, `_221`, `_222`, `_23`, `_230`, `_24`, `_240`, `_25`, `_250`, `_251`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8AsyncIter
```

---

## Flake8Bandit

`enum` · `ruff_linter::codes::Flake8Bandit`

```rust
enum Flake8Bandit
```

**Variants**: `_1`, `_10`, `_101`, `_102`, `_103`, `_104`, `_105`, `_106`, `_107`, `_108`, `_11`, `_110`, `_112`, `_113`, `_2`, `_20`, `_201`, `_202`, `_3`, `_30`, `_301`, `_302`, `_303`, `_304`, `_305`, `_306`, `_307`, `_308`, `_31`, `_310`, `_311`, `_312`, `_313`, `_314`, `_315`, `_316`, `_317`, `_318`, `_319`, `_32`, `_320`, `_321`, `_323`, `_324`, `_4`, `_40`, `_401`, `_402`, `_403`, `_404`, `_405`, `_406`, `_407`, `_408`, `_409`, `_41`, `_410`, `_411`, `_412`, `_413`, `_415`, `_5`, `_50`, `_501`, `_502`, `_503`, `_504`, `_505`, `_506`, `_507`, `_508`, `_509`, `_6`, `_60`, `_601`, `_602`, `_603`, `_604`, `_605`, `_606`, `_607`, `_608`, `_609`, `_61`, `_610`, `_611`, `_612`, `_7`, `_70`, `_701`, `_702`, `_704`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8BanditIter
```

---

## Flake8BlindExcept

`enum` · `ruff_linter::codes::Flake8BlindExcept`

```rust
enum Flake8BlindExcept
```

**Variants**: `_0`, `_00`, `_001`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8BlindExceptIter
```

---

## Flake8BooleanTrap

`enum` · `ruff_linter::codes::Flake8BooleanTrap`

```rust
enum Flake8BooleanTrap
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8BooleanTrapIter
```

---

## Flake8Bugbear

`enum` · `ruff_linter::codes::Flake8Bugbear`

```rust
enum Flake8Bugbear
```

**Variants**: `_0`, `_00`, `_002`, `_003`, `_004`, `_005`, `_006`, `_007`, `_008`, `_009`, `_01`, `_010`, `_011`, `_012`, `_013`, `_014`, `_015`, `_016`, `_017`, `_018`, `_019`, `_02`, `_020`, `_021`, `_022`, `_023`, `_024`, `_025`, `_026`, `_027`, `_028`, `_029`, `_03`, `_030`, `_031`, `_032`, `_033`, `_034`, `_035`, `_039`, `_04`, `_043`, `_9`, `_90`, `_901`, `_903`, `_904`, `_905`, `_909`, `_91`, `_911`, `_912`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8BugbearIter
```

---

## Flake8Builtins

`enum` · `ruff_linter::codes::Flake8Builtins`

```rust
enum Flake8Builtins
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`, `_006`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8BuiltinsIter
```

---

## Flake8Commas

`enum` · `ruff_linter::codes::Flake8Commas`

```rust
enum Flake8Commas
```

**Variants**: `_8`, `_81`, `_812`, `_818`, `_819`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8CommasIter
```

---

## Flake8Comprehensions

`enum` · `ruff_linter::codes::Flake8Comprehensions`

```rust
enum Flake8Comprehensions
```

**Variants**: `_0`, `_00`, `_01`, `_02`, `_03`, `_04`, `_05`, `_06`, `_08`, `_09`, `_1`, `_10`, `_11`, `_13`, `_14`, `_15`, `_16`, `_17`, `_18`, `_19`, `_2`, `_20`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8ComprehensionsIter
```

---

## Flake8Copyright

`enum` · `ruff_linter::codes::Flake8Copyright`

```rust
enum Flake8Copyright
```

**Variants**: `_0`, `_00`, `_001`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8CopyrightIter
```

---

## Flake8Datetimez

`enum` · `ruff_linter::codes::Flake8Datetimez`

```rust
enum Flake8Datetimez
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`, `_006`, `_007`, `_01`, `_011`, `_012`, `_9`, `_90`, `_901`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8DatetimezIter
```

---

## Flake8Debugger

`enum` · `ruff_linter::codes::Flake8Debugger`

```rust
enum Flake8Debugger
```

**Variants**: `_0`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8DebuggerIter
```

---

## Flake8Django

`enum` · `ruff_linter::codes::Flake8Django`

```rust
enum Flake8Django
```

**Variants**: `_0`, `_00`, `_001`, `_003`, `_006`, `_007`, `_008`, `_01`, `_012`, `_013`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8DjangoIter
```

---

## Flake8ErrMsg

`enum` · `ruff_linter::codes::Flake8ErrMsg`

```rust
enum Flake8ErrMsg
```

**Variants**: `_1`, `_10`, `_101`, `_102`, `_103`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8ErrMsgIter
```

---

## Flake8Executable

`enum` · `ruff_linter::codes::Flake8Executable`

```rust
enum Flake8Executable
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8ExecutableIter
```

---

## Flake8Fixme

`enum` · `ruff_linter::codes::Flake8Fixme`

```rust
enum Flake8Fixme
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8FixmeIter
```

---

## Flake8FutureAnnotations

`enum` · `ruff_linter::codes::Flake8FutureAnnotations`

```rust
enum Flake8FutureAnnotations
```

**Variants**: `_1`, `_10`, `_100`, `_102`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8FutureAnnotationsIter
```

---

## Flake8GetText

`enum` · `ruff_linter::codes::Flake8GetText`

```rust
enum Flake8GetText
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8GetTextIter
```

---

## Flake8ImplicitStrConcat

`enum` · `ruff_linter::codes::Flake8ImplicitStrConcat`

```rust
enum Flake8ImplicitStrConcat
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8ImplicitStrConcatIter
```

---

## Flake8ImportConventions

`enum` · `ruff_linter::codes::Flake8ImportConventions`

```rust
enum Flake8ImportConventions
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8ImportConventionsIter
```

---

## Flake8Logging

`enum` · `ruff_linter::codes::Flake8Logging`

```rust
enum Flake8Logging
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_004`, `_007`, `_009`, `_01`, `_014`, `_015`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8LoggingIter
```

---

## Flake8LoggingFormat

`enum` · `ruff_linter::codes::Flake8LoggingFormat`

```rust
enum Flake8LoggingFormat
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_01`, `_010`, `_1`, `_10`, `_101`, `_2`, `_20`, `_201`, `_202`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8LoggingFormatIter
```

---

## Flake8NoPep420

`enum` · `ruff_linter::codes::Flake8NoPep420`

```rust
enum Flake8NoPep420
```

**Variants**: `_0`, `_00`, `_001`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8NoPep420Iter
```

---

## Flake8Pie

`enum` · `ruff_linter::codes::Flake8Pie`

```rust
enum Flake8Pie
```

**Variants**: `_7`, `_79`, `_790`, `_794`, `_796`, `_8`, `_80`, `_800`, `_804`, `_807`, `_808`, `_81`, `_810`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8PieIter
```

---

## Flake8Print

`enum` · `ruff_linter::codes::Flake8Print`

```rust
enum Flake8Print
```

**Variants**: `_1`, `_3`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8PrintIter
```

---

## Flake8Pyi

`enum` · `ruff_linter::codes::Flake8Pyi`

```rust
enum Flake8Pyi
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`, `_006`, `_007`, `_008`, `_009`, `_01`, `_010`, `_011`, `_012`, `_013`, `_014`, `_015`, `_016`, `_017`, `_018`, `_019`, `_02`, `_020`, `_021`, `_024`, `_025`, `_026`, `_029`, `_03`, `_030`, `_032`, `_033`, `_034`, `_035`, `_036`, `_04`, `_041`, `_042`, `_043`, `_044`, `_045`, `_046`, `_047`, `_048`, `_049`, `_05`, `_050`, `_051`, `_052`, `_053`, `_054`, `_055`, `_056`, `_057`, `_058`, `_059`, `_06`, `_061`, `_062`, `_063`, `_064`, `_066`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8PyiIter
```

---

## Flake8PytestStyle

`enum` · `ruff_linter::codes::Flake8PytestStyle`

```rust
enum Flake8PytestStyle
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`, `_006`, `_007`, `_008`, `_009`, `_01`, `_010`, `_011`, `_012`, `_013`, `_014`, `_015`, `_016`, `_017`, `_018`, `_019`, `_02`, `_020`, `_021`, `_022`, `_023`, `_024`, `_025`, `_026`, `_027`, `_028`, `_029`, `_03`, `_030`, `_031`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8PytestStyleIter
```

---

## Flake8Quotes

`enum` · `ruff_linter::codes::Flake8Quotes`

```rust
enum Flake8Quotes
```

**Variants**: `_0`, `_00`, `_000`, `_001`, `_002`, `_003`, `_004`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8QuotesIter
```

---

## Flake8Raise

`enum` · `ruff_linter::codes::Flake8Raise`

```rust
enum Flake8Raise
```

**Variants**: `_1`, `_10`, `_102`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8RaiseIter
```

---

## Flake8Return

`enum` · `ruff_linter::codes::Flake8Return`

```rust
enum Flake8Return
```

**Variants**: `_5`, `_50`, `_501`, `_502`, `_503`, `_504`, `_505`, `_506`, `_507`, `_508`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8ReturnIter
```

---

## Flake8Self

`enum` · `ruff_linter::codes::Flake8Self`

```rust
enum Flake8Self
```

**Variants**: `_0`, `_00`, `_001`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8SelfIter
```

---

## Flake8Simplify

`enum` · `ruff_linter::codes::Flake8Simplify`

```rust
enum Flake8Simplify
```

**Variants**: `_1`, `_10`, `_101`, `_102`, `_103`, `_105`, `_107`, `_108`, `_109`, `_11`, `_110`, `_112`, `_113`, `_114`, `_115`, `_116`, `_117`, `_118`, `_2`, `_20`, `_201`, `_202`, `_208`, `_21`, `_210`, `_211`, `_212`, `_22`, `_220`, `_221`, `_222`, `_223`, `_3`, `_30`, `_300`, `_4`, `_40`, `_401`, `_9`, `_90`, `_905`, `_91`, `_910`, `_911`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8SimplifyIter
```

---

## Flake8Slots

`enum` · `ruff_linter::codes::Flake8Slots`

```rust
enum Flake8Slots
```

**Variants**: `_0`, `_00`, `_000`, `_001`, `_002`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8SlotsIter
```

---

## Flake8TidyImports

`enum` · `ruff_linter::codes::Flake8TidyImports`

```rust
enum Flake8TidyImports
```

**Variants**: `_2`, `_25`, `_251`, `_252`, `_253`, `_254`, `_255`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8TidyImportsIter
```

---

## Flake8Todos

`enum` · `ruff_linter::codes::Flake8Todos`

```rust
enum Flake8Todos
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`, `_006`, `_007`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8TodosIter
```

---

## Flake8TypeChecking

`enum` · `ruff_linter::codes::Flake8TypeChecking`

```rust
enum Flake8TypeChecking
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`, `_006`, `_007`, `_008`, `_01`, `_010`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8TypeCheckingIter
```

---

## Flake8UnusedArguments

`enum` · `ruff_linter::codes::Flake8UnusedArguments`

```rust
enum Flake8UnusedArguments
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8UnusedArgumentsIter
```

---

## Flake8UsePathlib

`enum` · `ruff_linter::codes::Flake8UsePathlib`

```rust
enum Flake8UsePathlib
```

**Variants**: `_1`, `_10`, `_100`, `_101`, `_102`, `_103`, `_104`, `_105`, `_106`, `_107`, `_108`, `_109`, `_11`, `_110`, `_111`, `_112`, `_113`, `_114`, `_115`, `_116`, `_117`, `_118`, `_119`, `_12`, `_120`, `_121`, `_122`, `_123`, `_124`, `_2`, `_20`, `_201`, `_202`, `_203`, `_204`, `_205`, `_206`, `_207`, `_208`, `_21`, `_210`, `_211`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> Flake8UsePathlibIter
```

---

## Flynt

`enum` · `ruff_linter::codes::Flynt`

```rust
enum Flynt
```

**Variants**: `_0`, `_00`, `_002`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> FlyntIter
```

---

## FromNameError

`enum` · `ruff_linter::codes::FromNameError`

```rust
enum FromNameError
```

**Variants**: `Unknown`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## Isort

`enum` · `ruff_linter::codes::Isort`

```rust
enum Isort
```

**Variants**: `_0`, `_00`, `_001`, `_002`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> IsortIter
```

---

## McCabe

`enum` · `ruff_linter::codes::McCabe`

```rust
enum McCabe
```

**Variants**: `_1`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> McCabeIter
```

---

## Numpy

`enum` · `ruff_linter::codes::Numpy`

```rust
enum Numpy
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_2`, `_20`, `_201`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> NumpyIter
```

---

## PEP8Naming

`enum` · `ruff_linter::codes::PEP8Naming`

```rust
enum PEP8Naming
```

**Variants**: `_8`, `_80`, `_801`, `_802`, `_803`, `_804`, `_805`, `_806`, `_807`, `_81`, `_811`, `_812`, `_813`, `_814`, `_815`, `_816`, `_817`, `_818`, `_9`, `_99`, `_999`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PEP8NamingIter
```

---

## PandasVet

`enum` · `ruff_linter::codes::PandasVet`

```rust
enum PandasVet
```

**Variants**: `_0`, `_00`, `_002`, `_003`, `_004`, `_007`, `_008`, `_009`, `_01`, `_010`, `_011`, `_012`, `_013`, `_015`, `_1`, `_10`, `_101`, `_9`, `_90`, `_901`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PandasVetIter
```

---

## Perflint

`enum` · `ruff_linter::codes::Perflint`

```rust
enum Perflint
```

**Variants**: `_1`, `_10`, `_101`, `_102`, `_2`, `_20`, `_203`, `_4`, `_40`, `_401`, `_402`, `_403`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PerflintIter
```

---

## Pycodestyle

`enum` · `ruff_linter::codes::Pycodestyle`

```rust
enum Pycodestyle
```

**Variants**: `E`, `E1`, `E10`, `E101`, `E11`, `E111`, `E112`, `E113`, `E114`, `E115`, `E116`, `E117`, `E2`, `E20`, `E201`, `E202`, `E203`, `E204`, `E21`, `E211`, `E22`, `E221`, `E222`, `E223`, `E224`, `E225`, `E226`, `E227`, `E228`, `E23`, `E231`, `E24`, `E241`, `E242`, `E25`, `E251`, `E252`, `E26`, `E261`, `E262`, `E265`, `E266`, `E27`, `E271`, `E272`, `E273`, `E274`, `E275`, `E3`, `E30`, `E301`, `E302`, `E303`, `E304`, `E305`, `E306`, `E4`, `E40`, `E401`, `E402`, `E5`, `E50`, `E501`, `E502`, `E7`, `E70`, `E701`, `E702`, `E703`, `E71`, `E711`, `E712`, `E713`, `E714`, `E72`, `E721`, `E722`, `E73`, `E731`, `E74`, `E741`, `E742`, `E743`, `E9`, `E90`, `E902`, `E99`, `E999`, `W`, `W1`, `W19`, `W191`, `W2`, `W29`, `W291`, `W292`, `W293`, `W3`, `W39`, `W391`, `W5`, `W50`, `W505`, `W6`, `W60`, `W605`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PycodestyleIter
```

---

## Pydoclint

`enum` · `ruff_linter::codes::Pydoclint`

```rust
enum Pydoclint
```

**Variants**: `_1`, `_10`, `_102`, `_2`, `_20`, `_201`, `_202`, `_4`, `_40`, `_402`, `_403`, `_5`, `_50`, `_501`, `_502`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PydoclintIter
```

---

## Pydocstyle

`enum` · `ruff_linter::codes::Pydocstyle`

```rust
enum Pydocstyle
```

**Variants**: `_1`, `_10`, `_100`, `_101`, `_102`, `_103`, `_104`, `_105`, `_106`, `_107`, `_2`, `_20`, `_200`, `_201`, `_202`, `_203`, `_204`, `_205`, `_206`, `_207`, `_208`, `_209`, `_21`, `_210`, `_211`, `_212`, `_213`, `_214`, `_215`, `_3`, `_30`, `_300`, `_301`, `_4`, `_40`, `_400`, `_401`, `_402`, `_403`, `_404`, `_405`, `_406`, `_407`, `_408`, `_409`, `_41`, `_410`, `_411`, `_412`, `_413`, `_414`, `_415`, `_416`, `_417`, `_418`, `_419`, `_42`, `_420`, `_421`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PydocstyleIter
```

---

## Pyflakes

`enum` · `ruff_linter::codes::Pyflakes`

```rust
enum Pyflakes
```

**Variants**: `_4`, `_40`, `_401`, `_402`, `_403`, `_404`, `_405`, `_406`, `_407`, `_5`, `_50`, `_501`, `_502`, `_503`, `_504`, `_505`, `_506`, `_507`, `_508`, `_509`, `_52`, `_521`, `_522`, `_523`, `_524`, `_525`, `_54`, `_541`, `_6`, `_60`, `_601`, `_602`, `_62`, `_621`, `_622`, `_63`, `_631`, `_632`, `_633`, `_634`, `_7`, `_70`, `_701`, `_702`, `_704`, `_706`, `_707`, `_72`, `_722`, `_8`, `_81`, `_811`, `_82`, `_821`, `_822`, `_823`, `_84`, `_841`, `_842`, `_9`, `_90`, `_901`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PyflakesIter
```

---

## PygrepHooks

`enum` · `ruff_linter::codes::PygrepHooks`

```rust
enum PygrepHooks
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_004`, `_005`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PygrepHooksIter
```

---

## Pylint

`enum` · `ruff_linter::codes::Pylint`

```rust
enum Pylint
```

**Variants**: `C`, `C0`, `C01`, `C010`, `C0105`, `C013`, `C0131`, `C0132`, `C02`, `C020`, `C0205`, `C0206`, `C0207`, `C0208`, `C04`, `C041`, `C0414`, `C0415`, `C1`, `C18`, `C180`, `C1802`, `C19`, `C190`, `C1901`, `C2`, `C24`, `C240`, `C2401`, `C2403`, `C27`, `C270`, `C2701`, `C28`, `C280`, `C2801`, `C3`, `C30`, `C300`, `C3002`, `E`, `E0`, `E01`, `E010`, `E0100`, `E0101`, `E011`, `E0115`, `E0116`, `E0117`, `E0118`, `E02`, `E023`, `E0237`, `E024`, `E0241`, `E03`, `E030`, `E0302`, `E0303`, `E0304`, `E0305`, `E0307`, `E0308`, `E0309`, `E06`, `E060`, `E0604`, `E0605`, `E064`, `E0643`, `E07`, `E070`, `E0704`, `E1`, `E11`, `E113`, `E1132`, `E114`, `E1141`, `E1142`, `E12`, `E120`, `E1205`, `E1206`, `E13`, `E130`, `E1300`, `E1307`, `E131`, `E1310`, `E15`, `E150`, `E1507`, `E151`, `E1519`, `E152`, `E1520`, `E17`, `E170`, `E1700`, `E2`, `E25`, `E250`, `E2502`, `E251`, `E2510`, `E2512`, `E2513`, `E2514`, `E2515`, `E4`, `E47`, `E470`, `E4703`, `R`, `R0`, `R01`, `R012`, `R0124`, `R013`, `R0133`, `R02`, `R020`, `R0202`, `R0203`, `R0206`, `R04`, `R040`, `R0402`, `R09`, `R090`, `R0904`, `R091`, `R0911`, `R0912`, `R0913`, `R0914`, `R0915`, `R0916`, `R0917`, `R1`, `R17`, `R170`, `R1701`, `R1702`, `R1704`, `R1706`, `R1708`, `R171`, `R1711`, `R1712`, `R1714`, `R1716`, `R172`, `R1722`, `R173`, `R1730`, `R1733`, `R1736`, `R2`, `R20`, `R200`, `R2004`, `R204`, `R2044`, `R5`, `R55`, `R550`, `R5501`, `R6`, `R61`, `R610`, `R6104`, `R62`, `R620`, `R6201`, `R63`, `R630`, `R6301`, `W`, `W0`, `W01`, `W010`, `W0108`, `W012`, `W0120`, `W0127`, `W0128`, `W0129`, `W013`, `W0131`, `W0133`, `W017`, `W0177`, `W02`, `W021`, `W0211`, `W024`, `W0244`, `W0245`, `W04`, `W040`, `W0406`, `W06`, `W060`, `W0602`, `W0603`, `W0604`, `W064`, `W0642`, `W07`, `W071`, `W0711`, `W0717`, `W1`, `W15`, `W150`, `W1501`, `W1507`, `W1508`, `W1509`, `W151`, `W1510`, `W1514`, `W16`, `W164`, `W1641`, `W2`, `W21`, `W210`, `W2101`, `W29`, `W290`, `W2901`, `W3`, `W32`, `W320`, `W3201`, `W33`, `W330`, `W3301`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PylintIter
```

---

## Pyupgrade

`enum` · `ruff_linter::codes::Pyupgrade`

```rust
enum Pyupgrade
```

**Variants**: `_0`, `_00`, `_001`, `_003`, `_004`, `_005`, `_006`, `_007`, `_008`, `_009`, `_01`, `_010`, `_011`, `_012`, `_013`, `_014`, `_015`, `_017`, `_018`, `_019`, `_02`, `_020`, `_021`, `_022`, `_023`, `_024`, `_025`, `_026`, `_027`, `_028`, `_029`, `_03`, `_030`, `_031`, `_032`, `_033`, `_034`, `_035`, `_036`, `_037`, `_038`, `_039`, `_04`, `_040`, `_041`, `_042`, `_043`, `_044`, `_045`, `_046`, `_047`, `_048`, `_049`, `_05`, `_050`, `_051`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PyupgradeIter
```

---

## Refurb

`enum` · `ruff_linter::codes::Refurb`

```rust
enum Refurb
```

**Variants**: `_1`, `_10`, `_101`, `_103`, `_105`, `_11`, `_110`, `_113`, `_116`, `_118`, `_12`, `_122`, `_129`, `_13`, `_131`, `_132`, `_136`, `_14`, `_140`, `_142`, `_145`, `_148`, `_15`, `_152`, `_154`, `_156`, `_157`, `_16`, `_161`, `_162`, `_163`, `_164`, `_166`, `_167`, `_168`, `_169`, `_17`, `_171`, `_177`, `_18`, `_180`, `_181`, `_187`, `_188`, `_189`, `_19`, `_192`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> RefurbIter
```

---

## Ruff

`enum` · `ruff_linter::codes::Ruff`

```rust
enum Ruff
```

**Variants**: `_0`, `_00`, `_001`, `_002`, `_003`, `_005`, `_006`, `_007`, `_008`, `_009`, `_01`, `_010`, `_011`, `_012`, `_013`, `_015`, `_016`, `_017`, `_018`, `_019`, `_02`, `_020`, `_021`, `_022`, `_023`, `_024`, `_026`, `_027`, `_028`, `_029`, `_03`, `_030`, `_031`, `_032`, `_033`, `_034`, `_035`, `_036`, `_037`, `_038`, `_039`, `_04`, `_040`, `_041`, `_043`, `_045`, `_046`, `_047`, `_048`, `_049`, `_05`, `_050`, `_051`, `_052`, `_053`, `_054`, `_055`, `_056`, `_057`, `_058`, `_059`, `_06`, `_060`, `_061`, `_063`, `_064`, `_065`, `_066`, `_067`, `_068`, `_069`, `_07`, `_070`, `_071`, `_072`, `_073`, `_074`, `_075`, `_077`, `_1`, `_10`, `_100`, `_101`, `_102`, `_103`, `_104`, `_105`, `_106`, `_2`, `_20`, `_200`, `_201`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> RuffIter
```

---

## Rule

`enum` · `ruff_linter::codes::Rule`

Also reachable as `ruff_linter::registry::Rule`

```rust
enum Rule
```

**Variants**: `AirflowVariableNameTaskIdMismatch`, `AirflowDagNoScheduleArgument`, `AirflowVariableGetOutsideTask`, `AirflowTaskBranchAsShortCircuit`, `AirflowXcomPullInTemplateString`, `AirflowTaskImplicitMultipleOutputs`, `Airflow3Removal`, `Airflow3MovedToProvider`, `Airflow3IncompatibleFunctionSignature`, `Airflow3DagDynamicValue`, `Airflow3SuggestedUpdate`, `Airflow3SuggestedToMoveToProvider`, `Airflow31Moved`, `CommentedOutCode`, `FastApiRedundantResponseModel`, `FastApiNonAnnotatedDependency`, `FastApiUnusedPathParameter`, `SysVersionSlice3`, `SysVersion2`, `SysVersionCmpStr3`, `SysVersionInfo0Eq3`, `SixPY3`, `SysVersionInfo1CmpInt`, `SysVersionInfoMinorCmpInt`, `SysVersion0`, `SysVersionCmpStr10`, `SysVersionSlice1`, `MissingTypeFunctionArgument`, `MissingTypeArgs`, `MissingTypeKwargs`, `MissingTypeSelf`, `MissingTypeCls`, `MissingReturnTypeUndocumentedPublicFunction`, `MissingReturnTypePrivateFunction`, `MissingReturnTypeSpecialMethod`, `MissingReturnTypeStaticMethod`, `MissingReturnTypeClassMethod`, `AnyType`, `CancelScopeNoCheckpoint`, `TrioSyncCall`, `AsyncFunctionWithTimeout`, `AsyncBusyWait`, `AsyncZeroSleep`, `LongSleepNotForever`, `YieldInContextManagerInAsyncGenerator`, `BlockingHttpCallInAsyncFunction`, `BlockingHttpCallHttpxInAsyncFunction`, `CreateSubprocessInAsyncFunction`, `RunProcessInAsyncFunction`, `WaitForProcessInAsyncFunction`, `BlockingOpenCallInAsyncFunction`, `BlockingPathMethodInAsyncFunction`, `BlockingInputInAsyncFunction`, `BlockingSleepInAsyncFunction`, `Assert`, `ExecBuiltin`, `BadFilePermissions`, `HardcodedBindAllInterfaces`, `HardcodedPasswordString`, `HardcodedPasswordFuncArg`, `HardcodedPasswordDefault`, `HardcodedTempFile`, `TryExceptPass`, `TryExceptContinue`, `RequestWithoutTimeout`, `FlaskDebugTrue`, `TarfileUnsafeMembers`, `SuspiciousPickleUsage`, `SuspiciousMarshalUsage`, `SuspiciousInsecureHashUsage`, `SuspiciousInsecureCipherUsage`, `SuspiciousInsecureCipherModeUsage`, `SuspiciousMktempUsage`, `SuspiciousEvalUsage`, `SuspiciousMarkSafeUsage`, `SuspiciousURLOpenUsage`, `SuspiciousNonCryptographicRandomUsage`, `SuspiciousTelnetUsage`, `SuspiciousXMLCElementTreeUsage`, `SuspiciousXMLElementTreeUsage`, `SuspiciousXMLExpatReaderUsage`, `SuspiciousXMLExpatBuilderUsage`, `SuspiciousXMLSaxUsage`, `SuspiciousXMLMiniDOMUsage`, `SuspiciousXMLPullDOMUsage`, `SuspiciousXMLETreeUsage`, `SuspiciousFTPLibUsage`, `SuspiciousUnverifiedContextUsage`, `HashlibInsecureHashFunction`, `SuspiciousTelnetlibImport`, `SuspiciousFtplibImport`, `SuspiciousPickleImport`, `SuspiciousSubprocessImport`, `SuspiciousXmlEtreeImport`, `SuspiciousXmlSaxImport`, `SuspiciousXmlExpatImport`, `SuspiciousXmlMinidomImport`, `SuspiciousXmlPulldomImport`, `SuspiciousLxmlImport`, `SuspiciousXmlrpcImport`, `SuspiciousHttpoxyImport`, `SuspiciousPycryptoImport`, `SuspiciousPyghmiImport`, `RequestWithNoCertValidation`, `SslInsecureVersion`, `SslWithBadDefaults`, `SslWithNoVersion`, `WeakCryptographicKey`, `UnsafeYAMLLoad`, `SSHNoHostKeyVerification`, `SnmpInsecureVersion`, `SnmpWeakCryptography`, `ParamikoCall`, `SubprocessPopenWithShellEqualsTrue`, `SubprocessWithoutShellEqualsTrue`, `CallWithShellEqualsTrue`, `StartProcessWithAShell`, `StartProcessWithNoShell`, `StartProcessWithPartialPath`, `HardcodedSQLExpression`, `UnixCommandWildcardInjection`, `DjangoExtra`, `DjangoRawSql`, `LoggingConfigInsecureListen`, `Jinja2AutoescapeFalse`, `MakoTemplates`, `UnsafeMarkupUse`, `BlindExcept`, `BooleanTypeHintPositionalArgument`, `BooleanDefaultValuePositionalArgument`, `BooleanPositionalValueInCall`, `UnaryPrefixIncrementDecrement`, `AssignmentToOsEnviron`, `UnreliableCallableCheck`, `StripWithMultiCharacters`, `MutableArgumentDefault`, `UnusedLoopControlVariable`, `FunctionCallInDefaultArgument`, `GetAttrWithConstant`, `SetAttrWithConstant`, `AssertFalse`, `JumpStatementInFinally`, `RedundantTupleInExceptionHandler`, `DuplicateHandlerException`, `UselessComparison`, `RaiseLiteral`, `AssertRaisesException`, `UselessExpression`, `CachedInstanceMethod`, `LoopVariableOverridesIterator`, `FStringDocstring`, `UselessContextlibSuppress`, `FunctionUsesLoopVariable`, `AbstractBaseClassWithoutAbstractMethod`, `DuplicateTryBlockException`, `StarArgUnpackingAfterKeywordArg`, `EmptyMethodWithoutAbstractDecorator`, `NoExplicitStacklevel`, `ExceptWithEmptyTuple`, `ExceptWithNonExceptionClasses`, `ReuseOfGroupbyGenerator`, `UnintentionalTypeAnnotation`, `DuplicateValue`, `ReSubPositionalArgs`, `StaticKeyDictComprehension`, `MutableContextvarDefault`, `DelAttrWithConstant`, `ReturnInGenerator`, `ClassAsDataStructure`, `RaiseWithoutFromInsideExcept`, `ZipWithoutExplicitStrict`, `LoopIteratorMutation`, `BatchedWithoutExplicitStrict`, `MapWithoutExplicitStrict`, `BuiltinVariableShadowing`, `BuiltinArgumentShadowing`, `BuiltinAttributeShadowing`, `BuiltinImportShadowing`, `StdlibModuleShadowing`, `BuiltinLambdaArgumentShadowing`, `MissingTrailingComma`, `TrailingCommaOnBareTuple`, `ProhibitedTrailingComma`, `UnnecessaryGeneratorList`, `UnnecessaryGeneratorSet`, `UnnecessaryGeneratorDict`, `UnnecessaryListComprehensionSet`, `UnnecessaryListComprehensionDict`, `UnnecessaryLiteralSet`, `UnnecessaryLiteralDict`, `UnnecessaryCollectionCall`, `UnnecessaryLiteralWithinTupleCall`, `UnnecessaryLiteralWithinListCall`, `UnnecessaryListCall`, `UnnecessaryCallAroundSorted`, `UnnecessaryDoubleCastOrProcess`, `UnnecessarySubscriptReversal`, `UnnecessaryComprehension`, `UnnecessaryMap`, `UnnecessaryLiteralWithinDictCall`, `UnnecessaryComprehensionInCall`, `UnnecessaryDictComprehensionForIterable`, `MissingCopyrightNotice`, `CallDatetimeWithoutTzinfo`, `CallDatetimeToday`, `CallDatetimeUtcnow`, `CallDatetimeUtcfromtimestamp`, `CallDatetimeNowWithoutTzinfo`, `CallDatetimeFromtimestamp`, `CallDatetimeStrptimeWithoutZone`, `CallDateToday`, `CallDateFromtimestamp`, `DatetimeMinMax`, `Debugger`, `DjangoNullableModelStringField`, `DjangoLocalsInRenderFunction`, `DjangoExcludeWithModelForm`, `DjangoAllWithModelForm`, `DjangoModelWithoutDunderStr`, `DjangoUnorderedBodyContentInModel`, `DjangoNonLeadingReceiverDecorator`, `RawStringInException`, `FStringInException`, `DotFormatInException`, `ShebangNotExecutable`, `ShebangMissingExecutableFile`, `ShebangMissingPython`, `ShebangLeadingWhitespace`, `ShebangNotFirstLine`, `LineContainsFixme`, `LineContainsTodo`, `LineContainsXxx`, `LineContainsHack`, `FutureRewritableTypeAnnotation`, `FutureRequiredTypeAnnotation`, `FStringInGetTextFuncCall`, `FormatInGetTextFuncCall`, `PrintfInGetTextFuncCall`, `SingleLineImplicitStringConcatenation`, `MultiLineImplicitStringConcatenation`, `ExplicitStringConcatenation`, `ImplicitStringConcatenationInCollectionLiteral`, `UnconventionalImportAlias`, `BannedImportAlias`, `BannedImportFrom`, `DirectLoggerInstantiation`, `InvalidGetLoggerArgument`, `LogExceptionOutsideExceptHandler`, `ExceptionWithoutExcInfo`, `UndocumentedWarn`, `ExcInfoOutsideExceptHandler`, `RootLoggerCall`, `LoggingStringFormat`, `LoggingPercentFormat`, `LoggingStringConcat`, `LoggingFString`, `LoggingWarn`, `LoggingExtraAttrClash`, `LoggingExcInfo`, `LoggingRedundantExcInfo`, `ImplicitNamespacePackage`, `UnnecessaryPlaceholder`, `DuplicateClassFieldDefinition`, `NonUniqueEnums`, `UnnecessarySpread`, `UnnecessaryDictKwargs`, `ReimplementedContainerBuiltin`, `UnnecessaryRangeStart`, `MultipleStartsEndsWith`, `Print`, `PPrint`, `UnprefixedTypeParam`, `ComplexIfStatementInStub`, `UnrecognizedVersionInfoCheck`, `PatchVersionComparison`, `WrongTupleLengthVersionComparison`, `BadVersionInfoComparison`, `UnrecognizedPlatformCheck`, `UnrecognizedPlatformName`, `PassStatementStubBody`, `NonEmptyStubBody`, `TypedArgumentDefaultInStub`, `PassInClassBody`, `EllipsisInNonEmptyClassBody`, `ArgumentDefaultInStub`, `AssignmentDefaultInStub`, `DuplicateUnionMember`, `ComplexAssignmentInStub`, `UnusedPrivateTypeVar`, `CustomTypeVarForSelf`, `QuotedAnnotationInStub`, `DocstringInStub`, `CollectionsNamedTuple`, `UnaliasedCollectionsAbcSetImport`, `TypeAliasWithoutAnnotation`, `StrOrReprDefinedInStub`, `UnnecessaryLiteralUnion`, `AnyEqNeAnnotation`, `LegacyTypeComment`, `NonSelfReturnType`, `UnassignedSpecialVariableInStub`, `BadExitAnnotation`, `RedundantNumericUnion`, `SnakeCaseTypeAlias`, `TSuffixedTypeAlias`, `FutureAnnotationsInStub`, `IterMethodReturnIterable`, `UnusedPrivateProtocol`, `UnusedPrivateTypeAlias`, `StubBodyMultipleStatements`, `UnusedPrivateTypedDict`, `NoReturnArgumentAnnotationInStub`, `RedundantLiteralUnion`, `UnannotatedAssignmentInStub`, `StringOrBytesTooLong`, `NumericLiteralTooLong`, `UnnecessaryTypeUnion`, `UnsupportedMethodCallOnAll`, `ByteStringUsage`, `GeneratorReturnFromIterMethod`, `GenericNotLastBaseClass`, `RedundantNoneLiteral`, `DuplicateLiteralMember`, `Pep484StylePositionalOnlyParameter`, `RedundantFinalLiteral`, `BadVersionInfoOrder`, `PytestFixtureIncorrectParenthesesStyle`, `PytestFixturePositionalArgs`, `PytestExtraneousScopeFunction`, `PytestMissingFixtureNameUnderscore`, `PytestIncorrectFixtureNameUnderscore`, `PytestParametrizeNamesWrongType`, `PytestParametrizeValuesWrongType`, `PytestPatchWithLambda`, `PytestUnittestAssertion`, `PytestRaisesWithoutException`, `PytestRaisesTooBroad`, `PytestRaisesWithMultipleStatements`, `PytestIncorrectPytestImport`, `PytestDuplicateParametrizeTestCases`, `PytestAssertAlwaysFalse`, `PytestFailWithoutMessage`, `PytestAssertInExcept`, `PytestCompositeAssertion`, `PytestFixtureParamWithoutValue`, `PytestDeprecatedYieldFixture`, `PytestFixtureFinalizerCallback`, `PytestUselessYieldFixture`, `PytestIncorrectMarkParenthesesStyle`, `PytestUnnecessaryAsyncioMarkOnFixture`, `PytestErroneousUseFixturesOnFixture`, `PytestUseFixturesWithoutParameters`, `PytestUnittestRaisesAssertion`, `PytestParameterWithDefaultArgument`, `PytestWarnsWithoutWarning`, `PytestWarnsTooBroad`, `PytestWarnsWithMultipleStatements`, `BadQuotesInlineString`, `BadQuotesMultilineString`, `BadQuotesDocstring`, `AvoidableEscapedQuote`, `UnnecessaryEscapedQuote`, `UnnecessaryParenOnRaiseException`, `UnnecessaryReturnNone`, `ImplicitReturnValue`, `ImplicitReturn`, `UnnecessaryAssign`, `SuperfluousElseReturn`, `SuperfluousElseRaise`, `SuperfluousElseContinue`, `SuperfluousElseBreak`, `PrivateMemberAccess`, `DuplicateIsinstanceCall`, `CollapsibleIf`, `NeedlessBool`, `SuppressibleException`, `ReturnInTryExceptFinally`, `IfElseBlockInsteadOfIfExp`, `CompareWithTuple`, `ReimplementedBuiltin`, `UncapitalizedEnvironmentVariables`, `EnumerateForLoop`, `IfWithSameArms`, `OpenFileWithContextHandler`, `IfElseBlockInsteadOfDictLookup`, `MultipleWithStatements`, `InDictKeys`, `NegateEqualOp`, `NegateNotEqualOp`, `DoubleNegation`, `IfExprWithTrueFalse`, `IfExprWithFalseTrue`, `IfExprWithTwistedArms`, `ExprAndNotExpr`, `ExprOrNotExpr`, `ExprOrTrue`, `ExprAndFalse`, `YodaConditions`, `IfElseBlockInsteadOfDictGet`, `SplitStaticString`, `DictGetWithNoneDefault`, `ZipDictKeysAndValues`, `NoSlotsInStrSubclass`, `NoSlotsInTupleSubclass`, `NoSlotsInNamedtupleSubclass`, `BannedApi`, `RelativeImports`, `BannedModuleLevelImports`, `LazyImportMismatch`, `LazyImportImmediatelyResolved`, `InvalidTodoTag`, `MissingTodoAuthor`, `MissingTodoLink`, `MissingTodoColon`, `MissingTodoDescription`, `InvalidTodoCapitalization`, `MissingSpaceAfterTodoColon`, `TypingOnlyFirstPartyImport`, `TypingOnlyThirdPartyImport`, `TypingOnlyStandardLibraryImport`, `RuntimeImportInTypeCheckingBlock`, `EmptyTypeCheckingBlock`, `RuntimeCastValue`, `UnquotedTypeAlias`, `QuotedTypeAlias`, `RuntimeStringUnion`, `UnusedFunctionArgument`, `UnusedMethodArgument`, `UnusedClassMethodArgument`, `UnusedStaticMethodArgument`, `UnusedLambdaArgument`, `OsPathAbspath`, `OsChmod`, `OsMkdir`, `OsMakedirs`, `OsRename`, `OsReplace`, `OsRmdir`, `OsRemove`, `OsUnlink`, `OsGetcwd`, `OsPathExists`, `OsPathExpanduser`, `OsPathIsdir`, `OsPathIsfile`, `OsPathIslink`, `OsReadlink`, `OsStat`, `OsPathIsabs`, `OsPathJoin`, `OsPathBasename`, `OsPathDirname`, `OsPathSamefile`, `OsPathSplitext`, `BuiltinOpen`, `PyPath`, `PathConstructorCurrentDirectory`, `OsPathGetsize`, `OsPathGetatime`, `OsPathGetmtime`, `OsPathGetctime`, `OsSepSplit`, `Glob`, `OsListdir`, `InvalidPathlibWithSuffix`, `OsSymlink`, `StaticJoinToFString`, `UnsortedImports`, `MissingRequiredImport`, `ComplexStructure`, `NumpyDeprecatedTypeAlias`, `NumpyLegacyRandom`, `NumpyDeprecatedFunction`, `Numpy2Deprecation`, `InvalidClassName`, `InvalidFunctionName`, `InvalidArgumentName`, `InvalidFirstArgumentNameForClassMethod`, `InvalidFirstArgumentNameForMethod`, `NonLowercaseVariableInFunction`, `DunderFunctionName`, `ConstantImportedAsNonConstant`, `LowercaseImportedAsNonLowercase`, `CamelcaseImportedAsLowercase`, `CamelcaseImportedAsConstant`, `MixedCaseVariableInClassScope`, `MixedCaseVariableInGlobalScope`, `CamelcaseImportedAsAcronym`, `ErrorSuffixOnExceptionName`, `InvalidModuleName`, `PandasUseOfInplaceArgument`, `PandasUseOfDotIsNull`, `PandasUseOfDotNotNull`, `PandasUseOfDotIx`, `PandasUseOfDotAt`, `PandasUseOfDotIat`, `PandasUseOfDotPivotOrUnstack`, `PandasUseOfDotValues`, `PandasUseOfDotReadTable`, `PandasUseOfDotStack`, `PandasUseOfPdMerge`, `PandasNuniqueConstantSeriesCheck`, `PandasDfVariableName`, `UnnecessaryListCast`, `IncorrectDictIterator`, `TryExceptInLoop`, `ManualListComprehension`, `ManualListCopy`, `ManualDictComprehension`, `MixedSpacesAndTabs`, `IndentationWithInvalidMultiple`, `NoIndentedBlock`, `UnexpectedIndentation`, `IndentationWithInvalidMultipleComment`, `NoIndentedBlockComment`, `UnexpectedIndentationComment`, `OverIndented`, `WhitespaceAfterOpenBracket`, `WhitespaceBeforeCloseBracket`, `WhitespaceBeforePunctuation`, `WhitespaceAfterDecorator`, `WhitespaceBeforeParameters`, `MultipleSpacesBeforeOperator`, `MultipleSpacesAfterOperator`, `TabBeforeOperator`, `TabAfterOperator`, `MissingWhitespaceAroundOperator`, `MissingWhitespaceAroundArithmeticOperator`, `MissingWhitespaceAroundBitwiseOrShiftOperator`, `MissingWhitespaceAroundModuloOperator`, `MissingWhitespace`, `MultipleSpacesAfterComma`, `TabAfterComma`, `UnexpectedSpacesAroundKeywordParameterEquals`, `MissingWhitespaceAroundParameterEquals`, `TooFewSpacesBeforeInlineComment`, `NoSpaceAfterInlineComment`, `NoSpaceAfterBlockComment`, `MultipleLeadingHashesForBlockComment`, `MultipleSpacesAfterKeyword`, `MultipleSpacesBeforeKeyword`, `TabAfterKeyword`, `TabBeforeKeyword`, `MissingWhitespaceAfterKeyword`, `BlankLineBetweenMethods`, `BlankLinesTopLevel`, `TooManyBlankLines`, `BlankLineAfterDecorator`, `BlankLinesAfterFunctionOrClass`, `BlankLinesBeforeNestedDefinition`, `MultipleImportsOnOneLine`, `ModuleImportNotAtTopOfFile`, `LineTooLong`, `RedundantBackslash`, `MultipleStatementsOnOneLineColon`, `MultipleStatementsOnOneLineSemicolon`, `UselessSemicolon`, `NoneComparison`, `TrueFalseComparison`, `NotInTest`, `NotIsTest`, `TypeComparison`, `BareExcept`, `LambdaAssignment`, `AmbiguousVariableName`, `AmbiguousClassName`, `AmbiguousFunctionName`, `IOError`, `SyntaxError`, `TabIndentation`, `TrailingWhitespace`, `MissingNewlineAtEndOfFile`, `BlankLineWithWhitespace`, `TooManyNewlinesAtEndOfFile`, `DocLineTooLong`, `InvalidEscapeSequence`, `DocstringExtraneousParameter`, `DocstringMissingReturns`, `DocstringExtraneousReturns`, `DocstringMissingYields`, `DocstringExtraneousYields`, `DocstringMissingException`, `DocstringExtraneousException`, `UndocumentedPublicModule`, `UndocumentedPublicClass`, `UndocumentedPublicMethod`, `UndocumentedPublicFunction`, `UndocumentedPublicPackage`, `UndocumentedMagicMethod`, `UndocumentedPublicNestedClass`, `UndocumentedPublicInit`, `UnnecessaryMultilineDocstring`, `BlankLineBeforeFunction`, `BlankLineAfterFunction`, `IncorrectBlankLineBeforeClass`, `IncorrectBlankLineAfterClass`, `MissingBlankLineAfterSummary`, `DocstringTabIndentation`, `UnderIndentation`, `OverIndentation`, `NewLineAfterLastParagraph`, `SurroundingWhitespace`, `BlankLineBeforeClass`, `MultiLineSummaryFirstLine`, `MultiLineSummarySecondLine`, `OverindentedSection`, `OverindentedSectionUnderline`, `TripleSingleQuotes`, `EscapeSequenceInDocstring`, `MissingTrailingPeriod`, `NonImperativeMood`, `SignatureInDocstring`, `FirstWordUncapitalized`, `DocstringStartsWithThis`, `NonCapitalizedSectionName`, `MissingNewLineAfterSectionName`, `MissingDashedUnderlineAfterSection`, `MissingSectionUnderlineAfterName`, `MismatchedSectionUnderlineLength`, `NoBlankLineAfterSection`, `NoBlankLineBeforeSection`, `BlankLinesBetweenHeaderAndContent`, `MissingBlankLineAfterLastSection`, `EmptyDocstringSection`, `MissingTerminalPunctuation`, `MissingSectionNameColon`, `UndocumentedParam`, `OverloadWithDocstring`, `EmptyDocstring`, `IncorrectSectionOrder`, `PropertyDocstringStartsWithVerb`, `UnusedImport`, `ImportShadowedByLoopVar`, `UndefinedLocalWithImportStar`, `LateFutureImport`, `UndefinedLocalWithImportStarUsage`, `UndefinedLocalWithNestedImportStarUsage`, `FutureFeatureNotDefined`, `PercentFormatInvalidFormat`, `PercentFormatExpectedMapping`, `PercentFormatExpectedSequence`, `PercentFormatExtraNamedArguments`, `PercentFormatMissingArgument`, `PercentFormatMixedPositionalAndNamed`, `PercentFormatPositionalCountMismatch`, `PercentFormatStarRequiresSequence`, `PercentFormatUnsupportedFormatCharacter`, `StringDotFormatInvalidFormat`, `StringDotFormatExtraNamedArguments`, `StringDotFormatExtraPositionalArguments`, `StringDotFormatMissingArguments`, `StringDotFormatMixingAutomatic`, `FStringMissingPlaceholders`, `MultiValueRepeatedKeyLiteral`, `MultiValueRepeatedKeyVariable`, `ExpressionsInStarAssignment`, `MultipleStarredExpressions`, `AssertTuple`, `IsLiteral`, `InvalidPrintSyntax`, `IfTuple`, `BreakOutsideLoop`, `ContinueOutsideLoop`, `YieldOutsideFunction`, `ReturnOutsideFunction`, `DefaultExceptNotLast`, `ForwardAnnotationSyntaxError`, `RedefinedWhileUnused`, `UndefinedName`, `UndefinedExport`, `UndefinedLocal`, `UnusedVariable`, `UnusedAnnotation`, `RaiseNotImplemented`, `Eval`, `DeprecatedLogWarn`, `BlanketTypeIgnore`, `BlanketNOQA`, `InvalidMockAccess`, `TypeNameIncorrectVariance`, `TypeBivariance`, `TypeParamNameMismatch`, `SingleStringSlots`, `DictIndexMissingItems`, `MissingMaxsplitArg`, `IterationOverSet`, `UselessImportAlias`, `ImportOutsideTopLevel`, `LenTest`, `CompareToEmptyString`, `NonAsciiName`, `NonAsciiImportName`, `ImportPrivateName`, `UnnecessaryDunderCall`, `UnnecessaryDirectLambdaCall`, `YieldInInit`, `ReturnInInit`, `NonlocalAndGlobal`, `ContinueInFinally`, `NonlocalWithoutBinding`, `LoadBeforeGlobalDeclaration`, `NonSlotAssignment`, `DuplicateBases`, `UnexpectedSpecialMethodSignature`, `InvalidLengthReturnType`, `InvalidBoolReturnType`, `InvalidIndexReturnType`, `InvalidStrReturnType`, `InvalidBytesReturnType`, `InvalidHashReturnType`, `InvalidAllObject`, `InvalidAllFormat`, `PotentialIndexError`, `MisplacedBareRaise`, `RepeatedKeywordArgument`, `DictIterMissingItems`, `AwaitOutsideAsync`, `LoggingTooManyArgs`, `LoggingTooFewArgs`, `BadStringFormatCharacter`, `BadStringFormatType`, `BadStrStripCall`, `InvalidEnvvarValue`, `SingledispatchMethod`, `SingledispatchmethodFunction`, `YieldFromInAsyncFunction`, `BidirectionalUnicode`, `InvalidCharacterBackspace`, `InvalidCharacterSub`, `InvalidCharacterEsc`, `InvalidCharacterNul`, `InvalidCharacterZeroWidthSpace`, `ModifiedIteratingSet`, `ComparisonWithItself`, `ComparisonOfConstant`, `NoClassmethodDecorator`, `NoStaticmethodDecorator`, `PropertyWithParameters`, `ManualFromImport`, `TooManyPublicMethods`, `TooManyReturnStatements`, `TooManyBranches`, `TooManyArguments`, `TooManyLocals`, `TooManyStatements`, `TooManyBooleanExpressions`, `TooManyPositionalArguments`, `RepeatedIsinstanceCalls`, `TooManyNestedBlocks`, `RedefinedArgumentFromLocal`, `AndOrTernary`, `StopIterationReturn`, `UselessReturn`, `SwapWithTemporaryVariable`, `RepeatedEqualityComparison`, `BooleanChainedComparison`, `SysExitAlias`, `IfStmtMinMax`, `UnnecessaryDictIndexLookup`, `UnnecessaryListIndexLookup`, `MagicValueComparison`, `EmptyComment`, `CollapsibleElseIf`, `NonAugmentedAssignment`, `LiteralMembership`, `NoSelfUse`, `UnnecessaryLambda`, `UselessElseOnLoop`, `SelfAssigningVariable`, `RedeclaredAssignedName`, `AssertOnStringLiteral`, `NamedExprWithoutContext`, `UselessExceptionStatement`, `NanComparison`, `BadStaticmethodArgument`, `RedefinedSlotsInSubclass`, `SuperWithoutBrackets`, `ImportSelf`, `GlobalVariableNotAssigned`, `GlobalStatement`, `GlobalAtModuleLevel`, `SelfOrClsAssignment`, `BinaryOpException`, `TooManyStatementsInTryClause`, `BadOpenMode`, `ShallowCopyEnviron`, `InvalidEnvvarDefault`, `SubprocessPopenPreexecFn`, `SubprocessRunWithoutCheck`, `UnspecifiedEncoding`, `EqWithoutHash`, `UselessWithLock`, `RedefinedLoopName`, `BadDunderMethodName`, `NestedMinMax`, `UselessMetaclassType`, `TypeOfPrimitive`, `UselessObjectInheritance`, `DeprecatedUnittestAlias`, `NonPEP585Annotation`, `NonPEP604AnnotationUnion`, `SuperCallWithParameters`, `UTF8EncodingDeclaration`, `UnnecessaryFutureImport`, `LRUCacheWithoutParameters`, `UnnecessaryEncodeUTF8`, `ConvertTypedDictFunctionalToClass`, `ConvertNamedTupleFunctionalToClass`, `RedundantOpenModes`, `DatetimeTimezoneUTC`, `NativeLiterals`, `TypingTextStrAlias`, `OpenAlias`, `ReplaceUniversalNewlines`, `ReplaceStdoutStderr`, `DeprecatedCElementTree`, `OSErrorAlias`, `UnicodeKindPrefix`, `DeprecatedMockImport`, `UnpackedListComprehension`, `YieldInForLoop`, `UnnecessaryBuiltinImport`, `FormatLiterals`, `PrintfStringFormatting`, `FString`, `LRUCacheWithMaxsizeNone`, `ExtraneousParentheses`, `DeprecatedImport`, `OutdatedVersionBlock`, `QuotedAnnotation`, `NonPEP604Isinstance`, `UnnecessaryClassParentheses`, `NonPEP695TypeAlias`, `TimeoutErrorAlias`, `ReplaceStrEnum`, `UnnecessaryDefaultTypeArgs`, `NonPEP646Unpack`, `NonPEP604AnnotationOptional`, `NonPEP695GenericClass`, `NonPEP695GenericFunction`, `WhileOne`, `PrivateTypeParameter`, `UselessClassMetaclassType`, `DeprecatedAbcDecorator`, `ReadWholeFile`, `WriteWholeFile`, `PrintEmptyString`, `IfExpInsteadOfOrOperator`, `RepeatedAppend`, `FStringNumberFormat`, `ReimplementedOperator`, `ForLoopWrites`, `ReadlinesInFor`, `DeleteFullSlice`, `CheckAndRemoveFromSet`, `IfExprMinMax`, `ReimplementedStarmap`, `ForLoopSetMutations`, `SliceCopy`, `UnnecessaryEnumerate`, `MathConstant`, `RepeatedGlobal`, `HardcodedStringCharset`, `VerboseDecimalConstructor`, `BitCount`, `FromisoformatReplaceZ`, `RedundantLogBase`, `UnnecessaryFromFloat`, `IntOnSlicedStr`, `RegexFlagAlias`, `IsinstanceTypeNone`, `TypeNoneComparison`, `SingleItemMembershipTest`, `ImplicitCwd`, `MetaClassABCMeta`, `HashlibDigestHex`, `ListReverseCopy`, `SliceToRemovePrefixOrSuffix`, `SubclassBuiltin`, `SortedMinMax`, `AmbiguousUnicodeCharacterString`, `AmbiguousUnicodeCharacterDocstring`, `AmbiguousUnicodeCharacterComment`, `CollectionLiteralConcatenation`, `AsyncioDanglingTask`, `ZipInsteadOfPairwise`, `MutableDataclassDefault`, `FunctionCallInDataclassDefaultArgument`, `ExplicitFStringTypeConversion`, `RuffStaticKeyDictComprehension`, `MutableClassDefault`, `ImplicitOptional`, `UnnecessaryIterableAllocationForFirstElement`, `InvalidIndexType`, `QuadraticListSummation`, `AssignmentInAssert`, `UnnecessaryKeyCheck`, `NeverUnion`, `ParenthesizeChainedOperators`, `UnsortedDunderAll`, `UnsortedDunderSlots`, `MutableFromkeysValue`, `DefaultFactoryKwarg`, `MissingFStringSyntax`, `InvalidFormatterSuppressionComment`, `UnusedAsync`, `AssertWithPrintMessage`, `IncorrectlyParenthesizedTupleInSubscript`, `DecimalFromFloatLiteral`, `PostInitDefault`, `UselessIfElse`, `RuffUnsafeMarkupUse`, `NoneNotAtEndOfUnion`, `UnnecessaryEmptyIterableWithinDequeCall`, `RedundantBoolLiteral`, `UnrawRePattern`, `InvalidAssertMessageLiteralArgument`, `UnnecessaryNestedLiteral`, `PytestRaisesAmbiguousPattern`, `ImplicitClassVarInDataclass`, `UnnecessaryCastToInt`, `NeedlessElse`, `MapIntVersionParsing`, `DataclassEnum`, `UnnecessaryIf`, `IfKeyInDictDel`, `UsedDummyVariable`, `ClassWithMixedTypeVars`, `IndentedFormFeed`, `UnnecessaryRegularExpression`, `FalsyDictGetFallback`, `UnnecessaryRound`, `StarmapZip`, `UnusedUnpackedVariable`, `InEmptyCollection`, `LegacyFormPytestRaises`, `AccessAnnotationsFromClassDict`, `NonOctalPermissions`, `LoggingEagerConversion`, `PropertyWithoutReturn`, `NonEmptyInitModule`, `DuplicateEntryInDunderAll`, `FloatEqualityComparison`, `UnnecessaryAssignBeforeYield`, `OsPathCommonprefix`, `UselessFinally`, `FStringPercentFormat`, `IncorrectDecoratorOrder`, `FallibleContextManager`, `MethodReceiverDefault`, `UnusedNOQA`, `RedirectedNOQA`, `InvalidRuleCode`, `InvalidSuppressionComment`, `UnmatchedSuppressionComment`, `NoqaComments`, `RuleCodesInSuppressionComments`, `InvalidPyprojectToml`, `RuleCodesInSelectors`, `RaiseVanillaClass`, `RaiseVanillaArgs`, `TypeCheckWithoutTypeError`, `ReraiseNoCause`, `VerboseRaise`, `UselessTryExcept`, `TryConsiderElse`, `RaiseWithinTry`, `ErrorInsteadOfException`, `VerboseLogMessage`, `PytestFixtureAutouse`

**Implements**: `core::fmt::Display`, `strum::IntoEnumIterator`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (18)

```rust
fn category(&self) -> codes::Category
fn explanation(&self) -> Option<&'static str>
fn file(&self) -> &'static str
const fn fixable(&self) -> FixAvailability
fn from_code(code: &str) -> Result<Self, FromCodeError>
fn from_name(name: &str) -> Result<Self, FromNameError>
fn is_deprecated(&self) -> bool
fn is_preview(&self) -> bool
fn is_removed(&self) -> bool
fn line(&self) -> u32
const fn lint_source(&self) -> LintSource
fn message_formats(&self) -> &'static [&'static str]
fn name(&self) -> LintName
fn name_and_code(&self) -> impl std::fmt::Display + use<>
fn noqa_code(&self) -> Option<NoqaCode>
fn status(&self) -> codes::RuleStatus
fn upstream_category(&self, linter: &Linter) -> Option<UpstreamCategoryAndPrefix>
fn url(&self) -> Option<String>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> RuleIter
```

---

## RuleCodePrefix

`enum` · `ruff_linter::codes::RuleCodePrefix`

```rust
enum RuleCodePrefix
```

**Variants**: `Airflow`, `Eradicate`, `FastApi`, `Flake82020`, `Flake8Annotations`, `Flake8Async`, `Flake8Bandit`, `Flake8BlindExcept`, `Flake8BooleanTrap`, `Flake8Bugbear`, `Flake8Builtins`, `Flake8Commas`, `Flake8Comprehensions`, `Flake8Copyright`, `Flake8Datetimez`, `Flake8Debugger`, `Flake8Django`, `Flake8ErrMsg`, `Flake8Executable`, `Flake8Fixme`, `Flake8FutureAnnotations`, `Flake8GetText`, `Flake8ImplicitStrConcat`, `Flake8ImportConventions`, `Flake8Logging`, `Flake8LoggingFormat`, `Flake8NoPep420`, `Flake8Pie`, `Flake8Print`, `Flake8Pyi`, `Flake8PytestStyle`, `Flake8Quotes`, `Flake8Raise`, `Flake8Return`, `Flake8Self`, `Flake8Simplify`, `Flake8Slots`, `Flake8TidyImports`, `Flake8Todos`, `Flake8TypeChecking`, `Flake8UnusedArguments`, `Flake8UsePathlib`, `Flynt`, `Isort`, `McCabe`, `Numpy`, `PEP8Naming`, `PandasVet`, `Perflint`, `Pycodestyle`, `Pydoclint`, `Pydocstyle`, `Pyflakes`, `PygrepHooks`, `Pylint`, `Pyupgrade`, `Refurb`, `Ruff`, `Tryceratops`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn linter(&self) -> &'static Linter
fn short_code(&self) -> &'static str
```

**via `core::convert::From`**

```rust
fn from(linter: Refurb) -> Self
fn from(linter: FastApi) -> Self
fn from(linter: Flake8GetText) -> Self
fn from(linter: Flake8UnusedArguments) -> Self
fn from(linter: Flake8Async) -> Self
fn from(linter: Flake8Logging) -> Self
fn from(linter: Isort) -> Self
fn from(linter: Flake8BooleanTrap) -> Self
fn from(linter: Flake8Pie) -> Self
fn from(linter: PEP8Naming) -> Self
fn from(linter: Flake8Commas) -> Self
fn from(linter: Flake8PytestStyle) -> Self
fn from(linter: Pycodestyle) -> Self
fn from(linter: Flake8Datetimez) -> Self
fn from(linter: Flake8Return) -> Self
fn from(linter: Pyflakes) -> Self
fn from(linter: Flake8ErrMsg) -> Self
fn from(linter: Flake8Slots) -> Self
fn from(linter: Pyupgrade) -> Self
fn from(linter: Eradicate) -> Self
fn from(linter: Flake8FutureAnnotations) -> Self
fn from(linter: Flake8TypeChecking) -> Self
fn from(linter: Tryceratops) -> Self
fn from(linter: Flake8Annotations) -> Self
fn from(linter: Flake8ImportConventions) -> Self
fn from(linter: Flynt) -> Self
fn from(linter: Flake8BlindExcept) -> Self
fn from(linter: Flake8NoPep420) -> Self
fn from(linter: Numpy) -> Self
fn from(linter: Flake8Builtins) -> Self
fn from(linter: Flake8Pyi) -> Self
fn from(linter: Perflint) -> Self
fn from(linter: Flake8Copyright) -> Self
fn from(linter: Flake8Raise) -> Self
fn from(linter: Pydocstyle) -> Self
fn from(linter: Flake8Django) -> Self
fn from(linter: Flake8Simplify) -> Self
fn from(linter: Pylint) -> Self
fn from(linter: Airflow) -> Self
fn from(linter: Flake8Fixme) -> Self
fn from(linter: Flake8Todos) -> Self
fn from(linter: Ruff) -> Self
fn from(linter: Flake82020) -> Self
fn from(linter: Flake8ImplicitStrConcat) -> Self
fn from(linter: Flake8UsePathlib) -> Self
fn from(linter: Flake8Bandit) -> Self
fn from(linter: Flake8LoggingFormat) -> Self
fn from(linter: McCabe) -> Self
fn from(linter: Flake8Bugbear) -> Self
fn from(linter: Flake8Print) -> Self
fn from(linter: PandasVet) -> Self
fn from(linter: Flake8Comprehensions) -> Self
fn from(linter: Flake8Quotes) -> Self
fn from(linter: Pydoclint) -> Self
fn from(linter: Flake8Debugger) -> Self
fn from(linter: Flake8Self) -> Self
fn from(linter: PygrepHooks) -> Self
fn from(linter: Flake8Executable) -> Self
fn from(linter: Flake8TidyImports) -> Self
```

---

## RuleStatus

`enum` · `ruff_linter::codes::RuleStatus`

```rust
enum RuleStatus
```

**Variants**: `Stable`, `Preview`, `Deprecated`, `Removed`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Tryceratops

`enum` · `ruff_linter::codes::Tryceratops`

```rust
enum Tryceratops
```

**Variants**: `_0`, `_00`, `_002`, `_003`, `_004`, `_2`, `_20`, `_200`, `_201`, `_203`, `_3`, `_30`, `_300`, `_301`, `_4`, `_40`, `_400`, `_401`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(code: &str) -> Result<Self, Self::Err>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> TryceratopsIter
```

---

## AirflowIter

`struct` · `ruff_linter::codes::AirflowIter`

```rust
struct AirflowIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Airflow]

---

## CategoryIter

`struct` · `ruff_linter::codes::CategoryIter`

```rust
struct CategoryIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Category]

---

## EradicateIter

`struct` · `ruff_linter::codes::EradicateIter`

```rust
struct EradicateIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Eradicate]

---

## FastApiIter

`struct` · `ruff_linter::codes::FastApiIter`

```rust
struct FastApiIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [FastApi]

---

## Flake82020Iter

`struct` · `ruff_linter::codes::Flake82020Iter`

```rust
struct Flake82020Iter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake82020]

---

## Flake8AnnotationsIter

`struct` · `ruff_linter::codes::Flake8AnnotationsIter`

```rust
struct Flake8AnnotationsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Annotations]

---

## Flake8AsyncIter

`struct` · `ruff_linter::codes::Flake8AsyncIter`

```rust
struct Flake8AsyncIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Async]

---

## Flake8BanditIter

`struct` · `ruff_linter::codes::Flake8BanditIter`

```rust
struct Flake8BanditIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Bandit]

---

## Flake8BlindExceptIter

`struct` · `ruff_linter::codes::Flake8BlindExceptIter`

```rust
struct Flake8BlindExceptIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8BlindExcept]

---

## Flake8BooleanTrapIter

`struct` · `ruff_linter::codes::Flake8BooleanTrapIter`

```rust
struct Flake8BooleanTrapIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8BooleanTrap]

---

## Flake8BugbearIter

`struct` · `ruff_linter::codes::Flake8BugbearIter`

```rust
struct Flake8BugbearIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Bugbear]

---

## Flake8BuiltinsIter

`struct` · `ruff_linter::codes::Flake8BuiltinsIter`

```rust
struct Flake8BuiltinsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Builtins]

---

## Flake8CommasIter

`struct` · `ruff_linter::codes::Flake8CommasIter`

```rust
struct Flake8CommasIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Commas]

---

## Flake8ComprehensionsIter

`struct` · `ruff_linter::codes::Flake8ComprehensionsIter`

```rust
struct Flake8ComprehensionsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Comprehensions]

---

## Flake8CopyrightIter

`struct` · `ruff_linter::codes::Flake8CopyrightIter`

```rust
struct Flake8CopyrightIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Copyright]

---

## Flake8DatetimezIter

`struct` · `ruff_linter::codes::Flake8DatetimezIter`

```rust
struct Flake8DatetimezIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Datetimez]

---

## Flake8DebuggerIter

`struct` · `ruff_linter::codes::Flake8DebuggerIter`

```rust
struct Flake8DebuggerIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Debugger]

---

## Flake8DjangoIter

`struct` · `ruff_linter::codes::Flake8DjangoIter`

```rust
struct Flake8DjangoIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Django]

---

## Flake8ErrMsgIter

`struct` · `ruff_linter::codes::Flake8ErrMsgIter`

```rust
struct Flake8ErrMsgIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8ErrMsg]

---

## Flake8ExecutableIter

`struct` · `ruff_linter::codes::Flake8ExecutableIter`

```rust
struct Flake8ExecutableIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Executable]

---

## Flake8FixmeIter

`struct` · `ruff_linter::codes::Flake8FixmeIter`

```rust
struct Flake8FixmeIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Fixme]

---

## Flake8FutureAnnotationsIter

`struct` · `ruff_linter::codes::Flake8FutureAnnotationsIter`

```rust
struct Flake8FutureAnnotationsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8FutureAnnotations]

---

## Flake8GetTextIter

`struct` · `ruff_linter::codes::Flake8GetTextIter`

```rust
struct Flake8GetTextIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8GetText]

---

## Flake8ImplicitStrConcatIter

`struct` · `ruff_linter::codes::Flake8ImplicitStrConcatIter`

```rust
struct Flake8ImplicitStrConcatIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8ImplicitStrConcat]

---

## Flake8ImportConventionsIter

`struct` · `ruff_linter::codes::Flake8ImportConventionsIter`

```rust
struct Flake8ImportConventionsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8ImportConventions]

---

## Flake8LoggingFormatIter

`struct` · `ruff_linter::codes::Flake8LoggingFormatIter`

```rust
struct Flake8LoggingFormatIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8LoggingFormat]

---

## Flake8LoggingIter

`struct` · `ruff_linter::codes::Flake8LoggingIter`

```rust
struct Flake8LoggingIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Logging]

---

## Flake8NoPep420Iter

`struct` · `ruff_linter::codes::Flake8NoPep420Iter`

```rust
struct Flake8NoPep420Iter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8NoPep420]

---

## Flake8PieIter

`struct` · `ruff_linter::codes::Flake8PieIter`

```rust
struct Flake8PieIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Pie]

---

## Flake8PrintIter

`struct` · `ruff_linter::codes::Flake8PrintIter`

```rust
struct Flake8PrintIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Print]

---

## Flake8PyiIter

`struct` · `ruff_linter::codes::Flake8PyiIter`

```rust
struct Flake8PyiIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Pyi]

---

## Flake8PytestStyleIter

`struct` · `ruff_linter::codes::Flake8PytestStyleIter`

```rust
struct Flake8PytestStyleIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8PytestStyle]

---

## Flake8QuotesIter

`struct` · `ruff_linter::codes::Flake8QuotesIter`

```rust
struct Flake8QuotesIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Quotes]

---

## Flake8RaiseIter

`struct` · `ruff_linter::codes::Flake8RaiseIter`

```rust
struct Flake8RaiseIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Raise]

---

## Flake8ReturnIter

`struct` · `ruff_linter::codes::Flake8ReturnIter`

```rust
struct Flake8ReturnIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Return]

---

## Flake8SelfIter

`struct` · `ruff_linter::codes::Flake8SelfIter`

```rust
struct Flake8SelfIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Self]

---

## Flake8SimplifyIter

`struct` · `ruff_linter::codes::Flake8SimplifyIter`

```rust
struct Flake8SimplifyIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Simplify]

---

## Flake8SlotsIter

`struct` · `ruff_linter::codes::Flake8SlotsIter`

```rust
struct Flake8SlotsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Slots]

---

## Flake8TidyImportsIter

`struct` · `ruff_linter::codes::Flake8TidyImportsIter`

```rust
struct Flake8TidyImportsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8TidyImports]

---

## Flake8TodosIter

`struct` · `ruff_linter::codes::Flake8TodosIter`

```rust
struct Flake8TodosIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8Todos]

---

## Flake8TypeCheckingIter

`struct` · `ruff_linter::codes::Flake8TypeCheckingIter`

```rust
struct Flake8TypeCheckingIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8TypeChecking]

---

## Flake8UnusedArgumentsIter

`struct` · `ruff_linter::codes::Flake8UnusedArgumentsIter`

```rust
struct Flake8UnusedArgumentsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8UnusedArguments]

---

## Flake8UsePathlibIter

`struct` · `ruff_linter::codes::Flake8UsePathlibIter`

```rust
struct Flake8UsePathlibIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flake8UsePathlib]

---

## FlyntIter

`struct` · `ruff_linter::codes::FlyntIter`

```rust
struct FlyntIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Flynt]

---

## IsortIter

`struct` · `ruff_linter::codes::IsortIter`

```rust
struct IsortIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Isort]

---

## McCabeIter

`struct` · `ruff_linter::codes::McCabeIter`

```rust
struct McCabeIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [McCabe]

---

## NoqaCode

`struct` · `ruff_linter::codes::NoqaCode`

```rust
struct NoqaCode
```

**Implements**: `core::fmt::Display`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn prefix(&self) -> &str
fn suffix(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## NumpyIter

`struct` · `ruff_linter::codes::NumpyIter`

```rust
struct NumpyIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Numpy]

---

## PEP8NamingIter

`struct` · `ruff_linter::codes::PEP8NamingIter`

```rust
struct PEP8NamingIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [PEP8Naming]

---

## PandasVetIter

`struct` · `ruff_linter::codes::PandasVetIter`

```rust
struct PandasVetIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [PandasVet]

---

## PerflintIter

`struct` · `ruff_linter::codes::PerflintIter`

```rust
struct PerflintIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Perflint]

---

## PycodestyleIter

`struct` · `ruff_linter::codes::PycodestyleIter`

```rust
struct PycodestyleIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Pycodestyle]

---

## PydoclintIter

`struct` · `ruff_linter::codes::PydoclintIter`

```rust
struct PydoclintIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Pydoclint]

---

## PydocstyleIter

`struct` · `ruff_linter::codes::PydocstyleIter`

```rust
struct PydocstyleIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Pydocstyle]

---

## PyflakesIter

`struct` · `ruff_linter::codes::PyflakesIter`

```rust
struct PyflakesIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Pyflakes]

---

## PygrepHooksIter

`struct` · `ruff_linter::codes::PygrepHooksIter`

```rust
struct PygrepHooksIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [PygrepHooks]

---

## PylintIter

`struct` · `ruff_linter::codes::PylintIter`

```rust
struct PylintIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Pylint]

---

## PyupgradeIter

`struct` · `ruff_linter::codes::PyupgradeIter`

```rust
struct PyupgradeIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Pyupgrade]

---

## RefurbIter

`struct` · `ruff_linter::codes::RefurbIter`

```rust
struct RefurbIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Refurb]

---

## RuffIter

`struct` · `ruff_linter::codes::RuffIter`

```rust
struct RuffIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Ruff]

---

## RuleIter

`struct` · `ruff_linter::codes::RuleIter`

```rust
struct RuleIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Rule]

---

## TryceratopsIter

`struct` · `ruff_linter::codes::TryceratopsIter`

```rust
struct TryceratopsIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Tryceratops]

---
