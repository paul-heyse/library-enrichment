# `deltalake_catalog_unity::models::TemporaryTableCredentials`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.TemporaryTableCredentials.json).

<a id="op-80948fb00035868d8cea507c"></a>
## TemporaryTableCredentials

`struct` · `deltalake_catalog_unity::models::TemporaryTableCredentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TemporaryTableCredentials
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L434).

Source: `crates/catalog-unity/src/models.rs:434`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-042a1458455c95e28901ea45"></a>
## aws_temp_credentials

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentials::aws_temp_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
aws_temp_credentials: Option<AwsTempCredentials>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L435).

Source: `crates/catalog-unity/src/models.rs:435`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ec25a8bb507835252eff720"></a>
## azure_user_delegation_sas

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentials::azure_user_delegation_sas` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
azure_user_delegation_sas: Option<AzureUserDelegationSas>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L436).

Source: `crates/catalog-unity/src/models.rs:436`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49f2796ffd79d212678e76b0"></a>
## clone

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TemporaryTableCredentials
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L433).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 30], "end": [433, 35], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/models.rs:433`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a8706c98a7f3ff112856fb4"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L433).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 10], "end": [433, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:433`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9fbe29f9a2b421c94e9947f"></a>
## expiration_time

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentials::expiration_time` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
expiration_time: chrono::DateTime<chrono::Utc>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L440).

Source: `crates/catalog-unity/src/models.rs:440`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0a36d1655f892361b8c6694"></a>
## fmt

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L433).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 23], "end": [433, 28], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:433`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba72e481d1f0ac1e32e04471"></a>
## gcp_oauth_token

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentials::gcp_oauth_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
gcp_oauth_token: Option<GcpOauthToken>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L437).

Source: `crates/catalog-unity/src/models.rs:437`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60ae9270ec0c70d4562c8517"></a>
## get_aws_credentials

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::get_aws_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_aws_credentials(&self) -> Option<HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L453).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [502, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/models.rs:453`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96e02e336e804ddbfff96cf8"></a>
## get_azure_credentials

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::get_azure_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_azure_credentials(&self) -> Option<HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L464).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [502, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/models.rs:464`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2868d94d15fdf2ac4673c7b"></a>
## get_credentials

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::get_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_credentials(self) -> Option<HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L496).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [502, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/models.rs:496`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61ec5b902535cb8b4a8cc10b"></a>
## get_gcp_credentials

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::get_gcp_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_gcp_credentials(&self) -> Option<HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L475).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [502, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/models.rs:475`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3612a049069585891b2e49b"></a>
## get_r2_credentials

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentials::get_r2_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_r2_credentials(&self) -> Option<HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L486).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentials", "path": "TemporaryTableCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [502, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/models.rs:486`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d45705ab6a3199090660fbc"></a>
## r2_temp_credentials

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentials::r2_temp_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
r2_temp_credentials: Option<R2TempCredentials>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L438).

Source: `crates/catalog-unity/src/models.rs:438`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edbe292ced8df7640fcc821d"></a>
## url

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentials::url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
url: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L441).

Source: `crates/catalog-unity/src/models.rs:441`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
