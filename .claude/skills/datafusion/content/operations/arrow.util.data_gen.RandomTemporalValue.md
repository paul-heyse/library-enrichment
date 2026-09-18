# `arrow::util::data_gen::RandomTemporalValue`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.data_gen.RandomTemporalValue.json).

<a id="op-a48c82ed537ab8a17a4ae65c"></a>
## RandomTemporalValue

`trait` · `arrow::util::data_gen::RandomTemporalValue` · arrow 59.3.0

```rust
trait RandomTemporalValue: ArrowTemporalType
```

Source: `src/util/data_gen.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Useful for testing. The range of values are not likely to be representative of the
actual bounds.

<a id="op-02633c403b8be6cb911b9e85"></a>
## gen_range

`function` · `arrow::util::data_gen::RandomTemporalValue::gen_range` · arrow 59.3.0

```rust
fn gen_range<R: Rng>(rng: &mut R) -> Self::Native where Self::Native: SampleUniform
```

Source: `src/util/data_gen.rs:515`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Generate a random value within the range of the type

<a id="op-5a36f81f35de798bb141e029"></a>
## random

`function` · `arrow::util::data_gen::RandomTemporalValue::random` · arrow 59.3.0

```rust
fn random<R: Rng>(rng: &mut R) -> Self::Native where Self::Native: SampleUniform
```

Source: `src/util/data_gen.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Generate a random value of the type

<a id="op-e7b33bf229bf125ec06f480d"></a>
## value_range

`function` · `arrow::util::data_gen::RandomTemporalValue::value_range` · arrow 59.3.0

```rust
fn value_range() -> impl SampleRange<Self::Native>
```

Source: `src/util/data_gen.rs:512`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Returns the range of values for `impl`'d type
