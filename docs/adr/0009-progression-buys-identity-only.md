# Progression buys identity only

**Anything that changes a match is free for everyone. Progression buys only identity**:
cosmetics, titles and proof of skill. All three classes and their full loadouts
([0008](0008-class-roster-and-loadouts.md)) are open from the first match, with no new-player
gate. Nothing persists between matches as power: no stats, perks or sidegrades. Account
level, mastery and cosmetics never feed matchmaking or rating. Any power a player has comes
from crates inside the match and resets at its end. We chose this in the grilling of
[#13](https://github.com/ALLiDoizCode/big_back/issues/13), from the
[fair-progression research](https://github.com/ALLiDoizCode/big_back/blob/research/fair-progression/docs/research/fair-progression.md).
At about 200 CCU the queue can't be split by power the way Brawl Stars does it, so any
lasting gap between players goes straight into who wins. OW2 took three steps to remove
its hero locks; we start where it ended.

## Reward tracks in Season 1

- **Ranked-win rewards:** dated, seasonal cosmetics for wins at the rank reached. The tiers,
  win counts, win history, batched rank updates and timed ranked events belong to
  [#12](https://github.com/ALLiDoizCode/big_back/issues/12).
- **Class mastery:** after each match, each player gets a **grade** from D to S for each class
  they played, measured against the whole lobby on combat (damage, knockouts), crate work
  (crates delivered to their pad, steals from enemy pads, recipes kicked open) and the
  class's own job (soldier: damage to vehicles and walls; engineer: damage blocked by walls,
  gates held; medic: healing and shields given). Points scale with the grade. A win adds a
  small bonus, but a losing player can still earn an S. A match's points are split by the time
  spent alive as each class. Each season, each class has a four-milestone track gated on
  higher grades, ending in that season's class title, then a repeating bonus milestone.
  Points reset each season; titles stay.
- **The pass:** one free, linear track that claims itself. XP comes from time in matchmade
  play, plus a small win bonus; losses still move it forward. Weekly challenges add XP. It
  pays cosmetics only, with no currency. Past seasons stay earnable. Whether a premium track
  ever exists is monetization, and out of scope.
- **Joke awards:** Team Buddies-style, public on the results screen. Every player gets
  exactly one, framed as a joke and never as shaming; leavers and idle players get none.
  Awards pay nothing directly and only feed challenges.

## Where progress counts

- **Only matchmade games**, casual or ranked. Custom lobbies, practice and the tutorial earn
  nothing.
- **Bots:** in unrated matches with a bot seat ([0012](0012-one-rated-queue-with-short-starts.md)), a player still earns XP for the match, but
  nothing done to a bot counts toward a stat (no knockouts, steals or crates taken from bots).
- **Challenges** may count outcomes (matches, wins, grades), awards, contested acts (a steal
  from an enemy pad that the thief's team then kicks open, kicking open a capstone, knocking
  out a carrier) and class milestones. Each stat counts only up to a small cap per match. No
  challenge rewards losing, dying, friendly fire or idling. A steal counts only once its
  crate is kicked open into a recipe, not when it's carried away. Win-trading detection
  belongs to the rating ticket, [#11](https://github.com/ALLiDoizCode/big_back/issues/11).
  Settled by [0018](0018-a-party-cap-and-flags-held-for-review.md): flags, held for review.

## Readability rule

Cosmetics may never change:

- a body's silhouette or hitbox (one shared body per class),
- the class read on the body,
- team colour,
- crates, pads or ghosts (crates get no cosmetics at all),
- how a carried crate or a stack reads,
- crafted items' silhouettes, projectile colour and shape, or the purple super trim,
- capstone sounds and minimap icons.

What's left for cosmetics: surface materials and trims, emotes, titles, banners and the
end-of-match pose. A free starter set ships so a new player isn't the plain one in the lobby.
Identity (titles, banners, mastery) shows on the loading and results screens only. The
in-match HUD shows names alone, so mastery doesn't tell the enemy who to focus.

## Considered options

- **Persistent power (Brawl Stars power levels)**: it needs floors, caps and separate queues,
  which only a mobile-scale population can afford.
- **Sidegrade or class unlocks through play (TF2, Splitgate's Rookie Levels)**: fair as
  design, unfair as gating, because at low CCU a new player meets veterans who have tools
  they lack.
- **Private medals (Splatoon)**: rejected for public joke awards that celebrate failure as
  well as success, which suits an accessible game where many players lose.
- **Mastery from time played**: it reads as grind; graded mastery reads as "I got better".

## Consequences

- **Nothing to unlock that matters**, so new players need no gate beyond whatever ranked
  onboarding decides.
- **Settled by [0019](0019-a-tutorial-then-the-rated-queue.md):** ranked onboarding. The only gate
  is a required tutorial, once per account.
- **Mastery needs per-class performance stats** recorded on the server for every matchmade
  match, including time alive as each class.
- **Art has a hard list** of things a skin can't touch, from day one, so it never has to be
  retrofitted the way Rocket League did with hitboxes.
- **Settled by [0023](0023-no-scheduled-ranked-events-in-season-1.md):** timed ranked events.
  Season 1 has none.
