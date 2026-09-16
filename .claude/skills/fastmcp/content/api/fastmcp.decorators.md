# `fastmcp.decorators`

Distribution: `fastmcp`

## FastMCPMeta

`fastmcp.decorators.FastMCPMeta`

```python
FastMCPMeta = ToolMeta | ResourceMeta | PromptMeta
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'ToolMeta | ResourceMeta | PromptMeta'> ````

## HasFastMCPMeta

`fastmcp.decorators.HasFastMCPMeta`

```python
class HasFastMCPMeta(Protocol)
```

**Bases** `Protocol`

Protocol for callables decorated with FastMCP metadata.


## get_fastmcp_meta

Import as `fastmcp.tools.function_tool.get_fastmcp_meta`  ·  defined at `fastmcp.decorators.get_fastmcp_meta`

```python
def get_fastmcp_meta(fn: Any) -> Any | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract FastMCP metadata from a function, handling bound methods and wrappers.


## resolve_task_config

`fastmcp.decorators.resolve_task_config`

```python
def resolve_task_config(task: bool | TaskConfig | None) -> bool | TaskConfig
```

Resolve task config, defaulting None to False.


