# `mcp.server.mcpserver.exceptions`

Distribution: `mcp`

## InvalidSignature

Import as `mcp.server.mcpserver.resolve.InvalidSignature`  ·  defined at `mcp.server.mcpserver.exceptions.InvalidSignature`

```python
class InvalidSignature(Exception)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Invalid signature for use with MCPServer.


## MCPServerError

`mcp.server.mcpserver.exceptions.MCPServerError`

```python
class MCPServerError(Exception)
```

**Bases** `Exception`

Base error for MCPServer.


## ResourceError

Import as `mcp.server.mcpserver.server.ResourceError`  ·  defined at `mcp.server.mcpserver.exceptions.ResourceError`

```python
class ResourceError(MCPServerError)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPServerError`

A resource failure you anticipated.

Raise this from a resource or resource template handler for a failure you saw
coming: the client receives a `-32603` protocol error carrying your message
(`ResourceNotFoundError` below is the `-32602` variant), and the server logs it
at INFO without a traceback. Any other exception (bar `MCPError`, which is a
protocol error) is treated as a crash: the client gets a generic message naming
only the URI, and the server logs the traceback at ERROR.

The SDK raises it too, and `UnexpectedResourceError` subclasses it, so
`except ResourceError` around `MCPServer.read_resource()` catches every read
failure, crash or not.


## ResourceNotFoundError

Import as `mcp.server.mcpserver.server.ResourceNotFoundError`  ·  defined at `mcp.server.mcpserver.exceptions.ResourceNotFoundError`

```python
class ResourceNotFoundError(ResourceError)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ResourceError`

Resource does not exist.

Raise this from a resource handler to signal that the requested instance does not exist.
Clients receive `-32602` (invalid params) per
[SEP-2164](https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2164).


## ToolError

Import as `mcp.server.mcpserver.server.ToolError`  ·  defined at `mcp.server.mcpserver.exceptions.ToolError`

```python
class ToolError(MCPServerError)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPServerError`

A tool failure you anticipated.

Raise this from a tool (or a resolver) for a failure you saw coming: the
call returns `is_error=True` with your message in `content` for the model to
read, and the server logs it at INFO without a traceback. A `ResourceError`
that escapes the tool (say from `ctx.read_resource()`) counts the same. Any
other exception bar `MCPError` (a protocol error) is treated as a crash: the
model sees only `Error executing tool <name>`, and the server logs the
traceback at ERROR. Inside a pydantic validator, raise `ValueError` as pydantic
expects; it arrives as an argument-validation failure, which is anticipated too.

The SDK raises it too, for an unknown tool name and for arguments that fail
the input schema, and `UnexpectedToolError` subclasses it, so `except ToolError`
around `MCPServer.call_tool()` catches every tool failure, crash or not.


## UnexpectedResourceError

Import as `mcp.server.mcpserver.server.UnexpectedResourceError`  ·  defined at `mcp.server.mcpserver.exceptions.UnexpectedResourceError`

```python
class UnexpectedResourceError(ResourceError)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ResourceError`

A resource read failed with something other than `ResourceError` or `MCPError`.

The SDK raises this itself, around a crash in a resource or resource template
handler. You never raise it. `__cause__` is the original exception, which the
server logs with its traceback. The message names only the URI, so the
original text is withheld from the client.


## UnexpectedToolError

Import as `mcp.server.mcpserver.server.UnexpectedToolError`  ·  defined at `mcp.server.mcpserver.exceptions.UnexpectedToolError`

```python
class UnexpectedToolError(ToolError)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ToolError`

A tool call failed with something other than `ToolError`, `ResourceError`, or `MCPError`.

The SDK raises this itself, around a crash in the tool (or a resolver) or a
return value that fails output conversion. You never raise it. The message is
only `Error executing tool <name>` (followed by the same for a nested tool or
resource that crashed), so nothing from the original reaches the client.
`__cause__` is the original exception (or, for a nested tool or resource
crash, its `Unexpected...Error` wrapper), which the server logs with its
traceback before returning the `is_error=True` result. Catch it around
`MCPServer.call_tool()` to tell a crash from a deliberate `ToolError`.


