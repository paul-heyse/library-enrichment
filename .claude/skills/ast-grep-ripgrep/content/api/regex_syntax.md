# `regex_syntax`

Crate `regex-syntax` · 7 public items · structured records in [`model/regex_syntax.json`](../model/regex_syntax.json)

## escape

`function` · `regex_syntax::escape`

```rust
fn escape(text: &str) -> alloc::string::String
```

Escapes all regular expression meta characters in `text`.

The string returned may be safely used as a literal in a regular
expression.

---

## escape_into

`function` · `regex_syntax::escape_into`

```rust
fn escape_into(text: &str, buf: &mut alloc::string::String)
```

Escapes all meta characters in `text` and writes the result into `buf`.

This will append escape characters into the given buffer. The characters
that are appended are safe to use as a literal in a regular expression.

---

## is_escapeable_character

`function` · `regex_syntax::is_escapeable_character`

```rust
fn is_escapeable_character(c: char) -> bool
```

Returns true if the given character can be escaped in a regex.

This returns true in all cases that `is_meta_character` returns true, but
also returns true in some cases where `is_meta_character` returns false.
For example, `%` is not a meta character, but it is escapable. That is,
`%` and `\%` both match a literal `%` in all contexts.

The purpose of this routine is to provide knowledge about what characters
may be escaped. Namely, most regex engines permit "superfluous" escapes
where characters without any special significance may be escaped even
though there is no actual _need_ to do so.

This will return false for some characters. For example, `e` is not
escapable. Therefore, `\e` will either result in a parse error (which is
true today), or it could backwards compatibly evolve into a new construct
with its own meaning. Indeed, that is the purpose of banning _some_
superfluous escapes: it provides a way to evolve the syntax in a compatible
manner.

# Example

```
use regex_syntax::is_escapeable_character;

assert!(is_escapeable_character('?'));
assert!(is_escapeable_character('-'));
assert!(is_escapeable_character('&'));
assert!(is_escapeable_character('#'));
assert!(is_escapeable_character('%'));
assert!(is_escapeable_character('/'));
assert!(is_escapeable_character('!'));
assert!(is_escapeable_character('"'));

assert!(!is_escapeable_character('e'));
```

---

## is_meta_character

`function` · `regex_syntax::is_meta_character`

```rust
fn is_meta_character(c: char) -> bool
```

Returns true if the given character has significance in a regex.

Generally speaking, these are the only characters which _must_ be escaped
in order to match their literal meaning. For example, to match a literal
`|`, one could write `\|`. Sometimes escaping isn't always necessary. For
example, `-` is treated as a meta character because of its significance
for writing ranges inside of character classes, but the regex `-` will
match a literal `-` because `-` has no special meaning outside of character
classes.

In order to determine whether a character may be escaped at all, the
[`is_escapeable_character`] routine should be used. The difference between
`is_meta_character` and `is_escapeable_character` is that the latter will
return true for some characters that are _not_ meta characters. For
example, `%` and `\%` both match a literal `%` in all contexts. In other
words, `is_escapeable_character` includes "superfluous" escapes.

Note that the set of characters for which this function returns `true` or
`false` is fixed and won't change in a semver compatible release. (In this
case, "semver compatible release" actually refers to the `regex` crate
itself, since reducing or expanding the set of meta characters would be a
breaking change for not just `regex-syntax` but also `regex` itself.)

# Example

```
use regex_syntax::is_meta_character;

assert!(is_meta_character('?'));
assert!(is_meta_character('-'));
assert!(is_meta_character('&'));
assert!(is_meta_character('#'));

assert!(!is_meta_character('%'));
assert!(!is_meta_character('/'));
assert!(!is_meta_character('!'));
assert!(!is_meta_character('"'));
assert!(!is_meta_character('e'));
```

---

## is_word_byte

`function` · `regex_syntax::is_word_byte`

```rust
fn is_word_byte(c: u8) -> bool
```

Returns true if and only if the given character is an ASCII word character.

An ASCII word character is defined by the following character class:
`[_0-9a-zA-Z]`.

---

## is_word_character

`function` · `regex_syntax::is_word_character`

```rust
fn is_word_character(c: char) -> bool
```

Returns true if and only if the given character is a Unicode word
character.

A Unicode word character is defined by
[UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
In particular, a character
is considered a word character if it is in either of the `Alphabetic` or
`Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
or `Connector_Punctuation` general categories.

# Panics

If the `unicode-perl` feature is not enabled, then this function
panics. For this reason, it is recommended that callers use
[`try_is_word_character`] instead.

---

## try_is_word_character

`function` · `regex_syntax::try_is_word_character`

```rust
fn try_is_word_character(c: char) -> core::result::Result<bool, UnicodeWordError>
```

Returns true if and only if the given character is a Unicode word
character.

A Unicode word character is defined by
[UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
In particular, a character
is considered a word character if it is in either of the `Alphabetic` or
`Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
or `Connector_Punctuation` general categories.

# Errors

If the `unicode-perl` feature is not enabled, then this function always
returns an error.

---
