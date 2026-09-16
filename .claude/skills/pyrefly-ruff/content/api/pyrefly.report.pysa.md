# `pyrefly::report::pysa`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.report.pysa.json`](../model/pyrefly.report.pysa.json)

## PysaFormat

`enum` · `pyrefly::report::pysa::PysaFormat`

```rust
enum PysaFormat
```

**Variants**: `Capnp`, `Json`

**Implements**: `clap_builder::derive::ValueEnum`

**Derives**: Clone, Copy, Debug

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

---

## PysaReporter

`struct` · `pyrefly::report::pysa::PysaReporter`

```rust
struct PysaReporter
```

**Fields**: `module_ids`, `pysa_directory`, `definitions_directory`, `type_of_expressions_directory`, `call_graphs_directory`, `format`

**Methods** (2)

```rust
fn new(pysa_directory: &Path, handles: &[Handle], format: PysaFormat) -> anyhow::Result<Box<Self>>
fn report_module(&self, handle: &Handle, transaction: &Transaction<'_>)
```

Marker stored in `Transaction` to indicate that Pysa reporting is in progress.

---

## PysaSolutions

`struct` · `pyrefly::report::pysa::PysaSolutions`

```rust
struct PysaSolutions
```

**Fields**: `module_id`, `module_index`, `function_base_definitions`, `global_variables`, `is_test_module`

**Derives**: Debug

**Methods** (1)

```rust
fn build(context: &ModuleAnswersContext) -> Arc<Self>
```

Per-module intermediate information required by Pysa for its report step.
Stored as `Arc<PysaSolutions>` inside pyrefly `Solutions` when pysa reporting is enabled.

---
