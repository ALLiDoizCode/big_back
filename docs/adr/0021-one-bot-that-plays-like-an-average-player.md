# One bot that plays like an average player

The one **bot** ([0019](0019-a-tutorial-then-the-rated-queue.md)) plays **like an average
player, through a player's inputs and a player's view**. It fills a short-start seat nobody took
([0012](0012-one-rated-queue-with-short-starts.md)) and a leaver's seat after seats close
([0017](0017-leavers-take-a-loss-and-their-seats-reopen.md)). It also runs scripted in the
tutorial and at easy or normal in practice. It does every job a player does, picks a class and
a role by what its team lacks, never takes anything a human could have, and is always shown as a
bot. This fills in 0012's "well enough not to break the match". We chose this in
[#53](https://github.com/ALLiDoizCode/big_back/issues/53) (grilling). The human agreed with each
recommendation. The numbers are tuning, not decisions.

## Strength

- **About average.** 0017 rates a bot as an average player (25) for its time on the map, so the
  bot is built to play like one. That keeps the 25 honest. If it falls short, its rated value
  moves; the bot isn't made stronger to match.
- **Rated seats always run at normal.** One fixed strength everywhere, with no hard setting at
  launch and no scaling to the lobby. A bot scaled to its lobby would make the 25 wrong
  everywhere but the middle.
- **Practice at normal is the rated bot.** Easy changes only aim and reaction time (0019).

## Playing by a player's rules

- **Only a player's inputs.** The bot sends the same move, aim, fire, grab, throw and kick
  commands a client does, through the same server rules. Its grabs are judged like anyone's
  ([0017](0017-server-grabs-judged-on-what-you-saw.md)).
- **Only a player's view.** It knows what a player's camera and minimap show, nothing more.
- **A human reaction floor** at normal, and aim with a human's error, not a perfect lead.
- The prototype bots break all four: their grabs skip the server, they know every crate on the
  map, they lead shots perfectly (±4° spread) and they decide every tick.

## Jobs

All of these are required at launch, at easy as well as normal:

- **Haul** crates.
- **Stack** on the pad, including jump-throws.
- **Kick open.**
- **Steal** off the enemy pad. The prototype bots make none on the big map.
- **Defend** its base and pad.
- **Raid** the enemy base.
- **Fight.**
- **Use its whole loadout** ([0008](0008-class-roster-and-loadouts.md)), capstone included:
  the tank, the gate, the rally.
- **React to visible threats**, such as mines and incoming shells. Traps are visible
  ([0004](0004-one-character-per-player-no-ai-units.md)), so a player would.

## Class, role and the pad

- **A class on each entry,** like a person, picked to fill what the team lacks. The prototype
  fixes class by slot.
- **A role from team need,** not from its seat: haul if nobody is bringing crates, defend if the
  pad is open, raid if nobody pressures the enemy. Rechecked every so often (about 15 s), not
  every tick.
- **What it builds:** a player recipe while its team has empty seats on the map, otherwise the
  biggest recipe the pad holds.
- **It doesn't take a human's item.** The kicker gets the item, so the bot doesn't kick open a
  pad while a human teammate is about to: near the pad (about 6 m) or carrying a crate to it.
  Player recipes are the exception, since they serve the whole team.
- **No talking, either way.** It never chats or sends messages, and it ignores pings. It reads
  only what teammates are doing on the map, such as following a human raider or covering an
  open pad. Answering pings can come later, if pings do.

## Seats

- **It enters, and waits, like a joined player:** through the pad on a player recipe, and out
  until the next one when knocked out.
- **Humans first on a player recipe.** Any waiting human comes in before any bot. Bots line up
  by wait among themselves. This adds to 0017's "whoever has waited longest".
- **Never the starting player,** and it never counts as "somebody" for 0017's "a team with
  nobody left loses". Otherwise a team that all quit would play on with bots.
- **Only once seats close.** A bot never holds a seat a queued player could take. A short-start
  seat nobody took gets its bot when seats close, not before.
- **A rejoiner displaces it.** A player more than 90 s away can still come back to a seat a bot
  took: the bot leaves, and the human returns at the pad if the bot was on the map, or waits for
  a player recipe if not. They are still a leaver, decided at the buzzer (0017).
- **In practice,** the bots a host adds are seated from the start. Seats don't close and nobody
  joins from the queue, but bots still enter through player recipes.

## Shown as a bot

- A **[BOT]** tag on its nameplate, the scoreboard and the results screen.
- A name no human could have, such as "Bot · Medic", and default cosmetics only.
- **No grade, mastery, award or pass XP**, and it's left out of the lobby that grades the
  humans ([0009](0009-progression-buys-identity-only.md)).

## Never

The bot never:

- takes or holds a seat a queued player could take, or keeps a rejoining player out of theirs;
- holds the starting-player role, or counts as somebody left on a team;
- kicks open a stack a human teammate is about to kick, except a player recipe;
- comes in ahead of a waiting human on a player recipe;
- uses information a player couldn't see, reacts faster than a human, or skips the server on a
  grab;
- chats or sends messages;
- passes for a human, or earns grades, mastery, awards or pass XP;
- throws on purpose, idles, or knocks its own pad's crates (the only friendly fire,
  [0016](0016-no-friendly-fire-and-one-random-spawn-spot.md)) more than a careful player would.

## The tutorial

- **The same brain, scripted.** A script layer sets the bot's goals and limits for each step,
  such as "carry this crate to your pad" or "let the player steal, then chase". There is no
  separate puppet, so the tutorial runs the bot that fills rated seats.

## Checking it

- **Before launch, headless:** bot-vs-bot runs (`CRATE_HEADLESS=1`) must clear a floor for each
  job, such as pad steals each match, little time standing still, kicks that aren't wasted and
  items used.
- **Before launch, with humans:** in playtests, a team with one human swapped for a bot should
  do about as well as with an average player, by feel and a rough win count.
- **After launch:** the server compares how often teams with a bot seat win against what the
  rating predicted, and moves the bot's rated value from 25 if they disagree.

## Considered options

- **A bot weaker than average:** safe against "the bot carried us", but its team is quietly
  short while the rating says it isn't.
- **A bot as strong as we can make it:** "the bot is better than my teammates", and a reason to
  fish for bot seats.
- **Scaling the bot to the lobby's rating:** its 25 would be wrong in every lobby but the middle.
- **Reading the game state directly:** cheaper to build, but it's cheating, and not a player's
  rules.
- **Always soldier, or a bot that only hauls and fights:** a hauler that never kicks is the
  team-owned AI hauler 0004 rejected.
- **Roles by slot,** as in the prototype: humans fill any role, so a fixed bot role often
  doubles one up.
- **A separate scripted puppet for the tutorial:** two bots to build, and the tutorial wouldn't
  test the one in rated seats.

## Consequences

- **The bot is a new build, not the prototype's.** Each prototype carries its own copy (about
  770 lines in `examples/launch_maps.rs`): a fixed priority list with roles by slot. It's a
  reference for the jobs, not the code to ship.
- **The bot needs a client's command path and a player's view** on the server, plus a reaction
  delay and aim error, so it can't skip what humans pay for.
- **The server tracks bot-seat results** against the rating's prediction, to tune the bot's
  rated value.
- **Settles** what the bot must do, which 0012 and 0019 left as "well enough not to break the
  match", and the practice bot settings 0019 left open, apart from tuning.
- **Open:** the reaction floor and aim error at easy and normal, the per-job floors, the
  pad-etiquette distance, how often roles are rechecked, and the playtest bar, once humans play.
