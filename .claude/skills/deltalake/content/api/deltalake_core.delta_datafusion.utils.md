# `deltalake_core::delta_datafusion::utils`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.delta_datafusion.utils.json`](../model/deltalake_core.delta_datafusion.utils.json)

## Expression

`enum` · `deltalake_core::delta_datafusion::utils::Expression`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.utils.Expression.md)

```rust
enum Expression
```

**Variants**: `DataFusion`, `String`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(val: String) -> Self
fn from(val: Expr) -> Self
fn from(val: &str) -> Self
```

Used to represent user input of either a Datafusion expression or string expression

---
