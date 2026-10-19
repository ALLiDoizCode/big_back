# An 8-minute timer

A Crate Race match ([0014](0014-one-mode-at-launch-crate-race.md)) has an **8-minute timer**.
With [0013](0013-base-knockouts-with-a-comeback-set.md)'s overtime, a match runs 8 minutes plus
a few seconds, and at most 9. We chose this after playing the
[match-pacing sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/match-pacing/docs/prototypes/match-pacing.md)
for [#34](https://github.com/ALLiDoizCode/big_back/issues/34), which played 5, 8 and 12 minutes
on [0006](0006-five-v-five-on-a-lanes-and-jungle-map.md)'s 5v5 lanes and jungle map. The
human's verdict: "8 min feels right".

In bot matches (8 per row, ADR 0013's rules, one open seat per team):

| Timer | Match length | Ended on a fight (last 10 s) | Lead changes | Late joiner: share of the match left when in |
|---|---|---|---|---|
| 5 min | 305 s | 9 of 24 | about 3 | 35-45% (5 of 64 never got in) |
| **8 min** | **485 s** | **11 of 24** | about 4 | **55-60%** |
| 12 min | 725 s | 8 of 24 | about 6 | 65-75% |

## Considered options

- **5 minutes** (the low end of the research's 5-8): a late joiner waited about 2 minutes at
  the pad and got in with well under half the match left. Some never got in.
- **12 minutes** (what the queue simulation in [0012](0012-one-rated-queue-with-short-starts.md)
  assumed): lead changes doubled, but the winner still led for good from about the last third
  of the match, as at 8. The extra 4 minutes mostly added more of the same.

## Consequences

- **0012's open seats close at about 3:12** (40% of 8 minutes). In play, seats filled well
  before the cutoff, so the cutoff rarely mattered.
- **0012's queue numbers assumed 12-minute matches.** At 8 minutes, players come back to the
  queue sooner, so the simulated waits are likely pessimistic. They haven't been rerun.
- **The timer doesn't make a match end on a fight.** At every timer, most bot matches went
  quiet 10-30 s before the end, and overtime usually lasted 0-12 s. Bots fight rarely, so
  this has to be checked with humans. If it holds, the fix is in 0013's overtime or scoring,
  not the timer.
- **The mercy rule, as tuned, is dead at 8 minutes.** A 15-point gap held for 20 s fired once in
  72 bot matches, and that was at 12 minutes. Bots score about 7 points a minute between both
  teams, and the comeback set keeps gaps small. Retuning it is open.
- **A late joiner's wait at the pad costs more than the timer.** It was about 2-2.5 minutes at
  every timer, because bot teams open player recipes slowly. The prototype brought in whoever
  had waited longest, knocked out or newly seated. That is the prototype's call, not a
  decision: who a player recipe brings in first is open for the leavers and rejoins grilling
  ([#37](https://github.com/ALLiDoizCode/big_back/issues/37)).
- **Open:** mercy-rule tuning; whether matches end on a fight once humans play; who a player
  recipe brings in first.
- **Settled by [0024](0024-leavers-take-a-loss-and-their-seats-reopen.md):** a player recipe brings in whoever has waited longest.
