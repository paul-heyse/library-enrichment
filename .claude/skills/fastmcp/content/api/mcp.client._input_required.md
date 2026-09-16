# `mcp.client._input_required`

Distribution: `mcp`

## DEFAULT_INPUT_REQUIRED_MAX_ROUNDS

Import as `mcp.client.client.DEFAULT_INPUT_REQUIRED_MAX_ROUNDS`  ·  defined at `mcp.client._input_required.DEFAULT_INPUT_REQUIRED_MAX_ROUNDS`

```python
DEFAULT_INPUT_REQUIRED_MAX_ROUNDS = 10
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default cap on `InputRequiredResult` retry rounds before the driver gives up.

Matches the typescript-sdk default; csharp-sdk and go-sdk use the same value
as a hard constant.


## ResultT

`mcp.client._input_required.ResultT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ResultT = TypeVar('ResultT')
```

## _STATE_ONLY_BACKOFF_CAP_SECONDS

`mcp.client._input_required._STATE_ONLY_BACKOFF_CAP_SECONDS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STATE_ONLY_BACKOFF_CAP_SECONDS = 0.25
```

Upper bound on the state-only backoff sleep; reached after three consecutive state-only legs.


## _STATE_ONLY_BACKOFF_INITIAL_SECONDS

`mcp.client._input_required._STATE_ONLY_BACKOFF_INITIAL_SECONDS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STATE_ONLY_BACKOFF_INITIAL_SECONDS = 0.05
```

First sleep when an `InputRequiredResult` carries only `request_state` (no input requests).


## InputRequiredRoundsExceededError

Import as `mcp.InputRequiredRoundsExceededError`  ·  defined at `mcp.client._input_required.InputRequiredRoundsExceededError`

```python
class InputRequiredRoundsExceededError(RuntimeError)
```

**Also exported as** `mcp.InputRequiredRoundsExceededError`, `mcp.client.InputRequiredRoundsExceededError`

**Bases** `RuntimeError`

**Declared members (1)**

- `max_rounds = max_rounds`  _instance-attribute_

The server kept returning `InputRequiredResult` past the configured `max_rounds`.


## _dispatch_all

`mcp.client._input_required._dispatch_all`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _dispatch_all(requests: dict[str, InputRequest], dispatch: Callable[[str, InputRequest], Awaitable[InputResponse | ErrorData]]) -> InputResponses
```

Run `dispatch` concurrently for every key, raising `MCPError` on the first `ErrorData`.

The first task to return `ErrorData` cancels its siblings via the task
group's cancel scope, so a refused input does not wait on a slow peer.
A callback that *raises* propagates as an `ExceptionGroup` like any other
task-group failure.


## run_input_required_driver

Import as `mcp.client.client.run_input_required_driver`  ·  defined at `mcp.client._input_required.run_input_required_driver`

```python
async def run_input_required_driver(first: InputRequiredResult, dispatch: Callable[[str, InputRequest], Awaitable[InputResponse | ErrorData]], retry: Callable[[InputResponses | None, str | None], Awaitable[ResultT | InputRequiredResult]], max_rounds: int = DEFAULT_INPUT_REQUIRED_MAX_ROUNDS) -> ResultT
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve an `InputRequiredResult` to its terminal result.

Loops until `retry` returns a non-`InputRequiredResult`, or `max_rounds` is
exhausted. Each round either dispatches all `input_requests` concurrently
and retries with the collected responses, or — when the server sent only
`request_state` — sleeps with exponential backoff (50ms doubling to a 250ms
cap, reset by any leg that carries input requests) and retries empty.
`request_state` is passed through byte-exact and never inspected.

Args:
    first: The `InputRequiredResult` the original call returned.
    dispatch: Runs one embedded `InputRequest` through the client's
        sampling / elicitation / roots callbacks. Called concurrently per
        request key. An `ErrorData` return aborts the loop as an `MCPError`.
    retry: Re-issues the original request with the collected responses and
        the latest `request_state`. Each call mints a fresh JSON-RPC id.
    max_rounds: Cap on retry rounds.

Raises:
    InputRequiredRoundsExceededError: `max_rounds` exhausted.
    MCPError: A `dispatch` call returned `ErrorData`.


