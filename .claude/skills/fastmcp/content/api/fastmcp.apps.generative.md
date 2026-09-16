# `fastmcp.apps.generative`

Distribution: `fastmcp`

## logger

`fastmcp.apps.generative.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## GenerativeUI

`fastmcp.apps.generative.GenerativeUI`

```python
class GenerativeUI(Provider)
```

**Bases** `Provider`

**Declared members (1)**

- `async def lifespan(self) -> AsyncIterator[None]`  _async_

**Inherited (16)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Provider that adds generative UI capabilities to a server.

Registers:

- A ``generate_ui`` tool that accepts Prefab Python code, executes
  it in a Pyodide sandbox, and returns the rendered PrefabApp.
  Supports streaming via ``ontoolinputpartial``.
- A ``components`` tool that searches the Prefab component library.
- The generative renderer resource with CSP for Pyodide CDN access.

Example::

    from fastmcp import FastMCP
    from fastmcp.apps.generative import GenerativeUI

    mcp = FastMCP("My Server")
    mcp.add_provider(GenerativeUI())


## _build_csp

`fastmcp.apps.generative._build_csp`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_csp() -> ResourceCSP
```

Build CSP from the generative renderer's declared requirements.


