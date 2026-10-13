Find which ignore rule matched each ignored file in the working tree:

```bash
git ls-files --others --ignored --exclude-standard -z \
    | xargs -0 -r -n 500 git check-ignore -v \
    | awk -F'\t' '{print $1}' | sort -u
```

- **`.git/info/exclude`:** propose moving an entry to `.gitignore` when every clone would produce the path (build output, caches, generated or agent scratch directories). Propose dropping entries that `.gitignore` already covers.
- **Global ignore file:** report entries that belong in this project's `.gitignore`, but never edit the global file.
- **`.gitignore` entries that match nothing:** propose removal only for a hand-added entry naming a path the project no longer has, confirmed with `git log --oneline -1 -- '<path>'`. Never propose removing gitignore.io blocks, secret patterns, OS or editor cruft, failure-only output (crash dumps, coverage), or negations and the patterns they depend on.

Never edit ignore files automatically. Report each finding as a task with its file, line, pattern, and reason.
