# `arrow_schema::ffi`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.ffi.json`](../model/arrow_schema.ffi.json)

## FFI_ArrowSchema

`struct` · `arrow_schema::ffi::FFI_ArrowSchema`

Also reachable as `arrow::ffi::FFI_ArrowSchema`, `arrow_array::ffi::FFI_ArrowSchema`

```rust
struct FFI_ArrowSchema
```

**Fields**: `format`, `name`, `metadata`, `flags`, `n_children`, `children`, `dictionary`, `release`, `private_data`

**Implements**: `core::convert::TryFrom`, `core::ops::drop::Drop`

**Derives**: Debug, Send

**Methods** (16)

```rust
fn child(&self, index: usize) -> &Self
fn children(&self) -> impl Iterator<Item = &Self>
fn dictionary(&self) -> Option<&Self>
fn dictionary_ordered(&self) -> bool
fn empty() -> Self
fn flags(&self) -> Option<Flags>
fn format(&self) -> &str
unsafe fn from_raw(schema: *mut FFI_ArrowSchema) -> Self
fn map_keys_sorted(&self) -> bool
fn metadata(&self) -> Result<HashMap<String, String>, ArrowError>
fn name(&self) -> Option<&str>
fn nullable(&self) -> bool
fn try_new(format: &str, children: Vec<FFI_ArrowSchema>, dictionary: Option<FFI_ArrowSchema>) -> Result<Self, ArrowError>
fn with_flags(self, flags: Flags) -> Result<Self, ArrowError>
fn with_metadata<I, S>(self, metadata: I) -> Result<Self, ArrowError> where I: IntoIterator<Item = (S, S)>, S: AsRef<str>
fn with_name(self, name: &str) -> Result<Self, ArrowError>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(schema: &Schema) -> Result<Self, ArrowError>
fn try_from(dtype: &DataType) -> Result<Self, ArrowError>
fn try_from(value: &FieldRef) -> Result<Self, Self::Error>
fn try_from(schema: Schema) -> Result<Self, ArrowError>
fn try_from(dtype: DataType) -> Result<Self, ArrowError>
fn try_from(field: &Field) -> Result<Self, ArrowError>
fn try_from(field: Field) -> Result<Self, ArrowError>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.ffi.FFI_ArrowSchema.md).


ABI-compatible struct for `ArrowSchema` from C Data Interface
See <https://arrow.apache.org/docs/format/CDataInterface.html#the-arrowschema-structure>

```
# use arrow_schema::DataType;
# use arrow_schema::ffi::FFI_ArrowSchema;
fn array_schema(data_type: &DataType) -> FFI_ArrowSchema {
    FFI_ArrowSchema::try_from(data_type).unwrap()
}
```

---

## Flags

`struct` · `arrow_schema::ffi::Flags`

```rust
struct Flags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Methods** (22)

```rust
const fn all() -> Self
const fn bits(&self) -> i64
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: i64) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: i64) -> Self
const fn from_bits_truncate(bits: i64) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<Flags>
const fn iter_names(&self) -> iter::IterNames<Flags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> Flags
fn bits(&self) -> i64
fn from_bits_retain(bits: i64) -> Flags
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: Flags) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.ffi.Flags.md).


Flags for [`FFI_ArrowSchema`]

Old Workaround at <https://github.com/bitflags/bitflags/issues/356>
is no longer required as `bitflags` [fixed the issue](https://github.com/bitflags/bitflags/pull/355).

---
