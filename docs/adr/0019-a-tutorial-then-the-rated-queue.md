# A tutorial, then the rated queue

A new player's only gate is a **required tutorial** of about 5 minutes. After it they go
straight into the one rated queue ([0012](0012-one-rated-queue-with-short-starts.md)), and
their 5 placements ([0011](0011-divisions-and-points-over-a-hidden-rating.md)) are the rest of
the onboarding. **Practice** against bots is open at any time but never required. This settles
the ranked onboarding that [0009](0009-progression-buys-identity-only.md) left open ("new players
need no gate beyond whatever ranked onboarding decides"). We chose this in
[#39](https://github.com/ALLiDoizCode/big_back/issues/39) (grilling). The human agreed with each
recommendation. The tutorial's length and steps are tuning, not decisions.

## Why no bigger gate

- **Every gate keeps a player out of a small pool.** At launch population, a player held back
  by bot matches or an account level is a player the queue doesn't have.
- **A gate costs a smurf only time, and a real new player the same time.** What actually tells
  them apart is already decided. A new account starts at the average rating with full
  uncertainty (0010), so a smurf who wins climbs away fast, and the new players they beat lose
  little. Each placement loss costs no points (0011), and the starting rank is capped.
- **No extra smurf friction**, such as phone verification or a paid account. It hurts
  accessibility and buys little against someone determined. The tutorial, once per account,
  is the only added cost.

## The tutorial

- **Required once per account.** It can't be skipped, but it can be replayed from the menu. A
  returning player on the same account never sees it again unless they choose to.
- **Passed by finishing each step.** There's no score threshold. A player who gets stuck gets
  hints, not a fail.
- **Played as soldier**, about 5 minutes:
  1. The crate grammar: grab, carry, jump-throw, toss, stack on the pad, kick open.
  2. One full Crate Race loop ([0014](0014-one-mode-at-launch-crate-race.md)): stack a
     recipe, kick open an item, steal from an enemy pad, score, and see a base knockout.
  3. A closing screen showing all three classes ([0008](0008-class-roster-and-loadouts.md)).
     Class depth is learned in practice and in matches, since a player picks a class every
     time they come in.
- **Earns nothing** (0009).

## Practice

- **A custom lobby**, open any time, including before the first rated match. Any party, and
  any mix of humans and bots on either side.
- **Unrated, and it earns nothing** (0009). It never feeds matchmaking.
- **No party cap.** The cap ([0018](0018-a-party-cap-and-flags-held-for-review.md)) only
  guards the queue, so a veteran can show a new friend the game here.

## One bot

- **The same bot everywhere:** 0012's seat bot, the tutorial and practice. The tutorial runs
  it scripted. Practice runs it unscripted, at easy or normal, which change only aim and
  reaction time.

## Placements for a new player

- **One explainer screen before the first queue:** it says that placements set a starting rank
  and that losses in them cost no points.
- **A counter during placements,** "placement N of 5 · losses cost nothing", in the lobby and
  on the results screen. Only the player sees it.
- **Only rated matches count as placements.** A match made unrated by a bot seat (0012) doesn't
  count. At 60 players at the busiest hour, about 28% of matches are unrated, so placements
  take about 7 matches there instead of 5.
- **Leaving a placement still counts** under
  [0017](0017-leavers-take-a-loss-and-their-seats-reopen.md): the rating takes the loss and the
  match uses up a placement. Otherwise leaving would be a free way to dodge a loss.

## Nothing marks a player as new

- **Others in the match see nothing** that marks a player as new. A marker would invite
  targeting, smurfs would wear it too, and teammates could do nothing with it.
- **The party cap applies unchanged** (0018). An account in placements counts at its
  predicted rank, so a fresh account near the average can't queue with a far higher friend,
  except as a wide 5-stack for fewer points, or in practice. A newcomer exception would be the
  hole boosting uses.

## Considered options

- **No gate at all:** a new player's first match would be a rated 5v5 against strangers with no
  idea what a crate does.
- **Tutorial plus a set number of bot matches:** it costs a smurf and a new player the same time,
  and keeps both out of the pool.
- **A level or hours gate** (League of Legends' level 30): the same cost, much longer. And
  progression is identity only (0009), so there is no level to gate on.
- **A tutorial for every class:** three classes are all open, and each is picked on every
  entry, so a player tries them all within a few matches.
- **A visible "new player" badge:** see above.

## Consequences

- **The bot has to run scripted** for the tutorial, as well as unscripted. 0012 already needs it
  to haul, stack and steal well enough not to break a match.
- **The account stores tutorial completion and a count of rated placements.**
- **Custom lobbies need bots and parties** of any shape.
- **Settles** the ranked onboarding left open by 0009.
- **Open:** the tutorial's exact steps and length, and the practice difficulty settings, once
  they can be played.
