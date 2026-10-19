# Class roster and loadouts

Season 1 has **three classes, soldier, engineer and medic**, and each class's loadout is fixed
below. We picked each item in play. Three candidate sets were each played and each "feels good"
([#24](https://github.com/ALLiDoizCode/big_back/issues/24),
[#25](https://github.com/ALLiDoizCode/big_back/issues/25),
[#26](https://github.com/ALLiDoizCode/big_back/issues/26)). The grilling in
[#22](https://github.com/ALLiDoizCode/big_back/issues/22) mixed the best slot from each, and the
mix was played too ([sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/class-items/docs/prototypes/class-items.md#final-22-the-picked-kit)).
The human's verdict: "Final feels good, calls stand".

| | Primary item (1 × 2) | Secondary item (2 × 2 flat) | Quick item (1 crate) | Capstone (all 8) |
|---|---|---|---|---|
| **Soldier** | Shotgun | Rockets | Grenade | Tank |
| *super* | 7 pellets, tighter spread | Rockets split into 3 | Sticks to the first thing it touches | 450 hp, cannon leaves a slow zone |
| **Engineer** | Rivet gun, repairs your walls | Shield wall | Patch kit | Gate across a choke; your team passes |
| *super* | Rivets stagger | Your team's shots pass through | +2 charges | Enemies pressed against it are slowed |
| **Medic** | Heal gun (visible heal shots) | Shield burst on an aimed ally | Med pack, thrown; anyone can take it | Rally: team to full health + shield |
| *super* | Heals bounce to a second ally | Also clears slows and roots | Adds a shield | 100 shield, clears slows |

Numbers and the calls made while playing are in the sheet. They are starting points.

- **Picking a class:** a player picks each time they come into the match, when a player
  recipe brings them in or when the starting player respawns. The class is locked while they're
  alive. There is no cap on duplicate classes.
- **How items run out:** the primary item replaces the base gun until its ammo runs out. The
  secondary item uses charges and works only with empty hands. The quick item is kept until
  used once, and never while carrying ([0003](0003-carrying-is-a-commitment.md)). A capstone
  ends on its timer or its hp.
- **One item per slot.** A new item replaces the one held; nothing stacks. Items go to the
  kicker and are all lost on knockout. Walls already placed stay. Getting into a vehicle drops
  the primary item.
- **Capstone lockout:** a team gets no new capstone until its current one ends.
- **Supers** add one behaviour twist and 1.25× strength, never more ammo, charges or time. A
  super crate in a player recipe gives each joiner their own class's super primary item.
- **Readability:** the class shows on the body, with no floating markers. Every crafted item
  shows on the character and has its own projectile colour and shape. Super items keep their
  silhouette with a purple trim. Every capstone sets off a map-wide sound and a minimap icon.
  Loudness follows cost.

## Considered options

- **1 crate:** full health (it makes the medic and the heal camps pointless), a refill of what
  you hold (the throwables were better in play), and class self-buffs (adrenaline, barricade,
  self-patch: they can't be seen on the body, or they double up a secondary item).
- **Capstones:** the bunker (risks a dead pause against steal only, [0007](0007-crate-grammar-class-loadouts-by-shape.md)),
  the hauler mech (gets round the carry commitment and the haul rhythm), the field ambulance
  (a fire-and-forget heal aura that stalls fights), and the barrage.
- **Second wind**, a medic capstone that brings knocked-out players back away from the pad.
  Ruled out on #26: [0005](0005-players-join-from-the-pad.md) stands.
- **Turrets**, ruled out under [0004](0004-one-character-per-player-no-ai-units.md#deployables).
