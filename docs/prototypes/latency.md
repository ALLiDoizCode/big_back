# PROTOTYPE sheet: grabs, steals and throws at 165 ms (issue #36)

Throwaway. Lives on `prototype/latency` only, branched from `prototype/friendly-fire` (the
5v5 lanes + jungle map from ADR 0006, the Final kit from #22, ADR 0013's knockout with the
comeback set, ADR 0015's 8 min timer, ADR 0012's open seats, #35's verdict).

```
cargo run --example latency
```

**Question:** ADR 0012 puts most matches across regions, at about 95 ms for the furthest
player at the busiest hours and 165 ms at the quietest. The netcode research makes crate grabs
and steals server decisions. Do server-decided grabs, steals, kick-opens and jump-throws still
feel acceptable at 95 ms and 165 ms? Or does the spec need client prediction (with a snap-back
on a lost grab), a wider grab reach, or another fix? Play each against no latency.

**Resolved when** you've played grabs, steals, kick-opens and jump-throws at 95 ms, 165 ms
and 0 ms, tried each fix, and ruled on what the spec needs (press **Enter** on the setup you
like; **P** prints your picks and the grab stats for every setup you played).

## Verdict

**Picked 2026-10-19: server grabs with rewind.** The human, after playing it: "server grabs
with rewind feels good".

- **Grabs and steals are server decisions** (key 2, server decides). Your hand reaches out at
  once, and the crate comes into your hands when the answer arrives, one round trip later.
  There is no predicted grab, so a lost grab never snaps back out of your hands.
- **The server judges a grab on what you saw** (key 5, rewind on). It looks back at most
  200 ms. If the crate was loose and in reach then, you get it, out of a rival's hands if
  they took it since, or loose again if it locked in since.
- **Grab reach stays 1.2 m** (key 4 unchanged).
- **Rejected:** predicted grabs with a snap-back, and judging grabs only on where things are
  now (the spec before this).
- **Not ruled:** throws, set-downs and kicks (key 3) were not part of the verdict, nor was a
  wider reach. The prototype's defaults for them (server decides, 1.2 m) stand until they're
  played on their own.

## How the latency is faked

The sim is the server. You are the only player with a ping: the bots run on the server, with
no latency. "Ping" means the round trip.

- **Your movement is predicted:** it shows at once, and the server agrees with it.
- **Everything else shows late:** bots, crates, their shots, and pad kick rings. You see them
  one round trip plus 40 ms of interpolation in the past (205 ms at 165 ms ping). The 40 ms
  is the netcode research's figure for a 30 Hz send rate.
- **A grab** goes for the nearest crate you *see* loose in reach. The server judges it when it
  arrives (the server is at your tick then), against where things really are. The answer
  reaches you one round trip after you pressed. So a grab is lost when the crate you saw was
  taken, locked in, or slid off in the 205 ms you couldn't see.
- **A crate the server removed** (kicked open, destroyed) stays in your view until the news
  arrives (one round trip).
- **O** draws the server's truth in orange wherever it differs from what you see.

## The latency switch (keys 1-6, all live)

| Key | Switch | Options |
|---|---|---|
| 1 | **ping** | 0 ms (no latency), 95 ms (busiest hours), **165 ms (quietest hours)**, 250 ms |
| 2 | **grabs and steals** | **server decides:** your hand reaches out, and the crate comes when the answer does (1 round trip). **Predicted:** in your hands at once, and it snaps back if the server says no |
| 3 | **throws, set-downs, kicks** | **server decides:** a thrown or set-down crate stays in your hands, and the kick ring fills, a round trip (+ interpolation) late. **Predicted:** shown at once |
| 4 | **grab reach** | **1.2 m (now)**, 1.7 m, 2.2 m. It's a rule, so the bots get it too |
| 5 | **server rewind** | **off:** judged on where things are now. **On:** judged on what you saw, looking back at most 200 ms (lightyear's demo caps the rewind) |
| 6 | **preset** for 2-5 | as switched; the spec now (server decides everything); the research's plan (server grabs, predicted throws); predict everything; server grabs judged on what you saw, with a wider reach |

- **L** swaps between the ping and 0 ms, to play the same thing against no latency.
- **Drills** set up the contested cases on demand:
  - **Z, scramble:** a crate lands just in front of you, and a red rival rushes it from
    2.5 m out (`drill: rival starts`).
  - **X, steal vs lock-in:** you're put beside red's pad, with a crate on it that locks in
    0.8 s after it lands (`drill: locks in after`).
  - **Shift+X, steal vs a defender:** the same, but a red defender rushes it instead.
  - **N, slider:** a crate slides past in front of you (`drill: slider speed`).
- **The HUD** shows the ping and how late you see things, your grab waiting for its answer,
  and per setup: grabs and steals tried and lost, why each was lost (taken first, moved off,
  locked in, gone), rewind saves (and crates pulled from someone's hands), snap-backs,
  throws and set-downs, and kick-opens.

**Calls the issue didn't make, made here so they can be played (say if any is wrong):**

1. **While a grab's answer is on its way, E and right click do nothing.** The alternative
   (queueing a throw of a crate you might not get) needs its own rollback.
2. **A lost grab says why** ("STEAL LOST: an enemy took it first (you saw it 205 ms late)").
3. **Rewind takes the crate from whoever grabbed it since.** If the server rewinds to what you
   saw and the crate was loose then, you get it: out of a rival's hands, or loose again if it
   locked in since. That is the fair-to-you half of rewind; the other half is the rival
   seeing it leave their hands.
4. **A wider reach is for everyone,** bots included.
5. **Your own shots are predicted, others' show late.** This is what the netcode research
   assumes. Shooting at bots you see 205 ms late is #2's projectile question, not this one.
   Grenades, blasts, walls and vehicles show without delay.

## What to try

1. **Plain grabs and steals** at 165 ms, server deciding. Haul a few crates home, then press
   **L** and do the same at 0 ms. Does the reach feel like a move, or like lag?
2. **The drills** (Z, X, Shift+X, N) at 0, 95 and 165 ms. How often do you lose, and does a
   loss feel fair? Turn on **O** to see why.
3. **Predicted grabs** (key 2). A lost grab now shows in your hands and snaps back. Is that
   worse than waiting?
4. **Rewind** (key 5) on the same drills. Do you win back what you lost? Watch the rival: the
   crate leaves its hands.
5. **Wider reach** (key 4) on the slider (N).
6. **Jump-throws and kick-opens** (key 3). Jump-throw onto your pad at 165 ms: the crate
   hangs in your hands for 205 ms, then flies from where you threw it. Hold-to-choose a
   recipe on your pad: the ring fills 165 ms late, so you may hold past the one you wanted.
   Then try them predicted.

## What the drill bench showed

No human, so these are a hint for what to play, not a verdict. Your seat ran each drill
300 times headless (`CRATE_BENCH`): it presses grab 175-325 ms after the crate shows up in
its view. The rival starts 0.3-2.3 m out (set so the scramble is close to even at 0 ms), the
stolen crate locks in 0.5 s after it lands, and the slider passes at 5 m/s. **Rivals are bots
on the server with no latency, so this is the worst case:** a human rival would also be late.
Prediction (key 2 and 3) changes only what you see, so it doesn't change these rates.

**Won**, as a share of trials (300 each; treat ±6 points as noise):

| Setup | Rules | Scramble (Z) | Steal vs lock-in (X) | Steal vs defender (Shift+X) | Slider, 10 m/s (N) |
|---|---|---|---|---|---|
| 0 ms | `000000` | 57% | 98% | 65% | 100% |
| 95 ms | `100000` | 24% | 100% | 51% | 32% |
| 165 ms | `200000` | **8%** | **59%** | **36%** | **0%** |
| 250 ms | `300000` | 1% | 14% | 9% | 0% |
| 165 ms, reach 1.7 m | `200100` | 5% | 58% | 62% | 99% |
| 165 ms, rewind | `200010` | **44%** | **99%** | **65%** | **77%** |
| 250 ms, rewind (capped at 200 ms) | `300010` | 32% | 97% | 59% | 36% |

- **Uncontested grabs are never lost.** Your seat's bot hauled a whole match through the laggy
  path (`CRATE_LAG_BOT=1`): 0 lost out of 834 grabs at 0, 165 and 250 ms over 24 matches. A
  crate at rest is where you saw it. Lost grabs only happen in contests and on moving crates.
- **Contests are where 165 ms hurts.** You see the crate 205 ms late and the rival doesn't, so
  a near-even scramble at 0 ms (57%) is almost always lost at 165 ms (8%). Steals against a
  defender go from 65% to 36%, and a steal with half a second to lock-in from 98% to 59%.
- **Rewind wins most of it back**, within its cap: 44% in the scramble, 99% against lock-in,
  65% against a defender. In the scramble, 108 of 300 wins came out of the rival's hands. At
  250 ms the 200 ms cap can't look back far enough, so some losses come back.
- **A wider reach fixes moving crates, not races.** The slider at 165 ms went from 0% to 99%
  with 1.7 m, but the scramble didn't move, because the rival's reach grows too.
- **What rewind can't fix:** you still *see* everything late, so a race decided by who sees
  the crate first (the scramble's "missed" share) stays lost.

## Notes on the model

- **Smoke runs:** `CRATE_RULES=<6 digits>` sets keys 1-6 (e.g. `200010` is 165 ms with
  rewind; a 6th digit of 1-4 applies a preset). `CRATE_TRUTH=1` starts with O on.
- **The drill bench:** `CRATE_HEADLESS=1 CRATE_BENCH=z|x|X|n CRATE_TRIALS=300` prints one
  `bench ...` line and exits. `CRATE_REACTION=<ms>` sets the middle of the reaction spread,
  and `CRATE_RIVAL`, `CRATE_LOCK` and `CRATE_SLIDE` set the drill numbers (the table used
  `CRATE_RIVAL=1.3 CRATE_LOCK=0.5 CRATE_SLIDE=10`). A scramble or defender trial with no red
  player free to rush doesn't count.
- **Whole matches:** `CRATE_HEADLESS=1 CRATE_LAG_BOT=1 CRATE_MATCHES=<n>` lets your seat's bot
  play through the laggy path. Each match prints a `net ...` line with its grab stats.
- **What isn't faked:** jitter and packet loss (the ping is steady); latency for the bots; the
  carry slowdown starts on the server's tick, not when you learn you're carrying (a real
  client would mispredict its speed for a round trip); other players' shots stop where they
  hit, a delay early.
