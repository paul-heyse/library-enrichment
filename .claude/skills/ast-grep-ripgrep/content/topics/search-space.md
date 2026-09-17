# Deciding which files get searched

**Why was this file skipped, and how do I control the file set?**

Both tools decide *which files to look at* before they decide *what matches*, and nearly every surprising empty result comes from the first decision rather than the second. The file set is the first thing to check when a search returns less than expected -- not the pattern.

The ignore machinery is a stack of independent layers, not one switch: VCS ignore files, `.ignore` files, parent-directory ignores, global git excludes, repository-local excludes, and the hidden-file rule. Turning one off leaves the others on. Probe P023 is the case worth internalising: `config/` is hidden by `.ignore`, so `--no-ignore-vcs` does not reveal it, and a search that looks thorough is quietly incomplete. Paths named explicitly on the command line bypass the stack entirely (P025), which is why testing a behaviour on one named file and generalising to a recursive search gets binary handling exactly backwards (P029).

## Flags

Every flag upstream files under FILTER OPTIONS. The grouping is ripgrep's own, taken from `rg --help`.

| tool | long | short | what it does |
|---|---|---|---|
| rg | --binary |  | Enabling this flag will cause ripgrep to search binary files. |
| rg | --follow | -L | This flag instructs ripgrep to follow symbolic links while traversing |
| rg | --glob | -g | Include or exclude files and directories for searching that match the |
| rg | --glob-case-insensitive |  | Process all glob patterns given with the -g/--glob flag case |
| rg | --hidden | -. | Search hidden files and directories. |
| rg | --iglob |  | Include or exclude files and directories for searching that match the |
| rg | --ignore-file |  | Specifies a path to one or more gitignore formatted rules files. |
| rg | --ignore-file-case-insensitive |  | Process ignore files (.gitignore, .ignore, etc.) case insensitively. |
| rg | --max-depth | -d | This flag limits the depth of directory traversal to NUM levels beyond |
| rg | --max-filesize |  | Ignore files larger than NUM in size. |
| rg | --no-ignore |  | When set, ignore files such as .gitignore, .ignore and .rgignore will |
| rg | --no-ignore-dot |  | Don't respect filter rules from .ignore or .rgignore files. |
| rg | --no-ignore-exclude |  | Don't respect filter rules from files that are manually configured for |
| rg | --no-ignore-files |  | When set, any --ignore-file flags, even ones that come after this flag, |
| rg | --no-ignore-global |  | Don't respect filter rules from ignore files that come from "global" |
| rg | --no-ignore-parent |  | When this flag is set, filter rules from ignore files found in parent |
| rg | --no-ignore-vcs |  | When given, filter rules from source control ignore files (e.g., |
| rg | --no-require-git |  | When this flag is given, source control ignore files such as .gitignore |
| rg | --one-file-system |  | When enabled, ripgrep will not cross file system boundaries relative to |
| rg | --type | -t | This flag limits ripgrep to searching files matching TYPE. |
| rg | --type-add |  | This flag adds a new glob for a particular file type. |
| rg | --type-clear |  | Clear the file type globs previously defined for TYPE. |
| rg | --type-not | -T | Do not search files matching TYPE. |
| rg | --unrestricted | -u | This flag reduces the level of "smart" filtering. |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P022 | confirmed | Are .gitignore'd paths skipped by default? | `rg -l generated .` |
| P023 | confirmed | Is .ignore a separate layer from .gitignore? | `rg -l --no-ignore-vcs 8080 .` |
| P024 | confirmed | Are hidden files skipped by default? | `rg -l in a hidden file .` |
| P025 | confirmed | Does naming an ignored file explicitly override the ignore rules? | `rg -c generated target/generated.rs` |
| P026 | confirmed | Does -t restrict the search to one language? | `rg -l -tpy print .` |
| P027 | confirmed | Does a negated glob remove a subtree? | `rg -l --no-ignore -g !node_modules/** console.log .` |
| P028 | confirmed | Does --max-depth bound traversal? | `rg -l --max-depth 2 buried .` |
| P029 | confirmed | Is a binary file searched when named explicitly but skipped when found by traversal? | `rg -c needle data/blob.bin` |

## Decision rules

- Narrow by path and type before tuning the pattern. Fewer files is the cheapest speedup available and it never changes what a match means.
- `-u` / `-uu` / `-uuu` is a ladder, not a switch: one relaxes ignores, two adds hidden files, three adds binary. Name the layer instead when you know which one is in the way.
- `--files` shows the candidate set without searching it. When results look wrong, look at the candidates first.
- ast-grep exposes the same vocabulary -- `--globs`, `--no-ignore <layer>`, `--follow`, `--threads` -- so a file-set policy worked out for one tool transfers to the other.

## Anti-patterns

- Reaching for `-uuu` to make a search work. It usually means one specific layer is in the way; find out which, or you will also be searching `.git` and `node_modules`.
- Assuming `--no-ignore-vcs` reveals everything. `.ignore` is a separate layer (P023).
- Concluding a symbol is absent without checking whether its file was in the candidate set at all.

## Checklist

- Was the file in `rg --files` output?
- Which ignore layer excluded it -- vcs, dot, parent, global, hidden?
- Is the path explicit, and therefore exempt from the stack?
