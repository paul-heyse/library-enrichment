# `mcp.client._probe`

Distribution: `mcp`

## _parse_supported

`mcp.client._probe._parse_supported`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_supported(data: Any) -> list[str] | None
```

Pull ``data.supported`` off a -32022 error, or ``None`` if not actionable.


## negotiate_auto

Import as `mcp.client.client.negotiate_auto`  ·  defined at `mcp.client._probe.negotiate_auto`

```python
async def negotiate_auto(session: ClientSession) -> None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Drive the ``mode='auto'`` connect-time policy on ``session``.

Probes ``server/discover`` once (twice if the server names a mutual
modern version via -32022), then either ``adopt()``s the result or falls
back to ``initialize()``. Idempotent only in the sense that one of
``session.discover_result`` / ``session.initialize_result`` is set on
return.

Raises:
    MCPError: The server is modern-only and shares no version with this
        client (-32022 with a disjoint ``supported`` list), or the
        fallback handshake failed and one corrective re-probe did too.
    Exception: Any transport/network error from the probe propagates as-is.


