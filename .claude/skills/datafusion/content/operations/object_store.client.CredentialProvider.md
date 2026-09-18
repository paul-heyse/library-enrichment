# `object_store::client::CredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.CredentialProvider.json).

<a id="op-76004bbb5f119134cca4253a"></a>
## CredentialProvider

`trait` · `object_store::client::CredentialProvider` · object_store 0.13.2

```rust
trait CredentialProvider: std::fmt::Debug + Send + Sync
```

Source: `src/client/mod.rs:907`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Provides credentials for use when signing requests

<a id="op-5d45dcf2432a67fe5f680b1c"></a>
## Credential

`assoc_type` · `object_store::client::CredentialProvider::Credential` · object_store 0.13.2

```rust
Credential
```

Source: `src/client/mod.rs:909`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The type of credential returned by this provider

<a id="op-5caedf6e78dd95cf7e883745"></a>
## get_credential

`function` · `object_store::client::CredentialProvider::get_credential` · object_store 0.13.2

```rust
async fn get_credential(&self) -> Result<Arc<Self::Credential>>
```

Source: `src/client/mod.rs:912`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return a credential
