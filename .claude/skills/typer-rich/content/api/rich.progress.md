# `rich.progress`

Distribution: `rich`

## GetTimeCallable

`rich.progress.GetTimeCallable`

```python
GetTimeCallable = Callable[[], float]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '() -> float'> ````

## ProgressType

`rich.progress.ProgressType`

```python
ProgressType = TypeVar('ProgressType')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TaskID

`rich.progress.TaskID`

```python
TaskID = NewType('TaskID', int)
```

**Inferred type** (`ty`, not declared in the source): ````xml <NewType pseudo-class 'TaskID'> ````

## _I

`rich.progress._I`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_I = typing.TypeVar('_I', TextIO, BinaryIO)
```

## console

`rich.progress.console`

```python
console = Console(record=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## examples

`rich.progress.examples`

```python
examples = cycle(progress_renderables)
```

**Inferred type** (`ty`, not declared in the source): `cycle[str | Panel | Table | ... omitted 3 union elements]`

## progress_renderables

`rich.progress.progress_renderables`

```python
progress_renderables = ['Text may be printed while the progress bars are rendering.', Panel('In fact, [i]any[/i] renderable will work'), 'Such as [magenta]tables[/]...', table, 'Pretty printed structures...', {'type': 'example', 'text': 'Pretty printed'}, 'Syntax...', syntax, Rule('Give it a try!')]
```

**Inferred type** (`ty`, not declared in the source): `list[str | Panel | Table | ... omitted 3 union elements]`

## syntax

`rich.progress.syntax`

```python
syntax = Syntax('def loop_last(values: Iterable[T]) -> Iterable[Tuple[bool, T]]:\n    """Iterate and generate a tuple with a flag for last value."""\n    iter_values = iter(values)\n    try:\n        previous_value = next(iter_values)\n    except StopIteration:\n        return\n    for value in iter_values:\n        yield False, previous_value\n        previous_value = value\n    yield True, previous_value', 'python', line_numbers=True)
```

**Inferred type** (`ty`, not declared in the source): `Syntax`

## table

`rich.progress.table`

```python
table = Table('foo', 'bar', 'baz')
```

**Inferred type** (`ty`, not declared in the source): `Table`

## task1

`rich.progress.task1`

```python
task1 = progress.add_task('[red]Downloading', total=1000)
```

**Inferred type** (`ty`, not declared in the source): `TaskID`

## task2

`rich.progress.task2`

```python
task2 = progress.add_task('[green]Processing', total=1000)
```

**Inferred type** (`ty`, not declared in the source): `TaskID`

## task3

`rich.progress.task3`

```python
task3 = progress.add_task('[yellow]Thinking', total=None)
```

**Inferred type** (`ty`, not declared in the source): `TaskID`

## BarColumn

`rich.progress.BarColumn`

```python
class BarColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (6)**

- `bar_width = bar_width`  _instance-attribute_
- `complete_style = complete_style`  _instance-attribute_
- `finished_style = finished_style`  _instance-attribute_
- `pulse_style = pulse_style`  _instance-attribute_
- `def render(self, task: 'Task') -> ProgressBar`
  Gets a progress bar widget for a task.
- `style = style`  _instance-attribute_

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders a visual progress bar.

Args:
    bar_width (Optional[int], optional): Width of bar or None for full width. Defaults to 40.
    style (StyleType, optional): Style for the bar background. Defaults to "bar.back".
    complete_style (StyleType, optional): Style for the completed bar. Defaults to "bar.complete".
    finished_style (StyleType, optional): Style for a finished bar. Defaults to "bar.finished".
    pulse_style (StyleType, optional): Style for pulsing bars. Defaults to "bar.pulse".


## DownloadColumn

`rich.progress.DownloadColumn`

```python
class DownloadColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (2)**

- `binary_units = binary_units`  _instance-attribute_
- `def render(self, task: 'Task') -> Text`
  Calculate common unit for completed and total.

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders file size downloaded and total, e.g. '0.5/2.3 GB'.

Args:
    binary_units (bool, optional): Use binary units, KiB, MiB etc. Defaults to False.


## FileSizeColumn

`rich.progress.FileSizeColumn`

```python
class FileSizeColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (1)**

- `def render(self, task: 'Task') -> Text`
  Show data completed.

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders completed filesize.


## MofNCompleteColumn

`rich.progress.MofNCompleteColumn`

```python
class MofNCompleteColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (2)**

- `def render(self, task: 'Task') -> Text`
  Show completed/total.
- `separator = separator`  _instance-attribute_

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders completed count/total, e.g. '  10/1000'.

Best for bounded tasks with int quantities.

Space pads the completed count so that progress length does not change as task progresses
past powers of 10.

Args:
    separator (str, optional): Text to separate completed and total values. Defaults to "/".


## Progress

`rich.progress.Progress`

```python
class Progress(JupyterMixin)
```

**Bases** `JupyterMixin`

**Declared members (29)**

- `def add_task(self, description: str, start: bool = True, total: Optional[float] = 100.0, completed: int = 0, visible: bool = True, fields: Any = {}) -> TaskID`
  Add a new 'task' to the Progress display.
- `def advance(self, task_id: TaskID, advance: float = 1) -> None`
  Advance task by a number of steps.
- `columns = columns or self.get_default_columns()`  _instance-attribute_
- `console: Console`  _property_
- `disable = disable`  _instance-attribute_
- `expand = expand`  _instance-attribute_
- `finished: bool`  _property_
  Check if all tasks have been completed.
- `def get_default_columns(cls) -> Tuple[ProgressColumn, ...]`  _classmethod_
  Get the default columns used for a new Progress instance:    - a text column for the description (TextColumn)    - the bar itself (BarColumn)    - a text column showing completion percentage (TextColumn)    - an estimated-time-remaining co…
- `def get_renderable(self) -> RenderableType`
  Get a renderable for the progress display.
- `def get_renderables(self) -> Iterable[RenderableType]`
  Get a number of renderables for the progress display.
- `get_time = get_time or self.console.get_time`  _instance-attribute_
- `live = Live(console=console or get_console(), auto_refresh=auto_refresh, refresh_per_second=refresh_per_second, transient=transient, redirect_stdout=redirect_stdout, redirect_stderr=redirect_stderr, get_renderable=self.get_renderable)`  _instance-attribute_
- `log = self.console.log`  _instance-attribute_
- `def make_tasks_table(self, tasks: Iterable[Task]) -> Table`
  Get a table to render the Progress display.
- `def open(self, file: Union[str, 'PathLike[str]', bytes], mode: Union[Literal['rb'], Literal['rt'], Literal['r']] = 'r', buffering: int = -1, encoding: Optional[str] = None, errors: Optional[str] = None, newline: Optional[str] = None, total: Optional[int] = None, task_id: Optional[TaskID] = None, description: str = 'Reading...') -> Union[BinaryIO, TextIO]`
  Track progress while reading from a binary file.
- `print = self.console.print`  _instance-attribute_
- `def refresh(self) -> None`
  Refresh (render) the progress information.
- `def remove_task(self, task_id: TaskID) -> None`
  Delete a task if it exists.
- `def reset(self, task_id: TaskID, start: bool = True, total: Optional[float] = None, completed: int = 0, visible: Optional[bool] = None, description: Optional[str] = None, fields: Any = {}) -> None`
  Reset a task so completed is 0 and the clock is reset.
- `speed_estimate_period = speed_estimate_period`  _instance-attribute_
- `def start(self) -> None`
  Start the progress display.
- `def start_task(self, task_id: TaskID) -> None`
  Start a task.
- `def stop(self) -> None`
  Stop the progress display.
- `def stop_task(self, task_id: TaskID) -> None`
  Stop a task.
- `task_ids: List[TaskID]`  _property_
  A list of task IDs.
- `tasks: List[Task]`  _property_
  Get a list of Task instances.
- `def track(self, sequence: Iterable[ProgressType], total: Optional[float] = None, completed: int = 0, task_id: Optional[TaskID] = None, description: str = 'Working...', update_period: float = 0.1) -> Iterable[ProgressType]`
  Track progress by iterating over a sequence.
- `def update(self, task_id: TaskID, total: Optional[float] = None, completed: Optional[float] = None, advance: Optional[float] = None, description: Optional[str] = None, visible: Optional[bool] = None, refresh: bool = False, fields: Any = {}) -> None`
  Update information associated with a task.
- `def wrap_file(self, file: BinaryIO, total: Optional[int] = None, task_id: Optional[TaskID] = None, description: str = 'Reading...') -> BinaryIO`
  Track progress file reading from a binary file.

Renders an auto-updating progress bar(s).

Args:
    console (Console, optional): Optional Console instance. Defaults to an internal Console instance writing to stdout.
    auto_refresh (bool, optional): Enable auto refresh. If disabled, you will need to call `refresh()`.
    refresh_per_second (float, optional): Number of times per second to refresh the progress information. Defaults to 10.
    speed_estimate_period: (float, optional): Period (in seconds) used to calculate the speed estimate. Defaults to 30.
    transient: (bool, optional): Clear the progress on exit. Defaults to False.
    redirect_stdout: (bool, optional): Enable redirection of stdout, so ``print`` may be used. Defaults to True.
    redirect_stderr: (bool, optional): Enable redirection of stderr. Defaults to True.
    get_time: (Callable, optional): A callable that gets the current time, or None to use Console.get_time. Defaults to None.
    disable (bool, optional): Disable progress display. Defaults to False
    expand (bool, optional): Expand tasks table to fit width. Defaults to False.


## ProgressColumn

`rich.progress.ProgressColumn`

```python
class ProgressColumn(ABC)
```

**Bases** `ABC`

**Declared members (3)**

- `def get_table_column(self) -> Column`
  Get a table column, used to build tasks table.
- `max_refresh: Optional[float] = None`  _class-attribute, instance-attribute_
- `def render(self, task: 'Task') -> RenderableType`  _abstractmethod_
  Should return a renderable object.

Base class for a widget to use in progress display.


## ProgressSample

`rich.progress.ProgressSample`

```python
class ProgressSample(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (2)**

- `completed: float`  _instance-attribute_
  Number of steps completed.
- `timestamp: float`  _instance-attribute_
  Timestamp of sample.

Sample of progress for a given time.


## RenderableColumn

`rich.progress.RenderableColumn`

```python
class RenderableColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (2)**

- `def render(self, task: 'Task') -> RenderableType`
- `renderable = renderable`  _instance-attribute_

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A column to insert an arbitrary column.

Args:
    renderable (RenderableType, optional): Any renderable. Defaults to empty string.


## SpinnerColumn

`rich.progress.SpinnerColumn`

```python
class SpinnerColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (4)**

- `finished_text = Text.from_markup(finished_text) if isinstance(finished_text, str) else finished_text`  _instance-attribute_
- `def render(self, task: 'Task') -> RenderableType`
- `def set_spinner(self, spinner_name: str, spinner_style: Optional[StyleType] = 'progress.spinner', speed: float = 1.0) -> None`
  Set a new spinner.
- `spinner = Spinner(spinner_name, style=style, speed=speed)`  _instance-attribute_

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A column with a 'spinner' animation.

Args:
    spinner_name (str, optional): Name of spinner animation. Defaults to "dots".
    style (StyleType, optional): Style of spinner. Defaults to "progress.spinner".
    speed (float, optional): Speed factor of spinner. Defaults to 1.0.
    finished_text (TextType, optional): Text used when task is finished. Defaults to " ".


## Task

`rich.progress.Task`

```python
class Task
```

**Declared members (18)**

- `completed: float`  _instance-attribute_
  float: Number of steps completed
- `description: str`  _instance-attribute_
  str: Description of the task.
- `elapsed: Optional[float]`  _property_
  Optional[float]: Time elapsed since task was started, or ``None`` if the task hasn't started.
- `fields: Dict[str, Any] = field(default_factory=dict)`  _class-attribute, instance-attribute_
  dict: Arbitrary fields passed in via Progress.update.
- `finished: bool`  _property_
  Check if the task has finished.
- `finished_speed: Optional[float] = None`  _class-attribute, instance-attribute_
  Optional[float]: The last speed for a finished task.
- `finished_time: Optional[float] = None`  _class-attribute, instance-attribute_
  float: Time task was finished.
- `def get_time(self) -> float`
  float: Get the current time, in seconds.
- `id: TaskID`  _instance-attribute_
  Task ID associated with this task (used in Progress methods).
- `percentage: float`  _property_
  float: Get progress of task as a percentage. If a None total was set, returns 0
- `remaining: Optional[float]`  _property_
  Optional[float]: Get the number of steps remaining, if a non-None total was set.
- `speed: Optional[float]`  _property_
  Optional[float]: Get the estimated speed in steps per second.
- `start_time: Optional[float] = field(default=None, init=False, repr=False)`  _class-attribute, instance-attribute_
  Optional[float]: Time this task was started, or None if not started.
- `started: bool`  _property_
  bool: Check if the task as started.
- `stop_time: Optional[float] = field(default=None, init=False, repr=False)`  _class-attribute, instance-attribute_
  Optional[float]: Time this task was stopped, or None if not stopped.
- `time_remaining: Optional[float]`  _property_
  Optional[float]: Get estimated time to completion, or ``None`` if no data.
- `total: Optional[float]`  _instance-attribute_
  Optional[float]: Total number of steps in this task.
- `visible: bool = True`  _class-attribute, instance-attribute_
  bool: Indicates if this task is visible in the progress display.

Information regarding a progress task.

This object should be considered read-only outside of the :class:`~Progress` class.


## TaskProgressColumn

`rich.progress.TaskProgressColumn`

```python
class TaskProgressColumn(TextColumn)
```

**Bases** `TextColumn`

**Declared members (4)**

- `def render(self, task: 'Task') -> Text`
- `def render_speed(cls, speed: Optional[float]) -> Text`  _classmethod_
  Render the speed in iterations per second.
- `show_speed = show_speed`  _instance-attribute_
- `text_format_no_percentage = text_format_no_percentage`  _instance-attribute_

**Inherited (7)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`
- from `rich.progress.TextColumn`: `highlighter`, `justify`, `markup`, `style`, `text_format`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Show task progress as a percentage.

Args:
    text_format (str, optional): Format for percentage display. Defaults to "[progress.percentage]{task.percentage:>3.0f}%".
    text_format_no_percentage (str, optional): Format if percentage is unknown. Defaults to "".
    style (StyleType, optional): Style of output. Defaults to "none".
    justify (JustifyMethod, optional): Text justification. Defaults to "left".
    markup (bool, optional): Enable markup. Defaults to True.
    highlighter (Optional[Highlighter], optional): Highlighter to apply to output. Defaults to None.
    table_column (Optional[Column], optional): Table Column to use. Defaults to None.
    show_speed (bool, optional): Show speed if total is unknown. Defaults to False.


## TextColumn

`rich.progress.TextColumn`

```python
class TextColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (6)**

- `highlighter = highlighter`  _instance-attribute_
- `justify: JustifyMethod = justify`  _instance-attribute_
- `markup = markup`  _instance-attribute_
- `def render(self, task: 'Task') -> Text`
- `style = style`  _instance-attribute_
- `text_format = text_format`  _instance-attribute_

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A column containing text.


## TimeElapsedColumn

`rich.progress.TimeElapsedColumn`

```python
class TimeElapsedColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (1)**

- `def render(self, task: 'Task') -> Text`
  Show time elapsed.

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders time elapsed.


## TimeRemainingColumn

`rich.progress.TimeRemainingColumn`

```python
class TimeRemainingColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (4)**

- `compact = compact`  _instance-attribute_
- `elapsed_when_finished = elapsed_when_finished`  _instance-attribute_
- `max_refresh = 0.5`  _class-attribute, instance-attribute_
- `def render(self, task: 'Task') -> Text`
  Show time remaining.

**Inherited (1)**

- from `rich.progress.ProgressColumn`: `get_table_column`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders estimated time remaining.

Args:
    compact (bool, optional): Render MM:SS when time remaining is less than an hour. Defaults to False.
    elapsed_when_finished (bool, optional): Render time elapsed when the task is finished. Defaults to False.


## TotalFileSizeColumn

`rich.progress.TotalFileSizeColumn`

```python
class TotalFileSizeColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (1)**

- `def render(self, task: 'Task') -> Text`
  Show data completed.

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders total filesize.


## TransferSpeedColumn

`rich.progress.TransferSpeedColumn`

```python
class TransferSpeedColumn(ProgressColumn)
```

**Bases** `ProgressColumn`

**Declared members (1)**

- `def render(self, task: 'Task') -> Text`
  Show data transfer speed.

**Inherited (2)**

- from `rich.progress.ProgressColumn`: `get_table_column`, `max_refresh`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders human readable transfer speed.


## _ReadContext

`rich.progress._ReadContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ReadContext(ContextManager[_I], Generic[_I])
```

**Bases** `ContextManager[_I]`, `Generic[_I]`

**Declared members (2)**

- `progress = progress`  _instance-attribute_
- `reader: _I = reader`  _instance-attribute_

A utility class to handle a context for both a reader and a progress.


## _Reader

`rich.progress._Reader`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Reader(RawIOBase, BinaryIO)
```

**Bases** `RawIOBase`, `BinaryIO`

**Declared members (21)**

- `def close(self) -> None`
- `close_handle = close_handle`  _instance-attribute_
- `closed: bool`  _property_
- `def fileno(self) -> int`
- `handle = handle`  _instance-attribute_
- `def isatty(self) -> bool`
- `mode: str`  _property_
- `name: str`  _property_
- `progress = progress`  _instance-attribute_
- `def read(self, size: int = -1) -> bytes`
- `def readable(self) -> bool`
- `def readinto(self, b: Union[bytearray, memoryview, mmap])`
- `def readline(self, size: int = -1) -> bytes`
- `def readlines(self, hint: int = -1) -> List[bytes]`
- `def seek(self, offset: int, whence: int = 0) -> int`
- `def seekable(self) -> bool`
- `task = task`  _instance-attribute_
- `def tell(self) -> int`
- `def writable(self) -> bool`
- `def write(self, s: Any) -> int`
- `def writelines(self, lines: Iterable[Any]) -> None`

A reader that tracks progress while it's being read from.


## _TrackThread

`rich.progress._TrackThread`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _TrackThread(Thread)
```

**Bases** `Thread`

**Declared members (6)**

- `completed = 0`  _instance-attribute_
- `done = Event()`  _instance-attribute_
- `progress = progress`  _instance-attribute_
- `def run(self) -> None`
- `task_id = task_id`  _instance-attribute_
- `update_period = update_period`  _instance-attribute_

A thread to periodically update progress.


## open

`rich.progress.open`

```python
def open(file: Union[str, 'PathLike[str]', bytes], mode: Union[Literal['rb'], Literal['rt'], Literal['r']] = 'r', buffering: int = -1, encoding: Optional[str] = None, errors: Optional[str] = None, newline: Optional[str] = None, total: Optional[int] = None, description: str = 'Reading...', auto_refresh: bool = True, console: Optional[Console] = None, transient: bool = False, get_time: Optional[Callable[[], float]] = None, refresh_per_second: float = 10, style: StyleType = 'bar.back', complete_style: StyleType = 'bar.complete', finished_style: StyleType = 'bar.finished', pulse_style: StyleType = 'bar.pulse', disable: bool = False) -> Union[ContextManager[BinaryIO], ContextManager[TextIO]]
```

**Overloads** (the signature above is the runtime dispatcher):

- `def open(file: Union[str, 'PathLike[str]', bytes], mode: Union[Literal['rt'], Literal['r']], buffering: int = -1, encoding: Optional[str] = None, errors: Optional[str] = None, newline: Optional[str] = None, total: Optional[int] = None, description: str = 'Reading...', auto_refresh: bool = True, console: Optional[Console] = None, transient: bool = False, get_time: Optional[Callable[[], float]] = None, refresh_per_second: float = 10, style: StyleType = 'bar.back', complete_style: StyleType = 'bar.complete', finished_style: StyleType = 'bar.finished', pulse_style: StyleType = 'bar.pulse', disable: bool = False) -> ContextManager[TextIO]`
- `def open(file: Union[str, 'PathLike[str]', bytes], mode: Literal['rb'], buffering: int = -1, encoding: Optional[str] = None, errors: Optional[str] = None, newline: Optional[str] = None, total: Optional[int] = None, description: str = 'Reading...', auto_refresh: bool = True, console: Optional[Console] = None, transient: bool = False, get_time: Optional[Callable[[], float]] = None, refresh_per_second: float = 10, style: StyleType = 'bar.back', complete_style: StyleType = 'bar.complete', finished_style: StyleType = 'bar.finished', pulse_style: StyleType = 'bar.pulse', disable: bool = False) -> ContextManager[BinaryIO]`

Read bytes from a file while tracking progress.

Args:
    path (Union[str, PathLike[str], BinaryIO]): The path to the file to read, or a file-like object in binary mode.
    mode (str): The mode to use to open the file. Only supports "r", "rb" or "rt".
    buffering (int): The buffering strategy to use, see :func:`io.open`.
    encoding (str, optional): The encoding to use when reading in text mode, see :func:`io.open`.
    errors (str, optional): The error handling strategy for decoding errors, see :func:`io.open`.
    newline (str, optional): The strategy for handling newlines in text mode, see :func:`io.open`
    total: (int, optional): Total number of bytes to read. Must be provided if reading from a file handle. Default for a path is os.stat(file).st_size.
    description (str, optional): Description of task show next to progress bar. Defaults to "Reading".
    auto_refresh (bool, optional): Automatic refresh, disable to force a refresh after each iteration. Default is True.
    transient: (bool, optional): Clear the progress on exit. Defaults to False.
    console (Console, optional): Console to write to. Default creates internal Console instance.
    refresh_per_second (float): Number of times per second to refresh the progress information. Defaults to 10.
    style (StyleType, optional): Style for the bar background. Defaults to "bar.back".
    complete_style (StyleType, optional): Style for the completed bar. Defaults to "bar.complete".
    finished_style (StyleType, optional): Style for a finished bar. Defaults to "bar.finished".
    pulse_style (StyleType, optional): Style for pulsing bars. Defaults to "bar.pulse".
    disable (bool, optional): Disable display of progress.
    encoding (str, optional): The encoding to use when reading in text mode.

Returns:
    ContextManager[BinaryIO]: A context manager yielding a progress reader.


## track

`rich.progress.track`

```python
def track(sequence: Iterable[ProgressType], description: str = 'Working...', total: Optional[float] = None, completed: int = 0, auto_refresh: bool = True, console: Optional[Console] = None, transient: bool = False, get_time: Optional[Callable[[], float]] = None, refresh_per_second: float = 10, style: StyleType = 'bar.back', complete_style: StyleType = 'bar.complete', finished_style: StyleType = 'bar.finished', pulse_style: StyleType = 'bar.pulse', update_period: float = 0.1, disable: bool = False, show_speed: bool = True) -> Iterable[ProgressType]
```

Track progress by iterating over a sequence.

You can also track progress of an iterable, which might require that you additionally specify ``total``.

Args:
    sequence (Iterable[ProgressType]): Values you wish to iterate over and track progress.
    description (str, optional): Description of task show next to progress bar. Defaults to "Working".
    total: (float, optional): Total number of steps. Default is len(sequence).
    completed (int, optional): Number of steps completed so far. Defaults to 0.
    auto_refresh (bool, optional): Automatic refresh, disable to force a refresh after each iteration. Default is True.
    transient: (bool, optional): Clear the progress on exit. Defaults to False.
    console (Console, optional): Console to write to. Default creates internal Console instance.
    refresh_per_second (float): Number of times per second to refresh the progress information. Defaults to 10.
    style (StyleType, optional): Style for the bar background. Defaults to "bar.back".
    complete_style (StyleType, optional): Style for the completed bar. Defaults to "bar.complete".
    finished_style (StyleType, optional): Style for a finished bar. Defaults to "bar.finished".
    pulse_style (StyleType, optional): Style for pulsing bars. Defaults to "bar.pulse".
    update_period (float, optional): Minimum time (in seconds) between calls to update(). Defaults to 0.1.
    disable (bool, optional): Disable display of progress.
    show_speed (bool, optional): Show speed if total isn't known. Defaults to True.
Returns:
    Iterable[ProgressType]: An iterable of the values in the sequence.


## wrap_file

`rich.progress.wrap_file`

```python
def wrap_file(file: BinaryIO, total: int, description: str = 'Reading...', auto_refresh: bool = True, console: Optional[Console] = None, transient: bool = False, get_time: Optional[Callable[[], float]] = None, refresh_per_second: float = 10, style: StyleType = 'bar.back', complete_style: StyleType = 'bar.complete', finished_style: StyleType = 'bar.finished', pulse_style: StyleType = 'bar.pulse', disable: bool = False) -> ContextManager[BinaryIO]
```

Read bytes from a file while tracking progress.

Args:
    file (Union[str, PathLike[str], BinaryIO]): The path to the file to read, or a file-like object in binary mode.
    total (int): Total number of bytes to read.
    description (str, optional): Description of task show next to progress bar. Defaults to "Reading".
    auto_refresh (bool, optional): Automatic refresh, disable to force a refresh after each iteration. Default is True.
    transient: (bool, optional): Clear the progress on exit. Defaults to False.
    console (Console, optional): Console to write to. Default creates internal Console instance.
    refresh_per_second (float): Number of times per second to refresh the progress information. Defaults to 10.
    style (StyleType, optional): Style for the bar background. Defaults to "bar.back".
    complete_style (StyleType, optional): Style for the completed bar. Defaults to "bar.complete".
    finished_style (StyleType, optional): Style for a finished bar. Defaults to "bar.finished".
    pulse_style (StyleType, optional): Style for pulsing bars. Defaults to "bar.pulse".
    disable (bool, optional): Disable display of progress.
Returns:
    ContextManager[BinaryIO]: A context manager yielding a progress reader.


