# `object_store::signer::Signer`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.signer.Signer.json).

<a id="op-15212c540ef0da309f950f9e"></a>
## Signer

`trait` · `object_store::signer::Signer` · object_store 0.13.2

```rust
trait Signer: Send + Sync + fmt::Debug + 'static
```

Source: `src/signer.rs:28`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Universal API to generate presigned URLs from multiple object store services.

<a id="op-9250ff0b0830dc8fe75b5a1f"></a>
## signed_url

`function` · `object_store::signer::Signer::signed_url` · object_store 0.13.2

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
```

Source: `src/signer.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Given the intended [`Method`] and [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) to use and the desired length of time for which
the URL should be valid, return a signed [`Url`] created with the object store
implementation's credentials such that the URL can be handed to something that doesn't have
access to the object store's credentials, to allow limited access to the object store.

Unresolved upstream links (retained, not inferred): ``Method``, ``Url``.

<a id="op-803aba8045c1615cae3d0c7b"></a>
## signed_urls

`function` · `object_store::signer::Signer::signed_urls` · object_store 0.13.2

```rust
async fn signed_urls(&self, method: Method, paths: &[Path], expires_in: Duration) -> Result<Vec<Url>>
```

Source: `src/signer.rs:38`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Generate signed urls for multiple paths.

See [`Signer::signed_url`](../operations/object_store.signer.Signer.md#op-9250ff0b0830dc8fe75b5a1f) for more details.
