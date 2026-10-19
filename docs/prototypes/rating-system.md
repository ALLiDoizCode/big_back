# PROTOTYPE sheet: rating system (issue #11)

Throwaway. Lives on `prototype/rating-system` only.

Open `docs/prototypes/rating-system.html` in a browser (double-click; no install, no server).
Hosted copy: https://claude.ai/artifact/RQpd72GMcY2cMky1axRWMZ (private to the owner).

**Question:** which rating algorithm should drive matchmaking and the ladder (Elo, Glicko-2,
TrueSkill, Weng-Lin/OpenSkill), and how are parties and team results scored? How many
matches does it take to settle a player at 2v2, 3v3, 4v4 (and 5v5, per ADR 0006)? Is
rating anything beyond win/loss worth it before there is match data? Should lopsided
matches get a rating cushion?

## The simulation

- 200 players with a hidden real skill (normal, mean 25, sd 6). A team's performance is the
  sum of its players' skill plus per-player luck (TrueSkill's own model, β = 25/6). That
  choice favours the Gaussian systems (TrueSkill, Weng-Lin Thurstone-Mosteller) a little.
- Each round, 85% of players queue. The matchmaker sorts by rating plus noise (its
  looseness setting stands in for low CCU), fills lobbies, and splits each lobby into the
  most even teams it can find under that system's own predictions.
- Parties have a real bonus per extra member (TrueSkill 2's squad finding), and by default
  queue together 70% of the time.
- Every system plays its own copy of the same seeded population.
- **Settled** = a player's ladder position stays within 15 places in 100 of where their
  real skill belongs, for the rest of the run.

The logic is one pure module (`RatingSim` in the first `<script>`); the page is a shell.

## Findings (seed 11, 150 rounds ≈ 125 matches each, no parties)

Median matches to settle:

| | 2v2 | 3v3 | 4v4 | 5v5 |
|---|---|---|---|---|
| Elo (team average) | 88 | 91 | 101 | 107 |
| Glicko-2 (per match, composite opponent) | 15 | 22 | 43 | 55 |
| TrueSkill | 23 | 27 | 34 | 45 |
| Weng-Lin Bradley-Terry (skillratings) | 39 | 57 | 70 | 75 |
| Weng-Lin Thurstone-Mosteller (openskill) | 25 | 38 | 45 | 50 |

Across three seeds, Thurstone-Mosteller Weng-Lin sits within a few matches of TrueSkill at
every size, and Bradley-Terry Weng-Lin needs about 1.5–2× as many. Scaling Weng-Lin's β by
team size did not help, so it was dropped.

- **Team size:** settling roughly doubles from 2v2 to 5v5 for the good systems. At 5v5 a
  new ladder needs about 45–50 matches per player before the median player is in the right
  band.
- **Parties:** with "team = sum of players", party members drift above their real skill
  (the system credits them with the teamwork). A party bonus learned online from results
  finds a value near the real one and gives the most accurate ladder. "Party = its best
  player" makes parties earn less per win. If friends *only* ever queue together, no rule
  can separate their skill from their teamwork (accuracy falls from about 0.94 to 0.82).
- **New players:** starting at the average with full uncertainty placed both ordinary new
  players and smurfs fastest (smurfs in about 7 matches). Starting one tier low (Valorant)
  roughly doubled smurfs' climb; extra uncertainty won most of it back.
- **Match stats (kills, crates):** with weakly informative stats and 15% stat farmers,
  adding them lowered accuracy (0.95 → 0.90) and lifted farmers about 18 places in 100.
  With honest stats (70%, no farmers) they helped (settled 52 → 35 matches). Which world
  we are in is unknown until there is match data, so win/loss only until then.
- **Underdog cushion:** halving the underdog's loss inflated the average rating by about
  +3 points over 200 rounds and slightly worsened the odds the matchmaker uses. All these
  systems already give underdogs smaller losses. If the cushion is wanted, put it on the
  visible rank (#12), not the hidden rating.

## Verdict

**Picked 2026-10-19: Weng-Lin Thurstone-Mosteller with a learned party bonus.** The human's
words: "Weng-Lin Thurstone-Mosteller with learned party bonus, record it". In the sim that is
the `Weng-Lin (Thurstone-Mosteller)` system with the "Party bonus learned from results" rule
(the Parties walkthrough with the base system set to Thurstone-Mosteller).

Recorded on main in ADR 0010 and the glossary.
