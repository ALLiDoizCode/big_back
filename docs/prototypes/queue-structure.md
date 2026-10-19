# PROTOTYPE sheet: queue structure at low concurrency (issue #14)

Throwaway. Lives on `prototype/queue-structure` only.

Open `docs/prototypes/queue-structure.html` in a browser (double-click; no install, no server).
Hosted copy: https://claude.ai/artifact/42y9HT5xxh515YwV3Naemk (private to the owner).

**Question:** with about 200 concurrent players across regions, how should ranked and casual
play share one small population, and which team sizes, formats and match lengths keep waits
acceptable? Resolved when the human picks a queue structure and a feasible range of team sizes
and formats. Input: the [low-CCU matchmaking research](https://github.com/ALLiDoizCode/big_back/blob/research/low-ccu-matchmaking/docs/research/low-ccu-matchmaking.md).

## The simulation

- One simulated day (after 6 hours of warm-up), in 5-second ticks. Everyone online cycles
  through the queue, a match, then results and menus. When the curve falls, players log off
  after a match. The online count follows the curve exactly, so a player who gives up is
  replaced by a new arrival.
- Three regions (Americas 45%, Europe 40%, Asia-Pacific 15% by default), each busiest around
  21:00 local and down to a third of that at its quietest hour. Ping: about 35 ms in-region,
  95 ms Americas–Europe, 165 ms Americas–Asia-Pacific, 230 ms Europe–Asia-Pacific.
- The matchmaker takes the longest-waiting tickets first. It fills from players nearest in
  skill inside a band that starts at ±8 percentile points, doubles every 20 s and opens to
  anyone at 2 min. Neighbouring regions join after 30 s, every region after 90 s.
  Parties always share a team. Teams are split to be as even as possible, using ADR 0010's
  team strength (sum of ratings plus a party bonus).
- Match quality is the favourite's predicted chance to win (Thurstone-Mosteller, β = 25/6), with
  ratings taken as settled.
- Bots are shown as bots and play like a player 3 rating points below the lobby. A match
  with a bot in it is never rated.
- **Starting short** (ADR 0005's open question): after 60 s, a match may start once every team
  has its starting player. Players still queuing take the open seats as joined players until
  40% of the match has been played. A seat nobody takes goes to a bot if bots are allowed;
  otherwise that team plays a player down.

The logic is one pure module (`QueueSim` in the first `<script>`); the page is a shell.
Every setting the issue asks for is a knob: the daily curve, the region split, the format
(2v2, 3v3, 4v4, 5v5, 2v2v2, 2v2v2v2), match length, ranked and casual structure, modes
(rotation, pick at least k, pick one), skill-band widening, region widening, party mix, bots
and starting short. Walkthroughs: the research's 3v3, 5v5 as decided, ranked and casual apart,
timed ranked events, regions, modes, parties, a quiet launch.

## Findings (medians over 4 simulated days)

Verdict rule on the page: **Acceptable** means the median wait is at most 90 s, 90% of players
are matched within 3 min, and under 2% give up.

**Every format, one queue, 12-minute matches, 25% of players in parties.** Each cell is the
median wait / the time within which 90% were matched. Close is the share of matches whose
favourite is at most 5 points over a coin flip.

| Format | 200 at peak: peak | off-peak | 100 at peak: off-peak | 60 at peak: off-peak | Close (200) |
|---|---|---|---|---|---|
| 2v2 | 20s / 0:50 | 20s / 1:00 | 30s / 1:30 | 40s / 2:10 | 30% |
| 3v3 | 25s / 1:00 | 30s / 1:20 | 40s / 2:25 | 35s / 4:00 | 40% |
| 4v4 | 25s / 1:05 | 30s / 1:30 | 40s / 2:05 | 1:05 / 5:05 | 56% |
| **5v5** | 25s / 1:15 | 35s / 1:55 | 35s / 3:40 | 1:20 / 6:00, 4% give up | 68% |
| 5v5, starting short | 25s / 1:00 | 30s / 1:00 | 40s / 1:00 | 40s / 1:00 | 47% |
| 2v2v2 | 20s / 1:00 | 25s / 1:20 | 40s / 2:20 | 40s / 3:55 | 12% |
| 2v2v2v2 | 25s / 1:10 | 30s / 1:30 | 40s / 2:05 | 1:05 / 4:45 | 7% |

- **At 200 at peak, every format is acceptable, 5v5 included.** The quietest global hour
  still has about 130 players online, because the regions peak at different times.
  The research's "about 70 at the trough" assumed the global curve dips like a single
  region's.
- **The waits are short because the matchmaker gives things up.** By default, 94% of 5v5
  matches mix regions, and the furthest player's ping is about 95 ms at peak and 165 ms
  off-peak. Keeping regions apart puts 5v5's off-peak 90% at 6:00 with 6% giving up.
  A tight band (±4%, doubling every 40 s) roughly doubles the median, but keeps 5v5
  acceptable at 200.
- **5v5 gets fragile below about 100 at peak.** At 100, its off-peak 90% is 3:40; at 60, it is
  6:00 with 4% giving up. 2v2 and 3v3 hold up best.
- **Starting short fixes 5v5's wait at any population.** Nobody waits more than about the
  start delay (60 s). The cost: 3% of matches play a player down at 200, 14% at 100 and 28% at 60
  (or the same share with bots, when allowed), and close matches drop from 68% to 47%.
  About a quarter of players join a match already being played.
- **Small and multi-team formats are less fair.** With fewer players per team, individual skill
  gaps decide more, so close matches fall from 68% at 5v5 to 30% at 2v2 under the same band.
  2v2v2 and 2v2v2v2 rarely get close (12% and 7% within 5 points of a coin flip).
- **Parties cost fairness more than they cost time.** At 5v5, close matches go from 99% with
  solo players to 68% with 25% in parties and 31% with 60%. The off-peak 90% goes from 1:40
  to 3:20.
- **Match length matters less than population.** At 5v5 and 200, 8-minute matches give an
  off-peak 90% of 1:30 and 20-minute matches 2:55.

**Queue structures at 5v5** (off-peak median / 90%, gave up, bot seats over the day):

| Structure | 200 at peak | 100 at peak |
|---|---|---|
| One queue, ranked on top | 35s / 1:55, 0% | 35s / 3:40, 0.5% |
| One queue + timed ranked events (3 h, bots in casual) | 35s / 1:25, 6% bots | 50s / 1:30, 13% bots |
| Separate ranked and casual, bots in casual | 45s / 2:35, 8% bots | 50s / 3:20, 3.6% give up, 15% bots |
| Separate ranked and casual, no bots | 35s / 3:05, 1% give up | 40s / 3:35, 2% give up |
| One queue, pick at least 2 of 3 modes | 30s / 1:50 | 35s / 3:45, 1.6% give up |
| One queue per mode (pick 1 of 3) | 35s / 2:20, 3% give up | 35s / 3:55, 2.9% give up |

- **Splitting ranked from casual is the most expensive choice.** Ranked loses the casual players,
  and casual leans on bots. Splitting by mode (pick 1 of 3) costs about as much in give-ups.
- **"Pick at least 2 of 3" costs almost nothing** compared with server rotation.
- **Timed events look cheap on waits but leave ranked closed 21 hours a day.** Outside the
  event the queue is casual, with bots filling it at off-peak.
- At 3v3 the same ordering holds, with smaller gaps. Every structure stays acceptable at 200.
  At 100, two go over the line: the ranked and casual split without bots (90% within 3:20)
  and one queue per mode (2.4% give up).

## Verdict

_Not yet picked._ The human picks a queue structure and a feasible range of team sizes and
formats.
