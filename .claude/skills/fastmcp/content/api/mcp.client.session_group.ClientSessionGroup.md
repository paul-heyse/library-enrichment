# `mcp.client.session_group.ClientSessionGroup`

Distribution: `mcp`

## _ComponentNames

`mcp.client.session_group.ClientSessionGroup._ComponentNames`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ComponentNames(BaseModel)
```

**Bases** `BaseModel`

**Declared members (3)**

- `prompts: set[str] = Field(default_factory=set)`  _class-attribute, instance-attribute_
- `resources: set[str] = Field(default_factory=set)`  _class-attribute, instance-attribute_
- `tools: set[str] = Field(default_factory=set)`  _class-attribute, instance-attribute_

Used for reverse index to find components.


