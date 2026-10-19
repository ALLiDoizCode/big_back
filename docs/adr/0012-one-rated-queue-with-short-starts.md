# One rated queue, with short starts

All matchmade play goes through **one queue, pooled across regions, in which every match is
rated**. The visible rank ([0011](0011-divisions-and-points-over-a-hidden-rating.md)) sits on
top of the hidden rating ([0010](0010-weng-lin-thurstone-mosteller-with-a-learned-party-bonus.md)).
Matches stay **5v5** ([0006](0006-five-v-five-on-a-lanes-and-jungle-map.md)), and a match may
**start short** once every team has its starting player. We chose this after playing the
[queue-structure sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/queue-structure/docs/prototypes/queue-structure.md)
for [#14](https://github.com/ALLiDoizCode/big_back/issues/14), which built on the
[low-CCU matchmaking research](https://github.com/ALLiDoizCode/big_back/blob/research/low-ccu-matchmaking/docs/research/low-ccu-matchmaking.md).
The human's verdict: "lets go with that, record it".

In the simulation (one day across three regions, 25% of players in parties, 12-minute matches),
these were the waits at the 4 quietest hours (median / 90% matched within):

| Players online at the busiest hour | 200 | 100 | 60 |
|---|---|---|---|
| 5v5, full lobbies only | 35s / 1:55 | 35s / 3:40 | 1:20 / 6:00 |
| **5v5, short starts** | 30s / 1:00 | 40s / 1:00 | 40s / 1:00 |
| 3v3, full lobbies only | 30s / 1:20 | 40s / 2:25 | 35s / 4:00 |

## The rules

- **One queue.** Solos and parties queue together, and every match in it is rated. There is no
  separate casual queue.
- **Modes** are rotated by the server, or each player picks at least 2. There is never a
  separate queue per mode.
- **Short starts** (settling the question [0005](0005-players-join-from-the-pad.md) left open):
  after about 60 s of waiting, a match may start once every team has its starting player.
  Players still in the queue take the **open seats** as joined players, entering when their
  team opens a player recipe, until about 40% of the match has been played.
- **A seat nobody takes** goes to a bot shown as a bot, and that match is unrated for everyone
  in it. In the sim this is about 3% of matches at 200 players online at the busiest hour, and
  about 28% at 60.
- **Skill band:** it starts narrow and doubles while a player waits (in the sim, ±8 percentile
  points, doubling every 20 s, open to anyone at 2 min).
- **Regions:** one global pool. A player is first matched within their own region, then with
  neighbouring regions after about 30 s, then with every region after about 90 s.
- **Feasible formats:** two teams, 3v3 to 5v5. 3v3 on 0006's smaller map (about 60 × 34 m)
  is the fallback.

## Considered options

- **Separate ranked and casual queues, with bots filling casual:** the most expensive option in
  every run. Ranked loses the casual players (5v5's off-peak 90% went to 2:35, or 3:05 without
  bots), and casual leans on bots.
- **One queue with timed ranked events:** cheap on waits, but ranked was closed 21 hours a day.
  Events may still come later on top of this queue
  ([0011](0011-divisions-and-points-over-a-hidden-rating.md) lists them as open).
- **One queue per mode:** about 3% of players gave up at 5v5, against almost none for
  "pick at least 2 of 3".
- **Full lobbies only at 5v5:** fine at 200 at the busiest hour, but the off-peak 90% passed
  3 minutes at about 100.
- **2v2, 2v2v2 and 2v2v2v2:** they fill fastest, but matches were rarely close (30%, 12% and 7%
  within 5 points of a coin flip, against 68% at 5v5). Three or four teams also don't fit a
  two-base lanes map.
- **Regional queues:** 5v5's off-peak 90% went to 6:00, with 6% giving up.

## Consequences

- **Most matches mix regions.** In the sim, 94% of 5v5 matches did, with about 95 ms for the
  furthest player at the busiest hours and 165 ms at the quietest. The netcode has to tolerate
  that ([netcode-budget research](https://github.com/ALLiDoizCode/big_back/tree/research/netcode-budget)).
- **A late joiner enters a match already being played.** About a quarter of players did in the
  sim. Joining through the pad hides this, but the match has to be fair to someone arriving
  late.
- **Short starts cost fairness.** Close matches fell from 68% to 47% at 5v5, because open seats
  go to whoever is queuing.
- **Parties cost fairness more than time.** At 5v5, close matches went from 99% with solo players
  only to 68% with a quarter of players in parties. Capping the skill spread within a party is
  the next lever.
- **Bots only fill seats nobody takes.** Bot quality matters less than with bot-filled lobbies,
  but a bot must still haul, stack and steal well enough not to break the match.
- **The sim is optimistic at low population:** a player who gives up is replaced, so long waits
  never shrink the pool there. Real launch numbers need watching.
- **Open:** the exact start delay, how long seats stay open, the band and region timings, the
  party skill cap, the leaver and rejoin policy, and showing players an estimated wait.
- **Settled by [0017](0017-leavers-take-a-loss-and-their-seats-reopen.md):** the leaver and rejoin policy. A leaver's seat reopens like a
  short-start seat, and a leaver never makes a match unrated.
- **Settled by [0018](0018-a-party-cap-and-flags-held-for-review.md):** the party cap's shape: shown in rank, enforced on the wider of rank
  and rating, with a full 5-stack exempt for fewer points. Its widths are set by
  [#47](https://github.com/ALLiDoizCode/big_back/issues/47).
- **Settled by [0019](0019-a-tutorial-then-the-rated-queue.md):** the seat bot is also the
  tutorial's and practice's bot, run scripted in the tutorial.
