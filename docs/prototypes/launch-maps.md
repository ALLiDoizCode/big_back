# PROTOTYPE sheet: launch maps for Crate Race (issue #42)

Throwaway. Lives on `prototype/launch-maps` only, branched from `prototype/latency` (the
5v5 lanes + jungle map from ADR 0006, the Final kit from #22, ADR 0013's knockout with the
comeback set, ADR 0015's 8 min timer, ADR 0012's open seats, ADR 0016's friendly fire and
randomness, ADR 0017's server grabs judged on what you saw).

```
cargo run --example launch_maps
```

**Question:** Season 1 has one mode (ADR 0014), so the maps carry the variety. How many maps
ship, and what are they? ADR 0006's lanes + jungle layout alone, plus its ~60 × 34 m smaller
version (also ADR 0012's 3v3 fallback), or distinct layouts? Mirror or rotational symmetry?
How does the queue pick among them? Weigh each new map against the work of tuning bots and
the comeback set for it.

**Resolved when** you've played at least two candidate layouts against the full map, in both
symmetries, tried the queue's picks, and ruled on what ships (press **Enter** on a map you'd
ship; **P** prints your picks, the maps played and every result).

## Verdict

**Picked 2026-10-19: one map, the big lanes + jungle map, mirrored** (ADR 0020). The human,
after playing the full map: "the map seems smaller than a League of Legends map and there
needs to be more jungle and the base area should be bigger". The big map was built for that,
and after playing it: "I like the bigger map". Asked, they picked one map over two or three,
mirror over rotational, and said the big map didn't feel too quiet.

- **Ships:** big (120 × 72 m), its 20 m walled base areas, four roomed jungle quadrants with a
  heal and a shield camp each, and the 10 m river with its two objectives.
- **Not shipped:** the 90 × 50 m map, the small map (and with it ADR 0012's 3v3 fallback),
  crossroads and canal.
- **The queue picks nothing:** with one map, keys 3 and 4 are unused.
- **Not settled here:** wall, camp and crate placement on the big map, and whether the
  comeback set needs retuning for its longer hauls and rarer fights (see the bot results).

## The maps (key 1)

Every map is drawn as **blue's half**, and red's half is made from it by the symmetry switch
(key 2). Walls are tall: players, crates and shots don't pass.

| Map | Size | m² per player at 5v5 | What it is |
|---|---|---|---|
| **full** | 90 × 50 m | 450 | ADR 0006's lanes + jungle: three lanes, four walled jungle quadrants (heal camps north, shield camps south), the river with an objective north and south |
| **small** | 60 × 34 m | 204 | The same layout scaled to ADR 0006's smaller size. Walls, lanes and entrances shrink with it; camps keep their hp, damage and 9 m leash |
| **crossroads** | 72 × 44 m | 317 | Two side lanes and a wide mid lane with a pillar at the centre. Each side has **one** walled jungle (heal camp) on one flank and an open field with low cover (shield camp) on the other. A plaza in the middle holds the objectives |
| **canal** | 76 × 40 m | 304 | No jungle. A fenced canal down the middle with three bridges (the objectives sit on the side bridges). Two long walls split each half into three lanes, which merge at the base and at the canal |
| **big** (the default) | 120 × 72 m | 864 | Added after the first play ("the map seems smaller than a League of Legends map, there needs to be more jungle and the base area should be bigger"). Bases 20 m deep (against 14), walled, with exits onto the three lanes and a gate into each jungle. Jungle quadrants 35 × 22 m (against 27 × 13), split into rooms by inner walls, each with a heal and a shield camp. A 10 m river with an objective north and south |
| open | 46 × 28 m | 129 | The #10 arena, kept to compare |

- **Mirror** (key 2): red's half is blue's, flipped east-west. Blue's jungle and red's are
  both on the north side.
- **Rotational** (key 2): red's half is blue's, turned 180°. On crossroads that puts red's
  jungle in the south, so each side lane has a jungle at one end and a field at the other.
  On the lanes maps the walls don't change (they're already symmetric north-south); only
  the camps swap, so red's heal camp is in the south.

## The map switch (keys 1-5)

| Key | Switch | Options |
|---|---|---|
| 1 | **map** (restarts) | full, small, crossroads, canal, **big**, open arena |
| 2 | **symmetry** (restarts) | **mirror**, rotational |
| 3 | **queue picks** the next map, when a match ends | **you pick (key 1)**, rotate through the pool, random from the pool, random but never the same map twice running |
| 4 | **map pool** for key 3 | full only, **full + small**, full + small + crossroads, full + small + crossroads + canal, big only, big + full |
| 5 | **small map plays** | **5v5**, 4v4, 3v3 (ADR 0012's fallback) |

- **The HUD** shows the map, its symmetry, the team size and floor per player, what the
  queue picks next, and the last 8 maps played.
- **Each match prints a `maps` line:** the map, its symmetry and size; the score, gap and
  whether it was close (≤ 5) or a comeback (the winner trailed by 5+); kills, PvE deaths and
  the first kill; fights a minute; crates picked up and seconds each spent carried;
  knockouts; lead changes; and bot-seconds stood still away from the bases.

**Calls the issue didn't make, made here so they can be played (say if any is wrong):**

1. **The small map is the full one, scaled.** Walls, entrances and lanes all shrink by
   2/3 (x) and 0.68 (z). Lanes are about 4.8 m wide, entrances about 2.7 m.
2. **Camps don't scale.** Every map has a heal and a shield camp per side and two objectives
   in the middle, with #20's hp, damage, leash and drops.
3. **The canal is fenced.** Like a wall, nothing crosses it (shots included) except at the
   bridges. Shooting across water is a later question.
4. **No bot code was changed per map.** Bots find their way on a grid built from each map's
   walls, guard and raid relative to the pads, and take camps from each map's camp list.
   What a map adds is data: its walls, starting crates, camps, gate chokes and how wide the
   middle is that new crates land in.
5. **The queue picks only when a match ends on its own.** R, and keys 1 and 2, restart on the
   map on key 1.
6. **Ping is 0 ms** (#36's verdict otherwise stands), so maps compare without latency.

## What to try

1. **Play full, then crossroads, then canal** (key 1), the same class each time. Where do
   fights happen? Is the haul tense on each? Does any map feel like the same match on a
   different floor?
2. **Rotational symmetry on crossroads** (key 2). You now share each side lane with a red
   jungle on the far end. Does that feel fair, or does it feel like one side has the better
   lane?
3. **The small map at 5v5, then 3v3** (key 5). Is 5v5 too crowded? Watch the camps: do
   players walk into them while crossing lanes?
4. **The queue** (keys 3 and 4). Watch a few bot matches (`your seat: bot` in the panel) with
   rotate, then random. Does rotation feel predictable in a good way or a stale way?
5. **Enter** on each map you'd ship.

## What the bot matches showed

No human, so these are a hint for what to play, not a verdict. Bots on both teams, 5v5
unless marked, ADR 0015's 8 min timer, headless (`CRATE_HEADLESS=1 CRATE_SEAT=bot`). 160
matches per row (40 for big). "Close" is a final gap of 5 or less; "comeback" is the winner
having trailed by 5+. Treat blue-win shares within ±8 points of 50% as noise at 160 matches
(±15 at 40).

| Map | Sym | m²/player | Score (both) | Kills | PvE deaths | First kill | Fights/min | Carry (s) | Knockouts | Close | Comeback | Blue wins | Bot-s still |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| big | mirror | 864 | 43 | 12.4 | 2.0 | 52 s | 3.8 | 7.2 | 0.70 | 95% | 10% | 60% | 1 |
| big | rot | 864 | | 11.9 | 2.4 | 63 s | 4.1 | 6.8 | 0.72 | 98% | 8% | 48% | 2 |
| full | mirror | 450 | 54 | 20.4 | 3.9 | 40 s | 5.2 | 6.9 | 0.88 | 80% | 19% | 48% | 8 |
| full | rot | 450 | | 20.5 | 4.3 | 41 s | 5.4 | 7.0 | 0.90 | 80% | 20% | 41% | 9 |
| small | mirror | 204 | 62 | 27.9 | 12.1 | 24 s | 5.6 | 2.9 | 0.39 | 87% | 26% | 55% | 12 |
| small | rot | 204 | | 26.9 | 13.1 | 26 s | 5.9 | 3.0 | 0.44 | 76% | 27% | 54% | 18 |
| small 4v4 | mirror | 255 | | 28.1 | 11.7 | 23 s | 5.6 | 3.0 | 0.33 | 80% | 22% | 53% | 13 |
| small 3v3 | mirror | 340 | | 28.0 | 12.3 | 24 s | 5.6 | 2.9 | 0.38 | 82% | 31% | 58% | 9 |
| crossroads | mirror | 317 | 66 | 25.9 | 5.9 | 20 s | 6.1 | 5.0 | 0.78 | 84% | 31% | 58% | 9 |
| crossroads | rot | 317 | | 26.3 | 6.2 | 20 s | 6.0 | 5.0 | 0.67 | 87% | 30% | 48% | 14 |
| canal | mirror | 304 | 68 | 25.8 | 5.7 | 22 s | 7.1 | 4.7 | 1.41 | 80% | 33% | 48% | 17 |
| canal | rot | 304 | | 25.6 | 6.1 | 22 s | 7.2 | 4.6 | 1.48 | 82% | 35% | 46% | 19 |

- **The big map is the emptiest:** about 40% fewer kills than full, the first kill near a
  minute in, the fewest fights a minute and the lowest scores. Its matches are almost always
  close, but because little scores, not because of comebacks (8-10%, the lowest). This is
  the #20 worry again (3v3 on the full map was "too empty"): bots only fight what they see
  within 10 m, so humans hunting fights would meet more, but the gap is large.
- **The small map's camps don't fit it.** PvE deaths triple (12-13 a match, against 4 on
  full): at #20's 9 m leash, the camps reach into the narrowed lanes. Shipping it means
  retuning the camps (leash, hp, spots), not just the walls.
- **Crossroads and canal are the busiest at their size:** the most fights a minute, the
  first kill at about 20 s, and the most comebacks (30-35%). Canal has the most knockouts
  (about 1.4 a match): the bridges funnel raiders straight to the base.
- **Symmetry doesn't move side fairness** on any map within the noise. Rotational only
  changes which side's camps are where; on crossroads it gives each side lane a jungle at
  one end, which bots can't judge.
- **Bots coped on every map with no code changed per map.** Bot-seconds stood still stay
  under 20 a match everywhere (most on canal and small, where lanes are narrow). The bot
  cost of a map is its data (walls, crate spots, camps, gate chokes), plus whatever humans
  find bots doing wrong on it. The comeback set (megacrate pull, spill, overtime, mercy) ran
  unchanged too; the comeback share is what moved, from 8% on big to 35% on canal.
