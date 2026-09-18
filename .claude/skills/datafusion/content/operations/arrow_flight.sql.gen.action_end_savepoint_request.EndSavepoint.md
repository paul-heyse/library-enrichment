# `arrow_flight::sql::gen::action_end_savepoint_request::EndSavepoint`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.action_end_savepoint_request.EndSavepoint.json).

<a id="op-fe385fb8e247494540fadf6a"></a>
## EndSavepoint

`enum` · `arrow_flight::sql::gen::action_end_savepoint_request::EndSavepoint` · arrow-flight 59.3.0

```rust
enum EndSavepoint
```

Source: `src/sql/arrow.flight.protocol.sql.rs:635`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d0a21e9e2d7520ce87cf2f"></a>
## Release

`variant` · `arrow_flight::sql::gen::action_end_savepoint_request::EndSavepoint::Release` · arrow-flight 59.3.0

```rust
Release
```

Source: `src/sql/arrow.flight.protocol.sql.rs:638`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Release the savepoint.

<a id="op-05813f8cbdfadf83ba349e7a"></a>
## Rollback

`variant` · `arrow_flight::sql::gen::action_end_savepoint_request::EndSavepoint::Rollback` · arrow-flight 59.3.0

```rust
Rollback
```

Source: `src/sql/arrow.flight.protocol.sql.rs:640`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Roll back to a savepoint.

<a id="op-ef891cf33227a9ae35afeeac"></a>
## Unspecified

`variant` · `arrow_flight::sql::gen::action_end_savepoint_request::EndSavepoint::Unspecified` · arrow-flight 59.3.0

```rust
Unspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:636`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
