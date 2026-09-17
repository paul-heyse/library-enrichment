# `grep_pcre2::matcher`

Crate `grep-pcre2` · 3 public items · structured records in [`model/grep_pcre2.matcher.json`](../model/grep_pcre2.matcher.json)

## RegexCaptures

`struct` · `grep_pcre2::matcher::RegexCaptures`

Also reachable as `grep_pcre2::RegexCaptures`

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

`struct` · `grep_pcre2::matcher::RegexMatcher`

Also reachable as `grep_pcre2::RegexMatcher`

```rust
struct RegexMatcher
```

**Implements**: `grep_matcher::Matcher`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(pattern: &str) -> Result<RegexMatcher, Error>
```

**via `grep_matcher::Matcher`**

```rust
fn capture_count(&self) -> usize
fn capture_index(&self, name: &str) -> Option<usize>
fn captures_at(&self, haystack: &[u8], at: usize, caps: &mut RegexCaptures) -> Result<bool, Error>
fn find_at(&self, haystack: &[u8], at: usize) -> Result<Option<Match>, Error>
fn new_captures(&self) -> Result<RegexCaptures, Error>
fn try_find_iter<F, E>(&self, haystack: &[u8], matched: F) -> Result<Result<(), E>, Error> where F: FnMut(Match) -> Result<bool, E>
```

An implementation of the `Matcher` trait using PCRE2.

---

## RegexMatcherBuilder

`struct` · `grep_pcre2::matcher::RegexMatcherBuilder`

Also reachable as `grep_pcre2::RegexMatcherBuilder`

```rust
struct RegexMatcherBuilder
```

**Derives**: Clone, Debug

**Methods** (18)

```rust
fn build(&self, pattern: &str) -> Result<RegexMatcher, Error>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<RegexMatcher, Error>
fn case_smart(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn caseless(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn crlf(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn disable_utf_check(&mut self) -> &mut RegexMatcherBuilder
fn dotall(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn extended(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn fixed_strings(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn jit(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn jit_if_available(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn max_jit_stack_size(&mut self, bytes: Option<usize>) -> &mut RegexMatcherBuilder
fn multi_line(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn new() -> RegexMatcherBuilder
fn ucp(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn utf(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn whole_line(&mut self, yes: bool) -> &mut RegexMatcherBuilder
fn word(&mut self, yes: bool) -> &mut RegexMatcherBuilder
```

A builder for configuring the compilation of a PCRE2 regex.

---
