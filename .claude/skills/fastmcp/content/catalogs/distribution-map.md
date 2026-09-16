# Distribution map

| Distribution | Items | Classes | Nameable | Role |
|---|---:|---:|---:|---|
| `authlib` | 1 | 0 | 1 |  |
| `fastmcp` | 1597 | 424 | 1049 |  |
| `mcp` | 837 | 233 | 491 | Indexed so the three-way `FastMCP` / `MCPServer` / tombstone distinction can be  |
| `mcp-types` | 616 | 462 | 246 | 462 classes in seven modules -- more than FastMCP itself. Defined in the private |
| `uncalled_for` | 5 | 0 | 5 |  |

`fastmcp` on PyPI is a metapackage that ships no code; every module lives in
`fastmcp-slim`, which is what its extras resolve through. Ask metadata questions of
`fastmcp-slim`.
