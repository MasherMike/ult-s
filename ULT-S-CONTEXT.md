# Ultimate S — Repo Context (handoff summary)

Working copy: `D:\ult-s` (user's own fork of chrispo-git's Ultimate S, branch `main`).
Goal of the fork: the user (former Ult S balance voice/contributor) is developing their own version.
Source of truth for move data: **the ACMD in `src/`** (not the generated `changelog/*.md`).
Balance comparisons are made against **vanilla Ultimate 13.0.4** and **current Ult S**.

---

## 1. What the mod is
A large overhaul of Smash Ultimate (game ver **13.0.4 only**) that reworks movesets and general game feel, plus adds a few new characters as costume-slot clones. Two halves:

| Half | Location | Loaded by | Contents |
|---|---|---|---|
| Code plugin | `src/` (Rust, one crate `plugin`, builds a Skyline `.nro`) | Skyline + smashline | Script replacements (ACMD), status logic, per-frame logic, engine hooks, config system, web menus |
| File mod | `romfs/` | ARCropolis | Animations (`.nuanmb`), effects, param overrides (`.prcxml`), UI/text (`.msbt`/`.xmsbt`), layouts, sounds, web-menu HTML |

Runtime dependencies (separate plugins): ARCropolis, smashline, nro_hook, lib_paramconfig, the_csk_collection (CSS/added chars + online hash allowlist), stage_config, one_slot_eff.
Cargo deps of note: `smashline` (HDR), `smash_script` / `smash-rs` (WuBoytH forks), `param_config`, `skyline-web` (feature `main_nro`).

## 2. Entry point (`src/lib.rs`)
`main()`:
1. Loads `sd:/ultimate/ult-s/config.toml` (fails with dialog if malformed).
2. `quick_validate_install()` checks required plugin `.nro`s exist.
3. Allows added-character UI hashes online (rayman, toad, bomberman, peppy).
4. Installs hooks (frame pacing fixes on Switch, version string → "Ver. 13.x / Ultimate S <version>" read from `ultimate/mods/Ultimate S Arcropolis/version.txt` or `Ultimate S Lite`), then each module's `install()`.
`is_on_ryujinx()` detects emulator by text address; emulator paths avoid web dialogs.

## 3. Per-character code
~90 folders `src/<internal_name>/` (internal names: `koopa`=Bowser, `murabito`=Villager, `gaogaen`=Incineroar, `pikmin`=Olimar, `purin`=Jigglypuff, `rockman`=Mega Man, `mariod`=Dr. Mario, `samusd`=Dark Samus, `szerosuit`=ZSS, `eflame/elight` under `src/element`=Pyra/Mythra, `brave`=Hero, `buddy`=Banjo, `dolly`=Terry, `edge`=Sephiroth, `demon`=Kazuya, `trail`=Sora, `pickel`=Steve, `tantan`=Min Min, `jack`=Joker, `packun`=Piranha Plant, `reflet`=Robin, `shizue`=Isabelle, `gekkouga`=Greninja, `krool`=K. Rool, `ptrainer`=PT, `popo/nana`=ICs, `peppy` (via falco), etc.)

Standard layout:
- `mod.rs` — `install()` calls frame/status/acmd installs; also per-character **param tweaks** via `crate::param_cache::update_float_2/update_int_2(kind, slots, (param_hash, index, value))` (e.g. Mario `jump_speed_x_mul = 1.2`).
- `acmd/` — `aerials.rs, ground.rs, tilts.rs, specials.rs, throws.rs, other.rs`. Each registers `Agent::new("<fighter>").set_costume(slots).acmd("game_attackairlw", fn, Priority::Low)…`. `game_*` = hitboxes/timing, `effect_*` = VFX, `sound_*`, `expression_*`. Hitboxes use `macros::ATTACK(...)` with inline comments naming every arg (ID, Part, Bone, Damage, Angle, KBG, FKB, BKB, Size, X/Y/Z, Hitlag, SDI, …).
- `frame.rs` — per-frame (`on_line(Main, ...)`) character logic (cancels, once-per-airtime flags, motion swaps).
- `status.rs` — new/replaced status scripts (pre/main/end).
Character-local state uses `VariableModule` (`src/variable_module`) and `crate::transition_set!/transition_reset!` macros (`src/s_macros`, `src/state_manager`).

### Costume-slot gating (important)
Changes apply **only to costume slots containing a `.marker` file**: `romfs/fighter/<char>/model/body/cXX/<marker>.marker`.
At boot `ModRegistry::fill()` (`src/util/mod.rs` ~line 1000) scans **every** folder in `sd:/ultimate/mods/*/fighter/*/model/body/c*/` for markers. Code uses `get_marked_costumes(char_folder, marker)`, `get_lowest_marked_costume`, `get_costume_count` to attach scripts/params to those slots. This is how the Setup tool enables/disables characters and how skins coexist.

Clone characters built on a base fighter's slots (distinguished by marker name):
| New char | Base fighter folder |
|---|---|
| Peppy | `falco` |
| Toad | `murabito` |
| Bomberman | `pacman` |
| Rayman | `pikmin` |
Other prebuilt markers/content: `bandana`, `funky`, `maskedman`, `sandbag` (`romfs/prebuilt/`).
Each fighter's `romfs/fighter/<char>/config.json` is an ARCropolis config (`new-dir-files` etc.) adding per-slot animation files.

## 4. System-wide mechanics
### `src/common/` (runs for every fighter via `Agent::new("fighter").on_line(Main, common)`)
- `hitstun.rs` — KD-throw handling, non-tumble DI (re-applies `correctDamageVector`), optional drift DI. *Known issue:* in the `ftStatusUniqProcessDamage_exec*` hooks, `if no_tumble_di != 0 { ret; }` doesn't return, so the toggle doesn't disable the correction.
- `cancel.rs` — jump-cancel table (`JC_LIST`: fighter, status, hit condition, frame window, slot range) e.g. Fox/Wolf shine, Falco/Peppy shine F4–32, Corrin neutral-B hold, Gunner LW1.
- `dacus.rs` — dash attack → up/down smash cancel; per-fighter frame windows (6/8/10/11/12/14/16).
- `landing.rs` — LLPC (landing-lag platform cancel), shield drop, endlag (crouch) drop, respawn wakeup anim, edge cancel table.
- `movement.rs` — perfect pivot, moonwalk, JC grab, DJC, hold-buffer killer, dash behavior, parry-cancel dash, footstool/tread jump overrides.
- `wavedash.rs` — wavedash/airdodge, per-fighter WD length/traction category, status change hooks.
- `jab.rs` — jab cancel behavior; `projectile_invuln.rs`; `bone.rs` (bone rotation, sword sizing); `remove_quake.rs`; `css.rs` (CSS hooks + web menus); `gamemodes/` (turbo, vampire, fgmode, sixtyfour, superboss, angles, pogo, fly, itemduel, effects, smym, crit).
- `src/cpu/` — CPU damage-state hooks (installed when `common` NRO loads).

### Global params
`romfs/fighter/common/param/common.prcxml` e.g. `precede`=7 (buffer), `shield_just_frame`=6 (parry), `dash_speed_keep_frame`=3, `turn_dash_frame`=-1, `escape_penalty_*` (no dodge staleness), `damage_fly_correction_max`=10, freeze tuning. Some fighters have `romfs/fighter/<char>/param/vl.prcxml`.
`src/param_cache` also supports **global multipliers** by hooking `get_param_int/float` (`set_global_multiplier(type, name, mult)`).

### User config / presets
- File: `sd:/ultimate/ult-s/config.toml`; presets in `sd:/ultimate/ult-s/presets/*.toml` (repo: `presets/ult-s.toml, vanilla.toml, rivals.toml, sm4sh.toml, chrispo-goofy.toml`).
- Struct in `src/config/mod.rs`; applied every frame in `src/config_apply/{general,movement,attacks,defense,special}.rs` and also checked inside `src/common/*`.
- Convention: **0 = Ult S default**, nonzero = alternatives (vanilla preset is mostly 1). Some options are opt-in (e.g. `drift_di = 1` enables).
- Sections & keys:
  - general: sh_macro, hold_buffer, hitstun (1=×1.5, 2=×0.75), hitlag, ledges, respawn_anim
  - movement: dash, moonwalk, pivots, llpc, jump_accel, hitfall, agt, djc, footstool, edge_cancel, jump_cancel, g2a
  - attacks: jab_cancel, dacus, ac, jcg, grab, lcancel, special_cancel, damage_mul
  - defense: shield, shield_health, shieldstun, airdodge, shield_drop, di, no_tumble_di, drift_di, hitstun_cancel, vertical_gravity
  - stats: ll, initial_dash, runspeed, walkspeed, airspeed, airaccel, traction, fallspeed, jump, gravity, weight
  - special: cancel_to_taunt, parry_reflect, taunt_cancel, no_special_fall, random_trip
- Edited in-game through a skyline-web menu (`romfs/ui/docs/menu.html, menu.js, toggles.js, gridmenu.js, gamemodes.html`), copied into `atmosphere/contents/<titleid>/manual_html/html-document/contents.htdocs`. On emulator, the button cycles presets instead (opt-in web menus with `ultimate/ult-s/web_menus.flag`). Gamemode defaults: `ultimate/ult-s/gamemode-default.txt`.

## 5. Text / UI files (relevant to text replacement work)
- `romfs/ui/message/`: `msg_melee.msbt`, `msg_tips.msbt` (binary), and **`msg_menu.xmsbt`, `msg_name.xmsbt`, `msg_tips.xmsbt`**: ARCropolis XML text patches.
- `.xmsbt` format: **UTF-16 LE with BOM**, `<?xml version="1.0" encoding="utf-16"?><xmsbt><entry label="..."><text>...</text></entry>…</xmsbt>`. Tip titles use inline control codes written as escaped chars (e.g. `&#xE;&#x1;&#x7;&#x4;\0&#x1;…`). Labels examples: `tip_title_0081`, `tip_text_0095`, `mnu_opt_btn_key_short_hop`, `mnu_opt_btn_key_appeal_up` ("Up Taunt/Footstool").
- `Labels.txt` (repo root, ~800KB) is a hash label list; `yamlist(.exe/.py)` converts motion_list.bin ↔ yaml.
- UI layouts: `romfs/ui/layout/info/*`, `romfs/ui/layout/menu/chara_select*`.

## 6. Tooling (`scripts/`)
- Build/package: `build.py` (+ `build_release/lite/dev/char/small/diff/git.py`, `merge_config.py`), `.github/workflows/build_full.yml, release.yml`.
- Data docs: `statistics.py` → `output.md` → `changelog/<char>.md` (needs vanilla dumped scripts from WuBoytH's repo in `scripts/vanilla_scripts/`); `stats2md.py`, `stat_ranker.py`.
- Helpers: `add-markers*.py`, `c00clone.py`, `batch_add_priority.py`, `to_rust.py`, `to_toml.py`, `vl_to_toml.py`, `sm4shconvert.py`, `export_yamlist.py`.
- `changelog/` has 47 character docs (attributes table + per-move frame/hitbox tables + FaF).

## 7. Working agreements for the balance chat
- Compare vanilla 13.0.4 vs current Ult S; read values from ACMD.
- Balance assumes the **default Ult S preset** (all config values 0); we evaluate and adjust the mod's own balance.
- Breakdown format per character: **frame data → design note → suggested change**.
- Direction comes from discovery, not a pre-made list.
