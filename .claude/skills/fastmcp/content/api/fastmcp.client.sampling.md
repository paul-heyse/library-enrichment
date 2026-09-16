# `fastmcp.client.sampling`

Distribution: `fastmcp`

## SamplingHandler

Import as `fastmcp.client.client.SamplingHandler`  ·  defined at `fastmcp.client.sampling.SamplingHandler`

```python
SamplingHandler: TypeAlias = Callable[[list[SamplingMessage], SamplingParams, RequestContext[SessionT, LifespanContextT]], SamplingHandlerResult | Awaitable[SamplingHandlerResult]]
```

**Also exported as** `fastmcp.client.client.SamplingHandler`

## SamplingHandlerResult

`fastmcp.client.sampling.SamplingHandlerResult`

```python
SamplingHandlerResult: TypeAlias = str | CreateMessageResult | CreateMessageResultWithTools
```

## SessionT

`fastmcp.client.sampling.SessionT`

```python
SessionT = TypeVar('SessionT', ClientSession, ServerSession)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`fastmcp.client.sampling.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['RequestContext', 'SamplingHandler', 'SamplingHandlerResult', 'SamplingMessage', 'SamplingParams', 'create_sampling_callback']
```

## create_sampling_callback

Import as `fastmcp.client.client.create_sampling_callback`  ·  defined at `fastmcp.client.sampling.create_sampling_callback`

```python
def create_sampling_callback(sampling_handler: SamplingHandler) -> SamplingFnT
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

