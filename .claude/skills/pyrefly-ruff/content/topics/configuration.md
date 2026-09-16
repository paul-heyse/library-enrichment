# Configuration surfaces

ruff's configuration is one flat namespace of dotted keys discoverable with `ruff config`; the checker's is a TOML file with a published JSON Schema, and every key also has a CLI override under `Config Overrides`. Its `ErrorKind` enum is the taxonomy you filter on with `--only`, `--error`, `--warn` and `--ignore`.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `pyrefly_config::config::ConfigFile` | struct | 50 | [prose](../api/pyrefly_config.config.md#configfile) | [records](../model/pyrefly_config.config.json) |
| `pyrefly_config::error_kind::ErrorKind` | enum | 27 | [prose](../api/pyrefly_config.error_kind.md#errorkind) | [records](../model/pyrefly_config.error_kind.json) |
| `pyrefly_config::args::ConfigOverrideArgs` | struct | 22 | [prose](../api/pyrefly_config.args.md#configoverrideargs) | [records](../model/pyrefly_config.args.json) |
| `ruff_options_metadata::OptionSet` | struct | 9 | [prose](../api/ruff_options_metadata.md#optionset) | [records](../model/ruff_options_metadata.json) |

## Upstream guides

- [`corpus/ruff/configuration.md`](../corpus/ruff/configuration.md)
- [`corpus/pyrefly/configuration.mdx`](../corpus/pyrefly/configuration.mdx)
- [`corpus/pyrefly/error-kinds.mdx`](../corpus/pyrefly/error-kinds.mdx)
- [`corpus/pyrefly/error-suppressions.mdx`](../corpus/pyrefly/error-suppressions.mdx)

## Decision rules

- Need one key's default? `ruff config <key>`, or `content/index/config.tsv`.
- Migrating from another checker? `pyrefly init` reads mypy and pyright configs.

## Anti-patterns

- Hand-writing a config from memory when both tools can print the schema.

## Agent checklist

- `pyrefly dump-config` shows which config actually applied and which files it covers.
