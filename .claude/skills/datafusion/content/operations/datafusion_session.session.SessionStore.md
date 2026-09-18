# `datafusion_session::session::SessionStore`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.session.SessionStore.json).

<a id="op-b153d92d272c385625eb8943"></a>
## SessionStore

`struct` · `datafusion_session::session::SessionStore` · datafusion-session 55.1.0

```rust
struct SessionStore
```

Source: `src/session.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

The state store that stores the reference of the runtime session state.

<a id="op-ca50a9a7aa019855f1f1b456"></a>
## default

`function` · `datafusion_session::session::SessionStore::default` · datafusion-session 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::session::SessionStore", "path": "SessionStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [255, 2], "filename": "src/session.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/session.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26d712edfceeec313439da0d"></a>
## fmt

`function` · `datafusion_session::session::SessionStore::fmt` · datafusion-session 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::session::SessionStore", "path": "SessionStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 10], "end": [226, 15], "filename": "src/session.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/session.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3453d586f409b06d943115f4"></a>
## get_session

`function` · `datafusion_session::session::SessionStore::get_session` · datafusion-session 55.1.0

```rust
fn get_session(&self) -> Weak<RwLock<dyn Session>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::session::SessionStore", "path": "SessionStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [249, 2], "filename": "src/session.rs"}, "trait": null, "trait_path": null}`

Source: `src/session.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the current session of the store

<a id="op-92c484086aebc04c2e1e2856"></a>
## new

`function` · `datafusion_session::session::SessionStore::new` · datafusion-session 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::session::SessionStore", "path": "SessionStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [249, 2], "filename": "src/session.rs"}, "trait": null, "trait_path": null}`

Source: `src/session.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a new [SessionStore](../operations/datafusion_session.session.SessionStore.md#op-b153d92d272c385625eb8943)

<a id="op-c0a878455fa9063cd4027ba4"></a>
## with_state

`function` · `datafusion_session::session::SessionStore::with_state` · datafusion-session 55.1.0

```rust
fn with_state(&self, state: Weak<RwLock<dyn Session>>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::session::SessionStore", "path": "SessionStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [249, 2], "filename": "src/session.rs"}, "trait": null, "trait_path": null}`

Source: `src/session.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Set the session state of the store
