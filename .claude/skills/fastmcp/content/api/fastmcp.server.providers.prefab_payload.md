# `fastmcp.server.providers.prefab_payload`

Distribution: `fastmcp`

## IdentityResolver

`fastmcp.server.providers.prefab_payload.IdentityResolver`

```python
IdentityResolver = Callable[[str], str | None]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(str, /) -> str | None'> ````

## _FASTMCP_KEY

`fastmcp.server.providers.prefab_payload._FASTMCP_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FASTMCP_KEY = 'fastmcp'
```

## _META_KEY

`fastmcp.server.providers.prefab_payload._META_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_META_KEY = '_meta'
```

## _TOOL_CALL_ACTION

`fastmcp.server.providers.prefab_payload._TOOL_CALL_ACTION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TOOL_CALL_ACTION = 'toolCall'
```

## _TOOL_NAMES_KEY

`fastmcp.server.providers.prefab_payload._TOOL_NAMES_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TOOL_NAMES_KEY = 'toolNames'
```

## _read_map

`fastmcp.server.providers.prefab_payload._read_map`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _read_map(payload: dict[str, Any]) -> dict[str, str]
```

## _walk_tool_calls

`fastmcp.server.providers.prefab_payload._walk_tool_calls`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _walk_tool_calls(node: Any) -> list[dict[str, Any]]
```

Collect every ``toolCall`` action object in a payload tree.


## _write_map

`fastmcp.server.providers.prefab_payload._write_map`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _write_map(payload: dict[str, Any], names: dict[str, str]) -> None
```

## annotate_payload_identities

`fastmcp.server.providers.prefab_payload.annotate_payload_identities`

```python
def annotate_payload_identities(payload: dict[str, Any]) -> dict[str, Any]
```

Record the identity-addressed form of each reference, at serialization.

References start out as ``<hash>_<local_name>``, so the map begins as an
identity map to itself. Once a later layer rewrites a name, this is the
only remaining route back: it carries both what the reference points at
and the address any server can fall back to.


## payload_has_identities

`fastmcp.server.providers.prefab_payload.payload_has_identities`

```python
def payload_has_identities(payload: Any) -> bool
```

Cheap guard: does this payload carry tool references worth rewriting?

Runs on every tool result, so it must not walk the tree.


## rewrite_payload_tool_names

`fastmcp.server.providers.prefab_payload.rewrite_payload_tool_names`

```python
def rewrite_payload_tool_names(payload: Any, resolve: IdentityResolver) -> Any
```

Re-address a payload's tool references to this server's own names.

Mutates in place and returns the payload.

A reference this server cannot resolve is restored to its
identity-addressed form rather than left as-is. Leaving it would strand
whatever name an inner server chose — a name that is correct there and
meaningless here — and, unlike the identity form, a stranded name has no
route back. Restoring keeps the reference resolvable by the dispatcher,
or by any server further out with a better view.


