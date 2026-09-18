# `arrow_flight::sql::gen::action_cancel_query_result::CancelResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.action_cancel_query_result.CancelResult.json).

<a id="op-3c829120f8bba8513d32f06d"></a>
## CancelResult

`enum` · `arrow_flight::sql::gen::action_cancel_query_result::CancelResult` · arrow-flight 59.3.0

```rust
enum CancelResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:997`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e81ee18d996c81968d617cac"></a>
## Cancelled

`variant` · `arrow_flight::sql::gen::action_cancel_query_result::CancelResult::Cancelled` · arrow-flight 59.3.0

```rust
Cancelled
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1004`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The cancellation request is complete. Subsequent requests with
the same payload may return CANCELLED or a NOT_FOUND error.

<a id="op-a5edac4f1b60a076e9cfa6d9"></a>
## Cancelling

`variant` · `arrow_flight::sql::gen::action_cancel_query_result::CancelResult::Cancelling` · arrow-flight 59.3.0

```rust
Cancelling
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1007`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The cancellation request is in progress. The client may retry
the cancellation request.

<a id="op-7d5a65b1deca379eb058735f"></a>
## NotCancellable

`variant` · `arrow_flight::sql::gen::action_cancel_query_result::CancelResult::NotCancellable` · arrow-flight 59.3.0

```rust
NotCancellable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1010`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The query is not cancellable. The client should not retry the
cancellation request.

<a id="op-cc6060307c47432b52ddbdb2"></a>
## Unspecified

`variant` · `arrow_flight::sql::gen::action_cancel_query_result::CancelResult::Unspecified` · arrow-flight 59.3.0

```rust
Unspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1001`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The cancellation status is unknown. Servers should avoid using
this value (send a NOT_FOUND error if the requested query is
not known). Clients can retry the request.
