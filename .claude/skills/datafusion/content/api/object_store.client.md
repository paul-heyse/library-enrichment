# `object_store::client`

Crate `object_store` · 5 public items · structured records in [`model/object_store.client.json`](../model/object_store.client.json)

## ClientConfigKey

`enum` · `object_store::client::ClientConfigKey`

Also reachable as `datafusion::object_store::ClientConfigKey`, `object_store::ClientConfigKey`

```rust
enum ClientConfigKey
```

**Variants**: `AllowHttp`, `AllowInvalidCertificates`, `ConnectTimeout`, `DefaultContentType`, `Http1Only`, `Http2KeepAliveInterval`, `Http2KeepAliveTimeout`, `Http2KeepAliveWhileIdle`, `Http2MaxFrameSize`, `Http2Only`, `PoolIdleTimeout`, `PoolMaxIdlePerHost`, `ProxyUrl`, `ProxyCaCertificate`, `ProxyExcludes`, `RandomizeAddresses`, `Timeout`, `UserAgent`

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

[Full member, field, variant and typed contracts](../operations/object_store.client.ClientConfigKey.md).


Configuration keys for [`ClientOptions`]

---

## Certificate

`struct` · `object_store::client::Certificate`

Also reachable as `datafusion::object_store::Certificate`, `object_store::Certificate`

```rust
struct Certificate
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn from_der(der: &[u8]) -> Result<Self>
fn from_pem(pem: &[u8]) -> Result<Self>
fn from_pem_bundle(pem_bundle: &[u8]) -> Result<Vec<Self>>
```

[Full member, field, variant and typed contracts](../operations/object_store.client.Certificate.md).


Represents a CA certificate provided by the user.

This is used to configure the client to trust a specific certificate. See
[Self::from_pem] for an example

---

## ClientOptions

`struct` · `object_store::client::ClientOptions`

Also reachable as `datafusion::object_store::ClientOptions`, `object_store::ClientOptions`

```rust
struct ClientOptions
```

**Derives**: Clone, Debug, Default

**Methods** (28)

```rust
fn get_config_value(&self, key: &ClientConfigKey) -> Option<String>
fn get_content_type(&self, path: &Path) -> Option<&str>
fn get_default_headers(&self) -> Option<&HeaderMap>
fn new() -> Self
fn with_allow_http(self, allow_http: bool) -> Self
fn with_allow_http2(self) -> Self
fn with_allow_invalid_certificates(self, allow_insecure: bool) -> Self
fn with_config(self, key: ClientConfigKey, value: impl Into<String>) -> Self
fn with_connect_timeout(self, timeout: Duration) -> Self
fn with_connect_timeout_disabled(self) -> Self
fn with_content_type_for_suffix(self, extension: impl Into<String>, mime: impl Into<String>) -> Self
fn with_default_content_type(self, mime: impl Into<String>) -> Self
fn with_default_headers(self, headers: HeaderMap) -> Self
fn with_http1_only(self) -> Self
fn with_http2_keep_alive_interval(self, interval: Duration) -> Self
fn with_http2_keep_alive_timeout(self, interval: Duration) -> Self
fn with_http2_keep_alive_while_idle(self) -> Self
fn with_http2_max_frame_size(self, sz: u32) -> Self
fn with_http2_only(self) -> Self
fn with_pool_idle_timeout(self, timeout: Duration) -> Self
fn with_pool_max_idle_per_host(self, max: usize) -> Self
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
fn with_root_certificate(self, certificate: Certificate) -> Self
fn with_timeout(self, timeout: Duration) -> Self
fn with_timeout_disabled(self) -> Self
fn with_user_agent(self, agent: HeaderValue) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.client.ClientOptions.md).


HTTP client configuration for remote object stores

---

## StaticCredentialProvider

`struct` · `object_store::client::StaticCredentialProvider`

Also reachable as `datafusion::object_store::StaticCredentialProvider`, `object_store::StaticCredentialProvider`

```rust
struct StaticCredentialProvider<T>
```

**Implements**: `object_store::client::CredentialProvider`

**Derives**: Debug

**Methods** (1)

```rust
fn new(credential: T) -> Self
```

**via `object_store::client::CredentialProvider`**

```rust
async fn get_credential(&self) -> Result<Arc<T>>
```

[Full member, field, variant and typed contracts](../operations/object_store.client.StaticCredentialProvider.md).


A static set of credentials

---

## CredentialProvider

`trait` · `object_store::client::CredentialProvider`

Also reachable as `datafusion::object_store::CredentialProvider`, `object_store::CredentialProvider`

```rust
trait CredentialProvider: std::fmt::Debug + Send + Sync
```

**Implementors** (1)

- `object_store::client::StaticCredentialProvider`

**Methods** (1)

```rust
async fn get_credential(&self) -> Result<Arc<Self::Credential>>
```

[Full member, field, variant and typed contracts](../operations/object_store.client.CredentialProvider.md).


Provides credentials for use when signing requests

---
