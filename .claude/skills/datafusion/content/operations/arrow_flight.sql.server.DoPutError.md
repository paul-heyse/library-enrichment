# `arrow_flight::sql::server::DoPutError`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.server.DoPutError.json).

<a id="op-56cf26be4692f49db784bacb"></a>
## DoPutError

`enum` · `arrow_flight::sql::server::DoPutError` · arrow-flight 59.3.0

```rust
enum DoPutError
```

Source: `src/sql/server.rs:992`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Unrecoverable errors associated with `do_put` requests

<a id="op-8055bc35972e30d71f77c843"></a>
## MissingCommand

`variant` · `arrow_flight::sql::server::DoPutError::MissingCommand` · arrow-flight 59.3.0

```rust
MissingCommand
```

Source: `src/sql/server.rs:994`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The first element in the request stream is missing the command

<a id="op-9e9ea9859f14693d469a9905"></a>
## MissingFlightDescriptor

`variant` · `arrow_flight::sql::server::DoPutError::MissingFlightDescriptor` · arrow-flight 59.3.0

```rust
MissingFlightDescriptor
```

Source: `src/sql/server.rs:996`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The first element in the request stream is missing the flight descriptor

<a id="op-28ebe87cd82d5cc7327464c4"></a>
## fmt

`function` · `arrow_flight::sql::server::DoPutError::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::server::DoPutError", "path": "DoPutError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [998, 1], "end": [1009, 2], "filename": "src/sql/server.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/sql/server.rs:999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
