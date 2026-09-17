# `regex_automata::util::interpolate`

Crate `regex-automata` · 2 public items · structured records in [`model/regex_automata.util.interpolate.json`](../model/regex_automata.util.interpolate.json)

## bytes

`function` · `regex_automata::util::interpolate::bytes`

```rust
fn bytes(replacement: &[u8], append: impl FnMut(usize, &mut alloc::vec::Vec<u8>), name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut alloc::vec::Vec<u8>)
```

Accepts a replacement byte string and interpolates capture references with
their corresponding values.

`append` should be a function that appends the byte string value of a
capture group at a particular index to the byte string given. If the
capture group index is invalid, then nothing should be appended.

`name_to_index` should be a function that maps a capture group name to a
capture group index. If the given name doesn't exist, then `None` should
be returned.

Finally, `dst` is where the final interpolated contents should be written.
If `replacement` contains no capture group references, then `dst` will be
equivalent to `replacement`.

See the [module documentation](self) for details about the format
supported.

# Example

```
use regex_automata::util::interpolate;

let mut dst = vec![];
interpolate::bytes(
    b"foo $bar baz",
    |index, dst| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    },
    |name| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    },
    &mut dst,
);
assert_eq!(&b"foo BAR baz"[..], dst);
```

---

## string

`function` · `regex_automata::util::interpolate::string`

```rust
fn string(replacement: &str, append: impl FnMut(usize, &mut alloc::string::String), name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut alloc::string::String)
```

Accepts a replacement string and interpolates capture references with their
corresponding values.

`append` should be a function that appends the string value of a capture
group at a particular index to the string given. If the capture group
index is invalid, then nothing should be appended.

`name_to_index` should be a function that maps a capture group name to a
capture group index. If the given name doesn't exist, then `None` should
be returned.

Finally, `dst` is where the final interpolated contents should be written.
If `replacement` contains no capture group references, then `dst` will be
equivalent to `replacement`.

See the [module documentation](self) for details about the format
supported.

# Example

```
use regex_automata::util::interpolate;

let mut dst = String::new();
interpolate::string(
    "foo $bar baz",
    |index, dst| {
        if index == 0 {
            dst.push_str("BAR");
        }
    },
    |name| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    },
    &mut dst,
);
assert_eq!("foo BAR baz", dst);
```

---
