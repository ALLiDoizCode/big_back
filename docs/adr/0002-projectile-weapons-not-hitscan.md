# Weapons fire visible projectiles, not hitscan

Shots are slow (18 m/s), visible projectiles resolved on the server, not hitscan. The
netcode research found that lag compensation in our stack covers ray casts only, so hitscan
would favour the shooter's view of the past. In play ([#9](https://github.com/ALLiDoizCode/big_back/blob/prototype/core-feel/docs/prototypes/core-feel.md#verdict)) dodgeable
projectiles felt better than hitscan anyway. Time to kill is short, about 1 s (3 shots/s,
4 hits), so dodging and positioning matter more than tracking.

## Considered Options

- **Hitscan with lag compensation:** the netcode supports it, but it was rejected in play.
  Instant hits also undercut dodging, which the slow-carrier rules depend on (ADR-0003).
