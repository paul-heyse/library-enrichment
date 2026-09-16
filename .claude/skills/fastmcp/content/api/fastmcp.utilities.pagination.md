# `fastmcp.utilities.pagination`

Distribution: `fastmcp`

## T

`fastmcp.utilities.pagination.T`

```python
T = TypeVar('T')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## CursorState

`fastmcp.utilities.pagination.CursorState`

```python
class CursorState
```

**Declared members (3)**

- `def decode(cls, cursor: str) -> CursorState`  _classmethod_
  Decode cursor from an opaque string.
- `def encode(self) -> str`
  Encode cursor state to an opaque string.
- `offset: int`  _instance-attribute_

Internal representation of pagination cursor state.

The cursor encodes the offset into the result set. This is opaque to clients
per the MCP spec - they should not parse or modify cursors.


## paginate_sequence

Import as `fastmcp.server.mixins.mcp_operations.paginate_sequence`  ·  defined at `fastmcp.utilities.pagination.paginate_sequence`

```python
def paginate_sequence(items: Sequence[T], cursor: str | None, page_size: int) -> tuple[list[T], str | None]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Paginate a sequence of items.

Args:
    items: The full sequence to paginate.
    cursor: Optional cursor from a previous request. None for first page.
    page_size: Maximum number of items per page.

Returns:
    Tuple of (page_items, next_cursor). next_cursor is None if no more pages.

Raises:
    ValueError: If the cursor is invalid.


