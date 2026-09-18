# `object_store::client::ClientConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.ClientConfigKey.json).

<a id="op-c6d3249f9e3bcf965b3fcc79"></a>
## ClientConfigKey

`enum` · `object_store::client::ClientConfigKey` · object_store 0.13.2

```rust
enum ClientConfigKey
```

Source: `src/client/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configuration keys for [`ClientOptions`](../operations/object_store.client.ClientOptions.md#op-9357d27ab9e13340e3ccb12f)

<a id="op-fbfd52d0c3870e60fd138978"></a>
## AllowHttp

`variant` · `object_store::client::ClientConfigKey::AllowHttp` · object_store 0.13.2

```rust
AllowHttp
```

Source: `src/client/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Allow non-TLS, i.e. non-HTTPS connections

Supported keys:
- `allow_http`

<a id="op-0c566e913c24954ea67d32af"></a>
## AllowInvalidCertificates

`variant` · `object_store::client::ClientConfigKey::AllowInvalidCertificates` · object_store 0.13.2

```rust
AllowInvalidCertificates
```

Source: `src/client/mod.rs:105`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Skip certificate validation on https connections.

<div class="warning">

**Warning**

You should think very carefully before using this method. If
invalid certificates are trusted, *any* certificate for *any* site
will be trusted for use. This includes expired certificates. This
introduces significant vulnerabilities, and should only be used
as a last resort or for testing

</div>

Supported keys:
- `allow_invalid_certificates`

<a id="op-8c68c738ae8360d6aa39b875"></a>
## ConnectTimeout

`variant` · `object_store::client::ClientConfigKey::ConnectTimeout` · object_store 0.13.2

```rust
ConnectTimeout
```

Source: `src/client/mod.rs:110`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Timeout for only the connect phase of a Client

Supported keys:
- `connect_timeout`

<a id="op-cf0f66ab4888201241b17fa0"></a>
## DefaultContentType

`variant` · `object_store::client::ClientConfigKey::DefaultContentType` · object_store 0.13.2

```rust
DefaultContentType
```

Source: `src/client/mod.rs:115`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

default [`Content-Type`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type) for uploads

Supported keys:
- `default_content_type`

<a id="op-e37fa032ca5860230c81b4c0"></a>
## Err

`assoc_type` · `object_store::client::ClientConfigKey::Err` · object_store 0.13.2

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [257, 2], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/client/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae7a0c48c4e1eb5f71a29bd0"></a>
## Http1Only

`variant` · `object_store::client::ClientConfigKey::Http1Only` · object_store 0.13.2

```rust
Http1Only
```

Source: `src/client/mod.rs:120`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Only use HTTP/1 connections

Supported keys:
- `http1_only`

<a id="op-e182c3ee6088be6196bd021a"></a>
## Http2KeepAliveInterval

`variant` · `object_store::client::ClientConfigKey::Http2KeepAliveInterval` · object_store 0.13.2

```rust
Http2KeepAliveInterval
```

Source: `src/client/mod.rs:125`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Interval for HTTP/2 Ping frames should be sent to keep a connection alive.

Supported keys:
- `http2_keep_alive_interval`

<a id="op-cbd039a3651d1311eef4bf1c"></a>
## Http2KeepAliveTimeout

`variant` · `object_store::client::ClientConfigKey::Http2KeepAliveTimeout` · object_store 0.13.2

```rust
Http2KeepAliveTimeout
```

Source: `src/client/mod.rs:130`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Timeout for receiving an acknowledgement of the keep-alive ping.

Supported keys:
- `http2_keep_alive_timeout`

<a id="op-81205fb121cd9e2991c06931"></a>
## Http2KeepAliveWhileIdle

`variant` · `object_store::client::ClientConfigKey::Http2KeepAliveWhileIdle` · object_store 0.13.2

```rust
Http2KeepAliveWhileIdle
```

Source: `src/client/mod.rs:135`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Enable HTTP/2 keep alive pings for idle connections

Supported keys:
- `http2_keep_alive_while_idle`

<a id="op-0e4658387fcbd7492cadd6c5"></a>
## Http2MaxFrameSize

`variant` · `object_store::client::ClientConfigKey::Http2MaxFrameSize` · object_store 0.13.2

```rust
Http2MaxFrameSize
```

Source: `src/client/mod.rs:140`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the maximum frame size to use for HTTP/2.

Supported keys:
- `http2_max_frame_size`

<a id="op-68e0616d6c6c29399165ba9e"></a>
## Http2Only

`variant` · `object_store::client::ClientConfigKey::Http2Only` · object_store 0.13.2

```rust
Http2Only
```

Source: `src/client/mod.rs:145`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Only use HTTP/2 connections

Supported keys:
- `http2_only`

<a id="op-54d7f3608b08dad8473c85e0"></a>
## PoolIdleTimeout

`variant` · `object_store::client::ClientConfigKey::PoolIdleTimeout` · object_store 0.13.2

```rust
PoolIdleTimeout
```

Source: `src/client/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The pool max idle timeout

This is the length of time an idle connection will be kept alive

Supported keys:
- `pool_idle_timeout`

<a id="op-e484709b578033f39e5f737c"></a>
## PoolMaxIdlePerHost

`variant` · `object_store::client::ClientConfigKey::PoolMaxIdlePerHost` · object_store 0.13.2

```rust
PoolMaxIdlePerHost
```

Source: `src/client/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

maximum number of idle connections per host

Supported keys:
- `pool_max_idle_per_host`

<a id="op-d523a040136eabcf8b22526b"></a>
## ProxyCaCertificate

`variant` · `object_store::client::ClientConfigKey::ProxyCaCertificate` · object_store 0.13.2

```rust
ProxyCaCertificate
```

Source: `src/client/mod.rs:167`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

PEM-formatted CA certificate for proxy connections

Supported keys:
- `proxy_ca_certificate`

<a id="op-ade52fc9e7649cff2fd2d0e6"></a>
## ProxyExcludes

`variant` · `object_store::client::ClientConfigKey::ProxyExcludes` · object_store 0.13.2

```rust
ProxyExcludes
```

Source: `src/client/mod.rs:172`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

List of hosts that bypass proxy

Supported keys:
- `proxy_excludes`

<a id="op-357876c113a90c34cf3d39dd"></a>
## ProxyUrl

`variant` · `object_store::client::ClientConfigKey::ProxyUrl` · object_store 0.13.2

```rust
ProxyUrl
```

Source: `src/client/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

HTTP proxy to use for requests

Supported keys:
- `proxy_url`

<a id="op-c220cbc2b5b9d6580ee14416"></a>
## RandomizeAddresses

`variant` · `object_store::client::ClientConfigKey::RandomizeAddresses` · object_store 0.13.2

```rust
RandomizeAddresses
```

Source: `src/client/mod.rs:187`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Randomize order addresses that the DNS resolution yields.

This will spread the connections across more servers.

<div class="warning">

**Warning**

This will override the DNS resolver configured by [`reqwest`].

</div>

Supported keys:
- `randomize_addresses`

Unresolved upstream links (retained, not inferred): ``reqwest``.

<a id="op-9c0de1113c543bfa49184014"></a>
## Timeout

`variant` · `object_store::client::ClientConfigKey::Timeout` · object_store 0.13.2

```rust
Timeout
```

Source: `src/client/mod.rs:195`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request timeout

The timeout is applied from when the request starts connecting until the
response body has finished

Supported keys:
- `timeout`

<a id="op-b8f64460d863019edb13c88f"></a>
## UserAgent

`variant` · `object_store::client::ClientConfigKey::UserAgent` · object_store 0.13.2

```rust
UserAgent
```

Source: `src/client/mod.rs:200`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

User-Agent header to be used by this client

Supported keys:
- `user_agent`

<a id="op-f036de2bac07aa7eb699458e"></a>
## as_ref

`function` · `object_store::client::ClientConfigKey::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [226, 2], "filename": "src/client/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/client/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f460dee3d2084648e7a57d69"></a>
## clone

`function` · `object_store::client::ClientConfigKey::clone` · object_store 0.13.2

```rust
fn clone(&self) -> ClientConfigKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 31], "end": [81, 36], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebfecd54d8d6a5e7fdc0e145"></a>
## deserialize

`function` · `object_store::client::ClientConfigKey::deserialize` · object_store 0.13.2

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 51], "end": [81, 62], "filename": "src/client/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/client/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bde49200f60ac25acb80af6e"></a>
## eq

`function` · `object_store::client::ClientConfigKey::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &ClientConfigKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 10], "end": [81, 19], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/client/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5457ed40b590e2d06f3ca58d"></a>
## fmt

`function` · `object_store::client::ClientConfigKey::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 38], "end": [81, 43], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb112930c7bbb44534f762a1"></a>
## from_str

`function` · `object_store::client::ClientConfigKey::from_str` · object_store 0.13.2

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [257, 2], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/client/mod.rs:231`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2343c2e00508f85e38278be1"></a>
## hash

`function` · `object_store::client::ClientConfigKey::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 25], "end": [81, 29], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/client/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c495898f5393986b4846b269"></a>
## serialize

`function` · `object_store::client::ClientConfigKey::serialize` · object_store 0.13.2

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientConfigKey", "path": "ClientConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 64], "end": [81, 73], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/client/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
