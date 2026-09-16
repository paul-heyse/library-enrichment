# `fastmcp._compat`

Distribution: `fastmcp`

## _ALIASES

`fastmcp._compat._ALIASES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ALIASES: dict[type, dict[str, str]] = {mcp_types.Tool: {'inputSchema': 'input_schema', 'outputSchema': 'output_schema'}, mcp_types.ToolAnnotations: {'readOnlyHint': 'read_only_hint', 'destructiveHint': 'destructive_hint', 'idempotentHint': 'idempotent_hint', 'openWorldHint': 'open_world_hint'}, mcp_types.Resource: {'mimeType': 'mime_type'}, mcp_types.ResourceTemplate: {'mimeType': 'mime_type', 'uriTemplate': 'uri_template'}, mcp_types.TextResourceContents: {'mimeType': 'mime_type'}, mcp_types.BlobResourceContents: {'mimeType': 'mime_type'}, mcp_types.ImageContent: {'mimeType': 'mime_type'}, mcp_types.AudioContent: {'mimeType': 'mime_type'}, mcp_types.CallToolResult: {'isError': 'is_error', 'structuredContent': 'structured_content'}, mcp_types.Completion: {'hasMore': 'has_more'}, mcp_types.InitializeResult: {'serverInfo': 'server_info', 'protocolVersion': 'protocol_version'}, mcp_types.ListToolsResult: {'nextCursor': 'next_cursor'}, mcp_types.ListResourcesResult: {'nextCursor': 'next_cursor'}, mcp_types.ListResourceTemplatesResult: {'nextCursor': 'next_cursor', 'resourceTemplates': 'resource_templates'}, mcp_types.ListPromptsResult: {'nextCursor': 'next_cursor'}, mcp_types.CreateMessageRequestParams: {'systemPrompt': 'system_prompt', 'maxTokens': 'max_tokens', 'stopSequences': 'stop_sequences', 'modelPreferences': 'model_preferences', 'toolChoice': 'tool_choice'}, mcp_types.ElicitRequestFormParams: {'requestedSchema': 'requested_schema'}}
```

## _installed

`fastmcp._compat._installed`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_installed = False
```

## _make_property

`fastmcp._compat._make_property`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_property(cls_name: str, camel: str, snake: str) -> property
```

Build a warn-once property routing a camelCase read to a snake attr.

The getter reads the live `mcp_camelcase_compat` setting on every access: if
the bridge is disabled it raises `AttributeError` (matching the message
Python raises for a genuinely missing attribute) so the shim is transparent;
if enabled it warns once and returns the snake_case value.


## install

`fastmcp._compat.install`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def install() -> None
```

Install camelCase compatibility properties on SDK v2 model classes.

Idempotent. Each bridged read warns once per (class, name) and returns the
snake_case value. Skips any camelCase name a class already defines to avoid
shadowing real upstream attributes.


