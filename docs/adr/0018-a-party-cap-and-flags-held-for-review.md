# A party cap, and flags held for review

The ladder is kept honest in two ways. First, a **party cap** limits how far apart in skill a party
may be before it can queue. Second, **flags** are raised automatically for win-trading,
boosting and throwing, and a person reviews them before anyone is punished. While a flag waits,
the points from the flagged matches are **held**. This settles the win-trading detection that
[0009](0009-progression-buys-identity-only.md) handed to the rating and that
[0010](0010-weng-lin-thurstone-mosteller-with-a-learned-party-bonus.md) and
[0011](0011-divisions-and-points-over-a-hidden-rating.md) left open. It also gives the shape of
the party skill cap that [0012](0012-one-rated-queue-with-short-starts.md) named as the next
lever. We chose this in [#38](https://github.com/ALLiDoizCode/big_back/issues/38) (grilling).
The human agreed with each recommendation. The numbers are tuning, not decisions. The cap's
width is set by a prototype ([#47](https://github.com/ALLiDoizCode/big_back/issues/47)).

## What it covers

- **Win-trading:** two parties queue together so that they meet as opponents, and take turns
  losing. In one global pool at about 200 CCU, two parties that queue together at the same
  quiet hour will very likely meet, so this is easy to do.
- **Boosting:** a strong player carries a weak account up in a party. This includes paid
  boosting. Account sharing is against the terms of service, and detection doesn't try to
  catch it.
- **Throwing:** losing on purpose to drop rank, often to set up a boost.
- **Not smurfing.** Smurfs are left to what is already decided: new accounts start at the
  average with full uncertainty (0010), placements are capped (0011), and the party cap
  applies to them too.

## The party cap

- **Measured in rank, enforced on the wider.** The cap is shown in tiers and divisions, so a
  party can see it and plan around it. It is enforced on whichever spread is wider, rank or
  the hidden rating, because rank lags rating and a smurf's or boosted account's rank reads low.
  An account still in placements counts at its predicted rank.
- **A wider party is blocked from queueing,** with a wider cap allowed for bigger parties.
  The fix is always plain: drop a member.
- **A full 5-stack may be any width.** It can't push a stranger into a lopsided match, so it
  isn't blocked. Instead its rank points are cut on a sliding scale by spread (Valorant does
  the same). A wide 5-stack's wins don't count toward ranked-win rewards (0011), so a stack
  can't carry a teammate to a tier's cosmetic.
- **What the party sees:**
  - At invite: a warning ("too far apart to queue together"). The invite isn't blocked,
    since custom lobbies have no cap.
  - At the queue button: the button is disabled, and the message names the out-of-range
    member and the allowed range in tiers and divisions.
  - A wide 5-stack: the button stays enabled, and "reduced points: −N%" is shown before
    queueing and on the results screen.
  - When only the hidden rating is too wide, the message says "skill too far apart" and
    shows no number.
- **Widths** (set by [#47](https://github.com/ALLiDoizCode/big_back/issues/47) after playing the
  [party-cap sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/party-cap/docs/prototypes/party-cap.md); the human's verdict: "lets go with that, record it"):

  | Party | May be at most |
  |---|---|
  | Duo | 2 tiers apart |
  | Party of 3 | 2 tiers apart |
  | Party of 4 | 3 tiers apart |
  | Full 5-stack | any width; points cut past 3 tiers |

- **The 5-stack cut:** none up to 3 tiers of spread, then −25% for 3–4 tiers, −50% for 4–5 and
  −75% beyond. About 70% of 5-stacks get no cut in the sim.
- **A party over its cap** is blocked as above. The sim modelled the plain fix: its furthest
  member drops out and queues solo, until the rest fit.

## Prevention in the matchmaker

- **A soft cost on repeat opponents.** Putting the same two parties on opposite sides again,
  within either's last 5 matches, counts against a candidate match in the matchmaker's score.
  It is never forbidden, so at the quietest hours they can still meet rather than wait.

## Flags

Signals raise flags on their own. A flag only matters once a reviewer looks at it.

- **Win-trading:** a pair of accounts who meet as opponents far more often than chance would
  give for their rating and region, and whose results alternate or keep going one way. The
  flag is stronger when the losing side contributed almost nothing while on the map (no
  damage, no crates delivered, no kicks).
- **Boosting:**
  - an account whose win rate with one partner is far above its solo win rate over many
    matches,
  - a rating that climbs faster than the account's solo results justify,
  - a wide 5-stack whose lineup repeats.
- **Throwing:**
  - repeated losses in which the player was on the map but contributed nearly nothing,
  - a steep drop in rank right before partying with a much higher player.

  Players who are away are already handled by [0024](0024-leavers-take-a-loss-and-their-seats-reopen.md).
  These signals are for players who are present and playing to lose.
- **Player reports:** each player can make one "suspected throwing or match fixing" report per
  match, from the results screen. A report only adds weight to a flag the signals have already
  raised. Reports alone never hold points, so a lobby can't use them to punish a player.

## Holds and review

- **People act on flags at launch.** At this population a false positive bans a noticeable
  share of the player base, and there's no history yet to tune thresholds against. Automatic
  penalties come later, once real data shows the flags are reliable.
- **The one automatic action is a hold.** The rank points from flagged matches are held, not
  removed. The rank shows them as held, without a reason: points that vanished would look like a
  bug, and naming the signal would teach abusers which signals to avoid.
- **A hold lifts after 7 days** if no one has reviewed it. A held account that is flagged
  again within those 7 days keeps all its holds until someone reviews it.

## Penalties

Once a reviewer confirms abuse:

1. **First offence:** the points and ranked-win reward progress from the flagged matches are
   voided for everyone who benefited. The boosted account's uncertainty is raised, so it settles
   again. The rating updates from those matches stand. The opponents who lost get their points
   back.
2. **Second offence:** the season's rank and ranked-win rewards are removed.
3. **Third offence:** the account is banned from ranked play.

Offences never step back down. Unlike a leaver's lockout (0024), finishing matches doesn't
wipe out confirmed abuse.

## Considered options

- **Cap widths of 1 tier:** close matches 93%, but about two parties in three blocked.
- **Cap widths of 3 tiers:** 9% blocked, but close matches only went from 78% to 83%.

- **No cap, trust the party bonus** (the rating research's pick): 0012's sim showed close
  matches falling from 99% to 68% once a quarter of players were in parties.
- **Allow every width, cut points by spread** (Valorant 5-stacks): a wide duo still makes three
  strangers' match lopsided. It's kept only for the 5-stack, which plays only with itself.
- **Rate a party as its best player** (Rocket League): it conflicts with 0010's learned party
  bonus, and parties earn less per win.
- **Cap measured on the hidden rating only:** it can't be explained to a party.
- **Cap measured on rank only:** a smurf's or boosted account's low rank lets it through.
- **Automatic penalties from day one:** no data to set thresholds yet, and false positives cost
  a lot at this population.
- **Never meeting the same party twice in a row (a hard rule):** at the quietest hours it means
  long waits.

## Consequences

- **The server records per-match contribution for every player:** damage, crates delivered and
  kicks. Mastery already needs much of this ([0009](0009-progression-buys-identity-only.md)).
- **The server keeps opponent history and party history per account,** to count repeat
  meetings and party-versus-solo win rates.
- **Someone has to review flags.** Launch needs a review tool and a person on it, even if part
  time.
- **The rank needs a "held points" state,** and the ranked-win reward count needs a way to
  void wins after the fact.
- **The queue simulator has to keep each party member's skill** to set the cap (#47).
- **Settles** the win-trading detection left open by 0009, 0010 and 0011, and the shape of the
  party skill cap left open by 0012.
- **What the cap buys** (the sim of #47: 5v5, full lobbies, 25% of players arriving in parties):
  close matches rise from 78% to 86% at 200 players at the busiest hour (99% with solo players
  only), about the same at 100 and 60. About a quarter of parties are blocked, and most of them
  still play a member short. Waits don't move.
- **Most of the gain comes from fewer parties, not narrower ones.** Under 0010's model a team is
  just the sum of its players plus the party bonus, so width matters only if a strong player
  lifts weaker friends beyond that. The sim assumed a 25% lift; real matches should measure it.
- **The widths need re-fitting from real party data.** The share blocked ran from 11% to 43%
  at these widths, depending on how alike friends' skills are and how far rank lags rating.
- **The cap matters little next to short starts** ([0012](0012-one-rated-queue-with-short-starts.md)): with them, late joiners and bots
  drive fairness far more than party width.
- **Open:** every flag threshold, once there are real matches to measure them against.
- **Settled by [#47](https://github.com/ALLiDoizCode/big_back/issues/47):** the cap widths and the 5-stack cut.
