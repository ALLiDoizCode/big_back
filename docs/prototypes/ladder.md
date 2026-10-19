# PROTOTYPE sheet: ladder presentation, placements, decay and seasons (issue #12)

Throwaway. Lives on `prototype/ladder` only.

Open `docs/prototypes/ladder.html` in a browser (double-click; no install, no server).
Hosted copy: https://claude.ai/artifact/SJ5hs1hVHfUkGZcjJxaqM5 (private to the owner).

**Question:** how is rating shown to players? Visible tiers and divisions or a number,
placement matches, rank decay, season length, soft-reset rules, and what reaching each
tier rewards. Resolved when the human picks a ladder structure.

## The simulation

- Under every ladder runs the rating from ADR 0010 (Weng-Lin Thurstone-Mosteller), per
  player, from win or loss only. Matches are solo 5v5 (ADR 0006), so the party bonus is 0.
- You are one player among 400. The matchmaker puts nine players near your hidden rating,
  a little loosely (as at about 200 CCU). Background players play 0.3, 1.5 or 5 matches a
  day and sometimes take breaks of 1–7 weeks.
- Tiers are cut by real skill: Bronze 15%, Silver 25%, Gold 27%, Platinum 18%, Diamond 10%,
  Master 4%, Champion 1%. Bronze to Diamond have divisions III, II, I. Master is open-ended;
  Champion is a leaderboard on top of it.
- Hidden uncertainty grows on idle days and a little at each season start.
- Rewards are ADR 0009's dated season cosmetics, one per tier.

The logic is one pure module (`Ladder` in the first `<script>`); the page is a shell.

## The four ladders (presets; every rule is also tunable)

| | Shown as | Placements | Decay | Season start | Season | Rewards |
|---|---|---|---|---|---|---|
| **Divisions and points** (LoL, Valorant) | division + points, convergence | 5, placed low, cap Diamond III | Diamond+, banked days | drop one tier | 3 months | 10 wins at rank |
| **Shown rating** (SC2 3.4, Splatoon) | the rating as a number + tier | none (provisional ±) | none (uncertainty grows) | uncertainty raised | 3 months | peak |
| **Progress bar, yearly placements** (OW2 S9) | % through division, reasons listed, underdog cushion | 10, cap Master, once a year | none | carries over | 9 weeks | 10 wins at rank |
| **Stars with floors** (Brawl Stars) | 5 stars a division, tier floors, streak and boost stars | none | none | drop two tiers | 7 weeks | season-end rank |

Walkthroughs: new account, losing streak, a month away, season rollover, great new player,
the grinder (10 a day), weekend player.

## Findings (seed 7; medians over 15 seeds where stated)

Matches until a new account is shown within 1.5 divisions of its real skill, and stays
there for 10 matches (median of 15 seeds, 3 matches a day):

| | bottom 10% | average | top 22% | top 2% |
|---|---|---|---|---|
| Divisions and points | 8 | 25 | 44 | 66 |
| Shown rating | 20 | 37 | 43 | 34 |
| Progress bar (10 placements, cap Master) | 24 | 26 | 38 | 32 |
| Stars with floors | 18 | 76 | 96 | 351 |

- **Placing low costs strong players.** Placing one uncertainty low with a Diamond III cap
  took a top-2% player about 66 matches to be shown right, against 32 when placed at the
  hidden rating with a Master cap.
  Those are lobbies where they sit visibly below the players they beat.
- **Stars do not follow skill.** An average player grinding 10 a day for a 7-week season
  went 254–236 and finished **Master**; on divisions and points the same player went
  456–454 and finished Silver I. Across the population, stars inflate about +1.4 divisions
  by season end and are then knocked down by the two-tier reset. A top-2% new account
  needed about 350 matches to be shown where it belongs.
- **Shown rating is honest but noisy.** It never inflates and needs no placements, but it
  moves every match by 15–25 points and swaps divisions often, and a new account's number
  starts at 1500 with a ±400 range.
- **Points with convergence track the hidden rating** (population average within 0.1 of a
  division all season). The cost of the one-tier reset: a Platinum player needed about 18
  matches to get back.
- **Decay** (Diamond+, banked days) only ever took a player who stopped playing down to
  Diamond III; a weekend player (8 matches a week) kept their bank.
- **Weekend player** (top 22%, 8 matches a week): earned Bronze–Platinum rewards on every
  ladder with "10 wins at rank"; on "peak" also Diamond, from a brief spike.
- **Underdog cushion** on the visible rank (from ADR 0010) halves losses in the 40%-or-worse
  matches. On the progress-bar ladder, which has it on, the population's shown rank sits
  about +0.2 divisions above its hidden rating, a small cost for the softer feel.

## Verdict

**Picked 2026-10-19: Divisions and points.** The human's words: "Divisions and points, record
it". In the sim that is the `Divisions and points` preset with its rules as they stand:
points with convergence, 5 placements for new accounts (placed one uncertainty low, capped at
Diamond III), no underdog cushion, Diamond+ decay from banked days, a one-tier drop at season
start, 3-month seasons, rank updated after every match, ranked-win rewards for 10 wins at a
rank or above, and Champion as Master above a rating line.

Recorded on main in ADR 0011 and the glossary.
