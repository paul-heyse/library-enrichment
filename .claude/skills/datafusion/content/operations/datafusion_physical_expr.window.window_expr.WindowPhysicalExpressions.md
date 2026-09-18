# `datafusion_physical_expr::window::window_expr::WindowPhysicalExpressions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.window_expr.WindowPhysicalExpressions.json).

<a id="op-b9235f2c0e74bebc80ae293a"></a>
## WindowPhysicalExpressions

`struct` · `datafusion_physical_expr::window::window_expr::WindowPhysicalExpressions` · datafusion-physical-expr 55.1.0

```rust
struct WindowPhysicalExpressions
```

Source: `src/window/window_expr.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Stores the physical expressions used inside the `WindowExpr`.

<a id="op-a5b85a918ad254301e02b0e9"></a>
## args

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowPhysicalExpressions::args` · datafusion-physical-expr 55.1.0

```rust
args: Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/window/window_expr.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Window function arguments

<a id="op-81f36a0338c6a968977a976d"></a>
## order_by_exprs

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowPhysicalExpressions::order_by_exprs` · datafusion-physical-expr 55.1.0

```rust
order_by_exprs: Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/window/window_expr.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

ORDER BY expressions

<a id="op-c273a9e94ab650d14388dd98"></a>
## partition_by_exprs

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowPhysicalExpressions::partition_by_exprs` · datafusion-physical-expr 55.1.0

```rust
partition_by_exprs: Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/window/window_expr.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

PARTITION BY expressions
