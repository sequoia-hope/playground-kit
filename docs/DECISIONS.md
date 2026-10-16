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
