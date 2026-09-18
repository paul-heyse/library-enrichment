# `object_store::gcp::builder::GoogleConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.builder.GoogleConfigKey.json).

<a id="op-9fd263966312a6a803e3e8a8"></a>
## GoogleConfigKey

`enum` · `object_store::gcp::builder::GoogleConfigKey` · object_store 0.13.2

```rust
enum GoogleConfigKey
```

Source: `src/gcp/builder.rs:136`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configuration keys for [`GoogleCloudStorageBuilder`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-c4745fb4786b89c0efb113fa)

Configuration via keys can be done via [`GoogleCloudStorageBuilder::with_config`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-3a5ff47849540dde6aae136b)

# Example
```
# use object_store::gcp::{GoogleCloudStorageBuilder, GoogleConfigKey};
let builder = GoogleCloudStorageBuilder::new()
    .with_config("google_service_account".parse().unwrap(), "my-service-account")
    .with_config(GoogleConfigKey::Bucket, "my-bucket");
```

<a id="op-8b95b91d594434c7a89153a7"></a>
## ApplicationCredentials

`variant` · `object_store::gcp::builder::GoogleConfigKey::ApplicationCredentials` · object_store 0.13.2

```rust
ApplicationCredentials
```

Source: `src/gcp/builder.rs:180`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Application credentials path

See [`GoogleCloudStorageBuilder::with_application_credentials`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-2947c8395874870ee19232f5).

Supported keys:
- `google_application_credentials`
- `application_credentials`

<a id="op-2c4c8f05939e077dc198d180"></a>
## BaseUrl

`variant` · `object_store::gcp::builder::GoogleConfigKey::BaseUrl` · object_store 0.13.2

```rust
BaseUrl
```

Source: `src/gcp/builder.rs:171`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Base URL

See [`GoogleCloudStorageBuilder::with_base_url`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-52470cbeba245318a6a453d0) for details.

Supported keys:
- `google_base_url`
- `base_url`

<a id="op-615720f66ba3350d8a750e79"></a>
## Bucket

`variant` · `object_store::gcp::builder::GoogleConfigKey::Bucket` · object_store 0.13.2

```rust
Bucket
```

Source: `src/gcp/builder.rs:162`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Bucket name

See [`GoogleCloudStorageBuilder::with_bucket_name`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-9c7f1b50df6a2ec62a4b9c71) for details.

Supported keys:
- `google_bucket`
- `google_bucket_name`
- `bucket`
- `bucket_name`

<a id="op-3905afa359f10548c0af4928"></a>
## Client

`variant` · `object_store::gcp::builder::GoogleConfigKey::Client` · object_store 0.13.2

```rust
Client
```

Source: `src/gcp/builder.rs:190`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Client options

<a id="op-1b664a2dc5274e41c44071e4"></a>
## Err

`assoc_type` · `object_store::gcp::builder::GoogleConfigKey::Err` · object_store 0.13.2

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [229, 2], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/gcp/builder.rs:208`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8238b98d83ed0cd5338063f0"></a>
## ServiceAccount

`variant` · `object_store::gcp::builder::GoogleConfigKey::ServiceAccount` · object_store 0.13.2

```rust
ServiceAccount
```

Source: `src/gcp/builder.rs:144`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Path to the service account file

Supported keys:
- `google_service_account`
- `service_account`
- `google_service_account_path`
- `service_account_path`

<a id="op-d9667d0482c989706cf7c10b"></a>
## ServiceAccountKey

`variant` · `object_store::gcp::builder::GoogleConfigKey::ServiceAccountKey` · object_store 0.13.2

```rust
ServiceAccountKey
```

Source: `src/gcp/builder.rs:151`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The serialized service account key.

Supported keys:
- `google_service_account_key`
- `service_account_key`

<a id="op-75091963febdbc1387853a92"></a>
## SkipSignature

`variant` · `object_store::gcp::builder::GoogleConfigKey::SkipSignature` · object_store 0.13.2

```rust
SkipSignature
```

Source: `src/gcp/builder.rs:187`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Skip signing request

Supported keys:
- `google_skip_signature`
- `skip_signature`

<a id="op-f3ece5d662890f10ce2a38bf"></a>
## as_ref

`function` · `object_store::gcp::builder::GoogleConfigKey::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [205, 2], "filename": "src/gcp/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/gcp/builder.rs:194`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a87184ab495bb357cb110467"></a>
## clone

`function` · `object_store::gcp::builder::GoogleConfigKey::clone` · object_store 0.13.2

```rust
fn clone(&self) -> GoogleConfigKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 31], "end": [134, 36], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gcp/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d1b104273623538efd56a9a"></a>
## deserialize

`function` · `object_store::gcp::builder::GoogleConfigKey::deserialize` · object_store 0.13.2

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 62], "end": [134, 73], "filename": "src/gcp/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/gcp/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c867989dec6b1005dfefb966"></a>
## eq

`function` · `object_store::gcp::builder::GoogleConfigKey::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &GoogleConfigKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 10], "end": [134, 19], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gcp/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06fe0a83180996e093f3dc8e"></a>
## fmt

`function` · `object_store::gcp::builder::GoogleConfigKey::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 38], "end": [134, 43], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gcp/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-966ce929c6a0ca0296a8853c"></a>
## from_str

`function` · `object_store::gcp::builder::GoogleConfigKey::from_str` · object_store 0.13.2

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [229, 2], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/gcp/builder.rs:210`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d26f9c32da32b21be624ac7"></a>
## hash

`function` · `object_store::gcp::builder::GoogleConfigKey::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 25], "end": [134, 29], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/gcp/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60788e8aa417d0fcee79067a"></a>
## serialize

`function` · `object_store::gcp::builder::GoogleConfigKey::serialize` · object_store 0.13.2

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleConfigKey", "path": "GoogleConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 51], "end": [134, 60], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/gcp/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
