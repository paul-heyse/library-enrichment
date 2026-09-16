# `fastmcp.exceptions`

Distribution: `fastmcp`

## FastMCPDeprecationWarning

`fastmcp.exceptions.FastMCPDeprecationWarning`

```python
FastMCPDeprecationWarning = _warnings.FastMCPDeprecationWarning
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'FastMCPDeprecationWarning'> ````

## McpError

`fastmcp.exceptions.McpError`

```python
McpError = MCPError
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'mcp.shared.exceptions.MCPError'> | <class 'fastmcp.exceptions.MCPError'> ````

## AuthorizationError

Import as `fastmcp.server.server.AuthorizationError`  ·  defined at `fastmcp.exceptions.AuthorizationError`

```python
class AuthorizationError(FastMCPError)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Error when authorization check fails.


## ClientError

`fastmcp.exceptions.ClientError`

```python
class ClientError(Exception)
```

**Bases** `Exception`

Error in client operations.


## DisabledError

Import as `fastmcp.server.mixins.mcp_operations.DisabledError`  ·  defined at `fastmcp.exceptions.DisabledError`

```python
class DisabledError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Object is disabled.


## FastMCPError

Import as `fastmcp.server.server.FastMCPError`  ·  defined at `fastmcp.exceptions.FastMCPError`

```python
class FastMCPError(Exception)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (1)**

- `log_level = log_level`  _instance-attribute_

Base error for FastMCP.


## InsufficientScopeError

Import as `fastmcp.server.middleware.authorization.InsufficientScopeError`  ·  defined at `fastmcp.exceptions.InsufficientScopeError`

```python
class InsufficientScopeError(AuthorizationError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AuthorizationError`

**Declared members (1)**

- `required_scopes = list(required_scopes)`  _instance-attribute_

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Authorization failed because the token is missing required OAuth scopes.

Unlike a bare ``AuthorizationError``, this carries the specific scopes the
caller must obtain. A component-level scope shortfall can then be signalled
as a spec-correct ``insufficient_scope`` step-up (SEP-2350 / RFC 6750 §3),
naming exactly what to re-authorize for instead of an opaque denial. The
named scopes are only the *unmet* ones, so an existing grant is accumulated
rather than replaced when the caller re-authorizes.


## InvalidSignature

`fastmcp.exceptions.InvalidSignature`

```python
class InvalidSignature(Exception)
```

**Bases** `Exception`

Invalid signature for use with FastMCP.


## MCPError

Import as `fastmcp.exceptions.McpError`  ·  defined at `fastmcp.exceptions.MCPError`

```python
class MCPError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (1)**

- `error = ErrorData(code=code, message=message, data=data)`  _instance-attribute_

Fallback used when MCP dependencies are not installed.

Mirrors the real ``mcp.MCPError`` interface — the ``(code, message,
data)`` constructor and the ``.error`` ``ErrorData`` payload — so static
analysis of both construction and read sites (e.g. ``to_mcp_error`` and
callers reading ``err.error.code``) is valid regardless of which branch
is in effect.


## NotFoundError

Import as `fastmcp.server.server.NotFoundError`  ·  defined at `fastmcp.exceptions.NotFoundError`

```python
class NotFoundError(Exception)
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Object not found.


## PromptError

Import as `fastmcp.server.server.PromptError`  ·  defined at `fastmcp.exceptions.PromptError`

```python
class PromptError(FastMCPError)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Error in prompt operations.


## ResourceError

Import as `fastmcp.server.server.ResourceError`  ·  defined at `fastmcp.exceptions.ResourceError`

```python
class ResourceError(FastMCPError)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Error in resource operations.


## ResourceSecurityError

Import as `fastmcp.server.server.ResourceSecurityError`  ·  defined at `fastmcp.exceptions.ResourceSecurityError`

```python
class ResourceSecurityError(NotFoundError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NotFoundError`

A templated resource parameter failed path-security screening.

Subclasses ``NotFoundError`` so the read handler surfaces a
non-leaky ``INVALID_PARAMS`` (-32602) "resource not found" error to
the client — a traversal attempt is indistinguishable from a request
for a resource that does not exist, and never reveals which parameter
or policy tripped.


## ToolError

Import as `fastmcp.server.server.ToolError`  ·  defined at `fastmcp.exceptions.ToolError`

```python
class ToolError(FastMCPError)
```

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Error in tool operations.


## ValidationError

Import as `fastmcp.server.server.ValidationError`  ·  defined at `fastmcp.exceptions.ValidationError`

```python
class ValidationError(FastMCPError)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Error in validating parameters or return values.


## to_mcp_error

Import as `fastmcp.server.mixins.mcp_operations.to_mcp_error`  ·  defined at `fastmcp.exceptions.to_mcp_error`

```python
def to_mcp_error(exc: Exception, default_code: int = INTERNAL_ERROR) -> MCPError
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Translate a FastMCP exception into a wire-format ``MCPError``.

Central mapping from FastMCP's public exception types to the JSON-RPC error
codes defined by the MCP spec (imported from ``mcp_types``). Request-handler
adapters call this instead of hand-rolling ``MCPError(code=..., ...)`` per
call site, so the wire codes stay spec-correct and consistent across
resources, prompts, and tools.

``NotFoundError`` and ``DisabledError`` map to ``INVALID_PARAMS`` (-32602):
per SEP-2164 a request naming a component that does not exist (or is
disabled) is an invalid-params error, which matches the SDK's own
``ResourceNotFoundError -> INVALID_PARAMS`` mapping in ``mcp.server.mcpserver``.
``ValidationError`` is also an invalid-params error. Everything else falls
back to ``default_code`` (``INTERNAL_ERROR`` by default).

If ``exc`` is already an ``MCPError``, it is returned unchanged so an
explicit code chosen upstream survives translation.


