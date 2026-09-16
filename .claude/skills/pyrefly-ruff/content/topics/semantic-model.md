# The semantic model: bindings and scopes

`ruff_python_semantic` is the layer that makes a lint rule more than a pattern: it resolves names to bindings, tracks scopes, and knows whether an identifier refers to a builtin, an import or a local. It is not a type checker -- it has no types. That boundary is exactly why the two projects compose: ruff resolves names, the checker resolves types.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_python_semantic::model::SemanticModel` | struct | 124 | [prose](../api/ruff_python_semantic.model.md#semanticmodel) | [records](../model/ruff_python_semantic.model.json) |
| `ruff_python_semantic::binding::BindingKind` | enum | 59 | [prose](../api/ruff_python_semantic.binding.md#bindingkind) | [records](../model/ruff_python_semantic.binding.json) |
| `ruff_python_semantic::scope::Scope` | struct | 14 | [prose](../api/ruff_python_semantic.scope.md#scope) | [records](../model/ruff_python_semantic.scope.json) |

## Decision rules

- Need to know what a name refers to? `SemanticModel`.
- Need to know its type? A different toolchain -- see the type-inference topic.

## Anti-patterns

- Expecting type information from the semantic model.
- Resolving imports by string matching when `BindingKind` already says.

## Agent checklist

- Scope is a tree; a name can bind differently in a comprehension than in its parent.
