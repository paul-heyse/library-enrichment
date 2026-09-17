# `aho_corasick::util::primitives`

Crate `aho-corasick` · 4 public items · structured records in [`model/aho_corasick.util.primitives.json`](../model/aho_corasick.util.primitives.json)

## PatternID

`struct` · `aho_corasick::util::primitives::PatternID`

Also reachable as `aho_corasick::PatternID`

```rust
struct PatternID
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
const fn as_i32(&self) -> i32
const fn as_u32(&self) -> u32
const fn as_u64(&self) -> u64
const fn as_usize(&self) -> usize
fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>
fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID
const fn from_u32_unchecked(index: u32) -> PatternID
fn must(value: usize) -> PatternID
fn new(value: usize) -> Result<PatternID, PatternIDError>
const fn new_unchecked(value: usize) -> PatternID
fn one_more(&self) -> usize
fn to_ne_bytes(&self) -> [u8; 4]
```

**via `core::convert::From`**

```rust
fn from(value: u8) -> PatternID
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u64) -> Result<PatternID, PatternIDError>
fn try_from(value: usize) -> Result<PatternID, PatternIDError>
fn try_from(value: u16) -> Result<PatternID, PatternIDError>
fn try_from(value: u32) -> Result<PatternID, PatternIDError>
```

The identifier of a pattern in an Aho-Corasick automaton.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `PatternID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

---

## PatternIDError

`struct` · `aho_corasick::util::primitives::PatternIDError`

Also reachable as `aho_corasick::PatternIDError`

```rust
struct PatternIDError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn attempted(&self) -> u64
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

---

## StateID

`struct` · `aho_corasick::util::primitives::StateID`

Also reachable as `aho_corasick::automaton::StateID`

```rust
struct StateID
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
const fn as_i32(&self) -> i32
const fn as_u32(&self) -> u32
const fn as_u64(&self) -> u64
const fn as_usize(&self) -> usize
fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>
fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID
const fn from_u32_unchecked(index: u32) -> StateID
fn must(value: usize) -> StateID
fn new(value: usize) -> Result<StateID, StateIDError>
const fn new_unchecked(value: usize) -> StateID
fn one_more(&self) -> usize
fn to_ne_bytes(&self) -> [u8; 4]
```

**via `core::convert::From`**

```rust
fn from(value: u8) -> StateID
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> Result<StateID, StateIDError>
fn try_from(value: u32) -> Result<StateID, StateIDError>
fn try_from(value: u64) -> Result<StateID, StateIDError>
fn try_from(value: usize) -> Result<StateID, StateIDError>
```

The identifier of a finite automaton state.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `StateID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

---

## StateIDError

`struct` · `aho_corasick::util::primitives::StateIDError`

Also reachable as `aho_corasick::automaton::StateIDError`

```rust
struct StateIDError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn attempted(&self) -> u64
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

---
