# `pyrefly::solver::solver`

Crate `pyrefly` · 14 public items · structured records in [`model/pyrefly.solver.solver.json`](../model/pyrefly.solver.solver.json)

## ArgumentSide

`enum` · `pyrefly::solver::solver::ArgumentSide`

```rust
enum ArgumentSide
```

**Variants**: `Got`, `Want`, `NotAnalyzingACall`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

Which side of a call argument check we are currently analyzing.

`NotAnalyzingACall` is required because `is_subset_eq` is also called in
contexts that are unrelated to callable argument-vs-parameter checks.

---

## OpenTypedDictSubsetError

`enum` · `pyrefly::solver::solver::OpenTypedDictSubsetError`

```rust
enum OpenTypedDictSubsetError
```

**Variants**: `MissingField`, `UnknownFields`

**Derives**: Clone, Debug

---

## PinError

`enum` · `pyrefly::solver::solver::PinError`

```rust
enum PinError
```

**Variants**: `ImplicitPartialContained`, `UnfinishedQuantified`

**Derives**: Debug

---

## SubsetError

`enum` · `pyrefly::solver::solver::SubsetError`

```rust
enum SubsetError
```

**Variants**: `PosParamName`, `CallableMissingPositionalParameters`, `TypeVarSpecialization`, `MissingAttribute`, `IncompatibleAttribute`, `TypedDict`, `OpenTypedDict`, `Shape`, `ShapedArraySubtyping`, `InternalError`, `TypeOfProtocolNeedsConcreteClass`, `TypeCannotAcceptSpecialForms`, `OpenTypedDictKwargs`, `Other`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn to_error_msg(self) -> Option<String>
```

If a got <: want check fails, the failure reason

---

## SubsetWithSnapshotResult

`enum` · `pyrefly::solver::solver::SubsetWithSnapshotResult`

```rust
enum SubsetWithSnapshotResult
```

**Variants**: `Ok`, `Err`

**Methods** (1)

```rust
fn is_ok(&self) -> bool
```

Result of `with_snapshot`, which performs an `is_subset_eq` call with var snapshotting.

---

## TypeVarSpecializationError

`enum` · `pyrefly::solver::solver::TypeVarSpecializationError`

```rust
enum TypeVarSpecializationError
```

**Variants**: `BadShapeExtensionSpecialization`, `ConflictingShapeExtensionSpecialization`, `BadBoundSpecialization`, `BadConstraintSpecialization`, `IncompatibleOverloadResidual`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn error_kind(&self) -> ErrorKind
fn to_error_msg<Ans: LookupAnswer>(self, ans: &AnswersSolver<'_, '_, Ans>) -> String
```

---

## TypedDictSubsetError

`enum` · `pyrefly::solver::solver::TypedDictSubsetError`

```rust
enum TypedDictSubsetError
```

**Variants**: `MissingField`, `ReadOnlyMismatch`, `RequiredMismatch`, `NotRequiredReadWriteMismatch`, `InvariantFieldMismatch`, `CovariantFieldMismatch`

**Derives**: Clone, Debug

---

## ArgumentKey

`struct` · `pyrefly::solver::solver::ArgumentKey`

```rust
struct ArgumentKey
```

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(index: usize) -> Self
```

Which argument of the call being checked is in hand.

---

## CallContext

`struct` · `pyrefly::solver::solver::CallContext`

```rust
struct CallContext<'subset>
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn for_argument_outside_call() -> Self
fn outside() -> Self
fn residual_witness(&self) -> Option<&ResidualWitnessContext>
fn residual_witness_mut(&mut self) -> Option<&mut ResidualWitnessContext>
fn take_residual_witness(&mut self) -> Option<ResidualWitnessContext>
fn with_argument(self, argument: ArgumentKey) -> Self
fn with_argument_side(self, argument_side: ArgumentSide) -> Self
fn with_outside_context(self) -> Self
fn with_residual_witness(self, witness: ResidualWitnessContext) -> Self
```

Recursive subset-checking context. Boundary ownership remains with `CallBoundary`.

---

## QuantifiedHandle

`struct` · `pyrefly::solver::solver::QuantifiedHandle`

```rust
struct QuantifiedHandle
```

**Derives**: Debug

**Methods** (2)

```rust
fn empty() -> Self
fn partition_by(self, ty: &Type) -> (Self, Self)
```

A linear obligation to finalize these created Var IDs. Handles may contain
Vars that later share union-find roots; they do not exclusively own roots.

---

## ResidualWitnessContext

`struct` · `pyrefly::solver::solver::ResidualWitnessContext`

```rust
struct ResidualWitnessContext
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn for_forall(argument: ArgumentKey, vars: &QuantifiedHandle, want: &Type, argument_side: ArgumentSide) -> Self
fn for_overload(argument: ArgumentKey, eligible_vars: &[Var], argument_side: ArgumentSide) -> Self
```

---

## Solver

`struct` · `pyrefly::solver::solver::Solver`

```rust
struct Solver
```

**Fields**: `infer_with_first_use`, `heap`, `tensor_shapes`, `strict_callable_subtyping`, `strict_partial_subtyping`, `spec_compliant_overloads`, `legacy_overload_expansion`

**Implements**: `core::fmt::Display`

**Derives**: Debug

**Methods** (45)

```rust
fn add_lower_bound(&self, v: Var, bound: Type, is_subset: &mut dyn FnMut(&Type, &Type) -> Result<(), SubsetError>) -> Result<(), SubsetError>
fn add_upper_bound(&self, v: Var, bound: Type, is_subset: &mut dyn FnMut(&Type, &Type) -> Result<(), SubsetError>) -> Result<(), SubsetError>
fn check_protocol_cache(&self, got: &Type, want: &Type) -> Option<Result<(), SubsetError>>
fn check_typed_dict_cache(&self, got: &TypedDict, want: &TypedDict) -> Option<Result<(), SubsetError>>
fn error_builder<'a>(&self, got: &Type, want: &Type, errors: &'a ErrorCollector, loc: TextRange, tcc: &dyn Fn() -> TypeCheckContext, subset_error: SubsetError) -> ErrorBuilder<'a>
fn expand(&self, t: Type) -> Type
fn expand_mut(&self, t: &mut Type)
fn expand_unwrap(&self, v: Var) -> Type
fn expand_with_bounds(&self, dim_ty: &mut Type)
fn finish_all_quantified<Ans: LookupAnswer>(&self, ty: &Type, type_order: TypeOrder<'_, Ans>) -> Result<(), Vec1<TypeVarSpecializationError>>
fn finish_class_targs(&self, targs: &mut TArgs, uniques: &UniqueFactory)
fn finish_quantified<Ans: LookupAnswer>(&self, vs: QuantifiedHandle, infer_with_first_use: bool, type_order: TypeOrder<'_, Ans>) -> Result<(), Vec1<TypeVarSpecializationError>>
fn for_display(&self, t: Type) -> Type
fn for_return_boundary(&self, t: Type) -> Type
fn for_return_boundary_with_type_level_dsl_errors(&self, t: Type) -> (Type, Vec<ShapeError>)
fn force(&self, t: Type) -> Type
fn force_mut(&self, t: &mut Type)
fn force_var(&self, v: Var) -> Type
fn fresh_partial_contained(&self, uniques: &UniqueFactory, range: TextRange) -> Var
fn fresh_quantified(&self, params: &TParams, t: Type, uniques: &UniqueFactory) -> (QuantifiedHandle, Type)
fn fresh_recursive(&self, uniques: &UniqueFactory) -> Var
fn fresh_unwrap(&self, uniques: &UniqueFactory) -> Var
fn freshen_class_targs(&self, targs: &mut TArgs, uniques: &UniqueFactory) -> QuantifiedHandle
fn generalize_class_targs(&self, targs: &mut TArgs, vars_with_residual_captures: &SmallSet<Var>)
fn has_instantiation_errors(&self, vs: &QuantifiedHandle) -> bool
fn has_new_instantiation_errors(&self, snapshot: &VarSnapshot) -> bool
fn instantiate_callable_self(&self, tparams: &TParams, self_obj: &Type, self_param: &Type, callable: Callable, uniques: &UniqueFactory, is_subset: &mut dyn FnMut(&Type, &Type) -> bool) -> Callable
fn is_consistent<Ans: LookupAnswer>(&self, got: &Type, want: &Type, type_order: TypeOrder<'_, Ans>) -> Result<(), SubsetError>
fn is_equivalent<Ans: LookupAnswer>(&self, got: &Type, want: &Type, type_order: TypeOrder<'_, Ans>) -> Result<(), SubsetError>
fn is_partial(&self, ty: &Type) -> bool
fn is_subset_eq<'subset, Ans: LookupAnswer>(&self, got: &Type, want: &Type, type_order: TypeOrder<'_, Ans>, call_context: Option<&CallContext<'subset>>) -> Result<(), SubsetError>
fn new(infer_with_first_use: bool, tensor_shapes: bool, strict_callable_subtyping: bool, strict_partial_subtyping: bool, spec_compliant_overloads: bool, legacy_overload_expansion: bool) -> Self
fn partial_sort_by_vars<'a>(&self, ts: &'a [Type]) -> impl Iterator<Item = (&'a Type, Vec<Var>)>
fn pin_placeholder_type(&self, var: Var, pin_partial_types: bool) -> Option<PinError>
fn record_recursive(&self, var: Var, ty: Type) -> Type
fn recurse<'a>(&self, var: Var, recurser: &'a VarRecurser) -> Option<Guard<'a, Var>>
fn restore_vars(&self, snapshot: VarSnapshot)
fn sanitize_type_vars(&self, ty: &Type, pin_partial_types: bool) -> Vec<PinError>
fn sanitize_vars(&self, pending: Vec<Var>, pin_partial_types: bool) -> Vec<PinError>
fn snapshot_vars(&self, vars: &[Var]) -> VarSnapshot
fn store_protocol_cache<Ans: LookupAnswer>(&self, got: &Type, want: &Type, result: &Result<(), SubsetError>, type_order: TypeOrder<'_, Ans>)
fn store_typed_dict_cache<Ans: LookupAnswer>(&self, got: &TypedDict, want: &TypedDict, result: &Result<(), SubsetError>, type_order: TypeOrder<'_, Ans>)
fn unions<Ans: LookupAnswer>(&self, branches: Vec<Type>, type_order: TypeOrder<'_, Ans>) -> Type
fn var_is_partial(&self, var: Var) -> bool
fn with_snapshot(&self, vars: &[Var], f: impl FnOnce() -> Result<(), SubsetError>) -> SubsetWithSnapshotResult
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## VarRecurser

`struct` · `pyrefly::solver::solver::VarRecurser`

```rust
struct VarRecurser
```

**Methods** (1)

```rust
fn new() -> Self
```

A recurser for Vars which is aware of unification.
Prefer this over Recurser<Var> and use Solver::recurse.

---

## VarSnapshot

`struct` · `pyrefly::solver::solver::VarSnapshot`

```rust
struct VarSnapshot
```

Snapshot of solver variable state.
IMPORTANT: this struct is deliberately opaque.
Var state should not be exposed outside this file.

---
