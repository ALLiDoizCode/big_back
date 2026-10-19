# Server grabs, judged on what you saw

**Grabs and steals are decided by the server**, and the server **judges each grab on what
the grabber saw**, looking back at most 200 ms (server rewind). The client never predicts a
grab: the hand reaches out at once, and the crate comes into it when the server's answer
arrives, one round trip later. Grab reach stays 1.2 m. We chose this after playing the
[latency sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/latency/docs/prototypes/latency.md)
for [#36](https://github.com/ALLiDoizCode/big_back/issues/36), which faked 95 ms and 165 ms
pings, the furthest player's ping at the busiest and quietest hours in
[0012](0012-one-rated-queue-with-short-starts.md)'s cross-region queue. The human's verdict:
"server grabs with rewind feels good".

## The rules

- **A grab goes for the nearest crate the player sees** loose and in reach.
- **The server grants the grab** if the crate is loose and in reach now, or was loose and in
  reach when the player saw it. It looks back by the player's round trip plus interpolation
  delay, at most 200 ms.
- **Judged on what you saw, the earlier grab wins.** A crate granted this way comes out of
  the hands of whoever took it since, and a crate that locked in since comes loose again.
- **A lost grab says why:** taken first, moved out of reach, locked in, or gone.
- **No prediction:** the crate is never shown in the player's hands before the server says
  yes, so a lost grab never snaps back. While the answer is on its way, the hand stays
  reached out, and grab and throw do nothing.

## Considered options

These come from a drill bench in the prototype. The player's seat grabbed 175-325 ms after the
crate showed up in its view, 300 trials each, against bot rivals with no latency (the worst
case). Share of trials won:

| Setup | Race for a crate | Steal vs lock-in | Steal vs a defender | Crate sliding past |
|---|---|---|---|---|
| 0 ms | 57% | 98% | 65% | 100% |
| 165 ms, judged on now | 8% | 59% | 36% | 0% |
| 165 ms, reach 1.7 m | 5% | 58% | 62% | 99% |
| **165 ms, judged on what you saw** | **44%** | **99%** | **65%** | **77%** |

- **Judged on now** (the netcode research's plan, and the spec before this): fine for a crate
  at rest, which was never lost (0 of 834 grabs in whole bot matches at 0, 165 and 250 ms).
  But every contest is lost at 165 ms: the rival sees the crate 205 ms sooner.
- **Predicted grabs, snapping back if lost:** the crate is in your hands at once. It doesn't
  change who wins, only what you see, and a lost grab is shown and then taken away. Rejected
  in play.
- **A wider reach** (1.7 m or 2.2 m, for everyone): it fixes a crate sliding past, but not a
  race, because the rival's reach grows too. Not picked.

## Consequences

- **The rival sees a crate leave their hands.** That is the price of judging on what the
  grabber saw. In the race drill, 108 of 300 wins came out of the rival's hands. Between two
  lagged humans, whoever grabbed first in their own view wins.
- **The cap matters.** At 250 ms ping, a 200 ms rewind can't look back far enough (the race
  fell to 32%). The cap bounds how much a high ping is favoured, as in lightyear's rewind
  demo. 0012 keeps most players under it.
- **What rewind can't fix:** a player still sees everything late. A race decided by who sees
  the crate first stays lost at high ping.
- **The server keeps a short history** of each crate's position and state (loose, carried,
  locked), about 1 s, to judge grabs on. The netcode research expected lag compensation for
  hitscan only; this extends it to grabs, which lightyear's ray-cast rewind doesn't cover, so
  the grab check is our own.
- **Open:** throws, set-downs and kick-opens weren't ruled on (server-decided or predicted),
  nor was the reach on its own. The rewind cap, and how a bot or a late joiner's history
  works with rewind, also stay open.
