# `object_store::Error`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.json).

<a id="op-51a98b07f5b717255c5dc721"></a>
## Error

`enum` · `object_store::Error` · object_store 0.13.2

```rust
enum Error
```

Source: `src/lib.rs:2021`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A specialized `Error` for object store-related errors

<a id="op-aa9eb7b8d711d881e621142c"></a>
## AlreadyExists

`variant` · `object_store::Error::AlreadyExists` · object_store 0.13.2

```rust
AlreadyExists
```

Source: `src/lib.rs:2066`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the object already exists

<a id="op-7d98456a2fa7b1a44976ed70"></a>
## Generic

`variant` · `object_store::Error::Generic` · object_store 0.13.2

```rust
Generic
```

Source: `src/lib.rs:2024`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A fallback error type when no variant matches

<a id="op-9e4716cabf372afc34651ac5"></a>
## InvalidPath

`variant` · `object_store::Error::InvalidPath` · object_store 0.13.2

```rust
InvalidPath
```

Source: `src/lib.rs:2042`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error for invalid path

<a id="op-d5b317e231612a8ecf307f7a"></a>
## JoinError

`variant` · `object_store::Error::JoinError` · object_store 0.13.2

```rust
JoinError
```

Source: `src/lib.rs:2051`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when `tokio::spawn` failed

<a id="op-05c916c94bd0100476adb324"></a>
## NotFound

`variant` · `object_store::Error::NotFound` · object_store 0.13.2

```rust
NotFound
```

Source: `src/lib.rs:2033`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the object is not found at given location

<a id="op-6ccd2e64d49edb4b598a4789"></a>
## NotImplemented

`variant` · `object_store::Error::NotImplemented` · object_store 0.13.2

```rust
NotImplemented
```

Source: `src/lib.rs:2093`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when an operation is not implemented

<a id="op-06842ee960fff2816c7230b3"></a>
## NotModified

`variant` · `object_store::Error::NotModified` · object_store 0.13.2

```rust
NotModified
```

Source: `src/lib.rs:2084`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the object at the location isn't modified

<a id="op-4db5b365ef9ce6e523d3e984"></a>
## NotSupported

`variant` · `object_store::Error::NotSupported` · object_store 0.13.2

```rust
NotSupported
```

Source: `src/lib.rs:2059`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the attempted operation is not supported

<a id="op-2f5d89db69a86ede547b17be"></a>
## PermissionDenied

`variant` · `object_store::Error::PermissionDenied` · object_store 0.13.2

```rust
PermissionDenied
```

Source: `src/lib.rs:2111`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the used credentials don't have enough permission
to perform the requested operation

<a id="op-530afde8313e7ef1611c8b0a"></a>
## Precondition

`variant` · `object_store::Error::Precondition` · object_store 0.13.2

```rust
Precondition
```

Source: `src/lib.rs:2075`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the required conditions failed for the operation

<a id="op-5da7c27db83efc349e3edb51"></a>
## Unauthenticated

`variant` · `object_store::Error::Unauthenticated` · object_store 0.13.2

```rust
Unauthenticated
```

Source: `src/lib.rs:2124`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when the used credentials lack valid authentication

<a id="op-ab1d4b2d4705d2f270f643cd"></a>
## UnknownConfigurationKey

`variant` · `object_store::Error::UnknownConfigurationKey` · object_store 0.13.2

```rust
UnknownConfigurationKey
```

Source: `src/lib.rs:2133`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Error when a configuration key is invalid for the store used

<a id="op-95080716e9dd3eeb733a0f54"></a>
## fmt

`function` · `object_store::Error::fmt` · object_store 0.13.2

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2019, 17], "end": [2019, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:2019`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa52c73197334ee99ddfaffd"></a>
## fmt

`function` · `object_store::Error::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2019, 10], "end": [2019, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:2019`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0d2c1f7ed581478a69e0c47"></a>
## from

`function` · `object_store::Error::from` · object_store 0.13.2

```rust
fn from(source: tokio::task::JoinError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2053, 9], "end": [2053, 16], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tokio::runtime::task::error::JoinError", "path": "JoinError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:2019`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf2eb05e88f0bc86b1c298c4"></a>
## from

`function` · `object_store::Error::from` · object_store 0.13.2

```rust
fn from(source: path::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2044, 9], "end": [2044, 16], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:2019`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c34690e788e7ee2635d5eea"></a>
## source

`function` · `object_store::Error::source` · object_store 0.13.2

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2019, 17], "end": [2019, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/lib.rs:2019`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
