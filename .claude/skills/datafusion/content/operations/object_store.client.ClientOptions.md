# `object_store::client::ClientOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.ClientOptions.json).

<a id="op-9357d27ab9e13340e3ccb12f"></a>
## ClientOptions

`struct` · `object_store::client::ClientOptions` · object_store 0.13.2

```rust
struct ClientOptions
```

Source: `src/client/mod.rs:312`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

HTTP client configuration for remote object stores

<a id="op-98b90e3a7478237b93f1cc48"></a>
## clone

`function` · `object_store::client::ClientOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> ClientOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 17], "end": [311, 22], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/mod.rs:311`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-722fbe550167a2e51efb506b"></a>
## default

`function` · `object_store::client::ClientOptions::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 1], "end": [374, 2], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/client/mod.rs:338`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10093829e4e59006926a936a"></a>
## fmt

`function` · `object_store::client::ClientOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 10], "end": [311, 15], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/mod.rs:311`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea17646260ad094f180004a2"></a>
## get_config_value

`function` · `object_store::client::ClientOptions::get_config_value` · object_store 0.13.2

```rust
fn get_config_value(&self, key: &ClientConfigKey) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:426`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get an option by key

<a id="op-aa2735afa88bf4a1d29f8baf"></a>
## get_content_type

`function` · `object_store::client::ClientOptions::get_content_type` · object_store 0.13.2

```rust
fn get_content_type(&self, path: &Path) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:724`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get the mime type for the file in `path` to be uploaded

Gets the file extension from `path`, and returns the
mime type if it was defined initially through
`ClientOptions::with_content_type_for_suffix`

Otherwise, returns the default mime type if it was defined
earlier through `ClientOptions::with_default_content_type`

<a id="op-99efc301ffbfdcf23298e9fa"></a>
## get_default_headers

`function` · `object_store::client::ClientOptions::get_default_headers` · object_store 0.13.2

```rust
fn get_default_headers(&self) -> Option<&HeaderMap>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:712`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get the default headers defined through `ClientOptions::with_default_headers`

<a id="op-1c19a6a324fd991f4f58cf77"></a>
## new

`function` · `object_store::client::ClientOptions::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:378`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`ClientOptions`](../operations/object_store.client.ClientOptions.md#op-9357d27ab9e13340e3ccb12f) with default values

<a id="op-dcba25c8aa68aab68790c9d3"></a>
## with_allow_http

`function` · `object_store::client::ClientOptions::with_allow_http` · object_store 0.13.2

```rust
fn with_allow_http(self, allow_http: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:508`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets what protocol is allowed.

If `allow_http` is :
* `false` (default):  Only HTTPS is allowed
* `true`:  HTTP and HTTPS are allowed

<a id="op-61dc60e680e8b7e0df806217"></a>
## with_allow_http2

`function` · `object_store::client::ClientOptions::with_allow_http2` · object_store 0.13.2

```rust
fn with_allow_http2(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:573`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use HTTP/2 if supported, otherwise use HTTP/1.

# See Also
* [`Self::with_http1_only`](../operations/object_store.client.ClientOptions.md#op-a7f42f41e2fcb92b3fbcdb8c) if you only want to use HTTP/1
* [`Self::with_http2_only`](../operations/object_store.client.ClientOptions.md#op-7dbfb5c8522ab2cbe448b8a0) if you only want to use HTTP/2

<div class="warning">
HTTP/2 is not used by default. See details [#104](https://github.com/apache/arrow-rs-object-store/issues/104)
</div>

<a id="op-7efab14f5952bee1a419f801"></a>
## with_allow_invalid_certificates

`function` · `object_store::client::ClientOptions::with_allow_invalid_certificates` · object_store 0.13.2

```rust
fn with_allow_invalid_certificates(self, allow_insecure: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:529`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Allows connections to invalid SSL certificates

If `allow_invalid_certificates` is :
* `false` (default):  Only valid HTTPS certificates are allowed
* `true`:  All HTTPS certificates are allowed

<div class="warning">

**Warning**

You should think very carefully before using this method. If
invalid certificates are trusted, *any* certificate for *any* site
will be trusted for use. This includes expired certificates. This
introduces significant vulnerabilities, and should only be used
as a last resort or for testing

</div>

<a id="op-62fbdd077f4de3b1f4805632"></a>
## with_config

`function` · `object_store::client::ClientOptions::with_config` · object_store 0.13.2

```rust
fn with_config(self, key: ClientConfigKey, value: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:383`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set an option by key

<a id="op-7a0cc47107c3726023d5ba61"></a>
## with_connect_timeout

`function` · `object_store::client::ClientOptions::with_connect_timeout` · object_store 0.13.2

```rust
fn with_connect_timeout(self, timeout: Duration) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:641`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a timeout for only the connect phase of a Client

This is the time allowed for the client to establish a connection
and if the connection is not established within this time,
the client returns a timeout error.

Timeout errors are retried, subject to the [`RetryConfig`]

Default is 5 seconds

# See Also
* [`Self::with_timeout`](../operations/object_store.client.ClientOptions.md#op-a4c8d234815441ef57cfc281) to set a timeout for the overall request
* [`Self::with_connect_timeout_disabled`](../operations/object_store.client.ClientOptions.md#op-21e9660712ecd6d25902fc60) to disable the connect timeout

[`RetryConfig`]: crate::RetryConfig

<a id="op-21e9660712ecd6d25902fc60"></a>
## with_connect_timeout_disabled

`function` · `object_store::client::ClientOptions::with_connect_timeout_disabled` · object_store 0.13.2

```rust
fn with_connect_timeout_disabled(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:650`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Disables the connection timeout

# See Also
* [`Self::with_connect_timeout`](../operations/object_store.client.ClientOptions.md#op-7a0cc47107c3726023d5ba61)

<a id="op-02d03f58b49c1e58214b5e00"></a>
## with_content_type_for_suffix

`function` · `object_store::client::ClientOptions::with_content_type_for_suffix` · object_store 0.13.2

```rust
fn with_content_type_for_suffix(self, extension: impl Into<String>, mime: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:488`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the [`Content-Type`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type) for a given file extension

<a id="op-778ec09d6cb461ac3cb4f573"></a>
## with_default_content_type

`function` · `object_store::client::ClientOptions::with_default_content_type` · object_store 0.13.2

```rust
fn with_default_content_type(self, mime: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:482`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the default [`Content-Type`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type) for uploads

<a id="op-369deb5cec40a1d5f65810f8"></a>
## with_default_headers

`function` · `object_store::client::ClientOptions::with_default_headers` · object_store 0.13.2

```rust
fn with_default_headers(self, headers: HeaderMap) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the default headers for every request

<a id="op-a7f42f41e2fcb92b3fbcdb8c"></a>
## with_http1_only

`function` · `object_store::client::ClientOptions::with_http1_only` · object_store 0.13.2

```rust
fn with_http1_only(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:543`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Only use HTTP/1 connections (default)

# See Also
* [`Self::with_http2_only`](../operations/object_store.client.ClientOptions.md#op-7dbfb5c8522ab2cbe448b8a0) if you only want to use HTTP/2
* [`Self::with_allow_http2`](../operations/object_store.client.ClientOptions.md#op-61dc60e680e8b7e0df806217) if you want to use HTTP/1 or HTTP/2

<div class="warning">
HTTP/2 is not used by default. See details [#104](https://github.com/apache/arrow-rs-object-store/issues/104)
</div>

<a id="op-60b6d2f19e16cfa12036c3d2"></a>
## with_http2_keep_alive_interval

`function` · `object_store::client::ClientOptions::with_http2_keep_alive_interval` · object_store 0.13.2

```rust
fn with_http2_keep_alive_interval(self, interval: Duration) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:676`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets an interval for HTTP/2 Ping frames should be sent to keep a connection alive.

Default is disabled enforced by reqwest

<a id="op-f593b81d038c513a69ed7a6a"></a>
## with_http2_keep_alive_timeout

`function` · `object_store::client::ClientOptions::with_http2_keep_alive_timeout` · object_store 0.13.2

```rust
fn with_http2_keep_alive_timeout(self, interval: Duration) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:687`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets a timeout for receiving an acknowledgement of the keep-alive ping.

If the ping is not acknowledged within the timeout, the connection will be closed.
Does nothing if `http2_keep_alive_interval` is disabled.

Default is disabled enforced by reqwest

<a id="op-272fc73114bd594aed003f80"></a>
## with_http2_keep_alive_while_idle

`function` · `object_store::client::ClientOptions::with_http2_keep_alive_while_idle` · object_store 0.13.2

```rust
fn with_http2_keep_alive_while_idle(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:698`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Enable HTTP/2 keep alive pings for idle connections

If disabled, keep-alive pings are only sent while there are open request/response
streams. If enabled, pings are also sent when no streams are active

Default is disabled enforced by reqwest

<a id="op-4a6f61d61dc60c98e2973317"></a>
## with_http2_max_frame_size

`function` · `object_store::client::ClientOptions::with_http2_max_frame_size` · object_store 0.13.2

```rust
fn with_http2_max_frame_size(self, sz: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:706`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the maximum frame size to use for HTTP/2.

Default is currently 16,384 but may change internally to optimize for common uses.

<a id="op-7dbfb5c8522ab2cbe448b8a0"></a>
## with_http2_only

`function` · `object_store::client::ClientOptions::with_http2_only` · object_store 0.13.2

```rust
fn with_http2_only(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:558`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Only use HTTP/2 connections

# See Also
* [`Self::with_http1_only`](../operations/object_store.client.ClientOptions.md#op-a7f42f41e2fcb92b3fbcdb8c) if you only want to use HTTP/1
* [`Self::with_allow_http2`](../operations/object_store.client.ClientOptions.md#op-61dc60e680e8b7e0df806217) if you want to use HTTP/1 or HTTP/2

<div class="warning">
HTTP/2 is not used by default. See details [#104](https://github.com/apache/arrow-rs-object-store/issues/104)
</div>

<a id="op-4a52d0a14047eca2da669f82"></a>
## with_pool_idle_timeout

`function` · `object_store::client::ClientOptions::with_pool_idle_timeout` · object_store 0.13.2

```rust
fn with_pool_idle_timeout(self, timeout: Duration) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:660`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the pool max idle timeout

This is the length of time an idle connection will be kept alive

Default is 90 seconds enforced by reqwest

<a id="op-91cf546f8ca2888e6e50ad4f"></a>
## with_pool_max_idle_per_host

`function` · `object_store::client::ClientOptions::with_pool_max_idle_per_host` · object_store 0.13.2

```rust
fn with_pool_max_idle_per_host(self, max: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:668`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the maximum number of idle connections per host

Default is no limit enforced by reqwest

<a id="op-4b2af0f0c23f824a3bdab74c"></a>
## with_proxy_ca_certificate

`function` · `object_store::client::ClientOptions::with_proxy_ca_certificate` · object_store 0.13.2

```rust
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:586`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a trusted proxy CA certificate

<a id="op-571e96be4fea440c70162bd7"></a>
## with_proxy_excludes

`function` · `object_store::client::ClientOptions::with_proxy_excludes` · object_store 0.13.2

```rust
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:592`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a list of hosts to exclude from proxy connections

<a id="op-33c7c0b8f76af90442babd15"></a>
## with_proxy_url

`function` · `object_store::client::ClientOptions::with_proxy_url` · object_store 0.13.2

```rust
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:580`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a proxy URL to use for requests

<a id="op-ea2d4dff0230bb9bd6f9a941"></a>
## with_root_certificate

`function` · `object_store::client::ClientOptions::with_root_certificate` · object_store 0.13.2

```rust
fn with_root_certificate(self, certificate: Certificate) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:476`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Add a custom root certificate.

This can be used to connect to a server that has a self-signed
certificate for example.

<a id="op-a4c8d234815441ef57cfc281"></a>
## with_timeout

`function` · `object_store::client::ClientOptions::with_timeout` · object_store 0.13.2

```rust
fn with_timeout(self, timeout: Duration) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:612`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set timeout for the overall request

The timeout starts from when the request starts connecting until the
response body has finished. If the request does not complete within the
timeout, the client returns a timeout error.

Timeout errors are retried, subject to the [`RetryConfig`]

Default is 30 seconds

# See Also
* [`Self::with_timeout_disabled`](../operations/object_store.client.ClientOptions.md#op-24bc8e4702017782d3878915) to disable the timeout
* [`Self::with_connect_timeout`](../operations/object_store.client.ClientOptions.md#op-7a0cc47107c3726023d5ba61) to set a timeout for the connect phase

[`RetryConfig`]: crate::RetryConfig

<a id="op-24bc8e4702017782d3878915"></a>
## with_timeout_disabled

`function` · `object_store::client::ClientOptions::with_timeout_disabled` · object_store 0.13.2

```rust
fn with_timeout_disabled(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:621`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Disables the request timeout

# See Also
* [`Self::with_timeout`](../operations/object_store.client.ClientOptions.md#op-a4c8d234815441ef57cfc281)

<a id="op-4f0020744950b96a2d613dc7"></a>
## with_user_agent

`function` · `object_store::client::ClientOptions::with_user_agent` · object_store 0.13.2

```rust
fn with_user_agent(self, agent: HeaderValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [857, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:466`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the [`User-Agent`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/User-Agent) header to be used by this client

Default is based on the version of this crate
