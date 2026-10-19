# PROTOTYPE sheet: throws, set-downs and kick-opens at 165 ms (issue #54)

Throwaway. Lives on `prototype/throw-latency` only, branched from `prototype/latency` (#36's
fake latency, on the 5v5 lanes + jungle map with #35's defaults). Grabs and steals are fixed at
ADR 0017: the server decides, judged on what you saw, reach 1.2 m.

```
cargo run --example throw_latency
```

**Question:** which of throws, set-downs and kick-opens does the client predict, and which wait
for the server? ADR 0017 left these three open, along with the 200 ms rewind cap. A throw decides
where a crate lands and what it knocks (ADR 0001), and a kick-open spends a pad's crates (ADR
0007), so a wrong guess is visible.

**Resolved when** you've played throws, set-downs and kick-opens at 165 ms (and 95 ms and 0 ms
to compare), each under at least two of its options, and ruled on each, plus the cap. Press
**Enter** on the setup you like; **P** prints your picks and the stats for every setup you played.

## Verdict

Not played yet.

## What's new since #36

#36's "predicted" showed you the server's own result early, so it could never be wrong. Here a
prediction is a **guess your client works out on the world as you see it**, one round trip plus
40 ms late:

- **A thrown or punted crate** flies against crates and rivals where you see them. A crate it
  strikes isn't guessed at: it moves when the server says.
- **A set-down** stacks on the crates you see at rest in the cell you aimed at. The server stacks
  it on whatever is really in that cell.
- **A kick-open** opens the recipe the ring shows you, on the pad as you see it.

One round trip after the server had it, your client compares the server's news with its guess.
**A guess is wrong** when it's more than 0.3 m from where the server had the crate, or the server
has the crate in someone's hands. A wrong guess is **corrected**: it blends (120 ms, `net:
correction blend` in the panel; 0 snaps) to where the server has it now while it still flies, or
to where you see it once it's at rest. The message says by how much ("THROW CORRECTED: the server
had it 1.2 m from your guess").

With a switch on "server decides", the guess is still worked out but not drawn, so the HUD can
say how often it would have been wrong.

## The switch (keys 1-6, all live)

| Key | Switch | Options |
|---|---|---|
| 1 | **ping** | 0 ms, 95 ms, **165 ms**, 250 ms |
| 2 | **throws and punts** | **server decides:** the crate stays in your hands one round trip (+ interpolation), then flies from where you threw it. **Predicted:** it flies at once, and it's corrected if the server differs |
| 3 | **set-downs** | **server decides:** in your hands one round trip (+ interpolation), then down. **Predicted:** down at once, corrected if the server differs |
| 4 | **kick-opens** | **server decides:** the ring fills and the crates go one round trip late. **Predicted ring:** the ring fills as you hold (it's only your hold time, so it's never wrong), and the crates go when the server says. **Predicted ring and result:** the crates you saw go at once, and they come back (or others go) if the server opened something else. **Predicted ring, judged on what you saw:** the server opens the recipe you saw |
| 5 | **rewind cap** | 100, 150, **200 (ADR 0017)**, 250, 300 ms. How far back the server looks for grabs and for kicks judged on what you saw |
| 6 | **preset** for 2-4 | as switched; the spec now (server decides all three); predict throws and set-downs and the kick ring; predict everything; predict throws and set-downs, kicks judged on what you saw |

- **L** swaps between the ping and 0 ms. **O** draws the server's truth in orange.
- **Drills** set up the cases a guess can get wrong (C, T and the rest are as in #36):
  - **Z, hit a runner:** a crate in your hands, and a red runner crosses 6 m in front of you
    (`drill: runner passes`). Jump-throw at it. Best done in the open.
  - **X, set down vs a steal:** you stand beside your pad holding a crate, facing a crate that a
    red rival rushes to steal from 4 m out (`drill: rival starts`, ±2 m). Set yours on top of it.
  - **Shift+X, set down as a crate lands:** a teammate's crate drops onto that cell within 1.5 s
    instead (`drill: crate lands within`).
  - **N, kick vs a steal:** 4 crates on your pad, and a red rival rushes one from 11 m out
    (`drill: kick rival starts`, ±2 m). Hold F for the 4-crate recipe.
  - **Shift+N, kick as a crate lands:** 3 crates on your pad, and a teammate's 4th drops on.
- **The HUD** shows each switch, and per setup: throws and set-downs (guessed wrong, with the
  average and largest error), kick-opens (not what the ring showed you when you let go; judged
  on what you saw, crates taken back out of hands, fell back to now), and grabs.

**Calls the issue didn't make, made here so they can be played (say if any is wrong):**

1. **Your client sends the throw** (from where, how fast) as it aimed it on what you see. The
   server flies it in the real world. Only the world differs, so that's all a guess gets wrong.
2. **A set-down sends the cell, not the height.** The server stacks the crate on whatever is
   really there.
3. **A set-down stacks only on crates at rest.** In #36 it could stack on a crate falling past
   (it took the highest crate over the cell), which a guess can't match.
4. **Judged on what you saw, the earlier kick wins**, like ADR 0017's grabs: a crate a rival took
   since comes out of their hands and is spent where you saw it. If a crate you saw is gone
   (kicked open, blown up), the server judges on the pad now.
5. **Your kick-open's message arrives with the news**, one round trip after you let go ("YOUR pad
   kicked open: ..."), or at once marked "(predicted)". What it makes is still given to you at
   once on the server (not faked: a real client would see it a round trip later).
6. **The ring only shows recipes on the pad as you see it**, at every setting, and gold outlines
   mark the crates you see it using.

## What to try

1. **Throws** (key 2) at 165 ms, server deciding: jump-throw onto your pad and at red players.
   The crate hangs in your hands for 205 ms. Then predicted: it leaves at once. Try **Z**: a
   predicted hit on the runner is often corrected to a miss. Is a mid-flight correction worse
   than the hang?
2. **Set-downs** (key 3): **X** and **Shift+X**, then a few on your pad while teammates feed it.
   Predicted, a wrong guess jumps a crate up or down a level. Is that worse than waiting?
3. **Kick-opens** (key 4): **N** under each option. Server deciding, the ring is late, so you
   hold longer. Predicted ring, you may get a smaller recipe than the ring showed. Predicted
   result, the crates go and come back. Judged on what you saw, you get the recipe you saw, and
   the rival sees a crate leave their hands. **Shift+N** too.
4. **The cap** (key 5) at 250 ms with kicks judged on what you saw.
5. Each one against **L** (0 ms).

## What the bench showed

No human, so these are a hint for what to play, not a verdict. Rivals and teammates are bots on
the server with no latency, so this is the worst case for you.

**Drills** (`CRATE_BENCH`, 300 trials each). Your seat throws at the runner as it sees it,
leading it; sets down at a random moment of the steal or drop; or holds the kick and lets go 250
± 75 ms after its ring reaches the biggest recipe it sees. A guess for a throw or a set-down
doesn't depend on keys 2 and 3 (they only decide whether it's drawn), so one row per ping.

| Ping | Z: guess wrong (avg error) | Z: hit on the server | X: guess wrong | Shift+X: guess wrong |
|---|---|---|---|---|
| 0 ms | (nothing to guess) | 67% | (nothing to guess) | (nothing to guess) |
| 95 ms | 54% (1.1 m) | 66% | 13% (0.8 m) | 7% (0.8 m) |
| 165 ms | 55% (0.9 m) | 66% | 15% (0.8 m) | 7% (0.9 m) |
| 250 ms | 100% (1.6 m) | 0% | 25% (0.8 m) | 15% (0.8 m) |

Kick vs a steal (N): share that opened something other than what the ring showed when you let
go, share that opened all 4 crates, and how long the seat held.

| Ping | Server decides | Predicted ring | Predicted ring and result | Judged on what you saw |
|---|---|---|---|---|
| 95 ms | 19%, 33% full, 2.22 s | 18%, 35%, 2.19 s | 18%, 35%, 2.19 s | **0%, 52%**, 2.16 s (56 taken back) |
| 165 ms | 21%, 24%, 2.30 s | 27%, 35%, 2.21 s | 27%, 35%, 2.21 s | **0%, 60%**, 2.16 s (74 taken back) |
| 250 ms | 42%, 10%, 2.43 s | 34%, 39%, 2.14 s | 34%, 39%, 2.14 s | 12%, 59%, 2.13 s (71 taken back) |

The cap at 250 ms ping, kicks judged on what you saw: 100 ms 22% wrong (48% full), 150 ms 13%
(53%), 200 ms 12% (59%), 250 ms 3% (68%), 300 ms 0% (69%).

**Whole matches** (`CRATE_LAG_BOT=1 CRATE_BOTS_THROW=1`, 12 matches per ping, seed 7): your
seat's bot throws, sets down and kicks through your client.

| Ping | Throws (guessed wrong) | Set-downs (wrong, avg error) | Kick-opens (not what it saw) |
|---|---|---|---|
| 95 ms | 394 (1) | 151 (18, 0.8 m) | 140 (0) |
| 165 ms | 381 (0) | 144 (15, 0.7 m) | 139 (0) |
| 250 ms | 417 (3) | 147 (11, 0.6 m) | 136 (0) |

- **Throws onto a pad are almost never guessed wrong** (4 of 1,192). A guess goes wrong when a
  throw meets something that moves: a runner (over half the time at 95 and 165 ms, always at
  250 ms) or a crate that moved in the last 205 ms.
- **Set-downs go wrong about 1 in 10 on a busy pad,** and always by one crate: the guess stacks
  on a pad crate the server had already spent or that had been taken (5 of 6 checked were one
  level too high). Contested set-downs (X) go wrong 13-25% of the time.
- **The kick ring is safe to predict**: it's your own hold. Server-decided, you see it late and
  hold about 90 ms longer at 165 ms, which gives a rival more time: 24% full against 35% with
  the ring predicted (10% against 39% at 250 ms).
- **What a kick opens is where 165 ms hurts.** Against a steal, about 1 in 5 kicks opens
  something other than what you saw, whether or not the result is predicted. Judged on what
  you saw, none do, and you get the full recipe twice as often, at the price of the rival seeing
  a crate leave their hands (74 of 300).
- **Shift+N never went wrong** in the bench: a crate falling onto a pad puts it out of line, so
  no kick opens until it lands. Play it anyway.
- **In whole matches, kicks were never contested** (bots don't steal while you kick), so the
  drills are where the difference shows.

## Notes on the model

- **Smoke runs:** `CRATE_RULES=<6 digits>` sets keys 1-6 (e.g. `211320` is 165 ms, predicted
  throws and set-downs, kicks judged on what you saw, a 200 ms cap; a 6th digit of 1-4 applies a
  preset). `CRATE_DRILL=z|x|X|n|N` starts a drill 2 s in (for screenshots). `CRATE_TRUTH=1`
  starts with O on.
- **The drill bench:** `CRATE_HEADLESS=1 CRATE_BENCH=z|x|X|n|N CRATE_TRIALS=300` prints one
  `bench ...` line and exits. `CRATE_REACTION=<ms>`, `CRATE_RIVAL`, `CRATE_KICK_RIVAL` and
  `CRATE_DROP` set the drill numbers. The bench turns the mercy rule off.
- **Whole matches:** `CRATE_HEADLESS=1 CRATE_LAG_BOT=1 CRATE_MATCHES=<n>` (add
  `CRATE_BOTS_THROW=1` for thrown crates). Each match prints a `net ...` line.
- **What isn't faked:** jitter and packet loss; latency for the bots; a guess at the crates your
  throw strikes; what your kick-open makes reaching you late.
