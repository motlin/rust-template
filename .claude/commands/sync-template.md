---
description: Sync tool versions from this template to sibling Rust projects
argument-hint: [project-name|all]
---

# Rust Template Sync

This template is the source of truth for Rust/Cargo project configurations.

Template path: !`pwd`

## Managed files

- `scripts/audit-just-options.py` — repository-wide `just` option policy audit

### Mise tools

Read tool versions from `.mise/config.toml` in this template. mise is the single source of truth for the Rust toolchain and the cargo helper tools — there is no `rust-toolchain.toml`.

- `rust` (pinned version + components: `clippy`, `rustfmt`, `llvm-tools`)
- `just`, `node`, `pre-commit`
- `aqua:nextest-rs/nextest/cargo-nextest`
- `aqua:EmbarkStudios/cargo-deny`
- `aqua:taiki-e/cargo-llvm-cov`

### Cargo (Cargo.toml)

- `edition`
- `[profile.release]` (thin LTO + strip)
- `[lints.rust]` / `[lints.clippy]` policy
- `[dependencies]` — the shared crate baseline below

#### Shared crate baseline

These crates are used across every own project, so the template pins them and propagates the pin. A project that does not need one simply omits it; the sync never adds a crate to a project that has no use for it.

- `clap = { version = "<exact>", features = ["derive"] }`

Pin it to a full version (`"4.6.6"`, not `"4"`) so the declared version is greppable and identical across every project, matching the exact-version policy already used for mise tools.

Note what this does _not_ do: it does not unblock Dependabot. Because `Cargo.lock` is committed in every one of these projects, Dependabot already opens a PR for in-range upgrades as a lockfile-only change, and only edits `Cargo.toml` when the new version falls outside the requirement. A caret spec hides nothing.

Also note that Cargo reads a bare `"4.6.6"` as `^4.6.6`, not as an exact pin. Use `"=4.6.6"` if a project ever needs a true hard pin; the shared baseline deliberately does not, so in-range security patches can still land via lockfile.

### GitHub workflows

- Auto-fix commit message format (`fmt-fix`, `clippy-fix`, `pre-commit-fix`)
- CI/CD patterns (merge-group gate, push build)

### Crate version policy

- **Pin full versions** for every crate in `[dependencies]` and `[dev-dependencies]` — write `clap = { version = "4.6.6", ... }`, never `"4"` or `"4.6"`. This is for readability and cross-project consistency, not for Dependabot coverage, which a committed `Cargo.lock` already provides.
- **Same version everywhere.** The template and every own project pin the identical version of a shared crate.
- **Apply the 24-hour rule** from the version policy below to crates too: check the crates.io publish timestamp before pinning a brand-new release.

```bash
curl -s https://crates.io/api/v1/crates/<crate>/versions | jq -r '.versions[0] | "\(.num) \(.created_at)"'
```

### Rust version policy

- **Pin a specific version**: pin an exact `rust` version (e.g. `1.97.1`) — the same version across the template and every own project. Do not use the `stable` channel alias; pinned versions keep builds reproducible.
- **Components**: always include `clippy,rustfmt,llvm-tools` (llvm-tools is required by `cargo-llvm-cov`).
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

## Git ignore files

@.claude/includes/sync-gitignore.md

## Default git test

@.claude/includes/sync-git-test.md

## Just recipe options

@.claude/includes/sync-just-options.md

## Workflow

Work through these in order:

- **Refresh the template.** Run the version checks above; if this template is behind, update it first.
- **Pull from projects.** Read `.llm/projects.yaml` and scan each project's `.mise/config.toml`, `justfile`, `Cargo.toml`, `.just/*.just`, and `.github/workflows/*`. If any project has a newer version, a drifted crate pin, or a better CI pattern (new auto-fix job, useful recipe), verify it is intentional, update this template, then push to the others.
- **Scan for stale configs.** For each project, run the stale-config scan above before generating tooling tasks. Alert on findings; do not delete.
- **Scan ignore files.** For each project, run the `.gitignore` / `.git/info/exclude` scan above. Promote per-clone excludes every peer needs; question only hand-added dead entries. Alert on findings; do not edit either file.
- **Audit recipe options.** Run the shared `just` option audit against each project and create one project-scoped task for every failure.
- **Generate tasks.** For each project, compare against this template and write tasks into its `.llm/todo.md` for any mismatches.

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

**Crate version pin:**

```
Pin clap to exact version <target>
  Edit Cargo.toml
  Change: clap = { version = "<current>", features = ["derive"] }
  To: clap = { version = "<target>", features = ["derive"] }
  Reason: full versions keep the declared pin consistent across projects
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
