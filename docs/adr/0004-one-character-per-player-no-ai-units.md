# One character per player, with no AI units

Each player controls one character, and no player owns or commands AI units. Team Buddies
gives each player a squad of AI Buddies, so this is a deliberate break from the source. We
chose it in play ([#10](https://github.com/ALLiDoizCode/big_back/blob/prototype/squad-model/docs/prototypes/squad-model.md#verdict-on-the-squad-model)).
The rejected options were all built and tried:

- a leader with 0–3 owned Buddies, given at the start or built from crate stacks;
- a hero plus one partner;
- team-owned AI haulers that take no orders.

The human's reason: "I don't want the build buddy I just want other players."

Bots exist only to stand in for missing players. They play by a player's rules and are never
units a player owns. This also keeps the netcode budget to one predicted character per
client. Don't add owned AI back without revisiting this ADR.
