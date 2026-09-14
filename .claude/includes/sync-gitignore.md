Two files decide what git ignores: `.gitignore`, which is tracked and shared with every clone, and `.git/info/exclude`, which is per-clone and invisible to everyone else. They drift in opposite directions, so scan both.

Both scans below rest on the same query — which ignore rule actually matched a file that exists in the working tree right now:

```bash
git ls-files --others --ignored --exclude-standard -z \
    | xargs -0 -r -n 500 git check-ignore -v \
    | awk -F'\t' '{print $1}' | sort -u
```

Each output line is `<source file>:<line number>:<pattern>`.

A third source can appear: the user's global ignore file (`~/.config/git/ignore` or `core.excludesFile`). It is per-user rather than per-clone, so it hides paths from this user in _every_ repo. Judge its entries the same way as `.git/info/exclude` ones — a path that every clone of _this_ project will produce belongs in the project's `.gitignore` — but never edit the global file as part of a project sync. Report it and let the user decide.

## Promote per-clone excludes that every peer needs

Entries in `.git/info/exclude` protect one working copy. A teammate, a fresh clone, or a new worktree gets none of them, so anything there that a build or tool recreates for everyone shows up as untracked noise for the whole team.

List the active entries, then check which ones matched above:

```bash
grep -vnE '^\s*(#|$)' .git/info/exclude
```

An entry belongs in `.gitignore` instead when the path it hides is reproducible for any clone: build output, tool or dependency caches, generated directories, agent scratch directories. Generate a task to move it.

Keep it in `.git/info/exclude` only when it is genuinely local — a personal scratch file, a machine-specific path, an editor's private state that peers would not create.

Also drop any exclude entry that a `.gitignore` pattern already covers. Duplicated intent rots: the two copies diverge and the local one silently wins.

## Question .gitignore entries that match nothing

Every non-comment `.gitignore` line missing from the query output matched nothing in the working tree.

Matching nothing is weak evidence on its own — plenty of correct patterns exist precisely so a file never appears. Never propose removing:

- Patterns inside a vendored gitignore.io block (between its generated header and footer comments). That block is upstream-managed and mostly prophylactic by design; editing it guarantees a conflict the next time it is regenerated.
- Secret and credential patterns (`.env`, `*.pem`, `*credentials*`), which are load-bearing exactly when nothing matches them.
- OS and editor cruft (`.DS_Store`, `Thumbs.db`, `*.swp`).
- Patterns guarding files a build produces only on failure or under a flag: crash dumps, `hs_err_pid*`, coverage and profiling output.
- Negation patterns (`!…`) and any pattern a negation depends on — removing the broad pattern changes what the negation re-includes.

Propose removal only for a hand-added, project-specific entry naming a concrete path the project no longer has: a dropped tool's cache directory, a renamed build directory, a module that was deleted. Confirm the path is gone for good rather than merely absent from a clean tree:

```bash
git log --oneline -1 -- '<path>'
```

## Never edit either file automatically

Same rule as stale configs. List every finding in the sync report and generate a task naming the file, the line number, the pattern, and why it looks dead or misplaced. The user confirms before anything is removed or moved.
