# One big lanes + jungle map at launch

Season 1 ships **one map**: the lanes + jungle layout from
[0006](0006-five-v-five-on-a-lanes-and-jungle-map.md), at **League of Legends scale,
120 × 72 m**, with **bigger, walled base areas** and **more jungle**. It is **mirror-symmetric**:
red's half is blue's, flipped east-west. Every match is played on it, so the queue has no map
to pick. We chose this in play ([launch-maps sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/launch-maps/docs/prototypes/launch-maps.md#verdict))
for [#42](https://github.com/ALLiDoizCode/big_back/issues/42). The human, after playing the
90 × 50 m map: "the map seems smaller than a League of Legends map and there needs to be more
jungle and the base area should be bigger", then, after playing the big map: "I like the
bigger map". They picked one map and mirror symmetry, and said the big map didn't feel too
quiet.

## The map

- **120 × 72 m**, about 1.9 times 0006's 90 × 50 m: 864 m² of floor per player at 5v5,
  against 450.
- **Base areas 20 m deep** (against about 14), walled off, with exits onto the three lanes and
  a gate into each of the team's two jungle quadrants. The pad and the base sit at the back.
- **Four jungle quadrants of about 35 × 22 m** (against 27 × 13), split into rooms by inner
  walls, each entered from mid lane, its side lane, the river and the base gate.
- **Two camps per quadrant**, a heal and a shield (8 in all, against 4), with 0006's hp,
  damage, leash and drops.
- **A 10 m river** with a river objective north and south.
- **Mirror symmetry.** Each team's heal camp sits in the base-side room of its north
  quadrant and the river-side room of its south one; the shield camps take the other rooms.

Exact wall placement, camp spots and crate spots are tuning, not decisions.

## Considered options

Played in the prototype, against 160 bot matches per map (40 for the big map):

- **0006's 90 × 50 m map alone:** the human found it too small next to League of Legends, with
  too little jungle and too small a base area.
- **Adding the ~60 × 34 m smaller version** (0006's and [0012](0012-one-rated-queue-with-short-starts.md)'s
  fallback): busier, but its camps reach into the narrowed lanes. PvE deaths tripled (about
  12 a match, against 4), so it would need its own camp tuning.
- **Distinct layouts:** *crossroads* (two side lanes, one jungle per side) and *canal* (three
  lanes and three bridges, no jungle). They were the busiest maps in bot play, with the first
  kill at about 20 s and 30-35% comebacks. Each would be a second map to tune bots and the
  comeback set for.
- **Rotational symmetry:** it didn't change which side won, within the noise, on any map.
  Mirror is what was played.
- **How the queue picks** (rotate, random, random with no repeats) was built but is unused
  with one map.

## Consequences

- **Fights are rarer.** In bot matches the big map had about 12 kills a match (against 20 on
  the 90 × 50 m map), the first kill near a minute in, 3.8 fights a minute (against 5.3) and
  lower scores. Bots only fight what they see within 10 m, so this undercounts players who go
  looking for fights, and the human didn't find it too quiet. If it plays too quiet with real
  players, pull the action inward first (crate drops, objectives) rather than shrinking the
  jungle.
- **Comebacks are rarer.** The winner had trailed by 5+ in 8-10% of bot matches, against
  about 20%. Matches were close (95% within 5 points), but because both teams scored little.
  The comeback set ([0013](0013-base-knockouts-with-a-comeback-set.md)) ran unchanged on it.
  [#52](https://github.com/ALLiDoizCode/big_back/issues/52) scaled it to the map (megacrates
  every 35 s, pulled 30 m, mercy at 12), which brought comebacks back to about the old map's
  level (0013, "On the launch map").
- **Hauls are long.** The pad sits at the back of a 20 m base area. Carries average about 7 s
  in bot play, the same as the 90 × 50 m map, because new crates still land in the middle.
- **Map variety doesn't carry the launch.** [0014](0014-one-mode-at-launch-crate-race.md)
  expected it to. With one mode and one map, variety at launch comes from the match itself:
  classes, recipes, camps and the comeback set.
- **One map to tune bots for.** Bots ran on every prototype map with no code changed per map
  (grid navigation, pad-relative guarding and raiding, the map's own camp list), so a later
  map costs its data and whatever players find bots doing wrong on it.
- **There's no smaller map.** 0012's fallback, 3v3 on the ~60 × 34 m map, isn't shipped. 3v3
  on the big map would leave about 1,440 m² per player. If the population ever needs 3v3,
  that needs its own map.
- **0012's map rotation is unused at launch.** A second map, if one is added, goes through
  0012's rule (the server rotates, or each player picks at least 2), and needs its own played
  prototype.
- **Open:** wall and camp placement, crate spots, and camp numbers and drops on the big map.
