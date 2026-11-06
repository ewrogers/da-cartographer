# Working in DA Cartographer

Read [CONTEXT.md](CONTEXT.md) before making changes. Use the
[editor guide](docs/EditorGuide.md) to check user-visible behavior and asset setup.

## Workflow

- Work on a scoped branch. Use [Conventional Commits](https://www.conventionalcommits.org/)
  with an imperative summary, such as `fix: preserve the saved map revision`.
- Keep Cargo.lock committed. Use stable Rust and `--locked` for repeatable
  builds. Separate dependency upgrades from feature work.
- Run focused tests for the changed behavior and format touched Rust files.
  Before a release, run `cargo test --workspace --locked` and verify the
  relevant release packages. Keep broader lint cleanup in its own change.
- Update the README or editor guide when a user workflow changes. Keep
  [CONTEXT.md](CONTEXT.md) current when project direction or handoff status changes.

## Rust conventions

- Use `name.rs` for a single-file module and `name/mod.rs` once it has child
  files. Move `name.rs` to `name/mod.rs` when adding the first child, including
  a substantial `tests.rs`. Cargo entry points keep their standard filenames.
- Keep format and render helpers independent of egui so they can be tested
  without starting the app. Keep GPU resources on the UI thread.
- Borrow inputs where ownership can stay with the caller. Return typed errors
  or `io::Result` for expected failures and validate file sizes and offsets
  before slicing or allocating. Reserve panics for broken invariants and tests.
- Preserve tile ID indexing, byte layout, and archive precedence. Test format
  changes with synthetic fixtures that cover legacy and newer asset packs.
  Keep game archives, personal maps, and prefabs out of version control.
- Run rustfmt and use Clippy for touched code. Prefer narrow interfaces and
  clear domain names over speculative abstractions.

## Branding and releases

- Use **DA Cartographer** for display and `cartographer` for the app package
  and executable. Keep the desktop ID in the app and platform metadata aligned.
- After changing the icon artwork, regenerate the shared PNG with
  `cargo run --locked -p render --example gen_icon`. Check the Windows icon
  resource, macOS bundle icon, and Linux desktop entry when changing packaging.
