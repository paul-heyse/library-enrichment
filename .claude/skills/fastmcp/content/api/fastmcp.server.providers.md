# `fastmcp.server.providers`

Distribution: `fastmcp`

## __all__

`fastmcp.server.providers.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['AggregateProvider', 'ClaudeSkillsProvider', 'FastMCPProvider', 'FileSystemProvider', 'LocalProvider', 'OpenAPIProvider', 'Provider', 'ProxyProvider', 'SkillProvider', 'SkillsDirectoryProvider']
```

## __getattr__

`fastmcp.server.providers.__getattr__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def __getattr__(name: str) -> object
```

Lazy import for providers to avoid circular imports.


