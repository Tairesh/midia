# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Followers of the Midia — a fantasy roguelike in Rust using the Tetra 0.9 game engine, inspired by a Savage Worlds
tabletop RPG campaign.

## Build & Development Commands

Uses **nightly Rust toolchain** exclusively. Common commands via `just`:

- `just check` — full CI check (fmt + cargo check + test + clippy)
- `just before-commit` — auto-fix + local check
- `just fix` — auto-fix lint issues and format code
- `just test` — run tests
- `just clippy` — clippy with pedantic lints and specific cast allowances
- `cargo +nightly test -- test_name` — run a single test

Clippy is configured with `-D clippy::pedantic` plus allowances for cast operations, `module_name_repetitions`, and `unnecessary_box_returns`.

## Architecture

### App Loop

`main.rs` → `window::create_window()` → `Tetra::run(App::new)`. The `App` struct implements Tetra's `State` trait and
manages a **scene stack** (push/pop/switch). Each frame: `update()` → `draw()` → `event()` on the top scene.

### Scene System (`src/scenes/`)

`Scene` trait with `update() → Transition`, `draw()`, `event()`. `SceneKind` enum is a factory for scene creation.
`Transition` enum controls flow: `Push`, `Pop`, `Switch`, `ExitToMainMenu`, `Quit`.

### Game Modes (`src/scenes/game_modes/`)

Within the Game scene, an `#[enum_dispatch]` `GameMode` enum handles input modes: Walking, Observing, MeleeAttack,
Shooting, Throwing, etc. Each mode defines cursors, validation, and update logic.

### World State (`src/game/world.rs`)

Central `World` struct holding: `meta` (tick counter), `units` (all actors), `map` (chunked terrain), `fov` (field of
view), `log` (game events), `game_view` (player vision).

### Action System (`src/game/actions/`)

Turn-based with tick-based timing. `ActionType` is an `#[enum_dispatch]` enum (Walk, Melee, Shoot, Throw, Reload, etc.).
Each action implements `is_possible() → ActionPossibility`, `on_start()`, `on_step()`, `on_finish()`. Actions are
created with a tick length and finish at `current_tick + length`.

### Unit Polymorphism (`src/game/units/`)

`Avatar` trait with `#[typetag::serde]` enables serializable trait objects (`Box<dyn Avatar>`). Implementations:
`Player`, `Monster`. The `Units` struct stores them in a `HashMap<usize, Box<dyn Avatar>>` with a "loaded bubble" (
128-tile radius) for active AI.

### Combat (`src/game/savage/`)

Savage Worlds-inspired: `CharSheet` (attributes, skills, wounds), dice mechanics (`Dice`, `DiceStack`, `RollResult`),
damage types, hit resolution.

### Map (`src/game/map/`)

Chunked lazy-loaded terrain with Perlin noise generation. Tiles contain terrain, items (with containers), and visibility
state.

### Settings (`src/settings/`)

Singleton via `OnceCell<Mutex<Settings>>`, auto-saves on drop, JSON serialization to `settings.json`.

## Key Patterns

- **`#[enum_dispatch]`** for zero-cost polymorphism on `ActionType` and `GameMode`
- **`#[typetag::serde]`** for serializable trait objects (`Avatar`)
- **Scene stack** with `Transition` enum for state management
- **UI**: `UiSprite` trait objects with `ButtonBuilder` and `Layout` anchoring
- **Data-driven items**: `ItemPrototype` definitions with materials and qualities

## Commit Style

Emoji prefixes: `:sparkles:` (new feature), `:bug:` (fix), `:recycle:` (refactor), `:arrow_up:` (dependency bump), etc.
