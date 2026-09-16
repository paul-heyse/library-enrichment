# `fastmcp.client.sampling.handlers.google_genai`

Distribution: `fastmcp`

## __all__

`fastmcp.client.sampling.handlers.google_genai.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['GoogleGenaiSamplingHandler']
```

## GoogleGenaiSamplingHandler

`fastmcp.client.sampling.handlers.google_genai.GoogleGenaiSamplingHandler`

```python
class GoogleGenaiSamplingHandler
```

**Declared members (3)**

- `client: GoogleGenaiClient = client or GoogleGenaiClient()`  _instance-attribute_
- `default_model: str = default_model`  _instance-attribute_
- `thinking_budget: int | None = thinking_budget`  _instance-attribute_

Sampling handler that uses the Google GenAI API with tool support.

Example:
    ```python
    from google.genai import Client as GoogleGenaiClient
    from fastmcp import Client as FastMCPClient
    from fastmcp.client.sampling.handlers.google_genai import (
        GoogleGenaiSamplingHandler,
    )

    handler = GoogleGenaiSamplingHandler(
        default_model="gemini-2.0-flash",
        client=GoogleGenaiClient(),
    )

    # Answers a handshake-era server's push request and a modern server's
    # input-required round alike.
    client = FastMCPClient("https://example.com/mcp", sampling_handler=handler)
    ```


## _convert_messages_to_google_genai_content

`fastmcp.client.sampling.handlers.google_genai._convert_messages_to_google_genai_content`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_messages_to_google_genai_content(messages: Sequence[SamplingMessage]) -> list[Content]
```

Convert MCP messages to Google GenAI content.


## _convert_tool_choice_to_google_genai

`fastmcp.client.sampling.handlers.google_genai._convert_tool_choice_to_google_genai`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_tool_choice_to_google_genai(tool_choice: ToolChoice | None) -> ToolConfig
```

Convert MCP ToolChoice to Google GenAI ToolConfig.


## _convert_tool_to_google_genai

`fastmcp.client.sampling.handlers.google_genai._convert_tool_to_google_genai`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_tool_to_google_genai(tool: MCPTool) -> GoogleTool
```

Convert an MCP Tool to Google GenAI format.

We prune ``title`` fields from the schema because Gemini 2.5 Flash
produces ``MALFORMED_FUNCTION_CALL`` when Pydantic's auto-generated
title annotations are present.


## _get_candidate_from_response

`fastmcp.client.sampling.handlers.google_genai._get_candidate_from_response`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_candidate_from_response(response: GenerateContentResponse) -> Candidate
```

Extract the first candidate from a response.


## _response_to_create_message_result

`fastmcp.client.sampling.handlers.google_genai._response_to_create_message_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _response_to_create_message_result(response: GenerateContentResponse, model: str) -> CreateMessageResult
```

Convert Google GenAI response to CreateMessageResult (no tools).


## _response_to_result_with_tools

`fastmcp.client.sampling.handlers.google_genai._response_to_result_with_tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _response_to_result_with_tools(response: GenerateContentResponse, model: str) -> CreateMessageResultWithTools
```

Convert Google GenAI response to CreateMessageResultWithTools.


## _sampling_content_to_google_genai_part

`fastmcp.client.sampling.handlers.google_genai._sampling_content_to_google_genai_part`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _sampling_content_to_google_genai_part(content: TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent) -> Part
```

Convert MCP content to Google GenAI Part.


