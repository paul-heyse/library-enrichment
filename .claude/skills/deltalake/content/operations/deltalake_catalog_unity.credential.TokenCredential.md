# `deltalake_catalog_unity::credential::TokenCredential`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.TokenCredential.json).

<a id="op-dc786eb2bc30c1795bea0430"></a>
## TokenCredential

`trait` · `deltalake_catalog_unity::credential::TokenCredential` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TokenCredential: std::fmt::Debug + Send + Sync + 'static
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L37).

Source: `crates/catalog-unity/src/credential.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait for providing authorization tokens for catalog requests

<a id="op-9c3e0a5ce0535dd1cd4beef6"></a>
## fetch_token

`function` · `deltalake_catalog_unity::credential::TokenCredential::fetch_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L39).

Source: `crates/catalog-unity/src/credential.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

get the token
