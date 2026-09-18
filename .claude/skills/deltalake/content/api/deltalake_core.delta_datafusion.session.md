# `deltalake_core::delta_datafusion::session`

Crate `deltalake-core` · 7 public items · structured records in [`model/deltalake_core.delta_datafusion.session.json`](../model/deltalake_core.delta_datafusion.session.json)

## SessionFallbackPolicy

`enum` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.SessionFallbackPolicy.md)

Also reachable as `deltalake::delta_datafusion::SessionFallbackPolicy`, `deltalake_core::delta_datafusion::SessionFallbackPolicy`

```rust
enum SessionFallbackPolicy
```

**Variants**: `InternalDefaults`, `DeriveFromTrait`, `RequireSessionState`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Controls how delta-rs resolves a caller-provided DataFusion `Session` into a `SessionState`.

This is an opt-in knob on operations that accept `with_session_state(...)`. Defaults to
`InternalDefaults` to preserve existing behavior.

---

## create_session

`function` · `deltalake_core::delta_datafusion::session::create_session`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.create_session.md)

Also reachable as `deltalake::delta_datafusion::create_session`, `deltalake_core::delta_datafusion::create_session`

```rust
fn create_session() -> DeltaSessionContext
```

Create a default [`DeltaSessionContext`], a DataFusion session pre-configured with the
settings delta-rs relies on (custom planner, object-store registration, etc.).

---

## create_session_state_with_spill_config

`function` · `deltalake_core::delta_datafusion::session::create_session_state_with_spill_config`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.create_session_state_with_spill_config.md)

Also reachable as `deltalake::delta_datafusion::create_session_state_with_spill_config`, `deltalake_core::delta_datafusion::create_session_state_with_spill_config`

```rust
fn create_session_state_with_spill_config(max_spill_size: Option<usize>, max_temp_directory_size: Option<u64>) -> datafusion::execution::SessionState
```

Create a [`SessionState`] with optional spill-to-disk configuration.

When either parameter is `Some`, a [`FairSpillPool`] memory pool and a sized
[`DiskManagerBuilder`] are wired into the runtime environment so that
DataFusion can spill intermediate results to disk instead of running out of
memory.

# Arguments
* `max_spill_size` – Maximum bytes kept in memory before spilling. `None` uses DataFusion's default (unbounded) pool.
* `max_temp_directory_size` – Maximum disk space for temporary spill files. `None` uses DataFusion's default disk manager.

---

## DeltaParserOptions

`struct` · `deltalake_core::delta_datafusion::session::DeltaParserOptions`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.DeltaParserOptions.md)

Also reachable as `deltalake::delta_datafusion::DeltaParserOptions`, `deltalake_core::delta_datafusion::DeltaParserOptions`

```rust
struct DeltaParserOptions
```

**Derives**: Default

A wrapper for sql_parser's ParserOptions to capture sane default table defaults

---

## DeltaRuntimeEnvBuilder

`struct` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.DeltaRuntimeEnvBuilder.md)

Also reachable as `deltalake::delta_datafusion::DeltaRuntimeEnvBuilder`, `deltalake_core::delta_datafusion::DeltaRuntimeEnvBuilder`

```rust
struct DeltaRuntimeEnvBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn build(self) -> Arc<RuntimeEnv>
fn new() -> Self
fn with_max_spill_size(self, size: usize) -> Self
fn with_max_temp_directory_size(self, size: u64) -> Self
```

A builder for configuring DataFusion RuntimeEnv with Delta-specific defaults

---

## DeltaSessionConfig

`struct` · `deltalake_core::delta_datafusion::session::DeltaSessionConfig`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.DeltaSessionConfig.md)

Also reachable as `deltalake::delta_datafusion::DeltaSessionConfig`, `deltalake_core::delta_datafusion::DeltaSessionConfig`

```rust
struct DeltaSessionConfig
```

**Derives**: Default

A wrapper for Deltafusion's SessionConfig to capture sane default table defaults

---

## DeltaSessionContext

`struct` · `deltalake_core::delta_datafusion::session::DeltaSessionContext`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.session.DeltaSessionContext.md)

Also reachable as `deltalake::delta_datafusion::DeltaSessionContext`, `deltalake_core::delta_datafusion::DeltaSessionContext`

```rust
struct DeltaSessionContext
```

**Derives**: Default

**Methods** (4)

```rust
fn into_inner(self) -> SessionContext
fn new() -> Self
fn state(&self) -> SessionState
fn with_runtime_env(runtime_env: Arc<RuntimeEnv>) -> Self
```

A wrapper for DataFusion's SessionContext with Delta-specific defaults

This provides a way of creating DataFusion sessions with consistent
Delta Lake configuration (case-sensitive identifiers, Delta planner, etc.)

---
