# `regex`

Crate `regex` · 2 public items · structured records in [`model/regex.json`](../model/regex.json)

## escape

`function` · `regex::escape`

```rust
fn escape(pattern: &str) -> alloc::string::String
```

Escapes all regular expression meta characters in `pattern`.

The string returned may be safely used as a literal in a regular
expression.

---

## regex

`macro` · `regex::regex`

```rust
macro_rules! regex
```

A convenient way to construct regex patterns from string literals.

This macro can be used to construct reusable instances of [`Regex`] with
reduced boilerplate. The constructed `Regex` is stored in a static so the
pattern is compiled approximately once, even when called multiple times.

There is *no compile-time checking of patterns* with `regex!`. Instead,
invalid patterns will panic the first time the regex is used. Invalid
patterns should still not be used with `regex!`; if compile-time checking
becomes feasible in the future, it may be added within a non-semver-breaking
release. In the meantime, consider enabling [`clippy::invalid_regex`].

# Examples

```
use regex::{Regex, regex};

assert!(regex!("[a-z]").is_match("a"));
assert!(regex!("(inconceivable!|classic blunder)").is_match("inconceivable!"));

let re: &Regex = regex!(r"(\d{3})-(\d{4})");
assert_eq!(&re.captures("867-5309").unwrap()[1], "867");
```

An invalid pattern will panic when it is first used:

```should_panic
use regex::regex;

let re = regex!("invalid -> ("); // no panic here
re.is_match("invalid -> (");     // panic!
```

[`clippy::invalid_regex`]: https://rust-lang.github.io/rust-clippy/master/#invalid_regex

---
