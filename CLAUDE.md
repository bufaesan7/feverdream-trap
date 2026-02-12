# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Feverdream Trap is a 3D game built with **Bevy 0.18** (Rust, edition 2024). The workspace has three crates:

- **`core/`** — Shared library: chunk/level systems, custom RON asset loaders, physics layers, audio, UI theme
- **`game/`** — Main game: screens, menus, character/camera controllers, scene management
- **`editor/`** — Level editor with egui dock UI, 3D viewport, and inspector

## Build & Run

Uses **Bevy CLI** for running, **Cargo** for everything else. Rust toolchain: nightly.

```bash
bevy run                  # Run game (native dev, default)
bevy run --release        # Run game (release)
bevy run --yes web        # Run game (web)
cargo run -p feverdream_trap_editor  # Run editor

cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --profile ci --all-features
cargo test --locked --workspace --all-targets --profile ci --no-fail-fast
```

## Architecture

Every subsystem is a Bevy plugin. Systems are ordered via `AppSystems` sets: `TickTimers → RecordInput → Update`.

**Game states:** `Screen` enum (Splash → Title → Loading → Gameplay) drives transitions. `Pause` state handles pause logic.

**Custom asset pipeline:** RON-based assets at three levels:
- `.chunk.element` — Individual 3D primitives (Plane, Cube, Sphere, GLTF)
- `.chunk.descriptor` — Collections of chunk elements
- `.chunk.layout` — Level layout referencing descriptors

Loaded via generic `RonAssetLoader<T>` for types implementing `RonAsset`.

**Physics:** Avian3D with three collision layers (Default, Player, Sensor) in `core/src/physics.rs`.

**Player:** Kinematic character controller via `bevy_ahoy` + `bevy_enhanced_input` for input.

**Editor:** `egui_dock` for dockable panels, `bevy-inspector-egui` for entity inspection.

## Conventions

- `clippy::too_many_arguments` and `clippy::type_complexity` are allowed (Bevy ECS convention)
- Dev builds use dynamic linking and asset hot-reloading (`dev_native` feature, on by default)
- Feature `dev` enables dev tools; assets live in `assets/`
