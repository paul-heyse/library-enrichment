# `arrow_flight::sql::ProstMessageExt`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.ProstMessageExt.json).

<a id="op-9784dab83f602b35dae8b7c7"></a>
## ProstMessageExt

`trait` · `arrow_flight::sql::ProstMessageExt` · arrow-flight 59.3.0

```rust
trait ProstMessageExt: prost::Message + Default
```

Source: `src/sql/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ProstMessageExt are useful utility methods for prost::Message types

<a id="op-1a0a37ff9b6cd3fc4cf362fa"></a>
## as_any

`function` · `arrow_flight::sql::ProstMessageExt::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Source: `src/sql/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert this Message to [`Any`](../operations/arrow_flight.sql.Any.md#op-a83fa9be583ec3b019e51a4e)

<a id="op-1ff9b9ca4bfba6a9372be372"></a>
## type_url

`function` · `arrow_flight::sql::ProstMessageExt::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Source: `src/sql/mod.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

type_url for this Message
