# `regex::regex::string`

Crate `regex` · 13 public items · structured records in [`model/regex.regex.string.json`](../model/regex.regex.string.json)

## CaptureLocations

`struct` · `regex::regex::string::CaptureLocations`

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
use regex::Regex;

let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
let mut locs = re.capture_locations();
let m = re.captures_read(&mut locs, "Bruce Springsteen").unwrap();
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

`struct` · `regex::regex::string::CaptureMatches`

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

`struct` · `regex::regex::string::CaptureNames`

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

`struct` · `regex::regex::string::Captures`

```rust
struct Captures<'h>
```

**Implements**: `core::ops::index::Index`

**Derives**: Debug

**Methods** (7)

```rust
fn expand(&self, replacement: &str, dst: &mut String)
fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N])
fn get(&self, i: usize) -> Option<Match<'h>>
fn get_match(&self) -> Match<'h>
fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h>
fn len(&self) -> usize
fn name(&self, name: &str) -> Option<Match<'h>>
```

**via `core::ops::index::Index`**

```rust
fn index<'a>(&'a self, name: &'n str) -> &'a str
fn index<'a>(&'a self, i: usize) -> &'a str
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
use regex::Regex;

let re = Regex::new(r"(?<first>\w)(\w)(?:\w)\w(?<last>\w)").unwrap();
let caps = re.captures("toady").unwrap();
assert_eq!("toady", &caps[0]);
assert_eq!("t", &caps["first"]);
assert_eq!("o", &caps[2]);
assert_eq!("y", &caps["last"]);
```

---

## Match

`struct` · `regex::regex::string::Match`

```rust
struct Match<'h>
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_str(&self) -> &'h str
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

Since this `Match` can only be produced by the top-level `Regex` APIs
that only support searching UTF-8 encoded strings, the byte offsets for a
`Match` are guaranteed to fall on valid UTF-8 codepoint boundaries. That
is, slicing a `&str` with [`Match::range`] is guaranteed to never panic.

Values with this type are created by [`Regex::find`] or
[`Regex::find_iter`]. Other APIs can create `Match` values too. For
example, [`Captures::get`].

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
use regex::Regex;

let re = Regex::new(r"\p{Greek}+").unwrap();
let hay = "Greek: αβγδ";
let m = re.find(hay).unwrap();
assert_eq!(7, m.start());
assert_eq!(15, m.end());
assert!(!m.is_empty());
assert_eq!(8, m.len());
assert_eq!(7..15, m.range());
assert_eq!("αβγδ", m.as_str());
```

---

## Matches

`struct` · `regex::regex::string::Matches`

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

`struct` · `regex::regex::string::NoExpand`

```rust
struct NoExpand<'s>
```

**Implements**: `regex::regex::string::Replacer`

**Derives**: Clone, Debug

**via `regex::regex::string::Replacer`**

```rust
fn no_expansion(&mut self) -> Option<Cow<'_, str>>
fn replace_append(&mut self, _: &Captures<'_>, dst: &mut String)
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
use regex::{NoExpand, Regex};

let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();
let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
assert_eq!(result, "$2 $last");
```

---

## Regex

`struct` · `regex::regex::string::Regex`

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
fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>>
fn captures_at<'h>(&self, haystack: &'h str, start: usize) -> Option<Captures<'h>>
fn captures_iter<'r, 'h>(&'r self, haystack: &'h str) -> CaptureMatches<'r, 'h>
fn captures_len(&self) -> usize
fn captures_read<'h>(&self, locs: &mut CaptureLocations, haystack: &'h str) -> Option<Match<'h>>
fn captures_read_at<'h>(&self, locs: &mut CaptureLocations, haystack: &'h str, start: usize) -> Option<Match<'h>>
fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>>
fn find_at<'h>(&self, haystack: &'h str, start: usize) -> Option<Match<'h>>
fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h>
fn is_match(&self, haystack: &str) -> bool
fn is_match_at(&self, haystack: &str, start: usize) -> bool
fn new(re: &str) -> Result<Regex, Error>
fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str>
fn replace_all<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str>
fn replacen<'h, R: Replacer>(&self, haystack: &'h str, limit: usize, rep: R) -> Cow<'h, str>
fn shortest_match(&self, haystack: &str) -> Option<usize>
fn shortest_match_at(&self, haystack: &str, start: usize) -> Option<usize>
fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h>
fn splitn<'r, 'h>(&'r self, haystack: &'h str, limit: usize) -> SplitN<'r, 'h>
fn static_captures_len(&self) -> Option<usize>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> Result<Regex, Error>
fn try_from(s: String) -> Result<Regex, Error>
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

While this crate will handle Unicode strings (whether in the regular
expression or in the haystack), all positions returned are **byte
offsets**. Every byte offset is guaranteed to be at a Unicode code point
boundary. That is, all offsets returned by the `Regex` API are guaranteed
to be ranges that can slice a `&str` without panicking. If you want to
relax this requirement, then you must search `&[u8]` haystacks with a
[`bytes::Regex`](crate::bytes::Regex).

The only methods that allocate new strings are the string replacement
methods. All other methods (searching and splitting) return borrowed
references into the haystack given.

# Example

Find the offsets of a US phone number:

```
use regex::Regex;

let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
let m = re.find("phone: 111-222-3333").unwrap();
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
use regex::Regex;

let hay = "
rabbit         54 true
groundhog 2 true
does not match
fox   109    false
";
let re = Regex::new(r"(?m)^\s*(\S+)\s+([0-9]+)\s+(true|false)\s*$").unwrap();
let mut fields: Vec<(&str, i64, bool)> = vec![];
for (_, [f1, f2, f3]) in re.captures_iter(hay).map(|caps| caps.extract()) {
    fields.push((f1, f2.parse()?, f3.parse()?));
}
assert_eq!(fields, vec![
    ("rabbit", 54, true),
    ("groundhog", 2, true),
    ("fox", 109, false),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: searching with the `Pattern` trait

**Note**: This section requires that this crate is compiled with the
`pattern` Cargo feature enabled, which **requires nightly Rust**.

Since `Regex` implements `Pattern` from the standard library, one can
use regexes with methods defined on `&str`. For example, `is_match`,
`find`, `find_iter` and `split` can, in some cases, be replaced with
`str::contains`, `str::find`, `str::match_indices` and `str::split`.

Here are some examples:

```ignore
use regex::Regex;

let re = Regex::new(r"\d+").unwrap();
let hay = "a111b222c";

assert!(hay.contains(&re));
assert_eq!(hay.find(&re), Some(1));
assert_eq!(hay.match_indices(&re).collect::<Vec<_>>(), vec![
    (1, "111"),
    (5, "222"),
]);
assert_eq!(hay.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);
```

---

## ReplacerRef

`struct` · `regex::regex::string::ReplacerRef`

```rust
struct ReplacerRef<'a, R: ?Sized>
```

**Implements**: `regex::regex::string::Replacer`

**Derives**: Debug

**via `regex::regex::string::Replacer`**

```rust
fn no_expansion(&mut self) -> Option<Cow<'_, str>>
fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String)
```

A by-reference adaptor for a [`Replacer`].

This permits reusing the same `Replacer` value in multiple calls to a
replacement routine like [`Regex::replace_all`].

This type is created by [`Replacer::by_ref`].

---

## Split

`struct` · `regex::regex::string::Split`

```rust
struct Split<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h str>
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

`struct` · `regex::regex::string::SplitN`

```rust
struct SplitN<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h str>
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

`struct` · `regex::regex::string::SubCaptureMatches`

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

`trait` · `regex::regex::string::Replacer`

```rust
trait Replacer
```

**Implementors** (4)

- `alloc::borrow::Cow`
- `alloc::string::String`
- `regex::regex::string::NoExpand`
- `regex::regex::string::ReplacerRef`

**Methods** (3)

```rust
fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self>
fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>>
fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String)
```

A trait for types that can be used to replace matches in a haystack.

In general, users of this crate shouldn't need to implement this trait,
since implementations are already provided for `&str` along with other
variants of string types, as well as `FnMut(&Captures) -> String` (or any
`FnMut(&Captures) -> T` where `T: AsRef<str>`). Those cover most use cases,
but callers can implement this trait directly if necessary.

# Example

This example shows a basic implementation of  the `Replacer` trait. This
can be done much more simply using the replacement string interpolation
support (e.g., `$first $last`), but this approach avoids needing to parse
the replacement string at all.

```
use regex::{Captures, Regex, Replacer};

struct NameSwapper;

impl Replacer for NameSwapper {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        dst.push_str(&caps["first"]);
        dst.push_str(" ");
        dst.push_str(&caps["last"]);
    }
}

let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();
let result = re.replace("Springsteen, Bruce", NameSwapper);
assert_eq!(result, "Bruce Springsteen");
```

---
