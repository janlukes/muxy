<p align="center">
  <img src="packaging/macos/AppIcon.png" alt="Muxy" width="128" height="128">
</p>

<h1 align="center">Muxy</h1>

<p align="center">Lightweight and Memory efficient terminal multiplexer built with Rust, GPUI and libghostty.</p>
<p align="center"><a href="#windows-native-port">Windows</a> | <a href="#install">Mac</a> | <a href="docs/user-guide/getting-started.md#linux-desktop-app">Linux</a> | <a href="https://apps.apple.com/de/app/muxy/id6762464046?l=en-GB">iOS</a> | <a href="https://play.google.com/store/apps/details?id=com.muxy.app">Android</a> | <a href="https://discord.gg/4eMXAmJQ2n">Discord</a></p>

## Windows native port

This fork includes a working native **Windows x64** port of stable Muxy 2.3.0
on the [`windows/stable-2.3`](https://github.com/janlukes/muxy/tree/windows/stable-2.3)
branch. It runs the desktop app, server, and CLI directly on Windows using
ConPTY; WSL is not required. The port also includes the GPUI fix for truncated
non-ASCII tab titles and the Muxy application icon in the Windows executable.

Local terminals, Unicode project paths and terminal titles, and paired TLS
connections to macOS servers have been verified. Windows server paths are
translated on the wire for compatibility with unmodified macOS/mobile clients.
The latest native checks passed **789 Rust tests**, Clippy, formatting, and
documentation checks. ARM64 is configured in CI but has not been verified locally.

This remains an **experimental development build**, not an official signed
Windows release. GUI startup freezes have been reported and remain under
investigation. Browser panes, SSH remotes from the Windows desktop, extension
scripts/webviews, voice, Quick Terminal, clipboard images, shell installation
of the CLI, and automatic updates are currently unavailable.

See [Windows build instructions and maintenance](docs/windows-port.md) for
requirements, commands, known issues, and the stable-branch workflow. The
upstream download links below provide the upstream macOS/Linux releases;
build this fork's Windows branch from source.

## Screenshots

<img height="1592" alt="image" src="https://github.com/user-attachments/assets/eca8fb92-b2f4-4a53-a25f-6f75316222af" />

## Top level features

- Background sessions
- Remote sessions
- Split panes
- Git worktrees
- Extensions
- Mobile companion apps (iOS & Android)
- AI coding agent integrations
- 490+ themes
- TUI

## Install

The desktop app runs on macOS 14 or newer, on Apple Silicon or Intel.

```bash
brew install --cask muxy-app/tap/muxy
```

Or download the DMG for your Mac from the [latest release](https://github.com/muxy-app/muxy/releases/latest).

On **Ubuntu 24.04**, use the [quick installer](docs/user-guide/getting-started.md#quick-installer-with-in-app-updates)
for the latest stable release and in-app updates, or download a **`.deb`** or
**AppImage** from the [latest release](https://github.com/muxy-app/muxy/releases/latest).
The [Linux section](docs/user-guide/getting-started.md#linux-desktop-app) covers all
three formats. See [Beta builds](docs/user-guide/getting-started.md#beta) for beta
installs. Linux support is experimental, targeting x86_64 and ARM64.

The `muxy` CLI and server also run on their own, on macOS and Linux:

```bash
brew install muxy-app/tap/muxy-cli
```

See [Getting started](docs/user-guide/getting-started.md) for the install script, upgrading from 1.x, and beta builds.

## Documentation

See [docs](https://muxy.app/docs).

## License

[MIT](LICENSE)
