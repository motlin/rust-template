# Repository Conventions

## Toolchain

Toolchain versions are pinned in `.mise/config.toml` — including the Rust toolchain,
`just`, and the cargo helper tools (`cargo-nextest`, `cargo-deny`, `cargo-llvm-cov`,
`release-plz`). Run `mise install` (or `just install`) to provision them. There is no
`rust-toolchain.toml`; mise is the single source of truth for the toolchain.

## Build & Check

Use `just` recipes — they wrap the underlying commands so local and CI invocations
stay in sync.

- `just build` — `cargo build --all-targets`
- `just test` — `cargo nextest run` (uses the `ci` profile when `CI` is set)
- `just clippy` — `cargo clippy --all-targets --all-features -- -D warnings`
- `just fmt` — `cargo fmt` (`--check` in CI, fix locally)
- `just deny` — `cargo deny check`
- `just coverage` — `cargo llvm-cov nextest`
- `just check` — build, test, clippy, fmt
- `just precommit` — `just check` plus `pre-commit run --all-files`

## Lints

Lint policy lives in the `[lints]` table of `Cargo.toml` (clippy `all` + `pedantic`,
`unsafe_code = "forbid"`). Prefer adjusting lints there over scattering `#[allow]`
attributes.

## Pre-commit

`.pre-commit-config.yaml` configures hooks. Run `pre-commit install` once after
cloning to enable the git hook, or `pre-commit run --all-files` to lint the whole
tree on demand.

## Style

- LF line endings everywhere (enforced via `.gitattributes` and the
  `mixed-line-ending` pre-commit hook).
- Markdown files are linted by `markdownlint-cli2` against `.markdownlint.jsonc`.
- YAML files follow the rules in `.yamllint.yaml`.
