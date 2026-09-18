# `deltalake_catalog_unity::client::retry::RetryError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.retry.RetryError.json).

<a id="op-3cdf66322be0f6938f094890"></a>
## RetryError

`struct` · `deltalake_catalog_unity::client::retry::RetryError` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RetryError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L11).

Source: `crates/catalog-unity/src/client/retry.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Retry request error

<a id="op-863f3d8ce440d16ff837b29b"></a>
## fmt

`function` · `deltalake_catalog_unity::client::retry::RetryError::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L10).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryError", "path": "RetryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10, 10], "end": [10, 15], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/retry.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bea34a973081d5002cf31f20"></a>
## fmt

`function` · `deltalake_catalog_unity::client::retry::RetryError::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L18).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryError", "path": "RetryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 1], "end": [29, 2], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/catalog-unity/src/client/retry.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-654aecd070132f3f8cafaca8"></a>
## source

`function` · `deltalake_catalog_unity::client::retry::RetryError::source` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryError", "path": "RetryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [35, 2], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/catalog-unity/src/client/retry.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfa5b375a3880e41e583a405"></a>
## status

`function` · `deltalake_catalog_unity::client::retry::RetryError::status` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn status(&self) -> Option<StatusCode>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::retry::RetryError", "path": "RetryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [42, 2], "filename": "crates/catalog-unity/src/client/retry.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/client/retry.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the status code associated with this error if any

<a id="op-f40d15b0cc3e22800a26a7b4"></a>
## message

`struct_field` · `deltalake_catalog_unity::client::retry::RetryError::message` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
message: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L13).

Source: `crates/catalog-unity/src/client/retry.rs:13`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e16a866a5123bcd1d6970d5b"></a>
## retries

`struct_field` · `deltalake_catalog_unity::client::retry::RetryError::retries` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
retries: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L12).

Source: `crates/catalog-unity/src/client/retry.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ff984524d7d4c3fb2fde5e1"></a>
## source

`struct_field` · `deltalake_catalog_unity::client::retry::RetryError::source` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Option<reqwest::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L14).

Source: `crates/catalog-unity/src/client/retry.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
