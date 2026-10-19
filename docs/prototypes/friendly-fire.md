# PROTOTYPE sheet: friendly fire and randomness budget (issue #35)

Throwaway. Lives on `prototype/friendly-fire` only, branched from `prototype/match-pacing`
(the 5v5 lanes + jungle map from ADR 0006, the Final kit from #22, ADR 0013's knockout with
the comeback set, ADR 0015's 8 min timer, ADR 0012's open seats).

```
cargo run --example friendly_fire
```

**Question:** the game should be accessible but skill-expressive. Which friendly fire applies:
bullets, blasts (rockets, grenades, launcher grenades, tank shells, barrage shells), thrown
crates, and knocks to your own pad? How much randomness: where crates spawn, what river
objectives drop, when megacrates come, and weapon spread? For each, what is on, what is fixed
or marked ahead, and what is left random?

**Resolved when** you've played at least two options of each switch and ruled on each one
(press **Enter** on the rules you like; **P** prints your picks).

## Verdict

_Waiting on a played verdict._

## The rule switch (keys 1-9, all live)

Every switch starts where the #34 prototype stood (**bold**).

| Key | Switch | Options |
|---|---|---|
| 1 | friendly fire: **bullets** | **off: they pass through allies**, on: they hurt allies |
| 2 | friendly fire: **blasts** | **off: they spare you and allies**, your own blasts hurt you, on: they hurt you and allies |
| 3 | friendly fire: **thrown crates** | **off: they pass allies**, on: they hit allies |
| 4 | friendly fire: **own pad** | off: your team's crates can't knock your pad, **on: any crate knocks any pad crate** |
| 5 | **crate spawns** | **random spot, unmarked**, random spot marked 3 s ahead, fixed spots in turn (marked) |
| 6 | **river drops** | **fixed: a super crate, back in 60 s**, random: a roll, scattered, back in 40-80 s |
| 7 | **megacrate timing** | **fixed: every 45 s, countdown shown**, random 25-65 s with no warning, random and marked 5 s ahead |
| 8 | **weapon spread** | **none: shots go where you aim**, random cone, bloom |
| 9 | **preset** for 1-8 | **as switched**, forgiving, punishing, the #34 prototype |

- **Forgiving** (key 9 once): no friendly fire anywhere, fixed spawns marked ahead, fixed
  river drops, fixed megacrates with a countdown, no spread.
- **Punishing** (key 9 twice): every kind of friendly fire on, unmarked random spawns, random
  river drops, unwarned random megacrates, a random cone.
- **The HUD** shows what friendly fire did this match, per team: damage to allies and to
  yourself, players knocked out, thrown-crate hits on allies, carriers made to drop their
  crate, and own-pad knocks. It also shows the next megacrate (countdown, `?`, or where a
  marked one lands) and each river objective's time back (`?` when random).
- **Marked drops** show on the floor as a ring that closes as the drop comes, with a beam
  (white for a crate, gold for a megacrate).
- **Panel numbers:** `ff: damage` (50% of a hit), `spread: cone` (5°), `spread: rapid cone`
  (8°), how far ahead drops are marked, the random megacrate and river ranges, and two bot
  switches: `ff: bots mind allies` and `bots throw onto the pad`.

**Calls the issue didn't make, made here so they can be played (say if any is wrong):**

1. **Friendly fire does half a hit** (`ff: damage`). It hurts and still knocks a carried
   crate loose (#9's "a hit drops the crate"), but rarely knocks a player out alone.
2. **Friendly fire hurts players only.** Your own base, walls, builds and pad crates never
   take your team's shots or blasts. Mines and the mech's stomp stay enemy-only, and heal shots
   and shield bursts never hurt.
3. **A team knockout counts for nobody.** It's not a kill and not a fight (#34's pacing).
4. **Own pad off** means your team's sliding crate stops dead against your own pad's resting
   crates. An enemy's crate still knocks them.
5. **The random river roll:** 40% a super crate, 30% 3 crates, 20% a super crate and 2 crates,
   10% a megacrate, thrown out in random directions.
6. **A marked megacrate is rolled 5 s sooner,** so it lands as often as an unmarked one.
7. **Bloom:** the first shot is exact, and each shot held after it opens the cone by a third.
   The cone closes once you stop firing for 0.35 s. The shotgun keeps its own fan (a random
   cone scatters its pellets in it), and the tank cannon never spreads.
8. **Bots mind their allies:** with friendly fire on, a bot holds a shot with an ally in its
   line, and a blast that would land on an ally (or on itself). `ff: bots mind allies` turns
   this off.
9. **Bots can throw:** before this, bots walked every crate onto the pad and set it down, so
   thrown crates and own-pad knocks never happened in a bot match. `bots throw onto the pad`
   has them jump-throw from 3-7 m out, aiming short so the crate slides onto its cell. The bot
   matches below have it on.

## What to try

1. **Friendly fire on bullets and blasts** (keys 1 and 2). Fight next to a teammate in a lane,
   and near your own pad while bots stack it. Do you start caring where they stand, or does it
   just punish a crowded fight? Try `your own blasts hurt you` on its own: rockets at close
   range.
2. **Thrown crates and your own pad** (keys 3 and 4). Jump-throw onto your pad while a
   teammate stands on it, and next to crates already there. Is knocking your own stack a skill
   to learn or a trap?
3. **Crate spawns** (key 5). Play random unmarked, then marked, then fixed. Can you plan a
   route around what's marked? Does fixed make the middle feel solved?
4. **River drops and megacrates** (keys 6 and 7). Does a random roll make a river fight feel
   like a gamble worth taking, or a coin toss? Does a random megacrate with no warning feel
   unfair at the buzzer?
5. **Weapon spread** (key 8). None, a random cone, then bloom (tap fire stays exact). Which
   one rewards aim?
6. **The presets** (key 9): forgiving, then punishing. Then build your own mix and press
   **Enter** on it.

## What the bot matches showed

_Running._

## Notes on the model

- Smoke runs: `CRATE_RULES=<9 digits>` sets keys 1-9 (the #34 prototype is `000100000`; a
  9th digit of 1-3 applies a preset), `CRATE_BOTS_MIND=0` (bots fire through allies),
  `CRATE_BOTS_THROW=1` (bots jump-throw onto the pad), `CRATE_MATCHES=<n>` exits after n
  matches, `CRATE_SPEED` goes up to 10, and `CRATE_SEED` varies the randomness. Each match
  prints an `ff ...` line with every number below, after #15's `result ...` and #34's
  `pacing ...` lines.
- **Luck** is the random goods a team got: normal crates that spawned on its half, and crates
  (a megacrate at its worth) from the river objectives it cleared. "Winner luckier" counts
  the matches where the winner got more of them.
