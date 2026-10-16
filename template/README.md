# A new game on Playground Kit: the template

The skeleton of a game that builds on the kit: `cargo xtask` on
`pk_xtask`, the loader page, the dev server, CI and Pages. Nothing in it
names a game: the places to change are marked.

## Instantiate

1. Copy this directory beside the kit, so the game sits at
   `../<game>` and the kit at `../game-engine`:

       cp -r ../game-engine/template ../<game> && cd ../<game> && git init

2. Register it with the port registry (`proj add <game> .`), which gives
   it a port; `serve.sh` asks for it, never stores it.
3. Write the client crate, `crates/game/` (`pk_shell` gives the plugins,
   status, bridge, store and web glue; the crate exports `gesture`,
   `gesture_at`, `run`, `screenshot` and `stage` for the page), and the
   simulation crate as a `pk_lockstep::Simulation`. Add each crate's rule
   to `deps.toml`.
4. Rename `game` where it appears: `xtask/src/main.rs` (`crate_name`,
   the out-names, `page_dir`), `web/index.html` (the three BRANDING blocks
   and `OUT_NAME`), `tools/serve.py` (`TITLE`, `PINNED`), `CLAUDE.md`,
   `.github/workflows/ci.yml` (the `-p game` lines) and `docs/DECISIONS.md`.
   `web/` goes to `crates/game/web/` (or point `page_dir` elsewhere).
5. `npm init -y && npm install --save-dev puppeteer-core` for the kit's
   headless-Chrome tools (`../game-engine/js/README.md`).
6. `cargo xtask web --release`, `proj up <game>`, open `/`.

## What is where

| File | What it is |
|---|---|
| `xtask/` | the `WebBuild` config and `pk_xtask::main_with`; add the game's own commands there |
| `web/index.html` | the loader page: backend pick (WebGPU, else WebGL2), download progress, gesture bridge, crash screen, `?stats=1`, `?perf=1`; branding in three marked blocks, the status global's name (`GLOBAL`) in one place, game hooks in one object |
| `web/perf.js` | the on-device measurement behind `?perf=1`, read by `js/perf.mjs` |
| `tools/serve.py`, `serve.sh` | the dev server on the registered port: the build over the root, no-store, precompressed files, a "not built yet" page |
| `.github/workflows/` | CI (fmt, clippy native and per wasm backend, tests, check-deps, the lavapipe smoke test, web build, size budget) and Pages, both cloning the kit beside the game |
| `Cargo.toml`, `.cargo/`, `rust-toolchain.toml` | the workspace: the `pk_*` path dependencies, the profiles, the kit's `[patch.crates-io]` block, the `cargo xtask` alias |
| `deps.toml` | the crate dependency rules `cargo xtask check-deps` enforces |
| `CLAUDE.md`, `docs/DECISIONS.md` | the working rules for agents and the decisions log |

The status contract (`window.__game`: `state`, `frames`, `ready`,
`backend`, `error`, `shot`, `wasmMemoryBytes`, `stage`) is documented in
`web/index.html`; the kit's `pk_shell` publishes it and the kit's `js/`
tools read it.

A `cargo generate` layer (`{{project-name}}` substitution) is not part of
this template yet; the rename list above is short enough by hand.
