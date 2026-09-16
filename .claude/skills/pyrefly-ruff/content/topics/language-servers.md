# Language servers: LSP and TSP

Both `pyrefly lsp` and `pyrefly tsp` speak framed JSON-RPC on stdio, and both advertise `referencesProvider`, `callHierarchyProvider`, `typeHierarchyProvider` and `inlayHintProvider` -- capabilities a weaker server either lacks or answers unreliably. TSP adds `typeServerMultiConnection` over IPC. The captured handshakes are in `../catalogs/protocols.md`, and `tsp_types` is indexed here so the protocol is readable as declarations rather than inferred from traffic.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_server::server::Server` | struct | 2 | [prose](../api/ruff_server.server.md#server) | [records](../model/ruff_server.server.json) |
| `pyrefly_config::config::ConfigFile` | struct | 50 | [prose](../api/pyrefly_config.config.md#configfile) | [records](../model/pyrefly_config.config.json) |

## Upstream guides

- [`corpus/pyrefly/IDE.mdx`](../corpus/pyrefly/IDE.mdx)
- [`corpus/pyrefly/IDE-features.mdx`](../corpus/pyrefly/IDE-features.mdx)

## Decision rules

- Interactive, one position at a time? A server.
- Batch, whole project? A report -- see the cross-references topic.

## Anti-patterns

- Driving a language server in a loop to build a project-wide index.

## Agent checklist

- Send `initialize`, then `initialized`, before any request; capabilities arrive in the reply.
