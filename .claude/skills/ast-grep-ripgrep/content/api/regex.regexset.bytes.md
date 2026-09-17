# `regex::regexset::bytes`

Crate `regex` · 4 public items · structured records in [`model/regex.regexset.bytes.json`](../model/regex.regexset.bytes.json)

## RegexSet

`struct` · `regex::regexset::bytes::RegexSet`

```rust
struct RegexSet
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn empty() -> RegexSet
fn is_empty(&self) -> bool
fn is_match(&self, haystack: &[u8]) -> bool
fn is_match_at(&self, haystack: &[u8], start: usize) -> bool
fn len(&self) -> usize
fn matches(&self, haystack: &[u8]) -> SetMatches
fn matches_at(&self, haystack: &[u8], start: usize) -> SetMatches
fn new<I, S>(exprs: I) -> Result<RegexSet, Error> where S: AsRef<str>, I: IntoIterator<Item = S>
fn patterns(&self) -> &[String]
```

Match multiple, possibly overlapping, regexes in a single search.

A regex set corresponds to the union of zero or more regular expressions.
That is, a regex set will match a haystack when at least one of its
constituent regexes matches. A regex set as its formulated here provides a
touch more power: it will also report *which* regular expressions in the
set match. Indeed, this is the key difference between regex sets and a
single `Regex` with many alternates, since only one alternate can match at
a time.

For example, consider regular expressions to match email addresses and
domains: `[a-z]+@[a-z]+\.(com|org|net)` and `[a-z]+\.(com|org|net)`. If a
regex set is constructed from those regexes, then searching the haystack
`foo@example.com` will report both regexes as matching. Of course, one
could accomplish this by compiling each regex on its own and doing two
searches over the haystack. The key advantage of using a regex set is
that it will report the matching regexes using a *single pass through the
haystack*. If one has hundreds or thousands of regexes to match repeatedly
(like a URL router for a complex web application or a user agent matcher),
then a regex set *can* realize huge performance gains.

Unlike the top-level [`RegexSet`](crate::RegexSet), this `RegexSet`
searches haystacks with type `&[u8]` instead of `&str`. Consequently, this
`RegexSet` is permitted to match invalid UTF-8.

# Limitations

Regex sets are limited to answering the following two questions:

1. Does any regex in the set match?
2. If so, which regexes in the set match?

As with the main [`Regex`][crate::bytes::Regex] type, it is cheaper to ask
(1) instead of (2) since the matching engines can stop after the first
match is found.

You cannot directly extract [`Match`][crate::bytes::Match] or
[`Captures`][crate::bytes::Captures] objects from a regex set. If you need
these operations, the recommended approach is to compile each pattern in
the set independently and scan the exact same haystack a second time with
those independently compiled patterns:

```
use regex::bytes::{Regex, RegexSet};

let patterns = ["foo", "bar"];
// Both patterns will match different ranges of this string.
let hay = b"barfoo";

// Compile a set matching any of our patterns.
let set = RegexSet::new(patterns).unwrap();
// Compile each pattern independently.
let regexes: Vec<_> = set
    .patterns()
    .iter()
    .map(|pat| Regex::new(pat).unwrap())
    .collect();

// Match against the whole set first and identify the individual
// matching patterns.
let matches: Vec<&[u8]> = set
    .matches(hay)
    .into_iter()
    // Dereference the match index to get the corresponding
    // compiled pattern.
    .map(|index| &regexes[index])
    // To get match locations or any other info, we then have to search the
    // exact same haystack again, using our separately-compiled pattern.
    .map(|re| re.find(hay).unwrap().as_bytes())
    .collect();

// Matches arrive in the order the constituent patterns were declared,
// not the order they appear in the haystack.
assert_eq!(vec![&b"foo"[..], &b"bar"[..]], matches);
```

# Performance

A `RegexSet` has the same performance characteristics as `Regex`. Namely,
search takes `O(m * n)` time, where `m` is proportional to the size of the
regex set and `n` is proportional to the length of the haystack.

# Trait implementations

The `Default` trait is implemented for `RegexSet`. The default value
is an empty set. An empty set can also be explicitly constructed via
[`RegexSet::empty`].

# Example

This shows how the above two regexes (for matching email addresses and
domains) might work:

```
use regex::bytes::RegexSet;

let set = RegexSet::new(&[
    r"[a-z]+@[a-z]+\.(com|org|net)",
    r"[a-z]+\.(com|org|net)",
]).unwrap();

// Ask whether any regexes in the set match.
assert!(set.is_match(b"foo@example.com"));

// Identify which regexes in the set match.
let matches: Vec<_> = set.matches(b"foo@example.com").into_iter().collect();
assert_eq!(vec![0, 1], matches);

// Try again, but with a haystack that only matches one of the regexes.
let matches: Vec<_> = set.matches(b"example.com").into_iter().collect();
assert_eq!(vec![1], matches);

// Try again, but with a haystack that doesn't match any regex in the set.
let matches: Vec<_> = set.matches(b"example").into_iter().collect();
assert!(matches.is_empty());
```

Note that it would be possible to adapt the above example to using `Regex`
with an expression like:

```text
(?P<email>[a-z]+@(?P<email_domain>[a-z]+[.](com|org|net)))|(?P<domain>[a-z]+[.](com|org|net))
```

After a match, one could then inspect the capture groups to figure out
which alternates matched. The problem is that it is hard to make this
approach scale when there are many regexes since the overlap between each
alternate isn't always obvious to reason about.

---

## SetMatches

`struct` · `regex::regexset::bytes::SetMatches`

```rust
struct SetMatches
```

**Implements**: `core::iter::traits::collect::IntoIterator`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn iter(&self) -> SetMatchesIter<'_>
fn len(&self) -> usize
fn matched(&self, index: usize) -> bool
fn matched_all(&self) -> bool
fn matched_any(&self) -> bool
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

A set of matches returned by a regex set.

Values of this type are constructed by [`RegexSet::matches`].

---

## SetMatchesIntoIter

`struct` · `regex::regexset::bytes::SetMatchesIntoIter`

```rust
struct SetMatchesIntoIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<usize>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
fn size_hint(&self) -> (usize, Option<usize>)
```

An owned iterator over the set of matches from a regex set.

This will always produces matches in ascending order of index, where the
index corresponds to the index of the regex that matched with respect to
its position when initially building the set.

This iterator is created by calling `SetMatches::into_iter` via the
`IntoIterator` trait. This is automatically done in `for` loops.

# Example

```
use regex::bytes::RegexSet;

let set = RegexSet::new([
    r"[0-9]",
    r"[a-z]",
    r"[A-Z]",
    r"\p{Greek}",
]).unwrap();
let hay = "βa1".as_bytes();
let mut matches = vec![];
for index in set.matches(hay) {
    matches.push(index);
}
assert_eq!(matches, vec![0, 1, 3]);
```

---

## SetMatchesIter

`struct` · `regex::regexset::bytes::SetMatchesIter`

```rust
struct SetMatchesIter<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<usize>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
fn size_hint(&self) -> (usize, Option<usize>)
```

A borrowed iterator over the set of matches from a regex set.

The lifetime `'a` refers to the lifetime of the [`SetMatches`] value that
created this iterator.

This will always produces matches in ascending order, where the index
corresponds to the index of the regex that matched with respect to its
position when initially building the set.

This iterator is created by the [`SetMatches::iter`] method.

---
