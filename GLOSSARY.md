# big_back

A competitive online team arena game inspired by Team Buddies (PS1), with matchmaking, a ranked ladder and casual modes.

## Language

**Crate economy**:
The game's core resource loop: physical crates that a character carries with their body, stacks on a pad to build things, and can have stolen. A fixed design pillar.
_Avoid_: resources, currency, loot

**Launch lineup**:
The set of modes, queues and ladder rules that ship for ranked Season 1. Modes beyond it are only listed, not specified.
_Avoid_: mode roster, full lineup

### Crate handling

**Carrier**:
A character holding a crate overhead. A carrier is slow, can't fire, and drops the crate when hit.
_Avoid_: hauler, holder

**Pad**:
A team's building area: 2 × 2 cells, at most two crates high. Crates are stacked on it to make a recipe, and can be stolen from it.
_Avoid_: build zone, stack area

**Cell**:
A crate-sized square of the arena grid, which lines up with the pads. A crate at rest sits in one.
_Avoid_: slot, tile

**Jump-throw**:
A full throw, made in the air. Its reach is greatest at the top of the jump.
_Avoid_: power throw, apex throw

**Toss**:
A short, grounded throw. It's the only throw available without jumping.
_Avoid_: drop, ground throw

**Touchdown point**:
Where a thrown crate first lands, and what the player aims at. The crate then slides on from there.
_Avoid_: target, landing spot

**Curling landing**:
How a thrown crate comes to rest: it slides from its touchdown point and settles into a cell.
_Avoid_: snap, drop, placement

**Knock**:
A sliding crate running into another crate and passing on its momentum.
_Avoid_: bump, push, hit

**Ghost**:
The preview of where a carried crate will come to rest if thrown or placed now.
_Avoid_: preview, marker, reticle

**Sealed crate**:
A crate that can't be carried and must be kicked open to release crates.
_Avoid_: loot crate, weapon crate

**Super crate**:
A rare crate that upgrades whatever recipe it's part of into a stronger version of the same result.
_Avoid_: gold crate, rare crate, power-up

**Recipe**:
A shape of crates on a pad, and what kicking the pad open gives. For example, two stacked crates bring in a player.
_Avoid_: blueprint, craft, build

**Kick open**:
To open a pad by kicking it, using up a recipe's crates to get its result. Holding the kick longer steps up to bigger recipes inside the pad; letting go opens the one reached. Nothing on a pad opens by itself.
_Avoid_: merge, craft, consume

**Class**:
What a player is (for example soldier, engineer or medic). The class of whoever kicks a pad open decides what its non-player recipes make.
_Avoid_: role, job, hero

**Kicker**:
The player who kicks a pad open. The kicker's class decides what the recipe makes, and the kicker gets the item.
_Avoid_: builder, crafter, opener

**Loadout**:
A class's four crafted items: its primary item, secondary item, quick item and capstone.
_Avoid_: kit, abilities, build

**Primary item**:
What 1 × 2 makes: a weapon that replaces the base gun until its ammo runs out.
_Avoid_: left-click item, LMB, main weapon

**Secondary item**:
What 2 × 2 flat makes: a tool with charges, used with empty hands.
_Avoid_: right-click item, RMB, ability

**Quick item**:
What 1 crate makes: a single-use item kept until used.
_Avoid_: Q item, consumable, throwable

**Capstone**:
What all 8 crates make: a class's biggest result, such as a vehicle or a team-wide effect.
_Avoid_: ultimate, super, all-8 item

**Capstone lockout**:
The rule that a team can't make a new capstone while its current one is still out.
_Avoid_: cooldown, all-8 lock

### Teams

**Player**:
A person in a match, controlling exactly one character. No player owns or commands AI units.
_Avoid_: buddy, unit, squad member, leader

**Bot**:
An AI stand-in for a missing player, playing by a player's rules.
_Avoid_: buddy, AI unit, minion

**Team size**:
The most players a team can have in a match. Every team starts with one player and joins the rest from its pad.
_Avoid_: squad size, roster size

**Starting player**:
A team's first player, the only one who respawns when knocked out.
_Avoid_: leader, captain, commander

**Joined player**:
A player who entered the match when the team kicked open a player recipe. A joined player who is knocked out stays out until another player recipe is opened.
_Avoid_: spawned player, reinforcement

### Map

**Lane**:
One of the three open routes between the bases: top, mid and bottom.
_Avoid_: path, corridor

**Jungle**:
The four walled quadrants between the lanes, entered by gaps in their walls. The PvE camps are here.
_Avoid_: forest, woods

**River**:
The strip running north to south through the middle of the map, between the jungles.
_Avoid_: middle, centre line

**Camp**:
A PvE monster that drops something when cleared, then respawns: a heal camp, a shield camp or a river objective.
_Avoid_: creep, neutral, mob

**River objective**:
The bigger camp in the river, north and south, that drops crates.
_Avoid_: boss, dragon, baron

### Progression

**Identity**:
What a player can earn across matches: cosmetics, titles and proof of skill. It never changes a match.
_Avoid_: unlocks, rewards, power

**Readability rule**:
The list of things a cosmetic may never change, such as a body's silhouette, team colour, crates and how a stack reads.
_Avoid_: cosmetic rules, skin limits

**Grade**:
A player's D to S result for a class in one match, measured against the whole lobby.
_Avoid_: score, rating, performance rank

**Mastery**:
A player's seasonal progress with one class, earned by grades and paid out in milestones and a title.
_Avoid_: class level, hero level, XP

**Pass**:
The one free, linear track of cosmetics that a player moves along by playing matchmade games.
_Avoid_: battle pass, season pass, track

**Challenge**:
A goal that adds pass XP, counted only from matchmade play and capped per match.
_Avoid_: quest, mission, bounty

**Joke award**:
The single public award each player gets on the results screen, framed as a joke.
_Avoid_: medal, accolade, card

### Rating

**Rating**:
A player's hidden skill estimate: a mean and an uncertainty, updated after every matchmade ranked match from the win or loss alone. It drives matchmaking. The visible rank is separate.
_Avoid_: MMR, Elo, score, rank

**Party**:
Players who queue together and are put on the same team.
_Avoid_: premade, stack, squad, group

**Party bonus**:
How much stronger a party plays than its players' ratings add up to, per extra member. It is learned from match results, not set by hand.
_Avoid_: squad offset, premade penalty, synergy

**Settled**:
Said of a player whose ladder position stays close to where their real skill belongs. It is how fast-converging a rating system is measured.
_Avoid_: converged, placed, calibrated

### Ladder

**Rank**:
What a player is shown: a tier, a division and points. It follows the rating through points, but it is not the rating.
_Avoid_: MMR, rating, level, elo

**Tier**:
One of the seven named steps of the ladder: Bronze, Silver, Gold, Platinum, Diamond, Master and Champion.
_Avoid_: league, rank group, medal

**Division**:
One of the three steps (III, II, I) inside each tier from Bronze to Diamond, 100 points each.
_Avoid_: sub-rank, stage, level

**Points**:
What a match adds to or takes from a rank. Wins pay more and losses cost less while the rank sits below the rating.
_Avoid_: LP, RR, SR, stars

**Placements**:
A new account's first 5 ranked matches, played with no rank shown. They set its starting rank.
_Avoid_: calibration, qualifiers, provisional matches

**Decay**:
The 50 points a Diamond-or-higher player loses for each idle day with no days banked. Each match banks 7 days, up to 28.
_Avoid_: inactivity penalty, rank rot

**Champion**:
The top tier: a leaderboard of the Master players who are above the Champion rating line and active.
_Avoid_: Grandmaster, Legend, top 500

**Season**:
A 3-month ranked period. Each one starts with every rank dropped one tier and ends by paying ranked-win rewards.
_Avoid_: split, act, episode

**Ranked-win reward**:
A dated season cosmetic for a tier, earned with 10 wins at that tier or above in the season.
_Avoid_: season reward, rank reward, end-of-season chest
