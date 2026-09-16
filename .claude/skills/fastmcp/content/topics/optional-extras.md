# What the optional installs unlock

Twelve modules bind names under `try`/`except ImportError`, so the library describes itself differently depending on what is installed. This index was built with every extra present, which is what makes it describe the library as it runs rather than as it degrades.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.settings.Settings` | class | `fastmcp.Settings` | [prose](../api/fastmcp.settings.md) | [records](../model/fastmcp.settings.json) |

## Upstream guides

- [`corpus/docs/getting-started/installation.mdx`](../corpus/docs/getting-started/installation.mdx)

## Decision rules

- A vendor-named provider generally needs that vendor's extra.
- `index/extras.tsv` maps each extra to the modules it unlocks.

## Anti-patterns

- Assuming a symbol in this index is importable in a slim install. Check `extras.tsv` and `conditional.tsv`.

## Agent checklist

- `index/conditional.tsv` records which branch of each guard was live at build time.
