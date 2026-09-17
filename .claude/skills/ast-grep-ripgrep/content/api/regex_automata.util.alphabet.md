# `regex_automata::util::alphabet`

Crate `regex-automata` · 5 public items · structured records in [`model/regex_automata.util.alphabet.json`](../model/regex_automata.util.alphabet.json)

## ByteClassElements

`struct` · `regex_automata::util::alphabet::ByteClassElements`

```rust
struct ByteClassElements<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Unit>
```

An iterator over all elements in an equivalence class.

This is created by the [`ByteClasses::elements`] method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

---

## ByteClassIter

`struct` · `regex_automata::util::alphabet::ByteClassIter`

```rust
struct ByteClassIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Unit>
```

An iterator over each equivalence class.

The last element in this iterator always corresponds to [`Unit::eoi`].

This is created by the [`ByteClasses::iter`] method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

---

## ByteClassRepresentatives

`struct` · `regex_automata::util::alphabet::ByteClassRepresentatives`

```rust
struct ByteClassRepresentatives<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Unit>
```

An iterator over representative bytes from each equivalence class.

This is created by the [`ByteClasses::representatives`] method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

---

## ByteClasses

`struct` · `regex_automata::util::alphabet::ByteClasses`

```rust
struct ByteClasses
```

**Derives**: Clone, Copy, Debug, Default

**Methods** (12)

```rust
fn alphabet_len(&self) -> usize
fn elements(&self, class: Unit) -> ByteClassElements<'_>
fn empty() -> ByteClasses
fn eoi(&self) -> Unit
fn get(&self, byte: u8) -> u8
fn get_by_unit(&self, unit: Unit) -> usize
fn is_singleton(&self) -> bool
fn iter(&self) -> ByteClassIter<'_>
fn representatives<R: core::ops::RangeBounds<u8>>(&self, range: R) -> ByteClassRepresentatives<'_>
fn set(&mut self, byte: u8, class: u8)
fn singletons() -> ByteClasses
fn stride2(&self) -> usize
```

A representation of byte oriented equivalence classes.

This is used in a DFA to reduce the size of the transition table. This can
have a particularly large impact not only on the total size of a dense DFA,
but also on compile times.

The essential idea here is that the alphabet of a DFA is shrunk from the
usual 256 distinct byte values down to a set of equivalence classes. The
guarantee you get is that any byte belonging to the same equivalence class
can be treated as if it were any other byte in the same class, and the
result of a search wouldn't change.

# Example

This example shows how to get byte classes from an
[`NFA`](crate::nfa::thompson::NFA) and ask for the class of various bytes.

```
use regex_automata::nfa::thompson::NFA;

let nfa = NFA::new("[a-z]+")?;
let classes = nfa.byte_classes();
// 'a' and 'z' are in the same class for this regex.
assert_eq!(classes.get(b'a'), classes.get(b'z'));
// But 'a' and 'A' are not.
assert_ne!(classes.get(b'a'), classes.get(b'A'));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Unit

`struct` · `regex_automata::util::alphabet::Unit`

```rust
struct Unit
```

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn as_eoi(self) -> Option<u16>
fn as_u8(self) -> Option<u8>
fn as_usize(self) -> usize
fn eoi(num_byte_equiv_classes: usize) -> Unit
fn is_byte(self, byte: u8) -> bool
fn is_eoi(self) -> bool
fn is_word_byte(self) -> bool
fn u8(byte: u8) -> Unit
```

Unit represents a single unit of haystack for DFA based regex engines.

It is not expected for consumers of this crate to need to use this type
unless they are implementing their own DFA. And even then, it's not
required: implementors may use other techniques to handle haystack units.

Typically, a single unit of haystack for a DFA would be a single byte.
However, for the DFAs in this crate, matches are delayed by a single byte
in order to handle look-ahead assertions (`\b`, `$` and `\z`). Thus, once
we have consumed the haystack, we must run the DFA through one additional
transition using a unit that indicates the haystack has ended.

There is no way to represent a sentinel with a `u8` since all possible
values *may* be valid haystack units to a DFA, therefore this type
explicitly adds room for a sentinel value.

The sentinel EOI value is always its own equivalence class and is
ultimately represented by adding 1 to the maximum equivalence class value.
So for example, the regex `^[a-z]+$` might be split into the following
equivalence classes:

```text
0 => [\x00-`]
1 => [a-z]
2 => [{-\xFF]
3 => [EOI]
```

Where EOI is the special sentinel value that is always in its own
singleton equivalence class.

---
