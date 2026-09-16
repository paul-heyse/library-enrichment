# `mcp.shared.transport_context`

Distribution: `mcp`

## __all__

`mcp.shared.transport_context.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['TransportContext']
```

## TransportContext

Import as `mcp.server.runner.TransportContext`  ·  defined at `mcp.shared.transport_context.TransportContext`

```python
class TransportContext
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `can_send_request: bool`  _instance-attribute_
  Whether this message's request-scoped channel can deliver a server-initiated request.
- `headers: Mapping[str, str] | None = None`  _class-attribute, instance-attribute_
  Request headers carried by this message, when the transport has them.
- `kind: str`  _instance-attribute_
  Short identifier for the transport (e.g. `"stdio"`, `"streamable-http"`).

Base transport metadata for an inbound message.

Subclass per transport and add fields as needed. Instances are immutable.


