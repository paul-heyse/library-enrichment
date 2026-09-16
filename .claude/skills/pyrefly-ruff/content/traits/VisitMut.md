# VisitMut

`pyrefly_util::visit::VisitMut`

```rust
trait VisitMut<To = Self>: Sized
```

Prose: [`api/pyrefly_util.visit.md`](../api/pyrefly_util.visit.md#visitmut) · records: [`model/pyrefly_util.visit.json`](../model/pyrefly_util.visit.json)

## Required

Every implementation must supply these.

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn type_eq() -> bool
fn visit_contains() -> bool
fn visit_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

## Implementors (163)

Read one before writing your own.

- `alloc::boxed::Box`
- `alloc::string::String`
- `alloc::sync::Arc`
- `alloc::vec::Vec`
- `compact_str::CompactString`
- `core::marker::PhantomData`
- `core::option::Option`
- `pyrefly::alt::answers::Traces`
- `pyrefly::alt::class::class_field::ClassField`
- `pyrefly::alt::class::class_field::ClassFieldInitialization`
- `pyrefly::alt::class::class_field::ClassFieldInner`
- `pyrefly::alt::class::class_field::Descriptor`
- `pyrefly::alt::class::class_field::IsInherited`
- `pyrefly::alt::class::variance_inference::VarianceMap`
- `pyrefly::alt::types::abstract_class::AbstractClassMembers`
- `pyrefly::alt::types::class_bases::ClassBases`
- `pyrefly::alt::types::class_metadata::ClassDisjointBase`
- `pyrefly::alt::types::class_metadata::ClassMetadata`
- `pyrefly::alt::types::class_metadata::ClassMro`
- `pyrefly::alt::types::class_metadata::ClassSynthesizedField`
- `pyrefly::alt::types::class_metadata::ClassSynthesizedFields`
- `pyrefly::alt::types::class_metadata::DjangoReverseRelationIndex`
- `pyrefly::alt::types::decorated_function::Decorator`
- `pyrefly::alt::types::decorated_function::UndecoratedFunction`
- `pyrefly::alt::types::legacy_lookup::LegacyTypeParameterLookup`
- `pyrefly::alt::types::pydantic::PydanticConfig`
- `pyrefly::alt::types::pydantic::PydanticModelKind`
- `pyrefly::alt::types::yields::YieldFromResult`
- `pyrefly::alt::types::yields::YieldResult`
- `pyrefly::binding::binding::AnnAssignHasValue`
- `pyrefly::binding::binding::AnnotationTarget`
- `pyrefly::binding::binding::AnnotationWithTarget`
- `pyrefly::binding::binding::EmptyAnswer`
- `pyrefly::binding::binding::NoneIfRecursive`
- `pyrefly::binding::binding::UndecoratedFunctionRangeAnswer`
- `pyrefly::binding::pydantic::PydanticAliasGenerator`
- `pyrefly_python::module_name::ModuleName`
- `pyrefly_python::module_path::ModulePath`
- `pyrefly_python::module_path::ModuleStyle`
- `pyrefly_python::short_identifier::ShortIdentifier`
- `pyrefly_types::annotation::Annotation`
- `pyrefly_types::annotation::Qualifier`
- `pyrefly_types::callable::Callable`
- `pyrefly_types::callable::DefaultValue`
- `pyrefly_types::callable::Param`
- `pyrefly_types::callable::ParamList`
- `pyrefly_types::callable::Params`
- `pyrefly_types::callable::PrefixParam`
- `pyrefly_types::callable::Required`
- `pyrefly_types::callable_residual::CallableResidual`
- `pyrefly_types::callable_residual::CallableResidualKind`
- `pyrefly_types::callable_residual::OverloadBranchProjection`
- `pyrefly_types::callable_residual::OverloadResidualIdentity`
- `pyrefly_types::class::Class`
- `pyrefly_types::class::ClassKind`
- `pyrefly_types::class::ClassType`
- `pyrefly_types::data_frame::DataFrameKind`
- `pyrefly_types::data_frame::DataFrameSchema`
- `pyrefly_types::data_frame::SchemaCompleteness`
- `pyrefly_types::data_frame::SchemaRole`
- `pyrefly_types::dimension::Int`
- `pyrefly_types::function::BodyKind`
- `pyrefly_types::function::Deprecation`
- `pyrefly_types::function::FuncDefId`
- `pyrefly_types::function::FuncDefIndex`
- `pyrefly_types::function::FuncFlags`
- `pyrefly_types::function::FuncMetadata`
- `pyrefly_types::function::FuncSymbol`
- `pyrefly_types::function::Function`
- `pyrefly_types::function::FunctionKind`
- `pyrefly_types::function::PropertyMetadata`
- `pyrefly_types::function::PropertyRole`
- `pyrefly_types::identity::IdentityIgnored`
- `pyrefly_types::keywords::ConverterMap`
- `pyrefly_types::keywords::DataclassFieldKeywords`
- `pyrefly_types::keywords::DataclassKeywords`
- `pyrefly_types::keywords::DataclassTransformMetadata`
- `pyrefly_types::keywords::KwCall`
- `pyrefly_types::keywords::TypeMap`
- `pyrefly_types::lit_int::LitInt`
- `pyrefly_types::lit_int::LitIntInner`
- `pyrefly_types::literal::Lit`
- `pyrefly_types::literal::LitEnum`
- `pyrefly_types::literal::LitStyle`
- `pyrefly_types::literal::Literal`
- `pyrefly_types::map_int_tuples::MapIntTuples`
- `pyrefly_types::map_int_tuples::MapIntTuplesInterpretation`
- `pyrefly_types::map_int_tuples::TypeLambda`
- `pyrefly_types::meta_shape_dsl::ShapeDslFunction`
- `pyrefly_types::meta_shape_dsl::ShapeTransform`
- `pyrefly_types::module::ModuleType`
- `pyrefly_types::param_spec::ParamSpec`
- `pyrefly_types::polars_dtype::PolarsArrayShape`
- `pyrefly_types::polars_dtype::PolarsDType`
- `pyrefly_types::polars_dtype::PolarsScalarDType`
- `pyrefly_types::quantified::AnchorIndex`
- `pyrefly_types::quantified::Quantified`
- `pyrefly_types::quantified::QuantifiedIdentity`
- `pyrefly_types::quantified::QuantifiedKind`
- `pyrefly_types::quantified::QuantifiedOrigin`
- `pyrefly_types::read_only::IsFinalVariableInitialized`
- `pyrefly_types::read_only::ReadOnlyReason`
- `pyrefly_types::sentinel::Sentinel`
- `pyrefly_types::series::SeriesSchema`
- `pyrefly_types::shaped_array::IntTuple`
- `pyrefly_types::shaped_array::IntTupleRepr`
- `pyrefly_types::shaped_array::ShapedArrayShapeStorage`
- `pyrefly_types::shaped_array::ShapedArraySyntax`
- `pyrefly_types::shaped_array::ShapedArrayType`
- `pyrefly_types::special_form::SpecialForm`
- `pyrefly_types::tuple::Tuple`
- `pyrefly_types::tuple::UnpackedTupleParts`
- `pyrefly_types::type_alias::TypeAlias`
- `pyrefly_types::type_alias::TypeAliasData`
- `pyrefly_types::type_alias::TypeAliasIndex`
- `pyrefly_types::type_alias::TypeAliasRef`
- `pyrefly_types::type_alias::TypeAliasStyle`
- `pyrefly_types::type_info::NarrowedFacet`
- `pyrefly_types::type_info::NarrowedFacets`
- `pyrefly_types::type_info::TypeInfo`
- `pyrefly_types::type_level_dsl::ParsedTypeShapeDslFunction`
- `pyrefly_types::type_level_dsl::ResolvedTypeShapeDslFunction`
- `pyrefly_types::type_level_dsl::StructurallyValidatedTypeShapeDslFunction`
- `pyrefly_types::type_level_dsl::TypeLevelDslCall`
- `pyrefly_types::type_level_dsl::TypeLevelDslFunction`
- `pyrefly_types::type_level_dsl::TypeShapeDslDomain`
- `pyrefly_types::type_level_dsl::TypeShapeDslInputDomain`
- `pyrefly_types::type_var::PreInferenceVariance`
- `pyrefly_types::type_var::Restriction`
- `pyrefly_types::type_var::TypeVar`
- `pyrefly_types::type_var::Variance`
- `pyrefly_types::type_var::flag::FlagDomain`
- `pyrefly_types::type_var::shape_extension::ShapeExtensionRestriction`
- `pyrefly_types::type_var::shape_extension::ShapeExtensionRestrictionKind`
- `pyrefly_types::type_var_tuple::TypeVarTuple`
- `pyrefly_types::typed_dict::AnonymousTypedDictInner`
- `pyrefly_types::typed_dict::TypedDict`
- `pyrefly_types::typed_dict::TypedDictField`
- `pyrefly_types::typed_dict::TypedDictInner`
- `pyrefly_types::types::AnyStyle`
- `pyrefly_types::types::BoundMethod`
- `pyrefly_types::types::BoundMethodType`
- `pyrefly_types::types::CalleeKind`
- `pyrefly_types::types::Forall`
- `pyrefly_types::types::Forallable`
- `pyrefly_types::types::NNModuleType`
- `pyrefly_types::types::NeverStyle`
- `pyrefly_types::types::Overload`
- `pyrefly_types::types::OverloadType`
- `pyrefly_types::types::SuperObj`
- `pyrefly_types::types::TArgs`
- `pyrefly_types::types::TParams`
- `pyrefly_types::types::Type`
- `pyrefly_types::types::Union`
- `pyrefly_types::types::Var`
- `pyrefly_util::uniques::Unique`
- `ruff_python_ast::generated::Expr`
- `ruff_python_ast::name::Name`
- `ruff_text_size::range::TextRange`
- `starlark_map::ordered_map::OrderedMap`
- `starlark_map::small_map::SmallMap`
- `thin_vec::ThinVec`
- `vec1::Vec1`

## Documentation

Like `Visit`, but mutably.
