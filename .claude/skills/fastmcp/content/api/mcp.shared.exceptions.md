# `mcp.shared.exceptions`

Distribution: `mcp`

## MCPDeprecationWarning

Import as `mcp.MCPDeprecationWarning`  ·  defined at `mcp.shared.exceptions.MCPDeprecationWarning`

```python
class MCPDeprecationWarning(UserWarning)
```

**Also exported as** `mcp.MCPDeprecationWarning`

_11 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `UserWarning`

A custom deprecation warning for the MCP SDK.

Unlike the built-in `DeprecationWarning`, this inherits from `UserWarning` so
it is shown by default, helping users discover deprecated features without
enabling warnings explicitly.

Reference: https://sethmlarson.dev/deprecations-via-warnings-dont-work-for-python-libraries


## MCPError

Import as `mcp.MCPError`  ·  defined at `mcp.shared.exceptions.MCPError`

```python
class MCPError(Exception)
```

**Also exported as** `mcp.MCPError`

_29 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (6)**

- `code: int`  _property_
- `data: Any`  _property_
- `error: ErrorData`  _instance-attribute_
- `def from_error_data(cls, error: ErrorData) -> MCPError`  _classmethod_
- `def from_jsonrpc_error(cls, error: JSONRPCError) -> MCPError`  _classmethod_
- `message: str`  _property_

Exception type raised when an error arrives over an MCP connection.


## NoBackChannelError

Import as `mcp.server.runner.NoBackChannelError`  ·  defined at `mcp.shared.exceptions.NoBackChannelError`

```python
class NoBackChannelError(MCPError)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPError`

**Declared members (1)**

- `method = method`  _instance-attribute_

**Inherited (6)**

- from `mcp.shared.exceptions.MCPError`: `code`, `data`, `error`, `from_error_data`, `from_jsonrpc_error`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Raised when a server-initiated request has no channel that can deliver it.

Raised by `DispatchContext.send_raw_request` when its request-scoped channel
reports `TransportContext.can_send_request` as `False` (the cases are
documented on that field), and by a connection's standalone channel when it
has none; serializes to an `INVALID_REQUEST` error response.


## UrlElicitationRequiredError

Import as `mcp.UrlElicitationRequiredError`  ·  defined at `mcp.shared.exceptions.UrlElicitationRequiredError`

```python
class UrlElicitationRequiredError(MCPError)
```

**Also exported as** `mcp.UrlElicitationRequiredError`

**Bases** `MCPError`

**Declared members (2)**

- `elicitations: list[ElicitRequestURLParams]`  _property_
  The list of URL elicitations required before the request can proceed.
- `def from_error(cls, error: ErrorData) -> UrlElicitationRequiredError`  _classmethod_
  Reconstruct from an ErrorData received over the wire.

**Inherited (6)**

- from `mcp.shared.exceptions.MCPError`: `code`, `data`, `error`, `from_error_data`, `from_jsonrpc_error`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Specialized error for when a tool requires URL mode elicitation(s) before proceeding.

Servers can raise this error from tool handlers to indicate that the client
must complete one or more URL elicitations before the request can be processed.

Example:
    ```python
    raise UrlElicitationRequiredError([
        ElicitRequestURLParams(
            message="Authorization required for your files",
            url="https://example.com/oauth/authorize",
            elicitation_id="auth-001"
        )
    ])
    ```


