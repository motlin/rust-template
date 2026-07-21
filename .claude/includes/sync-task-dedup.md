Create tasks in each project's `.llm/todo.md` with the markdown-tasks plugin script:

```bash
# Use the newest installed markdown-tasks plugin version (path rots if pinned)
TASK_ADD=$(find ~/.claude/plugins/cache/motlin-claude-code-plugins/markdown-tasks -name task_add.py | sort --version-sort | tail -1)
python3 "$TASK_ADD" ~/projects/<project>/.llm/todo.md "<task text>"
```

One task per out-of-sync file: name the file, say to match this template, and end the task
body with a `Source: <template path>` line naming this template. Tasks with prerequisites
must say so.

Stale task removal: before appending tasks to a project, delete existing unchecked task
blocks that carry this template's `Source:` marker; leave all other tasks untouched.
