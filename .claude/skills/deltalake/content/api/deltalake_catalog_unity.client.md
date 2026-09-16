# `deltalake_catalog_unity::client`

Crate `deltalake-catalog-unity` · 2 public items · structured records in [`model/deltalake_catalog_unity.client.json`](../model/deltalake_catalog_unity.client.json)

## ClientOptions

`struct` · `deltalake_catalog_unity::client::ClientOptions`

```rust
struct ClientOptions
```

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn builder() -> ClientOptionsBuilder<((), (), (), (), (), (), (), (), (), (), (), (), (), (), ())>
```

HTTP client configuration for remote catalogs

---

## ClientOptionsBuilder

`struct` · `deltalake_catalog_unity::client::ClientOptionsBuilder`

```rust
struct ClientOptionsBuilder<TypedBuilderFields = ((), (), (), (), (), (), (), (), (), (), (), (), (), (), ())>
```

**Derives**: Clone

**Methods** (16)

```rust
fn allow_http(self, allow_http: bool) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, (bool,), __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn allow_insecure(self, allow_insecure: bool) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, (bool,), __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn build(self) -> ClientOptions
fn connect_timeout(self, connect_timeout: Duration) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, (Option<Duration>,), __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn default_headers(self, default_headers: HeaderMap) -> ClientOptionsBuilder<(__user_agent, (Option<HeaderMap>,), __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn http1_only(self, http1_only: bool) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, (bool,), __http2_only, __retry_config)>
fn http2_keep_alive_interval(self, http2_keep_alive_interval: Duration) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, (Option<Duration>,), __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn http2_keep_alive_timeout(self, http2_keep_alive_timeout: Duration) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, (Option<Duration>,), __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn http2_keep_alive_while_idle(self, http2_keep_alive_while_idle: bool) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, (bool,), __http1_only, __http2_only, __retry_config)>
fn http2_only(self, http2_only: bool) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, (bool,), __retry_config)>
fn pool_idle_timeout(self, pool_idle_timeout: Duration) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, (Option<Duration>,), __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn pool_max_idle_per_host(self, pool_max_idle_per_host: usize) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, (Option<usize>,), __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn proxy_url(self, proxy_url: impl ::core::convert::Into<String>) -> ClientOptionsBuilder<(__user_agent, __default_headers, (Option<String>,), __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn retry_config(self, retry_config: RetryConfig) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, (Option<RetryConfig>,))>
fn timeout(self, timeout: Duration) -> ClientOptionsBuilder<(__user_agent, __default_headers, __proxy_url, __allow_http, __allow_insecure, (Option<Duration>,), __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
fn user_agent(self, user_agent: HeaderValue) -> ClientOptionsBuilder<((Option<HeaderValue>,), __default_headers, __proxy_url, __allow_http, __allow_insecure, __timeout, __connect_timeout, __pool_idle_timeout, __pool_max_idle_per_host, __http2_keep_alive_interval, __http2_keep_alive_timeout, __http2_keep_alive_while_idle, __http1_only, __http2_only, __retry_config)>
```

Builder for [`ClientOptions`] instances.

See [`ClientOptions::builder()`] for more info.

---
