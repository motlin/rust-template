# rust-template

A template for new Rust projects. It extends the language-agnostic
[`project-template`](https://github.com/motlin/project-template) with a complete
Rust build, lint, and test toolchain wired into pre-commit hooks and
GitHub Actions.

## Getting started

1. Create a new repository from this template.
2. Rename the crate: update `name`, `description`, and `repository` in `Cargo.toml`,
   and rename the binary in `src/main.rs` as needed.
3. Set `CLAUDE_CODE_TASK_LIST_ID` and `TAB_COLOR` in `.envrc`.
4. Provision the toolchain and enable hooks:

    ```console
    just install
    pre-commit install
    ```

5. Run the full check suite:

    ```console
    just check
    ```

## What's included

- **Toolchain** — [mise](https://mise.jdx.dev) pins Rust, `just`, and all cargo tools
- **Build / tasks** — [`just`](https://just.systems) with modular `.just/*.just` includes
- **Format** — `cargo fmt` (config in `rustfmt.toml`)
- **Lint** — `cargo clippy` (policy in `Cargo.toml` `[lints]`)
- **Test** — [`cargo-nextest`](https://nexte.st) (config in `.config/nextest.toml`)
- **Coverage** — [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov)
- **Supply chain** — [`cargo-deny`](https://embarkstudios.github.io/cargo-deny/) (`deny.toml`)
- **Hygiene** — pre-commit hooks, markdownlint, yamllint
- **CI** — merge-group gate, PR auto-fix jobs, push build, dependabot

## Toolchain policy

`mise` is the single source of truth for tool versions — there is intentionally no
`rust-toolchain.toml`, because all CI uses `jdx/mise-action`. See `CLAUDE.md` for
the recipe reference.

## License

Apache 2.0
