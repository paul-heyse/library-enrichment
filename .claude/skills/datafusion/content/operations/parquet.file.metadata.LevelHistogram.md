# `parquet::file::metadata::LevelHistogram`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.LevelHistogram.json).

<a id="op-35dbdd48a092ea687173e598"></a>
## LevelHistogram

`struct` · `parquet::file::metadata::LevelHistogram` · parquet 59.3.0

```rust
struct LevelHistogram
```

Source: `src/file/metadata/mod.rs:852`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Histograms for repetition and definition levels.

Each histogram is a vector of length `max_level + 1`. The value at index `i` is the number of
values at level `i`.

For example, `vec[0]` is the number of rows with level 0, `vec[1]` is the
number of rows with level 1, and so on.


<a id="op-e9b91263b06cc8499279f2b9"></a>
## add

`function` · `parquet::file::metadata::LevelHistogram::add` · parquet 59.3.0

```rust
fn add(&mut self, other: &Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:895`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Adds the values from the other histogram to this histogram

# Panics
If the histograms have different lengths

<a id="op-9dd2892f666837130904936b"></a>
## clone

`function` · `parquet::file::metadata::LevelHistogram::clone` · parquet 59.3.0

```rust
fn clone(&self) -> LevelHistogram
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 17], "end": [851, 22], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:851`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c5ba78d4be5a115232effdd"></a>
## default

`function` · `parquet::file::metadata::LevelHistogram::default` · parquet 59.3.0

```rust
fn default() -> LevelHistogram
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 45], "end": [851, 52], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/metadata/mod.rs:851`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c17462aa1cf49bbbe9d3112"></a>
## eq

`function` · `parquet::file::metadata::LevelHistogram::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &LevelHistogram) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 24], "end": [851, 33], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:851`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8beb092346e58f8c8b2ae72"></a>
## fmt

`function` · `parquet::file::metadata::LevelHistogram::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 10], "end": [851, 15], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:851`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9058ec0af187b2feb7a51ff"></a>
## from

`function` · `parquet::file::metadata::LevelHistogram::from` · parquet 59.3.0

```rust
fn from(inner: Vec<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [942, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file/metadata/mod.rs:939`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f47c64b2d674834b173257f"></a>
## get

`function` · `parquet::file::metadata::LevelHistogram::get` · parquet 59.3.0

```rust
fn get(&self, index: usize) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:887`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the histogram value at the given index.

The value of `i` is the number of values with level `i`. For example,
`get(1)` returns the number of values with level 1.

Returns `None` if the index is out of bounds.

<a id="op-2d39311ed895e235ffc57e66"></a>
## hash

`function` · `parquet::file::metadata::LevelHistogram::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 39], "end": [851, 43], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/file/metadata/mod.rs:851`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8d883a1e316e6c5c9891024"></a>
## increment_by

`function` · `parquet::file::metadata::LevelHistogram::increment_by` · parquet 59.3.0

```rust
fn increment_by(&mut self, level: i16, count: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:921`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Increments the count for a level value by `count`.

<a id="op-f029281eb7e36fe6b80a7a31"></a>
## into_inner

`function` · `parquet::file::metadata::LevelHistogram::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> Vec<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:877`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the inner vector, consuming self

<a id="op-7fc67b52f74c5c38df476695"></a>
## is_empty

`function` · `parquet::file::metadata::LevelHistogram::is_empty` · parquet 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:908`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

returns if the histogram is empty

<a id="op-65910343e895b5349c73a007"></a>
## len

`function` · `parquet::file::metadata::LevelHistogram::len` · parquet 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:903`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

return the length of the histogram

<a id="op-27bf41a6d6dfeeac2b79b6a4"></a>
## reset

`function` · `parquet::file::metadata::LevelHistogram::reset` · parquet 59.3.0

```rust
fn reset(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:913`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the values of all histogram levels to 0.

<a id="op-f6f740ab0ed35c679b396dd8"></a>
## try_new

`function` · `parquet::file::metadata::LevelHistogram::try_new` · parquet 59.3.0

```rust
fn try_new(max_level: i16) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:862`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new level histogram data.

Length will be `max_level + 1`.

Returns `None` when `max_level == 0` (because histograms are not necessary in this case)

<a id="op-0c8209fd519b382aa6891d2d"></a>
## update_from_levels

`function` · `parquet::file::metadata::LevelHistogram::update_from_levels` · parquet 59.3.0

```rust
fn update_from_levels(&mut self, levels: &[i16])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:931`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Updates histogram values using provided repetition levels

# Panics
if any of the levels is greater than the length of the histogram (
the argument supplied to [`Self::try_new`](../operations/parquet.file.metadata.LevelHistogram.md#op-f6f740ab0ed35c679b396dd8))

<a id="op-a28429a211cfa729ca2b1e89"></a>
## values

`function` · `parquet::file::metadata::LevelHistogram::values` · parquet 59.3.0

```rust
fn values(&self) -> &[i64]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::LevelHistogram", "path": "LevelHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [856, 1], "end": [936, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:872`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the the histogram's values.
