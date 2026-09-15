# `arrow::util::data_gen`

Crate `arrow` · 3 public items · structured records in [`model/arrow.util.data_gen.json`](../model/arrow.util.data_gen.json)

## create_random_array

`function` · `arrow::util::data_gen::create_random_array`

```rust
fn create_random_array(field: &Field, size: usize, null_density: f32, true_density: f32) -> error::Result<ArrayRef>
```

Create a random [ArrayRef] from a [DataType] with a length,
null density and true density (for [BooleanArray]).

# Arguments

* `field` - The field containing the data type for which to create a random array
* `size` - The number of elements in the generated array
* `null_density` - The approximate fraction of null values in the resulting array (0.0 to 1.0)
* `true_density` - The approximate fraction of true values in boolean arrays (0.0 to 1.0)

---

## create_random_batch

`function` · `arrow::util::data_gen::create_random_batch`

```rust
fn create_random_batch(schema: SchemaRef, size: usize, null_density: f32, true_density: f32) -> error::Result<RecordBatch>
```

Create a random [RecordBatch] from a schema

---

## RandomTemporalValue

`trait` · `arrow::util::data_gen::RandomTemporalValue`

```rust
trait RandomTemporalValue: ArrowTemporalType
```

**Implementors** (10)

- `arrow_array::types::Date32Type`
- `arrow_array::types::Date64Type`
- `arrow_array::types::Time32MillisecondType`
- `arrow_array::types::Time32SecondType`
- `arrow_array::types::Time64MicrosecondType`
- `arrow_array::types::Time64NanosecondType`
- `arrow_array::types::TimestampMicrosecondType`
- `arrow_array::types::TimestampMillisecondType`
- `arrow_array::types::TimestampNanosecondType`
- `arrow_array::types::TimestampSecondType`

**Methods** (3)

```rust
fn gen_range<R: Rng>(rng: &mut R) -> Self::Native where Self::Native: SampleUniform
fn random<R: Rng>(rng: &mut R) -> Self::Native where Self::Native: SampleUniform
fn value_range() -> impl SampleRange<Self::Native>
```

Useful for testing. The range of values are not likely to be representative of the
actual bounds.

---
