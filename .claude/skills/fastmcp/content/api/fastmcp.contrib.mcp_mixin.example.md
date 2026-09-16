# `fastmcp.contrib.mcp_mixin.example`

Distribution: `fastmcp`

## first_sample

`fastmcp.contrib.mcp_mixin.example.first_sample`

```python
first_sample = Sample('First')
```

**Inferred type** (`ty`, not declared in the source): `Sample`

## mcp

`fastmcp.contrib.mcp_mixin.example.mcp`

```python
mcp = FastMCP()
```

**Inferred type** (`ty`, not declared in the source): `FastMCP[Any]`

## second_sample

`fastmcp.contrib.mcp_mixin.example.second_sample`

```python
second_sample = Sample('Second')
```

**Inferred type** (`ty`, not declared in the source): `Sample`

## Sample

`fastmcp.contrib.mcp_mixin.example.Sample`

```python
class Sample(MCPMixin)
```

**Bases** `MCPMixin`

**Declared members (4)**

- `def first_prompt(self)`
  First prompt description.
- `def first_resource(self)`
  First resource description.
- `def first_tool(self)`
  First tool description.
- `name = name`  _instance-attribute_

## list_components

`fastmcp.contrib.mcp_mixin.example.list_components`

```python
async def list_components() -> None
```

