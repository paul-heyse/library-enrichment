# `fastmcp.utilities.openapi.formatters`

Distribution: `fastmcp`

## __all__

`fastmcp.utilities.openapi.formatters.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['format_array_parameter', 'format_deep_object_parameter', 'format_description_with_responses', 'format_json_for_description', 'generate_example_from_schema']
```

## logger

`fastmcp.utilities.openapi.formatters.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## format_array_parameter

Import as `fastmcp.utilities.openapi.format_array_parameter`  ·  defined at `fastmcp.utilities.openapi.formatters.format_array_parameter`

```python
def format_array_parameter(values: list, parameter_name: str, is_query_parameter: bool = False) -> str | list
```

**Also exported as** `fastmcp.utilities.openapi.format_array_parameter`

Format an array parameter according to OpenAPI specifications.

Args:
    values: List of values to format
    parameter_name: Name of the parameter (for error messages)
    is_query_parameter: If True, can return list for explode=True behavior

Returns:
    String (comma-separated) or list (for query params with explode=True)


## format_deep_object_parameter

Import as `fastmcp.utilities.openapi.format_deep_object_parameter`  ·  defined at `fastmcp.utilities.openapi.formatters.format_deep_object_parameter`

```python
def format_deep_object_parameter(param_value: dict, parameter_name: str) -> dict[str, str]
```

**Also exported as** `fastmcp.utilities.openapi.format_deep_object_parameter`

Format a dictionary parameter for deep-object style serialization.

According to OpenAPI 3.0 spec, deepObject style with explode=true serializes
object properties as separate query parameters with bracket notation.

For example, `{"id": "123", "type": "user"}` becomes
`param[id]=123&param[type]=user`.

Args:
    param_value: Dictionary value to format
    parameter_name: Name of the parameter

Returns:
    Dictionary with bracketed parameter names as keys


## format_description_with_responses

Import as `fastmcp.utilities.openapi.format_description_with_responses`  ·  defined at `fastmcp.utilities.openapi.formatters.format_description_with_responses`

```python
def format_description_with_responses(base_description: str, responses: dict[str, Any], parameters: list[ParameterInfo] | None = None, request_body: RequestBodyInfo | None = None) -> str
```

**Also exported as** `fastmcp.utilities.openapi.format_description_with_responses`

Formats the base description string with response, parameter, and request body information.

Args:
    base_description (str): The initial description to be formatted.
    responses (dict[str, Any]): A dictionary of response information, keyed by status code.
    parameters (list[ParameterInfo] | None, optional): A list of parameter information,
        including path and query parameters. Each parameter includes details such as name,
        location, whether it is required, and a description.
    request_body (RequestBodyInfo | None, optional): Information about the request body,
        including its description, whether it is required, and its content schema.

Returns:
    str: The formatted description string with additional details about responses, parameters,
    and the request body.


## format_json_for_description

Import as `fastmcp.utilities.openapi.format_json_for_description`  ·  defined at `fastmcp.utilities.openapi.formatters.format_json_for_description`

```python
def format_json_for_description(data: Any, indent: int = 2) -> str
```

**Also exported as** `fastmcp.utilities.openapi.format_json_for_description`

Formats Python data as a JSON string block for Markdown.


## generate_example_from_schema

Import as `fastmcp.utilities.openapi.generate_example_from_schema`  ·  defined at `fastmcp.utilities.openapi.formatters.generate_example_from_schema`

```python
def generate_example_from_schema(schema: JsonSchema | None) -> Any
```

**Also exported as** `fastmcp.utilities.openapi.generate_example_from_schema`

Generate a simple example value from a JSON schema dictionary.
Very basic implementation focusing on types.


