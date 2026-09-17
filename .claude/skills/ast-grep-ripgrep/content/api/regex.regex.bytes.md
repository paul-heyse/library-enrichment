# `regex::regex::bytes`

Crate `regex` · 13 public items · structured records in [`model/regex.regex.bytes.json`](../model/regex.regex.bytes.json)

## CaptureLocations

`struct` · `regex::regex::bytes::CaptureLocations`

```rust
struct CaptureLocations
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn get(&self, i: usize) -> Option<(usize, usize)>
fn len(&self) -> usize
```

A low level representation of the byte offsets of each capture group.

You can think of this as a lower level [`Captures`], where this type does
not support named capturing groups directly and it does not borrow the
haystack that these offsets were matched on.

Primarily, this type is useful when using the lower level `Regex` APIs such
as [`Regex::captures_read`], which permits amortizing the allocation in
which capture match offsets are stored.

In order to build a value of this type, you'll need to call the
[`Regex::capture_locations`] method. The value returned can then be reused
in subsequent searches for that regex. Using it for other regexes may
result in a panic or otherwise incorrect results.

# Example

This example shows how to create and use `CaptureLocations` in a search.

```
use regex::bytes::Regex;

let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
let mut locs = re.capture_locations();
let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
assert_eq!(0..17, m.range());
assert_eq!(Some((0, 17)), locs.get(0));
assert_eq!(Some((0, 5)), locs.get(1));
assert_eq!(Some((6, 17)), locs.get(2));

// Asking for an invalid capture group always returns None.
assert_eq!(None, locs.get(3));
# // literals are too big for 32-bit usize: #1041
# #[cfg(target_pointer_width = "64")]
assert_eq!(None, locs.get(34973498648));
# #[cfg(target_pointer_width = "64")]
assert_eq!(None, locs.get(9944060567225171988));
```

---

## CaptureMatches

`struct` · `regex::regex::bytes::CaptureMatches`

```rust
struct CaptureMatches<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Captures<'h>>
```

An iterator over all non-overlapping capture matches in a haystack.

This iterator yields [`Captures`] values. The iterator stops when no more
matches can be found.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the matched string.

This iterator is created by [`Regex::captures_iter`].

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

---

## CaptureNames

`struct` · `regex::regex::bytes::CaptureNames`

```rust
struct CaptureNames<'r>
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Option<&'r str>>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over the names of all capture groups in a regex.

This iterator yields values of type `Option<&str>` in order of the opening
capture group parenthesis in the regex pattern. `None` is yielded for
groups with no name. The first element always corresponds to the implicit
and unnamed group for the overall match.

`'r` is the lifetime of the compiled regular expression.

This iterator is created by [`Regex::capture_names`].

---

## Captures

`struct` · `regex::regex::bytes::Captures`

```rust
struct Captures<'h>
```

**Implements**: `core::ops::index::Index`

**Derives**: Debug

**Methods** (7)

```rust
fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>)
fn extract<const N: usize>(&self) -> (&'h [u8], [&'h [u8]; N])
fn get(&self, i: usize) -> Option<Match<'h>>
fn get_match(&self) -> Match<'h>
fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h>
fn len(&self) -> usize
fn name(&self, name: &str) -> Option<Match<'h>>
```

**via `core::ops::index::Index`**

```rust
fn index<'a>(&'a self, name: &'n str) -> &'a [u8]
fn index<'a>(&'a self, i: usize) -> &'a [u8]
```

Represents the capture groups for a single match.

Capture groups refer to parts of a regex enclosed in parentheses. They
can be optionally named. The purpose of capture groups is to be able to
reference different parts of a match based on the original pattern. In
essence, a `Captures` is a container of [`Match`] values for each group
that participated in a regex match. Each `Match` can be looked up by either
its capture group index or name (if it has one).

For example, say you want to match the individual letters in a 5-letter
word:

```text
(?<first>\w)(\w)(?:\w)\w(?<last>\w)
```

This regex has 4 capture groups:

* The group at index `0` corresponds to the overall match. It is always
present in every match and never has a name.
* The group at index `1` with name `first` corresponding to the first
letter.
* The group at index `2` with no name corresponding to the second letter.
* The group at index `3` with name `last` corresponding to the fifth and
last letter.

Notice that `(?:\w)` was not listed above as a capture group despite it
being enclosed in parentheses. That's because `(?:pattern)` is a special
syntax that permits grouping but *without* capturing. The reason for not
treating it as a capture is that tracking and reporting capture groups
requires additional state that may lead to slower searches. So using as few
capture groups as possible can help performance. (Although the difference
in performance of a couple of capture groups is likely immaterial.)

Values with this type are created by [`Regex::captures`] or
[`Regex::captures_iter`].

`'h` is the lifetime of the haystack that these captures were matched from.

# Example

```
use regex::bytes::Regex;

let re = Regex::new(r"(?<first>\w)(\w)(?:\w)\w(?<last>\w)").unwrap();
let caps = re.captures(b"toady").unwrap();
assert_eq!(b"toady", &caps[0]);
assert_eq!(b"t", &caps["first"]);
assert_eq!(b"o", &caps[2]);
assert_eq!(b"y", &caps["last"]);
```

---

## Match

`struct` · `regex::regex::bytes::Match`

```rust
struct Match<'h>
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_bytes(&self) -> &'h [u8]
fn end(&self) -> usize
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn range(&self) -> core::ops::Range<usize>
fn start(&self) -> usize
```

Represents a single match of a regex in a haystack.

A `Match` contains both the start and end byte offsets of the match and the
actual substring corresponding to the range of those byte offsets. It is
guaranteed that `start <= end`. When `start == end`, the match is empty.

Unlike the top-level `Match` type, this `Match` type is produced by APIs
that search `&[u8]` haystacks. This means that the offsets in a `Match` can
point to anywhere in the haystack, including in a place that splits the
UTF-8 encoding of a Unicode scalar value.

The lifetime parameter `'h` refers to the lifetime of the matched of the
haystack that this match was produced from.

# Numbering

The byte offsets in a `Match` form a half-open interval. That is, the
start of the range is inclusive and the end of the range is exclusive.
For example, given a haystack `abcFOOxyz` and a match of `FOO`, its byte
offset range starts at `3` and ends at `6`. `3` corresponds to `F` and
`6` corresponds to `x`, which is one past the end of the match. This
corresponds to the same kind of slicing that Rust uses.

For more on why this was chosen over other schemes (aside from being
consistent with how Rust the language works), see [this discussion] and
[Dijkstra's note on a related topic][note].

[this discussion]: https://github.com/rust-lang/regex/discussions/866
[note]: https://www.cs.utexas.edu/users/EWD/transcriptions/EWD08xx/EWD831.html

# Example

This example shows the value of each of the methods on `Match` for a
particular search.

```
use regex::bytes::Regex;

let re = Regex::new(r"\p{Greek}+").unwrap();
let hay = "Greek: αβγδ".as_bytes();
let m = re.find(hay).unwrap();
assert_eq!(7, m.start());
assert_eq!(15, m.end());
assert!(!m.is_empty());
assert_eq!(8, m.len());
assert_eq!(7..15, m.range());
assert_eq!("αβγδ".as_bytes(), m.as_bytes());
```

---

## Matches

`struct` · `regex::regex::bytes::Matches`

```rust
struct Matches<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Match<'h>>
```

An iterator over all non-overlapping matches in a haystack.

This iterator yields [`Match`] values. The iterator stops when no more
matches can be found.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the haystack.

This iterator is created by [`Regex::find_iter`].

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

---

## NoExpand

`struct` · `regex::regex::bytes::NoExpand`

```rust
struct NoExpand<'s>
```

**Implements**: `regex::regex::bytes::Replacer`

**Derives**: Clone, Debug

**via `regex::regex::bytes::Replacer`**

```rust
fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>>
fn replace_append(&mut self, _: &Captures<'_>, dst: &mut Vec<u8>)
```

A helper type for forcing literal string replacement.

It can be used with routines like [`Regex::replace`] and
[`Regex::replace_all`] to do a literal string replacement without expanding
`$name` to their corresponding capture groups. This can be both convenient
(to avoid escaping `$`, for example) and faster (since capture groups
don't need to be found).

`'s` is the lifetime of the literal string to use.

# Example

```
use regex::bytes::{NoExpand, Regex};

let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();
let result = re.replace(b"Springsteen, Bruce", NoExpand(b"$2 $last"));
assert_eq!(result, &b"$2 $last"[..]);
```

---

## Regex

`struct` · `regex::regex::bytes::Regex`

```rust
struct Regex
```

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Debug

**Methods** (23)

```rust
fn as_str(&self) -> &str
fn capture_locations(&self) -> CaptureLocations
fn capture_names(&self) -> CaptureNames<'_>
fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>>
fn captures_at<'h>(&self, haystack: &'h [u8], start: usize) -> Option<Captures<'h>>
fn captures_iter<'r, 'h>(&'r self, haystack: &'h [u8]) -> CaptureMatches<'r, 'h>
fn captures_len(&self) -> usize
fn captures_read<'h>(&self, locs: &mut CaptureLocations, haystack: &'h [u8]) -> Option<Match<'h>>
fn captures_read_at<'h>(&self, locs: &mut CaptureLocations, haystack: &'h [u8], start: usize) -> Option<Match<'h>>
fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>>
fn find_at<'h>(&self, haystack: &'h [u8], start: usize) -> Option<Match<'h>>
fn find_iter<'r, 'h>(&'r self, haystack: &'h [u8]) -> Matches<'r, 'h>
fn is_match(&self, haystack: &[u8]) -> bool
fn is_match_at(&self, haystack: &[u8], start: usize) -> bool
fn new(re: &str) -> Result<Regex, Error>
fn replace<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>
fn replace_all<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>
fn replacen<'h, R: Replacer>(&self, haystack: &'h [u8], limit: usize, rep: R) -> Cow<'h, [u8]>
fn shortest_match(&self, haystack: &[u8]) -> Option<usize>
fn shortest_match_at(&self, haystack: &[u8], start: usize) -> Option<usize>
fn split<'r, 'h>(&'r self, haystack: &'h [u8]) -> Split<'r, 'h>
fn splitn<'r, 'h>(&'r self, haystack: &'h [u8], limit: usize) -> SplitN<'r, 'h>
fn static_captures_len(&self) -> Option<usize>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(s: String) -> Result<Regex, Error>
fn try_from(s: &str) -> Result<Regex, Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Regex, Error>
```

A compiled regular expression for searching Unicode haystacks.

A `Regex` can be used to search haystacks, split haystacks into substrings
or replace substrings in a haystack with a different substring. All
searching is done with an implicit `(?s:.)*?` at the beginning and end of
an pattern. To force an expression to match the whole string (or a prefix
or a suffix), you must use an anchor like `^` or `$` (or `\A` and `\z`).

Like the `Regex` type in the parent module, matches with this regex return
byte offsets into the haystack. **Unlike** the parent `Regex` type, these
byte offsets may not correspond to UTF-8 sequence boundaries since the
regexes in this module can match arbitrary bytes.

The only methods that allocate new byte strings are the string replacement
methods. All other methods (searching and splitting) return borrowed
references into the haystack given.

# Example

Find the offsets of a US phone number:

```
use regex::bytes::Regex;

let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
let m = re.find(b"phone: 111-222-3333").unwrap();
assert_eq!(7..19, m.range());
```

# Example: extracting capture groups

A common way to use regexes is with capture groups. That is, instead of
just looking for matches of an entire regex, parentheses are used to create
groups that represent part of the match.

For example, consider a haystack with multiple lines, and each line has
three whitespace delimited fields where the second field is expected to be
a number and the third field a boolean. To make this convenient, we use
the [`Captures::extract`] API to put the strings that match each group
into a fixed size array:

```
use regex::bytes::Regex;

let hay = b"
rabbit         54 true
groundhog 2 true
does not match
fox   109    false
";
let re = Regex::new(r"(?m)^\s*(\S+)\s+([0-9]+)\s+(true|false)\s*$").unwrap();
let mut fields: Vec<(&[u8], i64, bool)> = vec![];
for (_, [f1, f2, f3]) in re.captures_iter(hay).map(|caps| caps.extract()) {
    // These unwraps are OK because our pattern is written in a way where
    // all matches for f2 and f3 will be valid UTF-8.
    let f2 = std::str::from_utf8(f2).unwrap();
    let f3 = std::str::from_utf8(f3).unwrap();
    fields.push((f1, f2.parse()?, f3.parse()?));
}
assert_eq!(fields, vec![
    (&b"rabbit"[..], 54, true),
    (&b"groundhog"[..], 2, true),
    (&b"fox"[..], 109, false),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: matching invalid UTF-8

One of the reasons for searching `&[u8]` haystacks is that the `&[u8]`
might not be valid UTF-8. Indeed, with a `bytes::Regex`, patterns that
match invalid UTF-8 are explicitly allowed. Here's one example that looks
for valid UTF-8 fields that might be separated by invalid UTF-8. In this
case, we use `(?s-u:.)`, which matches any byte. Attempting to use it in a
top-level `Regex` will result in the regex failing to compile. Notice also
that we use `.` with Unicode mode enabled, in which case, only valid UTF-8
is matched. In this way, we can build one pattern where some parts only
match valid UTF-8 while other parts are more permissive.

```
use regex::bytes::Regex;

// F0 9F 92 A9 is the UTF-8 encoding for a Pile of Poo.
let hay = b"\xFF\xFFfoo\xFF\xFF\xFF\xF0\x9F\x92\xA9\xFF";
// An equivalent to '(?s-u:.)' is '(?-u:[\x00-\xFF])'.
let re = Regex::new(r"(?s)(?-u:.)*?(?<f1>.+)(?-u:.)*?(?<f2>.+)").unwrap();
let caps = re.captures(hay).unwrap();
assert_eq!(&caps["f1"], &b"foo"[..]);
assert_eq!(&caps["f2"], "💩".as_bytes());
```

---

## ReplacerRef

`struct` · `regex::regex::bytes::ReplacerRef`

```rust
struct ReplacerRef<'a, R: ?Sized>
```

**Implements**: `regex::regex::bytes::Replacer`

**Derives**: Debug

**via `regex::regex::bytes::Replacer`**

```rust
fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>>
fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>)
```

A by-reference adaptor for a [`Replacer`].

This permits reusing the same `Replacer` value in multiple calls to a
replacement routine like [`Regex::replace_all`].

This type is created by [`Replacer::by_ref`].

---

## Split

`struct` · `regex::regex::bytes::Split`

```rust
struct Split<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h [u8]>
```

An iterator over all substrings delimited by a regex match.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the byte string being split.

This iterator is created by [`Regex::split`].

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

---

## SplitN

`struct` · `regex::regex::bytes::SplitN`

```rust
struct SplitN<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h [u8]>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over at most `N` substrings delimited by a regex match.

The last substring yielded by this iterator will be whatever remains after
`N-1` splits.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the byte string being split.

This iterator is created by [`Regex::splitn`].

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

Although note that the worst case time here has an upper bound given
by the `limit` parameter to [`Regex::splitn`].

---

## SubCaptureMatches

`struct` · `regex::regex::bytes::SubCaptureMatches`

```rust
struct SubCaptureMatches<'c, 'h>
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Option<Match<'h>>>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over all group matches in a [`Captures`] value.

This iterator yields values of type `Option<Match<'h>>`, where `'h` is the
lifetime of the haystack that the matches are for. The order of elements
yielded corresponds to the order of the opening parenthesis for the group
in the regex pattern. `None` is yielded for groups that did not participate
in the match.

The first element always corresponds to the implicit group for the overall
match. Since this iterator is created by a [`Captures`] value, and a
`Captures` value is only created when a match occurs, it follows that the
first element yielded by this iterator is guaranteed to be non-`None`.

The lifetime `'c` corresponds to the lifetime of the `Captures` value that
created this iterator, and the lifetime `'h` corresponds to the originally
matched haystack.

---

## Replacer

`trait` · `regex::regex::bytes::Replacer`

```rust
trait Replacer
```

**Implementors** (4)

- `alloc::borrow::Cow`
- `alloc::vec::Vec`
- `regex::regex::bytes::NoExpand`
- `regex::regex::bytes::ReplacerRef`

**Methods** (3)

```rust
fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self>
fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>>
fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>)
```

A trait for types that can be used to replace matches in a haystack.

In general, users of this crate shouldn't need to implement this trait,
since implementations are already provided for `&[u8]` along with other
variants of byte string types, as well as `FnMut(&Captures) -> Vec<u8>` (or
any `FnMut(&Captures) -> T` where `T: AsRef<[u8]>`). Those cover most use
cases, but callers can implement this trait directly if necessary.

# Example

This example shows a basic implementation of the `Replacer` trait. This can
be done much more simply using the replacement byte string interpolation
support (e.g., `$first $last`), but this approach avoids needing to parse
the replacement byte string at all.

```
use regex::bytes::{Captures, Regex, Replacer};

struct NameSwapper;

impl Replacer for NameSwapper {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        dst.extend_from_slice(&caps["first"]);
        dst.extend_from_slice(b" ");
        dst.extend_from_slice(&caps["last"]);
    }
}

let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();
let result = re.replace(b"Springsteen, Bruce", NameSwapper);
assert_eq!(result, &b"Bruce Springsteen"[..]);
```

---
