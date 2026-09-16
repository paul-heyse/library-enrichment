# `mcp_types.methods`

Distribution: `mcp-types`

## CACHEABLE_METHODS

Import as `mcp.server.caching.CACHEABLE_METHODS`  ·  defined at `mcp_types.methods.CACHEABLE_METHODS`

```python
CACHEABLE_METHODS: Final[frozenset[str]] = frozenset(method for method, row in MONOLITH_RESULTS.items() if any(issubclass(arm, types.CacheableResult) for arm in (get_args(row) if isinstance(row, UnionType) else (row,))))
```

**Also exported as** `mcp.server.caching.CACHEABLE_METHODS`

Runtime mirror of `CacheableMethod`, derived from `MONOLITH_RESULTS`.


## CLIENT_NOTIFICATIONS

`mcp_types.methods.CLIENT_NOTIFICATIONS`

```python
CLIENT_NOTIFICATIONS: Final[Mapping[tuple[str, str], type[BaseModel]]] = MappingProxyType({('notifications/cancelled', '2024-11-05'): v2025.CancelledNotification, ('notifications/initialized', '2024-11-05'): v2025.InitializedNotification, ('notifications/progress', '2024-11-05'): v2025.ProgressNotification, ('notifications/roots/list_changed', '2024-11-05'): v2025.RootsListChangedNotification, ('notifications/cancelled', '2025-03-26'): v2025.CancelledNotification, ('notifications/initialized', '2025-03-26'): v2025.InitializedNotification, ('notifications/progress', '2025-03-26'): v2025.ProgressNotification, ('notifications/roots/list_changed', '2025-03-26'): v2025.RootsListChangedNotification, ('notifications/cancelled', '2025-06-18'): v2025.CancelledNotification, ('notifications/initialized', '2025-06-18'): v2025.InitializedNotification, ('notifications/progress', '2025-06-18'): v2025.ProgressNotification, ('notifications/roots/list_changed', '2025-06-18'): v2025.RootsListChangedNotification, ('notifications/cancelled', '2025-11-25'): v2025.CancelledNotification, ('notifications/initialized', '2025-11-25'): v2025.InitializedNotification, ('notifications/progress', '2025-11-25'): v2025.ProgressNotification, ('notifications/roots/list_changed', '2025-11-25'): v2025.RootsListChangedNotification, ('notifications/cancelled', '2026-07-28'): v2026.CancelledNotification})
```

## CLIENT_REQUESTS

`mcp_types.methods.CLIENT_REQUESTS`

```python
CLIENT_REQUESTS: Final[Mapping[tuple[str, str], type[BaseModel]]] = MappingProxyType({('completion/complete', '2024-11-05'): v2025.CompleteRequest, ('initialize', '2024-11-05'): v2025.InitializeRequest, ('logging/setLevel', '2024-11-05'): v2025.SetLevelRequest, ('ping', '2024-11-05'): v2025.PingRequest, ('prompts/get', '2024-11-05'): v2025.GetPromptRequest, ('prompts/list', '2024-11-05'): v2025.ListPromptsRequest, ('resources/list', '2024-11-05'): v2025.ListResourcesRequest, ('resources/read', '2024-11-05'): v2025.ReadResourceRequest, ('resources/subscribe', '2024-11-05'): v2025.SubscribeRequest, ('resources/templates/list', '2024-11-05'): v2025.ListResourceTemplatesRequest, ('resources/unsubscribe', '2024-11-05'): v2025.UnsubscribeRequest, ('tools/call', '2024-11-05'): v2025.CallToolRequest, ('tools/list', '2024-11-05'): v2025.ListToolsRequest, ('completion/complete', '2025-03-26'): v2025.CompleteRequest, ('initialize', '2025-03-26'): v2025.InitializeRequest, ('logging/setLevel', '2025-03-26'): v2025.SetLevelRequest, ('ping', '2025-03-26'): v2025.PingRequest, ('prompts/get', '2025-03-26'): v2025.GetPromptRequest, ('prompts/list', '2025-03-26'): v2025.ListPromptsRequest, ('resources/list', '2025-03-26'): v2025.ListResourcesRequest, ('resources/read', '2025-03-26'): v2025.ReadResourceRequest, ('resources/subscribe', '2025-03-26'): v2025.SubscribeRequest, ('resources/templates/list', '2025-03-26'): v2025.ListResourceTemplatesRequest, ('resources/unsubscribe', '2025-03-26'): v2025.UnsubscribeRequest, ('tools/call', '2025-03-26'): v2025.CallToolRequest, ('tools/list', '2025-03-26'): v2025.ListToolsRequest, ('completion/complete', '2025-06-18'): v2025.CompleteRequest, ('initialize', '2025-06-18'): v2025.InitializeRequest, ('logging/setLevel', '2025-06-18'): v2025.SetLevelRequest, ('ping', '2025-06-18'): v2025.PingRequest, ('prompts/get', '2025-06-18'): v2025.GetPromptRequest, ('prompts/list', '2025-06-18'): v2025.ListPromptsRequest, ('resources/list', '2025-06-18'): v2025.ListResourcesRequest, ('resources/read', '2025-06-18'): v2025.ReadResourceRequest, ('resources/subscribe', '2025-06-18'): v2025.SubscribeRequest, ('resources/templates/list', '2025-06-18'): v2025.ListResourceTemplatesRequest, ('resources/unsubscribe', '2025-06-18'): v2025.UnsubscribeRequest, ('tools/call', '2025-06-18'): v2025.CallToolRequest, ('tools/list', '2025-06-18'): v2025.ListToolsRequest, ('completion/complete', '2025-11-25'): v2025.CompleteRequest, ('initialize', '2025-11-25'): v2025.InitializeRequest, ('logging/setLevel', '2025-11-25'): v2025.SetLevelRequest, ('ping', '2025-11-25'): v2025.PingRequest, ('prompts/get', '2025-11-25'): v2025.GetPromptRequest, ('prompts/list', '2025-11-25'): v2025.ListPromptsRequest, ('resources/list', '2025-11-25'): v2025.ListResourcesRequest, ('resources/read', '2025-11-25'): v2025.ReadResourceRequest, ('resources/subscribe', '2025-11-25'): v2025.SubscribeRequest, ('resources/templates/list', '2025-11-25'): v2025.ListResourceTemplatesRequest, ('resources/unsubscribe', '2025-11-25'): v2025.UnsubscribeRequest, ('tools/call', '2025-11-25'): v2025.CallToolRequest, ('tools/list', '2025-11-25'): v2025.ListToolsRequest, ('completion/complete', '2026-07-28'): v2026.CompleteRequest, ('prompts/get', '2026-07-28'): v2026.GetPromptRequest, ('prompts/list', '2026-07-28'): v2026.ListPromptsRequest, ('resources/list', '2026-07-28'): v2026.ListResourcesRequest, ('resources/read', '2026-07-28'): v2026.ReadResourceRequest, ('resources/templates/list', '2026-07-28'): v2026.ListResourceTemplatesRequest, ('server/discover', '2026-07-28'): v2026.DiscoverRequest, ('subscriptions/listen', '2026-07-28'): v2026.SubscriptionsListenRequest, ('tools/call', '2026-07-28'): v2026.CallToolRequest, ('tools/list', '2026-07-28'): v2026.ListToolsRequest})
```

## CLIENT_RESULTS

`mcp_types.methods.CLIENT_RESULTS`

```python
CLIENT_RESULTS: Final[Mapping[tuple[str, str], type[BaseModel] | UnionType]] = MappingProxyType({('ping', '2024-11-05'): v2025.EmptyResult, ('roots/list', '2024-11-05'): v2025.ListRootsResult, ('sampling/createMessage', '2024-11-05'): v2025.CreateMessageResult, ('ping', '2025-03-26'): v2025.EmptyResult, ('roots/list', '2025-03-26'): v2025.ListRootsResult, ('sampling/createMessage', '2025-03-26'): v2025.CreateMessageResult, ('elicitation/create', '2025-06-18'): v2025.ElicitResult, ('ping', '2025-06-18'): v2025.EmptyResult, ('roots/list', '2025-06-18'): v2025.ListRootsResult, ('sampling/createMessage', '2025-06-18'): v2025.CreateMessageResult, ('elicitation/create', '2025-11-25'): v2025.ElicitResult, ('ping', '2025-11-25'): v2025.EmptyResult, ('roots/list', '2025-11-25'): v2025.ListRootsResult, ('sampling/createMessage', '2025-11-25'): v2025.CreateMessageResult})
```

Results clients send, keyed by the originating server request's (method, version).


## CacheableMethod

Import as `mcp.server.caching.CacheableMethod`  ·  defined at `mcp_types.methods.CacheableMethod`

```python
CacheableMethod = Literal['prompts/list', 'resources/list', 'resources/read', 'resources/templates/list', 'server/discover', 'tools/list']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["prompts/list", "resources/list", "resources/read", "resources/templates/list", "server/discover", "tools/list"]'> ``` --- Methods whose results carry `ttlMs`/`cacheScope`; hand-written Literal, welded to `CACHEABLE_METHODS` by tests.`

**Also exported as** `mcp.server.caching.CacheableMethod`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Methods whose results carry `ttlMs`/`cacheScope`; hand-written Literal, welded to `CACHEABLE_METHODS` by tests.


## INPUT_REQUIRED_METHODS

Import as `mcp.server.request_state.INPUT_REQUIRED_METHODS`  ·  defined at `mcp_types.methods.INPUT_REQUIRED_METHODS`

```python
INPUT_REQUIRED_METHODS: Final[frozenset[str]] = frozenset(method for method, row in MONOLITH_RESULTS.items() if any(issubclass(arm, types.InputRequiredResult) for arm in (get_args(row) if isinstance(row, UnionType) else (row,))))
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Methods whose results may be `InputRequiredResult`, derived from `MONOLITH_RESULTS`.


## MONOLITH_NOTIFICATIONS

`mcp_types.methods.MONOLITH_NOTIFICATIONS`

```python
MONOLITH_NOTIFICATIONS: Final[Mapping[str, type[types.Notification[Any, Any]]]] = MappingProxyType({'notifications/cancelled': types.CancelledNotification, 'notifications/elicitation/complete': types.ElicitCompleteNotification, 'notifications/initialized': types.InitializedNotification, 'notifications/message': types.LoggingMessageNotification, 'notifications/progress': types.ProgressNotification, 'notifications/prompts/list_changed': types.PromptListChangedNotification, 'notifications/resources/list_changed': types.ResourceListChangedNotification, 'notifications/resources/updated': types.ResourceUpdatedNotification, 'notifications/roots/list_changed': types.RootsListChangedNotification, 'notifications/subscriptions/acknowledged': types.SubscriptionsAcknowledgedNotification, 'notifications/tools/list_changed': types.ToolListChangedNotification})
```

Monolith notification model per method, both directions.


## MONOLITH_REQUESTS

`mcp_types.methods.MONOLITH_REQUESTS`

```python
MONOLITH_REQUESTS: Final[Mapping[str, type[types.Request[Any, Any]]]] = MappingProxyType({'completion/complete': types.CompleteRequest, 'elicitation/create': types.ElicitRequest, 'initialize': types.InitializeRequest, 'logging/setLevel': types.SetLevelRequest, 'ping': types.PingRequest, 'prompts/get': types.GetPromptRequest, 'prompts/list': types.ListPromptsRequest, 'resources/list': types.ListResourcesRequest, 'resources/read': types.ReadResourceRequest, 'resources/subscribe': types.SubscribeRequest, 'resources/templates/list': types.ListResourceTemplatesRequest, 'resources/unsubscribe': types.UnsubscribeRequest, 'roots/list': types.ListRootsRequest, 'sampling/createMessage': types.CreateMessageRequest, 'server/discover': types.DiscoverRequest, 'subscriptions/listen': types.SubscriptionsListenRequest, 'tools/call': types.CallToolRequest, 'tools/list': types.ListToolsRequest})
```

Monolith request model per method, both directions.


## MONOLITH_RESULTS

Import as `fastmcp.client.caching.MONOLITH_RESULTS`  ·  defined at `mcp_types.methods.MONOLITH_RESULTS`

```python
MONOLITH_RESULTS: Final[Mapping[str, type[types.Result] | UnionType]] = MappingProxyType({'completion/complete': types.CompleteResult, 'elicitation/create': types.ElicitResult, 'initialize': types.InitializeResult, 'logging/setLevel': types.EmptyResult, 'ping': types.EmptyResult, 'prompts/get': types.GetPromptResult | types.InputRequiredResult, 'prompts/list': types.ListPromptsResult, 'resources/list': types.ListResourcesResult, 'resources/read': types.ReadResourceResult | types.InputRequiredResult, 'resources/subscribe': types.EmptyResult, 'resources/templates/list': types.ListResourceTemplatesResult, 'resources/unsubscribe': types.EmptyResult, 'roots/list': types.ListRootsResult, 'sampling/createMessage': types.CreateMessageResult | types.CreateMessageResultWithTools, 'server/discover': types.DiscoverResult, 'subscriptions/listen': types.SubscriptionsListenResult, 'tools/call': types.CallToolResult | types.InputRequiredResult, 'tools/list': types.ListToolsResult})
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Monolith result model (or two-arm union) per request method.


## SERVER_NOTIFICATIONS

`mcp_types.methods.SERVER_NOTIFICATIONS`

```python
SERVER_NOTIFICATIONS: Final[Mapping[tuple[str, str], type[BaseModel]]] = MappingProxyType({('notifications/cancelled', '2024-11-05'): v2025.CancelledNotification, ('notifications/message', '2024-11-05'): v2025.LoggingMessageNotification, ('notifications/progress', '2024-11-05'): v2025.ProgressNotification, ('notifications/prompts/list_changed', '2024-11-05'): v2025.PromptListChangedNotification, ('notifications/resources/list_changed', '2024-11-05'): v2025.ResourceListChangedNotification, ('notifications/resources/updated', '2024-11-05'): v2025.ResourceUpdatedNotification, ('notifications/tools/list_changed', '2024-11-05'): v2025.ToolListChangedNotification, ('notifications/cancelled', '2025-03-26'): v2025.CancelledNotification, ('notifications/message', '2025-03-26'): v2025.LoggingMessageNotification, ('notifications/progress', '2025-03-26'): v2025.ProgressNotification, ('notifications/prompts/list_changed', '2025-03-26'): v2025.PromptListChangedNotification, ('notifications/resources/list_changed', '2025-03-26'): v2025.ResourceListChangedNotification, ('notifications/resources/updated', '2025-03-26'): v2025.ResourceUpdatedNotification, ('notifications/tools/list_changed', '2025-03-26'): v2025.ToolListChangedNotification, ('notifications/cancelled', '2025-06-18'): v2025.CancelledNotification, ('notifications/message', '2025-06-18'): v2025.LoggingMessageNotification, ('notifications/progress', '2025-06-18'): v2025.ProgressNotification, ('notifications/prompts/list_changed', '2025-06-18'): v2025.PromptListChangedNotification, ('notifications/resources/list_changed', '2025-06-18'): v2025.ResourceListChangedNotification, ('notifications/resources/updated', '2025-06-18'): v2025.ResourceUpdatedNotification, ('notifications/tools/list_changed', '2025-06-18'): v2025.ToolListChangedNotification, ('notifications/cancelled', '2025-11-25'): v2025.CancelledNotification, ('notifications/elicitation/complete', '2025-11-25'): v2025.ElicitationCompleteNotification, ('notifications/message', '2025-11-25'): v2025.LoggingMessageNotification, ('notifications/progress', '2025-11-25'): v2025.ProgressNotification, ('notifications/prompts/list_changed', '2025-11-25'): v2025.PromptListChangedNotification, ('notifications/resources/list_changed', '2025-11-25'): v2025.ResourceListChangedNotification, ('notifications/resources/updated', '2025-11-25'): v2025.ResourceUpdatedNotification, ('notifications/tools/list_changed', '2025-11-25'): v2025.ToolListChangedNotification, ('notifications/cancelled', '2026-07-28'): v2026.CancelledNotification, ('notifications/message', '2026-07-28'): v2026.LoggingMessageNotification, ('notifications/progress', '2026-07-28'): v2026.ProgressNotification, ('notifications/prompts/list_changed', '2026-07-28'): v2026.PromptListChangedNotification, ('notifications/resources/list_changed', '2026-07-28'): v2026.ResourceListChangedNotification, ('notifications/resources/updated', '2026-07-28'): v2026.ResourceUpdatedNotification, ('notifications/subscriptions/acknowledged', '2026-07-28'): v2026.SubscriptionsAcknowledgedNotification, ('notifications/tools/list_changed', '2026-07-28'): v2026.ToolListChangedNotification})
```

## SERVER_REQUESTS

`mcp_types.methods.SERVER_REQUESTS`

```python
SERVER_REQUESTS: Final[Mapping[tuple[str, str], type[BaseModel]]] = MappingProxyType({('ping', '2024-11-05'): v2025.PingRequest, ('roots/list', '2024-11-05'): v2025.ListRootsRequest, ('sampling/createMessage', '2024-11-05'): v2025.CreateMessageRequest, ('ping', '2025-03-26'): v2025.PingRequest, ('roots/list', '2025-03-26'): v2025.ListRootsRequest, ('sampling/createMessage', '2025-03-26'): v2025.CreateMessageRequest, ('elicitation/create', '2025-06-18'): v2025.ElicitRequest, ('ping', '2025-06-18'): v2025.PingRequest, ('roots/list', '2025-06-18'): v2025.ListRootsRequest, ('sampling/createMessage', '2025-06-18'): v2025.CreateMessageRequest, ('elicitation/create', '2025-11-25'): v2025.ElicitRequest, ('ping', '2025-11-25'): v2025.PingRequest, ('roots/list', '2025-11-25'): v2025.ListRootsRequest, ('sampling/createMessage', '2025-11-25'): v2025.CreateMessageRequest})
```

## SERVER_RESULTS

`mcp_types.methods.SERVER_RESULTS`

```python
SERVER_RESULTS: Final[Mapping[tuple[str, str], type[BaseModel] | UnionType]] = MappingProxyType({('completion/complete', '2024-11-05'): v2025.CompleteResult, ('initialize', '2024-11-05'): v2025.InitializeResult, ('logging/setLevel', '2024-11-05'): v2025.EmptyResult, ('ping', '2024-11-05'): v2025.EmptyResult, ('prompts/get', '2024-11-05'): v2025.GetPromptResult, ('prompts/list', '2024-11-05'): v2025.ListPromptsResult, ('resources/list', '2024-11-05'): v2025.ListResourcesResult, ('resources/read', '2024-11-05'): v2025.ReadResourceResult, ('resources/subscribe', '2024-11-05'): v2025.EmptyResult, ('resources/templates/list', '2024-11-05'): v2025.ListResourceTemplatesResult, ('resources/unsubscribe', '2024-11-05'): v2025.EmptyResult, ('tools/call', '2024-11-05'): v2025.CallToolResult, ('tools/list', '2024-11-05'): v2025.ListToolsResult, ('completion/complete', '2025-03-26'): v2025.CompleteResult, ('initialize', '2025-03-26'): v2025.InitializeResult, ('logging/setLevel', '2025-03-26'): v2025.EmptyResult, ('ping', '2025-03-26'): v2025.EmptyResult, ('prompts/get', '2025-03-26'): v2025.GetPromptResult, ('prompts/list', '2025-03-26'): v2025.ListPromptsResult, ('resources/list', '2025-03-26'): v2025.ListResourcesResult, ('resources/read', '2025-03-26'): v2025.ReadResourceResult, ('resources/subscribe', '2025-03-26'): v2025.EmptyResult, ('resources/templates/list', '2025-03-26'): v2025.ListResourceTemplatesResult, ('resources/unsubscribe', '2025-03-26'): v2025.EmptyResult, ('tools/call', '2025-03-26'): v2025.CallToolResult, ('tools/list', '2025-03-26'): v2025.ListToolsResult, ('completion/complete', '2025-06-18'): v2025.CompleteResult, ('initialize', '2025-06-18'): v2025.InitializeResult, ('logging/setLevel', '2025-06-18'): v2025.EmptyResult, ('ping', '2025-06-18'): v2025.EmptyResult, ('prompts/get', '2025-06-18'): v2025.GetPromptResult, ('prompts/list', '2025-06-18'): v2025.ListPromptsResult, ('resources/list', '2025-06-18'): v2025.ListResourcesResult, ('resources/read', '2025-06-18'): v2025.ReadResourceResult, ('resources/subscribe', '2025-06-18'): v2025.EmptyResult, ('resources/templates/list', '2025-06-18'): v2025.ListResourceTemplatesResult, ('resources/unsubscribe', '2025-06-18'): v2025.EmptyResult, ('tools/call', '2025-06-18'): v2025.CallToolResult, ('tools/list', '2025-06-18'): v2025.ListToolsResult, ('completion/complete', '2025-11-25'): v2025.CompleteResult, ('initialize', '2025-11-25'): v2025.InitializeResult, ('logging/setLevel', '2025-11-25'): v2025.EmptyResult, ('ping', '2025-11-25'): v2025.EmptyResult, ('prompts/get', '2025-11-25'): v2025.GetPromptResult, ('prompts/list', '2025-11-25'): v2025.ListPromptsResult, ('resources/list', '2025-11-25'): v2025.ListResourcesResult, ('resources/read', '2025-11-25'): v2025.ReadResourceResult, ('resources/subscribe', '2025-11-25'): v2025.EmptyResult, ('resources/templates/list', '2025-11-25'): v2025.ListResourceTemplatesResult, ('resources/unsubscribe', '2025-11-25'): v2025.EmptyResult, ('tools/call', '2025-11-25'): v2025.CallToolResult, ('tools/list', '2025-11-25'): v2025.ListToolsResult, ('completion/complete', '2026-07-28'): v2026.CompleteResult, ('prompts/get', '2026-07-28'): v2026.AnyGetPromptResult, ('prompts/list', '2026-07-28'): v2026.ListPromptsResult, ('resources/list', '2026-07-28'): v2026.ListResourcesResult, ('resources/read', '2026-07-28'): v2026.AnyReadResourceResult, ('resources/templates/list', '2026-07-28'): v2026.ListResourceTemplatesResult, ('server/discover', '2026-07-28'): v2026.DiscoverResult, ('subscriptions/listen', '2026-07-28'): v2026.SubscriptionsListenResult, ('tools/call', '2026-07-28'): v2026.AnyCallToolResult, ('tools/list', '2026-07-28'): v2026.ListToolsResult})
```

Results servers send, keyed by the originating client request's (method, version).


## SPEC_CLIENT_METHODS

Import as `mcp.server.extension.SPEC_CLIENT_METHODS`  ·  defined at `mcp_types.methods.SPEC_CLIENT_METHODS`

```python
SPEC_CLIENT_METHODS: Final[frozenset[str]] = frozenset(m for m, _ in CLIENT_REQUESTS)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Spec request methods a client may send (any version); the server-side spec-method discriminator.


## SPEC_CLIENT_NOTIFICATION_METHODS

`mcp_types.methods.SPEC_CLIENT_NOTIFICATION_METHODS`

```python
SPEC_CLIENT_NOTIFICATION_METHODS: Final[frozenset[str]] = frozenset(m for m, _ in CLIENT_NOTIFICATIONS)
```

Spec notification methods a client may send (any version); the server-side spec-method discriminator.


## _MonolithT

`mcp_types.methods._MonolithT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MonolithT = TypeVar('_MonolithT')
```

## _NOTIFICATION_STUB

`mcp_types.methods._NOTIFICATION_STUB`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_NOTIFICATION_STUB: Final[Mapping[str, Any]] = MappingProxyType({'jsonrpc': '2.0'})
```

## _REQUEST_STUB

`mcp_types.methods._REQUEST_STUB`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_REQUEST_STUB: Final[Mapping[str, Any]] = MappingProxyType({'jsonrpc': '2.0', 'id': 0})
```

## __all__

`mcp_types.methods.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CACHEABLE_METHODS', 'CLIENT_NOTIFICATIONS', 'CLIENT_REQUESTS', 'CLIENT_RESULTS', 'CacheableMethod', 'INPUT_REQUIRED_METHODS', 'MONOLITH_NOTIFICATIONS', 'MONOLITH_REQUESTS', 'MONOLITH_RESULTS', 'SERVER_NOTIFICATIONS', 'SERVER_REQUESTS', 'SERVER_RESULTS', 'SPEC_CLIENT_METHODS', 'SPEC_CLIENT_NOTIFICATION_METHODS', 'is_input_required', 'parse_client_notification', 'parse_client_request', 'parse_client_result', 'parse_server_notification', 'parse_server_request', 'parse_server_result', 'serialize_server_result', 'validate_client_notification', 'validate_client_request', 'validate_client_result', 'validate_server_result']
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _adapter

`mcp_types.methods._adapter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _adapter(target: type[BaseModel] | UnionType) -> TypeAdapter[Any]
```

## _body

`mcp_types.methods._body`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _body(method: str, params: Mapping[str, Any] | None) -> dict[str, Any]
```

Build a JSON-RPC body, omitting `params` when None.


## _check_known_version

`mcp_types.methods._check_known_version`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _check_known_version(version: str) -> None
```

Raise ValueError for unknown `version` so a typo cannot silently gate every method.


## _monolith_row

`mcp_types.methods._monolith_row`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _monolith_row(monolith: Mapping[str, _MonolithT], method: str) -> _MonolithT
```

Look up `method` in `monolith`, raising RuntimeError on miss.

Not KeyError: the surface row already matched, so a miss is inconsistent
extension maps and must not be caught by the session's `except KeyError` gate.


## is_input_required

Import as `mcp.server.request_state.is_input_required`  ·  defined at `mcp_types.methods.is_input_required`

```python
def is_input_required(result: object) -> TypeGuard[types.InputRequiredResult | dict[str, Any]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

True when `result` is an `input_required` interim result, typed or wire-shaped.


## parse_client_notification

`mcp_types.methods.parse_client_notification`

```python
def parse_client_notification(method: str, version: str, params: Mapping[str, Any] | None, surface: Mapping[tuple[str, str], type[BaseModel]] = CLIENT_NOTIFICATIONS, monolith: Mapping[str, type[types.Notification[Any, Any]]] = MONOLITH_NOTIFICATIONS) -> types.Notification[Any, Any]
```

Validate a client notification against `surface`, then parse and return its `monolith` model.

Args:
    surface: `(method, version)` to wire-type map; the version-gate lookup
        and (per-schema-era) shape check run against this. Pass an extended
        map to admit custom methods.
    monolith: `method` to version-free model map; the returned instance is
        parsed from this row. Must cover every method `surface` admits.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: body fails surface or monolith validation.
    RuntimeError: surface matched but `method` has no monolith row.


## parse_client_request

`mcp_types.methods.parse_client_request`

```python
def parse_client_request(method: str, version: str, params: Mapping[str, Any] | None, surface: Mapping[tuple[str, str], type[BaseModel]] = CLIENT_REQUESTS, monolith: Mapping[str, type[types.Request[Any, Any]]] = MONOLITH_REQUESTS) -> types.Request[Any, Any]
```

Validate a client request against `surface`, then parse and return its `monolith` model.

Args:
    surface: `(method, version)` to wire-type map; the version-gate lookup
        and (per-schema-era) shape check run against this. Pass an extended
        map to admit custom methods.
    monolith: `method` to version-free model map; the returned instance is
        parsed from this row. Must cover every method `surface` admits.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface` (the version gate).
    pydantic.ValidationError: body fails surface or monolith validation.
    RuntimeError: surface matched but `method` has no monolith row.


## parse_client_result

`mcp_types.methods.parse_client_result`

```python
def parse_client_result(method: str, version: str, data: Mapping[str, Any], surface: Mapping[tuple[str, str], type[BaseModel] | UnionType] = CLIENT_RESULTS, monolith: Mapping[str, type[types.Result] | UnionType] = MONOLITH_RESULTS) -> types.Result
```

Validate a client result against `surface`, then parse and return its `monolith` model.

Args:
    surface: `(method, version)` to wire-type map; the version-gate lookup
        and (per-schema-era) shape check run against this. Pass an extended
        map to admit custom methods.
    monolith: `method` to version-free model map; the returned instance is
        parsed from this row. Must cover every method `surface` admits.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: result fails surface or monolith validation.
    RuntimeError: surface matched but `method` has no monolith row.


## parse_server_notification

`mcp_types.methods.parse_server_notification`

```python
def parse_server_notification(method: str, version: str, params: Mapping[str, Any] | None, surface: Mapping[tuple[str, str], type[BaseModel]] = SERVER_NOTIFICATIONS, monolith: Mapping[str, type[types.Notification[Any, Any]]] = MONOLITH_NOTIFICATIONS) -> types.Notification[Any, Any]
```

Validate a server notification against `surface`, then parse and return its `monolith` model.

Args:
    surface: `(method, version)` to wire-type map; the version-gate lookup
        and (per-schema-era) shape check run against this. Pass an extended
        map to admit custom methods.
    monolith: `method` to version-free model map; the returned instance is
        parsed from this row. Must cover every method `surface` admits.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: body fails surface or monolith validation.
    RuntimeError: surface matched but `method` has no monolith row.


## parse_server_request

`mcp_types.methods.parse_server_request`

```python
def parse_server_request(method: str, version: str, params: Mapping[str, Any] | None, surface: Mapping[tuple[str, str], type[BaseModel]] = SERVER_REQUESTS, monolith: Mapping[str, type[types.Request[Any, Any]]] = MONOLITH_REQUESTS) -> types.Request[Any, Any]
```

Validate a server request against `surface`, then parse and return its `monolith` model.

Args:
    surface: `(method, version)` to wire-type map; the version-gate lookup
        and (per-schema-era) shape check run against this. Pass an extended
        map to admit custom methods.
    monolith: `method` to version-free model map; the returned instance is
        parsed from this row. Must cover every method `surface` admits.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface` (the version gate).
    pydantic.ValidationError: body fails surface or monolith validation.
    RuntimeError: surface matched but `method` has no monolith row.


## parse_server_result

`mcp_types.methods.parse_server_result`

```python
def parse_server_result(method: str, version: str, data: Mapping[str, Any], surface: Mapping[tuple[str, str], type[BaseModel] | UnionType] = SERVER_RESULTS, monolith: Mapping[str, type[types.Result] | UnionType] = MONOLITH_RESULTS) -> types.Result
```

Validate a server result against `surface`, then parse and return its `monolith` model.

Args:
    surface: `(method, version)` to wire-type map; the version-gate lookup
        and (per-schema-era) shape check run against this. Pass an extended
        map to admit custom methods.
    monolith: `method` to version-free model map; the returned instance is
        parsed from this row. Must cover every method `surface` admits.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: result fails surface or monolith validation.
    RuntimeError: surface matched but `method` has no monolith row.


## serialize_server_result

`mcp_types.methods.serialize_server_result`

```python
def serialize_server_result(method: str, version: str, data: Mapping[str, Any], surface: Mapping[tuple[str, str], type[BaseModel] | UnionType] = SERVER_RESULTS) -> dict[str, Any]
```

Validate `data` against `surface` and return its surface-shaped dump.

The surface model carries `extra="ignore"`, so fields not in `version`'s
schema are dropped from the returned dict.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: result fails surface validation.


## validate_client_notification

`mcp_types.methods.validate_client_notification`

```python
def validate_client_notification(method: str, version: str, params: Mapping[str, Any] | None, surface: Mapping[tuple[str, str], type[BaseModel]] = CLIENT_NOTIFICATIONS) -> None
```

Validate a client notification against `surface` only.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: body fails surface validation.


## validate_client_request

`mcp_types.methods.validate_client_request`

```python
def validate_client_request(method: str, version: str, params: Mapping[str, Any] | None, surface: Mapping[tuple[str, str], type[BaseModel]] = CLIENT_REQUESTS) -> None
```

Validate a client request against `surface` only.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface` (the version gate).
    pydantic.ValidationError: body fails surface validation.


## validate_client_result

`mcp_types.methods.validate_client_result`

```python
def validate_client_result(method: str, version: str, data: Mapping[str, Any], surface: Mapping[tuple[str, str], type[BaseModel] | UnionType] = CLIENT_RESULTS) -> None
```

Validate a client result against `surface` only.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: result fails surface validation.


## validate_server_result

Import as `fastmcp.client.client.validate_server_result`  ·  defined at `mcp_types.methods.validate_server_result`

```python
def validate_server_result(method: str, version: str, data: Mapping[str, Any], surface: Mapping[tuple[str, str], type[BaseModel] | UnionType] = SERVER_RESULTS) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate a server result against `surface` only.

Raises:
    ValueError: `version` is not a known protocol version.
    KeyError: `(method, version)` is not in `surface`.
    pydantic.ValidationError: result fails surface validation.


