# Divisions and points over a hidden rating

Players see a **rank** made of tiers, divisions and points. The **rating**
([0010](0010-weng-lin-thurstone-mosteller-with-a-learned-party-bonus.md)) stays hidden and
runs matchmaking. Points pull the rank toward the rating over time (League of Legends and
Valorant work this way). We chose this after playing the
[ladder sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/ladder/docs/prototypes/ladder.md)
for [#12](https://github.com/ALLiDoizCode/big_back/issues/12), which built on the
[rating research](https://github.com/ALLiDoizCode/big_back/blob/research/rating-ladder/docs/research/rating-ladder.md).
The human's verdict: "Divisions and points, record it". The rules below are that preset as it
stood in the sheet.

## The rules

- **Tiers:** Bronze, Silver, Gold, Platinum, Diamond, Master and Champion. They are cut by
  rating so that, once players have settled, about 15%, 25%, 27%, 18%, 10%, 4% and 1% of
  players are in each.
- **Divisions and points:** Bronze to Diamond each have divisions III, II and I, 100 points
  apiece. Points carry over on promotion and demotion. Master has no divisions; its points
  just keep counting.
- **Convergence:** a win pays about 20 points and a loss costs about 20. While the rank is
  below the rating, wins pay more and losses cost less, and the reverse while it is above,
  within 8 to 40 points. This is the only link from rating to rank. Only the result counts.
  There is **no underdog cushion on the rank** either, which settles the question ADR 0010
  passed to #12.
- **Updates:** the rank changes after every match. It is never batched into sets.
- **Placements:** a new account plays 5 placement matches with no rank shown, only a
  predicted rank. Its rank is then set one uncertainty below its rating, capped at
  Diamond III. Each placement loss costs nothing. Placements happen once per account,
  not every season.
- **Decay, Diamond and up only:** each match banks 7 days, up to 28, and each day uses one.
  Each day spent with an empty bank costs 50 points, but decay never drops a player
  below Diamond III. The rating itself doesn't decay; its uncertainty just grows a little
  on idle days.
- **Champion:** a leaderboard at the top. It holds the Master players whose rating is above
  the Champion line and who have played at least 10 matches in the last 21 days. A player
  who stops playing drops back to Master.
- **Seasons:** 3 months each. At the start of a season every rank drops one tier
  (3 divisions), but never below Bronze III. The rating is kept, with its uncertainty raised
  slightly.
- **Ranked-win rewards** ([0009](0009-progression-buys-identity-only.md)): a dated season
  cosmetic for each tier. A player earns a tier's cosmetic with 10 wins at that tier or
  above in the season, so a win in Gold also counts toward Bronze and Silver.

## Considered options

- **The rating shown as a number** (StarCraft II since 3.4, Splatoon X Power): it never
  inflates and needs no placements, but it moves every match and swaps divisions often, and
  a new account starts at 1500 ±400.
- **Progress bar, 10 yearly placements, underdog cushion** (Overwatch 2 since Season 9): it
  placed strong new players fastest (about 32 matches for a top-2% player). The cushion kept
  shown ranks about 0.2 divisions above ratings across the population.
- **Stars with floors** (Brawl Stars): an average player grinding 10 matches a day finished
  a season at Master, and a top-2% new account needed about 350 matches to be shown in
  the right place. Rank paid for time spent, not for skill.

## Consequences

- **Strong new players start low.** Placing one uncertainty low under a Diamond III cap took a
  top-2% new account about 66 matches to be shown within 1.5 divisions of its real skill,
  against 25 for an average one. Those lobbies are fair, since matchmaking uses the
  rating, but the badge reads low. The placement offset and the cap are tuning knobs, and
  they are the first to revisit if this feels bad.
- **Each season costs a settled player a climb.** In the sim a Platinum player needed about 18
  matches to win back the tier dropped at season start.
- **Decay can only bring a player who stops playing down to Diamond III.** A weekend player
  (8 matches a week) keeps a full bank.
- **The tier cuts need re-fitting once there are real ratings.** They are set from the
  real-skill distribution in the sim.
- **Open:** whether a quit counts as a loss, and win-trading detection (both still open from
  [0010](0010-weng-lin-thurstone-mosteller-with-a-learned-party-bonus.md)); timed ranked
  events (handed here by 0009, not prototyped); per-region ladders.
- **Settled by [0017](0017-leavers-take-a-loss-and-their-seats-reopen.md):** a quit counts as a loss for the leaver, at the most points; the
  leaver's teammates lose none.
- **Settled by [0018](0018-a-party-cap-and-flags-held-for-review.md):** win-trading detection, and a "held points" state on the rank. A
  wide 5-stack plays for fewer points, and its wins don't count toward ranked-win rewards.
- **Settled by [0019](0019-a-tutorial-then-the-rated-queue.md):** only rated matches count as
  placements, and a leave during placements still uses one up.
