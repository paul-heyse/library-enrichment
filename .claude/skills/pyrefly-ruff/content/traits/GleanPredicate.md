# GleanPredicate

`pyrefly_glean_schema::report::glean::facts::GleanPredicate`

```rust
trait GleanPredicate
```

Prose: [`api/pyrefly_glean_schema.report.glean.facts.md`](../api/pyrefly_glean_schema.report.glean.facts.md#gleanpredicate) · records: [`model/pyrefly_glean_schema.report.glean.facts.json`](../model/pyrefly_glean_schema.report.glean.facts.json)

## Required

Every implementation must supply these.

```rust
fn GLEAN_name() -> String
```

## Implementors (85)

Read one before writing your own.

- `pyrefly_glean_schema::report::glean::schema::digest::FileDigest`
- `pyrefly_glean_schema::report::glean::schema::gencode::GenCode`
- `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeBySource`
- `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeClass`
- `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeCommand`
- `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeSignature`
- `pyrefly_glean_schema::report::glean::schema::python::BaseClassToDerived`
- `pyrefly_glean_schema::report::glean::schema::python::CalleeToCaller`
- `pyrefly_glean_schema::report::glean::schema::python::ClassBySName`
- `pyrefly_glean_schema::report::glean::schema::python::ClassDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::ClassDefinition`
- `pyrefly_glean_schema::report::glean::schema::python::ContainedBy`
- `pyrefly_glean_schema::report::glean::schema::python::ContainedByTopLevelDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::ContainingTopLevelDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::Contains`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationDefinition`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationDocstring`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationLocation`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationReference`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationToName`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationUses`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithLocalName`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithName`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithSName`
- `pyrefly_glean_schema::report::glean::schema::python::DeclarationsByFile`
- `pyrefly_glean_schema::report::glean::schema::python::DefinitionDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::DefinitionLocation`
- `pyrefly_glean_schema::report::glean::schema::python::DefinitionsByFile`
- `pyrefly_glean_schema::report::glean::schema::python::DerivedClassToBase`
- `pyrefly_glean_schema::report::glean::schema::python::DirectXRefsByFile`
- `pyrefly_glean_schema::report::glean::schema::python::FileCall`
- `pyrefly_glean_schema::report::glean::schema::python::FunctionBySName`
- `pyrefly_glean_schema::report::glean::schema::python::FunctionDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::FunctionDefinition`
- `pyrefly_glean_schema::report::glean::schema::python::ImportStarLocation`
- `pyrefly_glean_schema::report::glean::schema::python::ImportStarStatement`
- `pyrefly_glean_schema::report::glean::schema::python::ImportStarsByFile`
- `pyrefly_glean_schema::report::glean::schema::python::ImportStatement`
- `pyrefly_glean_schema::report::glean::schema::python::ImportStatementByAsName`
- `pyrefly_glean_schema::report::glean::schema::python::ImportStatementByAsSName`
- `pyrefly_glean_schema::report::glean::schema::python::IsAbstract`
- `pyrefly_glean_schema::report::glean::schema::python::IsTopLevelDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::IsTopLevelDefinition`
- `pyrefly_glean_schema::report::glean::schema::python::MethodByLocalNameStr`
- `pyrefly_glean_schema::report::glean::schema::python::MethodOverriden`
- `pyrefly_glean_schema::report::glean::schema::python::MethodOverrides`
- `pyrefly_glean_schema::report::glean::schema::python::Module`
- `pyrefly_glean_schema::report::glean::schema::python::ModuleBySName`
- `pyrefly_glean_schema::report::glean::schema::python::ModuleDefinition`
- `pyrefly_glean_schema::report::glean::schema::python::Name`
- `pyrefly_glean_schema::report::glean::schema::python::NameToSName`
- `pyrefly_glean_schema::report::glean::schema::python::NonImportDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::ResolveOriginalName`
- `pyrefly_glean_schema::report::glean::schema::python::SName`
- `pyrefly_glean_schema::report::glean::schema::python::SNameToName`
- `pyrefly_glean_schema::report::glean::schema::python::SNameWithDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::SearchClassByLowerCaseName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchClassByName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchFieldByLowerCaseName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchFieldByName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchFunctionByLowerCaseName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchFunctionByName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchMethodByLowerCaseName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchMethodByName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchModuleByLowerCaseName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchModuleByName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchVariableByLowerCaseName`
- `pyrefly_glean_schema::report::glean::schema::python::SearchVariableByName`
- `pyrefly_glean_schema::report::glean::schema::python::StringLiteral`
- `pyrefly_glean_schema::report::glean::schema::python::Type`
- `pyrefly_glean_schema::report::glean::schema::python::VariableBySName`
- `pyrefly_glean_schema::report::glean::schema::python::VariableDeclaration`
- `pyrefly_glean_schema::report::glean::schema::python::VariableDefinition`
- `pyrefly_glean_schema::report::glean::schema::python::XRefsViaNameByFile`
- `pyrefly_glean_schema::report::glean::schema::python::XRefsViaNameByTarget`
- `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefDeclarationsByFile`
- `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefsByFile`
- `pyrefly_glean_schema::report::glean::schema::src::ByteSpanContains`
- `pyrefly_glean_schema::report::glean::schema::src::File`
- `pyrefly_glean_schema::report::glean::schema::src::FileContent`
- `pyrefly_glean_schema::report::glean::schema::src::FileDigest`
- `pyrefly_glean_schema::report::glean::schema::src::FileLanguage`
- `pyrefly_glean_schema::report::glean::schema::src::FileLines`
- `pyrefly_glean_schema::report::glean::schema::src::IndexFailure`
- `pyrefly_glean_schema::report::glean::schema::src::RangeContains`
