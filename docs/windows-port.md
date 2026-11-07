# Native Windows port

The `windows/stable-2.3` branch in `janlukes/muxy` carries the native Windows
port on top of the upstream `v2.3.0` tag (`c79305a88b2815bb18147c19fb01a9aba0a8e93f`).
The fork's `main` is maintained separately; publishing this port does not
update it.

## Build on Windows x64

Install Git, Rust 1.92.0 with rustfmt and Clippy, Visual Studio 2022 C++ Build
Tools with the MSVC x64 toolchain and Windows SDK, and Zig 0.17.0. Put `zig.exe`
on `PATH`. Python 3 is needed for the repository's validation scripts.

Open **Developer PowerShell for Visual Studio 2022** with the x64 toolchain:

```powershell
git clone --branch windows/stable-2.3 https://github.com/janlukes/muxy.git
cd muxy
cargo build --locked --target x86_64-pc-windows-msvc -p muxy-app -p muxy-server -p muxy-cli
```

The binaries are in `target/x86_64-pc-windows-msvc/debug/`. Keep `muxy-app.exe`,
`muxy-server.exe`, and `muxy.exe` together. Launch the desktop app manually when
ready. Debug builds use `%LOCALAPPDATA%\Muxy Dev`; preserve this directory to
retain settings, projects, and pairing credentials. Do not share its contents:
it includes private authentication material.

The Windows SDK's `rc.exe` embeds `packaging/windows/Muxy.ico` into the desktop
executable as resource ID 1. Regenerate the icon from the existing stable macOS
artwork with `scripts/windows-icon.ps1`. No extra Rust dependency is needed.

For all native checks, use the native x64 Rust toolchain and leave
`CARGO_BUILD_TARGET` unset:

```powershell
python scripts/check-windows.py --target x86_64-pc-windows-msvc
```

That command checks build metadata without opening the GUI. Validation must
not automatically launch, close, or restart the user's desktop app or server.
The latest local run passed 789 Rust tests with two ignored tests, formatting,
all-target/all-feature Clippy with warnings denied, and rustdoc with warnings
denied. An isolated ConPTY smoke test also verified a Unicode project path,
command execution, and the title `Příliš 日本語`.

## Features and known issues

- Native desktop app, server, CLI, ConPTY terminals, and Unicode paths/titles.
- Paired TLS remotes are enabled independently of SSH support. Authentication,
  ping, and catalog retrieval from a stable macOS server were verified.
- Windows paths use `/C:/...` and `/UNC/server/share/...` at the CBOR boundary
  and are translated back for Windows requests. Existing native JSON paths
  are retained. This addresses stable Unix clients rejecting Windows catalogs.
- GPUI revision `ec24331257b2565cff99ad09a99d5f5855af9be0` fixes measuring and
  truncating multibyte text. Other development-version features are not adopted.
- Browser panes, SSH remotes from the Windows desktop, extension scripts and
  webviews, voice, Quick Terminal, clipboard images, CLI shell installation,
  and automatic updates do not have supported Windows backends yet.
- GUI startup freezes have been reported, especially during automated launches;
  the cause is not resolved. Launch the app manually. Published source and
  passing headless checks are not a claim of complete GUI stability.
- Desktop clients keep their own tab layouts. Use a project's **Existing
  Terminals…** menu to open sessions created by another client. Reports of
  missing older Mac sessions still need end-to-end investigation.
- Restarting the server ends live terminal processes. Updating only the
  desktop app does not require restarting a compatible running server.
- Windows ARM64 has a CI configuration but has not been tested on the local
  x64 machine. Windows packages are unsigned development builds.

## Maintaining the fork

Keep `main` separate from port work and advance it only deliberately to
selected upstream stable releases. The current `main` was left untouched
when this Windows branch was published. Rebasing the port does not depend on
moving `main`: rebase onto the selected upstream release tag directly.

The original Windows platform changes come from upstream commit `c9831000`,
adapted to stable 2.3.0. Keep the adapted port separate from the stable base
so the changes can be reviewed and rebased for later releases.

Before a future rebase, commit or otherwise preserve local edits, fetch
upstream tags, and create a backup branch of the published Windows tip.
Rebase onto the selected stable tag, resolve conflicts, and run native Windows
checks again. Review changes to protocol compatibility and the GPUI pin.
If updating the same published branch after a rebase, use `--force-with-lease`
with the previously recorded remote tip. Never force-push `main`. A new Windows
branch per stable version is also useful when retaining old builds.

## Sources

- [Stable Muxy 2.3.0](https://github.com/muxy-app/muxy/releases/tag/v2.3.0)
- [Upstream Windows platform](https://github.com/muxy-app/muxy/tree/c9831000)
- [GPUI non-ASCII truncation fix](https://github.com/muxy-app/gpui/commit/ec24331257b2565cff99ad09a99d5f5855af9be0)
