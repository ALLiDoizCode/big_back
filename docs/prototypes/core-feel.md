# PROTOTYPE sheet: core feel (issue #9)

Throwaway. Lives on `prototype/core-feel` only.

```
cargo run --example core_feel
```

**Question:** what should moving, carrying, throwing and shooting feel like for one
character, in any squad model? It's resolved when you have played at least two variants of
each contested parameter below and picked a baseline.

## How to use it

- **Play:** WASD or left stick to move, mouse or right stick to aim, Space/A to jump, E/X to
  grab or place a crate, RMB/RB to throw, LMB/RT to shoot, F/B to kick (tap a tan crate to
  punt it, hold on an orange crate to open it). Stack crates on the **blue pad**; the bot's
  pad is red.
- **Aiming while carrying = the ghost.** With "throw aim: to reticle", the reticle becomes a
  cyan ghost cube, snapped to a crate-sized grid cell (the grid lines up with the pads) or on
  top of the stack in that cell. The throw is solved to land exactly on it, and a crate that
  lands near a cell settles into it, so you can **build stacks and 2×2s by throwing**. The
  ghost is honest: the arc preview runs the same collisions as the game, so if the arc would
  clip a nearer stack, the cyan ghost moves to where the crate will really land and the cell
  you aimed at turns **orange**. Raise the throw angle, move, or fill the far cells first.
  That's the skill.
- **Jump power:** with "jump throw power: peaks at jump apex", throw reach grows the higher
  you are in a jump, up to "jump boost" × throw distance at the top. Time the throw at the
  apex to reach farther. "flat when airborne" gives the full boost anywhere in the air. The
  HUD shows the current throw reach and multiplier while you carry.
- **Curling landings** ("landing: curling"): the reticle becomes the **touchdown** point (a
  cyan ring). The crate keeps "slide keep on impact" of its speed, slides with "slide
  friction", drops off the edge of a stack, and once slow within "snap radius" of a cell
  clicks into it. A sliding crate that hits another hands over its momentum, curling-style,
  so you can knock crates into place, or off a pad. To land in a cell, aim **short** of it.
  Low throws slide far, high lobs barely slide. With "arc preview: arc + rest ghost" a ghost
  shows where it'll stop (orange = it'll knock into a crate there). "arc only" hides the rest
  spot, so you have to read the slide yourself. "landing: stick" is the exact landing from
  before.
- **Placing by hand:** E/X sets the crate on the ghost if it's within arm's length (1.6 m),
  otherwise on the cell a step in front, shown as a faint ghost.
- **Gamepad aim distance:** right-stick tilt sets the reticle distance. Full tilt reaches
  "throw distance" while carrying and 30 m otherwise.
- The HUD shows your pad's crate count and tallest stack.
- **Placeholder craft:** four crates on your pad in a 2×2 block, on the same level, merge into
  a **turret**. It shoots the bot within 14 m (so it guards your pad against the hauler) and
  expires after 40 s. Build it by placing crates side by side: aim at the empty cell next to
  a crate, not at the crate, or the ghost stacks on it. "placeholder craft: off" in the panel
  disables it. **This is not a grammar decision.** It exists so stacking does something while
  you play. What stacks really build is [#17](https://github.com/ALLiDoizCode/big_back/issues/17).
- **Tune live:** Up/Down (D-pad) picks a parameter, Left/Right changes it (Shift ×5).
- **A/B:** there are two full parameter slots. Tab/Select swaps between them. The panel marks
  every parameter that differs between the slots with `*`. Set A to one variant, swap, set B
  to the other, then swap back and forth while playing.
- **Record:** P prints the active slot to the terminal. Paste it into the issue as the
  baseline. R/Start resets the arena and the stats.
- The HUD shows your state (carrying, airborne, grab in flight), pad counts, bot HP, the last
  time-to-kill, accuracy, times hit, and grabs vs. denied grabs.

## What to try

| # | Compare (A vs B) | Setup | What it answers |
|---|---|---|---|
| 1 | **move speed** 6 vs 8 | bot: strafer | Base pace. Does crossing the arena (≈24 m pad to pad) feel snappy or frantic? |
| 2 | **carry speed** 0.6 vs 0.85 | bot: hauler | How much of a sitting duck is a carrier? Low = carrying is a commitment and escorts matter; high = hauling barely costs anything. Can you catch the hauler? |
| 3 | **carrying disarms**: full lock vs sidearm vs melee only | bot: duelist | Can a carrier defend themself? Full lock makes carrying a team problem (good for squads), sidearm keeps solo play viable, melee only is the middle ground. |
| 4 | **hit drops crate**: yes vs no | bot: duelist, then hauler | Is one hit enough to knock a crate loose? With "yes", shoot the hauler to steal back. Chaotic or readable? |
| 5 | **throw style**: ground throw vs jump-throw | anything | Jump-throw (Team Buddies) means a grounded throw is only a short toss, so a full throw takes a jump first. Is that extra input skill expression or friction? |
| 6 | **throw angle / distance**: 25°/7 m vs 45°/12 m | arc preview on, then off | Lob vs. line drive. Can you land a crate on your pad from mid-field? Should a throw reach across the map? Try with the preview off to see whether it's needed. |
| 6b | **throw aim**: to reticle vs fixed distance | mouse, arc preview on | To reticle: the crate lands where the mouse points, up to "throw distance" (the max). Fixed: every throw flies the full distance in the aimed direction (Team Buddies style), and your run speed carries over. Precision vs. a learnable fixed arc. On a gamepad, stick tilt sets the distance. Is tilt precise enough to hit a cell? |
| 6d | **jump throw power**: off vs flat vs peaks at apex (boost 1.5) | throw distance 9 m | Does jumping for reach add skill, or is it a chore? Can you feel the apex timing? Does "flat" make jumping mandatory on every throw? |
| 6e | **landing**: stick vs curling | arc preview "arc + rest ghost", then "arc only" | Curling: does reading the slide feel like mastery (aim short, use lobs vs. line drives, knock crates in), or does it make building a 2×2 frustrating? Is the rest ghost needed? |
| 6f | **slide**: keep 0.4 / friction 10 vs keep 0.8 / friction 4, snap radius 0.15 vs 0.35 | landing: curling, bot: hauler | How much slide is fun? A bigger snap radius forgives more. At 0, nothing aligns and crafts never form. Can you knock the hauler's crates off its pad? |
| 6c | **throw-building**: build a 2×2 by throwing from 6+ m, at throw angle 35° vs 60° | bot: hauler, placeholder craft on | Does throwing into cells feel like a skill worth learning (watch for orange "blocked" ghosts and the fill order), or is it fiddly? Does a high lob make it too easy? |
| 7 | **weapon**: hitscan vs projectile (30 m/s, then 14 m/s) | bot: strafer | Lag compensation covers ray casts only. Do slow, visible projectiles that can be dodged feel good enough to give up hitscan? Compare the TTK and accuracy readouts. |
| 8 | **fire rate / shots to kill**: 5/s and 8 vs 3/s and 4 | bot: strafer | Spray vs. deliberate. Target TTK around 1.5–3 s? |
| 8b | **shot range**: to reticle vs full 30 m | bot: strafer, both weapons | To reticle: shots stop at the mouse, so you must put the reticle on or past the target. Range becomes a skill, and you can avoid hitting things beyond. Full: shots fly 30 m whatever the distance. Gamepad aim is always full range. |
| 9 | **kick-open time** 0.5 s vs 1.5 s | stand by an orange crate | How long are you exposed when opening loot? Long = a risky, contestable moment. |
| 10 | **fake grab latency** 0 vs 100 ms vs 150 ms, mask **none** | bot: hauler | Online grabs cost about one round trip. Do you notice 100 ms? Do you get "GRAB DENIED" racing the hauler? |
| 11 | **latency mask** none vs reach vs lunge, at 120 ms | bot: hauler | Which hides the round trip best? Reach shows an arm extending; lunge steps you toward the crate. Does lunge feel like lost control? |
| 12 | **grab reach** 1.2 vs 2.0 m | 120 ms latency | Does a longer reach also hide latency, and does it make stealing too easy? |
| 13 | **placing:** build a 2×2 on your pad with carry speed 0.6 vs 0.85 | bot: hauler, placeholder craft on | Is placing precise enough to build a shape under pressure? How long does a 2×2 take while the hauler steals? (Feel only. What the shape builds is #17.) |

## Notes on the model

- The bot is "the server": its grabs are instant, and yours are confirmed after the fake
  latency, when the server checks the crate is still free and in reach.
- Thrown crates hurt (2 shots' worth) and knock crates loose, the same as shots.
- No avian3d: crates stack via a simple support check. Characters walk into resting crates
  and get pushed out. This is deliberate. It tunes feel, not physics.

## Verdict

**Decided (2026-10-18), played by the human:**

- **6e landing: curling** (slide + snap), not stick. Reading the slide is the skill: aim
  short, lob vs. line drive, knock crates into place or off a pad. Played at slide keep 0.6,
  friction 6 m/s², snap radius 0.3 m.
- **6d jump throw power: peaks at jump apex**, not off or flat. Timing the throw at the top
  of the jump adds reach, up to 1.5× at the apex.
- Both build on what came before: aiming while carrying = the ghost, throws solved to the
  touchdown, honest collision-aware preview, gamepad stick tilt = distance.
- Verdict: "this is fun".

**Still open:** every other comparison (1–5, 6, 6b, 6c, 6f, 7–13), including carry
disarm, hit drops crate, hitscan vs. projectile, the fire rate/TTK target, kick-open time,
grab latency masks and reach. The slide numbers (6f) are where they were played, not tuned.
