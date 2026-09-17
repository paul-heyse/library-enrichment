# Flag families

Every flag of both tools, grouped as upstream groups them. ripgrep's categories come from `rg --help`; ast-grep's are per subcommand, because its help is organised that way.

## FILTER OPTIONS

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --binary |  |  |  | Enabling this flag will cause ripgrep to search binary files. |
| rg | --follow | -L |  |  | This flag instructs ripgrep to follow symbolic links while traversing |
| rg | --glob | -g | GLOB |  | Include or exclude files and directories for searching that match the |
| rg | --glob-case-insensitive |  |  |  | Process all glob patterns given with the -g/--glob flag case |
| rg | --hidden | -. |  |  | Search hidden files and directories. |
| rg | --iglob |  | GLOB |  | Include or exclude files and directories for searching that match the |
| rg | --ignore-file |  | PATH |  | Specifies a path to one or more gitignore formatted rules files. |
| rg | --ignore-file-case-insensitive |  |  |  | Process ignore files (.gitignore, .ignore, etc.) case insensitively. |
| rg | --max-depth | -d | NUM |  | This flag limits the depth of directory traversal to NUM levels beyond |
| rg | --max-filesize |  | NUM+SUFFIX? |  | Ignore files larger than NUM in size. |
| rg | --no-ignore |  |  |  | When set, ignore files such as .gitignore, .ignore and .rgignore will |
| rg | --no-ignore-dot |  |  |  | Don't respect filter rules from .ignore or .rgignore files. |
| rg | --no-ignore-exclude |  |  |  | Don't respect filter rules from files that are manually configured for |
| rg | --no-ignore-files |  |  |  | When set, any --ignore-file flags, even ones that come after this flag, |
| rg | --no-ignore-global |  |  |  | Don't respect filter rules from ignore files that come from "global" |
| rg | --no-ignore-parent |  |  |  | When this flag is set, filter rules from ignore files found in parent |
| rg | --no-ignore-vcs |  |  |  | When given, filter rules from source control ignore files (e.g., |
| rg | --no-require-git |  |  |  | When this flag is given, source control ignore files such as .gitignore |
| rg | --one-file-system |  |  |  | When enabled, ripgrep will not cross file system boundaries relative to |
| rg | --type | -t | TYPE |  | This flag limits ripgrep to searching files matching TYPE. |
| rg | --type-add |  | TYPESPEC |  | This flag adds a new glob for a particular file type. |
| rg | --type-clear |  | TYPE |  | Clear the file type globs previously defined for TYPE. |
| rg | --type-not | -T | TYPE |  | Do not search files matching TYPE. |
| rg | --unrestricted | -u |  |  | This flag reduces the level of "smart" filtering. |

## INPUT OPTIONS

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --file | -f | PATTERNFILE |  | Search for patterns from the given file, with one pattern per line. |
| rg | --pre |  | COMMAND |  | For each input PATH, this flag causes ripgrep to search the standard |
| rg | --pre-glob |  | GLOB |  | This flag works in conjunction with the --pre flag. |
| rg | --regexp | -e | PATTERN |  | A pattern to search for. |
| rg | --search-zip | -z |  |  | This flag instructs ripgrep to search in compressed files. |

## LOGGING OPTIONS

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --debug |  |  |  | Show debug messages. |
| rg | --no-ignore-messages |  |  |  | When this flag is enabled, all error messages related to parsing ignore |
| rg | --no-messages |  |  |  | This flag suppresses some error messages. |
| rg | --stats |  |  |  | When enabled, ripgrep will print aggregate statistics about the search. |
| rg | --trace |  |  |  | Show trace messages. |

## OTHER BEHAVIORS

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --files |  |  |  | Print each file that would be searched without actually performing the |
| rg | --generate |  | KIND |  | This flag instructs ripgrep to generate some special kind of output |
| rg | --no-config |  |  |  | When set, ripgrep will never read configuration files. |
| rg | --pcre2-version |  |  |  | When this flag is present, ripgrep will print the version of PCRE2 in |
| rg | --type-list |  |  |  | Show all supported file types and their corresponding globs. |
| rg | --version | -V |  |  | This flag prints ripgrep's version. |

## OUTPUT MODES

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --count | -c |  |  | This flag suppresses normal output and shows the number of lines that |
| rg | --count-matches |  |  |  | This flag suppresses normal output and shows the number of individual |
| rg | --files-with-matches | -l |  |  | Print only the paths with at least one match and suppress match |
| rg | --files-without-match |  |  |  | Print the paths that contain zero matches and suppress match contents. |
| rg | --json |  |  |  | Enable printing results in a JSON Lines format. |

## OUTPUT OPTIONS

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --after-context | -A | NUM |  | Show NUM lines after each match. |
| rg | --before-context | -B | NUM |  | Show NUM lines before each match. |
| rg | --block-buffered |  |  |  | When enabled, ripgrep will use block buffering. |
| rg | --byte-offset | -b |  |  | Print the 0-based byte offset within the input file before each line of |
| rg | --color |  | WHEN |  | This flag controls when to use colors. |
| rg | --colors |  | COLOR_SPEC |  | This flag specifies color settings for use in the output. |
| rg | --column |  |  |  | Show column numbers (1-based). |
| rg | --context | -C | NUM |  | Show NUM lines before and after each match. |
| rg | --context-separator |  | SEPARATOR |  | The string used to separate non-contiguous context lines in the output. |
| rg | --field-context-separator |  | SEPARATOR |  | Set the field context separator. |
| rg | --field-match-separator |  | SEPARATOR |  | Set the field match separator. |
| rg | --heading |  |  |  | This flag prints the file path above clusters of matches from each file |
| rg | --help | -h |  |  | This flag prints the help output for ripgrep. |
| rg | --hostname-bin |  | COMMAND |  | This flag controls how ripgrep determines this system's hostname. |
| rg | --hyperlink-format |  | FORMAT |  | Set the format of hyperlinks to use when printing results. |
| rg | --include-zero |  |  |  | When used with -c/--count or --count-matches, this causes ripgrep to |
| rg | --line-buffered |  |  |  | When enabled, ripgrep will always use line buffering. |
| rg | --line-number | -n |  |  | Show line numbers (1-based). |
| rg | --max-columns | -M | NUM |  | When given, ripgrep will omit lines longer than this limit in bytes. |
| rg | --max-columns-preview |  |  |  | Prints a preview for lines exceeding the configured max column limit. |
| rg | --no-filename | -I |  |  | This flag instructs ripgrep to never print the file path with each |
| rg | --no-line-number | -N |  |  | Suppress line numbers. |
| rg | --null | -0 |  |  | Whenever a file path is printed, follow it with a NUL byte. |
| rg | --only-matching | -o |  |  | Print only the matched (non-empty) parts of a matching line, with each |
| rg | --passthru |  |  |  | Print both matching and non-matching lines. |
| rg | --path-separator |  | SEPARATOR |  | Set the path separator to use when printing file paths. |
| rg | --pretty | -p |  |  | This is a convenience alias for --color=always --heading --line-number. |
| rg | --quiet | -q |  |  | Do not print anything to stdout. |
| rg | --replace | -r | REPLACEMENT |  | Replaces every match with the text given when printing results. |
| rg | --sort |  | SORTBY |  | This flag enables sorting of results in ascending order. |
| rg | --sort-files |  |  |  | DEPRECATED. |
| rg | --sortr |  | SORTBY |  | This flag enables sorting of results in descending order. |
| rg | --trim |  |  |  | When set, all ASCII whitespace at the beginning of each line printed |
| rg | --vimgrep |  |  |  | This flag instructs ripgrep to print results with every match on its |
| rg | --with-filename | -H |  |  | This flag instructs ripgrep to print the file path for each matching |

## SEARCH OPTIONS

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| rg | --auto-hybrid-regex |  |  |  | DEPRECATED. |
| rg | --case-sensitive | -s |  |  | Execute the search case sensitively. |
| rg | --crlf |  |  |  | When enabled, ripgrep will treat CRLF (\r\n) as a line terminator |
| rg | --dfa-size-limit |  | NUM+SUFFIX? |  | The upper size limit of the regex DFA. |
| rg | --encoding | -E | ENCODING |  | Specify the text encoding that ripgrep will use on all files searched. |
| rg | --engine |  | ENGINE |  | Specify which regular expression engine to use. |
| rg | --fixed-strings | -F |  |  | Treat all patterns as literals instead of as regular expressions. |
| rg | --ignore-case | -i |  |  | When this flag is provided, all patterns will be searched case |
| rg | --invert-match | -v |  |  | This flag inverts matching. |
| rg | --line-regexp | -x |  |  | When enabled, ripgrep will only show matches surrounded by line |
| rg | --max-count | -m | NUM |  | Limit the number of matching lines per file searched to NUM. |
| rg | --mmap |  |  |  | When enabled, ripgrep will search using memory maps when possible. |
| rg | --multiline | -U |  |  | This flag enables searching across multiple lines. |
| rg | --multiline-dotall |  |  |  | This flag enables "dot all" mode in all regex patterns. |
| rg | --no-pcre2-unicode |  |  |  | DEPRECATED. |
| rg | --no-unicode |  |  |  | This flag disables Unicode mode for all patterns given to ripgrep. |
| rg | --null-data |  |  |  | Enabling this flag causes ripgrep to use NUL as a line terminator |
| rg | --pcre2 | -P |  |  | When this flag is present, ripgrep will use the PCRE2 regex engine |
| rg | --regex-size-limit |  | NUM+SUFFIX? |  | The size limit of the compiled regex, where the compiled regex |
| rg | --smart-case | -S |  |  | This flag instructs ripgrep to searches case insensitively if the |
| rg | --stop-on-nonmatch |  |  |  | Enabling this option will cause ripgrep to stop reading a file once it |
| rg | --text | -a |  |  | This flag instructs ripgrep to search binary files as if they were |
| rg | --threads | -j | NUM |  | This flag sets the approximate number of threads to use. |
| rg | --word-regexp | -w |  |  | When enabled, ripgrep will only show matches surrounded by word |

## ast-grep completions options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --config | -c | CONFIG_FILE |  |  |
| ast-grep | --help | -h |  |  |  |

## ast-grep lsp options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --config | -c | CONFIG_FILE |  |  |
| ast-grep | --help | -h |  |  |  |

## ast-grep new options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --config | -c | CONFIG_FILE |  | Path to ast-grep root config, default is sgconfig.yml |
| ast-grep | --help | -h |  |  | Print help (see a summary with '-h') |
| ast-grep | --lang | -l | LANG |  | The language of the item to create. |
| ast-grep | --yes | -y |  |  | Accept all default options without interactive input during creation. |

## ast-grep outline options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --color |  | WHEN | auto,always,ansi,never | Controls output color. |
| ast-grep | --config | -c | CONFIG_FILE |  | Path to ast-grep root config, default is sgconfig.yml |
| ast-grep | --follow |  |  |  | Follow symbolic links. |
| ast-grep | --globs |  | GLOBS |  | Include or exclude file paths. |
| ast-grep | --help | -h |  |  | Print help (see a summary with '-h') |
| ast-grep | --items |  | ITEMS | auto,structure,exports,imports,all | Select which top-level items to include. |
| ast-grep | --json |  | =<STYLE | pretty,stream,compact | Output outline entries in structured JSON. |
| ast-grep | --lang | -l | LANG |  | Specify the input language. |
| ast-grep | --match |  | REGEX |  | Keep only top-level items matching this regex. |
| ast-grep | --no-default-outline-rules |  |  |  | Do not load bundled outline extractor definitions |
| ast-grep | --no-ignore |  | FILE_TYPE | hidden,dot,exclude,global,parent,vcs | Do not respect hidden file system or ignore files (.gitignore, .ignore, etc.). |
| ast-grep | --outline-rules |  | FILE |  | Load additional outline extractor definitions |
| ast-grep | --pub-members |  |  |  | Display only public members in member views. |
| ast-grep | --stdin |  |  |  | Enable search code from StdIn. |
| ast-grep | --threads | -j | NUM |  | Set the approximate number of threads to use. |
| ast-grep | --type <TYPE |  |  |  | Keep only top-level items with these comma-separated symbol types. |
| ast-grep | --view |  | VIEW | auto,names,signatures,digest,expanded | Select the text presentation. |

## ast-grep run options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --after | -A | NUM |  | Show NUM lines after each match. |
| ast-grep | --before | -B | NUM |  | Show NUM lines before each match. |
| ast-grep | --color |  | WHEN | auto,always,ansi,never | Controls output color. |
| ast-grep | --config | -c | CONFIG_FILE |  | Path to ast-grep root config, default is sgconfig.yml |
| ast-grep | --context | -C | NUM |  | Show NUM lines around each match. |
| ast-grep | --debug-query |  | =<format | pattern,ast,cst,sexp | Print query pattern's tree-sitter AST. |
| ast-grep | --files-with-matches |  |  |  | Print only the paths with at least one match and suppress match contents. |
| ast-grep | --follow |  |  |  | Follow symbolic links. |
| ast-grep | --globs |  | GLOBS |  | Include or exclude file paths. |
| ast-grep | --heading |  | WHEN | auto,always,never | Controls whether to print the file name as heading. |
| ast-grep | --help | -h |  |  | Print help (see a summary with '-h') |
| ast-grep | --inspect |  | GRANULARITY | nothing,summary,entity | Inspect information for file/rule discovery and scanning. |
| ast-grep | --interactive | -i |  |  | Start interactive edit session. |
| ast-grep | --json |  | =<STYLE | pretty,stream,compact | Output matches in structured JSON. |
| ast-grep | --kind | -k | KIND |  | AST kind to match. |
| ast-grep | --lang | -l | LANG |  | The language of the pattern. |
| ast-grep | --no-ignore |  | FILE_TYPE | hidden,dot,exclude,global,parent,vcs | Do not respect hidden file system or ignore files (.gitignore, .ignore, etc.). |
| ast-grep | --pattern | -p | PATTERN |  | AST pattern to match |
| ast-grep | --rewrite | -r | FIX |  | String to replace the matched AST node |
| ast-grep | --selector |  | KIND |  | AST kind to extract sub-part of pattern to match. |
| ast-grep | --stdin |  |  |  | Enable search code from StdIn. |
| ast-grep | --strictness |  | STRICTNESS | cst,smart,ast,relaxed,signature,template | The strictness of the pattern. |
| ast-grep | --threads | -j | NUM |  | Set the approximate number of threads to use. |
| ast-grep | --update-all | -U |  |  | Apply all rewrite without confirmation if true |

## ast-grep scan options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --after | -A | NUM |  | Show NUM lines after each match. |
| ast-grep | --before | -B | NUM |  | Show NUM lines before each match. |
| ast-grep | --color |  | WHEN | auto,always,ansi,never | Controls output color. |
| ast-grep | --config | -c | CONFIG_FILE |  | Path to ast-grep root config, default is sgconfig.yml |
| ast-grep | --context | -C | NUM |  | Show NUM lines around each match. |
| ast-grep | --error |  | =<RULE_ID |  | Set rule severity to error |
| ast-grep | --files-with-matches |  |  |  | Print only the paths with at least one match and suppress match contents. |
| ast-grep | --filter |  | REGEX |  | Scan the codebase with rules with ids matching REGEX. |
| ast-grep | --follow |  |  |  | Follow symbolic links. |
| ast-grep | --format |  | FORMAT |  | Output warning/error messages in different formats. |
| ast-grep | --globs |  | GLOBS |  | Include or exclude file paths. |
| ast-grep | --help | -h |  |  | Print help (see a summary with '-h') |
| ast-grep | --hint |  | =<RULE_ID |  | Set rule severity to hint |
| ast-grep | --include-metadata |  |  |  | Include rule metadata in the json output. |
| ast-grep | --info |  | =<RULE_ID |  | Set rule severity to info |
| ast-grep | --inline-rules |  | RULE_TEXT |  | Scan the codebase with a rule defined by the provided RULE_TEXT. |
| ast-grep | --inspect |  | GRANULARITY | nothing,summary,entity | Inspect information for file/rule discovery and scanning. |
| ast-grep | --interactive | -i |  |  | Start interactive edit session. |
| ast-grep | --json |  | =<STYLE | pretty,stream,compact | Output matches in structured JSON. |
| ast-grep | --max-results |  | NUM |  | Show at most NUM results and stop running once the limit is reached. |
| ast-grep | --min-severity |  | SEVERITY |  | Set the minimum severity of rules to scan. |
| ast-grep | --no-ignore |  | FILE_TYPE | hidden,dot,exclude,global,parent,vcs | Do not respect hidden file system or ignore files (.gitignore, .ignore, etc.). |
| ast-grep | --off |  | =<RULE_ID |  | Turn off rule |
| ast-grep | --report-style |  | REPORT_STYLE | rich,medium,short | Possible values: |
| ast-grep | --rule | -r | RULE_FILE |  | Scan the codebase with the single rule located at the path RULE_FILE. |
| ast-grep | --stdin |  |  |  | Enable search code from StdIn. |
| ast-grep | --threads | -j | NUM |  | Set the approximate number of threads to use. |
| ast-grep | --update-all | -U |  |  | Apply all rewrite without confirmation if true |
| ast-grep | --warning |  | =<RULE_ID |  | Set rule severity to warning |

## ast-grep test options

| tool | long | short | arg | values | what it does |
|---|---|---|---|---|---|
| ast-grep | --color |  | WHEN | auto,always,ansi,never | Controls output color. |
| ast-grep | --config | -c | CONFIG_FILE |  | Path to ast-grep root config, default is sgconfig.yml |
| ast-grep | --filter | -f | REGEX |  | Only run rule test cases that matches REGEX |
| ast-grep | --follow |  |  |  | Follow symbolic links while searching test YAML files |
| ast-grep | --help | -h |  |  | Print help (see a summary with '-h') |
| ast-grep | --include-off |  |  |  | Include `severity:off` rules in test |
| ast-grep | --interactive | -i |  |  | Start an interactive review to update snapshots selectively |
| ast-grep | --skip-snapshot-tests |  |  |  | Only check if the test code is valid, without checking rule output. |
| ast-grep | --snapshot-dir |  | SNAPSHOT_DIR |  | Specify the directory name storing snapshots. |
| ast-grep | --test-dir | -t | TEST_DIR |  | the directories to search test YAML files |
| ast-grep | --update-all | -U |  |  | Update the content of all snapshots that have changed in test. |
