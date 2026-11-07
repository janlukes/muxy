# Agents

## Repository

- Before finishing: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS=-D warnings`.

## Main Guides

- Only test real scenarios and risks (data loss, security, compatibility, concurrency, regressions). Don't test obvious code or defaults.

## Third-party dependencies

When adding new third-party dependencies, provide user with their github url and stars and metrics if they are being maintained so user can approve adding it.

## Performance

- Do not ignore performance and memory efficiency. with GPUI it is easy to cause high CPU and memory usage.

## Git workflow for this fork

The owner explicitly permits AI agents to create commits and push requested
changes to working branches in `janlukes/muxy`. This replaces the original
upstream prohibition on AI commits and pushes for this fork only.

- Commit and push when requested by the owner; the Windows port is maintained
  on `windows/stable-2.3`.
- Leave remote `main` unchanged unless the owner explicitly requests an update.
- Do not rewrite a published branch without explicit authorization. For an
  authorized rebase, preserve the previous tip and use `--force-with-lease`.
- Do not push to `muxy-app/muxy` or other repositories without separate explicit
  authorization. Verify the destination remote before pushing.
- Opening or modifying PRs/issues remains a human action; read-only access is
  allowed.

## Desktop app lifecycle

Do not automatically launch, close, or restart the Muxy desktop app or restart
its server. Previous automated launches froze the app. Build and package
without opening the GUI; leave launching to the owner unless explicitly asked.
