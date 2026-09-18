# `datafusion_common::param_value`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.param_value.json`](../model/datafusion_common.param_value.json)

## ParamValues

`enum` · `datafusion_common::param_value::ParamValues`

Also reachable as `datafusion::common::ParamValues`, `datafusion_common::ParamValues`

```rust
enum ParamValues
```

**Variants**: `List`, `Map`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn get_placeholders_with_values(&self, id: &str) -> Result<ScalarAndMetadata>
fn verify(&self, expect: &[DataType]) -> Result<()>
fn verify_fields(&self, expect: &[FieldRef]) -> Result<()>
```

**via `core::convert::From`**

```rust
fn from(value: Vec<(K, ScalarValue)>) -> Self
fn from(value: HashMap<K, ScalarValue>) -> Self
fn from(value: Vec<ScalarValue>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.param_value.ParamValues.md).


The parameter value corresponding to the placeholder

---
