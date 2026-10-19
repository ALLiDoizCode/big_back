# Predicted throws, and kicks judged on what you saw

**Throws, punts and set-downs are predicted.** The client shows them at once, worked out on the
world as the player sees it, and corrects them if the server's answer differs. **Kick-opens are
decided by the server, judged on what the kicker saw**: the ring fills as the player holds, and
the server opens the recipe the ring showed, looking back at most 200 ms, like grabs
([0017](0017-server-grabs-judged-on-what-you-saw.md)). The rewind cap stays **200 ms**. We
chose this after playing the
[throw latency sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/throw-latency/docs/prototypes/throw-latency.md)
for [#54](https://github.com/ALLiDoizCode/big_back/issues/54), at 165 ms against 95 ms and no
latency. The human's verdict, on this setup: "feels good".

## The rules

### Throws, punts and set-downs

- **The client sends the throw as it aimed it:** from where, and how fast. The server flies it
  in the real world. A set-down sends the cell, and the server stacks the crate on whatever is
  really in it.
- **The client predicts it:** the crate leaves the hands at once and flies against the crates
  and players as that player sees them. A set-down stacks on the crates the player sees at rest.
  A crate the throw strikes isn't predicted; it moves when the server says.
- **A wrong guess is corrected.** When the server's answer is more than 0.3 m from the guess,
  or the server has the crate in someone's hands, the shown crate blends (about 120 ms) to where
  the server has it: on to where it really is while it still moves, or to where the player sees
  it once it's at rest.
- **A set-down stacks only on crates at rest,** never on one falling past the cell.

### Kick-opens

- **The ring is predicted.** It shows only how long the player has held, so it's never wrong.
  It shows the recipes on the pad as the player sees it.
- **The server opens what the kicker saw.** On letting go, it looks back by the kicker's round
  trip plus interpolation delay, at most 200 ms, and opens the recipe the hold reached on the
  pad as it was then.
- **The earlier kick wins.** A crate taken since comes out of the taker's hands and is spent
  where the kicker saw it. A crate that landed on the pad since isn't spent.
- **If a crate the kicker saw is gone** (kicked open, destroyed), the server judges on the pad
  as it is now.
- **What a kick makes isn't predicted:** the crates go, and its result arrives, with the
  server's answer, one round trip later.

## Considered options

These come from the prototype's drills and whole bot matches (the bench in the sheet). Rivals
and teammates were bots with no latency, the worst case.

- **The server decides throws and set-downs** (the spec before this): the crate hangs in the
  hands for a round trip plus interpolation (205 ms at 165 ms) before it goes. Rejected in play.
  A guess is almost never wrong for a throw onto a pad (4 of 1,192 in whole matches at 95-250
  ms). It's wrong about 1 time in 10 for a set-down on a busy pad, always by one crate, and
  55% of the time for a throw at a running player at 165 ms.
- **The server decides kicks, ring and all:** the ring shows late, so the player holds about
  90 ms longer at 165 ms, and a rival gets that long to steal. Against a steal, the full recipe
  opened 24% of the time.
- **A predicted ring, judged on now:** 27% of kicks against a steal opened something other than
  what the ring showed at 165 ms.
- **Predicting what a kick opens, corrected when wrong:** the same 27%, and the crates vanish
  then come back. Rejected in play.
- **Judged on what you saw (chosen):** none of the contested kicks opened anything else at
  95 or 165 ms, and the full recipe opened 60% of the time. In 74 of 300 trials a crate came
  out of the rival's hands.
- **A different cap:** at 250 ms ping, a 200 ms cap left 12% of contested kicks wrong, 250 ms
  left 3%, and 300 ms none. 200 ms stays, as for grabs.

## Consequences

- **The rival sees a crate leave their hands** when a kick is judged on what the kicker saw,
  as with grabs.
- **Corrections are visible.** A throw at a moving player is often corrected mid-flight, and a
  set-down on a busy pad sometimes moves up or down a level.
- **The server's crate history** (0017) also serves kicks: it rewinds each pad to judge them.
- **Settles** what 0017 left open: throws, set-downs and kick-opens, and the rewind cap.
- **Open:** how a bot or a late joiner's history works with rewind; jitter and packet loss,
  which the prototype didn't fake; whether predicting the crates a throw knocks is worth it.
