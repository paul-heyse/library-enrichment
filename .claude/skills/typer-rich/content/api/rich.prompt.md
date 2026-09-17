# `rich.prompt`

Distribution: `rich`

## DefaultType

`rich.prompt.DefaultType`

```python
DefaultType = TypeVar('DefaultType')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## PromptType

`rich.prompt.PromptType`

```python
PromptType = TypeVar('PromptType')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## doggie

`rich.prompt.doggie`

```python
doggie = Prompt.ask("What's the best Dog? (Case INSENSITIVE)", choices=['Border Terrier', 'Collie', 'Labradoodle'], case_sensitive=False)
```

**Inferred type** (`ty`, not declared in the source): `str`

## fruit

`rich.prompt.fruit`

```python
fruit = Prompt.ask('Enter a fruit', choices=['apple', 'orange', 'pear'])
```

**Inferred type** (`ty`, not declared in the source): `str`

## password

`rich.prompt.password`

```python
password = Prompt.ask('Please enter a password [cyan](must be at least 5 characters)', password=True)
```

**Inferred type** (`ty`, not declared in the source): `str`

## result

`rich.prompt.result`

```python
result = IntPrompt.ask(':rocket: Enter a number between [b]1[/b] and [b]10[/b]', default=5)
```

**Inferred type** (`ty`, not declared in the source): `int`

## Confirm

`rich.prompt.Confirm`

```python
class Confirm(PromptBase[bool])
```

**Bases** `PromptBase[bool]`

**Declared members (5)**

- `choices: List[str] = ['y', 'n']`  _class-attribute, instance-attribute_
- `def process_response(self, value: str) -> bool`
  Convert choices to a bool.
- `def render_default(self, default: DefaultType) -> Text`
  Render the default as (y) or (n) rather than True/False.
- `response_type = bool`  _class-attribute, instance-attribute_
- `validate_error_message = '[prompt.invalid]Please enter Y or N'`  _class-attribute, instance-attribute_

**Inherited (14)**

- from `rich.prompt.PromptBase`: `ask`, `case_sensitive`, `check_choice`, `console`, `get_input`, `illegal_choice_message`, `make_prompt`, `on_validate_error`, `password`, `pre_prompt`, `prompt`, `prompt_suffix`, `show_choices`, `show_default`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A yes / no confirmation prompt.

Example:
    >>> if Confirm.ask("Continue"):
            run_job()


## FloatPrompt

`rich.prompt.FloatPrompt`

```python
class FloatPrompt(PromptBase[float])
```

**Bases** `PromptBase[float]`

**Declared members (2)**

- `response_type = float`  _class-attribute, instance-attribute_
- `validate_error_message = '[prompt.invalid]Please enter a number'`  _class-attribute, instance-attribute_

**Inherited (17)**

- from `rich.prompt.PromptBase`: `ask`, `case_sensitive`, `check_choice`, `choices`, `console`, `get_input`, `illegal_choice_message`, `make_prompt`, `on_validate_error`, `password`, `pre_prompt`, `process_response`, `prompt`, `prompt_suffix`, `render_default`, `show_choices`, `show_default`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A prompt that returns a float.

Example:
    >>> temperature = FloatPrompt.ask("Enter desired temperature")


## IntPrompt

`rich.prompt.IntPrompt`

```python
class IntPrompt(PromptBase[int])
```

**Bases** `PromptBase[int]`

**Declared members (2)**

- `response_type = int`  _class-attribute, instance-attribute_
- `validate_error_message = '[prompt.invalid]Please enter a valid integer number'`  _class-attribute, instance-attribute_

**Inherited (17)**

- from `rich.prompt.PromptBase`: `ask`, `case_sensitive`, `check_choice`, `choices`, `console`, `get_input`, `illegal_choice_message`, `make_prompt`, `on_validate_error`, `password`, `pre_prompt`, `process_response`, `prompt`, `prompt_suffix`, `render_default`, `show_choices`, `show_default`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A prompt that returns an integer.

Example:
    >>> burrito_count = IntPrompt.ask("How many burritos do you want to order")


## InvalidResponse

`rich.prompt.InvalidResponse`

```python
class InvalidResponse(PromptError)
```

**Bases** `PromptError`

**Declared members (1)**

- `message = message`  _instance-attribute_

Exception to indicate a response was invalid. Raise this within process_response() to indicate an error
and provide an error message.

Args:
    message (Union[str, Text]): Error message.


## Prompt

`rich.prompt.Prompt`

```python
class Prompt(PromptBase[str])
```

**Bases** `PromptBase[str]`

**Declared members (1)**

- `response_type = str`  _class-attribute, instance-attribute_

**Inherited (18)**

- from `rich.prompt.PromptBase`: `ask`, `case_sensitive`, `check_choice`, `choices`, `console`, `get_input`, `illegal_choice_message`, `make_prompt`, `on_validate_error`, `password`, `pre_prompt`, `process_response`, `prompt`, `prompt_suffix`, `render_default`, `show_choices`, `show_default`, `validate_error_message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A prompt that returns a str.

Example:
    >>> name = Prompt.ask("Enter your name")


## PromptBase

`rich.prompt.PromptBase`

```python
class PromptBase(Generic[PromptType])
```

**Bases** `Generic[PromptType]`

**Declared members (19)**

- `def ask(cls, prompt: TextType = '', console: Optional[Console] = None, password: bool = False, choices: Optional[List[str]] = None, case_sensitive: bool = True, show_default: bool = True, show_choices: bool = True, default: Any = ..., stream: Optional[TextIO] = None) -> Any`  _classmethod_
  Shortcut to construct and run a prompt loop and return the result.
- `case_sensitive = case_sensitive`  _instance-attribute_
- `def check_choice(self, value: str) -> bool`
  Check value is in the list of valid choices.
- `choices: Optional[List[str]] = None`  _class-attribute, instance-attribute_
- `console = console or get_console()`  _instance-attribute_
- `def get_input(cls, console: Console, prompt: TextType, password: bool, stream: Optional[TextIO] = None) -> str`  _classmethod_
  Get input from user.
- `illegal_choice_message = '[prompt.invalid.choice]Please select one of the available options'`  _class-attribute, instance-attribute_
- `def make_prompt(self, default: DefaultType) -> Text`
  Make prompt text.
- `def on_validate_error(self, value: str, error: InvalidResponse) -> None`
  Called to handle validation error.
- `password = password`  _instance-attribute_
- `def pre_prompt(self) -> None`
  Hook to display something before the prompt.
- `def process_response(self, value: str) -> PromptType`
  Process response from user, convert to prompt type.
- `prompt = Text.from_markup(prompt, style='prompt') if isinstance(prompt, str) else prompt`  _instance-attribute_
- `prompt_suffix = ': '`  _class-attribute, instance-attribute_
- `def render_default(self, default: DefaultType) -> Text`
  Turn the supplied default in to a Text instance.
- `response_type: type = str`  _class-attribute, instance-attribute_
- `show_choices = show_choices`  _instance-attribute_
- `show_default = show_default`  _instance-attribute_
- `validate_error_message = '[prompt.invalid]Please enter a valid value'`  _class-attribute, instance-attribute_

Ask the user for input until a valid response is received. This is the base class, see one of
the concrete classes for examples.

Args:
    prompt (TextType, optional): Prompt text. Defaults to "".
    console (Console, optional): A Console instance or None to use global console. Defaults to None.
    password (bool, optional): Enable password input. Defaults to False.
    choices (List[str], optional): A list of valid choices. Defaults to None.
    case_sensitive (bool, optional): Matching of choices should be case-sensitive. Defaults to True.
    show_default (bool, optional): Show default in prompt. Defaults to True.
    show_choices (bool, optional): Show choices in prompt. Defaults to True.


## PromptError

`rich.prompt.PromptError`

```python
class PromptError(Exception)
```

**Bases** `Exception`

Exception base class for prompt related errors.


