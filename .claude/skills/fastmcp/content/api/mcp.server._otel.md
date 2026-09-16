# `mcp.server._otel`

Distribution: `mcp`

## OpenTelemetryMiddleware

Import as `mcp.server.lowlevel.server.OpenTelemetryMiddleware`  ·  defined at `mcp.server._otel.OpenTelemetryMiddleware`

```python
class OpenTelemetryMiddleware(ServerMiddleware[Any])
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ServerMiddleware[Any]`

Context-tier middleware that wraps each inbound message in an OpenTelemetry span.


