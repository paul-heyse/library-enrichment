# `arrow_cast::display::ArrayFormatter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.ArrayFormatter.json).

<a id="op-c2a994be3b7cca3db9a55992"></a>
## ArrayFormatter

`struct` · `arrow_cast::display::ArrayFormatter` · arrow-cast 59.3.0

```rust
struct ArrayFormatter<'a>
```

Source: `src/display.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A string formatter for an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

This can be used with [`std::write`] to write type-erased `dyn Array`

```
# use std::fmt::{Display, Formatter, Write};
# use arrow_array::{Array, ArrayRef, Int32Array};
# use arrow_cast::display::{ArrayFormatter, FormatOptions};
# use arrow_schema::ArrowError;
struct MyContainer {
    values: ArrayRef,
}

impl Display for MyContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let options = FormatOptions::default();
        let formatter = ArrayFormatter::try_new(self.values.as_ref(), &options)
            .map_err(|_| std::fmt::Error)?;

        let mut iter = 0..self.values.len();
        if let Some(idx) = iter.next() {
            write!(f, "{}", formatter.value(idx))?;
        }
        for idx in iter {
            write!(f, ", {}", formatter.value(idx))?;
        }
        Ok(())
    }
}
```

[`ValueFormatter::write`](../operations/arrow_cast.display.ValueFormatter.md#op-670b0503c94f323521d1c48c) can also be used to get a semantic error, instead of the
opaque [`std::fmt::Error`]

```
# use std::fmt::Write;
# use arrow_array::Array;
# use arrow_cast::display::{ArrayFormatter, FormatOptions};
# use arrow_schema::ArrowError;
fn format_array(
    f: &mut dyn Write,
    array: &dyn Array,
    options: &FormatOptions,
) -> Result<(), ArrowError> {
    let formatter = ArrayFormatter::try_new(array, options)?;
    for i in 0..array.len() {
        formatter.value(i).write(f)?
    }
    Ok(())
}
```


Unresolved upstream links (retained, not inferred): ``std::fmt::Error``, ``std::write``.

<a id="op-4e8a9734ff686328c6a5aefd"></a>
## new

`function` · `arrow_cast::display::ArrayFormatter::new` · arrow-cast 59.3.0

```rust
fn new(format: Box<dyn DisplayIndex + 'a>, safe: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::ArrayFormatter", "path": "ArrayFormatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 1], "end": [529, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:507`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns an [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992) using the provided formatter.

<a id="op-be20a7f02dffb528e47f0716"></a>
## try_new

`function` · `arrow_cast::display::ArrayFormatter::try_new` · arrow-cast 59.3.0

```rust
fn try_new(array: &'a dyn Array, options: &FormatOptions<'a>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::ArrayFormatter", "path": "ArrayFormatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 1], "end": [529, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:514`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns an [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992) that can be used to format `array`

This returns an error if an array of the given data type cannot be formatted

<a id="op-d4c679eadcfed8a0752a1122"></a>
## value

`function` · `arrow_cast::display::ArrayFormatter::value` · arrow-cast 59.3.0

```rust
fn value(&self, idx: usize) -> ValueFormatter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::ArrayFormatter", "path": "ArrayFormatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 1], "end": [529, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns a [`ValueFormatter`](../operations/arrow_cast.display.ValueFormatter.md#op-9ae555388794c1910e97ad8a) that implements [`Display`] for
the value of the array at `idx`

Unresolved upstream links (retained, not inferred): ``Display``.
