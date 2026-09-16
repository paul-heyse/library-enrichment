# Cross-references and the call graph

`reference.md` used to say this was unanswerable. It is answerable, from the checker's
batch reports rather than its language server -- one run over 388 files, not a loop over
3,056 symbols.

26,545 call edges · 39,204 reference rows · 2,143 external targets.

## Who calls this?

```bash
rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/callers.tsv | cut -f2,3,4
```

Column 1 is the callee, column 2 the caller, then the file and line. A caller reading
`… (module level)` is import-time code, `… (decorator)` a decorator expression, and
`… (class body)` a class-level statement -- all three are real call sites that no
function name would describe.

A constructor call is recorded against the **class**, not `__init__`: the question is
who builds a `FastMCP`, not who calls `object.__init__`.

## Where is this referenced?

```bash
rg -P '^fastmcp\.server\.context\.Context\t' content/index/references.tsv
```

One row per (target, file) with a count. References into typeshed and third-party code
are separated into `external-refs.tsv` by the target's **defining file**, which the
report carries -- not by guessing from the name.

## The most-called items

| Item | Callers | Referencing files | References |
|---|---:|---:|---:|
| `fastmcp.utilities.logging.get_logger` | 103 | 99 | 203 |
| `fastmcp.server.server.FastMCP` | 89 | 36 | 152 |
| `mcp.client.session.ClientSession` | 71 | 23 | 72 |
| `fastmcp.client.client.Client` | 69 | 15 | 90 |
| `fastmcp.server.middleware.middleware.CallNext` | 44 | 15 | 78 |
| `fastmcp.server.providers.local_provider.local_provider.LocalProvider` | 41 | 10 | 34 |
| `fastmcp.tools.base.Tool` | 40 | 40 | 273 |
| `mcp.shared.exceptions.MCPError` | 39 | 32 | 141 |
| `fastmcp.server.providers.base.Provider` | 37 | 17 | 42 |
| `fastmcp.utilities.versions.VersionSpec` | 35 | 24 | 129 |
| `fastmcp.server.context.Context` | 28 | 17 | 87 |
| `mcp.server.session.ServerSession` | 28 | 10 | 26 |
| `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy` | 27 | 11 | 42 |
| `mcp.server.streamable_http.StreamableHTTPServerTransport` | 27 | 1 | 7 |
| `fastmcp.utilities.auth.parse_scopes` | 23 | 17 | 47 |
| `mcp_types.jsonrpc.ErrorData` | 23 | 18 | 76 |
| `mcp.shared._context_streams.ContextSendStream` | 21 | 5 | 16 |
| `fastmcp.utilities.authorization.AuthContext` | 20 | 4 | 37 |
| `fastmcp.utilities.tasks.TaskConfig` | 20 | 9 | 44 |
| `mcp.shared.jsonrpc_dispatcher.JSONRPCDispatcher` | 19 | 5 | 17 |
| `mcp.server.connection.Connection` | 18 | 7 | 24 |
| `mcp_types._types.TextContent` | 18 | 21 | 67 |
| `fastmcp.utilities.authorization.run_auth_checks` | 17 | 3 | 21 |
| `fastmcp.resources.base.Resource` | 16 | 29 | 134 |
| `fastmcp.resources.template.ResourceTemplate` | 16 | 24 | 110 |

Full ranking in `content/index/usage.tsv`, which carries every item the checker saw
referenced or called. An item absent from it was never referenced *within these three
distributions* -- which is not the same as unused, because callers outside them are
outside this index's scope.
