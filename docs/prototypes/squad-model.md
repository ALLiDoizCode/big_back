# PROTOTYPE sheet: squad model (issue #10)

Throwaway. Lives on `prototype/squad-model` only (branched from `prototype/core-feel`).

```
cargo run --example squad_model
```

**Question:** which squad model makes the most fun and fair competitive game: one character
per player, a hero plus a partner, or a leader with 2–3 AI Buddies of their own (A)? Or no
squad at all, with neutral haulers owned by the team (B)? It's resolved when you pick a squad
model, and that pick sets the range of team sizes.

## What's in it

- **Two teams of leaders.** You are blue leader 1. The other leaders are bots standing in for
  humans, and they share one brain with the buddies. At 3v3 with 3 buddies each, that's 24
  bodies.
- **Core feel is fixed at the #9 baseline.** Curling jump-throws, 0.6× carry speed, full lock,
  a hit drops the crate, 18 m/s projectiles at 3/s, 4 shots to kill a leader, and reach-masked
  grabs at 120 ms. It's not in the panel.
- **Score** = crates resting on your pad (a placeholder turret counts 4). Matches last 120 s,
  then the result prints and the match restarts.
- **Model A**: every leader owns 0–3 buddies ("squad size"). Size 1 is the hero + partner
  model (candidate C), and size 0 is one character per player.
  - **Default job**, for a buddy with no order:
    - *follow & guard me*: shoots what's near you.
    - *haul to pad*: fetches crates within "leash" of your pad and sets them down on it.
    - *guard pad*: stands at your pad's front edge and shoots raiders.
  - **Orders**, depending on the "orders" setting:
    - *one context button* (Q / LB): what's under the reticle picks the verb. An enemy means
      attack, a crate means haul that crate, your pad means guard it, and open ground means
      hold there. Tap orders the nearest buddy; hold for 0.3 s (the ring fills) to order the
      whole squad.
    - *4 verbs* (Z follow, X haul, C guard pad, V attack or hold at the reticle; on a gamepad,
      hold LB and press the D-pad).
    - *none*: no orders at all.
    - G (or a left-stick click) recalls everyone to the default job.
    - Ordered jobs time out after 15–20 s.
  - **Tells**: when an order lands, a pip pops over the buddy's head, and everyone sees it.
    With "order lines" on, your own buddies show a line to their target.
  - **Leash**: following and holding buddies stay within "leash" of their leader. Haulers only
    fetch within "leash" of their pad, so stealing from the enemy pad takes an order.
  - A dead leader's followers fall back to guarding the pad.
- **Model B**: no squads. Each team has "haulers/player" × team size neutral haulers that haul
  and guard on fixed rules and take no orders. They're darker and have no owner band. Their
  "owner" is the team pad.
- **Bot leaders**: they haul (and steal), fight anything within 8 m, and chase enemy carriers.
  "blue/red bots order: micro" makes them re-order their squad every 2.5 s: attack a threat
  to the pad, or send each buddy after a different crate, wherever it is. "never" leaves
  their squad on its default job.
- **Online delay** (0–250 ms) shows every unit except you in the past, the way interpolation
  does online. It also delays orders by the same amount.
- The HUD shows the body count against the netcode budget (~32 networked characters, ~20–30
  moving crates).

### Readability looks

- **owner band**: a ring in the owner's colour (yellow, green, white or purple by leader slot).
  A leader and their buddies share one colour, and the team still shows in the body colour.
- **buddy scale**: AI drawn smaller. Hitboxes don't change; this is only a look.
- **human halo**: a white ring over each leader.
- **tethers**: faint lines from a leader to their buddies (yours only, or everyone's).

### The readability test (T)

The game freezes and a yellow ring marks one unit (two times in three, an AI one). **Click
its owner**: the unit itself if it's a human leader, its leader if it's a buddy, or its pad if
it's a hauler. The clock runs from the freeze, and a line then shows the right answer. After
8 freeze-frames it prints, for example, `readability: 6/8 right, 4/8 right in under 1 s,
median 1.10 s | looks | roster`. The target is right, in under a second. The test needs a mouse.

## Running the comparisons (guided)

1. Press **N**. The next comparison loads, and the box top-left shows the question.
2. Press **1–4** to switch variants. Each switch restarts the match.
3. Press **Enter** to keep the one you're playing. It becomes the baseline. **N** skips.
4. At the end, press **P** and paste everything into the issue: settings, verdicts, match
   results and readability results.

| # | Compare | What it answers |
|---|---|---|
| 1 | **squad size** 0 / 1 / 2 / 3 at 3v3 | How many buddies should you own? Is 1 (hero + partner) a shallow middle, or the sweet spot? Is 3 noise? |
| 2 | **A own squad** vs **B neutral haulers** | Is commanding your own squad more fun than team haulers nobody commands? Does B lose the Team Buddies identity? |
| 3 | **default job**: follow / haul / guard pad | What should an un-ordered buddy do? Does any job play the game for you? |
| 4 | **orders**: none / one context button / 4 verbs | Do you give orders mid-fight at all (watch "orders given")? Which scheme do your hands accept? |
| 5 | **readability at 24 bodies**: team colour only / owner bands / + small buddies / + human halo | **Press T in each.** Can you tell humans from AI, and who owns what, in under a second? |
| 6 | **fairness probe**, bots only at 2× speed, with 1 / 2 / 3 buddies | Blue never orders, red micros. Watch a match per variant; the result prints. Keep the size where an idle squad is neither useless nor dominant, and micro still wins. |
| 7 | **online delay** 0 / 100 / 200 ms | Buddies are always interpolated online. Does the squad still feel responsive? |
| 8 | **team size** 2v2 / 3v3 / 4v4 | With your squad pick, how many players per team? 4v4 with 3 buddies is 32 bodies, the whole budget. |

Comparison 6 sets your seat to a bot, the camera to the whole arena and time to 2×. When it
ends, those settings go back to what they were. To watch more matches outside the guide, set
"your seat: bot (watch)" in the panel.

## Notes on the model

- AI grabs are instant, because the AI runs on the server. Your grabs keep the 120 ms round
  trip.
- No avian3d: same hand-rolled physics as core feel. Bodies push apart, and stand in for crowding.
- Delay is shown on unit positions only. Shots and crates are drawn in real time.
- The placeholder 2×2 turret is still *not* a decision (#17). It works for both teams now.

## Verdict

_Open: waiting for play._
