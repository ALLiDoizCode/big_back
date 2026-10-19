# big_back

A competitive online team arena game inspired by Team Buddies (PS1), with matchmaking and a ranked ladder. Every matchmade match is rated.

## Language

**Crate economy**:
The game's core resource loop: physical crates that a character carries with their body, stacks on a pad to build things, and can have stolen. A fixed design pillar.
_Avoid_: resources, currency, loot

**Launch lineup**:
The set of modes, queues and ladder rules that ship for Season 1: one mode, Crate Race, in one rated queue. Modes beyond it are only listed, not specified.
_Avoid_: mode roster, full lineup

**Mode**:
A set of rules for how a match is won. Season 1 has one; any later ones share the one queue and the one rating.
_Avoid_: playlist, game type, ruleset, casual mode

### Crate handling

**Carrier**:
A character holding a crate overhead. A carrier is slow, can't fire, and drops the crate when hit.
_Avoid_: hauler, holder

**Grab**:
Picking up a loose crate. The server decides it, judged on what the grabber saw (up to 200 ms back), and the crate reaches the hands when the answer arrives.
_Avoid_: pickup, take

**Steal**:
A grab of a crate off the enemy pad before it locks in.
_Avoid_: theft, snatch

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
An AI stand-in for a missing player, playing by a player's rules: a player's inputs, view and reaction time, at about an average player's strength. It is always shown as a bot, and a human always comes before it.
_Avoid_: buddy, AI unit, minion

**Team size**:
The most players a team can have in a match. Every team starts with one player and joins the rest from its pad.
_Avoid_: squad size, roster size

**Starting player**:
A team's first player, the only one who respawns when knocked out. If they go away, the role passes to the teammate who has been in the match longest.
_Avoid_: leader, captain, commander

**Joined player**:
A player who entered the match when the team kicked open a player recipe. A joined player who is knocked out stays out until another player recipe is opened.
_Avoid_: spawned player, reinforcement

**Friendly fire**:
Harm a team does to itself. In Crate Race, the only friendly fire is a crate knocking its own team's pad crates. Shots, blasts and thrown crates never hurt allies.
_Avoid_: team damage, team kill

### Map

**Lane**:
One of the three open routes between the bases: top, mid and bottom.
_Avoid_: path, corridor

**Jungle**:
The four walled quadrants between the lanes, entered by gaps in their walls and split into rooms. The heal and shield camps are here, two in each quadrant.
_Avoid_: forest, woods

**Base area**:
The walled end of the map behind each team's lanes, holding its pad and base. It opens onto the three lanes and through a gate into each of the team's jungle quadrants.
_Avoid_: base (that's the block), fountain, spawn, home

**River**:
The strip running north to south through the middle of the map, between the jungles.
_Avoid_: middle, centre line

**Camp**:
A PvE monster that drops something when cleared, then respawns: a heal camp, a shield camp or a river objective.
_Avoid_: creep, neutral, mob

**River objective**:
The bigger camp in the river, north and south, that drops crates.
_Avoid_: boss, dragon, baron

### Scoring and production loss

**Crate Race**:
The launch mode. Teams score the crates locked in on their pad, knock out the enemy base, and the higher score at the end wins.
_Avoid_: Domination, Siege, capture

**Score**:
What decides a match: a team's locked-in pad crates, plus crates spent on its pad, plus points banked from base knockouts.
_Avoid_: points (that's the rank), tally, crate count

**Base**:
The block behind a team's pad that enemy shots and blasts can hurt.
_Avoid_: HQ, core, nexus, tower

**Base knockout**:
A base brought to 0 hp. Its pad is down for a while, making and scoring nothing, then the base rebuilds. Production is never lost for good.
_Avoid_: knockout (that's a player), base kill, destruction, decapitation

**Lock-in**:
A crate that has rested on its pad long enough to count for the score. It can't be stolen after that, and it survives a base knockout.
_Avoid_: banking, securing, scoring a crate

**Spill**:
The crates a base knockout throws onto the map, a few at most, where anyone can pick them up.
_Avoid_: drop, scatter, loot

**Megacrate**:
A rare crate worth several points of score, which drops on the trailing team's side. Unlike a super crate, it doesn't change what a recipe makes.
_Avoid_: gold crate, bonus crate, super crate

**Timer**:
How long a Crate Race match is played before the buzzer: 8 minutes. Overtime can run past it.
_Avoid_: clock, round, match length

**Overtime**:
Play past the buzzer that lasts while the trailing team carries a crate or contests the leader's pad.
_Avoid_: sudden death, extra time, golden goal

**Mercy rule**:
The match ending early because one team has held a big enough score lead for long enough.
_Avoid_: surrender, forfeit, early end

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

**Party cap**:
How far apart in skill a party may be and still queue: 2 tiers for a duo or a party of 3, 3 tiers for a party of 4. It is shown in tiers and divisions, and enforced on whichever is wider, rank or rating. A full 5-stack is exempt but plays for fewer points.
_Avoid_: party restriction, rank restriction, duo limit

**Wide 5-stack**:
A full five-player party more than 3 tiers apart. It may queue, but its rank points are cut by its spread (−25%, −50% or −75%), and its wins don't count toward ranked-win rewards.
_Avoid_: wide group, stack penalty

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
A new account's first 5 rated matches, played with no rank shown. They set its starting rank. A match made unrated by a bot seat doesn't count.
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

### Matchmaking

**Queue**:
The one pool that matchmade play draws from. Solos and parties queue together across regions, and every match in it is rated.
_Avoid_: playlist, casual queue, ranked queue, lobby

**Skill band**:
How far apart in rating the matchmaker lets players be. It starts narrow and widens while a player waits.
_Avoid_: MMR range, search range

**Short start**:
Starting a match once every team has its starting player, before every seat is filled.
_Avoid_: early start, partial lobby, backfill

**Open seat**:
A place on a team that a player still in the queue can take as a joined player, until the seat closes partway through the match. A short-start seat nobody takes goes to a bot, and that match is unrated. A leaver's seat also reopens, but never unrates the match.
_Avoid_: slot, backfill, empty spot

**Late joiner**:
A player who takes an open seat after the match has started, and enters when their team next opens a player recipe.
_Avoid_: backfill, reinforcement, latecomer

### Leavers

**Away**:
Said of a player who is disconnected, idle with no input, or not yet loaded in. An away player's character leaves the map, and their seat counts as empty.
_Avoid_: AFK, disconnected, DC

**Rejoin**:
An away player coming back into the match, possible until the buzzer unless their seat has been taken.
_Avoid_: reconnect, return

**Leaver**:
A player who quit a match on purpose, or was away too long in total. Decided at the buzzer. A leaver takes a loss and a lockout, and their teammates lose no points.
_Avoid_: quitter, abandoner, deserter

**Lockout**:
The time a leaver can't queue. It steps up with each leave and steps down with matches finished.
_Avoid_: ban, low-priority queue, cooldown

**Partial play**:
Counting each player in the rating by the share of the match they were on the map.
_Avoid_: time weighting, participation

### Ladder integrity

**Win-trading**:
Two parties queuing together to meet as opponents and take turns losing.
_Avoid_: queue sniping, match fixing

**Boosting**:
A strong player carrying a weaker account up the ladder, usually in a party.
_Avoid_: carrying, elo boosting

**Throwing**:
Losing on purpose while on the map, to drop rank.
_Avoid_: deranking, tanking, inting

**Flag**:
A mark the server raises automatically on accounts whose matches look like win-trading, boosting or throwing. Only a reviewer acts on it.
_Avoid_: report, strike, ban

**Hold**:
Rank points from flagged matches, kept back from the rank until a review or until 7 days pass. The player sees them as held, without a reason.
_Avoid_: freeze, pending points, suspension

**Offence**:
Abuse a reviewer has confirmed. Each one steps up the penalty, and offences never step down.
_Avoid_: strike, violation

### Onboarding

**Tutorial**:
The required solo lesson, once per account, that stands between first launch and the queue. It teaches the crate grammar and one full Crate Race loop, and is passed by finishing each step.
_Avoid_: training, boot camp, new player experience

**Practice**:
A custom lobby against bots, open any time, with any party. It is unrated and earns nothing.
_Avoid_: casual, training mode, vs AI, bot match
