# Registering tools, resources and prompts

`@mcp.tool` returns the decorated function unchanged, despite an implementation annotation that says `FunctionTool`. The overloads and the runtime agree with each other and disagree with the annotation, which is why `index/overloads.tsv` exists and why a type checker will mislead you here.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.tools.base.Tool` | class | `fastmcp.tools.Tool` | [prose](../api/fastmcp.tools.base.md) | [records](../model/fastmcp.tools.base.json) |
| `fastmcp.resources.base.Resource` | class | `fastmcp.resources.Resource` | [prose](../api/fastmcp.resources.base.md) | [records](../model/fastmcp.resources.base.json) |
| `fastmcp.prompts.base.Prompt` | class | `fastmcp.prompts.Prompt` | [prose](../api/fastmcp.prompts.base.md) | [records](../model/fastmcp.prompts.base.json) |
| `fastmcp.utilities.components.FastMCPComponent` | class | `fastmcp.tools.base.FastMCPComponent` | [prose](../api/fastmcp.utilities.components.md) | [records](../model/fastmcp.utilities.components.json) |

## Upstream guides

- [`corpus/docs/servers/tools.mdx`](../corpus/docs/servers/tools.mdx)
- [`corpus/docs/servers/resources.mdx`](../corpus/docs/servers/resources.mdx)
- [`corpus/docs/servers/prompts.mdx`](../corpus/docs/servers/prompts.mdx)

## Decision rules

- Parameter annotations drive the generated JSON schema. Unannotated parameters are tolerated, but the schema degrades.
- Returning a non-serialisable object is a runtime failure, not a registration failure.

## Anti-patterns

- Treating the return of `@mcp.tool` as a `FunctionTool`. It is your function.
- Assuming a resource URI template binds like a path. Read the template rules.

## Agent checklist

- `catalogs/registration.md` states what each decorator returns at runtime.
- 2,028 upstream registration sites: `ast-grep scan -c queries/sgconfig.yml --filter '^corpus-tool-registration$' content/corpus`
