# `object_store::azure::builder::MicrosoftAzureBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.builder.MicrosoftAzureBuilder.json).

<a id="op-448e2696c354c14c06e02e5c"></a>
## MicrosoftAzureBuilder

`struct` · `object_store::azure::builder::MicrosoftAzureBuilder` · object_store 0.13.2

```rust
struct MicrosoftAzureBuilder
```

Source: `src/azure/builder.rs:122`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure a connection to Microsoft Azure Blob Storage container using
the specified credentials.

# Example
```
# let ACCOUNT = "foo";
# let BUCKET_NAME = "foo";
# let ACCESS_KEY = "foo";
# use object_store::azure::MicrosoftAzureBuilder;
let azure = MicrosoftAzureBuilder::new()
 .with_account(ACCOUNT)
 .with_access_key(ACCESS_KEY)
 .with_container_name(BUCKET_NAME)
 .build();
```

<a id="op-c2f5a5c9e8666aa91899228f"></a>
## build

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::build` · object_store 0.13.2

```rust
fn build(self) -> Result<MicrosoftAzure>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:910`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure a connection to container with given name on Microsoft Azure Blob store.

<a id="op-490f2c8042d786ccfdd63b0b"></a>
## clone

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::clone` · object_store 0.13.2

```rust
fn clone(&self) -> MicrosoftAzureBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 19], "end": [121, 24], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/azure/builder.rs:121`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15904344e41079771f61eb4c"></a>
## default

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::default` · object_store 0.13.2

```rust
fn default() -> MicrosoftAzureBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 17], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/azure/builder.rs:121`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d07120ea43332ffabbf0368"></a>
## fmt

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [487, 2], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/azure/builder.rs:480`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34fc06dae9280e36733887cf"></a>
## from_env

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::from_env` · object_store 0.13.2

```rust
fn from_env() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:512`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create an instance of [`MicrosoftAzureBuilder`](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md#op-448e2696c354c14c06e02e5c) with values pre-populated from environment variables.

Variables extracted from environment:
* AZURE_STORAGE_ACCOUNT_NAME: storage account name
* AZURE_STORAGE_ACCOUNT_KEY: storage account master key
* AZURE_STORAGE_ACCESS_KEY: alias for AZURE_STORAGE_ACCOUNT_KEY
* AZURE_STORAGE_CLIENT_ID -> client id for service principal authorization
* AZURE_STORAGE_CLIENT_SECRET -> client secret for service principal authorization
* AZURE_STORAGE_TENANT_ID -> tenant id used in oauth flows
# Example
```
use object_store::azure::MicrosoftAzureBuilder;

let azure = MicrosoftAzureBuilder::from_env()
    .with_container_name("foo")
    .build();
```

<a id="op-b0e160dd579f21c2fa9aba7b"></a>
## get_config_value

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::get_config_value` · object_store 0.13.2

```rust
fn get_config_value(&self, key: &AzureConfigKey) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:612`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get config value via a [`AzureConfigKey`](../operations/object_store.azure.builder.AzureConfigKey.md#op-aa00d8de7bc841f29f90ac9d).

# Example
```
use object_store::azure::{MicrosoftAzureBuilder, AzureConfigKey};

let builder = MicrosoftAzureBuilder::from_env()
    .with_account("foo");
let account_name = builder.get_config_value(&AzureConfigKey::AccountName).unwrap_or_default();
assert_eq!("foo", &account_name);
```

<a id="op-7e51d54f2438b5c0390672f0"></a>
## new

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:491`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`MicrosoftAzureBuilder`](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md#op-448e2696c354c14c06e02e5c) with default values.

<a id="op-76367eb9f342620879054be2"></a>
## with_access_key

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_access_key` · object_store 0.13.2

```rust
fn with_access_key(self, access_key: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:731`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the Azure Access Key (required - one of access key, bearer token, or client credentials)

<a id="op-595237885d619087d0ee276f"></a>
## with_account

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_account` · object_store 0.13.2

```rust
fn with_account(self, account: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:719`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the Azure Account (required)

<a id="op-2a19142a769bd2c9e2d27652"></a>
## with_allow_http

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_allow_http` · object_store 0.13.2

```rust
fn with_allow_http(self, allow_http: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:818`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets what protocol is allowed

If `allow_http` is :
* false (default):  Only HTTPS are allowed
* true:  HTTP and HTTPS are allowed

<a id="op-405c6edc073601057c11b2a0"></a>
## with_authority_host

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_authority_host` · object_store 0.13.2

```rust
fn with_authority_host(self, authority_host: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:828`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets an alternative authority host for OAuth based authorization

Common hosts for azure clouds are defined in [authority_hosts](crate::azure::authority_hosts).

Defaults to <https://login.microsoftonline.com>

<a id="op-ff1d4513c9529d6d3e0f5508"></a>
## with_bearer_token_authorization

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_bearer_token_authorization` · object_store 0.13.2

```rust
fn with_bearer_token_authorization(self, bearer_token: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:737`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a static bearer token to be used for authorizing requests

<a id="op-c5f2204a034bf3965ab501e1"></a>
## with_client_id

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_client_id` · object_store 0.13.2

```rust
fn with_client_id(self, client_id: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:756`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the client id for use in client secret or k8s federated credential flow

<a id="op-f5d27fa257abfc4007e66e02"></a>
## with_client_options

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_client_options` · object_store 0.13.2

```rust
fn with_client_options(self, options: ClientOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:860`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the client options, overriding any already set

<a id="op-71f3b195e370691ff4274400"></a>
## with_client_secret

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_client_secret` · object_store 0.13.2

```rust
fn with_client_secret(self, client_secret: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:762`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the client secret for use in client secret flow

<a id="op-bfbbabac8a788b9dcc1e4c3c"></a>
## with_client_secret_authorization

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_client_secret_authorization` · object_store 0.13.2

```rust
fn with_client_secret_authorization(self, client_id: impl Into<String>, client_secret: impl Into<String>, tenant_id: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:743`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a client secret used for client secret authorization

<a id="op-7cd7fb1ba99bc0b236e85d94"></a>
## with_config

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_config` · object_store 0.13.2

```rust
fn with_config(self, key: AzureConfigKey, value: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:565`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set an option on the builder via a key - value pair.

<a id="op-125141d5adb6be39d917d81b"></a>
## with_container_name

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_container_name` · object_store 0.13.2

```rust
fn with_container_name(self, container_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:725`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the Azure Container Name (required)

<a id="op-2e4705b1dd040a4a8f3467dc"></a>
## with_credentials

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_credentials` · object_store 0.13.2

```rust
fn with_credentials(self, credentials: AzureCredentialProvider) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:780`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the credential provider overriding any other options

<a id="op-3131a914a17bb08ed49ef66c"></a>
## with_disable_tagging

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_disable_tagging` · object_store 0.13.2

```rust
fn with_disable_tagging(self, ignore: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:896`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If set to `true` will ignore any tags provided to put_opts

<a id="op-d9279060bc83aa65215c49ff"></a>
## with_endpoint

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_endpoint` · object_store 0.13.2

```rust
fn with_endpoint(self, endpoint: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:797`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Override the endpoint used to communicate with blob storage

Defaults to `https://{account}.blob.core.windows.net`

By default, only HTTPS schemes are enabled. To connect to an HTTP endpoint, enable
[`Self::with_allow_http`](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md#op-2a19142a769bd2c9e2d27652).

<a id="op-0218686671096b58d1e77a7f"></a>
## with_federated_token_file

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_federated_token_file` · object_store 0.13.2

```rust
fn with_federated_token_file(self, federated_token_file: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:874`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets a file path for acquiring azure federated identity token in k8s

requires `client_id` and `tenant_id` to be set

<a id="op-7547cb956fa4170b18a8a88a"></a>
## with_http_connector

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_http_connector` · object_store 0.13.2

```rust
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:904`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`HttpConnector`](../operations/object_store.client.http.connection.HttpConnector.md#op-04b72af01cc3e9636d12c25b) to use

On non-WASM32 platforms uses [`reqwest`] by default, on WASM32 platforms must be provided

Unresolved upstream links (retained, not inferred): ``reqwest``.

<a id="op-f49ce22d4530723c85f5b43a"></a>
## with_msi_endpoint

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_msi_endpoint` · object_store 0.13.2

```rust
fn with_msi_endpoint(self, msi_endpoint: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:866`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the endpoint for acquiring managed identity token

<a id="op-de67d5df07aba3267ad8f6c6"></a>
## with_proxy_ca_certificate

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_proxy_ca_certificate` · object_store 0.13.2

```rust
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:846`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a trusted proxy CA certificate

<a id="op-775656a2fe67242c47cecfdd"></a>
## with_proxy_excludes

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_proxy_excludes` · object_store 0.13.2

```rust
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:854`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a list of hosts to exclude from proxy connections

<a id="op-050f0f5222c60614be16a545"></a>
## with_proxy_url

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_proxy_url` · object_store 0.13.2

```rust
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:840`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the proxy_url to be used by the underlying client

<a id="op-06b9b1e578c353f7285a3348"></a>
## with_retry

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_retry` · object_store 0.13.2

```rust
fn with_retry(self, retry_config: RetryConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:834`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the retry configuration

<a id="op-07e514d909ca9c29565100a4"></a>
## with_sas_authorization

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_sas_authorization` · object_store 0.13.2

```rust
fn with_sas_authorization(self, query_pairs: impl Into<Vec<(String, String)>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:774`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set query pairs appended to the url for shared access signature authorization

<a id="op-e2bd8763cfe0fcd9ba63b26f"></a>
## with_skip_signature

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_skip_signature` · object_store 0.13.2

```rust
fn with_skip_signature(self, skip_signature: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:890`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If enabled, [`MicrosoftAzure`](../operations/object_store.azure.MicrosoftAzure.md#op-a50607143b9beb78c537c27a) will not fetch credentials and will not sign requests

This can be useful when interacting with public containers

<a id="op-3ff3772c827337444d601627"></a>
## with_tenant_id

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_tenant_id` · object_store 0.13.2

```rust
fn with_tenant_id(self, tenant_id: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:768`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the tenant id for use in client secret or k8s federated credential flow

<a id="op-43e71137b8a0beee09263696"></a>
## with_url

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_url` · object_store 0.13.2

```rust
fn with_url(self, url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:559`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `abfs[s]://<container>/<path>` (according to [fsspec](https://github.com/fsspec/adlfs))
- `abfs[s]://<file_system>@<account_name>.dfs.core.windows.net/<path>`
- `abfs[s]://<file_system>@<account_name>.dfs.fabric.microsoft.com/<path>`
- `az://<container>/<path>` (according to [fsspec](https://github.com/fsspec/adlfs))
- `adl://<container>/<path>` (according to [fsspec](https://github.com/fsspec/adlfs))
- `azure://<container>/<path>` (custom)
- `https://<account>.dfs.core.windows.net`
- `https://<account>.blob.core.windows.net`
- `https://<account>.blob.core.windows.net/<container>`
- `https://<account>.dfs.fabric.microsoft.com`
- `https://<account>.dfs.fabric.microsoft.com/<container>`
- `https://<account>.blob.fabric.microsoft.com`
- `https://<account>.blob.fabric.microsoft.com/<container>`

Note: Settings derived from the URL will override any others set on this builder

# Example
```
use object_store::azure::MicrosoftAzureBuilder;

let azure = MicrosoftAzureBuilder::from_env()
    .with_url("abfss://file_system@account.dfs.core.windows.net/")
    .build();
```

<a id="op-63f19e74e05d1c6055ad87c3"></a>
## with_use_azure_cli

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_use_azure_cli` · object_store 0.13.2

```rust
fn with_use_azure_cli(self, use_azure_cli: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:882`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set if the Azure Cli should be used for acquiring access token

<https://learn.microsoft.com/en-us/cli/azure/account?view=azure-cli-latest#az-account-get-access-token>

<a id="op-2e893d267184e5367e4f4c35"></a>
## with_use_emulator

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_use_emulator` · object_store 0.13.2

```rust
fn with_use_emulator(self, use_emulator: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:786`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set if the Azure emulator should be used (defaults to false)

<a id="op-b4e77f292df98e425a3c91b1"></a>
## with_use_fabric_endpoint

`function` · `object_store::azure::builder::MicrosoftAzureBuilder::with_use_fabric_endpoint` · object_store 0.13.2

```rust
fn with_use_fabric_endpoint(self, use_fabric_endpoint: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::MicrosoftAzureBuilder", "path": "MicrosoftAzureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [1059, 2], "filename": "src/azure/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/builder.rs:808`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set if Microsoft Fabric url scheme should be used (defaults to false)

When disabled the url scheme used is `https://{account}.blob.core.windows.net`
When enabled the url scheme used is `https://{account}.dfs.fabric.microsoft.com`

Note: [`Self::with_endpoint`](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md#op-d9279060bc83aa65215c49ff) will take precedence over this option
