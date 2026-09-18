# `object_store::gcp::builder`

Crate `object_store` · 2 public items · structured records in [`model/object_store.gcp.builder.json`](../model/object_store.gcp.builder.json)

## GoogleConfigKey

`enum` · `object_store::gcp::builder::GoogleConfigKey`

Also reachable as `object_store::gcp::GoogleConfigKey`

```rust
enum GoogleConfigKey
```

**Variants**: `ServiceAccount`, `ServiceAccountKey`, `Bucket`, `BaseUrl`, `ApplicationCredentials`, `SkipSignature`, `Client`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/object_store.gcp.builder.GoogleConfigKey.md).


Configuration keys for [`GoogleCloudStorageBuilder`]

Configuration via keys can be done via [`GoogleCloudStorageBuilder::with_config`]

# Example
```
# use object_store::gcp::{GoogleCloudStorageBuilder, GoogleConfigKey};
let builder = GoogleCloudStorageBuilder::new()
    .with_config("google_service_account".parse().unwrap(), "my-service-account")
    .with_config(GoogleConfigKey::Bucket, "my-bucket");
```

---

## GoogleCloudStorageBuilder

`struct` · `object_store::gcp::builder::GoogleCloudStorageBuilder`

Also reachable as `object_store::gcp::GoogleCloudStorageBuilder`

```rust
struct GoogleCloudStorageBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (19)

```rust
fn build(self) -> Result<GoogleCloudStorage>
fn from_env() -> Self
fn get_config_value(&self, key: &GoogleConfigKey) -> Option<String>
fn new() -> Self
fn with_application_credentials(self, application_credentials_path: impl Into<String>) -> Self
fn with_base_url(self, base_url: &str) -> Self
fn with_bucket_name(self, bucket_name: impl Into<String>) -> Self
fn with_client_options(self, options: ClientOptions) -> Self
fn with_config(self, key: GoogleConfigKey, value: impl Into<String>) -> Self
fn with_credentials(self, credentials: GcpCredentialProvider) -> Self
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
fn with_retry(self, retry_config: RetryConfig) -> Self
fn with_service_account_key(self, service_account: impl Into<String>) -> Self
fn with_service_account_path(self, service_account_path: impl Into<String>) -> Self
fn with_skip_signature(self, skip_signature: bool) -> Self
fn with_url(self, url: impl Into<String>) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.gcp.builder.GoogleCloudStorageBuilder.md).


Configure a connection to Google Cloud Storage.

If no credentials are explicitly provided, they will be sourced
from the environment as documented [here](https://cloud.google.com/docs/authentication/application-default-credentials).

# Example
```
# let BUCKET_NAME = "foo";
# use object_store::gcp::GoogleCloudStorageBuilder;
let gcs = GoogleCloudStorageBuilder::from_env().with_bucket_name(BUCKET_NAME).build();
```

---
