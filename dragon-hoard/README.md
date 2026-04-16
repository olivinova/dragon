
# Dragon Hoard

Small incremental web game in Rust + Yew. You are a dragon growing your hoard of gold and unlocking mechanics.

## Prerequisites

- Rust toolchain (install via rustup). See https://rustup.rs for installer.
- Add the WebAssembly target: `rustup target add wasm32-unknown-unknown`
- `trunk` (dev server / bundler) — install with `cargo install --locked trunk`

## Quick run (development)

```powershell
cd dragon-hoard
trunk serve --open
```

## Build for deployment

```powershell
cd dragon-hoard
trunk build --release
# Output will be in `dist/` or as configured in `trunk.toml`.
```

## Project structure

- `src/lib.rs` — application entry point, renders the `App` component.
- `src/app.rs` — main Yew component, UI layout, global state hooks, tab selection, and callbacks.
- `src/game.rs` — game state model, saving/loading, tick updates, resource and kobold mechanics.
- `src/hoard_tab.rs` — UI for the `Hoard` tab.
- `src/kobolds_tab.rs` — UI for the `Kobolds` tab.
- `src/enchantments_tab.rs` — UI for the `Enchantments` tab.
- `src/adventure_tab.rs` — UI for the `Adventure` tab.
- `static/style.css` — styling for the game interface.

## Tabs and UI

- `Hoard` tab: training claws and unlocking the treasure vault.
- `Kobolds` tab: recruit kobolds, assign mining/farming/digging, and upgrade efficiency.
- `Enchantments` tab: study magic, craft enchantments, and sell items.
- `Adventure` tab: conquer towns and explore dungeons.

The resource sidebar is intentionally separated from the tab panel so more categories can be added later without changing the core layout.

## Development notes

- The UI has been split so each tab is contained in its own component file, improving readability and maintainability.
- Game state is stored in `GameState` and persisted to browser `localStorage` under the key `dragon_hoard_save`.
- `GameState` now supports generic track adjustments through `GameTrack`, `add_track`, `subtract_track`, and `adjust_track`.
- A new generic property layer exposes optional `current`, `per_second`, `capacity`, and `modifier` values for each track.

## TODO / future improvements

Implement magic research system to allow for specialization and character building.
Change enchantments to provide bonuses outside of 'combat strength'.
Create a system that allows for conquering more towns and that giving you a larger cap for total space allowed.


## Troubleshooting & common installation steps (Windows / PowerShell)

- If `rustup` or `cargo` is not recognized:
  - Install Rust from https://rustup.rs and follow the Windows installer instructions.
  - Or use winget: `winget install --id Rustlang.Rust -e` then restart your shell.
  - After install, restart PowerShell so `cargo` and `rustup` are on `PATH`.

- Add the wasm target if missing:

```powershell
rustup target add wasm32-unknown-unknown
```

- Install Trunk (requires `cargo`):

```powershell
cargo install --locked trunk
```

- If `trunk` is not found after install, ensure Cargo's `bin` (usually `%USERPROFILE%\.cargo\bin`) is on your `PATH` and restart the shell.

- If the build fails with dependency or feature errors:
  - Run `cargo update` in the project directory.
  - Check the `Cargo.toml` versions and update as needed.

- If the site does not open automatically, visit `http://127.0.0.1:8080` (default) in your browser.

- LocalStorage saves progress under key `dragon_hoard_save`. Use the browser DevTools Application tab to inspect or clear it.
