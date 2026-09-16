# `mcp.client.__main__`

Distribution: `mcp`

## logger

`mcp.client.__main__.logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
logger = logging.getLogger('client')
```

## cli

`mcp.client.__main__.cli`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def cli()
```

## main

`mcp.client.__main__.main`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def main(command_or_url: str, args: list[str], env: list[tuple[str, str]])
```

## message_handler

`mcp.client.__main__.message_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def message_handler(message: IncomingMessage) -> None
```

## run_session

`mcp.client.__main__.run_session`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def run_session(read_stream: ReadStream[SessionMessage | Exception], write_stream: WriteStream[SessionMessage], client_info: types.Implementation | None = None)
```

