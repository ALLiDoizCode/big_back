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
- **Placing:** while carrying, a cyan ghost cube shows where E/X will set the crate down,
  about one step in front of you, in your aim direction. It snaps onto any crate or stack
  under it (to build up) and otherwise onto a crate-sized grid that lines up with the pads
  (to build out). The HUD shows your pad's crate count and tallest stack.
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
| 7 | **weapon**: hitscan vs projectile (30 m/s, then 14 m/s) | bot: strafer | Lag compensation covers ray casts only. Do slow, visible projectiles that can be dodged feel good enough to give up hitscan? Compare the TTK and accuracy readouts. |
| 8 | **fire rate / shots to kill**: 5/s and 8 vs 3/s and 4 | bot: strafer | Spray vs. deliberate. Target TTK around 1.5–3 s? |
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

_To be filled in after play: the picked baseline (paste the P output) and one line per
comparison._
