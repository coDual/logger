# AGENTS.md

## General principles

- **Minimal changes.** Don't introduce unnecessary dependency swaps, architecture
  changes, or runtime switches. Keep diffs small and focused on what was asked.
- **Don't create files unless required.** Prefer editing existing files over
  creating new ones.

## Language and tooling

- Async runtime: `async-std` (with `tokio1` compatibility feature for
  tokio-based dependencies like `reqwest`). Do not switch runtimes without being
  asked.
- Configuration is YAML-based, loaded via the `config` crate. There is a secrets file for testing
  (`secrets.yaml`) which is gitignored.

## Code style

- Run `cargo fmt` before finishing any change.
- Run `cargo clippy -- -W clippy::pedantic` and fix all warnings.
- Use `Self::` instead of the type name in enum/struct impls.

## Module structure

- `main.rs`: CLI argument parsing, output formatting, file writing, and
  top-level glue. Keep it lean.
- External API logic goes in its own module (e.g. `karakeep.rs`). Public exports
  should be clean, high-level types with resolved fields. Raw API response
  schemas stay private to the module.
- `settings.rs`: config struct, deserialization, and output path computation.
  Sub-modules (e.g. `settings/frequency.rs`) handle specific concerns.

## Testing and verification

- Run `cargo test` to verify unit tests pass.
- When a real config is available (`secrets.yaml`), run `cargo run -- secrets`
  to verify end-to-end behavior. Clean up any test output artifacts afterward.
- The CI pipeline (GitHub Actions) runs `check`, `test`, `fmt`, and `clippy`.
  Make sure all four pass before considering a change complete.

## Commits

Switch to a new branch when making any new set of changes.
Use the convention `<model family>/<topic>` (e.g. `opus/bump-chrono-to-v5`).

If you are an LLM, add a commit trailer to commits `Assisted-By: <Model Name>`.
For example: `Assisted-By: Claude Opus 4.6`.
Keep commits atomic. All tests, lints and build MUST pass on EVERY commit.
