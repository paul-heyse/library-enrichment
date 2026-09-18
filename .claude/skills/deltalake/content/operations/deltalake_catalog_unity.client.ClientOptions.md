# `deltalake_catalog_unity::client::ClientOptions`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.ClientOptions.json).

<a id="op-8fdf83aa9861883704a4f81c"></a>
## ClientOptions

`struct` · `deltalake_catalog_unity::client::ClientOptions` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ClientOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L31).

Source: `crates/catalog-unity/src/client/mod.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

HTTP client configuration for remote catalogs

<a id="op-290725a757a62906450163aa"></a>
## builder

`function` · `deltalake_catalog_unity::client::ClientOptions::builder` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder() -> ClientOptionsBuilder<((), (), (), (), (), (), (), (), (), (), (), (), (), (), ())>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 33], "end": [29, 45], "filename": "crates/catalog-unity/src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/client/mod.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a builder for building `ClientOptions`.
On the builder, call `.user_agent(...)`(optional), `.default_headers(...)`(optional), `.proxy_url(...)`(optional), `.allow_http(...)`(optional), `.allow_insecure(...)`(optional), `.timeout(...)`(optional), `.connect_timeout(...)`(optional), `.pool_idle_timeout(...)`(optional), `.pool_max_idle_per_host(...)`(optional), `.http2_keep_alive_interval(...)`(optional), `.http2_keep_alive_timeout(...)`(optional), `.http2_keep_alive_while_idle(...)`(optional), `.http1_only(...)`(optional), `.http2_only(...)`(optional), `.retry_config(...)`(optional) to set the values of the fields.
Finally, call `.build()` to create the instance of `ClientOptions`.
                

<a id="op-2db5811c3ee01bbf6d34d1a1"></a>
## clone

`function` · `deltalake_catalog_unity::client::ClientOptions::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ClientOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "crates/catalog-unity/src/client/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/client/mod.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-823c2c738a96f21abc2686e9"></a>
## default

`function` · `deltalake_catalog_unity::client::ClientOptions::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ClientOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 24], "end": [29, 31], "filename": "crates/catalog-unity/src/client/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/client/mod.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c86a55d0efef2d87c9c01e6"></a>
## fmt

`function` · `deltalake_catalog_unity::client::ClientOptions::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::ClientOptions", "path": "ClientOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "crates/catalog-unity/src/client/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/mod.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6c3c321f25b4f1a1ebc82d6"></a>
## allow_http

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::allow_http` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_http: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L43).

Source: `crates/catalog-unity/src/client/mod.rs:43`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow HTTP connections (default: false)

<a id="op-89c05696a89994e76299c255"></a>
## allow_insecure

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::allow_insecure` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_insecure: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L46).

Source: `crates/catalog-unity/src/client/mod.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow invalid SSL certificates (default: false)

<a id="op-cf73965d2c4f34ed54a0796c"></a>
## connect_timeout

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::connect_timeout` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
connect_timeout: Option<std::time::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L52).

Source: `crates/catalog-unity/src/client/mod.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Connect timeout

<a id="op-1dbbe18a8be00d9d49e9eea3"></a>
## default_headers

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::default_headers` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
default_headers: Option<reqwest::header::HeaderMap>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L37).

Source: `crates/catalog-unity/src/client/mod.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default headers for every request

<a id="op-23f78b54237b62c9fd0b066d"></a>
## http1_only

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::http1_only` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
http1_only: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L70).

Source: `crates/catalog-unity/src/client/mod.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only use HTTP1

<a id="op-9e973ef12278cc5320a2726d"></a>
## http2_keep_alive_interval

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::http2_keep_alive_interval` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
http2_keep_alive_interval: Option<std::time::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L61).

Source: `crates/catalog-unity/src/client/mod.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

HTTP2 keep alive interval

<a id="op-2632102bf65ca6bb7c85415b"></a>
## http2_keep_alive_timeout

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::http2_keep_alive_timeout` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
http2_keep_alive_timeout: Option<std::time::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L64).

Source: `crates/catalog-unity/src/client/mod.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

HTTP2 keep alive timeout

<a id="op-63e71fb2c3fd8f1ad7f891e8"></a>
## http2_keep_alive_while_idle

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::http2_keep_alive_while_idle` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
http2_keep_alive_while_idle: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L67).

Source: `crates/catalog-unity/src/client/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enable HTTP2 keep alive while idle

<a id="op-432668bb359fcd770b8d4e89"></a>
## http2_only

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::http2_only` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
http2_only: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L73).

Source: `crates/catalog-unity/src/client/mod.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only use HTTP2

<a id="op-32fb602e37bb72f7035ec985"></a>
## pool_idle_timeout

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::pool_idle_timeout` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
pool_idle_timeout: Option<std::time::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L55).

Source: `crates/catalog-unity/src/client/mod.rs:55`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Pool idle timeout

<a id="op-6103812b187651d920a7f9ba"></a>
## pool_max_idle_per_host

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::pool_max_idle_per_host` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
pool_max_idle_per_host: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L58).

Source: `crates/catalog-unity/src/client/mod.rs:58`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Maximum number of idle connections per host

<a id="op-3167893fbdc5f797fd487eff"></a>
## proxy_url

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::proxy_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
proxy_url: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L40).

Source: `crates/catalog-unity/src/client/mod.rs:40`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

HTTP proxy URL

<a id="op-f0efb176d72db93d1c0b95c7"></a>
## retry_config

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::retry_config` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
retry_config: Option<client::retry::RetryConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L76).

Source: `crates/catalog-unity/src/client/mod.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Retry configuration

<a id="op-6f2f5037b55f20e8bacb60b9"></a>
## timeout

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::timeout` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
timeout: Option<std::time::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L49).

Source: `crates/catalog-unity/src/client/mod.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Request timeout

<a id="op-a51035ef50d1671a40281d2f"></a>
## user_agent

`struct_field` · `deltalake_catalog_unity::client::ClientOptions::user_agent` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
user_agent: Option<reqwest::header::HeaderValue>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/mod.rs#L34).

Source: `crates/catalog-unity/src/client/mod.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

User-Agent header to use for requests
