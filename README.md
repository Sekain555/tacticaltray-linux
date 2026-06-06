# TacticalTray Linux

**A Linux system tray monitor featuring Nox, the agent from Nightfall Tactics.**

> Linux port of [TacticalTray](https://github.com/Sekain555/tacticaltray) — inspired by [RunCat365](https://github.com/Kyome22/RunCat365) by Takuto Nakamura.

[![License](https://img.shields.io/github/license/Sekain555/tacticaltray-linux)](https://github.com/Sekain555/tacticaltray-linux/blob/main/LICENSE)

`Rust` `KDE` `StatusNotifierItem` `Linux`

---

## What is TacticalTray?

TacticalTray is a system tray application for Linux that monitors your PC's performance through the eyes of **Nox** — a stealth agent from the indie game *Nightfall Tactics*.

Nox lives in your taskbar and animates faster as your CPU load increases. The harder your system works, the harder Nox runs.

---

## Features

- 🖥️ **System monitoring** — CPU, GPU, Memory, Temperature, Storage and Network
- 🎭 **Nox animations** — light and dark mode variants
- 🌗 **Auto theme detection** — adapts to your KDE theme automatically
- ⚡ **Native KDE support** — built on StatusNotifierItem via ksni

---

## Requirements

- Linux with KDE Plasma (Wayland or X11)
- `libayatana-appindicator`
- `xdotool`

---

## Installation

### AUR (Arch / CachyOS / Manjaro)

```bash
paru -S tacticaltray-linux
```

### Manual

```bash
git clone https://github.com/Sekain555/tacticaltray-linux.git
cd tacticaltray-linux
cargo build --release
./target/release/tacticaltray-linux
```

---

## Building from source

Requirements: `rust`, `cargo`, `libayatana-appindicator`, `xdotool`

```bash
cargo build --release
```

---

## Credits

Nox and the Nightfall Tactics universe are original creations by [Victor 'Sekain' Sepulveda](https://github.com/Sekain555).

TacticalTray Windows version available at [Sekain555/tacticaltray](https://github.com/Sekain555/tacticaltray).

---

## License

MIT License — Copyright 2026 Victor 'Sekain' Sepulveda
