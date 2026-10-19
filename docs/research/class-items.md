# Class items, and what 1 crate and all 8 make

Research notes for [#21](https://github.com/ALLiDoizCode/big_back/issues/21). Research date: 2026-10-18.

**Question.** What should each class's crafted items be (soldier, engineer, medic)? And what should the two open recipes make: **1 crate** and **all 8**? The grammar is fixed by [ADR 0007](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0007-crate-grammar-class-loadouts-by-shape.md):

- the kicker's class decides what a recipe makes;
- 1 × 2 makes the class's left-click item and 2 × 2 flat makes its right-click item;
- a held kick steps up through the recipes inside the pad, smallest first;
- results are instant, and pads are steal only;
- the pad shows what each step makes;
- a super crate upgrades the result into a stronger version of the same thing.

The prototype's items are placeholders ([crate-grammar sheet, "Class loadout"](../prototypes/crate-grammar.md#class-loadout-rule-1-class-loadout-lmbrmb)). 1 crate gives full health and all 8 gives a tank, for every class.

This file compares how class-based competitive games split a character into a main fire, a secondary and a rare big payoff. It then rules on deployables against [ADR 0004](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0004-one-character-per-player-no-ai-units.md) and ends with three candidate item sets.

Already covered elsewhere and not redone here:

- Team Buddies' recipe tables (every slot's normal and super toy, per world): [`team-buddies-core-loops.md` §1 "Stack shape to output category"](team-buddies-core-loops.md#stack-shape-to-output-category) and [§4 "Units and items"](team-buddies-core-loops.md#4-units-and-items).
- Splatoon 3's Salmon Run and Rainmaker loops: [`modern-team-buddies-likes.md` §10](modern-team-buddies-likes.md#10-splatoon-3-nintendo-switch-2022-salmon-run-and-rainmaker).

## How to read this doc

- Every factual claim links to its source. Sources are **primary** unless marked: official sites, official patch notes, developer blogs, GDC Vault, Valve's published paper, and the game's own disc data (via the Team Buddies research file).
- **[Lower confidence]** marks a claim that rests on a community wiki (Fandom, deadlock.wiki, the League or TF2 community wikis), a press report or a press interview, because no primary page could be found or read. TF2 developer-commentary lines are quoted from the community TF2 wiki's transcripts. They are developers' words, but the transcript is a secondary copy, so they carry the mark too.
- **[Synthesis]** marks my own reasoning: design opinion for this game, not a fact about any source.
- Terms follow [`GLOSSARY.md`](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/GLOSSARY.md): carrier, pad, recipe, kick open, super crate, class, **loadout** (our two crafted items; other games' "kits" are called by their own names), starting player, joined player, camp, river objective.
- Pages were read through a page-summarising fetch tool. Quotes should be close to verbatim but not character-perfect, so recheck a line before quoting it in public.

---

## TL;DR

1. **Every comparable game gives a character one always-on tool, one limited tool, and one rare payoff that is earned in play.** Splatoon ([main, sub, special](https://www.nintendo.com/us/store/products/splatoon-3-switch/)), Brawl Stars (attack, Super, Gadget), Overwatch (weapon, abilities, ultimate) and TF2 (Medi Gun and ÜberCharge) all do this. Our base gun is the always-on tool. The left-click and right-click items are the limited tools, and **all 8 is the rare payoff**. **[Synthesis]**
2. **Readability comes from silhouettes and from tying an effect's loudness to its importance.** Valve designed TF2's classes so players could "visually identify other players very quickly… to assess the possible threat" ([Mitchell et al. 2007](https://www.cs.princeton.edu/courses/archive/fall07/cos597B/papers/mitchell-team-fortress.pdf)). Riot says "the amount of attention an ability grabs should match its level of importance" ([Clarity in League](https://www.leagueoflegends.com/en-us/news/dev/clarity-in-league/)). DICE brought classes back to Battlefield 2042 partly for "readability of encounters" ([Dev Notes: Classes](https://www.ea.com/games/battlefield/battlefield-2042/news/dev-notes-classes-in-battlefield-2042)).
3. **Big payoffs are throttled twice: by what they cost, and by a lockout after use.** Splatoon tunes the special's cost per weapon kit and blocks recharging until the gauge drains ([Splatoon 3 update history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history)). Battlefield 2042 gives each team a vehicle budget and a cooldown ([Battlefield Briefing](https://www.ea.com/en-gb/games/battlefield/battlefield-2042/news/battlefield-briefing-answering-your-reveal-questions)). Our crates are already the cost. The haul (about 12 s per round trip), steal-only pads and ammo or timers do the rest. **[Synthesis]**
4. **1 crate should be a small, self-contained consumable, not full health.** It is the first step of every held kick and the easiest pad to steal from, so it will be made often. Full health from 1 crate makes the medic and the heal camps redundant. **[Synthesis]**
5. **All 8 should be the class's identity at full volume, loud enough to hear and see across a lane.** Team Buddies always makes a vehicle from 8 crates ([disc data, via the TB research file](team-buddies-core-loops.md#stack-shape-to-output-category)). A tank suits the soldier. The engineer and medic need their own capstones, or the class choice stops mattering at the top recipe. **[Synthesis]**
6. **Supports stay fun at 5v5 by having agency beyond healing, and by facing counters.** Blizzard says 5v5 gave supports "more agency to add some damage or duel other heroes" ([Director's Take on 5v5](https://overwatch.blizzard.com/en-us/news/24104605/director-s-take-opening-up-the-conversation-on-5v5-and-6v6/)). Riot prefers shields to heals because "temporary health has more gameplay and timing involved" ([Designing Milio](https://www.leagueoflegends.com/en-sg/news/dev/designing-milio-friends-and-flames/)). Hi-Rez made anti-heal a baseline because a must-buy counter was "required for a match to feel 'normal'" ([Paladins Season 5 notes](https://cms.paladins.com/2022/01/05/season-5-schism-update-notes/)).
7. **A super should keep the item's identity and silhouette. It can change one behaviour as well as the numbers.** Blizzard's Overwatch team says "the number buff perks are some of the perks we like the least" ([Director's Take: AMA](https://overwatch.blizzard.com/en-us/news/24179365/director-s-take-catch-up-on-our-ama/)). ADR 0007 rejected supers that branch to another result. The middle ground is the same item plus one twist. **[Synthesis]**
8. **Turret ruling:** anything that **picks its own target or moves by itself** is an owned AI unit and breaks ADR 0004. That rules out the prototype's turret drop and the fortress's turret. Static objects (walls, cover, heal zones), visible one-shot traps, and devices that fire only when their owner fires are allowed. See [§6](#6-ruling-deployables-against-adr-0004).

---

## 1. How the comparison games split a character

| Game | Always-on | Limited | Rare payoff | How the payoff is earned | Throttle after use |
|---|---|---|---|---|---|
| **Splatoon 3** | Main weapon | Sub weapon, paid from the same ink tank | Special | Inking turf ([Nintendo](https://www.nintendo.com/us/store/products/splatoon-3-switch/)) | Gauge can't refill until it has drained; extra cooldown on Tacticooler ([update history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history)) |
| **Brawl Stars** | Attack (ammo bar) | Gadget, a few uses per match | Super; later Hypercharge | Hitting enemies; Hypercharge charges "in the same way… but at a slower rate" ([patch notes](https://supercell.com/en/games/brawlstars/blog/release-notes/rangerranch-patch-notes-2/)) | Super is spent; Hypercharge lasts a short time |
| **Overwatch 2** | Primary and secondary fire | Cooldown abilities | Ultimate | Play over the match (only implied by patch notes such as the Tank passive's "-40% ultimate generation"; no explicit primary statement found) | Charge resets |
| **TF2 Medic** | Medi Gun | — | ÜberCharge | Healing, with "more charge when the Medic is healing injured teammates" **[Lower confidence]** ([Gravel Pit commentary](https://wiki.teamfortress.com/wiki/Gravel_Pit_developer_commentary)) | Charge resets |
| **Battlefield 6** | Weapon (mostly open to any class) | Two class gadgets, signature gadget ([loadouts](https://www.ea.com/en/games/battlefield/battlefield-6/news/loadout-creation-and-weapon-classes)) | Vehicles; in BF V, squad call-ins | BF V: squad points, e.g. supply drop 2,450, tank 20,750, V-1 41,500 ([reinforcements](https://www.ea.com/games/battlefield/news/how-to-use-reinforcements)) | BF2042: per-team budget and cooldown ([Briefing](https://www.ea.com/en-gb/games/battlefield/battlefield-2042/news/battlefield-briefing-answering-your-reveal-questions)) |
| **Paladins** | Weapon | Abilities; 5-card, 15-point loadout picked before the match | Ultimate that "passively charges over the course of a match" | Time and play ([Hi-Rez guide](https://support.hirezstudios.com/en/articles/9516132-new-returning-player-guide)) | Charge resets |
| **Deadlock** | Gun and abilities | Bought items in 4 price tiers **[Lower confidence]** ([deadlock.wiki](https://deadlock.wiki/Items)) | Top-tier items; extra item slots unlocked by killing objectives ([Map Rework update](https://store.steampowered.com/news/app/1422450/view/524216645064852902)) | Souls | Permanent for the match |
| **Team Buddies** | — | 1-crate light weapon, 2-flat grenades, 4-flat heavy weapon | 8-crate vehicle | Crates stacked on the pad ([disc data](team-buddies-core-loops.md#stack-shape-to-output-category)) | Ammo is one per-Buddy pool; vehicles can be destroyed or hijacked ([combat](team-buddies-core-loops.md#combat)) |

Details worth carrying over:

- **Splatoon sells kits as fixed bundles.** Weapons come as main + sub + special sets, and some sets share a main but differ in sub and special **[Lower confidence]** ([EDGE interview, reported](https://www.perfectly-nintendo.com/?p=11985)). Nintendo tunes the special's cost per kit: Booyah Bomb costs 200 on one weapon and 190 on another **[Lower confidence]** ([Shacknews](https://shacknews.com/article/132358/splatoon-3-weapon-sub-special-list)). Most balance patches move costs by about ±10 points ([update history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history)).
- **Splatoon subs share the main weapon's ink.** Burst Bomb went from 40% to 45% of a tank in Ver. 3.1.0, and Curling Bomb from 70% to 65% in Ver. 5.1.0. The stated aim was "limiting repetitive sub weapon tactics" ([update history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history)).
- **Brawl Stars Gadgets are activated abilities with a use limit.** Gadgets give "an activated ability", and "damage dealing gadgets do not charge Super" ([#BrawlGadgets](https://supercell.com/en/games/brawlstars/blog/release-notes/brawlgadgets-update/)). The per-match limit is only shown indirectly, by the Gadget Gear that "allows one more usage… per game (3 → 4)" ([patch notes](https://supercell.com/en/games/brawlstars/blog/release-notes/rangerranch-patch-notes-2/)).
- **TF2 unlocks are trade-offs, not upgrades.** The Kritzkrieg's charge gives guaranteed crits instead of invulnerability, and the Blutsauger loses crits but steals health on hit **[Lower confidence]** ([Shacknews](https://shacknews.com/article/52236/valve-reveals-final-team-fortress), [Engadget](https://www.engadget.com/2008-04-16-team-fortress-2-unlockable-medic-weapons-detailed.html)).
- **Battlefield 6 splits one job into a cheap and a full item.** The Supply Bag "refills all weapon ammunition, grenades and gadget charges… for allies within a designated radius". The Supply Pouch "only refills weapon ammunition", but it is harder to spot and you carry more ([Support class](https://www.ea.com/en/games/battlefield/battlefield-6/news/support-class)). That pair is a direct model for a 1-crate item next to a bigger recipe. **[Synthesis]**
- **Team Buddies' own 1-crate results:** on the pad, 1 crate is the light weapon (Uzi, Shotgun, Pocket Rocket…). Off the pad, kicking a crate open gives "health **or** ammo", and the tutorial calls this "the quickest way to reload" ([TB research, carrying and stacking](team-buddies-core-loops.md#carrying-and-stacking-crates)).

**[Synthesis] What this means for our loadout.** Our base gun (25 damage, 3 shots/s, 18 m/s, about 1 s to kill; [ADR 0002](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0002-projectile-weapons-not-hitscan.md)) is everyone's main fire. Unlike every game above, **a class has no class tools until its kicker crafts them.** So the class has to read before any crafting, and a crafted item is a temporary power spike, much like Halo's power weapons ([§3](#3-balancing-temporary-crafted-power-against-the-base-gun)). The left-click item replaces the gun until its ammo runs out (current prototype rule). The right-click item is a charged secondary that works with empty hands only, because right click while carrying still throws ([ADR 0003](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0003-carrying-is-a-commitment.md) keeps carriers unarmed).

## 2. What makes a class readable to opponents

Sources:

- **Silhouettes first.** Valve designed TF2's nine silhouettes "to be very distinct", and checked class designs as pure silhouettes with no internal detail ([Mitchell, Francke, Eng 2007](https://www.cs.princeton.edu/courses/archive/fall07/cos597B/papers/mitchell-team-fortress.pdf)). Riot: "Silhouettes are the single most important thing for champion recognition," and a silhouette "should highlight their source of power" ([Clarity in League](https://www.leagueoflegends.com/en-us/news/dev/clarity-in-league/)).
- **A fixed read order.** TF2's artists set one: "first, what team they're on, second, what class they're playing, and third, what weapon they're wielding" **[Lower confidence]** ([Well developer commentary](https://wiki.teamfortress.com/wiki/Well_developer_commentary)).
- **Show hidden state on the model, not over it.** Robin Walker said some information, "such as the Medic's invulnerability charge… would be completely invisible unless we exposed it". A floating disguise icon over the Spy "baffled" playtesters, so it was replaced by paper masks worn on the model **[Lower confidence]** ([Hydro developer commentary](https://wiki.teamfortress.com/wiki/Hydro_developer_commentary)).
- **Attention should match importance.** Riot: "Spells that drastically change how you play with or against them should be flashy," "if something is dodgeable, it should draw more attention," and "noise should be kept minimal". It also flags the problem "when hitboxes don't line up with their effects" ([Clarity in League](https://www.leagueoflegends.com/en-us/news/dev/clarity-in-league/)).
- **Counterplay is the point of reading.** Riot defines counterplay as any "action, choice, or strategy a player can use to mitigate or overcome an enemy threat". Its benefit is a "back and forth… instead of a 'who shoots first' one-and-done" ([Quick Gameplay Thoughts, May 2021](https://www.leagueoflegends.com/en-gb/news/dev/quick-gameplay-thoughts-may-14/)). Tom Cadwell's GDC 2013 talk frames counterplay as gameplay for your opponents ([GDC Vault](https://gdcvault.com/play/1018158/)).
- **Sound carries the big reads.** Overwatch's audio goal was to make the game playable by sound ([GDC 2016, Lawlor and Neumann](https://gdcvault.com/play/1023010/Overwatch-The-Elusive-Goal-Play)). The audio middleware vendor lists "gameplay information" and "Pavlovian response" among the pillars ([Audiokinetic blog](https://www.audiokinetic.com/en/blog/overwatch-game-audio-using-wwise-1/)). No primary source states that ultimate voice lines are meant as warnings to enemies, though players read them that way.
- **Readability was a reason classes came back.** DICE wanted players "able to recognize the Class and potential loadout scenarios that they may be carrying". Open gadget pools had made "min/maxing… prevalent" ([Dev Notes: Classes](https://www.ea.com/games/battlefield/battlefield-2042/news/dev-notes-classes-in-battlefield-2042)).
- **Show what the enemy bought.** Deadlock's shop log "now shows enemy and ally purchases with different colors", and imbue tooltips say exactly which items an ability carries ([11-21-2024 update](https://forums.playdeadlock.com/threads/11-21-2024-update.47476/)).
- **Projectile heals read better than instant ones.** Blizzard says Illari's heal was "initially a hitscan healing projectile" and changed "to fulfil combat clarity and visual goals" ([Illari design](https://overwatch.blizzard.com/en-gb/news/24003136)).

**[Synthesis] Rules for our items:**

1. **The class reads before any crafting.** The prototype's floating markers (^, square, +) are exactly what TF2 removed for the Spy. The class should live on the body instead: a backpack shape, a tool on the hip, a helmet.
2. **Every crafted item shows on the character**, and every left-click item has its own projectile colour and shape. A rocket, a rapid shot and a heal shot should be told apart in flight, because projectiles are the thing you dodge (ADR 0002).
3. **Super items keep the same silhouette** with a purple trim, the colour of the super crate. You read "the same thing, stronger" from the trim, never a new shape.
4. **The pad already does Deadlock's purchase log.** ADR 0007 shows the enemy's next result at each held step. Keep every result name short enough to read at a glance.
5. **All-8 results announce themselves**: a map-wide audio sting when kicked open, and a minimap icon. They are the "drastically change how you play" tier that Riot says should be flashy.
6. **Loudness follows cost**: 1-crate items are quiet, left- and right-click items are clear, and all-8 results are loud.

## 3. Balancing temporary crafted power against the base gun

Sources:

- **Halo limits power weapons on the map.** After "feedback around there being too much power on the map in Ranked Arena," 343 made weapon racks stop respawning a weapon "until the original has despawned", with ammo cuts to follow ([Halo Waypoint, February update](https://www.halowaypoint.com/news/february-update-preview)). On Streets the fix was moving strong weapons away from ideal spots ([Waypoint, Jan 9](https://www.halowaypoint.com/news/january-9-ranked-arena-updates)).
- **Splatoon throttles strong team buffs after use.** The Tacticooler's post-placement gauge cooldown went "from 7.5 seconds to 10 seconds" ([update history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history)).
- **Brawl Stars walked back raw power.** Hypercharges gave "slightly too high value to a point that it can feel frustrating to face". Supercell wanted "more ample counterplay" and less "'Hypercharge = win' gameplay" ([October 2025 notes](https://supercell.com/en/games/brawlstars/blog/game-updates/release-notes-october-2025/)). It then said: "We want to focus more on utility and skill expression as opposed to raw strength (like Hypercharge was)" ([Nov 2025](https://supercell.com/en/games/brawlstars/blog/news/new-power-brawl-pass-changes-and-a-new-starr-drop-2/)).
- **Battlefield budgets vehicles.** Call-ins have "a specific budget assigned to each team", and using one incurs "a cooldown so everyone has a fair chance" ([Briefing](https://www.ea.com/en-gb/games/battlefield/battlefield-2042/news/battlefield-briefing-answering-your-reveal-questions)). Update 1.2 started the cooldown when the vehicle is destroyed ([1.2 notes](https://www.ea.com/games/battlefield/battlefield-2042/news/battlefield-2042-update-notes-1-2)). BF2042's Rangers had "a limitation on the amount… available at any given time" ([Briefing](https://www.ea.com/en-gb/games/battlefield/battlefield-2042/news/battlefield-briefing-answering-your-reveal-questions)).
- **Paladins tunes cost and scaling separately.** Haven's cost went from 300 to 350 while its scaling rose from {5.5|11|16.5}% to {6|12|18}% ([RWBY notes](https://cms.paladins.com/2022/06/01/rwby-update-notes/)). Players may own "4 different items per game" ([Hi-Rez guide](https://support.hirezstudios.com/en/articles/9516132-new-returning-player-guide)).
- **Deadlock pushes against going all in.** Category bonuses are based on total spend in a tree, and "are not linear (maxing out one tree fully isn't optimal bonus-wise)" ([Shop Rework](https://store.steampowered.com/news/app/1422450/view/524216645064852902)).
- **Riot cut items that overrode kits.** Mythics had become "a larger percentage of the overall power budget than desired" and "overrode champion kits to an extreme degree" ([2024 item changes](https://www.leagueoflegends.com/en-gb/news/dev/dev-2024-item-changes/)).
- **League's team-wide payoffs are objectives one team can hold.** Only one team can have a Dragon Soul ([Preseason 2020](https://www.leagueoflegends.com/en-us/news/game-updates/preseason-2020-gameplay-rise-of-the-elements/)). Riot built it to "create an alternate win condition to Baron" ([2020 look back](https://www.leagueoflegends.com/en-gb/news/dev/dev-a-look-back-at-league-in-2020/)). Baron's buff lasts 180 s for living teammates **[Lower confidence]** ([League wiki](https://wiki.leagueoflegends.com/en-us/Hand_of_Baron)).
- **Team Buddies' ammo is one pool per Buddy**, so a stronger gun is also a hungrier one ([TB takeaways](team-buddies-core-loops.md#essential-the-core-that-makes-it-team-buddies)).

**[Synthesis] Our throttles**, from the outside in:

| Throttle | Already in the design | Use it for |
|---|---|---|
| **Crate cost** | 1 / 2 / 4 / 8 crates per recipe | How often a tier appears |
| **Haul time** | About 12 s per round trip from mid (ADR 0006); a carrier is slow, unarmed and drops the crate when hit (ADR 0003) | Makes all 8 a team-sized project of about 8 hauls |
| **Theft** | Steal only (ADR 0007) | A big pad is a target. All 8 is the most stealable recipe while it's being built |
| **Readout** | The enemy sees your next result (ADR 0007) | Counterplay before the kick: rush the pad, or prepare |
| **Ammo or charges** | Left-click items run dry; right-click items have charges | Caps how long a 2- or 4-crate spike lasts |
| **Timer and hp** | The tank: 45 s or 300 hp | Caps the capstone |
| **Loss on knockout** | Guns are lost when the holder is knocked out | Makes a crafted item a bounty |

Two gaps to close:

- **No lockout after all 8.** A team that kicks open all 8 can start hauling the next 8 at once. A Splatoon-style lockout fits: the team can't open another all 8 until its current one has ended. It also stops two tanks at once, which echoes Battlefield's team budget.
- **Supers double the charges.** The prototype's super gives "double the charges, and 1.5× damage or healing" ([sheet](../prototypes/crate-grammar.md#class-loadout-rule-1-class-loadout-lmbrmb)). Doubling charges doubles how long the spike lasts, which is the axis Halo had to cut. A super should change **strength or behaviour, not duration**.

## 4. What 1 crate and all 8 should make

### 1 crate: a quick, cheap consumable

**[Synthesis]** These follow from the grammar, not from a source:

- **It is made often, and often by accident.** It is the first step of every held kick: after 0.75 s, "the smallest recipe inside the pad is ready" ([sheet, hold to choose](../prototypes/crate-grammar.md#hold-to-choose-rule-7-hold-longer-to-use-more-crates)). Letting go early opens it, so it will be made on purpose and by mistake.
- **It is the "spend it before it's stolen" valve.** A lone crate on a pad is the easiest steal. The 1-crate result lets a defender cash it in.
- **It must not compete with the class items.** If 1 crate gives as much as a left-click item, nobody builds the pair.
- **Full health is the wrong result.** It turns every crate into a med kit, which undercuts the medic's whole loadout and the heal camps (+60 hp, 40 s respawn; [arena-map sheet](../prototypes/arena-map.md#pve-camps)). It is also the opposite of what Team Buddies did: its single crate gave a light weapon on the pad, and only a small amount of health or ammo off it.

Two families fit, and the candidate sets below try both:

1. **A refill**: top up the charges or ammo of the item you already hold, or give a small heal if you hold nothing. This is the Battlefield Supply Pouch next to the Supply Bag, and Team Buddies' "quickest way to reload". It makes the 1-crate result useful in proportion to the bigger items you already own.
2. **A class-flavoured throwable, used once**: a grenade, a patch kit, a med pack. It is quiet, short-lived and readable (§2 rule 6), and it adds variety without a new system. Throwing it on the same jump-throw arc as a crate ([ADR 0001](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0001-throws-are-jump-timed-and-land-by-curling.md)) reuses the throw skill.

### All 8: the capstone

**[Synthesis]**

- **Cost.** All 8 is about 8 hauls of roughly 12 s each, through a pad the enemy can watch and raid. It should be worth a team fight, like the river objective (320 hp, drops crates; [arena-map sheet](../prototypes/arena-map.md#pve-camps)).
- **Tradition.** Team Buddies' answer is always a vehicle ([disc data](team-buddies-core-loops.md#stack-shape-to-output-category)). A vehicle fits our rules well: it is one character per player, driven by the kicker, so ADR 0004 is satisfied. The prototype's tank already obeys ADR 0003 ("can't carry").
- **Why one tank for every class is wrong.** It flattens the class decision at the top recipe, which is the moment it matters most. ADR 0007's consequence, "Who kicks is a decision", should hold here too.
- **What a capstone is, per class**: the soldier gets a vehicle, the engineer reshapes the map around the pad, and the medic gets a team-wide moment. A team-wide moment follows TF2's ÜberCharge (invulnerability as the medic's "great goal"; [Gravel Pit](https://wiki.teamfortress.com/wiki/Gravel_Pit_developer_commentary) **[Lower confidence]**) and League's Baron.
- **Readability.** A capstone must be loud (§2 rule 5), visible, and capped by time and hp.
- **Watch the 5-crate case.** The hold steps go 1, 2 stacked, 1 × 2, 2 wide × 2 high, 4 flat, all 8 ([sheet](../prototypes/crate-grammar.md#hold-to-choose-rule-7-hold-longer-to-use-more-crates)). Nothing sits between 4 and 8, so crates 5 to 7 are stuck on the pad as targets. That is good tension, as long as all 8 is worth the wait.

## 5. Keeping support classes fun at 5v5

Sources:

- **5v5 gave supports agency.** Overwatch's move to 5v5: in 6v6 supports "had to focus almost exclusively on healing"; in 5v5 they have "more agency to add some damage or duel other heroes". There is also "less damage mitigation and CC" ([Director's Take on 5v5](https://overwatch.blizzard.com/en-us/news/24104605/director-s-take-opening-up-the-conversation-on-5v5-and-6v6/)).
- **Medics now heal themselves by healing others.** Overwatch's Medic sub-role: "Healing allies with your weapon also heals you for 25%" ([Feb 2026 patch notes](https://overwatch.blizzard.com/en-us/news/patch-notes/live/2026/02/)).
- **Supports work as gunners too.** Supercell describes Gus as "a unique support in how much he also functions as a marksman". His rework gives him "agency on if his Spirits are used for offense or defense" ([August 2026 notes](https://supercell.com/en/games/brawlstars/blog/release-notes/release-notes-august-2026/)).
- **Fire-and-forget healing feels invisible.** Alec Dawson said Illari's pylon "can feel invisible throughout a match due to its fire-and-forget automated healing", "can create situations that resemble stalemates", and lacks "the immediate impact and satisfaction" of her other tools ([Season 6 retrospective](https://overwatch.blizzard.com/en-us/news/24003143/director-s-take-retrospective-on-season-6-hero-balance/)). An earlier healing drone was dropped because it "required a ton of management and attention" ([Illari design](https://overwatch.blizzard.com/en-gb/news/24003136)).
- **Strong healing frustrates opponents.** Riot: "satisfying healing mechanics are typically frustrating for opponents to play against". Grievous Wounds "exists as an outlet to prevent healing stacking being an uncounterable strategy" ([Quick Gameplay Thoughts 7/29](https://www.leagueoflegends.com/en-us/news/dev/quick-gameplay-thoughts-7-29/)).
- **Riot prefers shields to heals:** "Too much sustain can be really annoying to fight against and temporary health has more gameplay and timing involved" ([Designing Milio](https://www.leagueoflegends.com/en-sg/news/dev/designing-milio-friends-and-flames/)).
- **Healing is a reaction**, "usually a reaction to enemies trying to kill your teammate," so build-up mechanics on it didn't work ([Quick Gameplay Thoughts 13.10, Riot test mirror](https://pre-blades-bw-testing.leagueoflegends.com/en-us/news/dev/quick-gameplay-thoughts-13-10/)).
- **A must-buy counter is unhealthy.** Hi-Rez said having "one or a very few items… REQUIRED… is generally an unhealthy idea" ([Genlock notes](https://cms.paladins.com/2021/06/23/genlock-update-notes/)). It then made anti-heal a ramping baseline on every weapon "to ensure Supports continue to feel impactful while also facilitating a healthier power curve for healing" ([Season 5 notes](https://cms.paladins.com/2022/01/05/season-5-schism-update-notes/)).
- **Deadlock spreads anti-heal across many items** and tunes it often: Toxic Bullets 65% → 50%, Healbane 45% → 40%, Decay 70% → 55% ([11-21-2024](https://forums.playdeadlock.com/threads/11-21-2024-update.47476/)).
- **Battlefield keeps the full revive for the medic.** Over three games the rule held. Others revive squadmates slowly and partly; the medic revives anyone, faster, to full health ([BF V deep dive](https://www.ea.com/en-gb/games/battlefield/news/battlefield-5-deep-dive-gameplay-changes-and-additions), [BF2042 Briefing](https://www.ea.com/en-gb/games/battlefield/battlefield-2042/news/battlefield-briefing-answering-your-reveal-questions)). BF6: "The Support Class is the only Class that can revive teammates, not just squadmates", and the defibrillator restores "full health on the spot". Headshots make an enemy "unrevivable" ([Support class](https://www.ea.com/en/games/battlefield/battlefield-6/news/support-class), [Battlefield 101](https://ea.com/en/games/battlefield/battlefield-6/news/battlefield-101)).
- **TF2's Medic is built around a partnership.** The Medic is kept "right in the thick of battle. Virtually no aiming is required". The Medi Gun was meant to "build a strong relationship between the Medic and his healing target" **[Lower confidence]** ([Gravel Pit](https://wiki.teamfortress.com/wiki/Gravel_Pit_developer_commentary), [Well](https://wiki.teamfortress.com/wiki/Well_developer_commentary)).

**[Synthesis] For our medic and engineer:**

1. **The medic heals with visible shots that also hurt enemies**, as the prototype's heal gun does (8 hp per shot to an ally, a weak shot otherwise). This is Gus's marksman-support model. It keeps the medic in the fight and obeys ADR 0002, because a heal beam would be a hitscan in disguise.
2. **Prefer shields and burst heals to slow regeneration.** The map's shield camp already uses temporary hp. The medic's right click should be a timed burst, never a heal zone that runs by itself (the pylon problem).
3. **Our anti-heal is built in.** Time to kill is about 1 s (ADR 0002), so healing can't outrun focused fire. Each heal also has to be crafted from crates. No anti-heal item is needed at launch, and the grammar shouldn't grow one (Paladins' lesson).
4. **Revives are the medic's natural capstone, but they touch ADR 0005.** A joined player who is knocked out "stays out until the team opens another player recipe" ([ADR 0005](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0005-players-join-from-the-pad.md)). A medic revive is a second way back into the match. It has to cost at least what a player recipe costs (2 crates per player) and happen away from the pad, or it undercuts that rule. Flag it for the human; don't decide it here.
5. **The engineer's support is space, not health**: walls, cover and pad defence. Everything it deploys must fall under the ADR 0004 ruling below.
6. **Shields vs ADR 0003.** A carrier "drops the crate when hit". **Any** hit should still drop the crate, even one a shield soaks up, or medic and engineer shields quietly arm the haul.

## 6. Ruling: deployables against ADR 0004

ADR 0004: "Each player controls one character, and no player owns or commands AI units." Bots are allowed only as stand-ins for players ([ADR 0004](https://github.com/ALLiDoizCode/big_back/blob/docs/crate-grammar-verdict/docs/adr/0004-one-character-per-player-no-ai-units.md)).

What other games found:

- **Battlefield 2042's sentry was nerfed over and over.** Boris's SG-36 rate of fire went "110 -> 40" ([1.1](https://www.ea.com/games/battlefield/battlefield-2042/news/battlefield-2042-update-notes-1-1)), then 450 → 250 with no more tracking through walls ([4.1](https://www.ea.com/games/battlefield/battlefield-2042/news/battlefield-2042-update-notes-4-1)). It was then steered to target vehicles first "unless infantry is within 25m" ([3.2.0](https://www.ea.com/games/battlefield/battlefield-2042/news/battlefield-2042-update-notes-3-2-0)).
- **Battlefield 6's bots are player-piloted.** Its engineer has an EOD Bot you "take control of", a "remote-controlled miniature war machine" ([Engineer class](https://www.ea.com/en/games/battlefield/battlefield-6/news/engineer-class)), and its recon has a piloted drone ([Recon class](https://www.ea.com/en/games/battlefield/battlefield-6/news/recon-class)). No autonomous sentry appears on the BF6 class pages.
- **TF2 made its sentry "essentially binary"**: lethal to opponents who don't take cover. Classes with indirect fire exist partly to "take out sentryguns around the corner" **[Lower confidence]** ([Hydro](https://wiki.teamfortress.com/wiki/Hydro_developer_commentary), [Gravel Pit](https://wiki.teamfortress.com/wiki/Gravel_Pit_developer_commentary)). The Wrangler swaps the sentry's auto-aim for the Engineer's own aim **[Lower confidence]** ([TF2 wiki: Sentry Gun](https://wiki.teamfortress.com/wiki/Sentry_Gun)).
- **Overwatch's pylon** came "while looking at Torbjörn and Symmetra's turrets" ([Illari design](https://overwatch.blizzard.com/en-gb/news/24003136)). Its fire-and-forget nature then caused the problems in §5.
- **Brawl Stars keeps automatic turrets, but owners only trigger them.** Pam's and Jessie's turrets act by themselves, and the owner's only control is a one-shot Gadget that triggers them ([#BrawlGadgets](https://supercell.com/en/games/brawlstars/blog/release-notes/brawlgadgets-update/)). They are tuned by their own hp and output ([August 2026](https://supercell.com/en/games/brawlstars/blog/release-notes/release-notes-august-2026/)).
- **Paladins' Barik keeps turrets, with a sold counter.** Barik's turrets are stronger while he stands near them ([Genlock](https://cms.paladins.com/2021/06/23/genlock-update-notes/)), and Hi-Rez sells a counter item, Bulldozer: "+Damage to Deployables, Pets, and Illusions" (same source).

**[Synthesis] Ruling.** ADR 0004 bans units that act for a player. The test is whether the thing **decides**: picks a target, aims, or moves by itself.

| Kind | Example | Under ADR 0004 |
|---|---|---|
| **Static object** with no decisions | Shield wall, cover, a ramp, a pad barricade | **Allowed.** It is terrain with hp and a timer |
| **Visible one-shot trap** that fires only on contact | A snare or mine you can see, on the floor | **Allowed**, if it is visible, static, can't be aimed, and is spent on one trigger. It is closer to a thrown grenade than to a unit |
| **Area effect** that pulses on a timer, hitting whoever is inside | Heal zone, slow field | **Allowed by the ADR, discouraged by the evidence**: the pylon problem. Keep it short and burst-like |
| **Slaved device** that fires only when its owner fires, along the owner's aim | A shoulder cannon, or a wall-mounted gun that mirrors your shots | **Allowed.** It is a second barrel on one character. Watch the damage: it doubles the owner's output |
| **Autonomous turret** that picks the nearest enemy and shoots | The prototype's turret drop and the fortress turret | **Not allowed.** It chooses targets for its owner, which makes it an owned AI unit. Revisit ADR 0004 before adding it |
| **Moving or pathing device** | A drone, a healing bot, a hauler bot | **Not allowed** if it moves by itself (ADR 0004 already rejected "team-owned AI haulers"). A player-piloted remote that replaces control of the character for a while is a separate ruling: it would mean two predicted bodies per client, which the netcode budget avoids |

This ruling **removes the prototype's turret drop** (engineer right click) **and the fortress turret**. The shield wall stays.

## 7. What a super crate should change

Sources:

- **Team Buddies' super slot names a separate toy**, often a different one: Baby Rocket Tank → Baby Tandem Tank, Plane → Copter Gunship ([TB research](team-buddies-core-loops.md#stack-shape-to-output-category)). ADR 0007 deliberately chose differently: "a stronger version of the same result". Branching was rejected in play.
- **Overwatch prefers behaviour to numbers.** Minor Perks are "lighter upgrades" and Major Perks are "gameplay-shifting changes" ([Perks spotlight](https://overwatch.blizzard.com/en-us/news/24177197/)). "The number buff perks are some of the perks we like the least" ([Director's Take: AMA](https://overwatch.blizzard.com/en-us/news/24179365/director-s-take-catch-up-on-our-ama/)). Stadium separates "Powers", which are "transformative modifiers", from "Items", which "supplement a Hero's weapons, abilities or overall survivability" ([Enter the Stadium](https://overwatch.blizzard.com/en-us/news/24188046/enter-the-stadium-get-ready-to-battle/)).
- **Brawl Stars Hypercharges mix both.** Each one adds flat stats plus a Super change. Some changes are only bigger (Colt's Super 120% wider). Others are new behaviour (Bull "takes 80% less damage while using his Super") ([patch notes](https://supercell.com/en/games/brawlstars/blog/release-notes/rangerranch-patch-notes-2/)). The design lead accepted that the Super changes "could be more emphasized" **[Lower confidence]** ([Pocket Gamer](https://pocketgamer.com/brawl-stars/frank-keineburg-interview-hypercharge-update/)).
- **Riot keeps stats flat on support upgrades.** It "purposely kept the combat stats the same" so players choose by the passive ([2024 item changes](https://www.leagueoflegends.com/en-gb/news/dev/dev-2024-item-changes/)). Ornn's Masterwork items, a stronger version of the same item, are the closest match to our super crate **[Lower confidence]** ([League wiki](https://wiki.leagueoflegends.com/en-us/Ornn)).
- **Splatoon tunes specials by behaviour, not only damage.** Crab Tank got a 0.33 s attack lockout after switching modes, and Wave Breaker now marks enemies around it ([update history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history)).

**[Synthesis] The rule for supers:** same name, same silhouette, same job, plus **one behaviour twist** and a **moderate number bump**, and no change to how long it lasts (§3).

- The twist should **extend what the item already does**. A wall that blocks also lets allies' shots through. A heal also shields. A rocket also splits. It should not add a second job.
- **Never double charges or duration.** Prefer 1.25 to 1.5× strength.
- **The pad readout names the twist** ("SUPER shield wall: allies shoot through"). That keeps the enemy's read intact.

---

## 8. Candidate item sets

Common to all three sets **[Synthesis]**:

- **Base gun for everyone:** 25 damage, 3 shots/s, 18 m/s (ADR 0002).
- **Left click replaces the gun** until its ammo runs out.
- **Right click works with empty hands only**, using charges. While carrying, right click still throws (ADR 0003).
- **Items go to the kicker** and are lost on knockout.
- **All 8 has a team lockout** (§3): a team can't open another all 8 until its current one has ended.
- **No result lets a carrier fire** (ADR 0003). **Every shot is a visible projectile** (ADR 0002). **No result picks its own targets or moves by itself** (§6).
- **Super:** one behaviour twist, about 1.25 to 1.5× strength, never more charges or more time.

Numbers are starting points for a prototype, not tuning.

### Set A: "Fix the placeholders" (closest to the current prototype)

Keeps the prototype's left- and right-click items wherever ADR 0004 allows them. Replaces the turret, the full-health crate and the shared tank.

| | Left click (1 × 2) | Right click (2 × 2 flat) | 1 crate | All 8 |
|---|---|---|---|---|
| **Soldier** | **Rapid gun**: 6 shots/s, 60 rounds | **Rockets**: 4 charges, 14 m/s, 2.5 m blast, knock loose crates | **Grenade**: 1, thrown on the jump-throw arc, 2 m blast after 1.5 s | **Tank**: 45 s or 300 hp, rocket cannon, can't carry, 0.8× speed |
| *super* | Shots pierce one target | Each rocket splits into 3 small blasts on impact | Sticks to the first thing it touches | Cannon blast leaves a 3 s slow zone; 450 hp |
| **Engineer** | **Rivet gun**: 2 shots/s, 30 damage, 40 rivets; a hit on a friendly wall repairs it 20 hp | **Shield wall**: 2 charges, 4 m wall where you aim (up to 8 m), 250 hp, 20 s | **Patch kit**: refills your right-click charges by 1, or puts 60 hp back on your nearest wall | **Bunker**: a horseshoe of walls round your pad (600 hp, 40 s) with one gap |
| *super* | Rivets also stagger (0.2 s) | Allies' shots pass through it | Refills 2 charges | The gap is one-way: your team can walk out, enemies can't walk in |
| **Medic** | **Heal gun**: 8 hp per shot to an ally, 10 m; a weak shot to enemies; 80 shots | **Heal pulse**: 3 charges, +40 hp to you and allies within 7 m | **Med pack**: 1, thrown, +40 hp to whoever picks it up | **Rally**: every living teammate gets full health and a 50-point shield; the medic is invulnerable for 4 s |
| *super* | Heals bounce to a second ally within 4 m | Pulse also gives a 25-point shield | +40 hp and a 25-point shield | Shield is 100, and the rally also clears slows |

**Main risk: the engineer's left click is weak, and the bunker may stall the game.** A rivet gun without the turret is a plain gun with a repair add-on, so engineers may skip the pair and save for walls. The bunker's one-way gap could also make a pad unraidable for 40 s, which works against "steal only" (ADR 0007). It needs play to see whether 40 s is a tense siege or a dead pause. Two smaller risks: Rally overlaps the heal camp, and the medic is weak until crafting.

### Set B: "Team Buddies ladder" (1 crate refills, all 8 is always a vehicle)

The closest to the original's spirit: 1 crate tops up what you hold (TB: "the quickest way to reload"), and every class's all 8 is a vehicle with its own job. Every result is a weapon or a vehicle, and each vehicle is driven by its kicker (ADR 0004).

| | Left click (1 × 2) | Right click (2 × 2 flat) | 1 crate | All 8 |
|---|---|---|---|---|
| **Soldier** | **Shotgun**: 5 pellets, short range, 12 shells | **Rockets**: 4 charges | **Refill**: tops up your held items (left click to full, right click +1); +25 hp if you hold nothing | **Tank** (as Set A) |
| *super* | 7 pellets with a tighter spread | Rockets split into 3 | Refill to full on both | 450 hp, slow zone |
| **Engineer** | **Nail-mine layer**: each click drops a visible floor mine (one-shot, 40 damage, 1 s root), 6 mines, max 3 out | **Shield wall**: 2 charges | **Refill** | **Hauler mech**: 40 s or 400 hp, carries up to 3 crates at 0.8× speed, **no weapon**, stomps for knockback |
| *super* | Mines also reveal the victim on the minimap for 3 s | Allies shoot through | Refill to full | Carries 4, and its stomp knocks crates off an enemy carrier at 3 m |
| **Medic** | **Heal gun** (as Set A) | **Heal pulse** (as Set A) | **Refill** | **Field ambulance**: 40 s or 350 hp; allies within 5 m heal 15 hp/s; a weak blaster |
| *super* | Bouncing heals | Pulse gives a shield | Refill to full | Ambulance also gives a 25-point shield every 5 s |

The hauler mech is the class-flavoured Team Buddies vehicle. In TB, vehicles collect crates and the Mech can tow megacrates ([TB vehicles](team-buddies-core-loops.md#vehicles)). Here, as a carrier, it can't fire, so ADR 0003 holds.

**Main risk: three vehicles on a 90 × 50 m map, and an aura that heals by itself.** The ambulance is the "fire-and-forget" healing Blizzard found invisible and prone to stalemates. A hauler mech that carries 3 to 4 crates may also break the haul rhythm the crate economy depends on. A team could chain all 8 into the next all 8, even with the lockout. A smaller risk: mines are allowed by the ruling, but they must stay readable on a busy floor.

### Set C: "Each class its own capstone" (1 crate is a class buff, all 8 is a team moment)

Behaviour first, following Overwatch's "number buffs are the perks we like least". Each class's 1 crate is a short, quiet self-buff, and its all 8 is a loud team-wide moment in that class's job: firepower, space or survival.

| | Left click (1 × 2) | Right click (2 × 2 flat) | 1 crate | All 8 |
|---|---|---|---|---|
| **Soldier** | **Rapid gun** | **Grenade launcher**: 3 charges, lobbed on a visible arc, bounces once | **Adrenaline**: 6 s of 1.3× fire rate (cancelled if you pick up a crate) | **Barrage**: you mark a 10 m circle; 3 s later, 12 visible shells fall over 4 s. You fire nothing while marking |
| *super* | Shots pierce | Grenades split on the bounce | 8 s, and reload on kill | Shells leave a slow zone |
| **Engineer** | **Rivet gun** with wall repair | **Shield wall**: 2 charges | **Barricade**: a 2 m crate-high block (150 hp, 20 s) where you aim | **Gate**: two long walls (8 m, 500 hp, 45 s) that shut one jungle entrance or lane choke; your team can pass, enemies can't |
| *super* | Rivets stagger | Allies shoot through | Barricade can be climbed by allies (a jump step) | Enemies touching the gate are slowed |
| **Medic** | **Heal gun** | **Shield burst**: 3 charges, a 40-point shield on an ally you aim at (10 m projectile) | **Self-patch**: +30 hp now, +30 more over 3 s | **Second wind**: brings back every knocked-out joined player at the medic's position, at half health, then the medic can't heal for 10 s |
| *super* | Bouncing heals | The shield also cleanses slows | +30 and a 25-point shield | They come back at full health |

**Main risk: the medic's capstone rewrites ADR 0005, and the barrage is hard to dodge.** Second wind is a second way to bring back joined players, away from the pad. At 8 crates for up to 4 players it costs the same as player recipes (2 crates each), but it skips their location and their readout. It needs a human ruling before a prototype. The barrage is visible and telegraphed, which satisfies ADR 0002, but 12 shells in 4 s may still feel like a no-dodge zone. And with the soldier's 1 crate a pure buff, nothing in this set refills ammo, so the left-click items may run dry before anyone reaches the pad again.

### Comparison

| | Set A | Set B | Set C |
|---|---|---|---|
| 1 crate | Class throwable | Universal refill | Class self-buff |
| All 8 | Vehicle / fortification / team heal | A vehicle for every class | A team-wide moment per class |
| Closest precedent | The prototype | Team Buddies; BF6 Supply Pouch | Overwatch Perks; TF2 ÜberCharge; League Baron |
| Biggest ADR question | ADR 0007 steal only (bunker) | ADR 0003 hauling (mech) | ADR 0005 joining (second wind) |
| Easiest to prototype | **Yes**: three items change | Medium: two new vehicles | Hardest: barrage, gate and revive are new systems |

**[Synthesis] Suggestion for the next prototype.** Start from **Set A** with **Set B's 1-crate refill** swapped in for one run. That compares a throwable with a refill directly. Then try **one** capstone from each other set (the hauler mech, then second wind) as single swaps. Change nothing else, so each verdict stays readable.

---

## Gaps and unverified points

- **Overwatch:** no primary source says ultimate voice lines are meant as warnings to enemies, or states plainly how ultimate charge is earned. Patch notes only imply it.
- **Splatoon:** how much special gauge you lose on being splatted, and the HUD cues for a ready special, are only on community sites. Inkipedia returned 403, so this is not stated in the doc.
- **Brawl Stars:** the "3 gadget uses per match" baseline is only implied by the Gadget Gear's "3 → 4". There is no official text on how players can tell a Hypercharge is active.
- **Battlefield:** BF6 vehicle caps per team and spawn timers, and BF3, BF4 and BF1 medic and engineer details, were not found on official pages. BF6 news pages show republish dates (April and July 2026) that may not be the originals.
- **Paladins:** no primary page for Resilience or Nimble values, or for Inara and Torvald deployables.
- **Deadlock:** the current item prices (800 / 1,600 / 3,200 / 6,400) come only from the community wiki. No official patch for the change from 500 / 1,250 / 3,000 / 6,200 was found. Valve has published no statement separating behaviour-changing items from flat-stat ones.
- **League:** no Riot post gives the design purpose of Baron.
- **TF2:** developer commentary and sentry metal costs come from the community TF2 wiki. teamfortress.com update pages are image-only or 404.
- **Halo:** no Bungie or 343 statement on why power weapons sit where they do, beyond the Waypoint notes cited.
- **Hades:** a "boons should change how you play" quote from Supergiant was looked for and not found, so it is not used.
