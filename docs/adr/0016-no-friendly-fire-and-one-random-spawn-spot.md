# No friendly fire, and the spawn spot as the only randomness

In Crate Race, **friendly fire is off** for bullets, blasts and thrown crates. The one
friendly fire left is **knocks to your own pad**: a sliding crate knocks any pad crate that
hasn't locked in, your own team's included. The **only randomness is where a new crate
spawns**. What a river objective drops, when a megacrate comes, and where a shot goes are all
fixed, and the drop timers are shown. We chose this after playing the
[friendly-fire sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/friendly-fire/docs/prototypes/friendly-fire.md)
for [#35](https://github.com/ALLiDoizCode/big_back/issues/35), which switched each kind of
friendly fire and each source of randomness on [0006](0006-five-v-five-on-a-lanes-and-jungle-map.md)'s
5v5 map with [0013](0013-base-knockouts-with-a-comeback-set.md)'s rules. The human's verdict
on the defaults: "feels good".

## The rules

- **Bullets** pass through allies. **Blasts** (rockets, grenades, launcher grenades, tank
  shells, barrage shells) spare you and your allies. **Thrown crates** pass allies.
- **Own-pad knocks are on.** Curling ([0001](0001-throws-are-jump-timed-and-land-by-curling.md))
  applies to every crate on the map: a crate your team throws onto your own pad can knock
  your pad's loose crates, as an enemy's can. Locked-in crates never move (0013).
- **Crate spawns:** a new crate falls on a random open spot in the contested middle (the river
  and mid lane), not marked ahead.
- **River objectives:** a cleared one drops a super crate (3 crates with super crates off) and
  comes back in 60 s, with the countdown shown.
- **Megacrates:** one every 45 s while none is out, with the countdown shown. Where it drops
  is set by 0013's pull toward the trailing team.
- **Weapon spread:** none. A shot goes where it was aimed, and the shotgun fires a fixed fan.

## Considered options

- **Friendly fire on bullets** (half damage): it makes you care where your team stands. In
  bot matches it barely registered: about 1.3 players a team are up at a time, so allies
  were rarely in the line (39 hp a match from bots that fired anyway, 10 hp from bots that
  held fire).
- **Friendly fire on blasts,** your own only or everyone's: about 1 hp a match in bot play.
  Rejected in play, along with friendly fire on thrown crates (1 hit on an ally in 32 matches).
- **Own-pad knocks off** (your team's crate stops dead against your own pad): it changed
  nothing in bot scores. A team's own crates hit its pad about 17 times a match either way,
  and lock-in protects most of the stack. Keeping knocks on means curling needs no exception.
- **Spawns marked ahead, or fixed spots in turn:** fixed spots made bot matches busier (22
  kills against 19, more lead changes) and evened out who got the crates. Rejected in play:
  a random spot keeps the middle from being solved.
- **A random river roll** (anything from 3 crates to a megacrate, scattered, back in 40-80 s)
  and **random megacrate timing** (25-65 s, unwarned or marked 5 s ahead): rejected. A drop's
  worth and timing are what a team plans a fight around.
- **A random cone, or bloom** (the first shot exact, held fire spreads): rejected. Time to
  kill is short and projectiles are dodgeable ([0002](0002-projectile-weapons-not-hitscan.md)),
  so aim should be the whole skill.

## Consequences

- **Luck doesn't decide matches.** In bot play the team that got more of the random crates
  won about half the time, under every rule tried. With spawns as the only randomness, that
  is the whole of it.
- **Bots don't have to avoid hitting allies.** With friendly fire off, a bot never holds a
  shot for a teammate. The bot work that matchmaking depends on stays smaller.
- **Throwing onto your own pad takes care.** A badly aimed throw can knock your own unscored
  crates off the pad. That is a skill to learn, not friendly fire to fear: locked-in crates
  are safe.
- **Megacrate timing is predictable but not regular.** None drops while one is out, so in bot
  play they came about every 95 s, not every 45 s. The countdown only runs while the map has
  none.
- **Open:** whether friendly fire comes back for any later mode. The prototype's other calls
  (friendly damage at half a hit, structures never hurt, a team knockout counting for nobody)
  only matter if it does.
