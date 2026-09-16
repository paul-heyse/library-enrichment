# Asking the caller: elicitation and sampling

Both invert the usual direction. Elicitation asks the user for structured input; sampling asks the client's model for a completion. Both require the client to have declared the capability, so both can fail for reasons your server cannot fix.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.server.context.Context` | class | `fastmcp.Context` | [prose](../api/fastmcp.server.context.md) | [records](../model/fastmcp.server.context.json) |
| `fastmcp.client.sampling.SamplingHandler` | attribute | `fastmcp.client.client.SamplingHandler` | [prose](../api/fastmcp.client.sampling.md) | [records](../model/fastmcp.client.sampling.json) |

## Upstream guides

- [`corpus/docs/servers/elicitation.mdx`](../corpus/docs/servers/elicitation.mdx)
- [`corpus/docs/servers/sampling.mdx`](../corpus/docs/servers/sampling.mdx)
- [`corpus/docs/clients/elicitation.mdx`](../corpus/docs/clients/elicitation.mdx)

## Decision rules

- Structured user input? Elicitation. Model output? Sampling.
- A client that did not advertise the capability will refuse; handle that path.

## Anti-patterns

- Treating a declined elicitation as an error rather than an answer.

## Agent checklist

- 23 upstream elicitation sites in the corpus; read one before designing a schema.
