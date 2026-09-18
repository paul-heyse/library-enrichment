# Choose conversion failure and output-schema policies explicitly

In Arrow CastOptions, safe=true means null on supported value-conversion failures; safe=false requests an error. Check the specific type pair and output field nullability.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| cast_with_options safe=true | Continue processing malformed supported values | Input nulls and failed conversions can become indistinguishable. |
| cast_with_options safe=false | Reject conversion failures | Still inspect type-specific precision, overflow and formatting rules. |
| DataFusion CAST / TRY_CAST | Conversion belongs inside a query | Use their own documented SQL semantics; do not transfer the Arrow option name blindly. |

## Contract

**shape.** &dyn Array + &DataType + &CastOptions -> Result<ArrayRef, ArrowError>. can_cast_types is a type-pair check, not proof that all values are representable.
Claim `arrow.cast.shape`; upstream_contract_interpretation; evidence: upstream.

**failure.** Default CastOptions.safe is true. Unsupported casts can still error. A null-on-failure array may require nullable output fields.
Claim `arrow.cast.failure`; upstream_contract_interpretation; evidence: upstream.

**schema.** An ArrayRef carries its datatype, not Field/Schema metadata or application validity. Preserve source validity/error distinctions separately if needed.
Claim `arrow.cast.schema`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Specify failure policy before choosing the output field.
- Check conversion-specific decimal/timezone/nested rules and test malformed, valid and input-null controls.

## Limits and unknowns

- Runtime coverage here is Utf8 to Int32; decimal, timestamp, overflow and nested conversion behavior requires its own probe.

## Exact contracts

- [`arrow_cast::cast::cast_with_options`](../operations/arrow_cast.cast.cast_with_options.md#op-3809055ee1c85876012267ab) — `fn cast_with_options(array: &dyn Array, to_type: &DataType, cast_options: &CastOptions<'_>) -> Result<ArrayRef, ArrowError>`
- [`arrow_cast::cast::CastOptions::safe`](../operations/arrow_cast.cast.CastOptions.md#op-e866b77c8af6ede1f3f743eb) — `safe: bool`
- [`arrow_cast::cast::can_cast_types`](../operations/arrow_cast.cast.can_cast_types.md#op-13b2ca9fae0da4f7ebf10839) — `fn can_cast_types(from_type: &DataType, to_type: &DataType) -> bool`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: cast_safe_true_nulls_failed_parses_but_false_errors
