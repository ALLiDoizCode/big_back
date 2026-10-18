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
- Score = crates resting on your pad (a placeholder turret counts 4). Matches last 120 s.
- **Guided:** press **N**, then **1–4** for 2v2 / 3v3 / 4v4 / 6v6 (each switch restarts the
  match). **Enter** keeps the one you're playing, and **P** prints settings, the verdict and
  the match results. "team size" in the panel goes from 1v1 to 8v8.
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
- The placeholder 2×2 turret is *not* a decision (#17).
