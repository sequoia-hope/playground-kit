# Decisions

Choices the plan (`PLAN.md`) did not cover, numbered, newest at the end.
Each says what was chosen, why, and what it changes.

## D1 (2026-10-16). Name, layout, licence, policy, pacing

The owner's answers to PLAN.md section 9:

- **Playground Kit**, crates `pk_*`; the GitHub repository is
  `sequoia-hope/playground-kit`. The local directory and the `proj`
  registry name stay `game-engine` (port 8046), as registered.
- The games depend on the kit by **sibling path**
  (`../game-engine/crates/pk_*`), and point their `[patch.crates-io]` at
  `../game-engine/patches/*`. Their CI clones the kit beside the game
  (`git clone ... ../game-engine`) before building.
- **MIT OR Apache-2.0** for the kit and for Midnight Playground, which had
  no LICENSE file.
- **Own code over third-party Bevy plugins** for input, widgets and
  persistence stands (Midnight's SPEC 2 policy); each kit crate's README
  names the crate it chose not to use and why (`docs/ECOSYSTEM.md`).
- Phases 2 to 5 run **autonomously**, pausing at each seam's design note
  in `docs/design/` for the owner's yes.

## D2 (2026-10-16). The patches live here

`patches/wgpu`, `patches/wgpu-hal` and `patches/bevy_ui_render` moved from
the games (they were byte-identical in both). The kit's own workspace
patches them too, so its Bevy crates build against the same wgpu. Each
`PATCHED.md` keeps the change list; `docs/ECOSYSTEM.md` section 3 names
the upstream issue or PR each one waits on. The games delete their copies
once their `[patch.crates-io]` points here.

## D3 (2026-10-16). pk_math

Midnight Playground's `mp_math` moved here whole (PLAN.md section 4,
layer 0), API unchanged but for the crate name: `kernel` (libm with
ECMAScript's `pow` and `hypot` edges), `Rng`/`Mulberry32`, `hash2`,
`Noise2D`/`fbm`/`ridged`, the small helpers, and its four test files. The
game's D1231 is the other side.

- **`js` is a cargo feature**, on by default for now because the game
  needs it everywhere. It is for ports of JavaScript code: `Math.round`,
  `Math.sign`'s zero, `max`/`min` with NaN and signed-zero ordering, `||`,
  ToInt32, `imul`, `fround`. The crate uses a few of them internally, so
  the module is always compiled; the feature decides only whether it is
  public. The two tests that exercise it carry `required-features`.
- **`vec` is `mp_vdyn::math`** (`Vec3`, `Quat`, `Iso3`, `Aabb` in `f64`),
  moved because it met the test: exact operations only, the one inexact
  step (a half-angle) through the kernel. The names stay `Vec3`/`Quat`
  rather than the plan's `DVec3`/`DQuat`: there is no `f32` sibling to
  tell them from, and the game's vehicle dynamics keep compiling unchanged.
  No glam: it does not promise bit-identical results across platforms
  (ECOSYSTEM.md row 13), and these are a few hundred lines.
- **`libm` is pinned exactly** (`=0.2.16`), as the game pinned it: a new
  libm may round differently from the goldens the games have recorded.
- **The golden is a copy.** `tests/golden/math.json` is the game's
  `parity/golden/math/math.json`, written by its JS oracle with the kernel
  on; the test compiles it in, so it runs in wasm too. The game's tool
  refreshes the copy when it is rerun.
- **Not moved**: the kernel bit check (`cargo xtask kernel --check`) and
  `mr_kernel`, the wasm the JS oracle loads. They are the game's parity
  tooling; `mr_kernel` now depends on `pk_math`. PLAN.md said the bit test
  moves to `pk_xtask`; that waits until a second game has a JS oracle.
- **Gates**: `cargo test -p pk_math` natively and under Node for
  `wasm32-unknown-unknown`, clippy with and without the `js` feature, all
  in this workspace once `pk_xtask` landed beside it the same afternoon.

## D4 (2026-10-16). pk_canvas

Midnight Playground's `mp_canvas` moved here as `pk_canvas`, its code
unchanged except for where fonts come from: the crate's `build.rs` baked
the game's `assets/fonts/fonts.json` (1.3 MB of font files) into the
library, and a library cannot carry one game's fonts.

- **Fonts at run time, no bundled set.** `FontBook` (which already
  existed: `add`, `set_generic`, `add_fallback`, `select`) is the only way
  faces reach a canvas; `FontBook::bundled()` is gone. `Canvas::new(w, h)`
  has an empty book (text draws nothing, measures zero) and
  `Canvas::with_fonts(w, h, Arc<FontBook>)` draws text. There is no
  `bundled-fonts` cargo feature: a game bakes its own manifest with
  `include_bytes!` and builds one `Arc<FontBook>` (Midnight does it in
  `mp_worldgen::fonts`, moving the old `build.rs` there whole, so its
  manifest stays the one copy of the mapping, its D373). An env-var-driven
  bake in the kit would have been a second mechanism for the same bytes
  and a build script in a library.
- **`add` keeps the weight range.** `add(family, data, (min, max),
  italic, file)` rather than a single weight: a variable font's `wght` axis
  follows the request within the range, and a single-weight face given the
  whole range is used for every weight, which is how Midnight pins "Arial
  Black" to Roboto at 900 without synthetic bold.
- **The Chrome-parity tests went with the fonts.** `measureText` and
  baseline numbers, the probes sheet and the font gallery need the game's
  faces and goldens, so they are `mp_worldgen/tests/canvas_text.rs`,
  `tests/canvas_probes.rs` and `examples/font_gallery.rs` in Midnight. The
  kit keeps `tests/pixels.rs` (the semantics that need no reference image)
  with one OFL face under `tests/fonts/` for the text cases.
- **Nothing in the shaping or rasterising path changed**: Midnight's
  texture goldens and probes pass bit for bit against the kit crate.

## D6 (2026-10-16). pk_lockstep: the Simulation seam, designed and written, not yet wired

The first seam of PLAN.md section 5, as `crates/pk_lockstep` and the design
note `docs/design/simulation.md`. Written from Midnight Playground's
`mp_sim`, `mp_net::{client, host, proto}` and `mp_game::play::session`, and
snowboarding's `sb_sim`, without touching either game: the owner reads the
note before phase 2 wires Midnight onto it (D1's pause).

- **Static methods on a marker type**, not `&self`: `Simulation::step(ctx,
  &mut state, inputs, events)`. Both games already pass the immutable part
  explicitly (`&LevelRuntime`) or will (snowboarding's `Course` out of its
  `Session`), the netcode holds the context in an `Arc` and the state in a
  ring, and nothing has to be constructed to use the trait.
- **`tick(&State)` is on the trait; `players` is not.** The rollback ring,
  the host's log and the recorder all read the tick off the state. The
  number of inputs per tick comes from the lobby (the netcode) or the file
  (the recording), as it does today, so the trait does not ask the state.
- **`NetSim` is exactly the host's and client's three guesses**: `predict`
  (a missing input is the last one without its edges), `absent` (a
  dropped player, given their last input), `accept` (what the host takes
  from a client; strips flags only the host may set). Plus a fixed
  `INPUT_BYTES` codec so a message's frame count is checked against its
  length before anything is allocated, as `mp_net::proto` does.
- **`SnapshotSim` is dependency-free**: the game writes the bytes (the
  `Bytes` writer helps) or uses serde behind its own feature. A `serde`
  feature in the kit that derives it is the next step, not this one
  (RECOMMENDATIONS.md item 2, phase 3).
- **`Recording` is a new binary format** (`PKREC`, run-length by whole
  ticks, hash checkpoints, an opaque setup blob). Midnight's JSON-lines
  recording and `mp-sim replay` are untouched and stay byte-identical; a
  converter from `mp_sim::replay::Recorded` lands with the wiring.
- **`fnv1a64` and the stream seeding rule moved as they are**, with the
  published FNV test vectors and Midnight's seeds pinned by tests, so every
  hash and every race the game has recorded still matches.
- **The frame arithmetic is a free function**, `advance(acc, frame_dt, dt,
  max_ticks, max_frame)`, so `pk_play::Session` and a headless runner share
  one definition; it adds the NaN-frame guard `session.rs` lacks.
- **Still game-side**: the per-tick sampler (assists and autopilot between
  `ActionState` and `quantise`), the trace record format and its `Writer`,
  `Input` versus `InputFrame`.

## D5 (2026-10-16). pk_xtask, the template and the js tools

Phase 1's tooling layer (PLAN.md section 4, layer 2), lifted from
Midnight Playground's `xtask/`, `crates/mp_game/web/`, `tools/serve.py`
and `tools/parity/`, with snowboarding's page and server improvements
read in.

- **`pk_xtask`** is a library: `main_with(args, &[Command])` runs the
  built-in `check-deps` and `imgdiff` plus the commands given;
  `web::commands(&'static WebBuild)` gives `web`, `prof` and `size`. A
  game's `xtask/src/main.rs` is a `WebBuild` and these calls
  (`template/xtask`). `WebBuild { crate_name, backends: &[Backend { name,
  out_name, target_dir, cfg }], page_dir, dist_dir, runtime_files {
  required, optional }, side_wasm: &[SideWasm { krate, features,
  opt_level, shim_js, what }], build_json_extra, wasm_opt, size_budget_gzip
  }`. The workspace root comes from `cargo locate-project --workspace`, so
  the library needs no `CARGO_MANIFEST_DIR` of its own. Pinned versions
  are checked in one way: wasm-bindgen's against `Cargo.lock`, binaryen's
  against `WebBuild::wasm_opt` (new; the games' CI installs 116);
  `build.json` records it. The WebGL2 cfg is the game's (`mp_webgl2` in
  Midnight); the template uses `pk_webgl2`, which the kit's own Bevy
  crates will read.
- **`deps.toml`** replaces the rules table in code: named `[banned]` sets,
  one `[crates.<name>]` entry per workspace member (`allowed`, `banned` as
  a set name or list, `pure`, `exact_math`). The engine is the one
  Midnight had (cargo tree over all targets, the source scans for
  `forbid(unsafe_code)`, clock, threads, hash maps and the 27 inexact
  float methods). The kit's own `deps.toml` rules pk_math, pk_lockstep,
  pk_canvas, pk_xtask and pk_shell; `cargo xtask check-deps` runs in CI.
- **`imgdiff`** is the screenshot metric alone (CIEDE2000 at quarter
  resolution in linear light, mean and 95th-percentile 16-px block, the
  HTML report and `summary.json`); the station and material orchestration
  stayed in Midnight.
- **The `__game` status contract** (PLAN.md section 5, item 7) is written
  down in `template/web/index.html`: `state` ('boot' | 'loading' |
  'building' | 'warming' | 'running' | 'failed' | 'crashed' | 'nogpu'),
  `progress`, `frames`, `ready`, `backend`, `error`, `shot`,
  `wasmMemoryBytes()`, `stage(cmd)`, `screen`; the page adds `startMs`,
  `adapter`, `webgpu`, `secure`, `fallback`, `touch`, `insets`, `wasmMB`,
  `footer`, `stats`, `screenshot`, `cover`, `reload`. The global's name is
  spelled once in the page (`GLOBAL`) and every tool takes `--global`
  (default `__game`). Midnight still publishes `__mp` (its page sets
  `GLOBAL = '__mp'`) until the shell carve renames it with an `__mp`
  alias.
- **The template page** merges snowboarding's crash screen, its respect of
  `build.json`'s `backends`, and its tolerance of winit's control-flow
  throw, into Midnight's loader (adapter retried four times, the
  `?backend=` forcing, the WebGL2 reload on a WebGPU failure before the
  first frame, the no-GPU screen naming a non-secure context, streaming
  download progress that knows a precompressed Content-Length is not the
  file's size, the gesture bridge, the inset probe, touch detection, the
  stats panel). A game fills three marked branding blocks and one hooks
  object (`buildLine`, `glue`, `load`, `showBuild`, `statsLine`); Midnight
  generates its page from the template with a script rather than editing
  the copy.
- **`js/`**: `server.mjs` (address, flags, `CHROME_PATH`, puppeteer-core
  resolved from the game's `node_modules` since the kit has none),
  `harness.mjs` (interception, devices, `Game`; the game subclasses it for
  its selectors), `perf.mjs`, `prof.mjs`, `gl-calls.mjs`, `build-diff.mjs`
  with `png.mjs`, `console.mjs`, `shot.mjs` and the runner half of
  `stress.mjs` (from snowboarding; the step catalogue is a parameter).
  Each exports `run({ args, defaults, root })` and runs as a CLI; a game
  keeps thin wrappers with its defaults. The JS-game comparison modes
  were dropped. A game needs only `puppeteer-core` in its `package.json`.
- **`template/`** also carries the dev server (the "not built yet" page
  from snowboarding), `serve.sh` (port from `$PORT` then `proj port`, the
  tailscale path from the directory name), CI with the lavapipe smoke
  test and the clone-the-kit step, Pages, the toolchain, the cargo
  config, the workspace with the `[patch.crates-io]` block, `deps.toml`,
  `CLAUDE.md` and `DECISIONS.md` stubs. The client crate itself is not in
  the template yet: it waits for `pk_shell` (the README says what the
  crate must export for the page).
