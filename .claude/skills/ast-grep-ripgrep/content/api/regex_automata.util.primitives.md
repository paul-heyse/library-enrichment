# `regex_automata::util::primitives`

Crate `regex-automata` · 7 public items · structured records in [`model/regex_automata.util.primitives.json`](../model/regex_automata.util.primitives.json)

## NonMaxUsize

`struct` · `regex_automata::util::primitives::NonMaxUsize`

```rust
struct NonMaxUsize
```

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn get(self) -> usize
fn new(value: usize) -> Option<NonMaxUsize>
```

A `usize` that can never be `usize::MAX`.

This is similar to `core::num::NonZeroUsize`, but instead of not permitting
a zero value, this does not permit a max value.

This is useful in certain contexts where one wants to optimize the memory
usage of things that contain match offsets. Namely, since Rust slices
are guaranteed to never have a length exceeding `isize::MAX`, we can use
`usize::MAX` as a sentinel to indicate that no match was found. Indeed,
types like `Option<NonMaxUsize>` have exactly the same size in memory as a
`usize`.

This type is defined to be `repr(transparent)` for
`core::num::NonZeroUsize`, which is in turn defined to be
`repr(transparent)` for `usize`.

---

## PatternID

`struct` · `regex_automata::util::primitives::PatternID`

Also reachable as `regex_automata::PatternID`

```rust
struct PatternID
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
const fn as_i32(&self) -> i32
const fn as_u32(&self) -> u32
const fn as_u64(&self) -> u64
const fn as_usize(&self) -> usize
fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>
fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID
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
fn try_from(value: u16) -> Result<PatternID, PatternIDError>
fn try_from(value: u32) -> Result<PatternID, PatternIDError>
fn try_from(value: u64) -> Result<PatternID, PatternIDError>
fn try_from(value: usize) -> Result<PatternID, PatternIDError>
```

The identifier of a regex pattern, represented by a [`SmallIndex`].

The identifier for a pattern corresponds to its relative position among
other patterns in a single finite state machine. Namely, when building
a multi-pattern regex engine, one must supply a sequence of patterns to
match. The position (starting at 0) of each pattern in that sequence
represents its identifier. This identifier is in turn used to identify and
report matches of that pattern in various APIs.

See the [`SmallIndex`] type for more information about what it means for
a pattern ID to be a "small index."

Note that this type is defined in the
[`util::primitives`](crate::util::primitives) module, but it is also
re-exported at the crate root due to how common it is.

---

## PatternIDError

`struct` · `regex_automata::util::primitives::PatternIDError`

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

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

---

## SmallIndex

`struct` · `regex_automata::util::primitives::SmallIndex`

```rust
struct SmallIndex
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
const fn as_i32(&self) -> i32
const fn as_u32(&self) -> u32
const fn as_u64(&self) -> u64
const fn as_usize(&self) -> usize
fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError>
fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex
fn must(index: usize) -> SmallIndex
fn new(index: usize) -> Result<SmallIndex, SmallIndexError>
const fn new_unchecked(index: usize) -> SmallIndex
fn one_more(&self) -> usize
fn to_ne_bytes(&self) -> [u8; 4]
```

**via `core::convert::From`**

```rust
fn from(index: u8) -> SmallIndex
```

**via `core::convert::TryFrom`**

```rust
fn try_from(index: usize) -> Result<SmallIndex, SmallIndexError>
fn try_from(index: u16) -> Result<SmallIndex, SmallIndexError>
fn try_from(index: u32) -> Result<SmallIndex, SmallIndexError>
fn try_from(index: u64) -> Result<SmallIndex, SmallIndexError>
```

A type that represents a "small" index.

The main idea of this type is to provide something that can index memory,
but uses less memory than `usize` on 64-bit systems. Specifically, its
representation is always a `u32` and has `repr(transparent)` enabled. (So
it is safe to transmute between a `u32` and a `SmallIndex`.)

A small index is typically useful in cases where there is no practical way
that the index will overflow a 32-bit integer. A good example of this is
an NFA state. If you could somehow build an NFA with `2^30` states, its
memory usage would be exorbitant and its runtime execution would be so
slow as to be completely worthless. Therefore, this crate generally deems
it acceptable to return an error if it would otherwise build an NFA that
requires a slice longer than what a 32-bit integer can index. In exchange,
we can use 32-bit indices instead of 64-bit indices in various places.

This type ensures this by providing a constructor that will return an error
if its argument cannot fit into the type. This makes it much easier to
handle these sorts of boundary cases that are otherwise extremely subtle.

On all targets, this type guarantees that its value will fit in a `u32`,
`i32`, `usize` and an `isize`. This means that on 16-bit targets, for
example, this type's maximum value will never overflow an `isize`,
which means it will never overflow a `i16` even though its internal
representation is still a `u32`.

The purpose for making the type fit into even signed integer types like
`isize` is to guarantee that the difference between any two small indices
is itself also a small index. This is useful in certain contexts, e.g.,
for delta encoding.

# Other types

The following types wrap `SmallIndex` to provide a more focused use case:

* [`PatternID`] is for representing the identifiers of patterns.
* [`StateID`] is for representing the identifiers of states in finite
automata. It is used for both NFAs and DFAs.

# Representation

This type is always represented internally by a `u32` and is marked as
`repr(transparent)`. Thus, this type always has the same representation as
a `u32`. It is thus safe to transmute between a `u32` and a `SmallIndex`.

# Indexing

For convenience, callers may use a `SmallIndex` to index slices.

# Safety

While a `SmallIndex` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `SmallIndex` with
an invalid value can be done in entirely safe code. This may in turn result
in panics or silent logical errors.

---

## SmallIndexError

`struct` · `regex_automata::util::primitives::SmallIndexError`

```rust
struct SmallIndexError
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

This error occurs when a small index could not be constructed.

This occurs when given an integer exceeding the maximum small index value.

When the `std` feature is enabled, this implements the `Error` trait.

---

## StateID

`struct` · `regex_automata::util::primitives::StateID`

```rust
struct StateID
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
const fn as_i32(&self) -> i32
const fn as_u32(&self) -> u32
const fn as_u64(&self) -> u64
const fn as_usize(&self) -> usize
fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>
fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID
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

The identifier of a finite automaton state, represented by a
[`SmallIndex`].

Most regex engines in this crate are built on top of finite automata. Each
state in a finite automaton defines transitions from its state to another.
Those transitions point to other states via their identifiers, i.e., a
`StateID`. Since finite automata tend to contain many transitions, it is
much more memory efficient to define state IDs as small indices.

See the [`SmallIndex`] type for more information about what it means for
a state ID to be a "small index."

---

## StateIDError

`struct` · `regex_automata::util::primitives::StateIDError`

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

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

---
