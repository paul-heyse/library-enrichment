# `mcp.shared.subscriptions`

Distribution: `mcp`

## LISTEN_STREAM_METHODS

Import as `mcp.server.connection.LISTEN_STREAM_METHODS`  ·  defined at `mcp.shared.subscriptions.LISTEN_STREAM_METHODS`

```python
LISTEN_STREAM_METHODS: frozenset[str] = frozenset({*_LIST_CHANGED_EVENTS, 'notifications/resources/updated'})
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The notification methods that ride `subscriptions/listen` streams at 2026-07-28
(and, at that era, nowhere else): the change-notification vocabulary.


## SUBSCRIPTION_ID_META_KEY

Import as `mcp.server.subscriptions.SUBSCRIPTION_ID_META_KEY`  ·  defined at `mcp.shared.subscriptions.SUBSCRIPTION_ID_META_KEY`

```python
SUBSCRIPTION_ID_META_KEY = 'io.modelcontextprotocol/subscriptionId'
```

**Inferred type** (`ty`, not declared in the source): `Literal["io.modelcontextprotocol/subscriptionId"]`

**Also exported as** `mcp.server.subscriptions.SUBSCRIPTION_ID_META_KEY`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The `_meta` key on every listen-stream frame; the value is the `subscriptions/listen` request's JSON-RPC id.


## ServerEvent

Import as `mcp.client.subscriptions.ServerEvent`  ·  defined at `mcp.shared.subscriptions.ServerEvent`

```python
ServerEvent = ToolsListChanged | PromptsListChanged | ResourcesListChanged | ResourceUpdated
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'ToolsListChanged | PromptsListChanged | ResourcesListChanged | ResourceUpdated'> ``` --- An event a server publishes for delivery to listen subscribers.`

**Also exported as** `mcp.client.subscriptions.ServerEvent`, `mcp.server.subscriptions.ServerEvent`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

An event a server publishes for delivery to listen subscribers.


## _LIST_CHANGED_EVENTS

`mcp.shared.subscriptions._LIST_CHANGED_EVENTS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LIST_CHANGED_EVENTS: dict[str, ServerEvent] = {'notifications/tools/list_changed': ToolsListChanged(), 'notifications/prompts/list_changed': PromptsListChanged(), 'notifications/resources/list_changed': ResourcesListChanged()}
```

## __all__

`mcp.shared.subscriptions.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['LISTEN_STREAM_METHODS', 'SUBSCRIPTION_ID_META_KEY', 'PromptsListChanged', 'ResourceUpdated', 'ResourcesListChanged', 'ServerEvent', 'ToolsListChanged', 'event_from_wire', 'event_matches', 'event_to_notification']
```

## PromptsListChanged

Import as `mcp.client.subscriptions.PromptsListChanged`  ·  defined at `mcp.shared.subscriptions.PromptsListChanged`

```python
class PromptsListChanged
```

**Also exported as** `mcp.client.subscriptions.PromptsListChanged`, `mcp.server.subscriptions.PromptsListChanged`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The server's prompt list changed.


## ResourceUpdated

Import as `mcp.client.subscriptions.ResourceUpdated`  ·  defined at `mcp.shared.subscriptions.ResourceUpdated`

```python
class ResourceUpdated
```

**Also exported as** `mcp.client.subscriptions.ResourceUpdated`, `mcp.server.subscriptions.ResourceUpdated`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `uri: str`  _instance-attribute_

The resource at `uri` changed and may need to be read again.


## ResourcesListChanged

Import as `mcp.client.subscriptions.ResourcesListChanged`  ·  defined at `mcp.shared.subscriptions.ResourcesListChanged`

```python
class ResourcesListChanged
```

**Also exported as** `mcp.client.subscriptions.ResourcesListChanged`, `mcp.server.subscriptions.ResourcesListChanged`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The server's resource list changed.


## ToolsListChanged

Import as `mcp.client.subscriptions.ToolsListChanged`  ·  defined at `mcp.shared.subscriptions.ToolsListChanged`

```python
class ToolsListChanged
```

**Also exported as** `mcp.client.subscriptions.ToolsListChanged`, `mcp.server.subscriptions.ToolsListChanged`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The server's tool list changed.


## event_from_wire

Import as `mcp.client.session.event_from_wire`  ·  defined at `mcp.shared.subscriptions.event_from_wire`

```python
def event_from_wire(method: str, params: Mapping[str, Any] | None) -> ServerEvent | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The event a raw listen-stream frame announces, or None if it carries none.

Takes the raw wire dict: the client demultiplexes before the typed notification parse.


## event_matches

Import as `mcp.client.subscriptions.event_matches`  ·  defined at `mcp.shared.subscriptions.event_matches`

```python
def event_matches(honored: SubscriptionFilter, uris: frozenset[str], event: ServerEvent) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Whether `event` is within the stream's honored filter (`uris`: the honored resource subscriptions as a set).

The admission predicate both sides share: server delivery and client intake honor only what was acknowledged.


## event_to_notification

Import as `mcp.client.client.event_to_notification`  ·  defined at `mcp.shared.subscriptions.event_to_notification`

```python
def event_to_notification(event: ServerEvent, meta: dict[str, Any]) -> ServerNotification
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build the stamped wire notification for `event` (the server's direction).


