# Typing health

From the checker's own coverage report, over public symbols only -- names without a
leading underscore, plus anything an `__all__` exports, followed through re-export
chains. That last part matters here: this library re-exports heavily, and a report over
raw modules would describe a different surface from the one a caller sees.

**92.3% covered**, 89.7% strictly, over 6083 typable positions in 308 modules.

5457 typed · 155 `Any` · 471 untyped · 233 suppressions.

`Any` and untyped are different failures. An `Any` is a type the checker resolved to
the top type -- it will not catch a mistake there. An untyped position has no
annotation at all, and `index/inferred.tsv` may still carry an inferred answer for it.

## Where the types are weakest

| Symbol | Kind | Any | Untyped |
|---|---|---:|---:|
| `fastmcp.server.http.RequestContextMiddleware.__call__` | function | 0 | 4 |
| `fastmcp.prompts.function_prompt.DecoratedPrompt.__call__` | function | 3 | 0 |
| `fastmcp.resources.function_resource.DecoratedResource.__call__` | function | 3 | 0 |
| `fastmcp.server.server.FastMCP.from_fastapi` | function | 3 | 0 |
| `fastmcp.tools.function_tool.DecoratedTool.__call__` | function | 3 | 0 |
| `fastmcp.utilities.json_schema.replace_refs` | function | 3 | 0 |
| `fastmcp.client._sdk_context_shim.RequestContext.__class_getitem__` | function | 2 | 0 |
| `fastmcp.client.auth.bearer.BearerAuth.auth_flow` | function | 0 | 2 |
| `fastmcp.client.client.Client.__init__` | function | 2 | 0 |
| `fastmcp.server.auth.auth.TokenHandler.handle` | function | 1 | 1 |
| `fastmcp.server.low_level.LowLevelServer.__init__` | function | 2 | 0 |
| `fastmcp.server.providers.fastmcp_provider.FastMCPProviderPrompt.__init__` | function | 2 | 0 |
| `fastmcp.server.providers.fastmcp_provider.FastMCPProviderResource.__init__` | function | 2 | 0 |
| `fastmcp.server.providers.fastmcp_provider.FastMCPProviderResourceTemplate.__init__` | function | 2 | 0 |
| `fastmcp.server.providers.fastmcp_provider.FastMCPProviderTool.__init__` | function | 2 | 0 |
| `fastmcp.server.providers.prefab_payload.rewrite_payload_tool_names` | function | 2 | 0 |
| `fastmcp.server.providers.proxy.StatefulProxyClient.__init__` | function | 2 | 0 |
| `fastmcp.server.server.FastMCP.from_openapi` | function | 2 | 0 |
| `fastmcp.settings.Settings.normalize_log_level` | function | 0 | 2 |
| `fastmcp.utilities.async_utils.call_sync_fn_in_threadpool` | function | 2 | 0 |

## Suppressions

233 in the indexed distributions. Each one is a place upstream decided
the checker was wrong or the cost was too high, which makes them the best available map
of where this library's typing is genuinely hard.

```bash
cut -f4 content/index/suppressions.tsv | tr ',' '\n' | sort | uniq -c | sort -rn
```
