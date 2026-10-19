# Players join a match from the pad

Every team starts a match with one player. More players join only when the team kicks open
a player recipe on its pad. We chose this in play ([#10](https://github.com/ALLiDoizCode/big_back/blob/prototype/squad-model/docs/prototypes/squad-model.md#verdict-on-how-players-join)).
The human's verdict: "this feels good". It keeps Team Buddies' "build your team from
crates" loop, with players in place of AI Buddies (see [0004](0004-one-character-per-player-no-ai-units.md)).

- The pad is 2 × 2 cells, at most 2 crates high, as in Team Buddies.
- Nothing opens by itself: a player kicks the pad open, and the shape decides the result.
- **2 stacked** brings in 1 player; **2 wide × 2 high** brings in 2.
- A joined player who is knocked out stays out until the team opens another player recipe.
  Only the starting player respawns.

This rejected two other options: full teams from the start, and a 2-high stack that opens
by itself. Both made players appear without the team earning them.

## Consequences

- **Team size is a cap, not a starting count.** It's 3 for now, in a 36 × 22 m arena
  (superseded by [0006](0006-five-v-five-on-a-lanes-and-jungle-map.md): 5v5 on a 90 × 50 m map). That's
  easy to change: the prototype grows the arena to keep about 130 m² of floor per player.
- **A match's head count depends on the crate economy.** Stealing crates off a pad breaks a
  team's next recipe.
- **What other shapes build is #17.**
- **Open:** where a team's other players are before they join, whether waiting in the match
  or matched in mid-game. Matchmaking has to decide this.
