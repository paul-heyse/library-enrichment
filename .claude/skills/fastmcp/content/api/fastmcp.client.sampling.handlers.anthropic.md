# `fastmcp.client.sampling.handlers.anthropic`

Distribution: `fastmcp`

## _ANTHROPIC_IMAGE_MEDIA_TYPES

`fastmcp.client.sampling.handlers.anthropic._ANTHROPIC_IMAGE_MEDIA_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ANTHROPIC_IMAGE_MEDIA_TYPES = frozenset({'image/jpeg', 'image/png', 'image/gif', 'image/webp'})
```

## __all__

`fastmcp.client.sampling.handlers.anthropic.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['AnthropicSamplingHandler']
```

## AnthropicSamplingHandler

`fastmcp.client.sampling.handlers.anthropic.AnthropicSamplingHandler`

```python
class AnthropicSamplingHandler
```

**Declared members (2)**

- `client: AsyncAnthropic = client or AsyncAnthropic()`  _instance-attribute_
- `default_model: ModelParam = default_model`  _instance-attribute_

Sampling handler that uses the Anthropic API.

Example:
    ```python
    from anthropic import AsyncAnthropic
    from fastmcp import Client
    from fastmcp.client.sampling.handlers.anthropic import AnthropicSamplingHandler

    handler = AnthropicSamplingHandler(
        default_model="claude-sonnet-4-5",
        client=AsyncAnthropic(),
    )

    # Answers a handshake-era server's push request and a modern server's
    # input-required round alike.
    client = Client("https://example.com/mcp", sampling_handler=handler)
    ```


## _image_content_to_anthropic_block

`fastmcp.client.sampling.handlers.anthropic._image_content_to_anthropic_block`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _image_content_to_anthropic_block(content: ImageContent) -> ImageBlockParam
```

Convert MCP ImageContent to Anthropic ImageBlockParam.


