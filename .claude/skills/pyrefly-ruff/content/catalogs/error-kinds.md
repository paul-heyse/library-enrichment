# Error kinds

The checker classifies every diagnostic into one of 144 kinds, enumerated by
`pyrefly_config::error_kind::ErrorKind`. The same vocabulary is the filter: `--only`, `--error`, `--warn` and
`--ignore` all take a kind, and a configuration file can set a severity per kind.

The CLI spells them in kebab-case while the enum spells them in PascalCase, so
`content/index/error-kinds.tsv` carries both.

```bash
rg -i 'annotation|return' content/index/error-kinds.tsv | cut -f2,3
```

Prose for each kind, with examples, is in
`content/corpus/pyrefly/error-kinds.mdx`; suppression syntax is in
`content/corpus/pyrefly/error-suppressions.mdx`.

ruff's equivalent vocabulary is not an enum of kinds but the 970 rule codes in
`rules.md`. The two do not correspond: a ruff rule is a check, a checker error kind is a
category of type error.

## The kinds

- `abstract-method-call`
- `assert-type`
- `bad-argument-count`
- `bad-argument-type`
- `bad-assignment`
- `bad-class-definition`
- `bad-context-manager`
- `bad-dataclass-descriptor`
- `bad-dunder-all`
- `bad-function-definition`
- `bad-index`
- `bad-instantiation`
- `bad-keyword-argument`
- `bad-match`
- `bad-override`
- `bad-override-mutable-attribute`
- `bad-override-param-name`
- `bad-param-name-override`
- `bad-raise`
- `bad-return`
- `bad-singledispatch-register`
- `bad-specialization`
- `bad-typed-dict`
- `bad-typed-dict-key`
- `bad-unpacking`
- `column-schema-mismatch`
- `column-type-mismatch`
- `coverage-missing`
- `coverage-partial`
- `deprecated`
- `direct-abstract-base-instantiation`
- `division-by-zero`
- `duplicate-column`
- `empty-body`
- `explicit-any`
- `implicit-abstract-class`
- `implicit-any`
- `implicit-any-attribute`
- `implicit-any-empty-container`
- `implicit-any-lambda`
- `implicit-any-parameter`
- `implicit-any-type-argument`
- `implicit-bool`
- `implicit-import`
- `implicit-reexport`
- `implicitly-defined-attribute`
- `incompatible-comparison`
- `incompatible-overload-residual`
- `inconsistent-inheritance`
- `inconsistent-overload`
- `inconsistent-overload-default`
- `internal-error`
- `invalid-abstract-method`
- `invalid-annotation`
- `invalid-argument`
- `invalid-cast`
- `invalid-decorator`
- `invalid-inheritance`
- `invalid-literal`
- `invalid-overload`
- `invalid-param-spec`
- `invalid-pattern`
- `invalid-self-type`
- `invalid-sentinel`
- `invalid-super-call`
- `invalid-syntax`
- `invalid-type-alias`
- `invalid-type-checking-constant`
- `invalid-type-var`
- `invalid-type-var-tuple`
- `invalid-variance`
- `invalid-yield`
- `misplaced-ignore`
- `missing-argument`
- `missing-attribute`
- `missing-attribute-patch-target`
- `missing-import`
- `missing-module-attribute`
- `missing-override-decorator`
- `missing-source`
- `missing-source-for-stubs`
- `missing-super-call`
- `name-mismatch`
- `no-access`
- `no-any-return`
- `no-any-return-explicit`
- `no-any-return-implicit`
- `no-matching-overload`
- `non-convergent-recursion`
- `non-exhaustive-match`
- `non-exhaustive-match-open-type`
- `not-a-type`
- `not-async`
- `not-callable`
- `not-iterable`
- `not-required-key-access`
- `open-unpacking`
- `parse-error`
- `potential-bad-keyword-argument`
- `protocol-implicitly-defined-attribute`
- `pytorch-efficiency-lint-cuda-call`
- `pytorch-efficiency-lint-item-call`
- `pytorch-efficiency-lint-print-tensor`
- `pytorch-efficiency-lint-redundant-to-call`
- `pytorch-efficiency-lints`
- `read-only`
- `redefinition`
- `redundant-cast`
- `redundant-condition`
- `regex`
- `reveal-type`
- `string-as-iterable`
- `unannotated-attribute`
- `unannotated-parameter`
- `unannotated-protocol-member`
- `unannotated-return`
- `unbound-name`
- `unexpected-keyword`
- `unexpected-positional-argument`
- `unimported-directive`
- `unknown-argument-type`
- `unknown-attribute-type`
- `unknown-column`
- `unknown-name`
- `unknown-variable-type`
- `unnecessary-comparison`
- `unnecessary-type-conversion`
- `unreachable`
- `unreachable-match-case`
- `unresolvable-dunder-all`
- `unsafe-overlap`
- `unsupported`
- `unsupported-delete`
- `unsupported-dynamic-base`
- `unsupported-operation`
- `untyped-class-decorator`
- `untyped-function-decorator`
- `untyped-import`
- `unused-call-result`
- `unused-coroutine`
- `unused-ignore`
- `unused-type-ignore`
- `useless-overload-body`
- `variance-mismatch`
