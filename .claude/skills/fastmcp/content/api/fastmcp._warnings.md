# `fastmcp._warnings`

Distribution: `fastmcp`

## FastMCPDeprecationWarning

Import as `fastmcp.FastMCPDeprecationWarning`  ·  defined at `fastmcp._warnings.FastMCPDeprecationWarning`

```python
class FastMCPDeprecationWarning(DeprecationWarning)
```

**Also exported as** `fastmcp.FastMCPDeprecationWarning`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `DeprecationWarning`

Deprecation warning for FastMCP APIs.

Subclass of DeprecationWarning so that standard warning filters
still apply, but FastMCP can selectively enable its own warnings
without affecting other libraries in the process.


