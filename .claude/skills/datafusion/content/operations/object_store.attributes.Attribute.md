# `object_store::attributes::Attribute`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.attributes.Attribute.json).

<a id="op-ba938db385f847e7b49d9c79"></a>
## Attribute

`enum` · `object_store::attributes::Attribute` · object_store 0.13.2

```rust
enum Attribute
```

Source: `src/attributes.rs:25`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Additional object attribute types

<a id="op-b8b30abfd9b70f0e8849b847"></a>
## CacheControl

`variant` · `object_store::attributes::Attribute::CacheControl` · object_store 0.13.2

```rust
CacheControl
```

Source: `src/attributes.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Overrides cache control policy of the object

See [Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control)

<a id="op-d0407e022679f73b5040198b"></a>
## ContentDisposition

`variant` · `object_store::attributes::Attribute::ContentDisposition` · object_store 0.13.2

```rust
ContentDisposition
```

Source: `src/attributes.rs:29`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Specifies how the object should be handled by a browser

See [Content-Disposition](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition)

<a id="op-1eb19ee0aaa5dcb649b350e3"></a>
## ContentEncoding

`variant` · `object_store::attributes::Attribute::ContentEncoding` · object_store 0.13.2

```rust
ContentEncoding
```

Source: `src/attributes.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Specifies the encodings applied to the object

See [Content-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding)

<a id="op-38d7eb985c318aa41fcf6dac"></a>
## ContentLanguage

`variant` · `object_store::attributes::Attribute::ContentLanguage` · object_store 0.13.2

```rust
ContentLanguage
```

Source: `src/attributes.rs:37`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Specifies the language of the object

See [Content-Language](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Language)

<a id="op-1a116a9067e8e07918238221"></a>
## ContentType

`variant` · `object_store::attributes::Attribute::ContentType` · object_store 0.13.2

```rust
ContentType
```

Source: `src/attributes.rs:43`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Specifies the MIME type of the object

This takes precedence over any [ClientOptions](crate::ClientOptions) configuration

See [Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type)

<a id="op-1aa869934f536a752aa38f01"></a>
## Metadata

`variant` · `object_store::attributes::Attribute::Metadata` · object_store 0.13.2

```rust
Metadata
```

Source: `src/attributes.rs:59`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Specifies a user-defined metadata field for the object

The String is a user-defined key

<a id="op-6e1d1b50e355d1445097f63c"></a>
## StorageClass

`variant` · `object_store::attributes::Attribute::StorageClass` · object_store 0.13.2

```rust
StorageClass
```

Source: `src/attributes.rs:55`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Specifies the storage class of the object.

See [AWS](https://aws.amazon.com/s3/storage-classes/),
[GCP](https://cloud.google.com/storage/docs/storage-classes), and
[Azure](https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tier).  
`StorageClass` is used as the name for this attribute because 2 of the 3 storage providers
use that name

<a id="op-be676ab650a0aea047dc5212"></a>
## clone

`function` · `object_store::attributes::Attribute::clone` · object_store 0.13.2

```rust
fn clone(&self) -> Attribute
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attribute", "path": "Attribute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 38], "end": [24, 43], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/attributes.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7f0fb3b3b5d0d4c13fdfd98"></a>
## eq

`function` · `object_store::attributes::Attribute::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Attribute) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attribute", "path": "Attribute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 27], "end": [24, 36], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/attributes.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca8335125667e3c9e339dc8f"></a>
## fmt

`function` · `object_store::attributes::Attribute::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attribute", "path": "Attribute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/attributes.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d51cf82421374b89f86e8aac"></a>
## hash

`function` · `object_store::attributes::Attribute::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attribute", "path": "Attribute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 17], "end": [24, 21], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/attributes.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
