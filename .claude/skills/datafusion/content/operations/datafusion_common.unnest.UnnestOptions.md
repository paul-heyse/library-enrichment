# `datafusion_common::unnest::UnnestOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.unnest.UnnestOptions.json).

<a id="op-fc8c0e778d2849560cc4457a"></a>
## UnnestOptions

`struct` · `datafusion_common::unnest::UnnestOptions` · datafusion-common 55.1.0

```rust
struct UnnestOptions
```

Source: `src/unnest.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for unnesting a column that contains a list type,
replicating values in the other, non nested rows.

Conceptually this operation is like joining each row with all the
values in the list column.

The behavior with `NULL` and empty input lists is controlled by
[`NullHandling`](../operations/datafusion_common.unnest.NullHandling.md#op-68e5cdc7da449744d6708464). See its variants for full details.

# Examples

## `Unnest(c1)`, null_handling: NullHandling::Drop
```text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │    3    │ │  E  │
     ├─────────┤ ├─────┤                └─────────┘ └─────┘
     │   {3}   │ │  E  │                    c1        c2
     └─────────┘ └─────┘
       c1         c2
```

## `Unnest(c1)`, null_handling: NullHandling::Preserve
```text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │  null   │ │  B  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │   {3}   │ │  E  │                │    3    │ │  E  │
     └─────────┘ └─────┘                └─────────┘ └─────┘
       c1         c2                        c1        c2
```

## `Unnest(c1)`, null_handling: NullHandling::PreserveAndExpandEmpty
```text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │  null   │ │  B  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │   {3}   │ │  E  │                │  null   │ │  D  │
     └─────────┘ └─────┘                ├─────────┤ ├─────┤
       c1         c2                    │    3    │ │  E  │
                                        └─────────┘ └─────┘
                                            c1        c2
```

`recursions` instruct how a column should be unnested (e.g unnesting a column multiple
time, with depth = 1 and depth = 2). Any unnested column not being mentioned inside this
options is inferred to be unnested with depth = 1

<a id="op-f0325f0803753a89e01fb450"></a>
## clone

`function` · `datafusion_common::unnest::UnnestOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> UnnestOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 17], "end": [100, 22], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unnest.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f28a8a4ac19860cbb6f294b"></a>
## default

`function` · `datafusion_common::unnest::UnnestOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [127, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unnest.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c854e38555803bb83ef2515f"></a>
## eq

`function` · `datafusion_common::unnest::UnnestOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &UnnestOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 24], "end": [100, 33], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unnest.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecd1d645cf0ae3f69f604c88"></a>
## expand_empty_as_null

`function` · `datafusion_common::unnest::UnnestOptions::expand_empty_as_null` · datafusion-common 55.1.0

```rust
fn expand_empty_as_null(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [175, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if empty input lists should produce a single
output row containing `NULL`.

<a id="op-8a629f1a9c2e75be5f940dfd"></a>
## fmt

`function` · `datafusion_common::unnest::UnnestOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 10], "end": [100, 15], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unnest.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0bb9f945e05d74b9f62f73d"></a>
## hash

`function` · `datafusion_common::unnest::UnnestOptions::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 47], "end": [100, 51], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/unnest.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37699f2b723d0a6875ac6fd5"></a>
## new

`function` · `datafusion_common::unnest::UnnestOptions::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [175, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a) with default values

<a id="op-0f2352b4f8831d2c792edc61"></a>
## null_handling

`struct_field` · `datafusion_common::unnest::UnnestOptions::null_handling` · datafusion-common 55.1.0

```rust
null_handling: NullHandling
```

Source: `src/unnest.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

How to handle `NULL` and empty list values in the input column.
Defaults to [`NullHandling::Preserve`](../operations/datafusion_common.unnest.NullHandling.md#op-4dedf59fe69b83f1dd1f200f).

<a id="op-6b8adc8e6f06232fe3fbc121"></a>
## partial_cmp

`function` · `datafusion_common::unnest::UnnestOptions::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &UnnestOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 35], "end": [100, 45], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/unnest.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-296257c9bd846e7eaed7ced5"></a>
## preserve_nulls

`function` · `datafusion_common::unnest::UnnestOptions::preserve_nulls` · datafusion-common 55.1.0

```rust
fn preserve_nulls(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [175, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if `NULL` input rows produce a single output row
containing `NULL`.

<a id="op-473b58d602c61a38dc1645be"></a>
## recursions

`struct_field` · `datafusion_common::unnest::UnnestOptions::recursions` · datafusion-common 55.1.0

```rust
recursions: Vec<RecursionUnnestOption>
```

Source: `src/unnest.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If specific columns need to be unnested multiple times (e.g at different depth),
declare them here. Any unnested columns not being mentioned inside this option
will be unnested with depth = 1

<a id="op-65c7e0a78a8cffe155734e8a"></a>
## with_null_handling

`function` · `datafusion_common::unnest::UnnestOptions::with_null_handling` · datafusion-common 55.1.0

```rust
fn with_null_handling(self, null_handling: NullHandling) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [175, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the [`NullHandling`](../operations/datafusion_common.unnest.NullHandling.md#op-68e5cdc7da449744d6708464) mode used when unnesting `NULL` or empty
input lists.

<a id="op-837a9c49cce6188c5e1486b6"></a>
## with_preserve_nulls

`function` · `datafusion_common::unnest::UnnestOptions::with_preserve_nulls` · datafusion-common 55.1.0

```rust
fn with_preserve_nulls(self, preserve_nulls: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [175, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Backward-compatible setter that maps the previous boolean
`preserve_nulls` flag onto [`NullHandling`](../operations/datafusion_common.unnest.NullHandling.md#op-68e5cdc7da449744d6708464).

`true` maps to [`NullHandling::Preserve`](../operations/datafusion_common.unnest.NullHandling.md#op-4dedf59fe69b83f1dd1f200f); `false` maps to
[`NullHandling::Drop`](../operations/datafusion_common.unnest.NullHandling.md#op-f4589eddcff39f31e7244379). To opt into the new empty-list-preserving
mode, call [`Self::with_null_handling`](../operations/datafusion_common.unnest.UnnestOptions.md#op-65c7e0a78a8cffe155734e8a) directly with
[`NullHandling::PreserveAndExpandEmpty`](../operations/datafusion_common.unnest.NullHandling.md#op-c44c790e1010575c59d513ff).

<a id="op-65f5d5e6cc3ed4f24f9a10cb"></a>
## with_recursions

`function` · `datafusion_common::unnest::UnnestOptions::with_recursions` · datafusion-common 55.1.0

```rust
fn with_recursions(self, recursion: RecursionUnnestOption) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::UnnestOptions", "path": "UnnestOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [175, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the recursions for the unnest operation
