# `object_store::aws::precondition::S3ConditionalPut`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.precondition.S3ConditionalPut.json).

<a id="op-6d48bb95dd976ecea4dd3593"></a>
## S3ConditionalPut

`enum` · `object_store::aws::precondition::S3ConditionalPut` · object_store 0.13.2

```rust
enum S3ConditionalPut
```

Source: `src/aws/precondition.rs:120`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure how to provide conditional put support for [`AmazonS3`].

[`AmazonS3`]: super::AmazonS3

<a id="op-a6157a456f27e8ac68a9f8ed"></a>
## Disabled

`variant` · `object_store::aws::precondition::S3ConditionalPut::Disabled` · object_store 0.13.2

```rust
Disabled
```

Source: `src/aws/precondition.rs:131`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Disable `conditional put`

<a id="op-944ddb32617ce60501101093"></a>
## ETagMatch

`variant` · `object_store::aws::precondition::S3ConditionalPut::ETagMatch` · object_store 0.13.2

```rust
ETagMatch
```

Source: `src/aws/precondition.rs:128`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Some S3-compatible stores, such as Cloudflare R2 and minio support conditional
put using the standard [HTTP precondition] headers If-Match and If-None-Match

Encoded as `etag` ignoring whitespace

[HTTP precondition]: https://datatracker.ietf.org/doc/html/rfc9110#name-preconditions

<a id="op-03f966717515eadcc1474e35"></a>
## clone

`function` · `object_store::aws::precondition::S3ConditionalPut::clone` · object_store 0.13.2

```rust
fn clone(&self) -> S3ConditionalPut
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3ConditionalPut", "path": "S3ConditionalPut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 17], "end": [117, 22], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aws/precondition.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4403e36a1d1b43d0b4c9a458"></a>
## default

`function` · `object_store::aws::precondition::S3ConditionalPut::default` · object_store 0.13.2

```rust
fn default() -> S3ConditionalPut
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3ConditionalPut", "path": "S3ConditionalPut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 39], "end": [117, 46], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aws/precondition.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c5e3ef9fba659895abcac55"></a>
## eq

`function` · `object_store::aws::precondition::S3ConditionalPut::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &S3ConditionalPut) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3ConditionalPut", "path": "S3ConditionalPut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 28], "end": [117, 37], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aws/precondition.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-007c2a451a611907932d4cf9"></a>
## fmt

`function` · `object_store::aws::precondition::S3ConditionalPut::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3ConditionalPut", "path": "S3ConditionalPut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [141, 2], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/aws/precondition.rs:135`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bddd4784f27b77f7d69d4c1b"></a>
## fmt

`function` · `object_store::aws::precondition::S3ConditionalPut::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3ConditionalPut", "path": "S3ConditionalPut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 10], "end": [117, 15], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/precondition.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
