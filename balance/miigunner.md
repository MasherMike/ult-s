# Mii Gunner: Ult S vs vanilla

- **Ult S source:** `src/miigunner/` (ACMD, `frame.rs`, `mod.rs` params).
- **Vanilla sources:**
  - Scripts: WuBoytH/SSBU-Dumped-Scripts, `smashline/miigunner/*` (13.0.4).
  - Attributes and landing lag: ultimateframedata.com.
- **Frame numbering:** frames are script frames, which match how ultimateframedata numbers them (for example, vanilla grab is F6 on both). Frames marked ~ were converted by hand from `FT_MOTION_RATE`. None of this has been tested in game.
- **Unchanged from vanilla:** jab, tilts, dash attack, down smash, throws other than up throw, Laser Blaze's projectile, Grenade Launcher, Gunner side-B 2's travel, up-B 2/3, and the rest of the neutral-B charge logic.

## Costume gating (different from most fighters)
Gunner's agents are hardcoded to slots `[0..7]` and her params to slots `[0..12]`. They don't use `get_marked_costumes`. The only marker check is "is there at least one `miigunner.marker`?" (`frame.rs`, `throws.rs`). So once Ult S Gunner is enabled, every Gunner slot gets the changes.

## Overall
- **Kit direction:** a faster ground game, a much stronger close-range kit (shine-style reflector, a dair that spikes on every hitbox, a stronger up-throw kill), and a buffed minimum Charge Blast.
- **What pays for it:** a slower grab, slower Gunner Missiles, a 100-frame cooldown on Flame Pillar, weaker up air and up smash finishers, slower air speed and lighter weight.
- **Where the power is:** close-range pressure, not zoning.

---

## Attributes (`mod.rs`)

| Stat | Vanilla | Ult S |
|---|---|---|
| Initial dash (`dash_speed`) | 1.613 | **1.7** |
| Run speed | 1.37 | **1.45** |
| Air speed | 0.93 | **0.89** |
| Weight | 104 | **102** |
| Traction (`ground_brake`) | not checked | 0.07 |
| `jump_speed_x_mul` | not checked | 0.8 |
| Nair landing lag | 11 | **9** |
| Fair landing lag | 12 | 12 |
| Dair landing lag | 17 | **15** |
| Bair / Uair landing lag | 11 / 13 | unchanged |

- **Design note:** better ground speed but slower air speed. The low `jump_speed_x_mul` also limits how much running speed carries into jumps. Together they push her to approach on the ground and short hop, not drift in the air.

## Down-B 1: Echo Reflector becomes a shine (`speciallw1start`, `common/cancel.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Hitbox | F3, 2%, 10°/24°, 30 KBG / 60 BKB, 1.0× hitlag | F3, 2%, 10°/24°, **40 KBG** / 60 BKB, **0.2× hitlag**, **+3 hitstun** |
| Jump cancel | none | Any time during the LW1 loop, hit and end states |

- **Design note:** this is the biggest change to her kit. A frame-3 hitbox with almost no hitlag and extra hitstun, cancellable by jumping, gives her Spacie-style shine combos and shield pressure. Her other changes fit around it: the faster run, F11 DACUS and the ground-focused movement.
- **Suggestion:** test shine → jump → aerial on shield. If it's safe or a true combo at most percents, cut the extra hitstun from 3 to 1–2, or make the jump cancel start only from the loop state.

## Dair (`aerials.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Startup | F20 | **~F17** (0.8× speed, applied from F4) |
| Early hit | Only the 15% sweetspot is 270°; other hitboxes 361° | **All three hitboxes 270°** (10% sour: 75 KBG) |
| Late hit (F22+) | 361° | **270°** (8%: 65 KBG, 12%: 90 KBG) |
| Active | F20–25 | F20–26 |
| Landing-lag cutoff | F54 | **F37** |
| Landing lag | 17 | **15** |

- **Design note:** every hitbox on every active frame now spikes. Vanilla needed the sweetspot. She also gets less landing lag and an earlier landing-lag cutoff. It goes from a niche risky spike to her go-to edgeguard. Onstage it now pops grounded opponents straight up instead of sending them sideways.
- **Bug:** the script calls `frame(4)` (landing flag) *before* `frame(1)` (motion rate), so the 0.8× speedup starts at F4 instead of F1. That puts the hitbox at about F17 instead of the F16 that "0.8 of 20" suggests. Bair has the same ordering bug.
- **Suggestion:** keep the late hit (F22+) at 361° so only the early hit spikes. That keeps the edgeguard power but stops the lingering hit from being a free spike.

## Neutral-B 1: Charge Blast (`miigunner_gunnercharge` `game_shoot`)

| | Vanilla | Ult S |
|---|---|---|
| Min charge | 4%, size 1.5, 0.5× hitlag | **7%**, size **2.0**, 1.0× hitlag |
| Max charge | 26%, 50°, 66 KBG / 28 BKB, size 6.0, 0.7× hitlag | **27%**, **45°**, **70 / 30**, size **8.0**, 1.0× hitlag |

- **Design note:** uncharged shots go from a poke to a real 7% projectile. Full charge is bigger and slightly stronger, and the lower angle helps it kill near the ledge.
- **Suggestion:** an 8.0 full-charge hitbox is very easy to land. If it feels like a free punish, set the size back to about 6.5–7.0 and keep the damage changes.

## Neutral-B (custom move number 1; most likely Laser Blaze, since numbering starts at 0)
- **Changes:** if she lands mid-move, she goes into 14 frames of landing lag. While airborne she can fast fall once she's moving down.
- **Dead code:** `gunner_laser` is defined in `specials.rs` but never registered, so the laser projectile itself is vanilla.

## Side-B 1: Flame Pillar (`specials1`, `miigunner_flamepillar`)
- **Cooldown:** the pillar only spawns if a 100-frame cooldown (`FP_DELAY`) has run out, and the cooldown resets each stock. The move still plays out during cooldown; it just doesn't fire anything.
- **Multihits:** 2.2% → 2.0%.
- **Finisher:** 2.8% at 45° → 2.5% at 70°.
- **Design note:** this is anti-spam. The pillar now launches more vertically, so it's better for juggles and worse for pushing opponents away.
- **Suggestion:** fine as is. You could add a visual or sound cue during cooldown so it's obvious when it's off cooldown again.

## Side-B 3: Gunner Missile (`specials31` / `specials32`)
- **Homing (31):** speed multiplier 1.15 → 1.304, so the missile fires at about F30 instead of about F26.
- **Super (32):** speed multiplier 1.0 → 1.087, so it fires at about F25 instead of F23.
- **Design note:** slower missile startup weakens zoning, which matches the push toward close range.

## Side-B 2: Stealth Burst (`miigunner_groundbomb`)
- **Explosion:** 9%, angle 72° → 80°, KBG 80 → 70, BKB 70 → 60. More vertical, less knockback.
- **Lifetime:** `param_groundbomb.life` is set to 180 frames. I didn't check the vanilla value.

## Up-B 1: Lunar Launch (`frame.rs`, new animations)
- **New behavior:**
  - Once per airtime (`can_upb`; resets when she isn't airborne).
  - At animation F30 it switches to a normal **fall**, not helpless fall.
  - It uses up her double jump (jump count set to max).
  - New `specialhi1` animations.
- **Design note:** she can act after up-B (aerial, airdodge), but she has no jump left. Useful out of juggles and for mixing up her recovery, but she can't stall for long.
- **Suggestion:** fine as is. Watch for up-B → dair offstage, since dair now spikes on every hitbox.

## Down-B 3: Absorbing Vortex (`speciallw3hold`)
- **Hitbox:** size 12.5 → 9.5.
- **Knockback:** set knockback 40 (vanilla 100 KBG, set 40) → 65 KBG, set 40, 35 BKB. Set knockback overrides BKB, so in practice it's almost the same.
- **Hitstun:** +4.
- **Design note:** smaller hitbox, but the extra hitstun makes it a combo starter.

## Nair (`aerials.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Hit | F8–21, 10%, 65 BKB, 5.5 / 5.5 | F8–16 clean: 10%, 60 BKB, 5.0 / 3.0 · F17–19 late: 5.5%, 50 BKB |
| Landing-lag cutoff | F35 | **F42** |
| Landing lag | 11 | **9** |

- **Design note:** shorter and smaller, with a weak late hit instead of a lingering 10% hit. Less landing lag, but the landing-lag cutoff comes later. It's a spacing tool now, not a safe out-of-shield option or a "throw it out" aerial.

## Fair (`aerials.rs`, `miigunner_attackairf_bullet`)
- **Gunner:** same as vanilla except the landing-lag cutoff moves from F42 to **F36**.
- **Bullet:** 8%, angle 48° → 50°, size 2.8 → 3.0. Late hit: 6%, angle 52° → **32°**, size 1.8 → 2.0, starts a frame sooner.
- **Design note:** the late hit's lower angle makes it a better edgeguard and ledge-trap tool. The earlier landing-lag cutoff makes it a little safer to land with.

## Bair (`aerials.rs`)
- **Startup:** F9 → **~F8** (0.78× speed, but it starts at F4 because of the same ordering bug as dair; the intent was about F7).
- **Knockback:** BKB 29/41 → 30/40. Basically the same.
- **Suggestion:** if F7 was the target, move the `FT_MOTION_RATE` call above the `frame(4)` flag.

## Uair (`aerials.rs`)
- **Finisher:** angle 72° → 70°, **KBG 175 → 160**.
- **Landing-lag cutoff:** F60 → **F49**.
- **Design note:** less kill power, more safety. That matches the shift away from top-platform kills.

## Smashes (`ground.rs`)
- **Fsmash:** multihits 1.8% → 2.0% (first hit set knockback 7 → 10); finisher 7.5% → 7.0%. Total damage is slightly up.
- **Usmash:** first sweetspot size 7.0 → 5.25; **final hit KBG 139 → 130**. A kill-power nerf.

## Grab (`other.rs`)
- **Startup:** **0.5× speed for the first 4 frames**, so the grab box comes out at about **F10** instead of F6. Same grab box sizes.
- **Design note:** a 4-frame slower grab is a big nerf. It's probably meant to offset the shine pressure and the stronger up-throw kill.
- **Suggestion:** confirm it in game. If it's too slow, trim to about F8 (0.75× speed) rather than reverting fully.

## Up throw (`miigunner_rapidshot_bullet` `game_flythrowhi`)
- **Laser hit:** 4% → **5%**, angle 89° → 90°, **KBG 150 → 165**.
- **Design note:** a stronger, earlier-killing up throw. Combined with the slower grab, each grab is worth more but harder to land.

## Movement systems that affect Gunner
- **DACUS:** 11-frame window (`common/dacus.rs`).
- **Jump cancels:** the Reflector jump-cancel entries apply to slots 0–255 (`common/cancel.rs`).

---

## Things to check
- [ ] Shine → jump on shield: is it safe or frame-advantaged? Is shine → aerial a true combo?
- [ ] Is the slower grab (~F10) acceptable? Does up throw kill too early now?
- [ ] Dair: does the late spike feel free offstage?
- [ ] Is the full-charge Charge Blast hitbox (8.0) too large?
- [ ] Is the Flame Pillar cooldown readable in game?
- [ ] Fix the bair/dair `frame(4)` before `frame(1)` motion-rate ordering if F7/F16 were the targets.
- [ ] Check vanilla traction, `jump_speed_x_mul` and Stealth Burst lifetime.
- [ ] Gunner ignores per-slot markers. Is that intended (all slots always Ult S)?
