# `fastmcp.server.providers.prefab_synthesis`

Distribution: `fastmcp`

## PREFAB_PLACEHOLDER_URI

`fastmcp.server.providers.prefab_synthesis.PREFAB_PLACEHOLDER_URI`

```python
PREFAB_PLACEHOLDER_URI = 'ui://prefab/renderer.html'
```

**Inferred type** (`ty`, not declared in the source): `Literal["ui://prefab/renderer.html"]`

## _build_resource_for_tool

`fastmcp.server.providers.prefab_synthesis._build_resource_for_tool`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_resource_for_tool(tool: Tool) -> Resource | None
```

Synthesize a TextResource for a prefab tool. Returns None if prefab_ui isn't installed.


## _get_tool_hash

`fastmcp.server.providers.prefab_synthesis._get_tool_hash`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_tool_hash(tool: Tool) -> str | None
```

Read the stored hash from tool meta, or compute from app name + tool name.


## _is_prefab_tool

`fastmcp.server.providers.prefab_synthesis._is_prefab_tool`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_prefab_tool(tool: Tool) -> bool
```

True if *tool* was marked as needing a Prefab renderer at registration.


## _merge_domain_lists

`fastmcp.server.providers.prefab_synthesis._merge_domain_lists`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _merge_domain_lists(base: list[str] | None, extra: list[str] | None) -> list[str] | None
```

## _walk_prefab_tools

`fastmcp.server.providers.prefab_synthesis._walk_prefab_tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _walk_prefab_tools(server: FastMCP) -> list[Tool]
```

Enumerate all prefab tools across the server's providers (sync walk of _components).


## rewrite_tool_meta_for_wire

`fastmcp.server.providers.prefab_synthesis.rewrite_tool_meta_for_wire`

```python
def rewrite_tool_meta_for_wire(tool: Tool) -> Tool
```

Return a model_copy with the per-tool URI and CSP stripped.

Reads the hash from the tool's own meta. If no hash is found,
returns the tool unchanged. Produces a fresh copy — the original
Tool object is untouched.


## synthesize_prefab_resource_by_uri

`fastmcp.server.providers.prefab_synthesis.synthesize_prefab_resource_by_uri`

```python
async def synthesize_prefab_resource_by_uri(server: FastMCP, uri: str) -> Resource | None
```

Intercept a Prefab renderer URI and synthesize on demand.


## synthesize_prefab_resources

`fastmcp.server.providers.prefab_synthesis.synthesize_prefab_resources`

```python
async def synthesize_prefab_resources(server: FastMCP) -> list[Resource]
```

Return fresh synthetic Prefab resources for all prefab tools. Pure.


