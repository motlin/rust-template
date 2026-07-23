---
description: Sync tool versions from this template to sibling Rust projects
argument-hint: [project-name|all]
---

# Rust Template Sync

This template is the source of truth for Rust/Cargo project configurations.

Template path: !`pwd`

## Managed files

### Mise tools

Read tool versions from `.mise/config.toml` in this template. mise is the single
source of truth for the Rust toolchain and the cargo helper tools — there is no
`rust-toolchain.toml`.

- `rust` (pinned version + components: `clippy`, `rustfmt`, `llvm-tools`)
- `just`, `node`, `pre-commit`
- `aqua:nextest-rs/nextest/cargo-nextest`
- `aqua:EmbarkStudios/cargo-deny`
- `aqua:taiki-e/cargo-llvm-cov`

### Cargo (Cargo.toml)

- `edition`
- `[profile.release]` (thin LTO + strip)
- `[lints.rust]` / `[lints.clippy]` policy

### GitHub workflows

- Auto-fix commit message format (`fmt-fix`, `clippy-fix`, `pre-commit-fix`)
- CI/CD patterns (merge-group gate, push build)

### Rust version policy

- **Pin a specific version**: pin an exact `rust` version (e.g. `1.97.1`) — the
  same version across the template and every own project. Do not use the `stable`
  channel alias; pinned versions keep builds reproducible.
- **Components**: always include `clippy,rustfmt,llvm-tools` (llvm-tools is
  required by `cargo-llvm-cov`).
- **Never add `rust-toolchain.toml`** — keep mise as the single source of truth.

## Version policy

@.claude/includes/sync-version-policy.md

Versions to check for this template:

```bash
mise ls-remote rust | tail -1
mise ls-remote just | tail -1
mise ls-remote "aqua:nextest-rs/nextest/cargo-nextest" | tail -1
mise ls-remote "aqua:EmbarkStudios/cargo-deny" | tail -1
mise ls-remote "aqua:taiki-e/cargo-llvm-cov" | tail -1
```

## Projects

`$ARGUMENTS` is a project name, `all`, or empty (treated as `all`).

@.claude/includes/sync-project-list.md

## Stale and conflicting tool configs

@.claude/includes/sync-stale-configs.md

Suspect configs for this template's toolchain:

- `rust-toolchain.toml` — conflicts with mise as the single source of truth
- Any other config for a tool the template has dropped

## Default git test

@.claude/includes/sync-git-test.md

## Workflow

1. **Refresh the template.** Run the version checks above; if this template is
   behind, update it first.
2. **Pull from projects.** Read `.llm/projects.yaml` and scan each project's
   `.mise/config.toml`, `justfile`, `.just/*.just`, and `.github/workflows/*`. If
   any project has a newer version or a better CI pattern (new auto-fix job, useful
   recipe), verify it is intentional, update this template, then push to the others.
3. **Scan for stale configs.** For each project, run the stale-config scan above
   before generating tooling tasks. Alert on findings; do not delete.
4. **Generate tasks.** For each project, compare against this template and write
   tasks into its `.llm/todo.md` for any mismatches.

## Creating tasks

@.claude/includes/sync-task-dedup.md

Marker for this template: `Source: ~/projects/rust-template`

### Task templates

**Mise tool update:**

```
Update just <current> → <target>
  Edit .mise/config.toml
  Change: just = "<current>"
  To: just = "<target>"
  Source: ~/projects/rust-template
```

**Adopt modular justfile includes:**

```
Adopt .just/*.just includes
  Replace flat justfile with imports of console.just, cargo.just, git.just, git-test.just
  Source: ~/projects/rust-template
```

## Report

@.claude/includes/sync-report.md
