# `datafusion_ffi::expr::distribution`

Crate `datafusion-ffi` · 6 public items · structured records in [`model/datafusion_ffi.expr.distribution.json`](../model/datafusion_ffi.expr.distribution.json)

## FFI_Distribution

`enum` · `datafusion_ffi::expr::distribution::FFI_Distribution`

```rust
enum FFI_Distribution
```

**Variants**: `Uniform`, `Exponential`, `Gaussian`, `Bernoulli`, `Generic`

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &Distribution) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.distribution.FFI_Distribution.md).


A stable struct for sharing [`Distribution`] across FFI boundaries.
See ['Distribution'] for the meaning of each variant.

---

## FFI_BernoulliDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_BernoulliDistribution`

```rust
struct FFI_BernoulliDistribution
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &BernoulliDistribution) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.distribution.FFI_BernoulliDistribution.md).


---

## FFI_ExponentialDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_ExponentialDistribution`

```rust
struct FFI_ExponentialDistribution
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &ExponentialDistribution) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.distribution.FFI_ExponentialDistribution.md).


---

## FFI_GaussianDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_GaussianDistribution`

```rust
struct FFI_GaussianDistribution
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &GaussianDistribution) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.distribution.FFI_GaussianDistribution.md).


---

## FFI_GenericDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_GenericDistribution`

```rust
struct FFI_GenericDistribution
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &GenericDistribution) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.distribution.FFI_GenericDistribution.md).


---

## FFI_UniformDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_UniformDistribution`

```rust
struct FFI_UniformDistribution
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &UniformDistribution) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.distribution.FFI_UniformDistribution.md).


---
