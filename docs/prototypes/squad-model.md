# PROTOTYPE sheet: squad model (issue #10)

Throwaway. Lives on `prototype/squad-model` only (branched from `prototype/core-feel`).

```
cargo run --example squad_model
```

**Question:** which squad model makes the most fun and fair competitive game: one character
per player, a hero plus a partner, a leader with 2–3 AI Buddies of their own, or no squad
with neutral haulers owned by the team? The pick sets the range of team sizes.

## Verdict on the squad model

**Picked 2026-10-18: one character per player, and no AI units at all.** The human's words:
"I don't want the build buddy I just want other players." That rules out three things:

- AI Buddies you own, whether you start with them or build them from crate stacks;
- the hero + partner model;
- team-owned AI haulers.

Teams are made only of players. Earlier versions of this branch built the AI variants: squad
size, buddy source, default jobs, orders, a readability test and a fairness probe. They were
played, but no comparison was settled before this pick. They're in the history at `f13ffe2`
and are now removed from the prototype.

Knock-on decisions:

- **Bots are still needed**, but only as stand-ins for missing players (backfill at low
  player counts), never as units a player owns or commands.
- **Readability** is now just team and player: body colour for team, and a band for each
  player.
- **Netcode:** every character is a player and predicts itself, so the budget is the number
  of players (~32 networked characters at most).
- **#17** (what stacks build) should not assume stacks build buddies.

## Still open: team size

It's resolved when you pick the team-size range.

- You are blue player 1. The other players are bots standing in for people. The core feel is
  fixed at the #9 baseline.
- **Teams start with one player each.** Others join from the pad, Team Buddies style:
  - **The pad is 4 cells (2 × 2), at most 2 high**, so 8 crates fill it. Crates you place or
    curl onto it sit in a cell. A third layer slides off.
  - **Kick it open.** Hold **F** beside your pad for 1.5 s; a ring shows the progress. The
    pad's outline turns **gold** when its shape makes something, and the HUD says what:
    `your pad: 2 stacked -> 1 new player`. Opening uses up the crates, but they still count
    for the score.
  - **Recipes**, by shape as in Team Buddies. The results here are placeholders; what
    shapes really build is #17:

    | Shape | Team Buddies | Here |
    |---|---|---|
    | 2 stacked | a Buddy | **1 new player** |
    | 2 wide × 2 high | specialist Buddy | **2 new players** |
    | 4 flat | heavy weapon | turret (40 s) |
    | all 8 | vehicle | turret (40 s) |
    | 1 crate, 2 side by side | weapon | full health |

    Other arrangements (3 crates, diagonals) are "no recipe yet": add crates. A shape that
    brings players won't open while your team is full. A pad with a crate out of line can
    only be broken.
  - **Knocked out = out.** A player who joined from the pad and then dies is gone until the
    team opens another player shape. Only each team's first player (you, and red's first)
    respawns after 3 s.
  - **Everyone builds**: bots on both teams build toward a shape (players while there's
    room, else a turret), hold their crate while the pad is ready, and kick it open. Turn
    "blue bots build" off to build and open blue's pad alone. Each opening names who
    kicked it, e.g. `BLUE pad kicked open by YOU: 2 stacked -> 1 new player (2/3)`.
  - Enemies can steal crates off your pad, which breaks your shape.
- **Which size you're in:** the box top-left always says `YOU ARE PLAYING 3v3` and lists
  the sizes, with the one you're playing marked `>> ... << PLAYING`.
- **Switching:** press **1** = 2v2, **2** = 3v3, **3** = 4v4, **4** = 6v6, at any time. The
  match restarts at that size. The game starts at 3v3.
- **Picking:** press **Enter** to pick the size you're playing. You can pick more than one,
  for example the smallest and largest sizes you'd queue. **P** prints your picks and the
  match results. "team size" in the panel also reaches 1v1, 5v5, 7v7 and 8v8.
- Things to watch: is there enough to do per player, how long the haul from mid-field to the
  pad feels, and whether a fight is still readable.
- **The arena grows with team size**, keeping the same floor space per player and the 3v3
  shape. At the default 130 m² per player:

  | Team size | 1v1 | 2v2 | 3v3 | 4v4 | 6v6 | 8v8 |
  |---|---|---|---|---|---|---|
  | Arena (m) | 21 × 13 | 29 × 18 | 36 × 22 | 41 × 25 | 51 × 31 | 58 × 36 |

  Pads stay 3.6 m in from each end, and the starting crates spread across the middle. Bigger
  teams mean longer hauls. "space per player" (60–250 m²) in the panel tunes the space: try
  less if 6v6 feels empty.
- Other settings:
  - **online delay**: shows other players in the past, the way interpolation does online.
  - **your seat: bot**: lets you watch bot matches.
  - **time scale** and **camera: whole arena**: for watching.

## Notes on the model

- Bots grab instantly, because they stand in for the server. Your grabs keep the 120 ms
  round trip.
- No avian3d: the same hand-rolled physics as core feel.
- The turret and health results are placeholders, not decisions (#17).
