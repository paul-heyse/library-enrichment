# `datafusion_physical_plan::display::DisplayAs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.display.DisplayAs.json).

<a id="op-87493fd4345d07ecabcbed6a"></a>
## DisplayAs

`trait` · `datafusion_physical_plan::display::DisplayAs` · datafusion-physical-plan 55.1.0

```rust
trait DisplayAs
```

Source: `src/display.rs:1440`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Trait for types which could have additional details when formatted in `Verbose` mode

<a id="op-667b669d046ad0f22bc0d78a"></a>
## fmt_as

`function` · `datafusion_physical_plan::display::DisplayAs::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

Source: `src/display.rs:1445`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Format according to `DisplayFormatType`, used when verbose representation looks
different from the default one

Should not include a newline
