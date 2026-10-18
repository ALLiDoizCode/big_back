# PROTOTYPE sheet: crate economy grammar (issue #17)

Throwaway. Lives on `prototype/crate-grammar` only (branched from `prototype/squad-model`).

```
cargo run --example crate_grammar
```

**Question:** what does stacking crates build in competitive play, and how vulnerable is a
stack while it's being built? Five things to pick: what stacks build, the recipe shape and
how readable it is, super crates, build time, and how vulnerable an unfinished stack is.

**Resolved when** you've played at least two options of recipe shape, build time and
vulnerability, and picked a grammar (press **Enter** on the rules you like).

## Already decided (fixed here)

- Core feel from #9: curling jump-throws, carrying is a commitment, 18 m/s projectiles
  (ADRs 0001–0003).
- From #10: one character per player, no AI units, 3v3 (ADR-0004). Teams start with one
  player. The pad is **2 × 2 cells, at most 2 high**, nothing opens until it's kicked, and
  **2 stacked = 1 player, 2 wide × 2 high = 2 players** (ADR-0005).

The issue's other options are ruled out by those: **owned Buddies or neutral haulers** (ADR-0004),
and **1×N towers or 3×3 blocks** (the pad is 2 × 2 × 2). What's left to pick is what the
other four shapes build, and the rules around them.

## The rules (keys 1–6)

Each key steps through that rule's options and restarts the match. The box top-left always
shows what you're playing, with the current option in `[brackets]`.

| Key | Rule | Options |
|---|---|---|
| **1** | What stacks build | **mix**, **weapons**, **base upgrades** (table below), **by kicker's class** (see "Classes") |
| **2** | Recipe shape | **by shape** (Team Buddies) · **by count only** (any arrangement; the number of crates decides) |
| **3** | Pad readout | **secret**: the HUD only says "4 crates: what will it make?", the pad never glows, and kicking a non-recipe does nothing · **pad shows what it makes**: `your pad: 4 flat -> rocket launcher`, gold outline when it can open |
| **4** | Super crates | **off** · **upgrade**: a purple crate in a recipe makes the SUPER version · **branch**: it makes the result from another book instead |
| **5** | Build time | **instant** · **1 s per crate** · **2 s per crate**, counted after the kick. While building, the crates are merged into one locked lump with 30 hp per crate. Enemy shots wear it down, and if it breaks the crates spill out and nothing is made. |
| **6** | Vulnerability of an unfinished stack | **steal only** (enemies take crates off your pad, as in #10) · **a hit splits the stack**: an enemy shot crossing a stack knocks its top crate off · **a hit spills the pad**: one hit scatters every crate on it · **locks once placed**: a crate that settles in a cell locks. Nobody can take it, knock it or shoot it off; only a kick uses it. |

Stealing is a fixed pillar (see the glossary), so "split" and "spill" add to stealing rather
than replacing it. "Lock" is the only option that stops stealing off pads. Stealing loose and
carried crates still works.

### What each shape makes

| Shape (count only) | mix | weapons | base upgrades | mix, branched by a super crate |
|---|---|---|---|---|
| 1 crate (1) | full health | shotgun | heal your team | shotgun |
| 2 side by side (3) | rapid gun | rapid gun | shield wall | shield wall |
| 2 stacked (2) | 1 player | 1 player | 1 player | 1 player |
| 2 wide × 2 high (4) | 2 players | 2 players | 2 players | 2 players |
| 4 flat (5–7) | turret | rocket launcher | turret | rocket launcher |
| all 8 (8) | tank | tank | fortress (turret + wall) | fortress |

A branched weapons book gives the base-upgrades results, and the other way round. Super
crates never branch a player recipe. Instead, a super crate there arms the new players with a
super rapid gun.

### Classes (rule 1: "by kicker's class")

Every player has a class, and the class of whoever **kicks the pad open** picks what the
non-player shapes make. The player recipes don't change. You start as a soldier; **C** changes
your class at any time without restarting. Bots get classes by slot: a team's 1st player is a
soldier, its 2nd an engineer and its 3rd a medic. So who joins from the pad decides what your
team can build.

| Shape | soldier | engineer | medic |
|---|---|---|---|
| 1 crate | shotgun | shield wall | full health |
| 2 side by side | rapid gun | turret | heal your team |
| 4 flat | rocket launcher | fortress (turret + wall) | shield wall |
| all 8 | tank | tank | fortress |

With the readout shown, a pad lists every class's result: `4 flat -> soldier (you): rocket
launcher | engineer: fortress | medic: shield wall`. A branching super crate makes the next
class's result (soldier > engineer > medic > soldier). A marker over each head shows the class:
soldier ^, engineer square, medic +. The match log tags each result with the kicker's class.

Things to watch: does choosing who kicks become a decision, or a chore? Can you read the
enemy's threat from who's standing at their pad? Does a team that loses its engineer feel
locked out of half the recipes?

- **Guns** go to whoever kicked the pad open, and are lost when they're knocked out or run
  dry. Shotgun: 5 pellets, short range, 12 shells. Rapid: 6 shots/s, 60 rounds. Rocket: slow
  14 m/s rocket, blast 2.5 m, 6 rockets. **SUPER**: 1.5× damage, twice the ammo.
- **Tank**: the kicker drives it for 45 s, or until its 300 hp is gone (SUPER: 600). It moves at
  0.8×, fires a rocket cannon, can't carry, and soaks every hit. The driver comes out unhurt.
- **Turret**: 40 s, shoots the nearest enemy (SUPER: 80 s, twice as fast). **Shield wall**: a
  4 m wall 4.6 m in front of your pad. It blocks enemy shots and players, with 250 hp, for 40 s
  (SUPER: double). **Fortress**: a turret and a wall, each lasting 60 s.
- **Rockets** blow loose crates about. On an enemy pad they follow rule 6: split knocks the
  top crate off each cell in the blast, spill scatters the whole pad, steal and lock do nothing.
  Against a build they do their damage to the build.

## What to try

1. **Recipe shape (key 2).** Play a match by shape, then one by count only. Does the shape
   puzzle add skill, or just friction? Under count only, 4 crates always make 2 players, so a
   turret or rocket costs 5 or more.
2. **Readout (key 3).** Play with the readout secret, then shown. Do you learn the shapes
   from the recipe line in the rules box, or do you want the pad to tell you? Can you read
   what the enemy is about to make (`red pad:` line)?
3. **What stacks build (key 1).** Weapons make the kicker a threat, base upgrades protect the
   pad, and mix is closest to Team Buddies. Watch whether anyone ever builds past players.
4. **Build time (key 5).** Instant vs 1 s and 2 s per crate. With a timer, the all-8 tank takes
   8 or 16 s on the pad. Is defending a build fun, or does it just mean big recipes never
   finish? Try shooting red's build (watch the red hp bar over their pad).
5. **Vulnerability (key 6).** Try each with the readout shown. In split/spill, shoot a red
   stack from range. In lock, watch the HUD's `locked into pads (asleep)` count, which is what
   the netcode research asks for. Does lock kill the stealing game?
6. **Classes (key 1 → "by kicker's class", C to switch).** Build 4 flat, then press C
   before you kick to see the result change. Compare it with a fixed book: is "the same stack
   makes different things" more fun, or just harder to read?
7. **Super crates (key 4).** One purple crate drops in the middle every 25 s (first at 15 s).
   Is it worth a fight? Does upgrade or branch read better?

**Shortcuts for trying recipes**: **T** drops a crate in front of you and **Y** drops a
super crate, so you can build any shape fast. "blue bots build: off" in the panel leaves your
pad to you.

## What it answers

- The match-end line (also printed with **P**) logs the rules, kills, **crates knocked off
  pads**, **builds broken**, and everything each team made. That shows which recipes ever get
  built.
- **Enter** records the rules you're playing as your pick. **P** prints your picks.

## Notes on the model

- Bots stand in for players. They build players first, then the big recipes. They open
  small recipes only when the pad has sat unchanged for 7 s. They walk 8 m further for a super
  crate, shoot your pad when hits do something (split, spill, or a build), and drive tanks at
  your pad.
- Bots grab instantly because they stand in for the server; your grabs keep the 120 ms round
  trip. No avian3d: the same hand-rolled physics as core feel.
- The win condition is still crates on the pad plus crates spent there. It isn't settled here
  (see the win-conditions research, [#3](https://github.com/ALLiDoizCode/big_back/issues/3)).
- Smoke runs: `CRATE_RULES=<6 digits>` sets each rule by option index (build time 0/1/2 = instant / 1 s / 2 s),
  `CRATE_SEAT=bot`, `CRATE_ARENA=1`, `CRATE_SPEED=4`, `CRATE_EXIT_AFTER=<s>`, `CRATE_SCREENSHOT=<png>`.
