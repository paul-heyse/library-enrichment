# `object_store::aws::precondition::S3CopyIfNotExists`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.precondition.S3CopyIfNotExists.json).

<a id="op-2c842b27294b70ddd3d5ebd4"></a>
## S3CopyIfNotExists

`enum` · `object_store::aws::precondition::S3CopyIfNotExists` · object_store 0.13.2

```rust
enum S3CopyIfNotExists
```

Source: `src/aws/precondition.rs:28`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure how to provide [`CopyMode::Create`] for [`AmazonS3`].

[`CopyMode::Create`]: crate::CopyMode::Create
[`AmazonS3`]: super::AmazonS3

<a id="op-077c679c9a21e6eebd606ee1"></a>
## Header

`variant` · `object_store::aws::precondition::S3CopyIfNotExists::Header` · object_store 0.13.2

```rust
Header
```

Source: `src/aws/precondition.rs:42`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Some S3-compatible stores, such as Cloudflare R2, support copy if not exists
semantics through custom headers.

If set, [`CopyMode::Create`] will perform a normal copy operation
with the provided header pair, and expect the store to fail with `412 Precondition Failed`
if the destination file already exists.

Encoded as `header:<HEADER_NAME>:<HEADER_VALUE>` ignoring whitespace

For example `header: cf-copy-destination-if-none-match: *`, would set
the header `cf-copy-destination-if-none-match` to `*`

[`CopyMode::Create`]: crate::CopyMode::Create

<a id="op-4a2e884bfa408680a1e4d1a5"></a>
## HeaderWithStatus

`variant` · `object_store::aws::precondition::S3CopyIfNotExists::HeaderWithStatus` · object_store 0.13.2

```rust
HeaderWithStatus
```

Source: `src/aws/precondition.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The same as [`S3CopyIfNotExists::Header`](../operations/object_store.aws.precondition.S3CopyIfNotExists.md#op-077c679c9a21e6eebd606ee1) but allows custom status code checking, for object stores that return values
other than 412.

Encoded as `header-with-status:<HEADER_NAME>:<HEADER_VALUE>:<STATUS>` ignoring whitespace

<a id="op-ef216d233b3181494fb55ccc"></a>
## Multipart

`variant` · `object_store::aws::precondition::S3CopyIfNotExists::Multipart` · object_store 0.13.2

```rust
Multipart
```

Source: `src/aws/precondition.rs:62`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Native Amazon S3 supports copy if not exists through a multipart upload
where the upload copies an existing object and is completed only if the
new object does not already exist.

WARNING: When using this mode, `copy_if_not_exists` does not copy tags
or attributes from the source object.

WARNING: When using this mode, `copy_if_not_exists` makes only a best
effort attempt to clean up the multipart upload if the copy operation
fails. Consider using a lifecycle rule to automatically clean up
abandoned multipart uploads. See [the module
docs](super#multipart-uploads) for details.

Encoded as `multipart` ignoring whitespace.

<a id="op-36c06371b8186d9442b45115"></a>
## clone

`function` · `object_store::aws::precondition::S3CopyIfNotExists::clone` · object_store 0.13.2

```rust
fn clone(&self) -> S3CopyIfNotExists
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3CopyIfNotExists", "path": "S3CopyIfNotExists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aws/precondition.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb97744be9e101e070381ef6"></a>
## eq

`function` · `object_store::aws::precondition::S3CopyIfNotExists::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &S3CopyIfNotExists) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3CopyIfNotExists", "path": "S3CopyIfNotExists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 24], "end": [26, 33], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aws/precondition.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-539cb10d63725ef681553643"></a>
## fmt

`function` · `object_store::aws::precondition::S3CopyIfNotExists::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3CopyIfNotExists", "path": "S3CopyIfNotExists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [75, 2], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/aws/precondition.rs:66`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b88187c8812c551142c2443d"></a>
## fmt

`function` · `object_store::aws::precondition::S3CopyIfNotExists::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::precondition::S3CopyIfNotExists", "path": "S3CopyIfNotExists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/aws/precondition.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/precondition.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
