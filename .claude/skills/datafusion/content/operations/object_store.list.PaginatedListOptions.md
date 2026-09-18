# `object_store::list::PaginatedListOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.list.PaginatedListOptions.json).

<a id="op-268017b840b4bf431169198c"></a>
## PaginatedListOptions

`struct` · `object_store::list::PaginatedListOptions` · object_store 0.13.2

```rust
struct PaginatedListOptions
```

Source: `src/list.rs:27`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Options for a paginated list request

<a id="op-422ba9365479f84ff18c6260"></a>
## clone

`function` · `object_store::list::PaginatedListOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PaginatedListOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::list::PaginatedListOptions", "path": "PaginatedListOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 26], "end": [26, 31], "filename": "src/list.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/list.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-116646229fd20a970560f994"></a>
## default

`function` · `object_store::list::PaginatedListOptions::default` · object_store 0.13.2

```rust
fn default() -> PaginatedListOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::list::PaginatedListOptions", "path": "PaginatedListOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 24], "filename": "src/list.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/list.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-539665399f4343e5a430be8c"></a>
## delimiter

`struct_field` · `object_store::list::PaginatedListOptions::delimiter` · object_store 0.13.2

```rust
delimiter: Option<std::borrow::Cow<'static, str>>
```

Source: `src/list.rs:39`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A delimiter use to group keys with a common prefix

Note: Some stores only support `/`

<a id="op-18d825a6470b71944911e023"></a>
## extensions

`struct_field` · `object_store::list::PaginatedListOptions::extensions` · object_store 0.13.2

```rust
extensions: http::Extensions
```

Source: `src/list.rs:54`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Implementation-specific extensions. Intended for use by implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

<a id="op-fa92b0ffb23b8aed4aa7c7ef"></a>
## fmt

`function` · `object_store::list::PaginatedListOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::list::PaginatedListOptions", "path": "PaginatedListOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/list.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73f6641d9a9392736847349b"></a>
## max_keys

`struct_field` · `object_store::list::PaginatedListOptions::max_keys` · object_store 0.13.2

```rust
max_keys: Option<usize>
```

Source: `src/list.rs:42`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The maximum number of paths to return

<a id="op-1b27f60445b04299eb37caa3"></a>
## offset

`struct_field` · `object_store::list::PaginatedListOptions::offset` · object_store 0.13.2

```rust
offset: Option<String>
```

Source: `src/list.rs:34`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Path to start listing from

Note: Not all stores support this

For stores that do support this, the returned
result should not include the object with this key.

<a id="op-bdfc60f222e8182094d789c9"></a>
## page_token

`struct_field` · `object_store::list::PaginatedListOptions::page_token` · object_store 0.13.2

```rust
page_token: Option<String>
```

Source: `src/list.rs:48`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A page token from a previous request

Note: Behaviour is implementation defined if the previous request
used a different prefix or options
