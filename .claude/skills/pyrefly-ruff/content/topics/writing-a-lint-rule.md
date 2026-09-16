# Writing and selecting lint rules

Every rule is a variant of `ruff_linter::codes::Rule`, and selection goes through `RuleSelector`, which understands prefixes as well as exact codes. The trap is status: 141 of the 970 rules are `Preview` and do nothing unless preview mode is on, and 17 are `Removed`, where selecting them is an error. Neither fact is visible in the code being linted.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_linter::codes::Rule` | enum | 24 | [prose](../api/ruff_linter.codes.md#rule) | [records](../model/ruff_linter.codes.json) |
| `ruff_linter::rule_selector::RuleSelector` | enum | 71 | [prose](../api/ruff_linter.rule_selector.md#ruleselector) | [records](../model/ruff_linter.rule_selector.json) |
| `ruff_linter::codes::RuleCodePrefix` | enum | 65 | [prose](../api/ruff_linter.codes.md#rulecodeprefix) | [records](../model/ruff_linter.codes.json) |
| `ruff_linter::locator::Locator` | struct | 15 | [prose](../api/ruff_linter.locator.md#locator) | [records](../model/ruff_linter.locator.json) |

## Upstream guides

- [`corpus/ruff/linter.md`](../corpus/ruff/linter.md)
- [`corpus/ruff/rule-proposals.md`](../corpus/ruff/rule-proposals.md)

## Decision rules

- Selecting by prefix pulls in future rules; selecting by code does not.
- A rule that seems inert is usually `Preview`. Check `../catalogs/rules.md`.

## Anti-patterns

- Putting a `Removed` code in a config file -- it fails rather than being ignored.
- Assuming `select = ["ALL"]` is stable across releases.

## Agent checklist

- `rg -P '\tRemoved\t' content/index/rules.tsv` before trusting an inherited config.
