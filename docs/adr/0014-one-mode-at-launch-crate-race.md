# One mode at launch: Crate Race

Season 1 ships **one mode, Crate Race**: the score race with base knockouts and the comeback
set from [0013](0013-base-knockouts-with-a-comeback-set.md). Variety at launch comes from the
map, not from more modes. Other modes go on a **post-launch list**: the spec names them and
specifies none. We chose this in [#33](https://github.com/ALLiDoizCode/big_back/issues/33)
(grilling). The human agreed with the recommendation.

## Why one mode

- **Every core-loop decision needs a played prototype.** Crate Race is the only mode that
  has one. The comeback set was tuned for it and only works as a whole (0013).
- **One bot AI.** Bots fill seats nobody takes ([0012](0012-one-rated-queue-with-short-starts.md)).
  Each extra mode would need its own bot that hauls, steals, raids and defends well enough.
- **One rating that means one thing.** Every match is rated (0012). A second mode would put
  results from different games into the same hidden rating
  ([0010](0010-weng-lin-thurstone-mosteller-with-a-learned-party-bonus.md)).
- **The research says consolidate.** Splitgate cut ranked to one mode and Splatoon rotates one
  ranked ruleset. Brawl Stars removed Siege, the closest analogue to a crate economy feeding
  production ([win-conditions research](https://github.com/ALLiDoizCode/big_back/blob/research/arena-win-conditions/docs/research/arena-win-conditions.md)).
  A second mode wouldn't split our queue, because 0012 already pools modes, but it would split
  the work.

## Post-launch list (named, not specified)

- **Bomb-Ball:** goal plus kickoff reset, from Team Buddies.
- **Crate Grab:** a hold-to-win countdown. Reach a crate threshold on the pad and hold it
  while a countdown runs. The countdown resets if the pad drops below.
- **Timed Crate Race variants:** the same rules, tuned differently, on a schedule (for
  example a megacrate rush).

Each needs its own played prototype before it ships. When a second mode arrives, 0012's rule
applies: the server rotates modes, or each player picks at least 2. There is never a queue per
mode, and one rating covers every mode.

## Considered options

- **One mode plus timed rule variants at launch:** cheap, but every variant is rated and
  would need retuning of the comeback set. Moved to the post-launch list.
- **Two or three distinct modes in rotation:** more variety, but each needs a prototype
  verdict, a bot AI and rating checks before the spec is ready.
- **Left off the list:** permanent base destruction (rejected in 0013), round resets (dragged
  in play, 0013), tug-of-war (doesn't fit a two-pad lanes map), multi-team formats (ruled out
  in 0012).

## Consequences

- **"Casual" names nothing any more.** There is no casual queue (0012) and no casual mode.
  0013's open point "whether the +3 knockout bonus and the mercy rule stay in casual play" is
  closed: they apply in every match. The "casual" in [0009](0009-progression-buys-identity-only.md)
  means matchmade play, and its bot-filled casual games don't exist; bots only take open seats.
- **0012's mode selection is unused at launch.** "Rotate or pick at least 2" waits for a
  second mode. The queue rotates maps only.
- **Map variety carries the launch.** How many maps and variants ship is still open.
