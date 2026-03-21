<div align="center">

# 🌃 Metropolis
### *The City Powered by Your Kernel*

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.1-neon.svg?style=for-the-badge)](https://github.com/5c0/metropolis/releases)
[![AUR](https://img.shields.io/badge/AUR-metropolis-deepskyblue.svg?style=for-the-badge&logo=arch-linux)](https://aur.archlinux.org/packages/metropolis)

**"The year is 20XX. Your processes aren't just rows in a table. They're the residents. Your CPU isn't just silicon. It's the infrastructure. And the city? The city is breathing."**

[Features](#-the-simulation) • [Usage](#-installation) • [Controls](#-controls) • [Architecture](#-built-with)

</div>

---

**Metropolis** is a high-performance, terminal system monitor that turns your kernel metrics into a living, breathing **Cyberpunk Skyline**. 

Your hardware drives the simulation in real-time: The city brands itself with your OS logo,  high CPU usage is rush hour, disk I/O is industrial freight, top processes become neon signs, and your RAM usage lights up the skyline.

## Demos

### 🌃 Night
![Metropolis demo](docs/clean.gif)

### ☔ Rain
![Metropolis rain demo](docs/rain.gif)

### ❄️ Snow
![Metropolis snow demo](docs/snow.gif)

---

## Features

- **Dynamic Branding**: The central building, where most of the residents go in for work, automatically detects your OS (Arch, Fedora, Ubuntu, etc.) and brands itself accordingly.
- **CPU (The Heartbeat)**: High utility triggers "Rush Hour"—flooding sky-lanes with high-speed traffic and increasing pedestrian density.
- **Disk I/O (Logistics)**: Intense activity triggers **Heavy Industrial Shuttles**. Watch long-haulers move "physical data" across the district.
- **RAM (Illumination)**: Memory usage dictates the overall occupancy and glow of the city's monoliths.
- **Neon Signage**: Secondary buildings display your **Top CPU Processes** as vibrant neon signs.
- **Dynamic Pursuits**: Random high-stakes police chases streak across the skyline. Look for the **Red Fugitive** and **Interceptor** units.
- **Procedural Night**: Window patterns and traffic cycles are session-unique.

---

## Installation

### One-Line Install (Linux / macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/5c0/metropolis/main/install.sh | bash
```

### Crates.io
```bash
cargo install metropolis-tui
```

### Arch Linux (AUR)
```bash
yay -S metropolis
```

### Windows (winget)
```bash
winget install 5c0.Metropolis
```

### Nix / NixOS (Flake)
Run directly without installing:
```bash
nix run github:5c0/metropolis
```

Or add to your `flake.nix` inputs:
```nix
inputs.metropolis.url = "github:5c0/metropolis";
```
Then add to `environment.systemPackages`:
```nix
environment.systemPackages = [
  inputs.metropolis.packages.${system}.default
];
```

### Build from Source
*Requires [Rust](https://www.rust-lang.org/) and `cargo` to be installed.*
```bash
git clone https://github.com/5c0/metropolis.git
cd metropolis
cargo run --release
```
---

## Controls

| Key | Action |
|:---:|:---|
| `q` | **Escape the city** (Quit) |
| `r` | **Atmospheric Shift** (Toggle Rain) |
| `s` | **Cryo Shift** (Toggle Snow) |
| `d` | **Core Diagnostics** (Debug Overlay) |

---

## Built With

- **[Rust](https://www.rust-lang.org/)**: For sub-millisecond, zero-overhead rendering.
- **[Ratatui](https://ratatui.rs/)**: The backbone of our terminal metropolis.
- **[Sysinfo](https://github.com/GuillaumeGomez/sysinfo)**: Our direct link to the kernel.

---

<div align="center">

### Connect with the Grid
*Metropolis is more than a monitor. It's the vision of what your hardware sees when you're not looking.*

[Star this repo](https://github.com/5c0/metropolis/stargazers) if you want to see more districts added.

</div>
