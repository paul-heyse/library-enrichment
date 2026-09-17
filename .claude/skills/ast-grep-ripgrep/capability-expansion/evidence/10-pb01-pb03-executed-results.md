# Evidence 10 — PB01, PB03 and PB04 executed

The three round-2 gating questions that needed a Rust compile. All three are now **answered**,
and two of them **contradict round-1 evidence**, which is corrected in place.

- Probe crate: [`probes/delta-arrow-probe/`](probes/delta-arrow-probe/)
- Captures: [`probes/PB01-PB03-delta-arrow.out.txt`](probes/PB01-PB03-delta-arrow.out.txt),
  [`probes/PB04-deltaops.out.txt`](probes/PB04-deltaops.out.txt)
- Mechanism read from source: [`probes/PB03-kernel-conversion-source.md`](probes/PB03-kernel-conversion-source.md)
- Pins: deltalake git rev `58f07cd6…`, arrow 59.3.0 (`canonical_extension_types`),
  datafusion 55.1.0, rustc 1.98.1 — the exact set the plan targets (evidence 09 P02)

---

## PB01 — Does Arrow extension-type metadata survive a Delta round trip? **YES**

```
wrote  entity_id : ARROW:extension:metadata="{\"v\":1}", ARROW:extension:name="codesearch.entity_id"
wrote  note      : codesearch.family="rustdoc-json",     codesearch.precision="exact"

after normalize_for_delta
       entity_id : both keys unchanged
       note      : both keys unchanged

read back
       entity_id : ARROW:extension:metadata="{\"v\":1}", ARROW:extension:name="codesearch.entity_id"
       note      : codesearch.family="rustdoc-json",     codesearch.precision="exact"

VERDICT  extension metadata survived : true
         control metadata survived   : true
```

**All four keys round-trip byte-for-byte.** Extension-type identity is a *storage* property, not
a re-attachment step. The matching kernel source documents the rule — *"All other entries pass
through unchanged"* in the write direction, with only the two parquet-field-id spellings
special-cased in either direction.

### Design consequence

Evidence 03 §2 and the plan's §3.2 proposed a storage/memory split in which the extension type
lives only in memory and is re-attached on read. **That step is unnecessary.** The extension name
and its metadata are carried by the Delta schema, so:

- `codesearch.entity_id`, `codesearch.content_hash`, `codesearch.mechanism_id`,
  `codesearch.source_anchor` and `codesearch.native_handle` can be declared once at table-create
  time and will still be there on read;
- the `boundary: re-attach on read` column in the plan's §3.1 table becomes `native`;
- the risk-register row "extension metadata stripped by Delta" is **closed**.

### The control earned its place

The first run of this probe reported that **both** fields had lost **exactly one** key. That
symmetry is not something delta-rs would do — it pointed straight at the probe. The cause was
mine: `StructField::with_metadata` **replaces** a field's metadata rather than extending it, so
calling it once per entry in a loop keeps only whichever entry came last out of the `HashMap`.
Without the control field the result would have read as a genuine delta-rs finding, and the plan
would have been built around a defect in the probe.

That is itself a recorded API fact: **`with_metadata` is not additive.**

---

## PB03 — Which Arrow types can be persisted? **Almost all of them — and several narrow silently**

Three phases, because the three ask different questions.

### Phase A — `deltalake::kernel::DataType::try_from_arrow`

| Arrow type | accepted | becomes |
|---|---|---|
| `Utf8`, `Int64`, `Int32`, `Boolean`, `Binary`, `Timestamp(µs,None)` | yes | the obvious primitive |
| `Struct`, `List<Utf8>`, `Map<Utf8,Utf8>` | yes | `Struct` / `Array` / `Map` |
| **`Dictionary(Int32,Utf8)`** | **yes** | `Primitive(String)` |
| **`FixedSizeBinary(32)`** | **yes** | `Primitive(Binary)` |
| **`UInt32`** | **yes** | `Primitive(Integer)` |
| **`UInt64`** | **yes** | `Primitive(Long)` |
| **`Utf8View`** | **yes** | `Primitive(String)` |
| **`BinaryView`** | **yes** | `Primitive(Binary)` |
| **`LargeUtf8`**, **`LargeList<Utf8>`** | **yes** | `String` / `Array` |
| `Decimal128(10,2)`, `Date32` | yes | `Decimal` / `Date` |
| `Float16` | **no** | `Schema error: Invalid data type for Delta Lake` |

Of eleven types evidence 04 §1 listed as absent from Delta, **ten are accepted**. Only `Float16`
is rejected.

### Phase B — `normalize_for_delta` changes nothing

Every one of the twenty inputs came back **identical**. Not one rewrite.

This is the trap. `normalize_for_delta` is the function whose name promises to tell you what
Delta will do to your schema, and it is infallible — so inspecting it looks like due diligence.
It reports no change for `UInt32`, `FixedSizeBinary(32)` and `Dictionary`, all of which the write
path then narrows. **`normalize_for_delta` is not a preview of persistence.**

### Phase C — end-to-end create + write + read back (authoritative)

| written as | read back as | lost |
|---|---|---|
| `Dictionary(Int32,Utf8)` | `Utf8` | the encoding |
| `FixedSizeBinary(32)` | `Binary` | **the width** |
| `UInt32` | `Int32` | **the upper half of the range** |
| `UInt64` | `Int64` | **the upper half of the range** |
| `Utf8View` | `Utf8` | the view representation |
| `BinaryView` | `Binary` | the view representation |
| `LargeUtf8` | `Utf8` | the 64-bit offsets |
| `LargeList<Utf8>` | `List(Utf8, field: 'element')` | 64-bit offsets, **and the child field is renamed** |
| `List<Utf8>` (child `item`) | `List(Utf8, field: 'element')` | **the child field is renamed** |
| `Map<Utf8,Utf8>` (`entries`/`keys`/`values`) | `Map("key_value" … "key"/"value")` | **all three names rewritten** |
| `Float16` | — | rejected at create |
| everything else | unchanged | — |

### Design consequences, in order of severity

1. **Byte offsets must be `Int64`, not `UInt32`.** The plan's §3.1 wanted `UInt32` for
   `SourceAnchor.native_range`. A source file larger than 2 GiB yields offsets above `i32::MAX`,
   and the conversion is a *schema* mapping — no value check, no error. Silent corruption.
   `Int64` is the only safe choice; `Integer` would also silently accept.
2. **Nested child field names are rewritten to Parquet conventions.** `item` → `element`,
   `entries`/`keys`/`values` → `key_value`/`key`/`value`. Any schema-equality check that compares
   an authored Arrow schema against a read-back one will fail on names, not types. The plan must
   compare *normalised* schemas, or author the Parquet spellings from the start.
3. **A fixed-width hash column is not fixed-width once stored.** `FixedSizeBinary(32)` → `Binary`.
   The width guarantee has to be re-imposed by a Delta `CheckConstraints` length predicate —
   which is a good argument for the constraint machinery the plan already specifies (§4.2), now
   with a concrete first customer.
4. **Dictionary encoding is an in-memory concern only** — as the plan assumed, but for the
   opposite reason. It is flattened, not rejected.
5. **The general rule** replaces evidence 04 §1's rejection list:

   > Choose each column's Arrow type so that it is a **fixed point** of the Delta conversion —
   > written type and read-back type identical. Anything else round-trips into a different type
   > with nothing raised.

---

## PB04 — Is `DeltaOps` reachable? **NO**

```
examples/pb04_deltaops.rs:18:5: error[E0432]: unresolved import `deltalake::DeltaOps`
```

Both `deltalake::DeltaOps` and `deltalake::operations::DeltaOps` are unresolved. The conventional
delta-rs entry point **does not exist at this rev**.

The `deltalake` skill's index was therefore correct, not incomplete — evidence 04 §5 raised both
possibilities and the compile settles it. The individual operation builders (`CreateBuilder`,
`WriteBuilder`, `MergeBuilder`, `ConstraintBuilder`, …) are the only entry point, which is what
the plan's call sites already assume. **No plan change required.**

---

## A note on the harness itself

The first `cargo build` was run as `cargo build 2>&1 | tail -40`. Cargo failed with three errors;
`tail` exited 0; the background-task notification reported **"completed (exit code 0)"**. The
build had not succeeded. `${PIPESTATUS[0]}` is the reading that matters, and a pipe through
`tail` also withholds all output until the process ends, which makes progress unobservable.

This is the same class of error the whole capability catalog exists to prevent: a confident
answer from the wrong oracle. It is recorded here rather than quietly fixed.

Three compile errors then surfaced one real API fact: **`try_from_arrow` is a trait method**
(`deltalake::kernel::engine::arrow_conversion::TryFromArrow`), not an inherent one. A flat method
index shows the signature on `DataType` and the trait requirement is easy to miss.
