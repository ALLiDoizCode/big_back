# Crate grammar: class loadouts, by shape, held kick, steal only

What stacks build is decided by **the class of whoever kicks the pad open**, and recipes are
read **by shape**, as in Team Buddies. We chose this grammar in play on the 5v5 map
([#17](https://github.com/ALLiDoizCode/big_back/issues/17),
[sheet](https://github.com/ALLiDoizCode/big_back/blob/prototype/arena-map/docs/prototypes/crate-grammar.md#verdict)). The human's verdicts: "it feels good", then "by
shape, instant, steal only".

- **Each class has a left-click and a right-click recipe.** 1 × 2 (two side by side) makes
  the kicker's class's left-click weapon or utility; 2 × 2 flat makes its right-click one.
  The player recipes from [0005](0005-players-join-from-the-pad.md) don't change.
- **Hold the kick longer to use more crates.** Holding the kick steps up through the recipes
  that fit inside the pad, smallest first; letting go opens that one and uses only its
  crates. The rest stay on the pad. Without it, 2 × 2 flat is built through 1 × 2, and a
  team that opens the pair never reaches its right-click item.
- **Instant:** a recipe's result comes out on the kick, with no build timer.
- **Steal only:** crates on a pad can be stolen, but shots don't knock them off and they
  don't lock.

Rejected after play: fixed recipe books (mix, weapons, base upgrades) and a whole book per
class; count-only recipes; 1 s and 2 s per crate build timers; hits that split or spill a
stack; crates that lock once placed; a kick that uses the whole pad.

## Consequences

- **Who kicks is a decision.** Classes are part of the game, and who joins from the pad
  decides what a team can build.
- **Netcode:** the netcode-budget research asked for crates on a pad to lock. Steal only
  rejects that, so resting pad crates have to sleep and stop replicating some other way,
  and stealing one wakes it.
- **Open:** the classes and their items (the prototype's are placeholders), what 1 crate and
  all 8 make, whether a pad shows what it will make, and super crates.
