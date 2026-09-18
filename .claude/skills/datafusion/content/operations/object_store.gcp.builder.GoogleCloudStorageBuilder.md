# `object_store::gcp::builder::GoogleCloudStorageBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.builder.GoogleCloudStorageBuilder.json).

<a id="op-c4745fb4786b89c0efb113fa"></a>
## GoogleCloudStorageBuilder

`struct` · `object_store::gcp::builder::GoogleCloudStorageBuilder` · object_store 0.13.2

```rust
struct GoogleCloudStorageBuilder
```

Source: `src/gcp/builder.rs:96`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure a connection to Google Cloud Storage.

If no credentials are explicitly provided, they will be sourced
from the environment as documented [here](https://cloud.google.com/docs/authentication/application-default-credentials).

# Example
```
# let BUCKET_NAME = "foo";
# use object_store::gcp::GoogleCloudStorageBuilder;
let gcs = GoogleCloudStorageBuilder::from_env().with_bucket_name(BUCKET_NAME).build();
```

<a id="op-8adf969daf19b7b3875469f1"></a>
## build

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::build` · object_store 0.13.2

```rust
fn build(self) -> Result<GoogleCloudStorage>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:504`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure a connection to Google Cloud Storage, returning a
new [`GoogleCloudStorage`](../operations/object_store.gcp.GoogleCloudStorage.md#op-4b3bbb82170d36762c713ba6) and consuming `self`

<a id="op-c4de494254ae9c5461dc5a90"></a>
## clone

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::clone` · object_store 0.13.2

```rust
fn clone(&self) -> GoogleCloudStorageBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 17], "end": [95, 22], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gcp/builder.rs:95`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d66211bb9f9e31078db03e8"></a>
## default

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [248, 2], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gcp/builder.rs:232`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77e4490ee97343b6c1c646ad"></a>
## fmt

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "src/gcp/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gcp/builder.rs:95`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c510816100d1ca6fb49e99b"></a>
## from_env

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::from_env` · object_store 0.13.2

```rust
fn from_env() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:274`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create an instance of [`GoogleCloudStorageBuilder`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-c4745fb4786b89c0efb113fa) with values pre-populated from environment variables.

Variables extracted from environment:
* GOOGLE_SERVICE_ACCOUNT: location of service account file
* GOOGLE_SERVICE_ACCOUNT_PATH: (alias) location of service account file
* SERVICE_ACCOUNT: (alias) location of service account file
* GOOGLE_SERVICE_ACCOUNT_KEY: JSON serialized service account key
* GOOGLE_BUCKET: bucket name
* GOOGLE_BUCKET_NAME: (alias) bucket name

# Example
```
use object_store::gcp::GoogleCloudStorageBuilder;

let gcs = GoogleCloudStorageBuilder::from_env()
    .with_bucket_name("foo")
    .build();
```

<a id="op-7bf1d22cf9868dac57bd4bad"></a>
## get_config_value

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::get_config_value` · object_store 0.13.2

```rust
fn get_config_value(&self, key: &GoogleConfigKey) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:344`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get config value via a [`GoogleConfigKey`](../operations/object_store.gcp.builder.GoogleConfigKey.md#op-9fd263966312a6a803e3e8a8).

# Example
```
use object_store::gcp::{GoogleCloudStorageBuilder, GoogleConfigKey};

let builder = GoogleCloudStorageBuilder::from_env()
    .with_service_account_key("foo");
let service_account_key = builder.get_config_value(&GoogleConfigKey::ServiceAccountKey).unwrap_or_default();
assert_eq!("foo", &service_account_key);
```

<a id="op-427ab45a4de6e47640085d21"></a>
## new

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:252`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`GoogleCloudStorageBuilder`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-c4745fb4786b89c0efb113fa) with default values.

<a id="op-2947c8395874870ee19232f5"></a>
## with_application_credentials

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_application_credentials` · object_store 0.13.2

```rust
fn with_application_credentials(self, application_credentials_path: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:440`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the path to the application credentials file.

<https://cloud.google.com/docs/authentication/provide-credentials-adc>

<a id="op-52470cbeba245318a6a453d0"></a>
## with_base_url

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_base_url` · object_store 0.13.2

```rust
fn with_base_url(self, base_url: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:400`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the base URL for communicating with GCS.

If not explicitly set, it will be:
1. Derived from the service account credentials, if provided
2. Otherwise, uses the default GCS endpoint

# Example
```
use object_store::gcp::GoogleCloudStorageBuilder;

let gcs = GoogleCloudStorageBuilder::from_env()
    .with_base_url("https://localhost:4443")
    .build();
```

<a id="op-9c7f1b50df6a2ec62a4b9c71"></a>
## with_bucket_name

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_bucket_name` · object_store 0.13.2

```rust
fn with_bucket_name(self, bucket_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:381`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the bucket name (required)

<a id="op-14e74d74e2cde25aacbd83a4"></a>
## with_client_options

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_client_options` · object_store 0.13.2

```rust
fn with_client_options(self, options: ClientOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:489`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the client options, overriding any already set

<a id="op-3a5ff47849540dde6aae136b"></a>
## with_config

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_config` · object_store 0.13.2

```rust
fn with_config(self, key: GoogleConfigKey, value: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:316`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set an option on the builder via a key - value pair.

<a id="op-4b95749b3b79e295ec97243b"></a>
## with_credentials

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_credentials` · object_store 0.13.2

```rust
fn with_credentials(self, credentials: GcpCredentialProvider) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:457`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the credential provider overriding any other options

<a id="op-d94c97a89f6274e13ae44703"></a>
## with_http_connector

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_http_connector` · object_store 0.13.2

```rust
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:497`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`HttpConnector`](../operations/object_store.client.http.connection.HttpConnector.md#op-04b72af01cc3e9636d12c25b) to use

On non-WASM32 platforms uses [`reqwest`] by default, on WASM32 platforms must be provided

Unresolved upstream links (retained, not inferred): ``reqwest``.

<a id="op-9654fac7c1546489515428b8"></a>
## with_proxy_ca_certificate

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_proxy_ca_certificate` · object_store 0.13.2

```rust
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:475`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a trusted proxy CA certificate

<a id="op-d166f8737cfa9e01e5cbde89"></a>
## with_proxy_excludes

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_proxy_excludes` · object_store 0.13.2

```rust
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:483`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a list of hosts to exclude from proxy connections

<a id="op-1db8710be2d59a5bd80537a5"></a>
## with_proxy_url

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_proxy_url` · object_store 0.13.2

```rust
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:469`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the proxy_url to be used by the underlying client

<a id="op-6b053cfe3387f461195f29b8"></a>
## with_retry

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_retry` · object_store 0.13.2

```rust
fn with_retry(self, retry_config: RetryConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:463`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the retry configuration

<a id="op-88d5f961eb5688f32843d747"></a>
## with_service_account_key

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_service_account_key` · object_store 0.13.2

```rust
fn with_service_account_key(self, service_account: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:432`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the service account key. The service account must be in the JSON
format.

This or [`GoogleCloudStorageBuilder::with_service_account_path`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-8d813028c4db6a8a2800c0cf) must be
set.

<a id="op-8d813028c4db6a8a2800c0cf"></a>
## with_service_account_path

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_service_account_path` · object_store 0.13.2

```rust
fn with_service_account_path(self, service_account_path: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:422`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the path to the service account file.

This or [`GoogleCloudStorageBuilder::with_service_account_key`](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md#op-88d5f961eb5688f32843d747) must be
set.

Example `"/tmp/gcs.json"`.

Example contents of `gcs.json`:

```json
{
   "gcs_base_url": "https://localhost:4443",
   "disable_oauth": true,
   "client_email": "",
   "private_key": ""
}
```

<a id="op-f97b86af55a8ad6c6eaf4bd2"></a>
## with_skip_signature

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_skip_signature` · object_store 0.13.2

```rust
fn with_skip_signature(self, skip_signature: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:451`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If enabled, [`GoogleCloudStorage`](../operations/object_store.gcp.GoogleCloudStorage.md#op-4b3bbb82170d36762c713ba6) will not fetch credentials and will not sign requests.

This can be useful when interacting with public GCS buckets that deny authorized requests.

<a id="op-dc4197bf5051bb70f1c420c5"></a>
## with_url

`function` · `object_store::gcp::builder::GoogleCloudStorageBuilder::with_url` · object_store 0.13.2

```rust
fn with_url(self, url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::builder::GoogleCloudStorageBuilder", "path": "GoogleCloudStorageBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [639, 2], "filename": "src/gcp/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/builder.rs:310`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `gs://<bucket>/<path>`

Note: Settings derived from the URL will override any others set on this builder

# Example
```
use object_store::gcp::GoogleCloudStorageBuilder;

let gcs = GoogleCloudStorageBuilder::from_env()
    .with_url("gs://bucket/path")
    .build();
```
