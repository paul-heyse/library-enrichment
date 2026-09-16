# `fastmcp.client.sampling.handlers.openai`

Distribution: `fastmcp`

## _OPENAI_AUDIO_FORMATS

`fastmcp.client.sampling.handlers.openai._OPENAI_AUDIO_FORMATS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OPENAI_AUDIO_FORMATS: dict[str, Literal['wav', 'mp3']] = {'audio/wav': 'wav', 'audio/x-wav': 'wav', 'audio/mp3': 'mp3', 'audio/mpeg': 'mp3'}
```

## _OPENAI_IMAGE_MEDIA_TYPES

`fastmcp.client.sampling.handlers.openai._OPENAI_IMAGE_MEDIA_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OPENAI_IMAGE_MEDIA_TYPES: frozenset[str] = frozenset({'image/jpeg', 'image/png', 'image/gif', 'image/webp'})
```

## OpenAISamplingHandler

`fastmcp.client.sampling.handlers.openai.OpenAISamplingHandler`

```python
class OpenAISamplingHandler
```

**Declared members (2)**

- `client: AsyncOpenAI = client or AsyncOpenAI()`  _instance-attribute_
- `default_model: ChatModel = default_model`  _instance-attribute_

Sampling handler that uses the OpenAI API.


## _audio_content_to_openai_part

`fastmcp.client.sampling.handlers.openai._audio_content_to_openai_part`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _audio_content_to_openai_part(content: AudioContent) -> ChatCompletionContentPartInputAudioParam
```

Convert MCP AudioContent to OpenAI input_audio content part.


## _image_content_to_openai_part

`fastmcp.client.sampling.handlers.openai._image_content_to_openai_part`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _image_content_to_openai_part(content: ImageContent) -> ChatCompletionContentPartImageParam
```

Convert MCP ImageContent to OpenAI image_url content part.


