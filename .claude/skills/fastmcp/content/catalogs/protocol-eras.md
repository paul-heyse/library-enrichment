# Protocol eras

`mcp_types` carries more than one dated copy of the wire model, because a FastMCP
connection negotiates a protocol era and a proxy can mirror the era of whatever is
in front of it. Same class names, different modules, different shapes.

| Era | Types |
|---|---:|
| `2025-11-25` | 175 |
| `2026-07-28` | 178 |
| `alidate-rendered-properties` | 1 |
| `alue-at-path` | 1 |
| `ersion-from-ctx` | 1 |
| `ersion-gated` | 1 |
| `ersion-request-meta` | 1 |

Never match a wire type by leaf name alone: `CallToolResult` exists in every era and
in the era-neutral module. `content/index/symbols.tsv` carries the era in its own
column; `-` means era-neutral.
