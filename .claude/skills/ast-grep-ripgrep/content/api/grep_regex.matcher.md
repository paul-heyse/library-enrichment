# `grep_regex::matcher`

Crate `grep-regex` · 3 public items · structured records in [`model/grep_regex.matcher.json`](../model/grep_regex.matcher.json)

## RegexCaptures

`struct` · `grep_regex::matcher::RegexCaptures`

Also reachable as `grep_regex::RegexCaptures`

```rust
struct RegexCaptures
```

**Implements**: `grep_matcher::Captures`

**Derives**: Clone, Debug

**via `grep_matcher::Captures`**

```rust
fn get(&self, i: usize) -> Option<Match>
fn len(&self) -> usize
```

Represents the match offsets of each capturing group in a match.

The first, or `0`th capture group, always corresponds to the entire match
and is guaranteed to be present when a match occurs. The next capture
group, at index `1`, corresponds to the first capturing group in the regex,
ordered by the position at which the left opening parenthesis occurs.

Note that not all capturing groups are guaranteed to be present in a match.
For example, in the regex, `(?P<foo>\w)|(?P<bar>\W)`, only one of `foo`
or `bar` will ever be set in any given match.

In order to access a capture group by name, you'll need to first find the
index of the group using the corresponding matcher's `capture_index`
method, and then use that index with `RegexCaptures::get`.

---

## RegexMatcher

`struct` · `grep_regex::matcher::RegexMatcher`

Also reachable as `grep_regex::RegexMatcher`

```rust
struct RegexMatcher
```

**Implements**: `grep_matcher::Matcher`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(pattern: &str) -> Result<RegexMatcher, Error>
fn new_line_matcher(pattern: &str) -> Result<RegexMatcher, Error>
```

**via `grep_matcher::Matcher`**

```rust
fn capture_count(&self) -> usize
fn capture_index(&self, name: &str) -> Option<usize>
fn captures_at(&self, haystack: &[u8], at: usize, caps: &mut RegexCaptures) -> Result<bool, NoError>
fn find_at(&self, haystack: &[u8], at: usize) -> Result<Option<Match>, NoError>
fn find_candidate_line(&self, haystack: &[u8]) -> Result<Option<LineMatchKind>, NoError>
fn line_terminator(&self) -> Option<LineTerminator>
fn new_captures(&self) -> Result<RegexCaptures, NoError>
fn non_matching_bytes(&self) -> Option<&ByteSet>
fn shortest_match_at(&self, haystack: &[u8], at: usize) -> Result<Option<usize>, NoError>
fn try_find_iter<F, E>(&self, haystack: &[u8], matched: F) -> Result<Result<(), E>, NoError> where F: FnMut(Match) -> Result<bool, E>
```

An implementation of the `Matcher` trait using Rust's standard regex
library.

---

## RegexMatcherBuilder

`struct` · `grep_regex::matcher::RegexMatcherBuilder`

Also reachable as `grep_regex::RegexMatcherBuilder`

```rust
struct RegexMatcherBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (21)

```rust
fn ban_byte(&mut self, byte: Option<u8>) -> &mut RegexMatcherBuilder
fn build(&self, pattern: &str) -> Result<RegexMatcher, Error>
fn build_literals<B: AsRef<str>>(&self, literals: &[B]) -> Result<RegexMatcher, Error>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<RegexMatcher, Error>
fn case_insensitive(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn case_smart(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn crlf(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexMatcherBuilder
fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn fixed_strings(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn line_terminator(&mut self, line_term: Option<u8>) -> &mut RegexMatcherBuilder
fn multi_line(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn nest_limit(&mut self, limit: u32) -> &mut RegexMatcherBuilder
fn new() -> RegexMatcherBuilder
fn octal(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn size_limit(&mut self, bytes: usize) -> &mut RegexMatcherBuilder
fn swap_greed(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn unicode(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn whole_line(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn word(&mut self, yes: bool) -> &mut RegexMatcherBuilder
```

A builder for constructing a `Matcher` using regular expressions.

This builder re-exports many of the same options found on the regex crate's
builder, in addition to a few other options such as smart case, word
matching and the ability to set a line terminator which may enable certain
types of optimizations.

The syntax supported is documented as part of the regex crate:
<https://docs.rs/regex/#syntax>.

---
