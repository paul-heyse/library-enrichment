# `arrow::util::test_util::BadIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.test_util.BadIterator.json).

<a id="op-1fdecbc2f7ca1d61b536e3b4"></a>
## BadIterator

`struct` · `arrow::util::test_util::BadIterator` · arrow 59.3.0

```rust
struct BadIterator<T>
```

Source: `src/util/test_util.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

An iterator that is untruthful about its actual length

<a id="op-b48514a252239d296c44f431"></a>
## Item

`assoc_type` · `arrow::util::test_util::BadIterator::Item` · arrow 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::util::test_util::BadIterator", "path": "BadIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [201, 2], "filename": "src/util/test_util.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/test_util.rs:184`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-294fe446d6dee2044f6ba05b"></a>
## clone

`function` · `arrow::util::test_util::BadIterator::clone` · arrow 59.3.0

```rust
fn clone(&self) -> BadIterator<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::util::test_util::BadIterator", "path": "BadIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 17], "end": [156, 22], "filename": "src/util/test_util.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/util/test_util.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45c9a0fda2e092c6eb7bc3c6"></a>
## fmt

`function` · `arrow::util::test_util::BadIterator::fmt` · arrow 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::util::test_util::BadIterator", "path": "BadIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 10], "end": [156, 15], "filename": "src/util/test_util.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/test_util.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7c116684ac1fa40a7883fd5"></a>
## items

`struct_field` · `arrow::util::test_util::BadIterator::items` · arrow 59.3.0

```rust
items: Vec<T>
```

Source: `src/util/test_util.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The items to return. If there are fewer items than `limit`
they will be repeated

<a id="op-97c4e30e35be2339cb36a46e"></a>
## new

`function` · `arrow::util::test_util::BadIterator::new` · arrow 59.3.0

```rust
fn new(limit: usize, claimed: usize, items: Vec<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::util::test_util::BadIterator", "path": "BadIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [181, 2], "filename": "src/util/test_util.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/test_util.rs:172`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a new iterator for `<limit>` items, but that reports to
produce `<claimed>` items. Must provide at least 1 item.

<a id="op-e7242e375986397306eb66fa"></a>
## next

`function` · `arrow::util::test_util::BadIterator::next` · arrow 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::util::test_util::BadIterator", "path": "BadIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [201, 2], "filename": "src/util/test_util.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/test_util.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69dead45cc1255023081c88e"></a>
## size_hint

`function` · `arrow::util::test_util::BadIterator::size_hint` · arrow 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::util::test_util::BadIterator", "path": "BadIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [201, 2], "filename": "src/util/test_util.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/test_util.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

report whatever the iterator says to
