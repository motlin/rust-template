Every public singular recipe parameter must be a documented command-line option:

- Declare `[arg(..., long, help=...)]`, or use an intentional short option with
  `help=...`.
- Convert parameter names to lowercase kebab case (`OUTPUT_FILE` becomes
  `--output-file`).
- Model boolean behavior as a semantic value-less flag such as `--warn`, `--quick`, or
  `--no-clean`, preserving the recipe's existing default behavior.
- Keep private helper parameters such as `_run command` positional.
- Keep variadic passthrough parameters such as `*FLAGS` and `*args` positional.
- Keep dependency expressions inside justfiles positional. Only command-line callers
  use option syntax.

Run the base template's audit against the template and every selected project. The audit
loads every tracked root and nested justfile through `just --dump --dump-format json` and
reports public singular parameters that lack an option or option help text.

```bash
python3 ~/projects/project-template/scripts/audit-just-options.py <project-path>
```

Create one task per failing project that lists every reported recipe and parameter. The
task must require the corresponding option declaration, caller migrations, `just
--usage <recipe>` inspection, representative `just --dry-run` calls, and a passing audit.
Do not create conversion tasks for private helpers or variadic passthrough parameters.

Each sibling template's `.claude/commands/sync-template.md` must include this file, and
its shared copy must remain byte-identical to the base template. A missing include,
different copy, or audit failure is an out-of-sync condition that requires a task.
