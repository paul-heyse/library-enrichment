# `fastmcp.server.providers.wrapped_provider`

Distribution: `fastmcp`

## _WrappedProvider

`fastmcp.server.providers.wrapped_provider._WrappedProvider`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _WrappedProvider(Provider)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Provider`

**Declared members (4)**

- `async def get_app_tool(self, app_name: str, tool_name: str) -> Tool | None`  _async_
  Delegate to inner, bypassing this wrapper's transforms.
- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Delegate to inner's get_tasks and apply wrapper's transforms.
- `async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None`  _async_
  Delegate to inner, bypassing this wrapper's transforms.
- `async def lifespan(self) -> AsyncIterator[None]`  _async_
  Combine lifespan with inner provider.

**Inherited (13)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Internal provider that wraps another provider with a transform.

Created by Provider.wrap_transform(). Delegates all component sourcing
to the inner provider's public methods (which apply inner's transforms),
then applies the wrapper's transform on top.

This enables immutable transform composition - the inner provider is
unchanged, and the wrapper adds its transform layer.


