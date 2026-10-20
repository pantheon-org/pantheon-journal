# pantheon-journal

Create and validate structured journal entries from the command line, with a bundled
`journal-entry-creator` skill that agents can install.

`pantheon-journal` covers entry creation from templates, linting against the journal conventions,
indexing, taxonomy, status and media archiving. `pantheon-journal skill install` copies the bundled
skill into detected agent directories.

## Install

Releases ship a shell installer and tarballs for macOS and Linux (arm64 and x86_64) once this
repository has its own release. Until then the latest is
[tekhne `tool/pantheon-journal-v0.2.5`](https://github.com/pantheon-org/tekhne/releases/tag/tool%2Fpantheon-journal-v0.2.5).

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The shared helpers come from
[agentic-harness-dev-kit](https://github.com/pantheon-org/agentic-harness-dev-kit)
(`ahdk-core`, `ahdk-install`), pinned to a git tag until they are published to crates.io. The
skill in `journal-entry-creator/` is embedded into the binary at build time and audited in CI with
[pantheon-skill-auditor](https://github.com/pantheon-org/pantheon-skill-auditor).

The tool keeps its state under `$XDG_DATA_HOME/tekhne/journal` (or `~/.local/share/tekhne/journal`),
the path it used before it moved here, so existing data stays where it is.

## History

Extracted from [pantheon-org/tekhne](https://github.com/pantheon-org/tekhne) with `git filter-repo`
(`crates/journal` and `skills/documentation/journal-entry-creator`, now `journal-entry-creator/`).
Commit messages keep their original `#NNN` references, which point at tekhne pull requests.
