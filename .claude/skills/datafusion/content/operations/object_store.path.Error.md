# `object_store::path::Error`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.path.Error.json).

<a id="op-28f038cdccc6e35e04b70c9a"></a>
## Error

`enum` · `object_store::path::Error` · object_store 0.13.2

```rust
enum Error
```

Source: `src/path/mod.rs:41`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error returned by [`Path::parse`](../operations/object_store.path.Path.md#op-93733e7df06c53ee809baf53)

<a id="op-31e3cdaddde00596bd22c6af"></a>
## BadSegment

`variant` · `object_store::path::Error::BadSegment` · object_store 0.13.2

```rust
BadSegment
```

Source: `src/path/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when an invalid segment is encountered in the given path

<a id="op-ac0efeacd4b78d2718a54d9b"></a>
## Canonicalize

`variant` · `object_store::path::Error::Canonicalize` · object_store 0.13.2

```rust
Canonicalize
```

Source: `src/path/mod.rs:60`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when path cannot be canonicalized

<a id="op-f2dd2e96397a9ba6d64291a0"></a>
## EmptySegment

`variant` · `object_store::path::Error::EmptySegment` · object_store 0.13.2

```rust
EmptySegment
```

Source: `src/path/mod.rs:44`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when there's an empty segment between two slashes `/` in the path

<a id="op-3a9488bf5a9e5dd0a2f56c26"></a>
## InvalidPath

`variant` · `object_store::path::Error::InvalidPath` · object_store 0.13.2

```rust
InvalidPath
```

Source: `src/path/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the path is not a valid URL

<a id="op-42263dd7c59827891da9612e"></a>
## NonUnicode

`variant` · `object_store::path::Error::NonUnicode` · object_store 0.13.2

```rust
NonUnicode
```

Source: `src/path/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when a path contains non-unicode characters

<a id="op-e37939c9ba5faa29bb7b1311"></a>
## PrefixMismatch

`variant` · `object_store::path::Error::PrefixMismatch` · object_store 0.13.2

```rust
PrefixMismatch
```

Source: `src/path/mod.rs:85`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the a path doesn't start with given prefix

<a id="op-24783ab5d5850c6968aa26b3"></a>
## fmt

`function` · `object_store::path::Error::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/path/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-251ad9af9759fcee4e67bfee"></a>
## fmt

`function` · `object_store::path::Error::fmt` · object_store 0.13.2

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 33], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/path/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bc956ec5bee04a2e5a70c0b"></a>
## source

`function` · `object_store::path::Error::source` · object_store 0.13.2

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 33], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/path/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
