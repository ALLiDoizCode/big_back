# PROTOTYPE sheet: lanes + jungle arena map

Throwaway. Lives on `prototype/arena-map` only (branched from `prototype/crate-grammar`, #17).
No issue yet; it came up while playing #17.

```
cargo run --example arena_map
```

**Question:** should the arena be bigger and laid out more like a League of Legends map: bases at
each end, lanes and jungle entrances that pull players into fights, and PvE camps that drop
heals and shields?

## Verdict

**Picked 2026-10-18: 5v5 on the lanes + jungle map.** The human: "lets go with 5v5 and the league
of legends map". This replaces two earlier decisions:

- **Team size 3v3** (#10's "3v3 for now", ADR-0005's consequences) becomes **5v5**. Teams
  still start with one player and the rest join from the pad (ADR-0005 otherwise stands).
- **The open 36 × 22 m arena** becomes **the 90 × 50 m map**: bases, three lanes, four walled
  jungle quadrants and a river, with PvE heal, shield and river-objective camps.

Not decided yet: exact map size, wall layout, camp numbers and drops. The crate grammar (#17)
continues on this map.

**Team size is 5v5 by default**, as in League of Legends (10 players). "team size" in the panel still
goes from 1 to 8. Teams still start with one player and the rest join from the pad (ADR-0005).

Everything from the crate-grammar prototype still works here (keys 1–6, C for class, T and Y
to drop crates). The **map** setting in the panel switches back to the open #10 arena
(36 × 22 m) to compare. Changing it restarts the match.

## The map

**90 × 50 m** (the open arena is 36 × 22 m), mirrored for both teams:

- **Bases** at each end (tinted blue and red), with the pad 3.6 m from the end wall.
- **Three lanes**: top, mid and bottom (the lighter strips).
- **Four jungle quadrants** (dark green), walled off from the lanes. Each has three
  entrances: one from mid, one from its side lane and one from its base. It also has pillars
  inside, and is open to the river.
- **The river** (blue) runs north–south through the middle.
- **Walls are tall**: players, crates and shots don't pass. Thrown crates bounce off them.
- A **minimap** sits bottom right: yellow dots are camps, and the big dot is you.

## PvE camps

| Camp | Where | Monster | Drops | Respawn |
|---|---|---|---|---|
| Heal camp | north jungle, one per side | 200 hp, 8 dmg/s | a heal: +60 hp | 40 s |
| Shield camp | south jungle, one per side | 200 hp, 8 dmg/s | a shield: absorbs 60 (max 100) | 40 s |
| River objective | river, north and south | 320 hp, 12 dmg/s | 3 crates, or a super crate when super crates are on | 60 s |

- A monster fights whoever comes within 3.5 m or shoots it. It chases within a 9 m leash
  (the yellow ring), then walks home and heals. Anyone can take a drop by walking over it.
  Drops vanish after 30 s.
- Shields show as a blue bar over the head and soak damage before hp. The HUD's `camps:`
  line shows each camp as up, or with its respawn timer.
- Deaths to monsters show in the match log as `lost to PvE`.

## What to try

1. **Play a match on each map** (panel → map). Does the bigger map feel like it has places
   worth fighting over, or is it just a longer walk with a crate?
2. **Hauling**: on this map a crate from mid takes about 12 s per round trip, against about
   5 s in the open arena. Is the haul tense or tedious?
3. **Camps**: clear one when you're hurt, and fight over the river objective. Are heals and
   shields worth the detour? Should camps drop something else, such as crates, guns or
   recipe pieces?
4. **Entrances**: wait at a jungle entrance for an enemy carrier. Do the walls make ambushes,
   or just make it slower to get around?

## What the bot matches showed (5 min, bots on both teams)

| | open arena, 3v3 | lanes + jungle, 3v3 | lanes + jungle, 5v5 |
|---|---|---|---|
| Player kills per match | about 45–50 | about 7–9 | about 22–26 |
| Deaths to PvE | — | about 2–4 | about 3–7 |
| Crates scored | about 100–120 | about 50 | about 70 |

**At 3v3 the map is too empty for PvP.** It has about 750 m² per player, where the #10
verdict tuned the arena to 130 m². That's why 5v5 is the default: it has 450 m² per player
and about three times the fights, though that's still about half the open arena's rate. Bots only fight what they can see within 10 m, so this
undercounts how often humans who go looking for fights would meet. The gap is still large.
Ways to close it, if the layout is otherwise liked:

- a smaller version of this layout (about 60 × 34 m);
- bigger teams: 5v5 is now the default here. If it's picked, it reopens the #10 team-size
  verdict (3v3, ADR-0005's "3 for now");
- more pull to the middle: all new crates already land in the river and mid lane here, but
  the river objectives could drop more, or the pads could move closer.

## Notes on the model

- Bots find their way around walls on a 2.5 m grid. Each team's 3rd player jungles: it
  clears camps, river objectives first. Other bots go to a camp when below 60 hp and pick up
  drops within 12 m.
- A bot carrying a crate with nobody free to kick sets the crate down beside the pad and kicks
  it itself. On the big map, bots also wait longer (25 s) before opening small recipes.
- There are no bushes or vision rules, and no minions or towers: only the layout and the
  camps are being tested.
- Smoke runs use the same variables as crate_grammar, plus `CRATE_MAP=open`.
