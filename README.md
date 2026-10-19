<p align="center">
  <img src="docs/assets/images/logo.svg" alt="WAVO logo" width="88" height="88">
</p>

<h1 align="center">WAVO</h1>

<p align="center"><strong>WhatsApp, native and fast.</strong><br>A lightweight WhatsApp app for Linux, macOS, and Windows.</p>

<p align="center">
  <a href="https://wavo.rocks/download/"><strong>Download</strong></a> ·
  <a href="https://wavo.rocks/getting-started/">Getting started</a> ·
  <a href="https://wavo.rocks/using-wavo/">User guide</a> ·
  <a href="CONTRIBUTING.md">Contribute</a>
</p>

<p align="center">
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Built_with-Rust-00a884" alt="Built with Rust"></a>
  <a href="https://wavo.rocks/download/"><img src="https://img.shields.io/badge/Linux_%C2%B7_macOS_%C2%B7_Windows-24292f" alt="Linux, macOS, and Windows"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-00a884" alt="MIT license"></a>
</p>

WAVO is a WhatsApp client written in Rust with
[egui](https://www.egui.rs) and [fastframe](https://fastframe.dev), the
framework behind a family of native desktop apps. It links to your phone as a
companion device through
[whatsapp-rust](https://github.com/oxidezap/whatsapp-rust) and has no browser
engine. In our Linux test it opens in under a second and uses about 200 MB of
idle RAM, against 1.13 GB for WhatsApp Web.
[See the measurements](https://wavo.rocks/benchmarks/).

The interface is available in English, Chinese (Simplified and Traditional),
French, German, Indonesian, Italian, Portuguese (Brazil), Russian, Spanish,
and Turkish.

<picture>
  <source media="(prefers-color-scheme: light)" srcset="docs/screenshot-light.png">
  <img src="docs/screenshot.png" alt="A WAVO conversation with an attachment, voice messages, reactions, a quoted reply, and a link preview">
</picture>

<details>
<summary><strong>Watch WAVO in action</strong></summary>

https://github.com/user-attachments/assets/2bf86b54-45fc-4add-8de7-c426c3cdad9b

</details>

[Features](#your-chats-on-your-desktop) · [Install](#install) · [Link your phone](#link-your-phone) · [Help](#guides-and-help)

## Your chats, on your desktop

| Feature | What you can do |
|---|---|
| **Messages** | Send formatted text, replies, edits, reactions, forwards, pictures, files, stickers, GIFs, polls, and voice messages. |
| **Media in the chat** | Play voice messages, videos, round video messages, GIFs, and animated stickers in place. |
| **Calls** | Make voice and video calls from a chat's header, answer incoming calls on the desktop, answer group calls and join call links, and browse every call in the Calls tab. |
| **Your history** | Keep your chats in an encrypted archive on this computer, and fetch older messages from your phone as you scroll. |
| **Several numbers** | Link more than one WhatsApp number and switch between them in one window. |
| **Desktop controls** | Use keyboard shortcuts, select and copy text across messages, and stay linked in the tray with notifications. |
| **Themes** | Choose light, dark, system appearance, or custom colours. On Omarchy, follow your desktop theme. |
| **Privacy** | Hide locked chats behind a code, or the whole window behind an app lock. Nothing goes to a server of ours. |

## Install

| Platform | Installation |
|---|---|
| **macOS** | `brew install --cask crmne/tap/wavo`, or [download the Mac app](https://wavo.rocks/download/#macos). |
| **Arch Linux** | `yay -S wavo-bin` |
| **Windows** | Choose your build on the [Download page](https://wavo.rocks/download/#windows). |
| **Other Linux** | Find Nix, AppImage, and other options on the [Download page](https://wavo.rocks/download/#linux). |
| **From source** | Follow [Getting started](https://wavo.rocks/getting-started/) for dependencies and commands. |

## Link your phone

1. Open WAVO. It shows a QR code.
2. On your phone, open WhatsApp, then **Settings**, **Linked devices**,
   **Link a device**, and scan the code. Or click **Link with phone number**
   and enter the eight-character code on your phone.
3. Your recent chats arrive from the phone. Older messages follow as you
   scroll up.

The link survives restarts, and your phone does not need to stay online to
read what WAVO has already stored. See
[Getting started](https://wavo.rocks/getting-started/) for the full
walkthrough and [How it links](https://wavo.rocks/how-it-links/) for the
details.

## Guides and help

The complete guide lives at **[wavo.rocks](https://wavo.rocks/)**.

| You want to… | Read |
|---|---|
| Know what WAVO does and does not do yet | [What is WAVO?](https://wavo.rocks/what-is-wavo/) |
| Link your phone or build from source | [Getting started](https://wavo.rocks/getting-started/) |
| Learn messages, stickers, polls, and shortcuts | [Using WAVO](https://wavo.rocks/using-wavo/) |
| Find settings, stored files, encryption, the app lock, and updates | [Settings and files](https://wavo.rocks/settings-and-files/) |
| Make your own theme | [Making a theme](https://wavo.rocks/themes/) |
| Understand what stays on this computer | [How it links](https://wavo.rocks/how-it-links/) |
| Understand account risk | [Account safety](https://wavo.rocks/what-is-wavo/#account-safety) |
| Report a bug or propose a feature | Read [Contributing](CONTRIBUTING.md), then use the [issue forms](https://github.com/crmne/wavo/issues/new/choose). |

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening an issue or pull
request. To look at the interface without a WhatsApp account, run:

```sh
cargo run --features demo -- --demo
```

[AGENTS.md](AGENTS.md) describes the architecture, [DEMO.md](DEMO.md) the
demo pages, screenshots, and recorded tours, and
[PACKAGING.md](PACKAGING.md) releases and packages.

Native notifications use Fastframe on Linux, macOS and Windows. WAVO keeps
its account/chat routing, privacy settings, notification limits and sound
previews. Clicks open the announced message; reading a chat withdraws its
tracked notifications. OS permissions and notification presentation still apply.


## More native apps

**Want Spotify just as fast and native?** [Spotifast](https://spotifast.rocks)
is WAVO's sibling. Both are built on
[fastframe](https://fastframe.dev).

## Acknowledgements

WAVO uses [whatsapp-rust](https://github.com/oxidezap/whatsapp-rust),
[egui](https://www.egui.rs), [fastframe](https://fastframe.dev), the
[Inter](https://rsms.me/inter/) typeface and Noto Color Emoji (OFL), and
[Lucide](https://lucide.dev) icons (ISC), also used for the chat wallpaper
doodles. The notification sounds are [Pidgin](https://pidgin.im)'s, under the
GPL-2.0 (see `assets/sounds/`).

WAVO is an unofficial client and is not affiliated with WhatsApp or Meta.
Using an unofficial client may be against WhatsApp's terms of service and
could get an account suspended. Use it at your own risk. WhatsApp is a
trademark of WhatsApp LLC.

Licensed under the [MIT License](LICENSE).

## Star history

<a href="https://www.star-history.com/?repos=crmne%2Fwavo&amp;type=date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=crmne/wavo&amp;type=date&amp;theme=dark">
    <img alt="WAVO GitHub stars over time" src="https://api.star-history.com/chart?repos=crmne/wavo&amp;type=date">
  </picture>
</a>
