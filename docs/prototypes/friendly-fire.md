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

Bots on both sides, the 5v5 lanes + jungle map, ADR 0013's rules, an 8 min timer, 1 open
seat each. Every row has bots jump-throwing onto their pad, and bots minding allies unless
it says careless. 32 matches per row (4 seeds × 8), run headless (`CRATE_HEADLESS=1`).
Figures are per match unless they're a count over the 32. **Treat a difference of 4 in 32
as noise.**

**Friendly fire** (randomness as in #34):

| Row | Rules | Score (both) | Kills | Ally damage | Team knockouts (of 32) | Shots held | Own-pad knocks | Knocks stopped |
|---|---|---|---|---|---|---|---|---|
| no friendly fire | `000000000` | 53 | 18.4 | 0 | 0 | 0 | 0 | 16.6 |
| #34 (own pad on) | `000100000` | 54 | 19.0 | 0 | 0 | 0 | 17.0 | 0 |
| bullets | `100100000` | 52 | 18.6 | 10 hp | 0 | 15 | 16.1 | 0 |
| bullets, careless | `100100000` | 54 | 18.8 | 39 hp | 2 | 0 | 17.1 | 0 |
| own blasts hurt you | `010100000` | 54 | 19.3 | 1 hp to self | 0 | 0 | 16.8 | 0 |
| blasts hurt all | `020100000` | 54 | 19.2 | 1 hp | 0 | 0 | 16.6 | 0 |
| blasts, careless | `020100000` | 54 | 19.2 | 1 hp | 0 | 0 | 16.6 | 0 |
| thrown crates | `001100000` | 54 | 19.0 | 1 hp | 0 (1 crate hit) | 0 | 16.8 | 0 |

**Randomness** (friendly fire as in #34):

| Row | Rules | Score (both) | Kills | Lead changes | Winner got more luck | Luck gap | River crates | Megacrates | Gap between megacrates | Bot hit rate |
|---|---|---|---|---|---|---|---|---|---|---|
| #34 | `000100000` | 54 | 19.0 | 3.9 | 11 of 28 | 4.9 | 0.5 | 5.1 | 95 s | 0.29 |
| spawns marked | `000110000` | 52 | 19.7 | 3.8 | 18 of 30 | 4.8 | 0.5 | 4.9 | 97 s | 0.32 |
| spawns fixed | `000120000` | 57 | 22.4 | 4.8 | 17 of 29 | 3.2 | 0.3 | 5.3 | 93 s | 0.33 |
| river random | `000101000` | 53 | 19.7 | 4.1 | 15 of 30 | 5.2 | 1.4 | 4.9 | 98 s | 0.30 |
| megacrates random | `000100100` | 54 | 19.9 | 4.0 | 16 of 28 | 3.7 | 0.7 | 5.2 | 95 s | 0.30 |
| megacrates marked | `000100200` | 52 | 18.6 | 3.8 | 15 of 30 | 5.2 | 0.6 | 4.9 | 97 s | 0.30 |
| spread: random cone | `000100010` | 55 | 19.3 | 3.6 | 13 of 30 | 5.1 | 0.8 | 5.1 | 94 s | 0.28 |
| spread: bloom | `000100020` | 53 | 20.3 | 4.2 | 12 of 26 | 5.4 | 0.7 | 5.1 | 96 s | 0.30 |
| **forgiving** preset | `000000001` | 57 | 21.5 | 4.0 | 20 of 28 | 3.4 | 0.3 | 5.1 | 94 s | 0.33 |
| **punishing** preset | `000000002` | 55 | 19.7 | 2.9 | 14 of 28 | 5.0 | 2.8 | 5.4 | 93 s | 0.28 |

"Luck gap" is how far apart the two teams' luck was. "Winner got more luck" leaves out ties
and draws.

**What it suggests** (bots, so a hint for what to play, not a verdict):

1. **Bot matches can't judge friendly fire.** Bots field about 1.3 players a team at a time,
   so an ally is rarely in the line. Even careless bots hurt allies for only 39 hp a match on
   bullets, with 2 team knockouts in 32 matches. Blasts did almost nothing (1 hp a match), and
   thrown crates hit an ally once in 32 matches. Minding allies cut bullet damage to a quarter
   (10 hp) and cost 15 held shots a match. Whether friendly fire is skill or punishment is a
   question for humans in a crowded lane or on a crowded pad.
2. **Knocking your own pad costs bots nothing measurable.** A team's own crates hit its own pad
   crates about 17 times a match. With the rule off, the same 17 or so are stopped instead, and
   scores don't move (53 vs 54). Lock-in (8 s) protects most of the stack, so a knock only
   nudges crates that haven't scored yet.
3. **Luck doesn't decide bot matches.** Under every rule, the team that got more of the random
   goods won about half the time (11-18 of about 29). Fixed spawns split them evenly (the luck
   gap falls from about 5 to 3.2) without changing who wins.
4. **Fixed spawns made matches busier:** 22 kills against 19, and 4.8 lead changes against 3.9.
   Both teams went to the same known spots and met there. The forgiving preset (fixed spawns
   and no friendly fire) shows the same. This is the one randomness result above the noise.
5. **Megacrate timing barely matters.** A megacrate lies about 50 s before anyone picks it up,
   and none drops while one is out. So megacrates come about every 95 s whatever the timer
   says, and a random 25-65 s timer only shuffles when the next one starts counting.
6. **Bots rarely clear river objectives** (0.3-0.6 crates a match, 1.4 with the random roll's
   bigger drops). The random river roll needs a human contesting it.
7. **Spread changed little for bots.** They already miss by up to ±4°, and they fire in taps,
   so bloom never opens for them. A random cone cost them 1 point of hit rate (0.29 to 0.28).
   Spread is also a question for a human aiming.

## Notes on the model

- Smoke runs: `CRATE_RULES=<9 digits>` sets keys 1-9 (the #34 prototype is `000100000`; a
  9th digit of 1-3 applies a preset), `CRATE_BOTS_MIND=0` (bots fire through allies),
  `CRATE_BOTS_THROW=1` (bots jump-throw onto the pad), `CRATE_MATCHES=<n>` exits after n
  matches, `CRATE_SPEED` goes up to 10, and `CRATE_SEED` varies the randomness.
  `CRATE_HEADLESS=1` runs with no window (about 0.25 s a match, no GPU). **Run bulk bot
  matches headless:** six windowed copies at once froze the machine by exhausting the
  GPU's 256 MiB BAR1 mapping area (about 65 MiB a window). Each match
  prints an `ff ...` line with every number below, after #15's `result ...` and #34's
  `pacing ...` lines.
- **Luck** is the random goods a team got: normal crates that spawned on its half, and crates
  (a megacrate at its worth) from the river objectives it cleared. "Winner luckier" counts
  the matches where the winner got more of them.
