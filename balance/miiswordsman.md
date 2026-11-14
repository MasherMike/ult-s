# Mii Swordfighter: Ult S vs vanilla

- **Ult S source:** `src/miiswordsman/` (ACMD, `frame.rs`, `status.rs`, `mod.rs` params). New animations in `romfs/fighter/miiswordsman/motion/body/`: ftilt (all 3 angles), fsmash, usmash (+charge), neutral-B 2/3, side-B 1 (all states), up throw.
- **Vanilla sources:**
  - Scripts: WuBoytH/SSBU-Dumped-Scripts, `smashline/miiswordsman/*` (13.0.4).
  - Stats and startup: ultimateframedata.com. Its landing-lag column didn't parse reliably, so vanilla landing lag isn't checked.
- **Frame numbering:** frames are script frames, which match ultimateframedata's numbering. Frames marked ~ were converted by hand from `FT_MOTION_RATE`. None of this has been tested in game.
- **Unchanged from vanilla:** dash attack, utilt, dsmash, dair, pummel, bthrow, side-B 3 (Chakram) moves (only its projectile params change), up-B 2 (Skyward Slash Dash), all down-Bs.

## Gating bug (fix first)
Every Swordfighter `install()` (`frame.rs`, `status.rs`, and all six `acmd/*.rs`) starts with:
```rust
if get_costume_count("miigunner","miigunner") < 1 { return; }
```
That checks **Gunner's** marker. If a player enables Swordfighter but disables Gunner in the Setup tool, none of Swordfighter's changes install. And Swordfighter's own marker is never checked. **Fix:** change it to `("miiswordsman","miiswordsman")` in all 8 places.

Like Gunner, the agents are hardcoded to slots `[0..7]` and the params to `[0..12]`, so every Swordfighter slot is Ult S once it's enabled.

## Possible param bug
In `mod.rs`, every weapon param, including `param_shuriken.life` and the 5 `param_tornado.*` values, is set on `-WEAPON_KIND_MIISWORDSMAN_CHAKRAM`. Param overrides are matched by object kind, so the shuriken and tornado values probably never apply. They should probably use `WEAPON_KIND_MIISWORDSMAN_LIGHTSHURIKEN` and `WEAPON_KIND_MIISWORDSMAN_TORNADOSHOT`. The shuriken one doesn't matter, because Ult S neutral-B 2 no longer spawns a shuriken. The tornado ones do: a speed of 1.6/2.0, a 50-frame lifetime and no pierce were presumably meant to apply. Confirm in game whether the tornado stops at the first target it hits.

## Overall
- **Kit direction:** Ult S makes her a faster, sword-reach rushdown character.
  - Most normals get bigger sword hitboxes that follow the blade (`haver` bone), and a longer grab.
  - Her ground speed goes up.
  - Three specials are replaced: Shuriken becomes a close-range burst, Blurring Blade becomes a dragging air-grab attack, and Airborne Assault becomes a lunge with a grounded follow-up.
  - Gale Strike comes out much faster but has a cooldown.
- **What she loses:** uair damage and size, and both the Shuriken and Blurring Blade projectile and charge play.

---

## Attributes (`mod.rs`)

| Stat | Vanilla | Ult S |
|---|---|---|
| Initial dash | 1.74 | **1.82** |
| Run speed | 1.58 | **1.70** |
| `jump_speed_x_mul` | not checked | 1.1 |
| Landing lag n / b / hi | not checked | **7 / 11 / 8** |
| Weight, air speed | 100, 1.17 | unchanged |

## Ground normals

| Move | Vanilla | Ult S | Note |
|---|---|---|---|
| Jab 1 | F6, 3%, size 1.8 | F6, 3%, **size 2.8** | Bigger hitbox, same data |
| Ftilt | F10–11, 1 hitbox (size 2.7), 12% at 96 KBG / 20 BKB, shieldstun ×0.7 | **F9–13, 3 sword hitboxes (size 4.5)**, 12% at **75 / 42** | New animation. Faster, lasts longer, much bigger hitboxes, flatter knockback scaling, and the shieldstun reduction is gone, so it's safer on shield |
| Dtilt | F5, 8%, 65° at 80 / 50 | F5, 8%, 65° at **60 / 60** | Lower growth, so it combos at higher percents |
| Fsmash | F15–17, 14/15/16% (`top` bone, size 2.8) | Same timing and damage, **follows the sword (size 4.5)** | New animation. Much more reach |
| Usmash | F11–23 multihit (4 + 3 + 3 + 7%, finisher 142 KBG) | **F8–12 single hit, 14%, 85° at 97 / 35**, 4 hitboxes (size 5.5 / 3.5) | New animation. Faster single-hit anti-air; kills somewhat later than the vanilla finisher |
| Grab | F6, reach to 8.7 / 10.35 | F6, **reach to 11.7 / 13.35** | +3 units of reach |

- **Suggestion:** the ftilt is the outlier. It's faster, 5 frames active, three 4.5 hitboxes, and the vanilla shieldstun penalty is gone. If it's too safe at range, put back `ATK_SET_SHIELD_SETOFF_MUL(0.7)` or shorten it to F9–11.

## Aerials

| Move | Vanilla | Ult S | Note |
|---|---|---|---|
| Nair | ~F10–23 (0.85× speed), 8%, 68 / 60 | **~F8–14** (0.6× speed, then 0.5× while active), **10%, 95 KBG / 20 BKB** | Faster, shorter, more damage and kill power. Landing lag 7 |
| Fair | 3 / 3 / 5%, finisher 50° at 130 / 55; landing-lag cutoff F48 | Finisher **35°** at **135 / 50**; cutoff **F29** | Lower kill angle and much earlier cutoff |
| Bair | F10–11, 14% | F10–11, **15%** | Landing lag 11 |
| Uair | F11, 16% (size 5.0), 3-frame clean hit; late 10% (size 5.0) | F11, **14% (size 3.0)**, **2-frame** clean hit; late 10% **(size 4.0)** | Nerfed. Landing lag 8 |

- **Design note:** nair changes the most. It now comes out on about F8 and kills (95 KBG at 361°) with 7 frames of landing lag. That makes it a strong out-of-shield option and a strong edgeguard.
- **Suggestion:** the uair nerf looks like it's offsetting the nair. If uair juggles feel weak, bring the clean-hit size back to about 4.0 and keep the damage at 14%.

## Throws (`throws.rs`)

| Throw | Vanilla | Ult S |
|---|---|---|
| Fthrow | 3%, 45° at 110 / 60 | 3%, **361°** at **50 / 70** (low, flat send) |
| Uthrow | 3%, 80° at 100 / 50, plus 2% hit at 95° 122 KBG | 3%, 80° at 100 / **100**, plus 2% hit at **361°** 100 / 30 (size 6, 0 SDI; only hits bystanders). New animation |
| Dthrow | 2%, 80° at 120 / 75 | 2%, 80° at **110** / 75 |

- **Design note:** fthrow now sends low and flat, which suits tech chases and ledge pressure. Uthrow's higher base knockback makes it a fixed-distance launcher. Dthrow scales slightly less, so it combos longer.

## Neutral-B 1: Gale Strike (`specialn1`, `miiswordsman_tornadoshot`)
- **Startup:** speed 1.15 → **0.706**, so the tornado comes out on about **F13** instead of F20.
- **Cooldown:** 70 frames (`TORNADO_MAX`); cast during cooldown and nothing fires. It resets on death or at match start.
- **Tornado hit:**
  - Vanilla: 13 → 9% decay, 86°, set knockback 150 (launches up).
  - Ult S: **9%, 45°, set knockback 40**. It now pushes opponents forward instead of launching them up.
- **Design note:** this turns a slow, punishable launcher into a fast poke for controlling space, kept in check by the cooldown.

## Neutral-B 2: replaced (`specialn2`, `frame.rs`)
- **Vanilla:** Shuriken of Light (projectile).
- **Ult S:** no projectile. A close-range burst, F12–16, 14%, 361°, 100 KBG / 30 BKB, electric, size 8.5. In the air her speed is frozen for the first 20 frames, so it works as a stall.
- **Suggestion:** a 14% hitbox of size 8.5 that also stalls her in the air is very strong, both as an edgeguard and out of a juggle. Watch it. Options: drop it to about 11–12%, or limit the stall to once per airtime.

## Neutral-B 3: replaced (`specialn3*`, `frame.rs`)
- **Vanilla:** Blurring Blade (charge multihit).
- **Ult S:** a dragging air grab-attack.
  - **Start:** catch hitbox at ~F9–11, 1%, 368° (pulls in). On the ground, connecting lifts her into the air.
  - **Whiff:** she falls (air) at F36 or returns to idle (ground) at F40.
  - **Loop:** after a hit she rises, then drives diagonally down (speed 2, −2.625) with a 0.5% hitbox that holds the opponent.
  - **End:** triggers on hitting the ground or after 30 loop frames. **10%, 50°, 80 / 40, size 9**, then she's actionable at F22.
- **Bug:** the loop sets hitbox id 0 twice in the same frame, so the 368° version is immediately overwritten by the 60° one. The hit effect is also spelled `collision_attr_nomal` (typo), so the effect lookup fails.
- **Suggestion:** fix the typo to `collision_attr_normal` and give the second hitbox id 1 (or delete one). Then test whether the drag is escapable. The 0 SDI and 0 hitlag look like it's meant to be inescapable.

## Side-B 1: Airborne Assault reworked (`specials1*`, `status.rs`, `frame.rs`)
- **Startup:** speed 0.8 → **0.6** (ground) / **0.5** (air). A 0-damage hitbox on F5–13 just detects contact.
- **On contact, ground:** a new custom state keeps her grounded and plays a new animation: **12%, 85° at 79 / 100 BKB**, F3–5. After its cancel frame she can cancel into any attack, special, dash, shield or jump.
- **On contact, air:** **8%, 45° at 80 / 80**, size 7, F2–3. The hit animation runs at 1.43× speed.
- **Vanilla, for comparison:** 14%, 50° at 85 / 70, F25+ (ground) or F30+ (air).
- **Design note:** a slow, high-commitment move becomes a fast lunge that turns into a vertical launcher on the ground, with a cancel window for combos.
- **Suggestion:** a 12% launcher with 100 BKB plus any-action cancels is a strong combo starter. Measure frame advantage on hit. If it's too strong, start the cancel window later or drop the BKB to about 80.

## Side-B 2: Gale Stab (`status.rs`)
- **Change:** the charge state is skipped (`SPECIAL_S2_HOLD` jumps straight to `SPECIAL_S2_DASH`). The hit is the same as vanilla (8%, 55°).
- **Design note:** no more charge, just an instant dash. Faster to use, but it never travels the charged distance or does the charged damage.

## Side-B 3: Chakram (params)
- **Flick version:** lifetime 15, speed 3.0. **Stuck version:** 50 frames. (These do apply, since they're set on the chakram's own kind.)
- **Check:** the vanilla values weren't checked.

## Up-B 1: Stone Scabbard (`specialhi1`)
- **Changes:** the falling hit (F30–45) has set knockback **39 → 100** (spike) and **20 → 80** (the 361° hitbox). Rising hits are unchanged.
- **Design note:** the drop is much stronger both onstage and as an offstage spike.

## Up-B 3: Hero's Spin (`specialhi3start`)
- **Startup:** speed **0.5 → 0.25**, so the charge-up before the spin is about twice as fast. The spin hit itself is vanilla.

---

## Things to check
- [ ] **Fix the gating bug** (`miigunner` → `miiswordsman` in 8 install functions).
- [ ] Check whether the tornado and shuriken params apply. Move them to the right weapon kinds.
- [ ] Neutral-B 3: fix the `collision_attr_nomal` typo and the duplicate hitbox id. Is the drag escapable?
- [ ] Side-B 1 grounded hit: frame advantage plus cancel window. Too strong?
- [ ] Neutral-B 2: is a 14% burst with an air stall too strong offstage?
- [ ] Ftilt: safe on shield without the 0.7× setoff?
- [ ] Check vanilla landing lag and `jump_speed_x_mul`.
