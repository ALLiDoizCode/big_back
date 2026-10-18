# Throws are jump-timed and land by curling

Throwing is where crate-handling skill lives. A full **jump-throw** only happens in the air,
and its reach grows to 1.5× at the top of the jump. On the ground you can only **toss** a
crate a short way. The player aims at a **touchdown point**, not at the final spot. A crate
keeps most of its speed when it lands, slides, **knocks** any crate it runs into (passing
on its momentum), and settles into a **cell** once it slows down near one. Placement is
therefore read from the slide. We picked this in play ([#9](https://github.com/ALLiDoizCode/big_back/issues/9),
[verdict](https://github.com/ALLiDoizCode/big_back/blob/prototype/core-feel/docs/prototypes/core-feel.md#verdict)) over exact "stick" landings, a flat airborne bonus, and throwing at
any time. The exact version was precise but had no skill ceiling. Curling plus apex timing
was the version the human called fun.

## Consequences

- Crates slide and collide after landing, so the server has to simulate sliding crates, not
  just thrown ones. A crate only stops being a moving body once it has settled into a cell.
  This fits the netcode budget's advice that crates placed on the pad lock or merge.
- The client's arc preview runs the same crate motion as the server, so the resting
  **ghost** it shows is where the crate really stops. Divergence would make the skill feel
  random.
- Knocks are a tool for stealing and defending: a crate can be curled into an enemy stack.
  The crate economy grammar ([#17](https://github.com/ALLiDoizCode/big_back/issues/17))
  should assume building happens by throwing into cells.
