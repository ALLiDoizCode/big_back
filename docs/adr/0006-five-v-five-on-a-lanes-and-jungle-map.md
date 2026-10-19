# 5v5 on a lanes + jungle map

Matches are **5v5** on a **90 × 50 m map laid out like League of Legends**: a base at each
end, top, mid and bottom lanes, four walled jungle quadrants with entrances, and a river
through the middle. We chose it in play ([arena-map sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/arena-map/docs/prototypes/arena-map.md#verdict)),
which came up while playing #17. The human's verdict: "lets go with 5v5 and the league of
legends map".

- PvE camps sit in the jungle: a heal camp and a shield camp per side, and a river objective
  north and south that drops crates.
- The rejected option was the open 36 × 22 m arena. At 3v3 the big map was too empty: bot
  matches had about 8 kills, against about 24 at 5v5.

This supersedes the team-size consequence of [0005](0005-players-join-from-the-pad.md)
("3 for now, in a 36 × 22 m arena"). The rest of 0005 stands: teams start with one player
and join the rest from the pad.

## Consequences

- **Hauls are long:** about 12 s per round trip from mid, against about 5 s in the open
  arena. Crates spawn in the river and mid lane to pull fights to the middle.
- **Open:** exact map size, wall layout, and camp numbers and drops. A smaller version of the
  layout (about 60 × 34 m) is the fallback if fights stay too rare.
- **Settled by [0020](0020-one-big-lanes-and-jungle-map-at-launch.md):** the map size (120 × 72 m,
  League of Legends scale), base areas 20 m deep and walled, two camps per jungle quadrant,
  and mirror symmetry. The smaller version isn't shipped. Wall and camp placement stays tuning.
