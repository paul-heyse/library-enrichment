# `typer._completion_shared`

Distribution: `typer`

## COMPLETION_SCRIPT_BASH

`typer._completion_shared.COMPLETION_SCRIPT_BASH`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
COMPLETION_SCRIPT_BASH = '\n%(complete_func)s() {\n    local IFS=$\'\n\'\n    COMPREPLY=( $( env COMP_WORDS="${COMP_WORDS[*]}" \\\n                   COMP_CWORD=$COMP_CWORD \\\n                   %(autocomplete_var)s=complete_bash $1 ) )\n    return 0\n}\n\ncomplete -o default -F %(complete_func)s %(prog_name)s\n'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## COMPLETION_SCRIPT_FISH

`typer._completion_shared.COMPLETION_SCRIPT_FISH`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
COMPLETION_SCRIPT_FISH = 'complete --command %(prog_name)s --no-files --arguments "(env %(autocomplete_var)s=complete_fish _TYPER_COMPLETE_FISH_ACTION=get-args _TYPER_COMPLETE_ARGS=(commandline -cp) %(prog_name)s)" --condition "env %(autocomplete_var)s=complete_fish _TYPER_COMPLETE_FISH_ACTION=is-args _TYPER_COMPLETE_ARGS=(commandline -cp) %(prog_name)s"'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## COMPLETION_SCRIPT_POWER_SHELL

`typer._completion_shared.COMPLETION_SCRIPT_POWER_SHELL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
COMPLETION_SCRIPT_POWER_SHELL = '\nImport-Module PSReadLine\nSet-PSReadLineKeyHandler -Chord Tab -Function MenuComplete\n$scriptblock = {\n    param($wordToComplete, $commandAst, $cursorPosition)\n    $Env:%(autocomplete_var)s = "complete_powershell"\n    $Env:_TYPER_COMPLETE_ARGS = $commandAst.ToString()\n    $Env:_TYPER_COMPLETE_WORD_TO_COMPLETE = $wordToComplete\n    %(prog_name)s | ForEach-Object {\n        $commandArray = $_ -Split ":::"\n        $command = $commandArray[0]\n        $helpString = $commandArray[1]\n        [System.Management.Automation.CompletionResult]::new(\n            $command, $command, \'ParameterValue\', $helpString)\n    }\n    $Env:%(autocomplete_var)s = ""\n    $Env:_TYPER_COMPLETE_ARGS = ""\n    $Env:_TYPER_COMPLETE_WORD_TO_COMPLETE = ""\n}\nRegister-ArgumentCompleter -Native -CommandName %(prog_name)s -ScriptBlock $scriptblock\n'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## COMPLETION_SCRIPT_ZSH

`typer._completion_shared.COMPLETION_SCRIPT_ZSH`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
COMPLETION_SCRIPT_ZSH = '\n#compdef %(prog_name)s\n\n%(complete_func)s() {\n  eval $(env _TYPER_COMPLETE_ARGS="${words[1,$CURRENT]}" %(autocomplete_var)s=complete_zsh %(prog_name)s)\n}\n\ncompdef %(complete_func)s %(prog_name)s\n'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _completion_scripts

`typer._completion_shared._completion_scripts`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_completion_scripts = {'bash': COMPLETION_SCRIPT_BASH, 'zsh': COMPLETION_SCRIPT_ZSH, 'fish': COMPLETION_SCRIPT_FISH, 'powershell': COMPLETION_SCRIPT_POWER_SHELL, 'pwsh': COMPLETION_SCRIPT_POWER_SHELL}
```

## _invalid_ident_char_re

`typer._completion_shared._invalid_ident_char_re`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_invalid_ident_char_re = re.compile('[^a-zA-Z0-9_]')
```

## Shells

Import as `typer.completion.Shells`  ·  defined at `typer._completion_shared.Shells`

```python
class Shells(str, Enum)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `str`, `Enum`

**Declared members (5)**

- `bash = 'bash'`  _class-attribute, instance-attribute_
- `fish = 'fish'`  _class-attribute, instance-attribute_
- `powershell = 'powershell'`  _class-attribute, instance-attribute_
- `pwsh = 'pwsh'`  _class-attribute, instance-attribute_
- `zsh = 'zsh'`  _class-attribute, instance-attribute_

## _get_shell_name

`typer._completion_shared._get_shell_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_shell_name() -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the current shell name, if available.

The name will always be lowercase. If the shell cannot be detected, None is
returned.


## get_completion_script

Import as `typer.completion.get_completion_script`  ·  defined at `typer._completion_shared.get_completion_script`

```python
def get_completion_script(prog_name: str, complete_var: str, shell: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## install

Import as `typer.completion.install`  ·  defined at `typer._completion_shared.install`

```python
def install(shell: str | None = None, prog_name: str | None = None, complete_var: str | None = None) -> tuple[str, Path]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## install_bash

`typer._completion_shared.install_bash`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def install_bash(prog_name: str, complete_var: str, shell: str) -> Path
```

## install_fish

`typer._completion_shared.install_fish`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def install_fish(prog_name: str, complete_var: str, shell: str) -> Path
```

## install_powershell

`typer._completion_shared.install_powershell`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def install_powershell(prog_name: str, complete_var: str, shell: str) -> Path
```

## install_zsh

`typer._completion_shared.install_zsh`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def install_zsh(prog_name: str, complete_var: str, shell: str) -> Path
```

