# `datafusion_datasource::display`

Crate `datafusion-datasource` · 1 public items · structured records in [`model/datafusion_datasource.display.json`](../model/datafusion_datasource.display.json)

## FileGroupDisplay

`struct` · `datafusion_datasource::display::FileGroupDisplay`

```rust
struct FileGroupDisplay<'a>
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`

**Derives**: Debug

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> FmtResult
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.display.FileGroupDisplay.md).


A wrapper to customize partitioned group of files display

Prints in the format:
```text
[file1, file2,...]
```

---
