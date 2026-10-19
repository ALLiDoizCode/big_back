# PROTOTYPE sheet: class items

Throwaway. Lives on `prototype/class-items` only, branched from `prototype/arena-map` (the
lanes + jungle map at 5v5, with the crate grammar picked in #17). One prototype for all three
item-set tickets: Set A is [#24](https://github.com/ALLiDoizCode/big_back/issues/24); Set B
([#25](https://github.com/ALLiDoizCode/big_back/issues/25)) and Set C
([#26](https://github.com/ALLiDoizCode/big_back/issues/26)) add their options to the same set
switch. Feeds the grilling in [#22](https://github.com/ALLiDoizCode/big_back/issues/22).

```
cargo run --example class_items
```

**Question (#24):** does **Set A, "fix the placeholders"**, work? It keeps the prototype's
items wherever ADR 0004 allows, and adds a class throwable on 1 crate and a class capstone on
all 8 ([research §8](https://github.com/ALLiDoizCode/big_back/blob/research/class-items/docs/research/class-items.md#set-a-fix-the-placeholders-closest-to-the-current-prototype)).
Compare it with Set A plus **Set B's 1-crate refill**: a throwable against a refill.

## Verdict

**2026-10-18: Set A feels good.** The human, after playing it: "feels good".

- **1 crate: Set A's throwables beat Set B's refill.** The human: "the throwables are
  better".
- **The five calls under "Set A, as built" stand.** The human: "calls are fine". Walls block
  both teams, the bunker's gap is open unless super, anyone can take a med pack, the patch
  kit fills a charge before a wall, and grenades do 70.

These carry to the grilling in #22 and to Set B and Set C (#25, #26).

## The set switch (keys 1–6)

The switch is **live**: the match keeps going and your next kick uses the new items. Items
you already hold keep working. R restarts the match.

| Key | Switch | Options now |
|---|---|---|
| 1 | item set (sets every slot at once) | **A: fix the placeholders**, **A + Set B's 1-crate refill** |
| 2 | left click (2 side by side) | A |
| 3 | right click (4 flat) | A |
| 4 | 1 crate | A (throwable / kit), B (refill) |
| 5 | all 8 | A |
| 6 | super crates | off, upgrade (one twist) |

**Enter** records the switch you're playing as your pick, and **P** prints your picks. Set B
and Set C add their options to keys 1–5.

The crate grammar is fixed at what ADR 0007 picked: the kicker's class decides, recipes are
read by shape, the readout is shown, results are instant, pads are steal only, and you hold
the kick to choose. Hold F beside your pad. The readout steps from 1 crate up to all 8,
naming each result and its super twist. Let go to open.

## Set A, as built

All numbers are live tunables in the panel (Up/Down to pick, Left/Right to change).

| | Left click (2 side by side) | Right click (4 flat), empty hands | 1 crate | All 8 |
|---|---|---|---|---|
| **Soldier** | **Rapid gun**: 6 shots/s, 15 damage, 60 rounds | **Rockets**: 4 charges, 14 m/s, 70 damage, 2.5 m blast; blows loose crates about | **Grenade (Q)**: 1, thrown on the jump-throw arc to where you aim (9 m, more from a jump); 2 m blast, 70 damage, 1.5 s after the throw | **Tank**: 45 s or 300 hp, rocket cannon, can't carry, 0.8× speed |
| *super* | Shots pierce one target | Each rocket also splits into 3 small blasts | Sticks to the first thing it touches | Cannon blasts leave a 3 s slow zone (0.6× speed); 450 hp |
| **Engineer** | **Rivet gun**: 2 shots/s, 30 damage, 40 rivets; a rivet that hits your own team's wall repairs it 20 hp | **Shield wall**: 2 charges, a 4 m wall where you aim (1.5–8 m), 250 hp, 20 s | **Patch kit** (instant): +1 right-click charge if one is missing, else +60 hp on your nearest hurt wall | **Bunker**: a horseshoe of walls round your pad (2.6 m out, 600 hp each, 40 s), with a 2 m gap facing the arena |
| *super* | Rivets stagger (0.2 s on top of the 0.15 s every hit gives) | Your team's shots pass through it | +2 charges | A gate fills the gap: your team walks through, enemies can't |
| **Medic** | **Heal gun**: a heal shot that gives +8 hp to the first hurt teammate it touches, out to 10 m; a weak 8-damage shot against enemies; 80 shots | **Heal pulse**: 3 charges, +40 hp to you and teammates within 7 m | **Med pack (Q)**: thrown like the grenade; +40 hp to **whoever** walks over it | **Rally** (instant): every living teammate goes to full health with a 50 shield; the medic can't be hurt for 4 s |
| *super* | Heals bounce on to a second hurt ally within 4 m | Pulse also gives a 25 shield | +40 hp and a 25 shield | 100 shield, and clears slows (immune to them for 4 s) |

Common to every set:

- **Base gun** for everyone: 25 damage, 3 shots/s, 18 m/s.
- **Left click replaces the gun** until its ammo runs out. **Right click works with empty
  hands only**; while carrying, right click still throws. **Q throws the 1-crate item**, never
  while carrying (ADR 0003).
- **Items go to the kicker** and are lost on knockout: left click, right click and the Q item.
- **All 8 has a team lockout.** While a team's tank, bunker or rally is out, all 8 isn't
  offered inside its pad: the hold stops at 4 flat. The HUD shows `all 8: blue LOCKED (tank
  23 s still out)`. A rally's lockout lasts as long as the medic can't be hurt (4 s).
- **Supers** add one twist and **1.25× strength** (the `super strength` tunable) to damage,
  healing and wall hp. They never add ammo, charges or time. The table fixes two values: the
  super tank's 450 hp (1.5×) and the super rally's 100 shield. A super crate in a player
  recipe still makes the new players join with a rapid gun.
- **ADR 0004:** the turret drop and the fortress turret are gone. Nothing picks its own
  targets or moves by itself: walls are static, grenades only fly where they're thrown, and
  the heal gun heals whoever its shot touches.
- **ADR 0002:** every shot, heal, rocket, grenade and med pack is a visible projectile. The
  heal gun used to heal instantly in a cone. It now fires a heal shot.

**Calls the table didn't make, made here so they can be played (say if any is wrong):**

1. **Walls block everyone.** Shield walls and bunker walls stop players, sliding crates and
   shots from **both** teams. The super's "allies' shots pass through" only means something
   if a plain wall stops your own team's shots. Enemy shots hurt a wall. Your own team's
   shots just stop, except rivets, which repair it.
2. **The bunker's gap is open to everyone** unless it's super. A super bunker's gate stops
   only enemy bodies; shots pass over it. Breaching any bunker wall also removes the gate.
3. **Anyone can pick up a med pack**, enemies included ("whoever picks it up").
4. **Patch kit order:** a missing charge first, then the nearest hurt wall of yours. If
   neither applies, the charge goes over full. With no right-click item and no hurt wall,
   it's wasted, and the match log says so.
5. **Grenade damage is 70** (the table gives none), the same as a rocket.

## What to try

0. **Play a match on Set A first**, as any class (C cycles your class). Note what you open
   and why. The pad readout lists every step with its result:
   `1 crate -> grenade (Q) > 2 side by side -> rapid gun (LMB) > 4 flat -> rockets (RMB)`.
1. **The engineer's left click** (main risk). Play a match as an engineer. Do you open
   2 side by side for the rivet gun, or skip it and save the crates for walls (4 flat) and
   the bunker? Does repairing your wall with rivets ever matter? The match log counts
   `crafted LMB shots` against `standard` and `wall hp repaired`.
2. **The bunker: a siege or a dead pause?** (main risk, ADR 0007 "steal only")
   - Raid one: press **G** and red puts up a bunker round its pad now (**Shift+G** for a
     super one with the gate). Walk to red's pad and try to steal from it. You can go
     through the gap, jump-throw over, or shoot a wall down (600 hp). Is 40 s tense, or do
     you just wait it out?
   - Build one: C to engineer, then drop 8 crates on your pad with **T** (or haul them), and
     hold F to the end. Is sitting in it fun? The match log counts
     `crates stolen off pads (while bunkered)` and `bunker up` seconds.
3. **Rally against the heal camp.** As a medic, open all 8 when your team is hurt. Do you
   still walk to the heal camp? The match log counts `heal camp pickups` against
   `med pack pickups`.
4. **Variant 2: press 1** for **A + Set B's 1-crate refill**, and play the same class again.
   One crate now tops up what you hold (left click to full, right click +1), or gives +25 hp
   if you hold nothing. Is a refill better than a grenade, patch kit or med pack? Does it
   make you keep your crafted items longer?
5. **Supers**: drop a super crate with **Y** (purple), put it in a recipe and read the
   readout's twist (`SUPER rockets (RMB) [rockets split into 3]`). Is the twist readable from
   the other side? Is 1.25× enough to fight over?
6. **The all-8 lockout**: open a tank, then build 8 again while it's out. The hold should
   stop at 4 flat. Does the lockout feel fair, or like a punishment for hauling?

**Shortcuts**: **T** drops a crate in front of you and **Y** a super crate. **G** gives red
a bunker. "blue bots build: off" in the panel leaves your pad to you. Every number is in the
panel.

## What it answers

The match-end line (also printed with **P**) logs the switch, your class, kills, crates
stolen off each pad (and how many of those while bunkered), how long each bunker stood, wall
damage taken, wall hp repaired, heal camp against med pack pickups, crafted left-click shots
against standard ones, and everything each team made, with each class and super twist.

## What the bot matches showed (5 min, 5v5, bots on both teams)

Three bot matches, two on Set A and one on A + refill:

| | Set A (bot soldier) | Set A (bot soldier) | A + refill (bot engineer) |
|---|---|---|---|
| Score | 43–37 | 41–46 | 41–37 |
| Kills | 8–7 | 9–8 | 11–6 |
| Items made (both teams) | 3 shield walls, 6 rockets (2 super), 1 heal gun | 4 shield walls (2 super), 4 rockets (1 super), 1 rivet gun | 4 shield walls (1 super), 2 super rockets |
| 1 crate or all 8 made | none | none | none |
| Wall damage taken | 586–108 | 62–218 | 375–125 |
| Crafted left-click shots against standard ones | 168 against 474 | 185 against 407 | 109 against 476 |

Bots only open the biggest recipe their pad holds. They mostly build players, so items only
start coming once a team is full. This says little about what humans would choose. It does
show the items working in a real match.

## Notes on the model

- Bot classes go by slot: soldier, engineer, medic, then again. Bots use their items:
  - rockets at enemies within 12 m, grenades at enemies 4–9 m away;
  - a shield wall between them and a fight, if they have none within 6 m;
  - rivets on a hurt wall of theirs;
  - heal shots at hurt teammates, a heal pulse when someone near is below 60 hp, and med
    packs thrown at the most hurt teammate close by.
- Bots walk round shield walls, go in and out of a bunker through its gap, and shoot a super
  bunker's gate when it shuts them out. Their pathing ignores walls otherwise, so a bot can
  get stuck on one for a while.
- Bots rarely raid the far pad on a 90 m map, so they barely test the bunker. Use **G** and
  raid it yourself.
- Smoke runs: `CRATE_RULES=<6 digits>` sets each switch by option index (key 1's digit also
  sets the slots: `CRATE_RULES=1` is A + refill), `CRATE_CLASS=<0-2>` your class, and
  `CRATE_SEAT=bot`, `CRATE_ARENA=1`, `CRATE_SPEED=4`, `CRATE_MAP=open`,
  `CRATE_EXIT_AFTER=<s>`, `CRATE_SCREENSHOT=<png>` as before.
