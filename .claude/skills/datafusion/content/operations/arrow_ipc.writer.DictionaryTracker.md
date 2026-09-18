# `arrow_ipc::writer::DictionaryTracker`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.DictionaryTracker.json).

<a id="op-29e5b236e3aaa8b8b95a1657"></a>
## DictionaryTracker

`struct` · `arrow_ipc::writer::DictionaryTracker` · arrow-ipc 59.3.0

```rust
struct DictionaryTracker
```

Source: `src/writer.rs:1362`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Keeps track of dictionaries that have been written, to avoid emitting the same dictionary
multiple times.

Can optionally error if an update to an existing dictionary is attempted, which
isn't allowed in the `FileWriter`.

<a id="op-9aec58c711cf7815796aea7d"></a>
## clear

`function` · `arrow_ipc::writer::DictionaryTracker::clear` · arrow-ipc 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 1], "end": [1531, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1527`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Clears the state of the dictionary tracker.

This allows the dictionary tracker to be reused for a new IPC stream while avoiding the
allocation cost of creating a new instance. This method should not be called if
the dictionary tracker will be used to continue writing to an existing IPC stream.

<a id="op-3f5179b278c1321b864dfdbc"></a>
## dict_id

`function` · `arrow_ipc::writer::DictionaryTracker::dict_id` · arrow-ipc 59.3.0

```rust
fn dict_id(&mut self) -> &[i64]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 1], "end": [1531, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1399`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return the sequence of dictionary IDs in the order they should be observed while
traversing the schema

<a id="op-6b8a712421c24fce19cf92e0"></a>
## fmt

`function` · `arrow_ipc::writer::DictionaryTracker::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 10], "end": [1361, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:1361`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b1b645edb897ad0cd1fb3fd"></a>
## insert

`function` · `arrow_ipc::writer::DictionaryTracker::insert` · arrow-ipc 59.3.0

```rust
fn insert(&mut self, dict_id: i64, column: &ArrayRef) -> Result<bool, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 1], "end": [1531, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1413`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Keep track of the dictionary with the given ID and values. Behavior:

* If this ID has been written already and has the same data, return `Ok(false)` to indicate
  that the dictionary was not actually inserted (because it's already been seen).
* If this ID has been written already but with different data, and this tracker is
  configured to return an error, return an error.
* If the tracker has not been configured to error on replacement or this dictionary
  has never been seen before, return `Ok(true)` to indicate that the dictionary was just
  inserted.

<a id="op-5be657f84621937faf8005dc"></a>
## insert_column

`function` · `arrow_ipc::writer::DictionaryTracker::insert_column` · arrow-ipc 59.3.0

```rust
fn insert_column(&mut self, dict_id: i64, column: &ArrayRef, dict_handling: DictionaryHandling) -> Result<DictionaryUpdate, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 1], "end": [1531, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1457`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Keep track of the dictionary with the given ID and values. The return
value indicates what, if any, update to the internal map took place
and how it should be interpreted based on the `dict_handling` parameter.

# Returns

* `Ok(Dictionary::New)` - If the dictionary was not previously written
* `Ok(Dictionary::Replaced)` - If the dictionary was previously written
  with completely different data, or if the data is a delta of the existing,
  but with `dict_handling` set to `DictionaryHandling::Resend`
* `Ok(Dictionary::Delta)` - If the dictionary was previously written, but
  the new data is a delta of the old and the `dict_handling` is set to
  `DictionaryHandling::Delta`
* `Err(e)` - If the dictionary was previously written with different data,
  and `error_on_replacement` is set to `true`.

<a id="op-d7892a096224badc9b450d9f"></a>
## new

`function` · `arrow_ipc::writer::DictionaryTracker::new` · arrow-ipc 59.3.0

```rust
fn new(error_on_replacement: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 1], "end": [1531, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1375`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Create a new [`DictionaryTracker`](../operations/arrow_ipc.writer.DictionaryTracker.md#op-29e5b236e3aaa8b8b95a1657).

If `error_on_replacement`
is true, an error will be generated if an update to an
existing dictionary is attempted.

<a id="op-59cb6eebd22b4ed44aa7177c"></a>
## next_dict_id

`function` · `arrow_ipc::writer::DictionaryTracker::next_dict_id` · arrow-ipc 59.3.0

```rust
fn next_dict_id(&mut self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryTracker", "path": "DictionaryTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 1], "end": [1531, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1385`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Record and return the next dictionary ID.
