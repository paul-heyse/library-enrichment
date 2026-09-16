# `mcp.shared.peer`

Distribution: `mcp`

## Meta

Import as `mcp.server.context.Meta`  ·  defined at `mcp.shared.peer.Meta`

```python
Meta = dict[str, Any]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'dict[str, Any]'> ``` --- Type alias for the `_meta` field carried on request/notification params.`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Type alias for the `_meta` field carried on request/notification params.


## __all__

`mcp.shared.peer.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ClientPeer', 'Meta']
```

## ClientPeer

`mcp.shared.peer.ClientPeer`

```python
class ClientPeer
```

**Declared members (7)**

- `async def elicit_form(self, message: str, requested_schema: ElicitRequestedSchema, meta: Meta | None = None, opts: CallOptions | None = None) -> ElicitResult`  _async_
  Send a form-mode `elicitation/create` request.
- `async def elicit_url(self, message: str, url: str, elicitation_id: str, meta: Meta | None = None, opts: CallOptions | None = None) -> ElicitResult`  _async_
  Send a URL-mode `elicitation/create` request.
- `async def list_roots(self, meta: Meta | None = None, opts: CallOptions | None = None) -> ListRootsResult`  _async_
  Send a `roots/list` request.
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
- `async def ping(self, meta: Meta | None = None, opts: CallOptions | None = None) -> None`  _async_
  Send a `ping` request and ignore the result.
- `async def sample(self, messages: list[SamplingMessage], max_tokens: int, system_prompt: str | None = None, include_context: IncludeContext | None = None, temperature: float | None = None, stop_sequences: list[str] | None = None, metadata: dict[str, Any] | None = None, model_preferences: ModelPreferences | None = None, tools: list[Tool] | None = None, tool_choice: ToolChoice | None = None, meta: Meta | None = None, opts: CallOptions | None = None) -> CreateMessageResult | CreateMessageResultWithTools`  _async_
  Send a `sampling/createMessage` request to the peer.
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_

Typed server-to-client request methods over a wrapped `Outbound`.

Use this when you have a bare dispatcher (or any `Outbound`) and want the
typed methods (`sample`, `elicit_form`, `elicit_url`, `list_roots`,
`ping`) without writing your own host class.


## dump_params

Import as `mcp.server.connection.dump_params`  ·  defined at `mcp.shared.peer.dump_params`

```python
def dump_params(model: BaseModel | None, meta: Meta | None = None) -> dict[str, Any] | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Serialize a params model to a wire dict, merging `meta` into `_meta`.

Shared by `ClientPeer` and `Connection` so every typed convenience method
gets the same `_meta` handling. `meta` keys take precedence over any
`_meta` already present on the model.

`meta` is serialized through `RequestParams` so Python field names emit
their wire aliases: an inbound `ctx.meta` carries `progress_token` (the
key `_extract_meta` validation produces), and forwarding it outbound via
`meta=ctx.meta` must put `progressToken` back on the wire. Keys not
declared on `RequestParamsMeta` pass through unchanged.


