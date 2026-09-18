# `arrow_select::interleave::interleave`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.interleave.interleave.json).

<a id="op-be8ca59e89328880f4e23523"></a>
## interleave

`function` · `arrow_select::interleave::interleave` · arrow-select 59.3.0

```rust
fn interleave(values: &[&dyn Array], indices: &[(usize, usize)]) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/interleave.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).


Takes elements by index from a list of [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), creating a new [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) from those values.

Each element in `indices` is a pair of `usize` with the first identifying the index
of the [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) in `values`, and the second the index of the value within that [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

```text
┌─────────────────┐      ┌─────────┐                                  ┌─────────────────┐
│        A        │      │ (0, 0)  │        interleave(               │        A        │
├─────────────────┤      ├─────────┤          [values0, values1],     ├─────────────────┤
│        D        │      │ (1, 0)  │          indices                 │        B        │
└─────────────────┘      ├─────────┤        )                         ├─────────────────┤
  values array 0         │ (1, 1)  │      ─────────────────────────▶  │        C        │
                         ├─────────┤                                  ├─────────────────┤
                         │ (0, 1)  │                                  │        D        │
                         └─────────┘                                  └─────────────────┘
┌─────────────────┐       indices
│        B        │        array
├─────────────────┤                                                    result
│        C        │
├─────────────────┤
│        E        │
└─────────────────┘
  values array 1
```

For selecting values by index from a single array see [`crate::take`](../modules/arrow_select.take.md#op-5d8ac342b21d47ca97c2d285)
