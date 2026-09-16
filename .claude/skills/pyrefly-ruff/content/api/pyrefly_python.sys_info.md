# `pyrefly_python::sys_info`

Crate `pyrefly_python` · 3 public items · structured records in [`model/pyrefly_python.sys_info.json`](../model/pyrefly_python.sys_info.json)

## PythonPlatform

`enum` · `pyrefly_python::sys_info::PythonPlatform`

```rust
enum PythonPlatform
```

**Variants**: `All`, `Platforms`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (6)

```rust
fn linux() -> Self
fn mac() -> Self
fn new(platform: &str) -> Self
fn new_many(platforms: Vec<String>) -> Self
fn new_platforms(platforms: impl IntoIterator<Item = Self>) -> Self
fn windows() -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

The platform on which Python is running, e.g. "linux", "darwin", "win32".
See <https://docs.python.org/3/library/sys.html#sys.platform> for examples.

---

## PythonVersion

`struct` · `pyrefly_python::sys_info::PythonVersion`

```rust
struct PythonVersion
```

**Fields**: `major`, `minor`, `micro`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `dupe::Dupe`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn at_least(self, major: u32, minor: u32) -> bool
fn new(major: u32, minor: u32, micro: u32) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: String) -> anyhow::Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> anyhow::Result<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## SysInfo

`struct` · `pyrefly_python::sys_info::SysInfo`

```rust
struct SysInfo
```

**Implements**: `dupe::Dupe`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
fn evaluate_bool(&self, x: &Expr) -> Option<bool>
fn evaluate_bool_with_sys_info(&self, x: &Expr) -> Option<bool>
fn is_not_type_checking_guard(x: &Expr) -> bool
fn is_type_checking_constant_name(x: &str) -> bool
fn is_type_checking_guard(x: &Expr) -> bool
fn new(version: PythonVersion, platform: PythonPlatform) -> Self
fn new_without_type_checking(version: PythonVersion, platform: PythonPlatform) -> Self
fn platform(&self) -> &PythonPlatform
fn pruned_if_branches<'a, 'b: 'a>(&'a self, x: &'b StmtIf) -> impl Iterator<Item = (Option<&'b Expr>, &'b [Stmt])> + 'a
fn type_checking(&self) -> bool
fn version(&self) -> PythonVersion
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Information available from the Python library `sys`, namely
`version` and `platform`.
Interned so that cloning is a trivial pointer copy (no atomic refcount).
There are very few distinct SysInfo values (typically 1 per run), so the
leaked memory from interning is negligible.

---
