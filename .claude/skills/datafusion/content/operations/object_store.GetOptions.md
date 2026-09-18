# `object_store::GetOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.GetOptions.json).

<a id="op-eb3de57203b85138a693ae44"></a>
## GetOptions

`struct` · `object_store::GetOptions` · object_store 0.13.2

```rust
struct GetOptions
```

Source: `src/lib.rs:1439`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Options for a get request, such as range

<a id="op-02d80f921a3f75b0f5b63db1"></a>
## check_preconditions

`function` · `object_store::GetOptions::check_preconditions` · object_store 0.13.2

```rust
fn check_preconditions(&self, meta: &ObjectMeta) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1500`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns an error if the modification conditions on this request are not satisfied

<https://datatracker.ietf.org/doc/html/rfc7232#section-6>

<a id="op-410449c72dbc6be11cf20015"></a>
## clone

`function` · `object_store::GetOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> GetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 26], "end": [1438, 31], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1438`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6678a71c7daef8971737830a"></a>
## default

`function` · `object_store::GetOptions::default` · object_store 0.13.2

```rust
fn default() -> GetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 17], "end": [1438, 24], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1438`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6335b0b78f8871e9eae7fa22"></a>
## extensions

`struct_field` · `object_store::GetOptions::extensions` · object_store 0.13.2

```rust
extensions: Extensions
```

Source: `src/lib.rs:1493`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Implementation-specific extensions. Intended for use by [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

<a id="op-ec4cc7b8167295ccaca5f2f0"></a>
## fmt

`function` · `object_store::GetOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 10], "end": [1438, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1438`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4312e49d006f20305eded3ac"></a>
## head

`struct_field` · `object_store::GetOptions::head` · object_store 0.13.2

```rust
head: bool
```

Source: `src/lib.rs:1488`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request transfer of no content

<https://datatracker.ietf.org/doc/html/rfc9110#name-head>

<a id="op-080af179618e26cf1f4989b8"></a>
## if_match

`struct_field` · `object_store::GetOptions::if_match` · object_store 0.13.2

```rust
if_match: Option<String>
```

Source: `src/lib.rs:1452`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request will succeed if the `ObjectMeta::e_tag` matches
otherwise returning [`Error::Precondition`](../operations/object_store.Error.md#op-530afde8313e7ef1611c8b0a)

See <https://datatracker.ietf.org/doc/html/rfc9110#name-if-match>

Examples:

```text
If-Match: "xyzzy"
If-Match: "xyzzy", "r2d2xxxx", "c3piozzzz"
If-Match: *
```

<a id="op-fd65745d6c45c66c5d4fa6f5"></a>
## if_modified_since

`struct_field` · `object_store::GetOptions::if_modified_since` · object_store 0.13.2

```rust
if_modified_since: Option<chrono::DateTime<chrono::Utc>>
```

Source: `src/lib.rs:1469`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request will succeed if the object has been modified since

<https://datatracker.ietf.org/doc/html/rfc9110#section-13.1.3>

<a id="op-36b3b809c82f7150ae54c408"></a>
## if_none_match

`struct_field` · `object_store::GetOptions::if_none_match` · object_store 0.13.2

```rust
if_none_match: Option<String>
```

Source: `src/lib.rs:1465`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request will succeed if the `ObjectMeta::e_tag` does not match
otherwise returning [`Error::NotModified`](../operations/object_store.Error.md#op-06842ee960fff2816c7230b3)

See <https://datatracker.ietf.org/doc/html/rfc9110#section-13.1.2>

Examples:

```text
If-None-Match: "xyzzy"
If-None-Match: "xyzzy", "r2d2xxxx", "c3piozzzz"
If-None-Match: *
```

<a id="op-c0aeac71ada7fa81347430c9"></a>
## if_unmodified_since

`struct_field` · `object_store::GetOptions::if_unmodified_since` · object_store 0.13.2

```rust
if_unmodified_since: Option<chrono::DateTime<chrono::Utc>>
```

Source: `src/lib.rs:1477`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request will succeed if the object has not been modified since
otherwise returning [`Error::Precondition`](../operations/object_store.Error.md#op-530afde8313e7ef1611c8b0a)

Some stores, such as S3, will only return `NotModified` for exact
timestamp matches, instead of for any timestamp greater than or equal.

<https://datatracker.ietf.org/doc/html/rfc9110#section-13.1.4>

<a id="op-7c2281f8b530d5bcc5bf1cb4"></a>
## new

`function` · `object_store::GetOptions::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1540`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`GetOptions`](../operations/object_store.GetOptions.md#op-eb3de57203b85138a693ae44)

<a id="op-58ec02b046cf303bd37dc1f2"></a>
## range

`struct_field` · `object_store::GetOptions::range` · object_store 0.13.2

```rust
range: Option<GetRange>
```

Source: `src/lib.rs:1482`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request transfer of only the specified range of bytes
otherwise returning [`Error::NotModified`](../operations/object_store.Error.md#op-06842ee960fff2816c7230b3)

<https://datatracker.ietf.org/doc/html/rfc9110#name-range>

<a id="op-6a1fba4604b0696f51061c51"></a>
## version

`struct_field` · `object_store::GetOptions::version` · object_store 0.13.2

```rust
version: Option<String>
```

Source: `src/lib.rs:1484`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request a particular object version

<a id="op-7b509557fb0cfc6aa44a2940"></a>
## with_extensions

`function` · `object_store::GetOptions::with_extensions` · object_store 0.13.2

```rust
fn with_extensions(self, extensions: Extensions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1611`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `extensions` condition.

See [`GetOptions::extensions`](../operations/object_store.GetOptions.md#op-6335b0b78f8871e9eae7fa22)

<a id="op-8426d313609b7a192ad6c4b8"></a>
## with_head

`function` · `object_store::GetOptions::with_head` · object_store 0.13.2

```rust
fn with_head(self, head: impl Into<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1602`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `head` condition.

See [`GetOptions::head`](../operations/object_store.GetOptions.md#op-4312e49d006f20305eded3ac)

<a id="op-f8b064e19f04e17794d1482f"></a>
## with_if_match

`function` · `object_store::GetOptions::with_if_match` · object_store 0.13.2

```rust
fn with_if_match(self, etag: Option<impl Into<String>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1548`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `if_match` condition.

See [`GetOptions::if_match`](../operations/object_store.GetOptions.md#op-080af179618e26cf1f4989b8)

<a id="op-5b48bd71d483c7b22ccd6fe2"></a>
## with_if_modified_since

`function` · `object_store::GetOptions::with_if_modified_since` · object_store 0.13.2

```rust
fn with_if_modified_since(self, dt: Option<impl Into<DateTime<Utc>>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1566`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `if_modified_since` condition.

See [`GetOptions::if_modified_since`](../operations/object_store.GetOptions.md#op-fd65745d6c45c66c5d4fa6f5)

<a id="op-190e364d6926ed12336a9890"></a>
## with_if_none_match

`function` · `object_store::GetOptions::with_if_none_match` · object_store 0.13.2

```rust
fn with_if_none_match(self, etag: Option<impl Into<String>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1557`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `if_none_match` condition.

See [`GetOptions::if_none_match`](../operations/object_store.GetOptions.md#op-36b3b809c82f7150ae54c408)

<a id="op-d8c1578ac4047648028edde2"></a>
## with_if_unmodified_since

`function` · `object_store::GetOptions::with_if_unmodified_since` · object_store 0.13.2

```rust
fn with_if_unmodified_since(self, dt: Option<impl Into<DateTime<Utc>>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1575`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `if_unmodified_since` condition.

See [`GetOptions::if_unmodified_since`](../operations/object_store.GetOptions.md#op-c0aeac71ada7fa81347430c9)

<a id="op-e58d7f5457c8dfad2c86c4af"></a>
## with_range

`function` · `object_store::GetOptions::with_range` · object_store 0.13.2

```rust
fn with_range(self, range: Option<impl Into<GetRange>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1584`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `range` condition.

See [`GetOptions::range`](../operations/object_store.GetOptions.md#op-58ec02b046cf303bd37dc1f2)

<a id="op-43dea78a09d12de16bd81930"></a>
## with_version

`function` · `object_store::GetOptions::with_version` · object_store 0.13.2

```rust
fn with_version(self, version: Option<impl Into<String>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetOptions", "path": "GetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1496, 1], "end": [1615, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1593`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `version` condition.

See [`GetOptions::version`](../operations/object_store.GetOptions.md#op-6a1fba4604b0696f51061c51)
