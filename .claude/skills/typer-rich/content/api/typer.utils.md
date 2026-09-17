# `typer.utils`

Distribution: `typer`

## AnnotatedParamWithDefaultValueError

`typer.utils.AnnotatedParamWithDefaultValueError`

```python
class AnnotatedParamWithDefaultValueError(Exception)
```

**Bases** `Exception`

**Declared members (2)**

- `argument_name: str = argument_name`  _instance-attribute_
- `param_type: type[ParameterInfo] = param_type`  _instance-attribute_

## DefaultFactoryAndDefaultValueError

`typer.utils.DefaultFactoryAndDefaultValueError`

```python
class DefaultFactoryAndDefaultValueError(Exception)
```

**Bases** `Exception`

**Declared members (2)**

- `argument_name: str = argument_name`  _instance-attribute_
- `param_type: type[ParameterInfo] = param_type`  _instance-attribute_

## MixedAnnotatedAndDefaultStyleError

`typer.utils.MixedAnnotatedAndDefaultStyleError`

```python
class MixedAnnotatedAndDefaultStyleError(Exception)
```

**Bases** `Exception`

**Declared members (3)**

- `annotated_param_type: type[ParameterInfo] = annotated_param_type`  _instance-attribute_
- `argument_name: str = argument_name`  _instance-attribute_
- `default_param_type: type[ParameterInfo] = default_param_type`  _instance-attribute_

## MultipleTyperAnnotationsError

`typer.utils.MultipleTyperAnnotationsError`

```python
class MultipleTyperAnnotationsError(Exception)
```

**Bases** `Exception`

**Declared members (1)**

- `argument_name: str = argument_name`  _instance-attribute_

## _param_type_to_user_string

`typer.utils._param_type_to_user_string`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _param_type_to_user_string(param_type: type[ParameterInfo]) -> str
```

## _split_annotation_from_typer_annotations

`typer.utils._split_annotation_from_typer_annotations`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _split_annotation_from_typer_annotations(base_annotation: type[Any]) -> tuple[type[Any], list[ParameterInfo]]
```

## get_params_from_function

`typer.utils.get_params_from_function`

```python
def get_params_from_function(func: Callable[..., Any]) -> dict[str, ParamMeta]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## parse_boolean_env_var

`typer.utils.parse_boolean_env_var`

```python
def parse_boolean_env_var(env_var_value: str | None, default: bool) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

