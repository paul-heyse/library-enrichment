# `datafusion_physical_plan::joins::array_map`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.joins.array_map.json`](../model/datafusion_physical_plan.joins.array_map.json)

## ArrayMap

`struct` · `datafusion_physical_plan::joins::array_map::ArrayMap`

```rust
struct ArrayMap
```

A dense map for single-column integer join keys within a limited range.

Maps join keys to build-side indices using direct array indexing:
`data[val - min_val_in_build_side] -> val_idx_in_build_side + 1`.

NULL values are ignored on both the build side and the probe side.

# Handling Negative Numbers with `wrapping_sub`

This implementation supports signed integer ranges (e.g., `[-5, 5]`) efficiently by
treating them as `u64` (Two's Complement) and relying on the bitwise properties of
wrapping arithmetic (`wrapping_sub`).

In Two's Complement representation, `a_signed - b_signed` produces the same bit pattern
as `a_unsigned.wrapping_sub(b_unsigned)` (modulo 2^N). This allows us to perform
range calculations and zero-based index mapping uniformly for both signed and unsigned
types without branching.

## Examples

Consider an `Int64` range `[-5, 5]`.
* `min_val (-5)` casts to `u64`: `...11111011` (`u64::MAX - 4`)
* `max_val (5)` casts to `u64`: `...00000101` (`5`)

**1. Range Calculation**

```text
In modular arithmetic, this is equivalent to:
  (5 - (2^64 - 5)) mod 2^64
= (5 - 2^64 + 5) mod 2^64
= (10 - 2^64) mod 2^64
= 10

```
The resulting `range` (10) correctly represents the size of the interval `[-5, 5]`.

**2. Index Lookup (in `get_matched_indices_with_limit_offset`)**

For a probe value of `0` (which is stored as `0u64`):
```text
In modular arithmetic, this is equivalent to:
  (0 - (2^64 - 5)) mod 2^64
= (-2^64 + 5) mod 2^64
= 5
```
This correctly maps `-5` to index `0`, `0` to index `5`, etc.

---
