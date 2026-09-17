# `pcre2::bytes`

Crate `pcre2` · 7 public items · structured records in [`model/pcre2.bytes.json`](../model/pcre2.bytes.json)

## CaptureLocations

`struct` · `pcre2::bytes::CaptureLocations`

```rust
struct CaptureLocations
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn get(&self, i: usize) -> Option<(usize, usize)>
fn len(&self) -> usize
```

CaptureLocations is a low level representation of the raw offsets of each
submatch.

Primarily, this type is useful when using `Regex` APIs such as
`captures_read`, which permits amortizing the allocation in which capture
match locations are stored.

In order to build a value of this type, you'll need to call the
`capture_locations` method on the `Regex` being used to execute the search.
The value returned can then be reused in subsequent searches.

---

## CaptureMatches

`struct` · `pcre2::bytes::CaptureMatches`

```rust
struct CaptureMatches<'r, 's>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Captures<'s>, Error>>
```

An iterator that yields all non-overlapping capture groups matching a
particular regular expression.

The iterator stops when no more matches can be found.

`'r` is the lifetime of the compiled regular expression and `'s` is the
lifetime of the subject string.

---

## Captures

`struct` · `pcre2::bytes::Captures`

```rust
struct Captures<'s>
```

**Implements**: `core::ops::index::Index`

**Derives**: Debug

**Methods** (3)

```rust
fn get(&self, i: usize) -> Option<Match<'s>>
fn len(&self) -> usize
fn name(&self, name: &str) -> Option<Match<'s>>
```

**via `core::ops::index::Index`**

```rust
fn index<'a>(&'a self, name: &'i str) -> &'a [u8]
fn index(&self, i: usize) -> &[u8]
```

Captures represents a group of captured byte strings for a single match.

The 0th capture always corresponds to the entire match. Each subsequent
index corresponds to the next capture group in the regex. If a capture
group is named, then the matched byte string is *also* available via the
`name` method. (Note that the 0th capture is always unnamed and so must be
accessed with the `get` method.)

Positions returned from a capture group are always byte indices.

`'s` is the lifetime of the matched subject string.

---

## Match

`struct` · `pcre2::bytes::Match`

```rust
struct Match<'s>
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_bytes(&self) -> &'s [u8]
fn end(&self) -> usize
fn start(&self) -> usize
```

Match represents a single match of a regex in a subject string.

The lifetime parameter `'s` refers to the lifetime of the matched portion
of the subject string.

---

## Matches

`struct` · `pcre2::bytes::Matches`

```rust
struct Matches<'r, 's>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Match<'s>, Error>>
```

An iterator over all non-overlapping matches for a particular subject
string.

The iterator yields matches (if no error occurred while searching)
corresponding to the start and end of the match. The indices are byte
offsets. The iterator stops when no more matches can be found.

`'r` is the lifetime of the compiled regular expression and `'s` is the
lifetime of the subject string.

---

## Regex

`struct` · `pcre2::bytes::Regex`

```rust
struct Regex
```

**Derives**: Clone, Debug

**Methods** (14)

```rust
fn as_str(&self) -> &str
fn capture_locations(&self) -> CaptureLocations
fn capture_names(&self) -> &[Option<String>]
fn captures<'s>(&self, subject: &'s [u8]) -> Result<Option<Captures<'s>>, Error>
fn captures_iter<'r, 's>(&'r self, subject: &'s [u8]) -> CaptureMatches<'r, 's>
fn captures_len(&self) -> usize
fn captures_read<'s>(&self, locs: &mut CaptureLocations, subject: &'s [u8]) -> Result<Option<Match<'s>>, Error>
fn captures_read_at<'s>(&self, locs: &mut CaptureLocations, subject: &'s [u8], start: usize) -> Result<Option<Match<'s>>, Error>
fn find<'s>(&self, subject: &'s [u8]) -> Result<Option<Match<'s>>, Error>
fn find_at<'s>(&self, subject: &'s [u8], start: usize) -> Result<Option<Match<'s>>, Error>
fn find_iter<'r, 's>(&'r self, subject: &'s [u8]) -> Matches<'r, 's>
fn is_match(&self, subject: &[u8]) -> Result<bool, Error>
fn is_match_at(&self, subject: &[u8], start: usize) -> Result<bool, Error>
fn new(pattern: &str) -> Result<Regex, Error>
```

A compiled PCRE2 regular expression.

This regex is safe to use from multiple threads simultaneously. For top
performance, it is better to clone a new regex for each thread.

---

## RegexBuilder

`struct` · `pcre2::bytes::RegexBuilder`

```rust
struct RegexBuilder
```

**Derives**: Clone, Debug

**Methods** (13)

```rust
fn build(&self, pattern: &str) -> Result<Regex, Error>
fn caseless(&mut self, yes: bool) -> &mut RegexBuilder
fn crlf(&mut self, yes: bool) -> &mut RegexBuilder
fn disable_utf_check(&mut self) -> &mut RegexBuilder
fn dotall(&mut self, yes: bool) -> &mut RegexBuilder
fn extended(&mut self, yes: bool) -> &mut RegexBuilder
fn jit(&mut self, yes: bool) -> &mut RegexBuilder
fn jit_if_available(&mut self, yes: bool) -> &mut RegexBuilder
fn max_jit_stack_size(&mut self, bytes: Option<usize>) -> &mut RegexBuilder
fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder
fn new() -> RegexBuilder
fn ucp(&mut self, yes: bool) -> &mut RegexBuilder
fn utf(&mut self, yes: bool) -> &mut RegexBuilder
```

A builder for configuring the compilation of a PCRE2 regex.

---
