# `fastmcp.types`

Distribution: `fastmcp`

## Textarea

`fastmcp.types.Textarea`

```python
Textarea = Annotated[str, Field(json_schema_extra={'format': 'textarea'})]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'typing.Annotated[str, <metadata>]'> ``` --- A string rendered as a multiline textarea in form-based UIs. Produces `"format": "textarea"` in the JSON Schema, which `fastmcp dev apps` picks up automatically.`

A string rendered as a multiline textarea in form-based UIs.

Produces `"format": "textarea"` in the JSON Schema, which
`fastmcp dev apps` picks up automatically.


## __all__

`fastmcp.types.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Textarea']
```

