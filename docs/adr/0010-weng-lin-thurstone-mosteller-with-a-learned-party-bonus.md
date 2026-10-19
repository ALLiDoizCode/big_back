# Weng-Lin Thurstone-Mosteller with a learned party bonus

The **rating** behind matchmaking and the ladder is **Weng-Lin's Thurstone-Mosteller model**
(the analytic update from Weng & Lin 2011, with TrueSkill's Gaussian likelihood). A team's
strength is the sum of its players' ratings, **plus a party bonus for each extra party
member, learned from match results**. We chose it after playing the
[rating-system sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/rating-system/docs/prototypes/rating-system.md)
for [#11](https://github.com/ALLiDoizCode/big_back/issues/11), which built on the
[rating research](https://github.com/ALLiDoizCode/big_back/blob/research/rating-ladder/docs/research/rating-ladder.md).
The human's verdict: "Weng-Lin Thurstone-Mosteller with learned party bonus, record it".

In the simulation (200 players, two teams), it settled players about as fast as TrueSkill, and
in about two thirds of the matches the Bradley-Terry version of Weng-Lin needed:

| Median matches to settle | 2v2 | 3v3 | 4v4 | 5v5 |
|---|---|---|---|---|
| Weng-Lin Thurstone-Mosteller | 25 | 38 | 45 | 50 |
| TrueSkill | 23 | 27 | 34 | 45 |
| Weng-Lin Bradley-Terry | 39 | 57 | 70 | 75 |

The simulation decides matches with TrueSkill's own Gaussian model, which slightly favours
both Gaussian systems.

## The rules

- **Per player:** a mean and an uncertainty. New accounts start at the average (25) with
  full uncertainty (25/3), not one tier low. Starting low doubled a smurf's climb in the sim.
- **From the result only.** Ratings move on win or loss. Kills, crates and other match stats
  don't count until match data shows they predict wins (TrueSkill 2's method). With weak
  stats and stat farmers, counting them made the ladder worse in the sim.
- **Party bonus:** one shared value per extra party member, starting at 0. It is updated
  after each match from how far the result missed the prediction, and is used both to
  predict matches (and so to balance teams) and in the rating update. In the sim it found the
  real bonus without being told it, and gave the most accurate ladder of the four party rules.
- **No underdog cushion on the rating.** The update already costs an underdog less for a loss.
  Halving underdog losses on top of that inflated the population's rating in the sim. If a
  cushion is wanted for how losses feel, it goes on the visible rank
  ([#12](https://github.com/ALLiDoizCode/big_back/issues/12)).

## Considered options

- **Weng-Lin Bradley-Terry through `skillratings`** (the research's pick): it settles players
  about 1.5–2× slower in the sim, at every team size.
- **TrueSkill:** as fast, but Microsoft holds a patent and the trademark.
- **Glicko-2:** fast at 2v2, but it sees only team averages and slows down most as teams
  grow. It is also designed for batched rating periods.
- **Elo:** it has no uncertainty term and needed about 90–110 matches at every size.
- **Other party rules:** treating a team as just the sum of its players lets party members
  drift above their real skill. "A party counts as its best player" (Rocket League) makes
  parties earn less per win. A fixed, hand-set bonus is only as good as the guess.

## Consequences

- **We write the update ourselves.** It is about 30 lines (the sheet's `RatingSim` has a
  version). As far as we know, the `skillratings` crate's Weng-Lin is Bradley-Terry only.
  Check that before writing our own.
- **At 5v5 ([0006](0006-five-v-five-on-a-lanes-and-jungle-map.md)) a fresh ladder needs
  about 50 matches per player** before the median player is in the right band. Placements,
  a strong prior and how rank tracks the rating belong to #12.
- **Parties that only ever play together can't be told apart.** Nothing separates their skill
  from their teamwork. The bonus is learned from parties whose players also play apart.
- **Open:** tuning β (luck) and τ (drift) from real data, whether a quit counts as a loss,
  per-size party bonuses, and win-trading detection (handed to #11 by
  [0009](0009-progression-buys-identity-only.md)).
- **Settled by [0024](0024-leavers-take-a-loss-and-their-seats-reopen.md):** a quit counts as a loss for the leaver, and every player is rated by
  their share of the match played (partial play).
- **Settled by [0018](0018-a-party-cap-and-flags-held-for-review.md):** win-trading detection. Flags are raised automatically, a person
  reviews them, and flagged matches' points are held until then.
