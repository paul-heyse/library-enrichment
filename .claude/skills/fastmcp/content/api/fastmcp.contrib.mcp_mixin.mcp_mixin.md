# `fastmcp.contrib.mcp_mixin.mcp_mixin`

Distribution: `fastmcp`

## _DEFAULT_SEPARATOR_PROMPT

`fastmcp.contrib.mcp_mixin.mcp_mixin._DEFAULT_SEPARATOR_PROMPT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_SEPARATOR_PROMPT = '_'
```

## _DEFAULT_SEPARATOR_RESOURCE

`fastmcp.contrib.mcp_mixin.mcp_mixin._DEFAULT_SEPARATOR_RESOURCE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_SEPARATOR_RESOURCE = '+'
```

## _DEFAULT_SEPARATOR_TOOL

`fastmcp.contrib.mcp_mixin.mcp_mixin._DEFAULT_SEPARATOR_TOOL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_SEPARATOR_TOOL = '_'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _MCP_REGISTRATION_PROMPT_ATTR

`fastmcp.contrib.mcp_mixin.mcp_mixin._MCP_REGISTRATION_PROMPT_ATTR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_REGISTRATION_PROMPT_ATTR = '_mcp_prompt_registration'
```

## _MCP_REGISTRATION_RESOURCE_ATTR

`fastmcp.contrib.mcp_mixin.mcp_mixin._MCP_REGISTRATION_RESOURCE_ATTR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_REGISTRATION_RESOURCE_ATTR = '_mcp_resource_registration'
```

## _MCP_REGISTRATION_TOOL_ATTR

`fastmcp.contrib.mcp_mixin.mcp_mixin._MCP_REGISTRATION_TOOL_ATTR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_REGISTRATION_TOOL_ATTR = '_mcp_tool_registration'
```

## _MIXIN_ENABLED_KEY

`fastmcp.contrib.mcp_mixin.mcp_mixin._MIXIN_ENABLED_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MIXIN_ENABLED_KEY = '_mixin_enabled'
```

## _PROMPT_VALID_KWARGS

`fastmcp.contrib.mcp_mixin.mcp_mixin._PROMPT_VALID_KWARGS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PROMPT_VALID_KWARGS: frozenset[str] = frozenset(p for p in inspect.signature(Prompt.from_function).parameters if p != 'fn')
```

## _RESOURCE_VALID_KWARGS

`fastmcp.contrib.mcp_mixin.mcp_mixin._RESOURCE_VALID_KWARGS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RESOURCE_VALID_KWARGS: frozenset[str] = frozenset(p for p in inspect.signature(Resource.from_function).parameters if p not in ('fn', 'uri'))
```

## _TOOL_VALID_KWARGS

`fastmcp.contrib.mcp_mixin.mcp_mixin._TOOL_VALID_KWARGS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TOOL_VALID_KWARGS: frozenset[str] = frozenset(p for p in inspect.signature(Tool.from_function).parameters if p != 'fn')
```

## MCPMixin

Import as `fastmcp.contrib.mcp_mixin.MCPMixin`  ·  defined at `fastmcp.contrib.mcp_mixin.mcp_mixin.MCPMixin`

```python
class MCPMixin
```

**Also exported as** `fastmcp.contrib.mcp_mixin.MCPMixin`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `def register_all(self, mcp_server: FastMCP, prefix: str | None = None, tool_separator: str = _DEFAULT_SEPARATOR_TOOL, resource_separator: str = _DEFAULT_SEPARATOR_RESOURCE, prompt_separator: str = _DEFAULT_SEPARATOR_PROMPT) -> None`
  Registers all marked tools, resources, and prompts with the server.
- `def register_prompts(self, mcp_server: FastMCP, prefix: str | None = None, separator: str = _DEFAULT_SEPARATOR_PROMPT) -> None`
  Registers all methods marked with @mcp_prompt with the FastMCP server.
- `def register_resources(self, mcp_server: FastMCP, prefix: str | None = None, separator: str = _DEFAULT_SEPARATOR_RESOURCE) -> None`
  Registers all methods marked with @mcp_resource with the FastMCP server.
- `def register_tools(self, mcp_server: FastMCP, prefix: str | None = None, separator: str = _DEFAULT_SEPARATOR_TOOL) -> None`
  Registers all methods marked with @mcp_tool with the FastMCP server.

Base mixin class for objects that can register tools, resources, and prompts
with a FastMCP server instance using decorators.

This mixin provides methods like ``register_all``, ``register_tools``, etc.,
which iterate over the methods of the inheriting class, find methods
decorated with ``@mcp_tool``, ``@mcp_resource``, or ``@mcp_prompt``, and
register them with the provided FastMCP server instance.


## mcp_prompt

Import as `fastmcp.contrib.mcp_mixin.mcp_prompt`  ·  defined at `fastmcp.contrib.mcp_mixin.mcp_mixin.mcp_prompt`

```python
def mcp_prompt(name: str | None = None, enabled: bool | None = None, kwargs: Any = {}) -> Callable[[Callable[..., Any]], Callable[..., Any]]
```

**Also exported as** `fastmcp.contrib.mcp_mixin.mcp_prompt`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Decorator to mark a method as an MCP prompt for later registration.

Accepts all parameters supported by ``Prompt.from_function``.  Any new
parameters added to ``Prompt.from_function`` are automatically forwarded
without requiring changes here.

Args:
    name: Prompt name.  Defaults to the decorated method name.
    enabled: If ``False``, the prompt is skipped during registration.
    **kwargs: Additional keyword arguments forwarded verbatim to
        ``Prompt.from_function`` (e.g. ``description``, ``tags``,
        ``auth``, ``version``, …).

Raises:
    TypeError: If an unrecognised keyword argument is supplied.  The error
        is raised immediately at decoration time rather than later.


## mcp_resource

Import as `fastmcp.contrib.mcp_mixin.mcp_resource`  ·  defined at `fastmcp.contrib.mcp_mixin.mcp_mixin.mcp_resource`

```python
def mcp_resource(uri: str, name: str | None = None, enabled: bool | None = None, kwargs: Any = {}) -> Callable[[Callable[..., Any]], Callable[..., Any]]
```

**Also exported as** `fastmcp.contrib.mcp_mixin.mcp_resource`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Decorator to mark a method as an MCP resource for later registration.

Accepts all parameters supported by ``Resource.from_function``.  Any new
parameters added to ``Resource.from_function`` are automatically forwarded
without requiring changes here.

Args:
    uri: Resource URI (required).
    name: Resource name.  Defaults to the decorated method name.
    enabled: If ``False``, the resource is skipped during registration.
    **kwargs: Additional keyword arguments forwarded verbatim to
        ``Resource.from_function`` (e.g. ``description``, ``tags``,
        ``mime_type``, ``auth``, ``version``, …).

Raises:
    TypeError: If an unrecognised keyword argument is supplied.  The error
        is raised immediately at decoration time rather than later.


## mcp_tool

Import as `fastmcp.contrib.mcp_mixin.mcp_tool`  ·  defined at `fastmcp.contrib.mcp_mixin.mcp_mixin.mcp_tool`

```python
def mcp_tool(name: str | None = None, enabled: bool | None = None, kwargs: Any = {}) -> Callable[[Callable[..., Any]], Callable[..., Any]]
```

**Also exported as** `fastmcp.contrib.mcp_mixin.mcp_tool`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Decorator to mark a method as an MCP tool for later registration.

Accepts all parameters supported by ``Tool.from_function``.  Any new
parameters added to ``Tool.from_function`` are automatically forwarded
without requiring changes here.

Args:
    name: Tool name.  Defaults to the decorated method name.
    enabled: If ``False``, the tool is skipped during registration.
    **kwargs: Additional keyword arguments forwarded verbatim to
        ``Tool.from_function`` (e.g. ``description``, ``tags``,
        ``annotations``, ``auth``, ``timeout``, ``version``, …).

Raises:
    TypeError: If an unrecognised keyword argument is supplied.  The error
        is raised immediately at decoration time rather than later.


