# PROTOTYPE sheet: the comeback set on the big map (issue #52)

Throwaway. Lives on `prototype/comeback-big-map` only, branched from `prototype/launch-maps`
(#42's verdict fixed: the 120 × 72 m lanes + jungle map, mirrored, ADR 0020; the Final kit
from #22; ADR 0013's knockout and comeback set; ADR 0015's 8 min timer; ADR 0012's open seats;
ADR 0016's friendly fire and randomness; ADR 0017's server grabs at 0 ms).

```
cargo run --example comeback_big_map
```

**Question:** does ADR 0013's comeback set still work on the big map? In #42's bot matches the
big map had comebacks in 8–10% of matches against about 20% on the 90 × 50 m map, with longer
hauls and rarer fights. Retune the set, or leave it? Does the 8 min timer still fit?

**Resolved when** you've played from behind (key 3) under A and at least one of B or C, and
ruled which set ships (press **Enter** on it; **P** prints your picks and results).

## Verdict

**Picked 2026-10-19: B, the set scaled to the map** (ADR 0013, amended). The human, after
playing it on the big map: "big map feels good", with B on.

- **Ships on the big map:** a megacrate every 35 s, pulled up to 30 m toward the trailing
  team at a 10-point gap, and mercy at a 12-point gap held 20 s. Lock-in 8 s, spill up to 3,
  megacrate worth 3 and overtime up to 60 s are unchanged.
- **Not shipped:** A unchanged (comebacks at about half the old map's rate in bot play), C's
  longer steal window (bots couldn't test it, and B was enough), and no set at all.

## The switch (keys 1-3, each restarts)

| Key | Switch | Options |
|---|---|---|
| 1 | **map** | **big** (ships), full 90 × 50 m (to compare) |
| 2 | **comeback set** | **A. ADR 0013 as is**; B. scaled to the map; C. B + a longer steal window; off (the reference) |
| 3 | **red head start** | **none**; 1 player; 2 players (#15's test) |

| Number | A (ADR 0013) | B scaled | C steal window |
|---|---|---|---|
| Lock-in | 8 s | 8 s | **12 s** |
| Spill, at most | 3 | 3 | **5** |
| Megacrate every | 45 s | **35 s** | **35 s** |
| Megacrate worth | 3 | 3 | 3 |
| Megacrate pull at gap 10 | 22 m | **30 m** | **30 m** |
| Overtime, at most | 60 s | 60 s | 60 s |
| Mercy | 15 pts for 20 s | **12 pts** for 20 s | **12 pts** for 20 s |

- **B scales the set to the map.** The map is 1.33× wider, so the pull reaches 1.33× further.
  Scores run about 20% lower (43 against 54), so the mercy gap is 20% smaller, and a megacrate
  comes 10 s sooner to make up for the longer hauls.
- **C also widens the steal window.** Raiders need longer to reach the leader's pad, so its
  crates stay stealable for 12 s, and a knockout spills up to 5.
- Every number is still live in the panel. Picking a set on key 2 overwrites them.

## What to try

1. **Play behind under A** (key 3 to "1 player", key 2 on A). Watch the megacrate drop on
   your side. Is it close enough to haul before red reaches it? Do you ever get a lead back?
2. **The same under B.** Is the megacrate now a real way back, or a handout that decides the
   match?
3. **The same under C, and raid red's pad.** Is there time to steal before crates lock in? As
   the leader, does 12 s feel like your score is never safe?
4. **Off, once,** to feel what the set is doing at all.
5. **Even matches (head start none) under the set you like.** Does the leader still feel
   rewarded for leading?
6. **Enter** on the set you'd ship.

## What the bot matches showed

No human, so these are a hint for what to play, not a verdict. Bots on both teams, 5v5,
headless (`CRATE_HEADLESS=1 CRATE_SEAT=bot CRATE_RULES=<map><set><head>`), 160 matches per row.
"Close" is a final gap of 5 or less. "Comeback" is the winner having trailed by 5+. "Led for
good" is when the winner took the lead for the last time (mean, in s). With 160 matches, blue
win shares within ±8 points of 50% are noise.

| Map | Set | Red head start | Blue wins | Close | Comeback | Score (both) | Lead changes | Led for good | Overtime | Mercy |
|---|---|---|---|---|---|---|---|---|---|---|
| big | A | 0 | 58% | 91% | 5% | 42 | 3.4 | 358 s | 52% | 1% |
| big | A | 1 | 62% | 86% | 9% | 44 | 2.7 | 351 s | 52% | 1% |
| big | A | 2 | 54% | 84% | 15% | 45 | 3.2 | 363 s | 52% | 0% |
| big | B | 0 | 55% | 92% | 9% | 43 | 3.8 | 394 s | 50% | 1% |
| big | B | 1 | 55% | 91% | 24% | 45 | 2.6 | 401 s | 58% | 2% |
| big | B | 2 | 64% | 89% | 16% | 46 | 3.4 | 384 s | 57% | 3% |
| big | C | 0 | 54% | 94% | 10% | 43 | 2.9 | 399 s | 44% | 1% |
| big | C | 1 | 53% | 91% | 26% | 45 | 2.9 | 400 s | 58% | 2% |
| big | C | 2 | 65% | 89% | 19% | 46 | 3.7 | 391 s | 48% | 3% |
| big | off | 0 | 46% | 83% | 1% | 37 | 2.6 | 364 s | – | – |
| big | off | 1 | 39% | 72% | 5% | 37 | 1.5 | 313 s | – | – |
| big | off | 2 | 48% | 76% | 6% | 37 | 1.5 | 333 s | – | – |
| full | A | 0 | 46% | 84% | 15% | 53 | 4.3 | 370 s | 57% | 0% |
| full | A | 1 | 46% | 86% | 31% | 56 | 5.3 | 374 s | 61% | 0% |
| full | A | 2 | 40% | 89% | 21% | 56 | 4.6 | 362 s | 52% | 1% |
| full | B | 1 | 41% | 86% | 22% | 56 | 5.5 | 372 s | 50% | 4% |
| full | C | 1 | 41% | 86% | 24% | 56 | 5.9 | 379 s | 50% | 3% |

- **The set still works on the big map, but at about half strength.** With it off, close
  matches fall from 91% to 72–83% and comebacks to 1–6%. Under A, comebacks run 5–15%
  against 15–31% on the full map.
- **B brings the big map back to about the full map's level.** Comebacks from 1 player down
  go from 9% to 24% (full map under A: 31%), and the winner takes the lead for good about
  40 s later. Close matches don't move. Almost all of it comes from the megacrate (pulled
  further, sooner), since mercy fires in 1–3% of matches either way.
- **C adds little over B in bot play,** because bots never steal off pads (0 a match on every
  row). The longer steal window can only be judged by playing it.
- **A head start barely hurts in bot play.** Blue still wins 39–65% a player or two down.
  Open seats fill in the first 3 min (ADR 0012), and bots don't press a numbers lead. So the
  "comeback" column measures swings more than real deficits. Play behind to judge it.
- **The 8 min timer still fits.** Matches average 8:00–8:06 on every row, overtime fires in
  about half of them (short: the mean match runs about 5 s past the buzzer). Whether the big
  map's quieter stretches make 8 min feel long is for play to say.
- **Nothing here changed bots or the map.** The sets are numbers only.
