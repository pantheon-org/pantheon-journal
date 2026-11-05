# pantheon-journal

Rust command line tool (`pantheon-journal`) that creates, lints, indexes and archives structured journal entries,
plus the bundled `journal-entry-creator` agent skill. See `README.md` for what it does and how it is installed.

## Layout

- `crates/journal/`: the only workspace member. `src/lib.rs` documents each module; `tests/` holds the integration
  tests (`backfill.rs`, `entries_dir.rs`, `index.rs`, `lint.rs`, `roundtrip.rs`).
- `journal-entry-creator/`: the skill (`SKILL.md`, `references/`, `evals/` and `assets/`, which holds
  the entry templates in `assets/templates/`, the entry frontmatter schema and the taxonomy schema in `assets/schemas/`, and `assets/taxonomy.default.json`). `crates/journal/build.rs`
  embeds it in the binary, so a change here changes the binary.
- `.github/workflows/`: CI. `rust-ci.yml` is the source of truth for what must pass; `audit-skill.yml` validates and
  grades the skill with a released `pantheon-skill-auditor`.
- `.context/`: gitignored working state. Never commit it.

## Commands

Tools are pinned in `mise.toml` (`mise install` sets them up and installs the git hooks).

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --workspace --locked
cargo test --workspace --locked
mise run lint        # markdown, via rumdl
mise run check       # every hk check, the same steps as the git hooks
```

Run `cargo` with `--locked`: CI does. When checking test results, require `0 failed` on every suite; do not add up
the "passed" counts.

## Conventions

- Branch off `main`, never commit to it. Conventional commits with a scope, for example `fix(cli): reject an empty
  title`; the squash-merge subject ends in the PR number.
- PR bodies use the three headings in `.github/pull_request_template.md`: What, Why, Notes.
- Git hooks run through `hk` (`hk.pkl`): rustfmt, rumdl, yamllint and shellcheck on commit; clippy and tests on push.
  Do not skip them.
- Markdown is linted by rumdl with `.rumdl.toml`. Shell scripts must pass shellcheck.
- The Rust toolchain version is set in `rust-toolchain.toml`, `mise.toml` and `rust-ci.yml`. Change all three together.
- The `pahdk-*` dependencies are pinned in the root `Cargo.toml`, to a `rev` of the dev-kit's `main` until a tag includes
  `validate`; move them to the tag then. Both must share one pin.
- A workflow `run:` block holds at most 15 lines of code; longer logic goes in a script under `.github/scripts/`.
- Keep the skill's documented behaviour and the CLI in step: `references/compliance.md` and the templates describe the
  rules `src/validate.rs` enforces. The frontmatter rules live in `assets/schemas/entry-frontmatter.schema.json`,
  embedded by `validate.rs`; change the schema, not Rust, to change them. The skill ships no scripts; validation is
  `pantheon-journal validate`.
- State is stored under `$XDG_DATA_HOME/tekhne/journal` (or `~/.local/share/tekhne/journal`). Keep that path.
- Commands that read entries take `--entries-dir` (default `.`) for a wrapper directory under `--root`/`--dir` that
  holds the `YYYY/MM-Month/DD-Weekday/` tree. Stored `file` paths keep that prefix.
- Only `archive-media` uses the network, and `status` reads the consuming repository's own `.context/` files. Keep
  every other command offline and deterministic.
