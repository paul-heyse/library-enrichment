# Observed behaviour

19 probes executed against the pinned capsule: 16 confirmed, 3 recorded.

Every capture below is a fact about a **construction**, not about a terminal. Each pins
`width`, `color_system`, `force_terminal` and `legacy_windows`, and runs under a replaced
environment that inherits nothing -- change any one of those and the bytes change. Quote
the construction whenever you quote the output.

`confirmed` means the probe and its control both came out as expected and differed.
`recorded` is a shape probe with nothing to falsify -- weaker, and labelled as such.
An `inconclusive` probe demonstrates nothing and is never reported as a pass.

Animated rendering is deliberately absent; see `reference.md` for what this index does
not claim.

---
## E001 — Does NO_COLOR strip colour even from an explicitly forced truecolor Console?

`E001 · render('[bold red]hi[/] plain', env={'NO_COLOR': '1'}, force_terminal=True, color_system='truecolor') · confirmed · probes/captures/E001.ansi.txt`

```text
'\x1b[1mhi\x1b[0m plain\n'
```

**Control** (`contains:31m`): `render('[bold red]hi[/] plain', force_terminal=True, color_system='truecolor')`

> [1;31mhi[0m plain

Measured: the red goes and the bold stays -- '\x1b[1;31m' becomes '\x1b[1m'. force_terminal and color_system are both explicit and neither wins. The variable is declared through rich's own `_environ` injection point rather than set on os.environ, because the probe harness isolates os.environ by design; the first draft did the latter and came out inconclusive, which is the isolation proving itself.

---

## E002 — Is an EMPTY NO_COLOR ignored rather than treated as set?

`E002 · render('[bold red]hi[/] plain', env={'NO_COLOR': ''}, force_terminal=True, color_system='truecolor') · confirmed · probes/captures/E002.ansi.txt`

```text
'\x1b[1;31mhi\x1b[0m plain\n'
```

**Control** (`absent:31m`): `render('[bold red]hi[/] plain', env={'NO_COLOR': '1'}, force_terminal=True, color_system='truecolor')`

> [1mhi[0m plain

14.0 made an empty value mean disabled, which is the opposite of the usual convention that presence is enough. Documented only in the changelog.

---

## E003 — Does TERM=dumb override a width passed explicitly to the constructor?

`E003 · Console(file=io.StringIO(), width=40, force_terminal=True, legacy_windows=False).width · confirmed · not captured`

```text
80
```

**Control** (`ok`): `Console(file=io.StringIO(), width=40, force_terminal=True, legacy_windows=False).width`

> 40

Measured: 80 with TERM=dumb, 40 without -- an explicitly passed width is NOT authoritative. This probe deliberately builds its Console raw rather than through the prelude, because the prelude's `_environ` isolation is exactly what it is testing the absence of.

---

## E004 — Does Console(_environ=...) isolate COLUMNS where the process environment does not?

`E004 · [Console(file=io.StringIO(), _environ={}).width, Console(file=io.StringIO()).width] · recorded · not captured`

```text
[80, 30]
```

A shape probe with no control: it records both numbers side by side, and the pair IS the finding -- the isolated Console falls back to 80 while the ordinary one takes 30.

---

## E005 — Does UNICODE_VERSION change how wide a character is measured to be?

`E005 · cell_len('\u231a') · confirmed · not captured`

```text
1
```

**Control** (`ok`): `cell_len('\u231a')`

> 2

Read from os.environ directly in rich/_unicode_data, so Console(_environ=...) never sees it and the loader is cached -- which makes it order-dependent as well as environment-dependent. One character changes width and every table containing it changes shape.

---

## M001 — Does Console.print silently DELETE a bracketed word that is not a style?

`M001 · render('status [ok] done') · confirmed · probes/captures/M001.txt`

```text
status  done
```

**Control** (`contains:[ok]`): `render('status [ok] done', markup=False)`

> status [ok] done

The flagship failure. The word is parsed as a style tag, resolves to nothing, and is dropped -- no exception, no warning, and no type checker involved. The control is the same string with markup off, which keeps it. Every help string in a Typer app goes through this path by default.

---

## M002 — Which bracketed forms survive markup parsing and which are eaten?

`M002 · {s: render('v ' + s + ' w').strip() for s in ['[path]', '[key=value]', '[a b]', '[x]', '[FILE]', '[1-9]', '[--flag]', '[0]']} · recorded · probes/captures/M002.txt`

```text
{'[path]': 'v  w', '[key=value]': 'v  w', '[a b]': 'v  w', '[x]': 'v  w', '[FILE]': 'v [FILE] w', '[1-9]': 'v [1-9] w', '[--flag]': 'v [--flag] w', '[0]': 'v [0] w'}
```

Measured: lowercase word-ish content is consumed; uppercase, digits and a leading double dash survive. The rule is 'parses as a style name', not 'contains a square bracket', which is why the trap is so hard to spot by eye -- [FILE] in one help string works and [path] in the next does not.

---

## M003 — Does rich.markup.escape make a bracketed word survive?

`M003 · render('v ' + escape('[path]') + ' w') · confirmed · not captured`

```text
v [path] w
```

**Control** (`absent:[path]`): `render('v [path] w')`

> v  w

The fix, and its own proof that the fix is needed.

---

## M004 — Does a stray closing tag raise rather than being dropped?

`M004 · 'MarkupError: ' + str(e)[:60] · confirmed · not captured`

```text
MarkupError: closing tag '[/]' at position 2 has nothing to close
```

**Control** (`ok`): `render('a [x] b')`

> a  b

Two different failure modes for the same class of input. An unknown tag is deleted in silence; an unbalanced close raises. So neither 'it always raises' nor 'it never raises' is safe to assume.

---

## O001 — Do typer.echo and Console.print treat the same string differently?

`O001 · b.getvalue() · confirmed · probes/captures/O001.txt`

```text
[bold]hi[/bold] [path]
```

**Control** (`absent:[path]`): `render('[bold]hi[/bold] [path]')`

> hi

The overlap row that justifies one skill rather than two. Same input, two functions an agent treats as interchangeable, and one of them deletes a word. typer.echo is literal and width-unaware; Console.print parses markup and wraps.

---

## O002 — Do typer.style and rich Style emit different bytes for the same intent?

`O002 · typer.style('hi', fg=typer.colors.RED, bold=True) · confirmed · probes/captures/O002.ansi.txt`

```text
'\x1b[31m\x1b[1mhi\x1b[0m'
```

**Control** (`ansi`): `Style(color='red', bold=True).render('hi')`

> [1;31mhi[0m

Measured: typer emits two separate SGR sequences, rich emits one combined. Identical on screen, different in a file -- which matters to anyone asserting on captured CLI output in a test.

---

## R001 — What does a Table actually look like at a pinned width?

`R001 · render(t, width=40) · confirmed · probes/captures/R001.txt`

```text
      demo      
┏━━━━━━━━┳━━━━━┓
┃ name   ┃ qty ┃
┡━━━━━━━━╇━━━━━┩
│ widget │   3 │
└────────┴─────┘
```

**Control** (`ok`): `render(t, width=40)`

> demo      
+--------------+
| name   | qty |
|--------+-----|
| widget |   3 |
+--------------+

The question a signature cannot answer. The control swaps the box style, so the capture is shown to reflect the box rather than being the only way a table can look.

---

## R002 — Does a rendered Traceback carry absolute source paths?

`R002 · render(Traceback(show_locals=False, width=60), width=60) · confirmed · probes/captures/R002.txt`

```text
╭─────────── Traceback (most recent call last) ────────────╮
│ in <module>:68                                           │
│ in boom:2                                                │
╰──────────────────────────────────────────────────────────╯
ZeroDivisionError: division by zero
```

**Control** (`contains:locals`): `render(Traceback(show_locals=True, width=60), width=60)`

> ╭─────────── Traceback (most recent call last) ────────────╮
│ in <module>:68                                           │
│ ╭─────────────────────── locals ───────────────────────╮ │
│ │    _ENV = {

_Normalised: home, path, address._

Compiled under the synthetic filename '<probe>' precisely so no real path reaches the frame headers -- rich.traceback renders absolute source paths verbatim, which is a transferability leak into content/ as well as a determinism hazard. The control turns locals on and demonstrates why Typer leaves them off: the locals panel prints the environment and the interpreter path. Those are elided by the `home` normaliser, recorded on this row.

---

## R003 — Is a disabled Progress a clean no-op, given animation is refused?

`R003 · c._probe_buffer.getvalue() · recorded · not captured`

```text
```

The one exception to refusing animated rendering. disable=True is deterministic, is what belongs in CI, and recording it means the refusal elsewhere reads as a boundary rather than an omission. A shape probe: there is nothing to falsify about emptiness.

---

## T001 — Does a help string lose bracketed text under the default rich_markup_mode?

`T001 · [l for l in cli(app, ['--help']).splitlines() if 'give a' in l] · confirmed · probes/captures/T001.txt`

```text
['│ --x                         <str>  give a  here [default: a]                 │']
```

**Control** (`contains:[path]`): `[l for l in cli(app, ['--help']).splitlines() if 'give a' in l]`

> ['  --x <str>             give a [path] here  [default: a]']

The same deletion, reached through the surface an agent actually writes. typer.Typer() defaults rich_markup_mode to 'rich', so this is the out-of-the-box behaviour, and the control shows the opt-out.

---

## T002 — How does an Annotated argument render in --help at 0.27.2?

`T002 · cli(app, ['--help']) · confirmed · probes/captures/T002.txt`

```text
                                                                                
 Usage: greet [OPTIONS] [name]                                                  
                                                                                
╭─ Arguments ──────────────────────────────────────────────────────────────────╮
│   name      <str>  who [default: World]                                      │
╰──────────────────────────────────────────────────────────────────────────────╯
╭─ Options ────────────────────────────────────────────────────────────────────╮
│ --install-completion          Install completion for the current shell.      │
│ --show-completion             Show completion for the current shell, to copy │
│                               it or customize the installation.              │
│ --help                        Show this message and exit.                    │
╰──────────────────────────────────────────────────────────────────────────────╯

```

**Control** (`contains:<int>`): `cli(app, ['--help'])`

> Usage: greet [OPTIONS] [name]

0.27.0 changed metavar printing. The control is the same app with a different annotation, which is what proves the column reflects the type rather than being a constant -- the must-come-out-the-other-way discipline applied to a rendering.

---

## V001 — Does `from rich import Console` raise at runtime, though type checkers accept it?

`V001 · type(e).__name__ + ': ' + str(e).split(' (')[0] · confirmed · not captured`

```text
ImportError: cannot import name 'Console' from 'rich'
```

**Control** (`contains:imported`): `'imported ' + Console.__name__`

> imported Console

rich/__init__.py imports Console under `if TYPE_CHECKING:` and nowhere else. Measured: `ty check` and `pyrefly check` both report zero errors on the failing line. Two static analysers certify an import Python refuses, which is this repository's thesis in one row. The message is cut at the opening parenthesis because CPython appends the absolute path of the module it searched, which is a fact about this machine rather than about rich.

---

## V002 — Does rich.__version__ exist at 15.0.0?

`V002 · 'AttributeError: ' + str(e)[:60] · confirmed · not captured`

```text
AttributeError: module 'rich' has no attribute '__version__'
```

**Control** (`contains:0.27.2`): `typer.__version__`

> 0.27.2

Removed by the lazy-loading change in 14.3.4. The control is typer, which DOES carry __version__ -- so the asymmetry between the two libraries is the finding, and importlib.metadata.version is the answer for both.

---

## V003 — Does the command Typer hands back descend from its OWN vendored Click?

`V003 · [c.__module__ + '.' + c.__name__ for c in type(cmd).__mro__] · confirmed · probes/captures/V003.txt`

```text
['typer.core.TyperCommand', 'typer._click.core.Command', 'abc.ABC', 'builtins.object']
```

**Control** (`contains:not installed`): `'installed' if importlib.util.find_spec('click') else 'click is not installed in this environment'`

> click is not installed in this environment

get_command returns a Command, so the call site reads exactly as an agent expects -- but the MRO lands in typer._click, and no Click plugin will accept that class. The control is the other half of the trap: click is not in the resolved graph at all, so an `import click` in a project resolves to some unrelated copy typer has never seen, and it still type-checks.
