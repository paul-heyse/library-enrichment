# `arrow_schema::SortOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.SortOptions.json).

<a id="op-78d98c3e0c6da432658d0949"></a>
## SortOptions

`struct` · `arrow_schema::SortOptions` · arrow-schema 59.3.0

```rust
struct SortOptions
```

Source: `src/lib.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Options that define the sort order of a given column

The default sorts equivalently to of `ASC NULLS FIRST` in SQL (i.e.
ascending order with nulls sorting before any other values).

# Example creation
```
# use arrow_schema::SortOptions;
// configure using explicit initialization
let options = SortOptions {
  descending: false,
  nulls_first: true,
};
// Default is ASC NULLs First
assert_eq!(options, SortOptions::default());
assert_eq!(options.to_string(), "ASC NULLS FIRST");

// Configure using builder APIs
let options = SortOptions::default()
 .desc()
 .nulls_first();
assert_eq!(options.to_string(), "DESC NULLS FIRST");

// configure using explicit field values
let options = SortOptions::default()
 .with_descending(false)
 .with_nulls_first(false);
assert_eq!(options.to_string(), "ASC NULLS LAST");
```

# Example operations
It is also possible to negate the sort options using the `!` operator.
```
use arrow_schema::SortOptions;
let options = !SortOptions::default();
assert_eq!(options.to_string(), "DESC NULLS LAST");
```

<a id="op-b6a025c4253783fb314c1a05"></a>
## Output

`assoc_type` · `arrow_schema::SortOptions::Output` · arrow-schema 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [183, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::ops::bit::Not", "path": "Not"}, "trait_path": "core::ops::bit::Not"}`

Source: `src/lib.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-242a54166567bc4cd92fab90"></a>
## asc

`function` · `arrow_schema::SortOptions::asc` · arrow-schema 59.3.0

```rust
fn asc(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set this sort options to sort in ascending order

See [Self::with_descending](../operations/arrow_schema.SortOptions.md#op-665ddddb1b914f0b7d946e23) to explicitly set the underlying field

<a id="op-16c69ae1e2e0f5038ecd23de"></a>
## clone

`function` · `arrow_schema::SortOptions::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> SortOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c24be2c533a5b7022503ef48"></a>
## cmp

`function` · `arrow_schema::SortOptions::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &SortOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 51], "end": [84, 54], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/lib.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c42af1ae3aec3e08842ec8a"></a>
## default

`function` · `arrow_schema::SortOptions::default` · arrow-schema 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [170, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:163`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72ce470ab2294182263e9b3f"></a>
## desc

`function` · `arrow_schema::SortOptions::desc` · arrow-schema 59.3.0

```rust
fn desc(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set this sort options to sort in descending order

See [Self::with_descending](../operations/arrow_schema.SortOptions.md#op-665ddddb1b914f0b7d946e23) to explicitly set the underlying field

<a id="op-11c7eb62289658c6a7130d85"></a>
## descending

`struct_field` · `arrow_schema::SortOptions::descending` · arrow-schema 59.3.0

```rust
descending: bool
```

Source: `src/lib.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Whether to sort in descending order

<a id="op-7e7ee3fc6adc12e46a640ad5"></a>
## eq

`function` · `arrow_schema::SortOptions::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &SortOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 40], "end": [84, 49], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-418a52e256ea9f7e7075fcf9"></a>
## fmt

`function` · `arrow_schema::SortOptions::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [106, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47f0a50639dac8db13643c27"></a>
## fmt

`function` · `arrow_schema::SortOptions::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 29], "end": [84, 34], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c4538e32ec3789c0e28dc37"></a>
## hash

`function` · `arrow_schema::SortOptions::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 17], "end": [84, 21], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/lib.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0a2200cbe67570818cac3d8"></a>
## new

`function` · `arrow_schema::SortOptions::new` · arrow-schema 59.3.0

```rust
fn new(descending: bool, nulls_first: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new `SortOptions` struct

<a id="op-68dadd41d6eda517dcba8f03"></a>
## not

`function` · `arrow_schema::SortOptions::not` · arrow-schema 59.3.0

```rust
fn not(self) -> SortOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [183, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::ops::bit::Not", "path": "Not"}, "trait_path": "core::ops::bit::Not"}`

Source: `src/lib.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-276a99172b51d6903cd00ef2"></a>
## nulls_first

`struct_field` · `arrow_schema::SortOptions::nulls_first` · arrow-schema 59.3.0

```rust
nulls_first: bool
```

Source: `src/lib.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Whether to sort nulls first

<a id="op-348baacc05280b9d6fdd7da1"></a>
## nulls_first

`function` · `arrow_schema::SortOptions::nulls_first` · arrow-schema 59.3.0

```rust
fn nulls_first(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set this sort options to sort nulls first

See [Self::with_nulls_first](../operations/arrow_schema.SortOptions.md#op-107737684f457913d8f66ca6) to explicitly set the underlying field

<a id="op-07e0d18b812747e738b624d3"></a>
## nulls_last

`function` · `arrow_schema::SortOptions::nulls_last` · arrow-schema 59.3.0

```rust
fn nulls_last(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set this sort options to sort nulls last

See [Self::with_nulls_first](../operations/arrow_schema.SortOptions.md#op-107737684f457913d8f66ca6) to explicitly set the underlying field

<a id="op-b84294bb73298c919247f3d8"></a>
## partial_cmp

`function` · `arrow_schema::SortOptions::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &SortOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 56], "end": [84, 66], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/lib.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-665ddddb1b914f0b7d946e23"></a>
## with_descending

`function` · `arrow_schema::SortOptions::with_descending` · arrow-schema 59.3.0

```rust
fn with_descending(self, descending: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:150`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set this sort options to sort descending if argument is true

<a id="op-107737684f457913d8f66ca6"></a>
## with_nulls_first

`function` · `arrow_schema::SortOptions::with_nulls_first` · arrow-schema 59.3.0

```rust
fn with_nulls_first(self, nulls_first: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [160, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set this sort options to sort nulls first if argument is true
