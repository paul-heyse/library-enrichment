# `regex_automata::util::captures`

Crate `regex-automata` · 6 public items · structured records in [`model/regex_automata.util.captures.json`](../model/regex_automata.util.captures.json)

## Captures

`struct` · `regex_automata::util::captures::Captures`

```rust
struct Captures
```

**Derives**: Clone, Debug

**Methods** (21)

```rust
fn all(group_info: GroupInfo) -> Captures
fn clear(&mut self)
fn empty(group_info: GroupInfo) -> Captures
fn extract<'h, const N: usize>(&self, haystack: &'h str) -> (&'h str, [&'h str; N])
fn extract_bytes<'h, const N: usize>(&self, haystack: &'h [u8]) -> (&'h [u8], [&'h [u8]; N])
fn get_group(&self, index: usize) -> Option<Span>
fn get_group_by_name(&self, name: &str) -> Option<Span>
fn get_match(&self) -> Option<Match>
fn group_info(&self) -> &GroupInfo
fn group_len(&self) -> usize
fn interpolate_bytes(&self, haystack: &[u8], replacement: &[u8]) -> Vec<u8>
fn interpolate_bytes_into(&self, haystack: &[u8], replacement: &[u8], dst: &mut Vec<u8>)
fn interpolate_string(&self, haystack: &str, replacement: &str) -> String
fn interpolate_string_into(&self, haystack: &str, replacement: &str, dst: &mut String)
fn is_match(&self) -> bool
fn iter(&self) -> CapturesPatternIter<'_>
fn matches(group_info: GroupInfo) -> Captures
fn pattern(&self) -> Option<PatternID>
fn set_pattern(&mut self, pid: Option<PatternID>)
fn slots(&self) -> &[Option<NonMaxUsize>]
fn slots_mut(&mut self) -> &mut [Option<NonMaxUsize>]
```

The span offsets of capturing groups after a match has been found.

This type represents the output of regex engines that can report the
offsets at which capturing groups matches or "submatches" occur. For
example, the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). When a match
occurs, it will at minimum contain the [`PatternID`] of the pattern that
matched. Depending upon how it was constructed, it may also contain the
start/end offsets of the entire match of the pattern and the start/end
offsets of each capturing group that participated in the match.

Values of this type are always created for a specific [`GroupInfo`]. It is
unspecified behavior to use a `Captures` value in a search with any regex
engine that has a different `GroupInfo` than the one the `Captures` were
created with.

# Constructors

There are three constructors for this type that control what kind of
information is available upon a match:

* [`Captures::all`]: Will store overall pattern match offsets in addition
to the offsets of capturing groups that participated in the match.
* [`Captures::matches`]: Will store only the overall pattern
match offsets. The offsets of capturing groups (even ones that participated
in the match) are not available.
* [`Captures::empty`]: Will only store the pattern ID that matched. No
match offsets are available at all.

If you aren't sure which to choose, then pick the first one. The first one
is what convenience routines like,
[`PikeVM::create_captures`](crate::nfa::thompson::pikevm::PikeVM::create_captures),
will use automatically.

The main difference between these choices is performance. Namely, if you
ask for _less_ information, then the execution of regex search may be able
to run more quickly.

# Notes

It is worth pointing out that this type is not coupled to any one specific
regex engine. Instead, its coupling is with [`GroupInfo`], which is the
thing that is responsible for mapping capturing groups to "slot" offsets.
Slot offsets are indices into a single sequence of memory at which matching
haystack offsets for the corresponding group are written by regex engines.

# Example

This example shows how to parse a simple date and extract the components of
the date via capturing groups:

```
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group(1));
assert_eq!(Some(Span::from(5..7)), caps.get_group(2));
assert_eq!(Some(Span::from(8..10)), caps.get_group(3));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: named capturing groups

This example is like the one above, but leverages the ability to name
capturing groups in order to make the code a bit clearer:

```
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^(?P<y>[0-9]{4})-(?P<m>[0-9]{2})-(?P<d>[0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group_by_name("y"));
assert_eq!(Some(Span::from(5..7)), caps.get_group_by_name("m"));
assert_eq!(Some(Span::from(8..10)), caps.get_group_by_name("d"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## CapturesPatternIter

`struct` · `regex_automata::util::captures::CapturesPatternIter`

```rust
struct CapturesPatternIter<'a>
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Option<Span>>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over all capturing groups in a `Captures` value.

This iterator includes capturing groups that did not participate in a
match. See the [`Captures::iter`] method documentation for more details
and examples.

The lifetime parameter `'a` refers to the lifetime of the underlying
`Captures` value.

---

## GroupInfo

`struct` · `regex_automata::util::captures::GroupInfo`

```rust
struct GroupInfo
```

**Derives**: Clone, Debug, Default

**Methods** (15)

```rust
fn all_group_len(&self) -> usize
fn all_names(&self) -> GroupInfoAllNames<'_>
fn empty() -> GroupInfo
fn explicit_slot_len(&self) -> usize
fn group_len(&self, pid: PatternID) -> usize
fn implicit_slot_len(&self) -> usize
fn memory_usage(&self) -> usize
fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError> where P: IntoIterator<Item = G>, G: IntoIterator<Item = Option<N>>, N: AsRef<str>
fn pattern_len(&self) -> usize
fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_>
fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize>
fn slot_len(&self) -> usize
fn slots(&self, pid: PatternID, group_index: usize) -> Option<(usize, usize)>
fn to_index(&self, pid: PatternID, name: &str) -> Option<usize>
fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str>
```

Represents information about capturing groups in a compiled regex.

The information encapsulated by this type consists of the following. For
each pattern:

* A map from every capture group name to its corresponding capture group
index.
* A map from every capture group index to its corresponding capture group
name.
* A map from capture group index to its corresponding slot index. A slot
refers to one half of a capturing group. That is, a capture slot is either
the start or end of a capturing group. A slot is usually the mechanism
by which a regex engine records offsets for each capturing group during a
search.

A `GroupInfo` uses reference counting internally and is thus cheap to
clone.

# Mapping from capture groups to slots

One of the main responsibilities of a `GroupInfo` is to build a mapping
from `(PatternID, u32)` (where the `u32` is a capture index) to something
called a "slot." As mentioned above, a slot refers to one half of a
capturing group. Both combined provide the start and end offsets of
a capturing group that participated in a match.

**The mapping between group indices and slots is an API guarantee.** That
is, the mapping won't change within a semver compatible release.

Slots exist primarily because this is a convenient mechanism by which
regex engines report group offsets at search time. For example, the
[`nfa::thompson::State::Capture`](crate::nfa::thompson::State::Capture)
NFA state includes the slot index. When a regex engine transitions through
this state, it will likely use the slot index to write the current haystack
offset to some region of memory. When a match is found, those slots are
then reported to the caller, typically via a convenient abstraction like a
[`Captures`] value.

Because this crate provides first class support for multi-pattern regexes,
and because of some performance related reasons, the mapping between
capturing groups and slots is a little complex. However, in the case of a
single pattern, the mapping can be described very simply: for all capture
group indices `i`, its corresponding slots are at `i * 2` and `i * 2 + 1`.
Notice that the pattern ID isn't involved at all here, because it only
applies to a single-pattern regex, it is therefore always `0`.

In the multi-pattern case, the mapping is a bit more complicated. To talk
about it, we must define what we mean by "implicit" vs "explicit"
capturing groups:

* An **implicit** capturing group refers to the capturing group that is
present for every pattern automatically, and corresponds to the overall
match of a pattern. Every pattern has precisely one implicit capturing
group. It is always unnamed and it always corresponds to the capture group
index `0`.
* An **explicit** capturing group refers to any capturing group that
appears in the concrete syntax of the pattern. (Or, if an NFA was hand
built without any concrete syntax, it refers to any capturing group with an
index greater than `0`.)

Some examples:

* `\w+` has one implicit capturing group and zero explicit capturing
groups.
* `(\w+)` has one implicit group and one explicit group.
* `foo(\d+)(?:\pL+)(\d+)` has one implicit group and two explicit groups.

Turning back to the slot mapping, we can now state it as follows:

* Given a pattern ID `pid`, the slots for its implicit group are always
at `pid * 2` and `pid * 2 + 1`.
* Given a pattern ID `0`, the slots for its explicit groups start
at `group_info.pattern_len() * 2`.
* Given a pattern ID `pid > 0`, the slots for its explicit groups start
immediately following where the slots for the explicit groups of `pid - 1`
end.

In particular, while there is a concrete formula one can use to determine
where the slots for the implicit group of any pattern are, there is no
general formula for determining where the slots for explicit capturing
groups are. This is because each pattern can contain a different number
of groups.

The intended way of getting the slots for a particular capturing group
(whether implicit or explicit) is via the [`GroupInfo::slot`] or
[`GroupInfo::slots`] method.

See below for a concrete example of how capturing groups get mapped to
slots.

# Example

This example shows how to build a new `GroupInfo` and query it for
information.

```
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;
// The number of patterns being tracked.
assert_eq!(4, info.pattern_len());
// We can query the number of groups for any pattern.
assert_eq!(2, info.group_len(PatternID::must(0)));
assert_eq!(1, info.group_len(PatternID::must(1)));
assert_eq!(5, info.group_len(PatternID::must(2)));
assert_eq!(3, info.group_len(PatternID::must(3)));
// An invalid pattern always has zero groups.
assert_eq!(0, info.group_len(PatternID::must(999)));
// 2 slots per group
assert_eq!(22, info.slot_len());

// We can map a group index for a particular pattern to its name, if
// one exists.
assert_eq!(Some("foo"), info.to_name(PatternID::must(3), 2));
assert_eq!(None, info.to_name(PatternID::must(2), 4));
// Or map a name to its group index.
assert_eq!(Some(1), info.to_index(PatternID::must(0), "foo"));
assert_eq!(Some(2), info.to_index(PatternID::must(3), "foo"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: mapping from capture groups to slots

This example shows the specific mapping from capture group indices for
each pattern to their corresponding slots. The slot values shown in this
example are considered an API guarantee.

```
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;

// We first show the slots for each pattern's implicit group.
assert_eq!(Some((0, 1)), info.slots(PatternID::must(0), 0));
assert_eq!(Some((2, 3)), info.slots(PatternID::must(1), 0));
assert_eq!(Some((4, 5)), info.slots(PatternID::must(2), 0));
assert_eq!(Some((6, 7)), info.slots(PatternID::must(3), 0));

// And now we show the slots for each pattern's explicit group.
assert_eq!(Some((8, 9)), info.slots(PatternID::must(0), 1));
assert_eq!(Some((10, 11)), info.slots(PatternID::must(2), 1));
assert_eq!(Some((12, 13)), info.slots(PatternID::must(2), 2));
assert_eq!(Some((14, 15)), info.slots(PatternID::must(2), 3));
assert_eq!(Some((16, 17)), info.slots(PatternID::must(2), 4));
assert_eq!(Some((18, 19)), info.slots(PatternID::must(3), 1));
assert_eq!(Some((20, 21)), info.slots(PatternID::must(3), 2));

// Asking for the slots for an invalid pattern ID or even for an invalid
// group index for a specific pattern will return None. So for example,
// you're guaranteed to not get the slots for a different pattern than the
// one requested.
assert_eq!(None, info.slots(PatternID::must(5), 0));
assert_eq!(None, info.slots(PatternID::must(1), 1));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## GroupInfoAllNames

`struct` · `regex_automata::util::captures::GroupInfoAllNames`

```rust
struct GroupInfoAllNames<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<(PatternID, usize, Option<&'a str>)>
```

An iterator over capturing groups and their names for a `GroupInfo`.

This iterator is created by [`GroupInfo::all_names`].

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

---

## GroupInfoError

`struct` · `regex_automata::util::captures::GroupInfoError`

```rust
struct GroupInfoError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that may occur when building a `GroupInfo`.

Building a `GroupInfo` does a variety of checks to make sure the
capturing groups satisfy a number of invariants. This includes, but is not
limited to, ensuring that the first capturing group is unnamed and that
there are no duplicate capture groups for a specific pattern.

---

## GroupInfoPatternNames

`struct` · `regex_automata::util::captures::GroupInfoPatternNames`

```rust
struct GroupInfoPatternNames<'a>
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Option<&'a str>>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over capturing groups and their names for a specific pattern.

This iterator is created by [`GroupInfo::pattern_names`].

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

---
