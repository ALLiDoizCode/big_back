# Base knockouts, with a comeback set

Each team has a **base** behind its pad that enemy shots and blasts can hurt. A base brought to
0 hp is **knocked out**: its pad goes down for a while, then the base rebuilds. Production can
be attacked, but never lost for good. Five **comeback** rules come with it: lock-in, spill,
megacrates on the trailing side, overtime, and a mercy rule. We chose this after playing the
[production-loss sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/production-loss/docs/prototypes/production-loss.md)
for [#15](https://github.com/ALLiDoizCode/big_back/issues/15), which built on the
[win-conditions research](https://github.com/ALLiDoizCode/big_back/blob/research/arena-win-conditions/docs/research/arena-win-conditions.md)
§9. The human's verdict: "knockout with all comeback knobs on feels good". The rules below
are that setup as it stood in the sheet. The numbers are tuning, not decisions.

## The rules

- **Score:** crates resting on a team's pad that have locked in, plus crates spent there on
  recipes, plus points banked from knockouts. Most score at the end wins.
- **Base:** a block behind each pad, 750 hp. Enemy shots, rockets, grenades and tank shells
  hurt it. The pad's own crates still can't be shot (ADR [0007](0007-crate-grammar-class-loadouts-by-shape.md)).
- **Base knockout:** at 0 hp the pad is **down for 20 s**. It makes nothing and scores nothing
  new. The attackers get **+3** to their score. Then the base rebuilds at full hp.
- **Lock-in:** a crate scores only once it has rested in a pad cell for **8 s**. Until then it
  can be stolen and doesn't count. After that it is locked in: it counts, nobody can steal
  or knock it, it can still be kicked open for a recipe, and it survives a knockout.
- **Spill:** a knockout throws the pad's crates that aren't locked in onto the map, toward the
  middle, **at most 3**, top crates first. Anyone can pick them up. The rest stay on the pad.
- **Megacrates:** one drops every **45 s** while none is out, worth **3** to the score and one
  cell in a recipe. It drops on the **trailing team's side**, pulled toward its base by
  2.2 m per point of the gap, up to 22 m.
- **Overtime:** at the buzzer, play goes on while the trailing team carries a crate or stands
  within 5 m of the leader's pad. It ends 1 s after they stop, and lasts at most **60 s**. A
  tie runs until someone leads. A tie at the cap is a draw.
- **Mercy rule:** a **15-point** gap held for **20 s** ends the match for the leader.

## Considered options

- **Permanent base destruction** (Team Buddies decapitation): a destroyed pad makes nothing,
  nobody joins, and the starting player stops respawning. A team with no base and nobody left
  loses. In bot matches 3-5 of 8 ended early by elimination, at about 4-4.5 min. That is the
  Brawl Stars Siege pattern: the match is decided but keeps going.
- **A base kill ends a round,** best of 3 with a reset: bot rounds score low and drew often,
  so a match ran 9-10 min (up to 6 rounds) without overtime to break ties. The reset also
  throws away what the winning team built.
- **No destructible base, theft only:** with a 2-player head start, the leading team won 8 of
  8 with no comeback rules. It led by 11, and the trailing team spent 43 s a match 10 or more
  behind. With the five comeback rules on, the trailing team won 6 of 8.
- **Megacrates in the river:** they doubled the leader's biggest lead (to 24 points) and kept
  the trailing team 10+ behind for 3 min a match. A megacrate is only a comeback tool if it
  drops on the trailing side.
- **Any comeback rule alone:** none of them changed a theft-only head-start result on its own
  (lock-in won 2 of 8, overtime 0). They only did it together.

## Consequences

- **Lock-in partly reverses "steal only" (ADR [0007](0007-crate-grammar-class-loadouts-by-shape.md)).**
  That pick rejected crates that lock as soon as they're placed. Here they lock 8 s later, so a
  steal has a window and then stops. It also brings back the netcode win that research wanted:
  a locked crate can sleep and stop replicating.
- **Production can be attacked but is never lost for good.** This follows the research rule
  that destroyed production must not be permanent within a scoring period (§9.2.2).
- **Megacrates are a visible handout to the trailing team.** The research warned that
  comeback levers must stay legible (§9.2.5). The drop says where it fell and why.
- **Matches end on a fight, not a buzzer.** A match can run up to 60 s past the timer.
- **Bots must raid and defend bases.** In the prototype, bot teams (about 1.2-1.6 players up)
  felled a base only about once every two matches. Bot AI for raids is part of the bot work
  matchmaking already depends on.
- **Open:** whether the +3 knockout bonus and the mercy rule stay in casual play; the
  mercy rule's interaction with the rating (a mercy end counts as a normal win or loss
  unless decided otherwise); exact tuning of base hp and the down time once humans play
  against humans.
- **Settled by [0017](0017-leavers-take-a-loss-and-their-seats-reopen.md):** a mercy end counts as a normal win and loss.
