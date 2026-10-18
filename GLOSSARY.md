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

**Recipe**:
A shape of crates on a pad, and what kicking the pad open gives. For example, two stacked crates bring in a player.
_Avoid_: blueprint, craft, build

**Kick open**:
To open a pad by kicking it, using up its crates to get the recipe's result. Nothing on a pad opens by itself.
_Avoid_: merge, craft, consume

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
