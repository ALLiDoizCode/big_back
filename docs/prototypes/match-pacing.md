# PROTOTYPE sheet: match length and pacing of the crate race (issue #34)

Throwaway. Lives on `prototype/match-pacing` only, branched from `prototype/production-loss`
(the 5v5 lanes + jungle map from ADR 0006, the Final kit from #22, and ADR 0013's base
knockouts with the comeback set, now on by default).

```
cargo run --example match_pacing
```

**Question:** how long is a match of Crate Race? The research suggests 5-8 min plus
overtime. The queue simulation (ADR 0012) assumed 12 min. A round trip from mid takes about
12 s. Does a match end on a fight, not a buzzer? When does the mercy rule fire? How much of
the match does a late joiner (ADR 0012's open seats) still get to play?

**Resolved when** you've played at least two timers and picked one (press **Enter** on the
timer you like; **P** prints your picks).

## What's new

| Key | Switch | Options |
|---|---|---|
| 7 | **timer** (restarts the match) | 5 min, **8 min**, 12 min |
| 8 | **open seats** per team (restarts) | none, **1 per team**, 2 per team |
| 9 | **you** (restarts) | **start the match**, take blue's open seat (late joiner) |

Keys 1-6 are the #15 switches, now set to ADR 0013's pick: knockout, lock-in, spill,
megacrates nearer the trailing team, overtime and mercy, all on.

- **Open seats (ADR 0012).** The top slots of each team start empty. Each is filled from the
  queue at a random time in the first 2 min (`open seats: filled within`), and by 40% of
  the timer at the latest (`open seats: until`): a seat nobody took goes to a bot then. A
  filled seat waits at its pad until the team opens a player recipe.
- **Who a player recipe brings in.** Whoever has waited longest: a player knocked out at
  2:00 comes in before a late joiner seated at 2:30. Before this, the lowest slot always
  came first, and in bot matches no late joiner ever got in.
- **Key 9** puts you in blue's last open seat. Blue's last bot starts the match in your place.
  You watch your pad until the seat fills and blue opens a player recipe.
- **The HUD** shows the timer, time played, how long since the last fight, who leads and
  since when, and each open seat (queued, waiting at the pad, or in).
- **A fight** is a kill, a steal off a pad, or a hit on a base. A match **ends on a fight** if
  one came in its last 10 s (`fight: within`).

**Calls the issue didn't make, made here so they can be played (say if any is wrong):**

1. **A seat nobody takes goes to a bot at the cutoff,** not at the start. ADR 0012 says the
   seat goes to a bot but not when.
2. **First come, first served at the pad** (above). ADR 0005 doesn't say who a player recipe
   brings back when several players are out.
3. **The timer cutoff for open seats scales with the timer** (40% of 5 min is 2:00, of 12 min
   is 4:48), as ADR 0012 words it. The queue fills seats in seconds, not in percent, so the
   fill window is fixed at 2 min.

## What to try

1. **Play a 5 min and an 8 min match** (key 7), with the defaults. Ask at the end: was the
   result decided before the buzzer? Did the last minute have a fight in it, or was it a
   walk home?
2. **Play a 12 min match.** Does the middle drag? Does anything new happen after 8 min?
3. **Fall behind on purpose** (`red head start` 2 in the panel). When does the mercy rule
   end it, and does that feel like a relief or a cut-off?
4. **Join late** (key 9). How long do you wait at the pad, and is the match still worth
   joining when you get in? Try it on 5 min and on 12 min.
5. **Enter** records the timer and rules you're playing as your pick.

## What the bot matches showed

Bots on both sides, the 5v5 lanes + jungle map, ADR 0013's rules (knockout, all five comeback
knobs on), 8 matches per row, run at 10× speed. "Head start" is red starting with 2 extra
players. **Treat a difference of 2 in 8 as noise.**

| Timer | Start | Open seats | Ended by time / overtime / mercy | Length | Ended on a fight | Fights in the last minute | Winner led for good from | Gap at the buzzer | Final score (both teams) |
|---|---|---|---|---|---|---|---|---|---|
| 5 min | even | 1 each | 3 / 5 / 0 | 305 s | 6 of 8 | 5.9 | 66% of the match | 3.2 | 36 |
| 5 min | head start | 1 each | 3 / 5 / 0 | 305 s | 1 of 8 | 5.1 | 71% | 2.4 | 35 |
| 5 min | even | 2 each | 2 / 6 / 0 | 315 s | 2 of 8 | 3.9 | 89% | 1.2 | 32 |
| 8 min | even | 1 each | 3 / 5 / 0 | 485 s | 1 of 8 | 2.6 | 58% | 4.9 | 56 |
| 8 min | head start | 1 each | 4 / 4 / 0 | 484 s | 7 of 8 | 5.0 | 72% | 2.8 | 54 |
| 8 min | even | 2 each | 3 / 5 / 0 | 484 s | 3 of 8 | 5.5 | 72% | 3.9 | 54 |
| 12 min | even | 1 each | 2 / 5 / 1 | 715 s | 4 of 8 | 5.0 | 90% | 2.0 | 78 |
| 12 min | head start | 1 each | 4 / 4 / 0 | 724 s | 3 of 8 | 3.9 | 88% | 3.2 | 82 |
| 12 min | even | 2 each | 3 / 5 / 0 | 730 s | 1 of 8 | 3.4 | 71% | 3.5 | 80 |

**Late joiners** (the open seats in the rows above, pooled by timer):

| Timer | Late joiners who got in | Seat filled → in the match | Share of the match left when in | Seconds alive in the match |
|---|---|---|---|---|
| 5 min | 59 of 64 | 2:10 | 35-45% | 15 |
| 8 min | 64 of 64 | 2:30 | 55-60% | 30 |
| 12 min | 64 of 64 | 2:30 | 65-75% | 45-50 |

**What it suggests** (bots, so a hint for what to play, not a verdict):

1. **A match is the timer plus a few seconds.** Overtime fired in over half the matches but
   usually lasted 0-12 s. It ran to the 60 s cap once in 72. Matches are close at the buzzer
   (a 1-5 point gap), so overtime is on often but the trailing team rarely keeps contesting.
2. **Most matches don't end on a fight, at any timer.** 28 of 72 had a kill, steal or base
   hit in their last 10 s: 9 of 24 at 5 min, 11 at 8 min, 8 at 12 min. The typical match went
   quiet 10-30 s before the end. Bots fight about 4-5 times a minute near the end, so this
   is the number most likely to change with humans, and the one to judge in play. The timer
   isn't what decides it.
3. **The mercy rule almost never fires.** Once in 72, at 10:46 of a 12 min match (red
   25-40). Bots score about 7 points a minute between both teams, and the comeback set keeps
   the gap under 15. Even a 2-player head start never got there. At 5 or 8 min, mercy as
   tuned is a dead rule.
4. **Longer matches stay close longer,** but they aren't decided later in the match. Lead
   changes rose from about 3 a match at 5 min to about 6 at 12 min. The winner still led for
   good from roughly the last third onwards at every timer.
5. **A late joiner waits about 2-2.5 min at the pad,** whatever the timer, because bot
   teams open player recipes slowly and anyone knocked out earlier comes in first. At 5 min,
   that leaves a late joiner well under half the match, and 5 of 64 never got in. At 8 min
   it's over half; at 12 min about two thirds. The wait, not the timer, is the main cost:
   the cutoff (40% of the timer) almost never mattered, since seats filled within 2 min.
6. **Bot fights are thin.** A joined player lives about 15-50 s in the match. So "ends on a
   fight" and "late joiner gets to play" are both pessimistic here, and both need a human
   playing.

## Notes on the model

- Smoke runs: `CRATE_RULES=<9 digits>` sets keys 1-9 (ADR 0013's pick with an 8 min timer,
  1 open seat each and you starting is `111211110`), `CRATE_HEADSTART`, `CRATE_MATCHES=<n>`
  exits after n matches, `CRATE_SPEED` goes up to 10, `CRATE_SEED` varies the randomness.
  Each match prints a `pacing ...` line with every number below, after #15's `result ...`
  line.
