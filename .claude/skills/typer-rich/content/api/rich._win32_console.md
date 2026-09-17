# `rich._win32_console`

Distribution: `rich`

## COORD

`rich._win32_console.COORD`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
COORD = wintypes._COORD
```

## ENABLE_VIRTUAL_TERMINAL_PROCESSING

`rich._win32_console.ENABLE_VIRTUAL_TERMINAL_PROCESSING`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ENABLE_VIRTUAL_TERMINAL_PROCESSING = 4
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## STDOUT

`rich._win32_console.STDOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDOUT = -11
```

## _FillConsoleOutputAttribute

`rich._win32_console._FillConsoleOutputAttribute`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FillConsoleOutputAttribute = windll.kernel32.FillConsoleOutputAttribute
```

## _FillConsoleOutputCharacterW

`rich._win32_console._FillConsoleOutputCharacterW`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FillConsoleOutputCharacterW = windll.kernel32.FillConsoleOutputCharacterW
```

## _GetConsoleCursorInfo

`rich._win32_console._GetConsoleCursorInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GetConsoleCursorInfo = windll.kernel32.GetConsoleCursorInfo
```

## _GetConsoleMode

`rich._win32_console._GetConsoleMode`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GetConsoleMode = windll.kernel32.GetConsoleMode
```

## _GetConsoleScreenBufferInfo

`rich._win32_console._GetConsoleScreenBufferInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GetConsoleScreenBufferInfo = windll.kernel32.GetConsoleScreenBufferInfo
```

## _GetStdHandle

`rich._win32_console._GetStdHandle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GetStdHandle = windll.kernel32.GetStdHandle
```

## _SetConsoleCursorInfo

`rich._win32_console._SetConsoleCursorInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SetConsoleCursorInfo = windll.kernel32.SetConsoleCursorInfo
```

## _SetConsoleCursorPosition

`rich._win32_console._SetConsoleCursorPosition`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SetConsoleCursorPosition = windll.kernel32.SetConsoleCursorPosition
```

## _SetConsoleTextAttribute

`rich._win32_console._SetConsoleTextAttribute`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SetConsoleTextAttribute = windll.kernel32.SetConsoleTextAttribute
```

## _SetConsoleTitle

`rich._win32_console._SetConsoleTitle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SetConsoleTitle = windll.kernel32.SetConsoleTitleW
```

## console

`rich._win32_console.console`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
console = Console()
```

## handle

`rich._win32_console.handle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
handle = GetStdHandle()
```

## heading

`rich._win32_console.heading`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
heading = Style.parse('black on green')
```

## style

`rich._win32_console.style`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
style = Style(color='black', bgcolor='red')
```

## term

`rich._win32_console.term`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
term = LegacyWindowsTerm(sys.stdout)
```

## windll

`rich._win32_console.windll`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
windll: Any = None
```

## CONSOLE_CURSOR_INFO

`rich._win32_console.CONSOLE_CURSOR_INFO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CONSOLE_CURSOR_INFO(ctypes.Structure)
```

**Bases** `ctypes.Structure`

## CONSOLE_SCREEN_BUFFER_INFO

`rich._win32_console.CONSOLE_SCREEN_BUFFER_INFO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CONSOLE_SCREEN_BUFFER_INFO(Structure)
```

**Bases** `Structure`

## LegacyWindowsError

`rich._win32_console.LegacyWindowsError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LegacyWindowsError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

## LegacyWindowsTerm

`rich._win32_console.LegacyWindowsTerm`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LegacyWindowsTerm
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (20)**

- `ANSI_TO_WINDOWS = [0, 4, 2, 6, 1, 5, 3, 7, 8, 12, 10, 14, 9, 13, 11, 15]`  _class-attribute, instance-attribute_
- `BRIGHT_BIT = 8`  _class-attribute, instance-attribute_
- `cursor_position: WindowsCoordinates`  _property_
  Returns the current position of the cursor (0-based)
- `def erase_end_of_line(self) -> None`
  Erase all content from the cursor position to the end of that line
- `def erase_line(self) -> None`
  Erase all content on the line the cursor is currently located at
- `def erase_start_of_line(self) -> None`
  Erase all content from the cursor position to the start of that line
- `flush = file.flush`  _instance-attribute_
- `def hide_cursor(self) -> None`
  Hide the cursor
- `def move_cursor_backward(self) -> None`
  Move the cursor backward a single cell. Wrap to the previous line if required.
- `def move_cursor_down(self) -> None`
  Move the cursor down a single cell
- `def move_cursor_forward(self) -> None`
  Move the cursor forward a single cell. Wrap to the next line if required.
- `def move_cursor_to(self, new_position: WindowsCoordinates) -> None`
  Set the position of the cursor
- `def move_cursor_to_column(self, column: int) -> None`
  Move cursor to the column specified by the zero-based column index, staying on the same row
- `def move_cursor_up(self) -> None`
  Move the cursor up a single cell
- `screen_size: WindowsCoordinates`  _property_
  Returns the current size of the console screen buffer, in character columns and rows
- `def set_title(self, title: str) -> None`
  Set the title of the terminal window
- `def show_cursor(self) -> None`
  Show the cursor
- `write = file.write`  _instance-attribute_
- `def write_styled(self, text: str, style: Style) -> None`
  Write styled text to the terminal.
- `def write_text(self, text: str) -> None`
  Write text directly to the terminal without any modification of styles

This class allows interaction with the legacy Windows Console API. It should only be used in the context
of environments where virtual terminal processing is not available. However, if it is used in a Windows environment,
the entire API should work.

Args:
    file (IO[str]): The file which the Windows Console API HANDLE is retrieved from, defaults to sys.stdout.


## WindowsCoordinates

`rich._win32_console.WindowsCoordinates`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class WindowsCoordinates(NamedTuple)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NamedTuple`

**Declared members (3)**

- `col: int`  _instance-attribute_
- `def from_param(cls, value: WindowsCoordinates) -> COORD`  _classmethod_
  Converts a WindowsCoordinates into a wintypes _COORD structure. This classmethod is internally called by ctypes to perform the conversion.
- `row: int`  _instance-attribute_

Coordinates in the Windows Console API are (y, x), not (x, y).
This class is intended to prevent that confusion.
Rows and columns are indexed from 0.
This class can be used in place of wintypes._COORD in arguments and argtypes.


## FillConsoleOutputAttribute

`rich._win32_console.FillConsoleOutputAttribute`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def FillConsoleOutputAttribute(std_handle: wintypes.HANDLE, attributes: int, length: int, start: WindowsCoordinates) -> int
```

Sets the character attributes for a specified number of character cells,
beginning at the specified coordinates in a screen buffer.

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.
    attributes (int): Integer value representing the foreground and background colours of the cells.
    length (int): The number of cells to set the output attribute of.
    start (WindowsCoordinates): The coordinates of the first cell whose attributes are to be set.

Returns:
    int: The number of cells whose attributes were actually set.


## FillConsoleOutputCharacter

`rich._win32_console.FillConsoleOutputCharacter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def FillConsoleOutputCharacter(std_handle: wintypes.HANDLE, char: str, length: int, start: WindowsCoordinates) -> int
```

Writes a character to the console screen buffer a specified number of times, beginning at the specified coordinates.

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.
    char (str): The character to write. Must be a string of length 1.
    length (int): The number of times to write the character.
    start (WindowsCoordinates): The coordinates to start writing at.

Returns:
    int: The number of characters written.


## GetConsoleCursorInfo

`rich._win32_console.GetConsoleCursorInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def GetConsoleCursorInfo(std_handle: wintypes.HANDLE, cursor_info: CONSOLE_CURSOR_INFO) -> bool
```

Get the cursor info - used to get cursor visibility and width

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.
    cursor_info (CONSOLE_CURSOR_INFO): CONSOLE_CURSOR_INFO ctype struct that receives information
        about the console's cursor.

Returns:
      bool: True if the function succeeds, otherwise False.


## GetConsoleMode

`rich._win32_console.GetConsoleMode`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def GetConsoleMode(std_handle: wintypes.HANDLE) -> int
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Retrieves the current input mode of a console's input buffer
or the current output mode of a console screen buffer.

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.

Raises:
    LegacyWindowsError: If any error occurs while calling the Windows console API.

Returns:
    int: Value representing the current console mode as documented at
        https://docs.microsoft.com/en-us/windows/console/getconsolemode#parameters


## GetConsoleScreenBufferInfo

`rich._win32_console.GetConsoleScreenBufferInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def GetConsoleScreenBufferInfo(std_handle: wintypes.HANDLE) -> CONSOLE_SCREEN_BUFFER_INFO
```

Retrieves information about the specified console screen buffer.

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.

Returns:
    CONSOLE_SCREEN_BUFFER_INFO: A CONSOLE_SCREEN_BUFFER_INFO ctype struct contain information about
        screen size, cursor position, colour attributes, and more.


## GetStdHandle

`rich._win32_console.GetStdHandle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def GetStdHandle(handle: int = STDOUT) -> wintypes.HANDLE
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Retrieves a handle to the specified standard device (standard input, standard output, or standard error).

Args:
    handle (int): Integer identifier for the handle. Defaults to -11 (stdout).

Returns:
    wintypes.HANDLE: The handle


## SetConsoleCursorInfo

`rich._win32_console.SetConsoleCursorInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def SetConsoleCursorInfo(std_handle: wintypes.HANDLE, cursor_info: CONSOLE_CURSOR_INFO) -> bool
```

Set the cursor info - used for adjusting cursor visibility and width

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.
    cursor_info (CONSOLE_CURSOR_INFO): CONSOLE_CURSOR_INFO ctype struct containing the new cursor info.

Returns:
      bool: True if the function succeeds, otherwise False.


## SetConsoleCursorPosition

`rich._win32_console.SetConsoleCursorPosition`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def SetConsoleCursorPosition(std_handle: wintypes.HANDLE, coords: WindowsCoordinates) -> bool
```

Set the position of the cursor in the console screen

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.
    coords (WindowsCoordinates): The coordinates to move the cursor to.

Returns:
    bool: True if the function succeeds, otherwise False.


## SetConsoleTextAttribute

`rich._win32_console.SetConsoleTextAttribute`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def SetConsoleTextAttribute(std_handle: wintypes.HANDLE, attributes: wintypes.WORD) -> bool
```

Set the colour attributes for all text written after this function is called.

Args:
    std_handle (wintypes.HANDLE): A handle to the console input buffer or the console screen buffer.
    attributes (int): Integer value representing the foreground and background colours.


Returns:
    bool: True if the attribute was set successfully, otherwise False.


## SetConsoleTitle

`rich._win32_console.SetConsoleTitle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def SetConsoleTitle(title: str) -> bool
```

Sets the title of the current console window

Args:
    title (str): The new title of the console window.

Returns:
    bool: True if the function succeeds, otherwise False.


