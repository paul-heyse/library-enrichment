# `arrow_cast::display::FormatOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.FormatOptions.json).

<a id="op-739ef37988037ce736e7faff"></a>
## FormatOptions

`struct` · `arrow_cast::display::FormatOptions` · arrow-cast 59.3.0

```rust
struct FormatOptions<'a>
```

Source: `src/display.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Options for formatting arrays

By default nulls are formatted as `""` and temporal types formatted
according to RFC3339

# Equality

Most fields in [`FormatOptions`](../operations/arrow_cast.display.FormatOptions.md#op-739ef37988037ce736e7faff) are compared by value, except `formatter_factory`. As the trait
does not require an [`Eq`] and [`Hash`] implementation, this struct only compares the pointer of
the factories.

Unresolved upstream links (retained, not inferred): ``Eq``, ``Hash``.

<a id="op-dfbb095993974721f22b3dde"></a>
## clone

`function` · `arrow_cast::display::FormatOptions::clone` · arrow-cast 59.3.0

```rust
fn clone(&self) -> FormatOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 22], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/display.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14fd4e3db3221ee17bf87c5b"></a>
## date_format

`function` · `arrow_cast::display::FormatOptions::date_format` · arrow-cast 59.3.0

```rust
const fn date_format(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the format used for [`DataType::Date32`](../operations/arrow_schema.datatype.DataType.md#op-9e3071e3ccfadc14c99b045d) columns.

<a id="op-9c69325cf81ccbf608ea7674"></a>
## datetime_format

`function` · `arrow_cast::display::FormatOptions::datetime_format` · arrow-cast 59.3.0

```rust
const fn datetime_format(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:266`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the format used for [`DataType::Date64`](../operations/arrow_schema.datatype.DataType.md#op-4e02b24282692b3bcad20b8c) columns.

<a id="op-50a475773bb41da5f8718f2d"></a>
## default

`function` · `arrow_cast::display::FormatOptions::default` · arrow-cast 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [94, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/display.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdb9aa4f726c804526d3c2c1"></a>
## duration_format

`function` · `arrow_cast::display::FormatOptions::duration_format` · arrow-cast 59.3.0

```rust
const fn duration_format(&self) -> DurationFormat
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the [`DurationFormat`](../operations/arrow_cast.display.DurationFormat.md#op-ae18af0f29abc24d8d67ccc5) used for duration columns.

<a id="op-fe3242cd15105cc425036ca2"></a>
## eq

`function` · `arrow_cast::display::FormatOptions::eq` · arrow-cast 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [114, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/display.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2a8d8e31ebb077fb0913999"></a>
## fmt

`function` · `arrow_cast::display::FormatOptions::fmt` · arrow-cast 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1737b5c943484318a057cf80"></a>
## formatter_factory

`function` · `arrow_cast::display::FormatOptions::formatter_factory` · arrow-cast 59.3.0

```rust
const fn formatter_factory(&self) -> Option<&'a dyn ArrayFormatterFactory>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the [`ArrayFormatterFactory`](../operations/arrow_cast.display.ArrayFormatterFactory.md#op-981732bb15cd351972b89354) used to instantiate custom [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992)s.

<a id="op-2276c8dc890f92ee5ca26487"></a>
## hash

`function` · `arrow_cast::display::FormatOptions::hash` · arrow-cast 59.3.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [134, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/display.rs:119`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6491f92785d93a3984dcb44"></a>
## new

`function` · `arrow_cast::display::FormatOptions::new` · arrow-cast 59.3.0

```rust
const fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Creates a new set of format options

<a id="op-b34aa37c5a2bfa0e4c02e4bb"></a>
## null

`function` · `arrow_cast::display::FormatOptions::null` · arrow-cast 59.3.0

```rust
const fn null(&self) -> &'a str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the string used for displaying nulls.

<a id="op-7409e4e13db6ac84b17b4af1"></a>
## quoted_strings

`function` · `arrow_cast::display::FormatOptions::quoted_strings` · arrow-cast 59.3.0

```rust
const fn quoted_strings(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:296`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns whether string values should be quoted.

<a id="op-83dd6f9b8c23639bda4365ec"></a>
## safe

`function` · `arrow_cast::display::FormatOptions::safe` · arrow-cast 59.3.0

```rust
const fn safe(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:251`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns whether formatting errors should be written to the output instead of being converted
into a [`std::fmt::Error`].

Unresolved upstream links (retained, not inferred): ``std::fmt::Error``.

<a id="op-a46fdd57ce82313e7ccd79a5"></a>
## time_format

`function` · `arrow_cast::display::FormatOptions::time_format` · arrow-cast 59.3.0

```rust
const fn time_format(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the format used for [`DataType::Time32`](../operations/arrow_schema.datatype.DataType.md#op-6a017bf7a588fdb9294652ac) and [`DataType::Time64`](../operations/arrow_schema.datatype.DataType.md#op-2a5f5e259eb3a58bc4a55d67) columns.

<a id="op-606173a7330b7ad7043846a4"></a>
## timestamp_format

`function` · `arrow_cast::display::FormatOptions::timestamp_format` · arrow-cast 59.3.0

```rust
const fn timestamp_format(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:271`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the format used for [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) columns without a timezone.

<a id="op-6c12030d34e7f5315da5d713"></a>
## timestamp_tz_format

`function` · `arrow_cast::display::FormatOptions::timestamp_tz_format` · arrow-cast 59.3.0

```rust
const fn timestamp_tz_format(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:276`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns the format used for [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) columns with a timezone.

<a id="op-683a85d79a5cddc916566799"></a>
## types_info

`function` · `arrow_cast::display::FormatOptions::types_info` · arrow-cast 59.3.0

```rust
const fn types_info(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:291`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Returns true if type info should be included in a visual representation of batches.

<a id="op-80ff3197d6485cb2f9e1186b"></a>
## with_date_format

`function` · `arrow_cast::display::FormatOptions::with_date_format` · arrow-cast 59.3.0

```rust
const fn with_date_format(self, date_format: Option<&'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:169`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the format used for [`DataType::Date32`](../operations/arrow_schema.datatype.DataType.md#op-9e3071e3ccfadc14c99b045d) columns

<a id="op-e3918e6885c3e3785b114466"></a>
## with_datetime_format

`function` · `arrow_cast::display::FormatOptions::with_datetime_format` · arrow-cast 59.3.0

```rust
const fn with_datetime_format(self, datetime_format: Option<&'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the format used for [`DataType::Date64`](../operations/arrow_schema.datatype.DataType.md#op-4e02b24282692b3bcad20b8c) columns

<a id="op-41375742483c8a65e4258294"></a>
## with_display_error

`function` · `arrow_cast::display::FormatOptions::with_display_error` · arrow-cast 59.3.0

```rust
const fn with_display_error(self, safe: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

If set to `true` any formatting errors will be written to the output
instead of being converted into a [`std::fmt::Error`]

Unresolved upstream links (retained, not inferred): ``std::fmt::Error``.

<a id="op-171fcdfba4d5e9dc42415c5e"></a>
## with_duration_format

`function` · `arrow_cast::display::FormatOptions::with_duration_format` · arrow-cast 59.3.0

```rust
const fn with_duration_format(self, duration_format: DurationFormat) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the format used for duration columns

Defaults to [`DurationFormat::ISO8601`](../operations/arrow_cast.display.DurationFormat.md#op-c697aa6d5036a8badee42bc8)

<a id="op-93ef397045d14d628774615f"></a>
## with_formatter_factory

`function` · `arrow_cast::display::FormatOptions::with_formatter_factory` · arrow-cast 59.3.0

```rust
const fn with_formatter_factory(self, formatter_factory: Option<&'a dyn ArrayFormatterFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:239`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the [`ArrayFormatterFactory`](../operations/arrow_cast.display.ArrayFormatterFactory.md#op-981732bb15cd351972b89354) used to instantiate custom [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992)s.

Using [`None`] causes pretty-printers to use the default [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992)s.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-eaba72b82a7ec39c8c70b580"></a>
## with_null

`function` · `arrow_cast::display::FormatOptions::with_null` · arrow-cast 59.3.0

```rust
const fn with_null(self, null: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:164`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the string used to represent a null

Defaults to `""`

<a id="op-baa658491c489bdb6303e3f6"></a>
## with_quoted_strings

`function` · `arrow_cast::display::FormatOptions::with_quoted_strings` · arrow-cast 59.3.0

```rust
const fn with_quoted_strings(self, quoted_strings: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Sets whether string values should be quoted

When `true`, strings are formatted using [`Debug`]-style with double quotes and escaping.
Defaults to `false`

Unresolved upstream links (retained, not inferred): ``Debug``.

<a id="op-b08e4905d04bdd39b6d12150"></a>
## with_time_format

`function` · `arrow_cast::display::FormatOptions::with_time_format` · arrow-cast 59.3.0

```rust
const fn with_time_format(self, time_format: Option<&'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the format used for [`DataType::Time32`](../operations/arrow_schema.datatype.DataType.md#op-6a017bf7a588fdb9294652ac) and [`DataType::Time64`](../operations/arrow_schema.datatype.DataType.md#op-2a5f5e259eb3a58bc4a55d67) columns

<a id="op-1e71af7121157f68ce4d5552"></a>
## with_timestamp_format

`function` · `arrow_cast::display::FormatOptions::with_timestamp_format` · arrow-cast 59.3.0

```rust
const fn with_timestamp_format(self, timestamp_format: Option<&'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the format used for [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) columns without a timezone

<a id="op-3b4bc1eea640b8d96e6652b9"></a>
## with_timestamp_tz_format

`function` · `arrow_cast::display::FormatOptions::with_timestamp_tz_format` · arrow-cast 59.3.0

```rust
const fn with_timestamp_tz_format(self, timestamp_tz_format: Option<&'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:193`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides the format used for [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) columns with a timezone

<a id="op-a0660832deee2d9c5c6cc0e6"></a>
## with_types_info

`function` · `arrow_cast::display::FormatOptions::with_types_info` · arrow-cast 59.3.0

```rust
const fn with_types_info(self, types_info: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::display::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [304, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:221`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Overrides if types should be shown

Defaults to [`false`]

Unresolved upstream links (retained, not inferred): ``false``.
