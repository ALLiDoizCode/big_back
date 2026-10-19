# PROTOTYPE sheet: production loss and comebacks (issue #15)

Throwaway. Lives on `prototype/production-loss` only, branched from `prototype/class-items`
(the lanes + jungle map at 5v5, the crate grammar from #17 and the Final kit from #22).
Input: [arena win-conditions research](https://github.com/ALLiDoizCode/big_back/blob/research/arena-win-conditions/docs/research/arena-win-conditions.md) §9.

```
cargo run --example production_loss
```

**Question:** what happens when a team's production is attacked, and how does a losing team
come back? Brawl Stars Siege, the closest shipped analogue (carry, then production, then
base hp), was removed for having the highest termination rate of any mode.

**Resolved when** you've played the four production-loss rules, picked one, and picked a set
of comeback knobs (press **Enter** on the rules you like; **P** prints your picks).

## Verdict

**Picked 2026-10-19: knockout, with all five comeback knobs on.** The human, after playing it:
"knockout with all comeback knobs on feels good". As played, with the defaults below:

- **Knockout:** a base has 750 hp. At 0 its pad is down for 20 s (it makes nothing and scores
  nothing new), the attackers get +3, and then the base rebuilds at full hp.
- **Lock-in:** a crate scores and can no longer be stolen once it has rested on its pad for 8 s.
- **Spill:** a knocked-out pad throws at most 3 of its crates that aren't locked in onto the
  map. The rest stay on the pad.
- **Megacrates nearer the trailing team:** worth 3, one every 45 s while none is out, pulled
  up to 22 m toward the trailing team's base at a 10-point gap.
- **Overtime:** play goes on past the buzzer while the trailing team carries a crate or stands
  at the leader's pad, for at most 60 s.
- **Mercy:** a 15-point gap held for 20 s ends the match.

Rejected: permanent destruction (it ended 3-5 of 8 bot matches early by elimination: the
Siege pattern), rounds (bot rounds drew often and dragged to 9-10 min), and theft only (a
2-player head start won 8 of 8 with the knobs off). The numbers are tuning, not decisions.
Recorded in ADR 0013 (PR #32).

## What's new: the base

Each team has a **base**: the block behind its pad (blue or red, about 1.4 × 1.4 × 1.8 m).
Enemy shots hurt it, and so do rockets, grenades and tank shells that go off on it. It has
**750 hp** (`base: hp` in the panel), so the standard gun alone takes about 10 s. It shrinks as
it takes damage, with an hp bar above it, and lies flat while it's down. Your own team's shots
pass through it.

The **score** stays what it was: crates resting on your pad plus crates spent there on recipes.
Most points at the end wins.

## The rule switch (keys 1-6)

| Key | Switch | Options |
|---|---|---|
| 1 | **production loss** (restarts the match) | permanent, **knockout**, rounds, no base |
| 2 | lock-in | **off**, on |
| 3 | spill | **off**, on |
| 4 | megacrates | **off**, in the river, nearer the trailing team |
| 5 | overtime | **off**, on |
| 6 | mercy | **off**, on |

Keys 2-6 are live: the match keeps going. **R** restarts. The kit is the Final set (#22);
**C** changes your class.

### 1. Production loss: what a base kill does

| Rule | At 0 hp | How the match ends |
|---|---|---|
| **Permanent** (Team Buddies decapitation) | The pad is gone for good: it makes nothing, nobody joins, and the starting player stops respawning. Whatever is left on it is banked. | A team with no base and nobody left loses at once. Otherwise most points at time. |
| **Knockout** | The pad is **down for 20 s**: it makes nothing and scores nothing new. The attackers get **+3**. Then the base rebuilds at full hp. | Most points at time. |
| **Rounds** | The attackers **win the round** at once. | **Best of 3**, 150 s rounds. Each round starts fresh: one player each, crates back in the middle. A round that runs out of time goes to whoever has more points. A drawn round counts for nobody (at most 6 rounds). |
| **No base: theft only** | There's no base. Production is only lost by theft: taking crates off the enemy pad. | Most points at time. |

In the permanent and knockout rules, the pad's crates are **lost** when its base falls: the ones
on top go first. Spill (key 3) decides how.

### 2-6. Comeback knobs

| Knob | Off | On |
|---|---|---|
| **2 lock-in** | A crate scores the moment it rests on your pad, and can be stolen until it's kicked open. | A crate scores only once it has rested in a pad cell for **8 s**. Until then it can be stolen and doesn't count (a ring on it fills up). Then it **locks in**: it scores, nobody can steal or knock it, it still counts for recipes, and it survives a base fall. |
| **3 spill** | A fallen base's crates are **destroyed**. | They're **thrown onto the map** toward the middle, and **at most 3** go (the rest stay on the pad). Anyone can pick them up. |
| **4 megacrates** | None. | A **megacrate** (gold, with a beam) drops every **45 s** while none is out. It's worth **3 points** on a pad and is one cell in a recipe. "In the river" drops it in the middle. "Nearer the trailing team" pulls it toward the trailing team's base: 2.2 m per point of the gap, up to 22 m at a 10-point gap. |
| **5 overtime** | The match ends on the buzzer. | At the buzzer, play goes on while the **trailing team carries a crate or stands within 5 m of the leader's pad**, and ends 1 s after they stop (at most 60 s). A tie runs until someone leads. |
| **6 mercy** | None. | A **15-point gap held for 20 s** ends the match (or the round) for the leader. |

**Calls the issue didn't make, made here so they can be played (say if any is wrong):**

1. **The base is a separate block behind the pad,** not the pad itself. Shooting the pad's crates
   still does nothing (#17 picked "steal only").
2. **Lock-in locks the crate,** so stealing has a window and then stops. #17 rejected "locks once
   placed" because it stopped all theft. Here a crate locks only after the timer.
3. **"Capped loss" means a fallen base loses at most 3 crates,** spilled where anyone can grab
   them. Theft itself is already physical (you carry the crate home, and a hit drops it), so
   spill doesn't change stealing.
4. **The economy gap is the score gap.** The score already counts the economy: crates on your
   pad plus crates spent there, which includes the players you brought in.
5. **Megacrates are new.** Nothing in the glossary covered them, so they're added here as a crate
   worth more points. They are not super crates: super crates still upgrade a recipe.

## What to try

0. **Play one match on each production-loss rule** (key 1), with the knobs off. Shoot the red
   base: it's the red block behind red's pad. **K** drops red's base to 1 hp, to see a base
   fall quickly (**Shift+K** does it to yours).
1. **Fall behind on purpose.** Set `red head start` in the panel to 2 (red starts with 3
   players to your 1). Under each rule, ask: **once you're behind, is there still something
   worth doing?** Or does the match feel over? That's the Siege question.
2. **Permanent:** lose your base (Shift+K, then get it shot). Does playing on without a pad or
   respawns feel like a fight or a wait? A team with nobody left loses at once.
3. **Knockout:** is 20 s down long enough to matter, and short enough not to decide the match?
   Does +3 feel like a reward or noise? (`knockout: pad down`, `knockout: bonus`)
4. **Rounds:** does a reset after a base kill feel like a fresh chance, or does it wipe out
   what you earned? Is a 150 s round long enough to build anything?
5. **Comebacks:** turn the knobs on one at a time (keys 2-6) and replay the head-start match.
   - **Lock-in:** do you race to steal crates before they lock? Does it make defending your pad
     matter more?
   - **Spill:** do spilled crates start a scramble, or just change hands?
   - **Megacrates:** **U** drops one at your feet. Does "nearer the trailing team" feel fair,
     or like a handout? (Research §9.2.5: comeback levers have to stay visible.)
   - **Overtime:** does the leader have to win one last fight?
   - **Mercy:** do hopeless matches end sooner, without cutting off real comebacks?
6. **Enter** records the rule and knobs you're playing as your pick.

## What the bot matches showed

Bots on both sides, the 5v5 lanes + jungle map, 5 min matches (rounds: best of 3, 150 s each),
8 matches per row, run at 10× speed. "Knobs on" means lock-in, spill, megacrates nearer the
trailing team, overtime and mercy, all at once. **Treat a difference of 2 wins in 8 as noise.**
Reruns of the same settings vary by that much: spill does nothing under theft only, and the
spill-only row below still came out 1 win and 1 draw different.

**Red starts with 2 extra players (blue is the team behind):**

| Rule | Knobs | Blue wins / draws | Ended by | Bases fallen per match (blue / red) | Lead changes | Biggest red lead | Blue 10+ behind | Length |
|---|---|---|---|---|---|---|---|---|
| permanent | off | 5 / 0 | time 4, elimination 4 | 0.4 / 0.1 | 3.1 | 1.6 | 0 s | 251 s |
| permanent | on | 3 / 0 | elimination 3, time 3, overtime 2 | 0.4 / 0.1 | 4.2 | 2.6 | 0 s | 279 s |
| knockout | off | 5 / 0 | time 8 | 0.5 / 0.2 | 2.1 | 5.0 | 17 s | 300 s |
| knockout | on | 3 / 0 | time 4, overtime 4 | 0.4 / 0.0 | 4.6 | 4.0 | 0 s | 303 s |
| rounds | off | 0 / 1 | rounds 8 | 0.5 / 0.0 | 9.4 | 3.2 | 0 s | 609 s |
| rounds | on | 4 / 0 | rounds 8 | 0.2 / 0.4 | 5.8 | 3.2 | 0 s | 366 s |
| theft only | off | **0 / 0** | time 8 | — | 1.0 | **11.0** | **43 s** | 300 s |
| theft only | on | **6 / 0** | overtime 6, mercy 1, time 1 | — | 3.5 | 8.8 | 10 s | 311 s |

**Even start:**

| Rule | Knobs | Blue wins / draws | Ended by | Bases fallen per match (blue / red) | Lead changes | Length |
|---|---|---|---|---|---|---|
| permanent | off | 4 / 1 | time 5, elimination 3 | 0.1 / 0.2 | 1.8 | 276 s |
| permanent | on | 4 / 0 | elimination 5, overtime 2, time 1 | 0.4 / 0.2 | 2.5 | 259 s |
| knockout | off | 3 / 1 | time 8 | 0.4 / 0.2 | 2.0 | 300 s |
| knockout | on | 3 / 0 | overtime 5, time 3 | 0.4 / 0.0 | 4.4 | 306 s |
| rounds | off | 5 / 1 | rounds 8 | 0.0 / 0.0 | 7.4 | 544 s |
| rounds | on | 2 / 0 | rounds 8 | 0.2 / 0.0 | 4.8 | 338 s |
| theft only | off | 6 / 0 | time 8 | — | 3.8 | 300 s |
| theft only | on | 1 / 0 | overtime 6, mercy 2 | — | 3.1 | 310 s |

**One knob at a time** (theft only, red head start; the off row is the theft-only row above):

| Knob on | Blue wins / draws | Biggest red lead | Blue 10+ behind | Overtime |
|---|---|---|---|---|
| none | 0 / 0 | 11.0 | 43 s | — |
| lock-in | 2 / 0 | 10.4 | 38 s | — |
| spill (does nothing without a base: shows the noise) | 1 / 1 | 11.0 | 45 s | — |
| megacrates in the river | 0 / 0 | **23.6** | **182 s** | — |
| megacrates nearer the trailing team | 0 / 1 | 10.9 | 60 s | — |
| overtime | 0 / 0 | 12.8 | 68 s | 20 s |
| mercy | 0 / 0 | 11.0 | 48 s | — |
| all five | **6 / 0** | 8.8 | 10 s | 26 s |

**What it suggests** (bots, so a hint for what to play, not a verdict):

1. **A base to shoot evened out the head start; theft alone didn't.** Without a base, red's 2
   extra players hauled, and red won 8 of 8 with blue 10+ behind for 43 s a match. With a
   base, blue won 5 of 8 under permanent and under knockout. But bases fell only about once
   every two matches. What evened things out is the **cost of raiding**: red's extra players
   left the haul to go shoot a base. The base kills themselves did little. A human who goes
   looking will kill bases far more often.
2. **Permanent ends matches early.** 3-5 of 8 ended by elimination, and the average match ran
   4-4.5 min. This is the Siege pattern: once a base is gone, the match is just waiting to end.
3. **Rounds need a tiebreak.** Bot rounds score low and often draw, so best of 3 dragged on to
   9-10 min (up to 6 rounds). With overtime on to break ties, it took about 6 min.
4. **No single knob turns a match around; all five together do.** Lock-in helped a little. Overtime
   alone never changed a result: it added 20 s and the leader won the last fight. Mercy hardly
   ever fired at a 15-point gap.
5. **A megacrate in the river makes the leader stronger.** It doubled red's biggest lead and kept
   blue 10+ behind for 3 min. Pulled toward the trailing team, it was neutral. So megacrates
   only work as a comeback if they drop on the trailing side.

## Notes on the model

- **Bot raids.** When a team has two or more players up, its highest living slot raids
  (`bots raiding per team`, 1 by default). Raiders never include the starting player, who hauls
  and kicks, and take the jungler last. A raider walks to a post 6.5 m in front of the enemy base
  and shoots it, fighting only enemies within 5 m. Once its own base is destroyed, a whole team
  raids. Bots within 30 m come home when their base is hit.
- **Teams are small in bot matches**: about 1.2-1.6 players up on average, out of 5, because
  joined players stay out once knocked out (ADR 0005). Raiders spend most of their raid walking
  or dying, so bases fall less often than they would to a human who goes looking.
- **Red head start** (`red head start`, 0-4) brings that many of red's players in at the start.
  It's there to make a losing team on purpose.
- A pad that's down (knocked out) shows as `DOWN` in the HUD. Bots carrying a crate wait beside
  it until it rebuilds.
- Smoke runs: `CRATE_RULES=<6 digits>` sets keys 1-6 (e.g. `111211`), `CRATE_HEADSTART`,
  `CRATE_MATCHES=<n>` exits after n matches, `CRATE_SPEED` goes up to 10. Each match prints a
  `result ...` line with every number above.
