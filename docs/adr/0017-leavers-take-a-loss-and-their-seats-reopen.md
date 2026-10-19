# Leavers take a loss, and their seats reopen

A player who drops can **rejoin until the buzzer**. A player who quits, or stays away too long,
is a **leaver**: they take a loss and a **lockout**, their teammates lose no points, and their
seat reopens to the queue like a short-start seat
([0012](0012-one-rated-queue-with-short-starts.md)). A leaver never makes a match unrated. The
rating counts every player by the share of the match they played (**partial play**). We
chose this in [#37](https://github.com/ALLiDoizCode/big_back/issues/37) (grilling). The human
agreed with each recommendation. The numbers are tuning, not decisions.

## The rules

### Being away

- **Away:** a player is away while disconnected, while idle with no input for **45 s**
  (waiting at the pad after a knockout doesn't count), or until they have loaded in, if
  that takes more than **60 s**. There is no "accept match" step: queueing is the consent,
  including to being seated mid-match.
- **An away character leaves the map at once.** It drops any crate it carries, as if hit. An
  away player's seat counts as empty.
- **Rejoin:** an away player can come back at any time until the buzzer, unless their seat
  has been taken (below). If their character was on the map when they went away, they come
  straight back in at the pad, since the team already paid for them. If it was knocked out,
  they wait for a player recipe like anyone else.
- **The starting player's role passes on.** When the starting player goes away, the role moves
  at once to the teammate who has been in the match longest, so the team still has someone
  who respawns. It doesn't move back when they rejoin.

### Leavers

- **Who is a leaver:** decided at the buzzer. A player who quit on purpose (quitting is final,
  with no rejoin), or who was away for more than **90 s** in total, is a leaver, even if they
  came back.
- **The seat:** after 90 s away, the seat becomes an **open seat** while seats are still open
  (until about 3:12, [0015](0015-an-eight-minute-timer.md)). If a queued player takes it, the
  away player can't come back. After seats close, a bot takes it, shown as a bot and entering
  through the pad. For the rating, the bot counts as an average player (25) for its time on
  the map.
- **A leaver never unrates a match.** 0012's "a seat nobody takes goes to a bot, and the match
  is unrated" covers only short-start seats nobody ever filled. Otherwise a losing team could
  cancel a match by quitting.
- **A team with nobody left loses.** If no player on a team is on the map, or able to come
  back, for 90 s, the match ends. That team loses, and the other team gets a full win.

### Rating and rank

- **The leaver takes a loss** on both the rating and the rank (the most points, 40), whatever
  their team's result. This is the one exception to "from the result only"
  ([0010](0010-weng-lin-thurstone-mosteller-with-a-learned-party-bonus.md)). TrueSkill 2
  found that quitting predicts losing.
- **Teammates lose no points.** If a teammate left, the rest of the team loses no rank points
  for that match. A win pays as normal. Their rating still updates, so matchmaking still
  learns from the match. The opponents' result counts in full.
- **Partial play.** Team strength, in both the prediction and the update, weighs each player
  by the share of the match they were on the map. This covers late joiners, leavers, bots in
  leavers' seats and players waiting at the pad. A late joiner who played the last 40% is
  updated with weight 0.4.
- **A mercy end** ([0013](0013-base-knockouts-with-a-comeback-set.md)) is a normal win and a
  normal loss.
- **A server fault** that ends a match voids it: unrated for everyone, with no leaver
  penalties. A player's own connection problems go through the away rules instead.

### Penalties

- **Lockout:** each leave adds a step to a queue lockout of **5 min, 30 min, 2 h, 24 h**,
  on top of the loss. Every **10** matches finished without leaving takes the player down
  one step. There's no warning-only first offence, and no permanent ban for leaving alone.
- **Per player, not per party.** Only the leaver is penalized; the rest of the party gets the
  teammates' protection. A party can't queue until every member's lockout is over.
- **In placements** ([0011](0011-divisions-and-points-over-a-hidden-rating.md)), where a loss
  costs nothing, a leave is still a loss on the rating, adds a lockout step and uses up one
  of the 5 placement matches.
- **In an unrated match,** a leave still adds a lockout step.

### Player recipes

- **A player recipe brings in whoever has waited longest.** Knocked-out joined players and
  late joiners wait in one line. A late joiner's wait counts from when they took the seat.
  This settles the question [0015](0015-an-eight-minute-timer.md) handed here, and it is what
  the pacing prototype did.

## Considered options

- **No rejoin, or a short fixed rejoin window:** a crash or a router hiccup would be the same
  as quitting.
- **Leaver status only for a quit:** idling, or ducking out and back at 89 s, would get round it.
- **The rating gets the team's real result, and only the rank punishes the leaver:** truer to
  "from the result only", but a leaver whose team won would gain rating.
- **Half the points lost for a leaver's teammates** (closer to Valorant): it still makes them
  pay for someone else's leave.
- **An empty seat after seats close, instead of a bot:** the team plays one short for the rest
  of the match. Points protection softens the rank, but not the match.
- **A surrender vote:** with 8-minute matches and a mercy rule it adds little, and it opens
  "surrender, then requeue" as a way to dump losses.
- **A ready check before each match:** it adds a dodge, and needs its own penalty. Mid-match
  seating makes it awkward anyway.
- **Late joiners first in the player-recipe line:** a knocked-out player could wait forever.

## Consequences

- **0010's rating counts everyone fully today; it needs partial play.** Short starts already
  made that wrong (0012). The update has to carry each player's time on the map.
- **The server tracks time on the map and time away** for every player. Both are inputs to the
  rating and to leaver status.
- **Bots now play in rated matches,** in leavers' seats after seats close. Bot quality matters
  there too.
- **A leave hurts the team that stays only in the match itself,** not in rank. A team that
  keeps playing short can still win.
- **Settles** the quit-as-loss question left open by 0010 and 0011, the leaver and rejoin
  policy left open by 0012, the mercy rule's rating question left open by 0013, and who a
  player recipe brings in first, left open by 0015.
- **Open:** the exact thresholds (45 s idle, 60 s load, 90 s away, the lockout steps) once
  humans play; whether the server can tell its own fault from a player's connection reliably.
