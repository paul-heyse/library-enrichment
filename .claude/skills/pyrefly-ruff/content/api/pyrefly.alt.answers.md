# `pyrefly::alt::answers`

Crate `pyrefly` · 15 public items · structured records in [`model/pyrefly.alt.answers.json`](../model/pyrefly.alt.answers.json)

## AttributeReferenceKind

`enum` · `pyrefly::alt::answers::AttributeReferenceKind`

```rust
enum AttributeReferenceKind
```

**Variants**: `Textual`, `ConstructorCall`

**Derives**: Clone, Copy, Debug

How the source text at a reference range relates to the attribute it resolves to.

---

## OverloadedCallee

`enum` · `pyrefly::alt::answers::OverloadedCallee`

```rust
enum OverloadedCallee
```

**Variants**: `Resolved`, `Candidates`

**Derives**: Clone, Debug

---

## AnswerEntry

`struct` · `pyrefly::alt::answers::AnswerEntry`

```rust
struct AnswerEntry<K: Keyed>
```

**Derives**: Debug, Default

Result slots indexed identically to `Bindings`.

---

## AnswerSlot

`struct` · `pyrefly::alt::answers::AnswerSlot`

```rust
struct AnswerSlot<T: Send>
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug, Default

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A first-write-wins result slot containing an aligned answer pointer.
Null is unpublished, `PENDING_TAG` marks a pending pointer, and `ALIAS_TAG`
marks a pointer to another slot's allocation. An untagged non-null pointer is
published and owned by this slot.

The tags are independent, so a pending alias carries both. No caller reserves
an alias today, but the slot supports it because a tag combination that costs
nothing to handle is cheaper than a rule that every caller has to remember.

---

## AnswerTable

`struct` · `pyrefly::alt::answers::AnswerTable`

```rust
struct AnswerTable
```

**Fields**: `types`, `expectations`, `type_aliases`, `exports`, `decorators`, `decorated_functions`, `undecorated_functions`, `func_defs`, `classes`, `tparams`, `class_base_types`, `class_fields`, `class_synthesized_fields`, `variance`, `class_checks`, `annotations`, `class_metadata`, `django_relations`, `class_mros`, `class_disjoint_bases`, `abstract_class_check`, `class_subscript_symmetry`, `legacy_tparams`, `yields`, `yield_froms`

**Implements**: `pyrefly::binding::table::TableKeyed`

**Derives**: Debug, Default

**via `pyrefly::binding::table::TableKeyed`**

```rust
fn get(&self) -> &Self::Value
fn get_mut(&mut self) -> &mut Self::Value
```

---

## Answers

`struct` · `pyrefly::alt::answers::Answers`

```rust
struct Answers
```

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Debug

**Methods** (17)

```rust
fn add_parent_method_mapping(&self, child_range: TextRange, parent_module: ModulePath, parent_range: TextRange)
fn get_all_overload_trace(&self, range: TextRange) -> Option<(Vec<Callable>, Option<usize>)>
fn get_chosen_overload_trace(&self, range: TextRange) -> Option<Type>
fn get_expected_type_trace(&self, range: TextRange) -> Option<Type>
fn get_type_at(&self, idx: Idx<Key>) -> Option<Type>
fn get_type_trace(&self, range: TextRange) -> Option<Type>
fn heap(&self) -> &TypeHeap
fn new(bindings: &Bindings, solver: Solver, enable_index: bool, enable_trace: bool) -> Self
fn publish_reserved_preliminary<Ans: LookupAnswer>(&self, reserved: &mut ReservedSlot<'_, '_, '_, Ans>)
fn reserve_preliminary(&self, any_idx: &AnyIdx, answer: AnyAnswer) -> bool
fn rollback_reserved_if_pending_preliminary<Ans: LookupAnswer>(&self, reserved: &mut ReservedSlot<'_, '_, '_, Ans>) -> bool
fn solve<Ans: LookupAnswer>(&self, exports: &dyn LookupExport, answers: &Ans, bindings: &Bindings, errors: &ErrorCollector, stdlib: &Stdlib, uniques: &UniqueFactory, compute_everything: bool, recursion_limit_config: Option<RecursionLimitConfig>, pysa_context: Option<&report::pysa::context::ModuleAnswersContext>, enable_cinderx_solutions: bool) -> Solutions
fn solve_exported_key<'ctx, 'answer, Ans: LookupAnswer, K: Solve<Ans> + Exported>(&'answer self, exports: &'ctx dyn LookupExport, answers: &'ctx Ans, bindings: &'answer Bindings, errors: &'ctx ErrorCollector, stdlib: &'ctx Stdlib, uniques: &'ctx UniqueFactory, key: Hashed<&K>, thread_state: &'answer ThreadState, answer_scope: &'answer AnswerScope) -> Option<&'answer K::Answer> where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn solve_idx_erased<Ans: LookupAnswer>(&self, any_idx: &AnyIdx, answers: &Ans, bindings: &Bindings, exports: &dyn LookupExport, errors: &ErrorCollector, stdlib: &Stdlib, uniques: &UniqueFactory, thread_state: &ThreadState, answer_scope: &AnswerScope)
fn solver(&self) -> &Solver
fn table(&self) -> &AnswerTable
fn try_get_getter_for_range(&self, range: TextRange) -> Option<Type>
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, bindings: &Bindings) -> fmt::Result
```

Invariants:

* Every module name referenced anywhere MUST be present
  in the `exports` and `bindings` map.
* Every key referenced in `bindings`/`answers` MUST be present.

We never issue contains queries on these maps.

---

## Index

`struct` · `pyrefly::alt::answers::Index`

```rust
struct Index
```

**Fields**: `externally_defined_variable_references`, `renamed_imports`, `externally_defined_attribute_references`, `constructor_references`, `parent_methods_map`

**Derives**: Debug, Default

The index stores reference edges that cannot be recovered by scanning the current module's AST.
This includes references to external definitions and implicit constructor-protocol references.

---

## OverloadTrace

`struct` · `pyrefly::alt::answers::OverloadTrace`

```rust
struct OverloadTrace
```

**Derives**: Clone, Debug

---

## Solutions

`struct` · `pyrefly::alt::answers::Solutions`

```rust
struct Solutions
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (11)

```rust
fn changed_exports(&self, other: &Self, changed: &mut ModuleChanges)
fn changed_exports_vs_answers(&self, old_bindings: &Bindings, old_answers: &Answers, changed: &mut ModuleChanges)
fn cinderx_solutions(&self) -> Option<&Arc<CinderxSolutions>>
fn first_difference<'a>(&'a self, other: &'a Self) -> Option<SolutionsDifference<'a>>
fn get<K: Exported>(&self, key: &K) -> &<K as Keyed>::Answer where SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_hashed<K: Exported>(&self, key: Hashed<&K>) -> &<K as Keyed>::Answer where SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_hashed_opt<K: Exported>(&self, key: Hashed<&K>) -> Option<&<K as Keyed>::Answer> where SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_index(&self) -> Option<Arc<Mutex<Index>>>
fn metadata(&self) -> &Arc<BindingsMetadata>
fn module_ranges(&self) -> &Arc<ModuleRanges>
fn pysa_solutions(&self) -> Option<&Arc<PysaSolutions>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## SolutionsDifference

`struct` · `pyrefly::alt::answers::SolutionsDifference`

```rust
struct SolutionsDifference<'a>
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## SolutionsEntry

`struct` · `pyrefly::alt::answers::SolutionsEntry`

```rust
struct SolutionsEntry<K: Keyed>
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Debug, Default

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

---

## SolutionsTable

`struct` · `pyrefly::alt::answers::SolutionsTable`

```rust
struct SolutionsTable
```

**Fields**: `types`, `expectations`, `type_aliases`, `exports`, `decorators`, `decorated_functions`, `undecorated_functions`, `func_defs`, `classes`, `tparams`, `class_base_types`, `class_fields`, `class_synthesized_fields`, `variance`, `class_checks`, `annotations`, `class_metadata`, `django_relations`, `class_mros`, `class_disjoint_bases`, `abstract_class_check`, `class_subscript_symmetry`, `legacy_tparams`, `yields`, `yield_froms`

**Implements**: `pyrefly::binding::table::TableKeyed`

**Derives**: Debug, Default

**via `pyrefly::binding::table::TableKeyed`**

```rust
fn get(&self) -> &Self::Value
fn get_mut(&mut self) -> &mut Self::Value
```

---

## TraceSideEffects

`struct` · `pyrefly::alt::answers::TraceSideEffects`

```rust
struct TraceSideEffects
```

**Fields**: `types`, `overloaded_callees`, `invoked_properties`, `expected_types`

**Derives**: Clone, Debug, Default

Accumulates trace events during a single calculation.
Published to `Traces` only when the calculation result is committed.

---

## Traces

`struct` · `pyrefly::alt::answers::Traces`

```rust
struct Traces
```

**Implements**: `pyrefly_util::visit::VisitMut`

**Derives**: Debug, Default

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

---

## LookupAnswer

`trait` · `pyrefly::alt::answers::LookupAnswer`

```rust
trait LookupAnswer: Sized
```

**Methods** (5)

```rust
fn get<'answer, K: Solve<Self> + Exported>(&self, module: ModuleName, path: Option<&ModulePath>, k: &K, stack: &'answer ThreadState, answer_scope: &'answer AnswerScope) -> Option<&'answer K::Answer> where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_class_fields(&self, cls: &Class) -> Option<&ClassFields>
fn publish_reserved_in_module(&self, _reserved: &mut ReservedSlot<'_, '_, '_, Self>) -> bool
fn reserve_in_module(&self, _calc_id: &CalcId, _answer: AnyAnswer) -> Option<Arc<Answers>>
fn solve_idx_erased(&self, _calc_id: &CalcId, _thread_state: &ThreadState, _answer_scope: &AnswerScope) -> bool
```

---
