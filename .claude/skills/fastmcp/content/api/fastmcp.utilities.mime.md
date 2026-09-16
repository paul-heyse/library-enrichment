# `fastmcp.utilities.mime`

Distribution: `fastmcp`

## UI_MIME_TYPE

Import as `fastmcp.apps.UI_MIME_TYPE`  ·  defined at `fastmcp.utilities.mime.UI_MIME_TYPE`

```python
UI_MIME_TYPE = 'text/html;profile=mcp-app'
```

**Inferred type** (`ty`, not declared in the source): `Literal["text/html;profile=mcp-app"]`

**Also exported as** `fastmcp.apps.UI_MIME_TYPE`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## resolve_ui_mime_type

Import as `fastmcp.apps.resolve_ui_mime_type`  ·  defined at `fastmcp.utilities.mime.resolve_ui_mime_type`

```python
def resolve_ui_mime_type(uri: str, explicit_mime_type: str | None) -> str | None
```

**Also exported as** `fastmcp.apps.resolve_ui_mime_type`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return the appropriate MIME type for a resource URI.

For ``ui://`` scheme resources, defaults to ``UI_MIME_TYPE`` when no
explicit MIME type is provided.

Args:
    uri: The resource URI string
    explicit_mime_type: The MIME type explicitly provided by the user

Returns:
    The resolved MIME type (explicit value, UI default, or None)


