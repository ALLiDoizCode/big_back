# PROTOTYPE sheet: class items

Throwaway. Lives on `prototype/class-items` only, branched from `prototype/arena-map` (the
lanes + jungle map at 5v5, with the crate grammar picked in #17). One prototype for all three
item-set tickets, all on the same set switch: Set A is
[#24](https://github.com/ALLiDoizCode/big_back/issues/24), Set B is
[#25](https://github.com/ALLiDoizCode/big_back/issues/25) and Set C is
[#26](https://github.com/ALLiDoizCode/big_back/issues/26). Feeds the grilling in
[#22](https://github.com/ALLiDoizCode/big_back/issues/22).

Jump to [Final (#22)](#final-22-the-picked-kit), [Set B (#25)](#set-b-25-team-buddies-ladder) or [Set C (#26)](#set-c-26-a-capstone-per-class).

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
| 1 | item set (sets every slot at once) | **A: fix the placeholders**, **A + Set B's 1-crate refill**, **B: Team Buddies ladder**, **B, ambulance fires heal shots**, **C: a capstone per class**, **C, sparser barrage**, **Final (#22)** |
| 2 | left click (2 side by side) | A and C share it, B (shotgun / nail-mine layer / heal gun), Final (shotgun / rivet / heal gun) |
| 3 | right click (4 flat) | A and B share it, C (grenade launcher / shield wall / shield burst), Final (rockets / shield wall / shield burst) |
| 4 | 1 crate | A (throwable / kit), B (refill), C (adrenaline / barricade / self-patch) |
| 5 | all 8 | A, B (heal aura ambulance), B (heal shot ambulance), C (barrage / gate / rally), C (sparser barrage / gate / rally), Final (tank / gate / rally) |
| 6 | super crates | off, upgrade (one twist) |

**Enter** records the switch you're playing as your pick, and **P** prints your picks.

The crate grammar is fixed at what ADR 0007 picked: the kicker's class decides, recipes are
read by shape, the readout is shown, results are instant, pads are steal only, and you hold
the kick to choose. Hold F beside your pad. The readout steps from 1 crate up to all 8,
naming each result and its super twist. Let go to open.

## Final (#22): the picked kit

**Question (#22):** does the kit picked in the grilling hold together? Every slot was played and
felt good in its own set, but never in this mix: the gate has never met a tank, and the shield
burst has never met the shotgun.

```
cargo run --example class_items      # then press 1 until it says "Final (#22)"
```

| | Left click (2 side by side) | Right click (4 flat) | 1 crate (Q) | All 8 |
|---|---|---|---|---|
| **Soldier** | Shotgun (Set B) | Rockets (Set A) | Grenade (Set A) | Tank (Set A) |
| **Engineer** | Rivet gun (Set A) | Shield wall (every set) | Patch kit (Set A) | Gate (Set C) |
| **Medic** | Heal gun (every set) | Shield burst (Set C) | Med pack (Set A) | Rally (Set A) |

Each item plays with the numbers, super twist and calls from its own set, below. One rule
changed, from the grilling: **a super crate in a player recipe now gives each joiner their own
class's super left-click item** (a soldier comes in with a super shotgun, a medic with a super
heal gun), not a super rapid gun for everyone. This applies under every set.

### What to try (Final)

0. **Press 1 until it says "Final (#22)"** and play a full match as each class (C cycles your
   class). **V** gives you your class's all 8 now.
1. **Does any pairing break?** Watch the new meetings: a tank against a gate across a lane, rockets
   against shield walls, shotgun pushes into a shield-burst medic, and grenades against the
   rally's shield.
2. **Does each class still have a reason to be the kicker?** Is there a class nobody wants to kick
   as?
3. **Super players:** drop a super crate (**Y**) into 2 stacked. Does the joiner's super item read?

### Verdict

**2026-10-19: Final feels good.** The human, after playing it: "Final feels good, calls stand".
Every call from the sets it mixes stands, and so does the super player recipe giving each joiner
their own class's super left-click item. Recorded in ADR 0008.

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

---

## Set B (#25): Team Buddies ladder

**Question (#25):** does **Set B, "Team Buddies ladder"**, work? 1 crate refills what you hold,
and every class's all 8 is a vehicle its kicker drives
([research §8](https://github.com/ALLiDoizCode/big_back/blob/research/class-items/docs/research/class-items.md#set-b-team-buddies-ladder-1-crate-refills-all-8-is-always-a-vehicle)).
Compare it with the variant on **key 1**: the same set, but the ambulance has **no aura** and
its driver **fires heal shots** instead. That tests the research's warning about
fire-and-forget healing.

```
cargo run --example class_items      # then press 1 twice: "B: Team Buddies ladder"
```

### Verdict

**2026-10-18: Set B feels good.** The human, after playing it: "it feels good".

- **The ambulance keeps its heal aura** over the heal shot the driver fires. The human: "the
  heal aura".
- **The eight calls under "Set B, as built" stand.** The human: "calls are fine". That
  includes the minimap hiding enemies out of sight, for Set A too.

These carry to the grilling in #22 and to Set C (#26).

### Set B, as built

Every number is a live tunable in the panel, after the Set A ones (Up/Down to pick,
Left/Right to change).

| | Left click (2 side by side) | Right click (4 flat) | 1 crate | All 8 |
|---|---|---|---|---|
| **Soldier** | **Shotgun**: 5 pellets in a 30° fan, 12 damage each, out to 7 m, 1.25 shots/s, 12 shells | **Rockets** (as Set A) | **Refill**: left click to full, right click +1; +25 hp if you hold neither | **Tank** (as Set A) |
| *super* | 7 pellets in a fan 0.6× as wide | Rockets split into 3 | Both to full | 450 hp, slow zone |
| **Engineer** | **Nail-mine layer**: each click drops a mine at your feet (6 mines, 3 out at most). The first enemy to step on one takes 40 damage and is rooted 1 s | **Shield wall** (as Set A) | **Refill** | **Hauler mech**: 40 s or 400 hp, carries up to 3 crates, 0.8× speed, no weapon. Left click stomps: enemies within 3 m are shoved away |
| *super* | Mines show the victim on your minimap for 3 s | Allies shoot through | Both to full | Carries 4; the stomp knocks a crate off an enemy carrier |
| **Medic** | **Heal gun** (as Set A) | **Heal pulse** (as Set A) | **Refill** | **Field ambulance**: 40 s or 350 hp, 0.8× speed. Allies within 5 m (you too) heal 15 hp/s. A weak blaster (8 damage) |
| *super* | Heals bounce | Pulse adds a 25 shield | Both to full | Everyone in the aura gets a 25 shield every 5 s |

**The variant (key 1, "B, ambulance fires heal shots"; or key 5's third option):** the
ambulance has no aura. Left click fires a heal shot (+12 hp to the first hurt teammate it
touches, out to 12 m, 3 a second, 8 damage against enemies). Super: one shot every 5 s also
carries a 25 shield.

**How to drive the hauler mech:** **E** picks up the crate in reach, until it's full. E sets the
top crate down when you're beside your pad, when the mech is full, or when there's nothing to
pick up. **RMB** throws the top crate. **LMB** stomps (every 1.5 s). The crates ride stacked on
its roof. Once it's set them all down, it can kick the pad open.

Everything common to every set (base gun, items go to the kicker, the all-8 lockout, supers,
ADR 0002 and 0004) is as Set A has it, above. The lockout now also names the vehicle:
`all 8: blue LOCKED (hauler mech 21 s still out)`.

**Calls the table didn't make, made here so they can be played (say if any is wrong):**

1. **Shotgun numbers.** The table gives none: 12 damage a pellet (60 if all 5 hit, so 2 shots
   kill up close), a 30° fan, 7 m, 1.25 shots a second. The super's 7 pellets are its extra
   strength (1.4×), so there's no 1.25× on top.
2. **Mines.** They drop at your feet and arm after 0.5 s. Only enemies set them off. Both teams
   see them (ADR 0002), as a spiked ring in the layer's colour, dim until armed. Laying a 4th
   removes your oldest. Mines stay on the floor after you're knocked out (you lose the layer,
   not the mines). Like any left-click item, the layer **replaces your gun**: an engineer has
   no gun until all 6 are down.
3. **The minimap now hides enemies** unless one of your team can see them (within 20 m, no
   jungle wall between). Without that, "shows the victim on the minimap" means nothing,
   because the minimap showed everyone. This applies to Set A too. "minimap enemies" in the
   panel turns it back to always shown.
4. **The mech's crates only fall when it ends.** Like the tank, it soaks hits: shots don't
   knock its crates loose. They spill when it's destroyed or runs out of time.
5. **The stomp works while the mech carries.** It's not a shot, so ADR 0003 holds. It does no
   damage: a 3 m shove (12 m/s, dying away over about 2 m) and a 0.3 s stun. A plain stomp
   doesn't knock crates loose; a super one knocks one off a carrier, or the top one off an
   enemy mech.
6. **The aura heals the driver too**, and a super aura's shield goes to everyone in it (capped
   at 100). Super strength (1.25×) applies to the aura heal, the heal shot, mine damage and the
   stomp's shove. Vehicle hp isn't scaled (the table gives none), except the tank's 450.
7. **Getting into any vehicle drops your left-click item**, as the tank did in Set A. A crate in
   your hands goes into a mech.
8. **The ambulance moves at 0.8×**, like the tank. It can't carry.

### What to try (Set B)

0. **Press 1 until it says "B: Team Buddies ladder"** and play a match as each class (C cycles
   your class). The readout names each result:
   `1 crate -> refill > 2 side by side -> nail-mine layer (LMB) > 4 flat -> shield wall (RMB)`.
1. **Do three vehicle types crowd a 90 × 50 m map?** (main risk) Each team can have one vehicle
   out at a time (the lockout), so at most two are on the map at once. **V** gives you your
   class's all 8 now (Shift+V a super one). Try all three. Do vehicles take over the match,
   or are they rare treats? The match log counts `vehicle time` per team and `most out at
   once`.
2. **Does the ambulance stall fights?** (main risk) As a medic, **V** for an ambulance and park
   it by a fight. Does the fight end, or does it drag on while the aura heals? Then press **1**
   for **"B, ambulance fires heal shots"** and do the same. Is healing you aim more readable,
   for you and for the enemy? The match log counts `ambulance healed` and the kills made while
   an ambulance was out, against all kills.
3. **Does a mech carrying 3 or 4 crates break the haul rhythm?** (main risk) As an engineer,
   **V** for a mech and haul. Is one trip with 3 crates too strong against carrying one at a
   time? Can the enemy stop it (400 hp, and hits don't knock its crates off)? Raid red's pad
   with it. The match log counts `mech crates picked up` and `set on own pad`, and `crates
   stolen off pads`.
4. **Can you read the mines on a busy floor?** **M** gives every living red player a nail-mine
   layer (Shift+M super). Fight red near its pad and in the jungle. Do you see mines before you
   step on them? Does the 1 s root feel fair? As an engineer, open 2 side by side yourself:
   is a mine layer worth losing your gun for? The match log counts `mines laid` and `went off`.
5. **Supers**: drop a super crate with **Y** into a recipe and read the twist
   (`SUPER hauler mech [carries 4, stomp knocks crates off carriers]`). With a super mine,
   watch the victim's marker grow on the minimap.
6. **The refill** is the same one you compared in Set A. Does it read differently when every
   left-click item has ammo to refill (12 shells, 6 mines)?

### What it answers (Set B)

On top of Set A's counts, the match-end line (also printed with **P**) logs mines laid and how
many went off, crates the mechs picked up and set on their own pad, each team's vehicle time
and the most out at once, how much the ambulances healed, and the kills made while an
ambulance was out against all kills in the match.

### What the bot matches showed (Set B, 5 min, 5v5, bots on both teams, 4× speed)

Your seat was a bot, given its class's all 8 at 100–120 s (`CRATE_ALL8`):

| | Hauler mech | Ambulance (aura) | Ambulance (heal shot) | Tank |
|---|---|---|---|---|
| Score | 46–37 | 43–38 | 43–42 | 45–46 |
| Kills | 10–9 | 12–7 | 4–8 | 7–7 |
| Vehicle | 4 crates picked up, all 4 set on the pad | healed 79 hp in 40 s | healed 168 hp in 40 s | destroyed after 11 s |
| Kills while the ambulance was out | | 2 in 40 s (19 in 300 s overall) | 1 in 40 s (12 in 300 s overall) | |

In an earlier mech run the bots also built a mech of their own from 8 crates. The two mechs
picked up 12 crates and set 9 on the pad. Bots haul when crates are scarce, so a mech mostly
finds one or two at a time. Bots lay mines round their own pad and at their feet in a fight;
in these runs no bot stepped on one. Like Set A's, this shows the items working in a real
match, not what humans would choose.

### Notes on the model (Set B)

- Bots: a mech bot picks up crates until it's full (or none are within 10 m) and sets them on
  its pad, and stomps enemies within reach. An ambulance bot stays 2 m from the most hurt
  teammate (else the one nearest the enemy pad). A heal-shot ambulance aims at hurt
  teammates, as the heal gun does. A mine-layer bot lays mines at its feet when an enemy is
  within 8 m, and keeps 3 out round its pad.
- Bots don't see mines and walk over them.
- Smoke runs: `CRATE_RULES=2` is Set B and `CRATE_RULES=3` the heal-shot variant.
  `CRATE_ALL8=<s>` gives your seat its class's all 8 at that match time, and `CRATE_MINES=1`
  starts you with a super nail-mine layer.

---

## Set C (#26): a capstone per class

**Question (#26):** does **Set C, "a capstone per class"**, work? 1 crate is a short class
self-buff, and all 8 is a loud team-wide moment in the class's job
([research §8](https://github.com/ALLiDoizCode/big_back/blob/research/class-items/docs/research/class-items.md#set-c-each-class-its-own-capstone-1-crate-is-a-class-buff-all-8-is-a-team-moment)).
Compare it with the variant on **key 1**: the same set with a **sparser barrage** (6 shells over
4 s in place of 12). That tests whether the barrage can be dodged.

```
cargo run --example class_items      # then press 1 until it says "C: a capstone per class"
```

### Verdict

**2026-10-18: Set C feels good.** The human, after playing it: "feels good".

- **The barrage keeps 12 shells**, as written, over the sparser 6.
- **The eight calls under "Set C, as built" stand.** The human: "calls are fine". That
  includes the 1-crate buffs waiting for Q, the gate snapping across chokes, and no shooting
  while a barrage or gate waits to be placed.

These carry to the grilling in #22.

### Ruling before the build

**2026-10-18: second wind is out, and ADR 0005 stands.** Second wind would bring knocked-out
joined players back at the medic, away from the pad, and so rewrite ADR 0005. The human ruled
for "Rally instead", so Set C's medic all 8 is **Set A's rally**, unchanged.

### Set C, as built

Every number is a live tunable in the panel, after the Set B ones.

| | Left click (2 side by side) | Right click (4 flat) | 1 crate (Q) | All 8 |
|---|---|---|---|---|
| **Soldier** | **Rapid gun** (as Set A) | **Grenade launcher**: 3 charges, lobbed on a visible arc to where you aim (out to 14 m). It bounces once and goes off on the next touch, or on an enemy: 60 damage, 2 m blast | **Adrenaline**: 6 s of 1.3× fire rate. Picking up a crate ends it | **Barrage**: left click marks a 10 m circle (out to 25 m). 3 s later, 12 shells fall on it over 4 s: 50 damage each, 2 m blast |
| *super* | Shots pierce | Splits into 3 on the bounce (each half damage, 0.75× blast) | 8 s, and a kill reloads your left-click item | Shells leave a slow zone (Set A's: 3 s, 0.6× speed) |
| **Engineer** | **Rivet gun** (as Set A) | **Shield wall** (as Set A) | **Barricade**: a 2 m block, chest-high (1.3 m), 150 hp, 20 s, where you aim (1.5–8 m) | **Gate**: two 8 m walls (500 hp each, 45 s), placed one at a time with left click. Your team passes through them and enemies don't |
| *super* | Rivets stagger | Allies shoot through | Your team can jump over it | Enemies pressed against it are slowed for 1.5 s |
| **Medic** | **Heal gun** (as Set A) | **Shield burst**: 3 charges, a visible shot (10 m). The first ally it touches gets a 40 shield | **Self-patch**: +30 hp now, +30 more over 3 s | **Rally** (as Set A, by the ruling) |
| *super* | Heals bounce | Also clears slows and roots | Adds a 25 shield | 100 shield, clears slows |

**The variant (key 1, "C, sparser barrage"; or key 5's fifth option):** the barrage drops
6 shells over 4 s, so there's a gap between shells. Everything else is the same.

**How a barrage or gate is placed:** kicking it open doesn't use it. It waits, and the HUD says
`LMB: mark the barrage circle`. While it waits, **left click places it and fires nothing**. A
grey preview means the spot is past 25 m. If you don't place it within 30 s, or you're knocked
out first, it's lost. The all-8 lockout holds from the kick until the last shell lands, or the
last gate wall falls.

**How to read a barrage:** both teams see the circle, in the marker's colour, from the moment
it's marked. A ring closes in on it over the 3 s delay. Each shell is a visible ball falling
from 16 m at 12 m/s, and a red ring under it closes as it falls. If an enemy barrage is on you,
the HUD says `INCOMING BARRAGE: GET OUT OF THE CIRCLE`.

Everything common to every set (base gun, items go to the kicker, the all-8 lockout, supers,
ADR 0002 and 0004) is as Set A has it, above.

**Calls the table didn't make, made here so they can be played (say if any is wrong):**

1. **The 1-crate items wait for Q.** A self-buff that started at the pad would run out before
   the fight on a 90 m map. So adrenaline, the barricade and the self-patch are kept, like Set
   A's throwables, and Q uses them. They're lost on a knockout. Q doesn't work while carrying.
2. **Barrage numbers.** The table gives no damage: 50 a shell with a 2 m blast, so two hits
   knock a player out. Shells land at random spots spread evenly over the circle. They don't hurt
   your own team. Super strength (1.25×) applies to shell damage too. Bots ignore the 25 m mark
   range (they stand in for players who know the map).
3. **Grenade launcher numbers.** The table gives none: 60 damage, 2 m blast, 14 m range. Your
   own team doesn't set it off. The first touch, floor or wall, is the bounce. A super one's
   3 pieces are its extra strength, so there's no 1.25× on top.
4. **Adrenaline speeds up every gun**, the crafted ones and the standard gun. The super's
   "reload on kill" fills your left-click item to full. With only the standard gun there's
   nothing to reload. 8 s is the table's number, even though supers otherwise never add time.
5. **The barricade is chest-high (1.3 m)**, so it stops shots like a shield wall. A shot fired
   from the top of a jump goes over it. It stops bodies and crates from both teams. Nobody can
   jump it (a jump reaches 0.9 m), except your own team over a super one.
6. **The gate snaps across a choke.** A gate wall placed within 5 m of a choke snaps across it.
   That's any of the three lanes where they leave each base, or one of the six jungle
   entrances on each half. Anywhere else, it faces the axis nearest your aim. An 8 m wall shuts
   any lane or entrance on its own, so the two walls can shut two chokes, or double one up.
   Your team's bodies and shots pass through it. Enemy bodies, shots and grenades stop, and so
   do crates from both teams. Enemies can shoot it down. A super gate's walls get 1.25× hp,
   and so does a super barricade.
7. **Shield burst stops on the first player it touches.** An ally gets the shield, up to 100.
   An enemy gets nothing, and the charge is spent. You can't shield yourself. The super also
   clears roots, like slows.
8. **The super self-patch** gives 1.25× on both heals, and a 25 shield on top.

### What to try (Set C)

0. **Press 1 until it says "C: a capstone per class"**, and play a match as each class (C cycles
   your class). The readout names each result:
   `1 crate -> adrenaline (Q) > 2 side by side -> rapid gun (LMB) > 4 flat -> grenade launcher (RMB)`.
1. **Can you dodge the barrage?** (main risk) Press **B**: red marks a barrage where you stand.
   Get out of the circle in the 3 s. Then stand in it and try to dodge shell by shell. Press B
   in a fight and in a narrow lane. Then press **1** for **"C, sparser barrage"** and do the
   same. Does 12 shells in 4 s feel like a no-go zone you can only leave? Do 6 feel dodgeable
   from inside? As a soldier, press **V** and mark one on red. Is it a loud team moment, or does
   everyone just walk out? The match log counts `barrages`, the enemies `in the circle at the
   mark`, those still in it `when shells began`, and those `hit`.
2. **Do left-click items run dry before anyone reaches the pad?** (main risk) This set has no
   refill. Play a full match as any class, opening 2 side by side for your left-click item. Does
   it run out mid-fight, a long way from your pad? Does that make you hold off on using it? The
   match log counts `left-click items ran dry` and `lost on knockout`, and crafted shots
   against standard ones.
3. **Does the gate stall a lane?** (main risk) Press **J**: red shuts its mid lane and a side
   lane on its half. Try to reach red's pad: through the jungle, shooting the gate down
   (500 hp a wall), or round by the other lane. Is it a puzzle or a dead end? As an engineer,
   press **V** and shut a lane yourself. Aim near a lane's mouth and the preview snaps across
   it. The match log counts `gate up` seconds and `players held at an enemy gate`.
4. **The 1-crate buffs.** As each class, open 1 crate (**T** drops a crate) and use it with
   **Q**. Is a self-buff worth a crate? Does adrenaline's "picking up a crate ends it" ever make
   you choose? Does the barricade feel like cover? The match log counts `adrenaline cut by a
   crate`.
5. **The right clicks.** As a soldier, try the grenade launcher's bounce. Can you land it on
   someone? As a medic, fire shield bursts at bots in a fight. Is aiming a shield readable? The
   match log counts `shield burst gave`.
6. **Supers**: drop a super crate with **Y** into a recipe and read the twist
   (`SUPER barrage (LMB marks it) [shells leave a slow zone]`). Shift+B and Shift+J give red
   super ones.

### What it answers (Set C)

On top of the Set A and Set B counts, the match-end line (also printed with **P**) logs:

- each team's barrages, and the enemies in the circle at the mark, still in it when the shells
  began, and hit by a shell;
- each team's gate time, and the seconds each team's players were held at an enemy gate;
- left-click items run dry, and lost on a knockout;
- adrenaline cut short by a crate, and the shield the shield bursts gave.

### What the bot matches showed (Set C, 5 min, 5v5, bots on both teams, 4× speed)

Your seat was a bot, given its class's all 8 at 100 s (`CRATE_ALL8`):

| | Barrage (soldier) | Sparse barrage (soldier) | Gate (engineer) | Rally (medic) |
|---|---|---|---|---|
| Score | 37–45 | 20–50 | 37–47 | 41–45 |
| Kills | 8–11 | 2–6 | 7–8 | 9–9 |
| All 8 | 1 enemy in the circle at the mark, 0 when shells began, 0 hit | 1, 0, 0 | 2 walls across chokes, up 45 s, nobody held | rally at 100 s |
| Left-click items ran dry / lost on knockout | 0 / 2–3 | 0 / 1–1 | 0 / 3–4 | 0 / 2–0 |

Bots never stand still, so every enemy left the circle before the first shell. That says the
telegraph works on bots. It says nothing about humans in a fight, which is what B is for. No
left-click item ran dry: bots seldom open them, and lose them to knockouts first. Bots made
grenade launchers and shield bursts (one match: 50–40 shield given), but rarely the 1-crate
buffs. They open the biggest recipe their pad holds.

**One gate check, not shipped:** red's gates were moved onto blue's lanes for one run, where
blue's bots walk. They held blue's mid lane, and a blue bot shot one 500 hp wall down within
the 14 s the run lasted. That's how a gate falls, and roughly how fast against bots.

### Notes on the model (Set C)

- Bots:
  - a barrage bot pushes toward the enemy pad until an enemy is in sight within 25 m, then marks
    the circle on them;
  - a gate bot puts its walls at once across the two chokes nearest it that aren't shut;
  - soldiers lob launcher grenades at enemies 4–14 m away, aiming short to use the bounce;
  - medics fire shield bursts at a teammate with no shield, who's hurt or in a fight;
  - self-patch below 50 hp; adrenaline and a barricade when a fight is within 12 m.
- Bots walk up to an enemy gate and shoot it down. They don't dodge shells; they just keep
  moving.
- Bots barely raid the far half, so they rarely meet a gate on its own side. Use **J** and go
  there yourself.
- Smoke runs: `CRATE_RULES=4` is Set C and `CRATE_RULES=5` the sparser barrage. `CRATE_B=<s>`
  marks red's barrage on you at that match time, and `CRATE_J=<s>` puts up red's gates.
