# `datafusion_ffi::table_provider`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.table_provider.json`](../model/datafusion_ffi.table_provider.json)

## FFI_TableProvider

`struct` · `datafusion_ffi::table_provider::FFI_TableProvider`

```rust
struct FFI_TableProvider
```

**Fields**: `statistics`, `logical_codec`, `version`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(provider: Arc<dyn TableProvider>, can_support_pushdown_filters: bool, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
fn new_with_ffi_codec(provider: Arc<dyn TableProvider>, can_support_pushdown_filters: bool, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing [`TableProvider`] across FFI boundaries.

# Struct Layout

The following description applies to all structs provided in this crate.

Each of the exposed structs in this crate is provided with a variant prefixed
with `Foreign`. This variant is designed to be used by the consumer of the
foreign code. The `Foreign` structs should _never_ access the `private_data`
fields. Instead they should only access the data returned through the function
calls defined on the `FFI_` structs. The second purpose of the `Foreign`
structs is to contain additional data that may be needed by the traits that
are implemented on them. Some of these traits require borrowing data which
can be far more convenient to be locally stored.

For example, we have a struct `FFI_TableProvider` to give access to the
`TableProvider` functions like `table_type()` and `scan()`. If we write a
library that wishes to expose it's `TableProvider`, then we can access the
private data that contains the Arc reference to the `TableProvider` via
`FFI_TableProvider`. This data is local to the library.

If we have a program that accesses a `TableProvider` via FFI, then it
will use `ForeignTableProvider`. When using `ForeignTableProvider` we **must**
not attempt to access the `private_data` field in `FFI_TableProvider`. If a
user is testing locally, you may be able to successfully access this field, but
it will only work if you are building against the exact same version of
`DataFusion` for both libraries **and** the same compiler. It will not work
in general.

It is worth noting that which library is the `local` and which is `foreign`
depends on which interface we are considering. For example, suppose we have a
Python library called `my_provider` that exposes a `TableProvider` called
`MyProvider` via `FFI_TableProvider`. Within the library `my_provider` we can
access the `private_data` via `FFI_TableProvider`. We connect this to
`datafusion-python`, where we access it as a `ForeignTableProvider`. Now when
we call `scan()` on this interface, we have to pass it a `FFI_SessionConfig`.
The `SessionConfig` is local to `datafusion-python` and **not** `my_provider`.
It is important to be careful when expanding these functions to be certain which
side of the interface each object refers to.

---

## ForeignTableProvider

`struct` · `datafusion_ffi::table_provider::ForeignTableProvider`

```rust
struct ForeignTableProvider
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug, Send, Sync

**via `datafusion_session::table::TableProvider`**

```rust
async fn insert_into(&self, session: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn statistics(&self) -> Option<Statistics>
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_TableProvider to interact with the foreign table provider.

---
