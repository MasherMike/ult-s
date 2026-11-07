# Mario: Ult S vs vanilla

- **Ult S source:** `src/mario/` (ACMD, `frame.rs`, `status.rs`, `mod.rs`).
- **Vanilla source:** WuBoytH/SSBU-Dumped-Scripts, `smashline/mario/mario/*.txt` (13.0.4).
- **Timing:** frames marked ~ were converted to real frames by hand from `FT_MOTION_RATE`. None of this has been tested in game.
- **Unchanged from vanilla:** jab, ftilt, utilt, all three smashes, nair, bair, grabs, throws, the fireball projectile itself, and up-B's scripts.

## Overall
Ult S Mario keeps vanilla's frame data and grounded kit, with three big changes:
- **Cape becomes an attack and a recovery tool.** It no longer turns opponents around.
- **FLUDD becomes a ground pound.**
- **Dair becomes a drill kick.**

The changes that matter most for balance are **down-B's spike** and **air side-B's rise**.

---

## Down-B: FLUDD replaced with a ground pound (`speciallwstart` scripts, `status.rs`, `frame.rs`)

| | Vanilla | Ult S (air) |
|---|---|---|
| Move | FLUDD (charge, push) | Ground pound |
| Hitbox | none | F14–18: 10%, 270° spike, 90 KBG / 30 BKB · F19–58: 10%, 70° |
| On hit | — | Bounces (switches to jump physics and skips to animation F60) |
| Landing | — | Screen shake, then helpless fall (`FALL_SPECIAL`) |

- **Design note:** a 5-frame spike window at 10%, and Mario bounces back up when it connects. That makes offstage spikes low-risk, and that pairing is the strongest edgeguarding tool in his kit.
- **Grounded down-B:** `game_speciallwstart` does nothing but a hop at F3. It has **no hitbox**, and it falls after animation F35. Either it's meant to be pure movement, or the grounded motion was supposed to switch to the air script.
- **Dead code in `frame.rs`:** `set_rate(0.5)` sits inside a check for "has hit", but its own condition is "has not hit". So it can never run.
- **Suggestion:** first test that grounded down-B really has no hitbox. If the spike turns out too strong, pick one of these:
  - shorten the spike window to F14–16
  - drop the spike to 8%
  - stop the bounce when the spike hits (keep it only for the 70° hit)

## Side-B: cape reworked (`specials.rs`, `frame.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Hitbox | F12–14, 7%, 110°, turns opponent around | ~F7–17 (doc says F8–19), 11% on 4 hitboxes, 1.5× hitlag |
| Angle / KB | set knockback 80 | Ground: 30°, 45 KBG / 70 BKB · Air: 361°, 100 KBG / 30 BKB |
| Reflector | F9–21 | ~F7–17 (starts earlier, ends sooner) |
| Movement | small one-time stall | Air: rises at speed (0.45, 1.9) |
| Reuse | can reuse; stall only once | Locked until he lands or is hit (`can_sideb`) |

- **Design note:** Mario loses the cape turnaround, his signature edgeguard. In exchange he gets damage, a fast reflector and a second recovery option.
  - The ground version's low KBG with high BKB means knockback barely grows with percent. It's a consistent mid-percent tool, not a kill move.
  - The air version's 100 KBG at 361° can kill near the ledge.
- **Suggestion:** the stacking is the thing to watch. A rising side-B plus up-B gives Mario far more recovery than vanilla. If you want some edgeguard identity back, a cheap option is to bring back the turnaround on the ground hitbox only.

## Dair: drill kick (`aerials.rs`, new animation)

| | Vanilla | Ult S |
|---|---|---|
| Multihit | 5 × 1.4%, 94°, F5–13 | 7 × 1.0%, 367° (pulls in), F5–17, +5 hitstun per hit, SDI 0.8 |
| Finisher | F23, 5.5%, 75°, BKB 80 | F20, 8%, 361°, 100 KBG / 20 BKB |
| Total | 12.5% | 15% |
| Landing hit | 2 hitboxes at 361° | 4 hitboxes, adding two at 55° |

- **Design note:** the pull-in angle, the extra hitstun and the low SDI make the multihit hard to escape. If he lands mid-drill, the landing hit picks the opponent up. It's a strong way to drag opponents to the ground and start combos.
- **Suggestion:** the extra hitstun is the main lever. Test landing mid-drill on a shielding or grounded opponent. If it leads to guaranteed jab or grab follow-ups at most percents, drop the hitstun from +5 to +2–3.

## Uair (`aerials.rs`)

| | Vanilla | Ult S |
|---|---|---|
| Startup | F4 (motion sped up to 0.75× on F1–5) | F5 (speedup removed) |
| KB | 135 KBG / 10 BKB | 140 / 12 |
| Hitstun | — | +1 frame |
| Landing-lag cutoff (`ENABLE_LANDING` off) | F18 | F25 |

- **Design note:** this is a deliberate trade. Each hit combos slightly better, but uair is slower to start and gets landing lag until much later. That cuts down on short-hop uair spam and landing loops.
- **Suggestion:** keep it. If juggles feel sluggish, move the landing-lag cutoff back to around F20 rather than bringing back the F4 startup.

## Dtilt (`ground.rs`)
- **Frame data:** angle 80° → 90°, BKB 35 → 50. Timing is unchanged.
- **Design note:** it now launches straight up, a cleaner setup into uair and up-B, which matches the uair changes.
- **Suggestion:** none needed. Check that dtilt into uair still works at low percent now that uair starts on F5.

## Dash attack (`ground.rs`, new animation)
- **Startup:** about F7 (animation runs at 0.5× speed until F11), vs F6 in vanilla.
- **Hitboxes:** larger (5.0 / 3.5 vs 3.5 / 2.7) and recentered on Mario.
- **Hitlag:** 1.0× vs 1.25×.
- **Active frames:** about F7–19, vs F6–25 in vanilla. The Ult S doc lists FaF 33.
- **Design note:** less lingering hitbox and a bigger hit area. Mario's DACUS window is 10 frames (`src/common/dacus.rs`), so it's easy to cancel into an up smash.
- **Suggestion:** fine as is.

## Smaller changes
- **Neutral-B:** vanilla slows the startup to 1.15×; Ult S removes that, so the fireball comes out on F14 instead of about F16. Small buff.
- **Fair:** the arm hitboxes use the fire effect. The numbers are identical, so it's cosmetic.
- **Movement:**
  - `jump_speed_x_mul` is set to 1.2 in `mod.rs`, so he carries more running speed into jumps. The vanilla value hasn't been checked.
  - The run switches to a new `run_max` animation after 30 frames.
  - New walk and back-jump animations. These are cosmetic.
- **Up-B:** new animation files (`d02specialhi`) and an edited `motion_list.bin`, but the hit scripts are vanilla. If the timing changed, it was done through `motion_list`.
- **Up taunt:** mushroom-grow effect. Cosmetic.

## Things to check
- [ ] Does grounded down-B really have no hitbox (just a hop)?
- [ ] How much total recovery does air side-B plus up-B give him?
- [ ] Does the drill-kick landing lead to guaranteed follow-ups?
- [ ] Did the up-B timing change through `motion_list`?

## Bug seen outside Mario's code
In `is_enable_transition_term_hook` (`src/util/mod.rs:143`), the check for objects that aren't fighters calls `original!()` without `return`, so it falls through to the fighter logic anyway.
