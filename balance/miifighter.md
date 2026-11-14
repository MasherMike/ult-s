# Mii Brawler: Ult S vs vanilla

- **Ult S source:** `src/miifighter/` (ACMD, `frame.rs`, `mod.rs` params). New animations in `romfs/fighter/miifighter/motion/body/`: dair (plus bounce and landing), side-B 1 (Onslaught, all parts), down-B 1.
- **Vanilla sources:**
  - Scripts: WuBoytH/SSBU-Dumped-Scripts, `smashline/miifighter/*` (13.0.4).
  - Stats, landing lag and startup: ultimateframedata.com.
- **Frame numbering:** frames are script frames, which match ultimateframedata's numbering. Frames marked ~ were converted by hand from `FT_MOTION_RATE`. None of this has been tested in game.
- **Unchanged from vanilla:** tilts, ftilt, fsmash, usmash, dash attack, nair, uair (re-registered with identical values), grab and throws, neutral-B 1 (Shot Put; the projectile values match vanilla), neutral-B 2, side-B 2/3, up-B 2/3.
- **Gating:** Brawler checks her own `miifighter` marker (correct, unlike Swordfighter). Agents are hardcoded to slots `[0..7]` and params to `[0..12]`.

## Overall
- **Kit direction:** a much more aggressive rushdown.
  - Jabs, dsmash, bair and fair are all reworked toward combos.
  - Dair is now a fast diagonal dive kick that bounces off on hit.
  - Exploding Side Kick becomes a chargeable, cancellable and storable kill move.
  - Head-On Assault is replaced by a grounded rush / aerial launch-and-spike move.
  - Onslaught becomes a short launcher.
  - Counter Throw also reflects.
- **What she pays:** 4 points of weight (94 → 90), dair's 28-frame landing lag, and two possible bugs that may quietly remove parts of up-B 1 and down-B 2.

---

## Possible bugs (check in game)
1. **Up-B 1 (Soaring Axe Kick) startup script never runs.** `acmd/specials.rs` registers `game_specialhi11` / `game_specialairhi11`. The vanilla scripts are `SpecialHi1, SpecialHi12, SpecialHi13, SpecialHi14`, so the first part is `game_specialhi1`, not `hi11`. The 0.8× speedup on the startup probably never applies. The landing-hit override (`game_specialhi14`) is named correctly, so that one works.
2. **Down-B 2 (Feint Jump) may have lost its kick.** The Ult S `game_speciallw2start` leaves out vanilla's F3 `START_WAIT_INPUT` and F17 `KICK_ENABLE` + `REVERSE` flags. If no status code sets them, Feint Jump can no longer kick or turn around; it's just a jump plus the wall-jump window.
3. **Dead code in `frame.rs`.** The "HOA Special Kick" block (checks `SPECIAL_LW2_KICK_LANDING` with custom down-B number 0) and the "CT Special Kick Bounce" block (checks `SPECIAL_LW2_KICK` with custom number 2) look for down-B 2's kick states, but with the down-B 1 and 3 custom numbers. Elsewhere in the same file, 0 is used for down-B 1. So these branches probably never fire.

## Attributes (`mod.rs`)

| Stat | Vanilla | Ult S |
|---|---|---|
| Initial dash | 1.87 | **2.02** |
| Run speed | 1.92 | unchanged |
| Air speed | 1.15 | **1.17** |
| `air_accel_x_mul` | not checked (UFD total 0.068) | 0.045 |
| `jump_speed_x_mul` | not checked | 1.3 |
| Traction (`ground_brake`) | not checked | 0.125 |
| Weight | 94 | **90** |
| Fair / Bair / Dair landing lag | 11 / 11 / 18 | **10 / 8 / 28** |
| Down-B 2 `lw2_landing_brake_x` | not checked | 0.2 |

- **Design note:** a faster dash plus a high `jump_speed_x_mul` makes her carry a lot of momentum into jumps, which suits approaches with fair and bair. The weight drop is the tax for it.

## Jabs (`ground.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Jab 1 | F2, 1.8%, 25 KBG / 25 BKB | F2, **2.8%, set knockback 15** |
| Jab 2 | F2, 1.0%, 25 / 25; can continue at F7 | F2, 1.0%, **set knockback 15**; can continue at **F5** |

- **Design note:** set knockback keeps jabs connecting at any percent, and jab 1 does more damage. That makes jab 1 → jab 2 a reliable frame-2 combo starter and a strong out-of-shield option.

## Dsmash (`ground.rs`)
- **Startup:** 0.667× speed until F9, so it hits about **F7** instead of F9.
- **Hit:** 13% stays the same. Angles 32°/35° → **30°/30°**, first hitbox BKB 41 → 50.
- **Design note:** a faster, lower-angle dsmash. Good for ledge traps and for punishing rolls.

## Fair (`aerials.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Hit 1 (F8–9) | 5%, 25°, 2 hitboxes | **4%, 4 hitboxes: two at 367° (pull in), two at 75°**, 0.7 SDI |
| Hit 2 (F15–17) | 6%, 40° at 100 / 35 | **6.65%, 54° at 105 / 42** |
| Landing-lag cutoff | F30 | **F26** |
| Landing lag | 11 | **10** |

- **Design note:** hit 1 now pulls opponents into hit 2 reliably. Hit 2 launches higher and links into juggles. A cleaner combo tool than vanilla's.

## Bair (`aerials.rs`)
- **Hit:** F7, 12% / 10% (was 12% / 12%), **70 KBG / 40 BKB** (was 105 / 10), hitbox sizes 5.0 / 5.0 (was 4.7 / 4.3).
- **Landing lag:** **8** (was 11).
- **Design note:** low-growth bair for combos and spacing, at the cost of most of its kill power.

## Dair: dive kick (`aerials.rs`, `frame.rs`, new animations)

| | Vanilla | Ult S |
|---|---|---|
| Motion | Vertical stomp | Small hop, then **diagonal dive** forward and down (speed 1.75, −3.03) from about F13 |
| Startup | F16 | **~F13** (1.83× speed until F7) |
| Hit | F16–17: 13% spike (270°, sweetspot) or 13% at 70°; F18: 12% at 361° | **~F13: 10% at 290°** (spike forward and down) · **then 7% at 65°, through F34** |
| On hit | — | Switches to a bounce animation (`attack_air_lw_bounce`) |
| Landing lag | 18 | **28** |

- **Design note:** this is an aerial that travels like Zero Suit Samus's or Sheik's dive kick. It can't be canceled, it covers a lot of distance, and it bounces off on hit. The long 7% hitbox at 65° pops opponents up on hit, and the bounce gives her a follow-up. The 28 landing lag is the punish if it's blocked or whiffed.
- **Suggestion:** test it on shield. With 28 landing lag it should be punishable unless she spaces the late hit or hits shield with the bounce. If the bounce also triggers on shield hits (`is_infliction_status(HIT)` shouldn't), it's too safe. Offstage, a forward dive with a spike on the first frame is risky by design. Keep it.

## Neutral-B 3: Exploding Side Kick, now chargeable (`specialn3`, `frame.rs`)
- **Charge:**
  - The animation stalls at 0.1× speed for its first 10 frames, then loops frames 16–39 while she charges, up to **186 frames**.
  - Pressing special releases it (jumps to F40 with a small forward lunge). It also auto-releases at full charge, and on the ground she drops back to idle.
- **Cancels:** during the charge window, shield → shield (ground) or airdodge (air), and jump → jump.
- **Stored charge:** the charge counter only resets on release, death, or if she's hit right after the move. So **charge carries over** when she cancels, like DK's Giant Punch.
- **Damage:** **10.5% + 0.075 × charge** (sweet) and 9% + 0.065 × charge (sour). That's 10.5–24.5% / 9–21.1%. Vanilla was a fixed 25% / 23%.
- **Hit:** 361° at **60 KBG / 50 BKB**, hitbox sizes **6.0 / 5.0** (vanilla 80 / 40, 4.0 / 3.0). Above 170 charge, shieldstun is ×0.8.
- **Turnaround:** a stick-back input on F5 turns her around.
- **Armor:** the same "always" no-flinch armor window as vanilla.
- **Design note:** a slow, committal, armored kill move becomes a storable threat. When it's quick-released, it's a weak and fast-ish poke.
- **Suggestion:** stored charge with shield and jump cancels plus armor is a lot. Check how fast a full-charge release hits after the special press. Starting from F40 at 0.7× speed, it's about 7 frames to the F50 hit. If that's too fast for a 24.5% move, slow the release section (make F40–50 0.8–1.0× speed).

## Down-B 1: Head-On Assault replaced (`speciallw1`, `frame.rs`, new animations)
- **Ground version:**
  - **Rush:** F10–36, 1.2% hits on 4 hitboxes (size 7, 0.4 SDI), rehitting every 2 frames.
  - **Finisher:** F40–42, **5%, 361° at 90 / 70, size 9**. Roughly 15–20% total.
  - **Ending:** returns to idle at F79. If there's no ground under her after F55, she falls instead.
- **Air version:**
  - **Launch:** F8–11, 4% at 90°, set knockback 65 / 50, 0 SDI.
  - **Spike:** **F31: 12%, 280° (spike), size 12**, then F32–34 12% at 50° (size 10).
  - **Ending:** she becomes actionable at F60 and falls at F61. It's usable **once per airtime** (`BAN_DOWNB`).
- **Removed:** the vanilla dive loop (`special_lw1_loop`) is skipped straight to a fall.
- **Design note:** the air version launches up and then follows with a giant spike. Size 12 is enormous. On the ground it's a long multihit that traps opponents.
- **Suggestion:** a size-12 spike that follows a guaranteed launch is the thing to test. If launch → spike is a true combo, shrink the spike hitbox to about 8 or move it a few frames later so it can be dodged.

## Down-B 3: Counter Throw now reflects (`frame.rs`)
- **Change:** the counter script is identical to vanilla. New: when the counter's detection box touches something (`SearchModule::is_inflict`), she switches to the fthrow animation from F12 and creates a **reflector** (1.4× damage, 1.5× speed). She can act after F20, and in the air she lands if she's close to the ground.
- **Design note:** gives her an answer to projectiles, which the vanilla Counter Throw couldn't catch.

## Side-B 1: Onslaught reworked (`specials1*`, `frame.rs`, new animations)
- **Startup:** 0.75× speed until F16, so the grab-hit comes out on about **F12** instead of F16.
- **Ground follow-up:** a single 2.5% hit at 367° (F2), then **10% at 87°, 30 KBG / 70 BKB** (F12–14). She can act after F53.
- **Air follow-up:** rises (speed 1.0, 1.2 → 1.0, 1.5). 2.5% at F2, then **13% at 361°, 75 / 60** (F11–14). She falls at F48 (not helpless) and lands normally if she touches ground after F18.
- **Vanilla, for comparison:** a ~6-hit rush ending in a 7% finisher (ground 80° at 144 KBG; air 70° at 135).
- **Design note:** a long multihit becomes a fast launcher. On the ground it's a low-growth vertical send for juggles. In the air it's a 13% horizontal hit that can kill offstage, and she isn't left helpless.

## Up-B 1: Soaring Axe Kick (`specialhi*`)
- **Landing hit:** 4% → **4.5%** (45°, 170 KBG unchanged).
- **Startup:** a 0.8× speedup, but see possible bug #1. It probably doesn't apply.

---

## Things to check
- [ ] Does the `game_specialhi11` script ever run? Rename it to `game_specialhi1` / `game_specialairhi1` if not.
- [ ] Does Feint Jump still kick? If not, add back the F3 / F17 flags.
- [ ] Clean up or fix the down-B 2 kick branches in `frame.rs` (custom-move-number mismatch).
- [ ] Air down-B 1: is launch → size-12 spike a true combo?
- [ ] ESK: is stored charge plus cancels plus ~7-frame release too strong?
- [ ] Dair: punishable on shield with 28 landing lag? Does the bounce trigger on shield?
- [ ] Check vanilla traction, `jump_speed_x_mul` and `air_accel_x_mul`.
