# `deltalake_catalog_glue::GlueError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_glue.GlueError.json).

<a id="op-692164a4e68c9dab1292308a"></a>
## GlueError

`enum` · `deltalake_catalog_glue::GlueError` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum GlueError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L7).

Source: `crates/catalog-glue/src/lib.rs:7`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e80a953200215e073e268440"></a>
## AWSError

`variant` · `deltalake_catalog_glue::GlueError::AWSError` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AWSError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L17).

Source: `crates/catalog-glue/src/lib.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error calling the AWS SDK

<a id="op-2b9dce9ae572b0577eec9a89"></a>
## MissingMetadata

`variant` · `deltalake_catalog_glue::GlueError::MissingMetadata` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L10).

Source: `crates/catalog-glue/src/lib.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Missing metadata in the catalog

<a id="op-25a7399dff4de81df98090ad"></a>
## fmt

`function` · `deltalake_catalog_glue::GlueError::fmt` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L6).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueError", "path": "GlueError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 28], "end": [6, 33], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-glue/src/lib.rs:6`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bdb44407f61e4aa7a38624b"></a>
## fmt

`function` · `deltalake_catalog_glue::GlueError::fmt` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L6).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueError", "path": "GlueError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 10], "end": [6, 26], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/catalog-glue/src/lib.rs:6`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dabe85d4a07adcc21735c59f"></a>
## from

`function` · `deltalake_catalog_glue::GlueError::from` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: aws_sdk_glue::Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L6).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueError", "path": "GlueError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [18, 9], "end": [18, 16], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "aws_sdk_glue::error_meta::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/catalog-glue/src/lib.rs:6`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94716a65dc2545a981fef092"></a>
## source

`function` · `deltalake_catalog_glue::GlueError::source` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L6).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueError", "path": "GlueError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 10], "end": [6, 26], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/catalog-glue/src/lib.rs:6`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
