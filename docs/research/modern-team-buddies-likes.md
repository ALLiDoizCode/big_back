# Modern games like Team Buddies: core loops and what makes them fun

Research notes comparing games from about 2010 onward with Team Buddies (PS1, 2000), to inform the design of a new game. Compiled 2026-10-18. The reference breakdown of Team Buddies itself is [`team-buddies-core-loops.md`](team-buddies-core-loops.md); "TB" below means Team Buddies, and TB mechanics are named the way that file names them (pad, stack shape, super crate, decapitation, and so on).

## How to read this

Every claim carries a source tag. The tag says how far to trust the claim:

| Tag | Meaning | Confidence |
|---|---|---|
| **[P]** | Primary: the developer or publisher speaking for itself. This covers Steam announcements and patch notes, official version histories, official blogs, store pages, Nintendo "Ask the Developer" interviews, and PlayStation Blog or Xbox Wire posts written by the studio | **Highest** for design intent and for what changed. Store copy is promotional |
| **[I]** | A named developer quoted in a press interview (Game Developer, Game Informer, Red Bull, Push Square and similar) | High for what the developer said. The outlet chose and framed the quote |
| **[R]** | Review or press opinion, or a press summary of facts | **Secondary signal.** Used only for reception and for facts with no primary source |
| **[W]** | Wiki, guide, Steam forum or Reddit | Low. Use only to find leads, or for numbers no official source states |
| **unverified** | A detail recalled or inferred that I could not find on any page | Treat as a hypothesis |

Two caveats apply to the quotes:

- **Steam patch notes** were read in full through Steam's news API, so [P] quotes from them are exact. Links use the form `store.steampowered.com/news/app/<app>/view/<post>`; the post IDs came from that API.
- **Press-interview quotes** ([I]) were read through a page-summarising fetch tool. They should be close to verbatim but not character-perfect. Recheck any line before quoting it in public.

Sections marked **Analysis** are my own reasoning, not a source's.

### A market fact first: Team Buddies is being remastered

There is an official *Team Buddies* Steam page from Kaeru Interactive, with Hiroshi Shimizu also credited as developer. The release date is "Coming soon". It lists online PvP, cross-platform multiplayer and a "New Reborn Mode (Online)" with "larger maps, an expanded weapon selection, and even more frenetic combat"; local multiplayer is not mentioned ([P] [Steam app 3521810](https://store.steampowered.com/app/3521810/Team_Buddies/)). A fan remake in UE4 was abandoned, and its developer said they moved on to a project "directly inspired" by Team Buddies; I couldn't find that project's name ([W] [IndieDB](https://www.indiedb.com/games/team-buddies-fan-remake)). A PC *Team Buddies 2* was cancelled ([W] [Unseen64](https://www.unseen64.net/2019/02/01/team-buddies-2-pc-cancelled/)). No indie "spiritual successor" turned up in searches of Steam, itch.io, Kickstarter or Reddit.

**Analysis:** a successor will be compared with the remaster. The remaster's page says nothing about couch play, which leaves local split-screen as open ground.

### The six Team Buddies pillars used for matching

| Code | Pillar |
|---|---|
| **H** | Physical resource hauling (you carry crates, and while carrying you can't shoot) |
| **B** | Building units or items from resources mid-match (stack shape on the pad picks the output) |
| **S** | Small-squad command (up to 4, possessable) |
| **C** | Chaotic local or online multiplayer with friendly-fire comedy |
| **R** | Base raiding and stealing; destroying a base stops its production |
| **P** | Crate or physics stacking |

---

## Candidate evaluation

Twelve games get a full breakdown. Every candidate from the brief, plus the extra matches I found, is judged below.

### Included (full breakdowns follow)

| # | Game | Pillars | Why it's in |
|---|---|---|---|
| 1 | **Pikmin 4** (2023), with Pikmin 3 Deluxe's Bingo Battle | H, B, S, R | Squad hauls loot to your base, which produces more squad; the Sneak Bomb steals from the rival's base. The closest overall match |
| 2 | **Helldivers 2** (2024) | H, B, S, C | Weapons and vehicles called down mid-mission; always-on friendly fire; some carried objectives restrict your weapon |
| 3 | **Deep Rock Galactic** (2020) | H, S, C | Carrying heavy objects blocks your weapons; pooled resource buys a supply drop; an AI drone squadmate for solo |
| 4 | **Overcooked 2** (2018) and *All You Can Eat* | H, B, C | Carry, throw, combine inputs into an output under time pressure; the template for couch chaos |
| 5 | **Moving Out** (2020) / **Moving Out 2** (2023) | H, C, P | Mass-based physics hauling, two-person carries, co-op throws, packing a truck |
| 6 | **Unrailed!** (2020) | H, B, C | Haul raw resources to a crafter wagon that outputs track; 2v2 versus |
| 7 | **Lethal Company** (2023) | H, C | Haul-or-lose loot under weight and hand limits; the developer revised the carry rules |
| 8 | **R.E.P.O.** (2025) | H, C, P | Physics hauling where damage costs value; shared carries; the best modern reference for comic hauling |
| 9 | **Sea of Thieves** (2018) | H, S, C, R | Carrying treasure disarms you; stealing other crews' loot; the ship is base and vault |
| 10 | **Splatoon 3** (2022): Salmon Run and Rainmaker | H, C, R | Egg hauling to a basket that enemies raid; Rainmaker replaces the carrier's weapon |
| 11 | **Ultimate Chicken Horse** (2016) | B, C, R | Building mid-match from a drafted pool; sabotage scoring; a documented comeback-scoring redesign |
| 12 | **Tricky Towers** (2016) | B, C, R, P | Physics stacking where stack shape is the outcome; spells to sabotage rival stacks |

### Short entries (relevant to one pillar; not given full breakdowns)

| Game | Pillars | What's useful | Source |
|---|---|---|---|
| **Gang Beasts** (EA 2014, 1.0 2017) | C, H | Grab, lift, throw, with the *player* as the cargo. Developer James Brown: "if we could get a satisfying punch mechanic, we could build a game around it" and "a big part of the fun is coming back after your opponent thinks he's been victorious". He also said "Didn't have an AI at that time, so we just went local multiplayer." Online took years of Early Access; v0.5.0 (Dec 2016) declared it "sufficiently stable and scalable" | [I] [bit-tech p.1](https://bit-tech.net/reviews/gaming/pc/interview-gang-beasts/1/), [p.2](https://bit-tech.net/reviews/gaming/pc/interview-gang-beasts/2/); [P] [0.5.0 notes](https://gangbeasts.game/posts/2016/12/21/hgxgssm2b4n7me03ixuk3eg8b4443o); [P] [Steam](https://store.steampowered.com/app/285900/) |
| **Fall Guys** (2020): Egg Scramble / Egg Siege rounds | H, R, C | Teams grab eggs, carry them to their nest, and steal from rival nests; golden eggs are worth 5. The purest modern "raid the rival's pad" loop | [R] [Gfinity](https://www.gfinityesports.com/article/fall-guys-season-2-egg-siege-level-guide-tips-tricks-and-more-pc-ps4) |
| **Halo Infinite CTF** | H, R | The long-standing Halo rule: the flag carrier can't fire but can melee with the flag. In Infinite the flag melee doesn't one-shot a full-shield player | [W] [Halopedia](https://www.halopedia.org/Capture_the_flag) |
| **Mindustry** (2019) | B, R | Conveyors feed turrets and unit factories; PvP attacks enemy cores. Production destroyed means army stopped, but hauling is automated | [P] [Steam](https://store.steampowered.com/app/1127400/Mindustry/) |
| **Foxhole** (1.0 2022) | H, B, R | "Every bullet, weapon, vehicle… is produced and supplied by real players"; you win by cutting supply lines. TB's economy at MMO scale | [P] [Steam](https://store.steampowered.com/app/505460/Foxhole/) |
| **Teardown** (multiplayer 2026) | P, H, C | Up to 12 players co-op in voxel-destruction heists, plus Deathmatch, TDM, CTF and "Treasure Thieves" (rules unverified). Multiplayer was announced Mar 2025 and reportedly arrived Mar 2026 | [P] [announcement](https://gamespress.com/WERE-PUTTING-A-CREW-TOGETHER-MULTIPLAYER-COMES-TO-TEARDOWN-ON-STEAM-LA); [R] [KitGuru](https://www.kitguru.net/tech-news/mustafa-mahmoud/physics-based-sandbox-teardown-gets-12-player-multiplayer-and-much-more-in-new-update/) |
| **Starship Troopers: Extermination** (1.0 2024) | H, B | Up to 16 players build refineries, walls and towers; reviewers describe carrying full ore canisters back to base. PvE only | [P] [Steam](https://store.steampowered.com/app/1268750/Starship_Troopers_Extermination/); [R] [A Little Bit Human](https://www.alittlebithuman.com/starship-troopers-extermination-would-you-like-to-know-more/) |

### Rejected

| Game | Reason | Source |
|---|---|---|
| Totally Accurate Battle Simulator | Units are bought from a budget *before* the battle; no hauling, no mid-match building. Only the physics comedy matches | [P] [Steam](https://store.steampowered.com/app/508440/) |
| Peak (2025) | 4-player co-op climbing with proximity chat and a backpack-weight stamina cost ([W] guide), but no combat, base or production | [P] [Steam](https://store.steampowered.com/app/3527290/PEAK/) |
| Content Warning (2024) | 4-player co-op filming monsters. Comedy, but nothing is hauled to a base or built | [P] [Steam](https://store.steampowered.com/app/2881650/) |
| Party Animals (2023) | Physics brawler like Gang Beasts but with less design writing available; Gang Beasts covers the same lesson | [P] [Steam](https://store.steampowered.com/app/1260320/Party_Animals/) |
| PlateUp! (2022) | Co-op restaurant roguelite; a weaker version of Overcooked's lessons, plus between-day layout building | [P] [Steam](https://store.steampowered.com/app/1599600/) |
| Lovers in a Dangerous Spacetime (2015) | 1–4 couch co-op running between ship stations; combining gems changes the loadout, a light echo of "the combination picks the weapon". Close but no hauling | [P] [Steam](https://store.steampowered.com/app/252110/Lovers_in_a_Dangerous_Spacetime/) |
| Battlerite | Arena brawler with no resources | [P] [Steam](https://store.steampowered.com/app/504370/) |
| Northgard | RTS with an abstract economy; no hands-on hauling or squad possession | [P] [Steam](https://store.steampowered.com/app/466560/) |
| Satisfactory | Co-op factory automation with no opponent | [P] [Steam](https://store.steampowered.com/app/526870/) |
| Worms Rumble | Real-time arena shooter; no economy | [P] [Steam](https://store.steampowered.com/app/1186040/Worms_Rumble/) |
| Brawlhalla | Platform fighter; only the local-chaos pillar matches | [P] [Steam](https://store.steampowered.com/app/291550/) |
| Clustertruck | Single-player physics platformer | [P] [Steam](https://store.steampowered.com/app/397950/) |
| Splitgate | One press article describes a ball-carry mode where the carrier can't shoot ([R] [Digit](https://www.digit.in/features/gaming/splitgate-taking-fps-to-the-next-level-63547.html)). Halo CTF covers the same rule with better sourcing | — |
| Hell Let Loose | Truck-and-node supply logistics for 50v50 ([W] [Steam forum](https://steamcommunity.com/app/686810/discussions/0/5168416410415154455)). The scale and pace are far from a 4-unit squad | — |
| Rocket League | 3v3 car football; no economy | [P] [Steam](https://store.steampowered.com/app/252950/) |

Considered but not covered: Brawl Stars *Gem Grab* (carried gems drop on death; 10 held gems start a 15-second win countdown; [W] [Wikipedia](https://www.Wikipedia.org/wiki/Brawl_Stars)), *Rubber Bandits* (2021, 1–4 player physics heists; [W] [MobyGames](https://www.mobygames.com/game/176295)), *Looters* (2026, 2-player split-screen goblin heist where goblins stack on each other to climb; [P] [Steam](https://store.steampowered.com/app/4140170/Looters/)), and *AirMech* (your mech picks up and drops units; [P] [Steam](https://store.steampowered.com/app/206500/)).

**No modern game found combines "stack shape on a pad decides the output" with the other pillars.** Tricky Towers comes closest on the stacking side, and Pikmin 4 on the economy side.

---

## 1. Pikmin 4 (Nintendo, Switch, 2023), including Dandori Battle

| Players | Local / online | Match length | Platform |
|---|---|---|---|
| 1–2 on one system ([P] [store page](https://www.nintendo.com/us/store/products/pikmin-4-switch/)) | Local only; no online | Story days are 15 min ([W] [Pikmin Wiki](https://www.pikminwiki.com/Pikmin_4)); Dandori Battles 4–7 min by stage ([W] [Pikmin Wiki](https://www.pikminwiki.com/Dandori_Battle)) | Switch |

Whether two-player Dandori Battle uses split-screen is **unverified**; Nintendo only says "two players on the same system".

### Loops

- **Moment to moment:** whistle Pikmin, throw them, split the squad, ride or command the dog Oatchi, and let the Pikmin carry treasure, pellets and corpses to the ship or Onion. Oatchi carries the leader and up to 60 Pikmin, and can handle a separate task when you split up ([P] [store page](https://www.nintendo.com/us/store/products/pikmin-4-switch/)).
- **Per run:**
  - Story: one 15-minute day in one area; time runs at 1/6 speed in caves ([W]).
  - Dandori Challenge: a timed "collect every treasure" level.
  - Dandori Battle: a timed versus match. You can play P1 vs CPU, P1 vs P2, or P1 and P2 vs CPU, with P2 using a "Pebble Pitcher" cursor ([W] [Pikmin Wiki](https://www.pikminwiki.com/Dandori_Battle)).
- **Meta:** treasure becomes Sparklium at day's end, which unlocks new areas ([W] [Pikmin Wiki](https://www.pikminwiki.com/Sparklium)). Rescued castaways fill a hub and pay raw materials for gear; Oatchi is trained. Ver. 1.1.0 (Nov 2025) added "Roll Over", a new playthrough that keeps some save data ([P] [version history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/62986)).

### Parallels to Team Buddies (Dandori Battle)

All rules here are [W] [Pikmin Wiki](https://www.pikminwiki.com/Dandori_Battle) unless noted.

| Pikmin 4 | TB equivalent | Notes |
|---|---|---|
| Squad carries treasure to your Onion, scoring its weight in points; Bonus Finds multiply value 2–5× | Hauling crates to the pad | Here the *units* haul, not the leader. The leader never carries |
| The Onion produces live Pikmin, up to 50 per player | A vertical stack produces a Buddy | Production is a side effect of hauling, with no shape choice |
| **Sneak Bomb**: carried into the *rival's* Onion, it costs them points in proportion to their score, capped at 100. The lost points spill out as "Dandorium" weights that **either team** can carry home | Stealing from or smashing the rival pad | **Best modern version of the TB raid.** Theft becomes a scramble on the map, not a silent transfer |
| You can't whistle rival Pikmin; contested objects become a tug-of-war, and Oatchi's Rush knocks rival Pikmin off | Intercepting a crate carrier | — |
| Mystery Capsules drop about every 30 s and give a random item (Quick Pik +10, Lightning Shock, Mine, Rock Storm, Warp, Bewilder Bomb and others) | Parachuted crates; off-pad crates as consumables | — |
| Leader and dog split the squad between tasks | TB's binocular orders to one Buddy or the whole team | — |

**Pikmin 3 Deluxe Bingo Battle** (all [W] [Pikmin Wiki](https://www.pikminwiki.com/Bingo_Battle)): 2 players, local. You win by completing a bingo line of four items carried to your Onion, **or by stealing the rival's Victory Macaroon and carrying it to your own Onion**, or by wiping out the rival's Pikmin or knocking out a leader. Onions release seeds up to 50, and Pikmin killed by the rival respawn at the Onion. Carrying a Cupid's Grenade home gives an item spin. At 8 minutes a "Give it a spin!" event starts. The Macaroon is TB's "steal the crate from their pad" made into a win condition.

### What makes it fun (developers)

From Nintendo's *Ask the Developer* Vol. 10 ([P] [Part 1](https://www.nintendo.com/sg/interview/ampy/index.html), [Part 2](https://www.nintendo.com/sg/interview/ampy/02.html)):

- Miyamoto: "The fun in this franchise's gameplay lies in players setting their own goals and overcoming them as they go along", and "Games are fun because of the continual trial and error."
- Miyamoto: "The gameplay of the Pikmin franchise isn't about competition." Dandori Battle exists anyway. Kando explains that "the versus mode in the second and third games required two players to play", so they "implemented a CPU battle mode so that it can be enjoyed in single player."
- Kando: "Pikmin games are not only about fighting but also about finding and collecting things", and "task management games give a sense of accomplishment that even people who don't play games can relate to."
- Kando on the dog: "Oatchi deepens the strategic nature of the game." Hiramuki: "we were able to solve two problems at once with a single idea."
- Hiramuki on onboarding: "We thought it was more important to lower the barrier to entry for new players." On the rewind feature, they were "careful not to make them feel like they're cheating."
- Hino: Pikmin 1's 30-day limit meant a failed run restarted from scratch, which "felt too long for some people", so Pikmin 2 removed the time limit.

### Frustrations and responses

- **Reception** ([R], secondary): broadly praised. Repeated complaints were that player 2 in story co-op "lacks any real agency" ([R] [Destructoid](https://destructoid.com/reviews/review-pikmin-4)), that multiplayer is local only, and that it is easy. Dandori Battle was called "better than nothing" and a possible "wasted opportunity" ([R] [Prima](https://primagames.com/reviews/pikmin-4-review-bigger-than-it-needed-to-be)).
- **Patches** ([P] [version history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/62986)): 1.0.2 (Sep 2023) fixed a bug that let players inflate their Pikmin count and added a loading bar. 1.1.0 (Nov 2025) added creature activity levels (Relaxed / Normal / Fierce), an Auto Target Lock toggle, a "Precise" aim speed and a skippable intro. Reading "Fierce" as an answer to the difficulty complaint is my interpretation; the notes give no reason. **No patch has touched Dandori Battle balance or added online play.**

---

## 2. Helldivers 2 (Arrowhead / PlayStation Publishing, 2024)

| Players | Local / online | Mission length | Platform |
|---|---|---|---|
| "squads of up to four" ([P] [Steam](https://store.steampowered.com/app/553850)) | Online co-op only; no split-screen | About 40 min for most missions, about 10–12 min for Blitz ([R] [Dot Esports](https://dotesports.com/helldivers-2/news/all-missions-in-helldivers-2)) | PS5 and PC (Feb 2024); Xbox Series X\|S from Aug 2025, with crossplay ([R] [Push Square](https://www.pushsquare.com/news/2025/07/playstation-is-bringing-helldivers-2-to-xbox-out-on-26th-august)) |

Sony reported 12M copies in 12 weeks, the fastest-selling PlayStation Studios game ([R] [GameSpot](https://www.gamespot.com/articles/helldivers-2-is-fastest-selling-playstation-game-ever/1100-6523423/)).

### Loops

- **Moment to moment:** shoot; enter a stratagem's directional code; throw the beacon; the strike or supply drop lands where it fell. Each player brings up to 4 stratagems, and Reinforce and Resupply are always available ([W] [Stratagems](https://helldivers.wiki.gg/wiki/Stratagems)).
- **Per mission:** drop in, do the main and side objectives, gather samples, call extraction. Samples count only if "someone must be carrying them when boarding the extraction shuttle" ([W] [Samples](https://helldivers.wiki.gg/wiki/Samples)). An operation chains missions, and failing one fails the operation ([W] [Missions](https://helldivers.wiki.gg/wiki/Missions)).
- **Meta:** requisition, medals and samples buy stratagems, Warbonds and ship modules. The Galactic War is a shared community campaign ([P] [Steam](https://store.steampowered.com/app/553850)). In June 2026 Major Orders became 1–3-week campaigns, with game director Mikael Eriksson admitting "it's never been clear if the actions of the community actually lead to different outcomes" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/1834602721187299)).

### Parallels to Team Buddies

| Helldivers 2 | TB equivalent | Notes |
|---|---|---|
| **Stratagems**: weapons, turrets and vehicles delivered by beacon | Kicking open a finished stack to get a toy | The cost is a code input plus a cooldown, not hauled crates. Same *output* categories (weapon, vehicle), faster delivery |
| SSSD drives carried one-handed, which restricts you to one-handed weapons ([W] [Recover SSSD](https://helldivers.wiki.gg/wiki/Recover_SSSD)); two-handed items (barrels, artillery shells you haul to the gun) | "Can't shoot while carrying" | **A graded carry penalty:** one-handed loads allow a sidearm, two-handed loads don't |
| Samples are lost unless someone carrying them boards the shuttle | Crates are worthless until they reach the pad | — |
| Destroying bug nests and Automaton Fabricators | Destroying a base to stop production | Enemies are AI factions, not rival players |
| Friendly fire always on, including airstrikes | Splash damage hurts your own side | — |
| Vehicles for the squad (FRV, Exosuits, a 4-seat tank) | The 8-crate vehicle | "Bring the whole squad; that FRV isn't going to drive itself" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/1839041357026952)) |

**Gap:** no PvP, so no stealing. Johan Pilestedt ruled out PvP to keep the community "fighting on the same side" ([R] [Mein-MMO](https://mein-mmo.de/en/helldivers-kein-pvp-weil-toxisch,1091280)).

### What makes it fun (developers)

From Pilestedt's PlayStation Blog interview, 12 Mar 2024 ([P] [PlayStation Blog](https://blog.playstation.com/2024/03/12/helldivers-2-how-arrowhead-turned-the-twin-stick-shooter-original-into-a-bombastic-ps5-sequel/)):

- "A good co-op game forces players to play together, but a great one makes players want to play together"
- "Things should be consistent in the game world and therefore, we must have friendly fire."
- "the chuckles that happen when you accidentally carpet bomb your friends allows for a playfulness"
- "Generally, stratagems are based on real world examples." and "We play the game and get bored by the current selection, so we add another one."

The store page puts it as: "friendly fire is an unfortunate certainty of war, but victory without teamwork is impossible" ([P] [Steam](https://store.steampowered.com/app/553850)). The stated balance goal is "challenging yet fair, rewarding your strategic thinking and teamwork", and a bug that let a missile destroy Fabricators from any angle was left in because "the community clearly voiced their enjoyment" ([P] [Patch 1.001.002 explained](https://store.steampowered.com/news/app/553850/view/5963413728354471629)).

The often-quoted studio motto "A Game for Everyone is a Game for No One" has **no primary source** I could find; it appears only in secondary blogs and forum threads ([W]).

### Frustrations and responses

1. **Weapon nerfs and the August 2024 backlash.** Eriksson wrote that "we didn't hit our target... what matters most now is action. Not talk", and set out a 60-day plan whose first principle was that "balance should be fun, not 'balanced' for the sake of balance" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/5963414363409216198)). Patch 1.001.100 then reworked armour penetration, raising partial damage from 50% to 65% so non-specialist weapons could hurt heavy enemies ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/6250522009530732352)). The wrap-up reported "a comprehensive balancing pass on over 70 weapons and stratagems" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/6211118320240881542)).
2. **Carry penalty too harsh.** Patch 01.002.101 (Feb 2025) began "allowing Helldivers to jog while carrying two-handed items" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/1790214123239446)).
3. **PSN account requirement** (May 2024). Linking was framed as "our main way to protect players from griefing", then reversed: it "will not be moving forward" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/5766372999945329774)). Press reported a flood of negative reviews ([R] [VGC](https://www.videogameschronicle.com/news/playstation-u-turns-after-helldivers-2-steam-backlash-scraps-mandatory-psn-account-linking/)).
4. **Stability** (Sep 2025): "our recent updates haven't hit the mark, and our silence hasn't helped. That's on us", and a 60-day patch "would be duct tape where we need to solder" ([P] [Steam news](https://store.steampowered.com/news/app/553850/view/1811772772210855)).

---

## 3. Deep Rock Galactic (Ghost Ship Games / Coffee Stain, EA 2018, 1.0 2020)

| Players | Local / online | Mission length | Platform |
|---|---|---|---|
| "1-4 players" ([P] [Steam](https://store.steampowered.com/app/548430)) | Online co-op; solo with an AI drone; no split-screen | Commonly 20–40 min; Deep Dives run 3 stages ([W], unverified) | PC, Xbox (2018); PS4/PS5 (2022) ([P] [Steam news](https://store.steampowered.com/news/app/548430/view/4224935490657239680)) |

### Loops

- **Moment to moment:** mine and dig with class tools (Scout, Gunner, Engineer, Driller), fight swarms in fully destructible procedural caves ([P] [Steam](https://store.steampowered.com/app/548430)). A resupply pod costs 80 Nitra from the team pool, and its landing damages dwarves too ([W] [Resupply Pod](https://deeprockgalactic.wiki.gg/wiki/Resupply_Pod)).
- **Per mission:** meet the primary quota (Morkite, Aquarq, eggs and so on), do the secondary objective, then run to the drop pod. In 2017 Søren Lundgaard described the primary objective as "always the same: collect X Morkite" ([I] [GamingBolt](https://gamingbolt.com/deep-rock-galactic-interview-heigh-ho-off-to-plunder-we-go/amp)).
- **Meta:** credits and minerals buy upgrades, overclocks, perks and "thousands of cosmetics". Seasons are free: "No time-limited battlepasses, no FOMO shop rotations, no gated content" ([P] [Steam](https://store.steampowered.com/app/548430)). Hazard levels 1–5, plus Hazard 5+ with adjustable modifiers ([P] [Steam news](https://store.steampowered.com/news/app/548430/view/6890025005348584366)).

### Parallels to Team Buddies

| DRG | TB equivalent | Notes |
|---|---|---|
| Carrying a heavy object: you "cannot do anything but throw flares, ride ziplines, and move with a 25% speed penalty. Sprinting is also disabled." You can throw it, or drop it by switching weapons ([W] [Alien Egg](https://deeprockgalactic.wiki.gg/wiki/Alien_Egg)) | "Can't shoot while carrying"; throwing crates | **Direct match**, including the throw |
| Aquarq carried by hand to the Mine Head; minerals deposited into the M.U.L.E. robot (M.U.L.E. deposit is [W], unverified) | The pad | Output is a quota, not a recipe |
| Nitra resupply pod: a pooled resource spent on a drop | Spending crates on an output | Only health and ammo, not new units |
| Friendly-fire multiplier 0.10 at Hazard 1, 0.40 at Hazard 4, 0.70 at Hazard 5 ([W] [Hazard Level](https://deeprockgalactic.wiki.gg/wiki/Hazard_Level)) | Splash damage hurts your own side | **Friendly fire as a difficulty dial** |
| **Bosco**, the solo drone: mines, digs, throws carryables to you, revives you, fires rockets; leaves when a second player joins ([W] [Bosco](https://deeprockgalactic.wiki.gg/wiki/Bosco); [P] [Update 7](https://store.steampowered.com/news/app/548430/view/4249665521686926874)) | AI Buddies | One drone with a useful default job. You still deposit loot yourself |

**Gap:** no PvP or rival bases. Season 01's "Rival" faction is AI.

### What makes it fun (developers)

- Mikkel Martin Pedersen: "We feel there's an underdeveloped market in the co-op genre" and "It's about shooting monsters and exploring caves in space" ([I] [PCGamesInsider](http://www.pcgamesinsider.biz/indie-interview/66706/we-feel-the-co-op-shooter-market-is-underdeveloped-ghost-ship-games-tells-us-about-deep-rock-galactic/)).
- Lundgaard: "It's a co-op game, so we have not spent a ton of time on the class balance aspect", and they wanted it "solo player friendly" ([I] [GamingBolt](https://gamingbolt.com/deep-rock-galactic-interview-heigh-ho-off-to-plunder-we-go/amp)).
- Lundgaard: it is "possible, through game design, to create a positive community" ([I] [NME](https://www.nme.com/features/soren-lundgaard-ghost-ship-games-deep-rock-galactic-3586886)).

### Frustrations and responses

1. **Friendly-fire griefing from specific gear.** Rather than remove friendly fire, they cut it item by item: Ripper grenade friendly fire reduced 70% ([P] [S03 Patch 3](https://store.steampowered.com/news/app/548430/view/6368680725420902567)); the SMG turret's Arc overclock "Significantly reduced"; EM Discharge "Removed friendly damage entirely" ([P] [Nov 2023 patch](https://store.steampowered.com/news/app/548430/view/6560117310263971525)).
2. **Solo difficulty.** "Added more caretaker scaling based on player numbers to make solo fights a little easier" ([P] [S01 Patch 01](https://store.steampowered.com/news/app/548430/view/4186653398784758162)).
3. **Ally AI bugs.** "Fixed a bug that made Bosco sometimes attack the player" ([P] [S01 Patch 04](https://store.steampowered.com/news/app/548430/view/4227187225810670032)).
4. **Seasonal FOMO.** Past seasons were made replayable with all cosmetics "still earnable for free" ([P] [Steam news](https://store.steampowered.com/news/app/548430/view/5756237364322707431)); a toggle for stale seasonal events followed ([P] [Steam news](https://store.steampowered.com/news/app/548430/view/5229301621572556798)).

---

## 4. Overcooked 2 (Ghost Town Games / Team17, 2018) and *All You Can Eat*

| Players | Local / online | Level length | Platform |
|---|---|---|---|
| 1–4 ([P] [Steam OC2](https://store.steampowered.com/app/728880/)) | OC1 local only ([P] [Steam OC1](https://store.steampowered.com/app/448510/)); OC2 couch or online, co-op and versus; AYCE adds online to OC1 levels and later cross-play ([P] [Steam news](https://store.steampowered.com/news/app/1243830/view/4703417046214246479)) | A few minutes per kitchen (**unverified**) | All major consoles and PC |

### Loops

- **Moment to moment:** pick up, chop, cook, plate, serve and wash, against an order queue and a timer. Kitchens move or split mid-round. Duncan: "This time around we've got new things like dynamic levels where recipes change halfway through" ([I] [Red Bull](https://www.redbull.com/int-en/overcooked-2-ghost-town-games-interview)).
- **Per level:** a timed shift scored against 1–3 star thresholds. A later update showed the star boundaries on the map, loading and results screens ([P] [Steam news](https://store.steampowered.com/news/app/728880/view/2444779293791243508)), and New Game+ added a 4th star ([P] [Steam news](https://store.steampowered.com/news/app/728880/view/2454911758755479698)).
- **Meta:** an overworld unlocked by stars, chef unlocks, DLC kitchens and free seasonal updates ([P] [Steam news](https://store.steampowered.com/news/app/728880/view/2403127338425496592)).

### Parallels to Team Buddies

| Overcooked | TB equivalent | Notes |
|---|---|---|
| You hold one item; holding it stops other actions (**unverified**, recalled) | Carry blocks attack | — |
| **Throwing, added in OC2** because some players "wouldn't have enough to do when the map separated teams into different sections" (the article's wording) ([I] [Red Bull](https://www.redbull.com/int-en/overcooked-2-ghost-town-games-interview)). In practice only ingredients can be thrown, not plates or pans (**unverified**) | Jump-throwing a crate to a teammate or the pad | **Throwing turns separation into a hand-off game** |
| A recipe is a fixed set of processed inputs that makes one dish | Stack shape makes one toy | De-Vine: "We boiled down recipes into their finest form to keep things simple" ([I] Red Bull) |
| Shared kitchen; players block, bump and burn each other | Friendly-fire comedy | Versus is 2v2, with no stealing in co-op |

### What makes it fun (developers)

- Duncan: "with Overcooked all players are equally responsible for the success of the team" and "we were constantly adjusting the ratio of freedom and restriction that we gave the players" ([I] [Game Developer](https://gamedeveloper.com/design/road-to-the-igf-ghost-town-games-i-overcooked-i-)).
- They designed each level around a single idea: "We had a design philosophy of building levels around a single element" ([I] [Red Bull](https://www.redbull.com/int-en/overcooked-2-ghost-town-games-interview)).
- **Fixed roles kill the fun.** In early prototypes players "would just sit there in silence, do their job, and get through it". The moving kitchens "force players to realise there's no one role you can do forever." Duncan: "We want to avoid what we call 'First to Fun'" (one player taking charge). De-Vine: "You don't want to be the player who feels like they have to pull the other player along" ([I] [Nintendo Life](https://www.nintendolife.com/features/interview-were-not-making-a-party-game-now-its-more-of-an-evening-thing-ghost-town-is-moving-on-from-overcooked)).
- "the difficulty comes much more from a team's ability to communicate", and "We had to do a lot of work to ensure players weren't getting frustrated making little mistakes" ([I] [Push Square](https://pushsquare.com/news/2018/08/interview_chewing_the_fat_with_overcooked_2_developer_ghost_town_games)).

No GDC talk by Ghost Town Games was found.

### Frustrations and responses

- **No online in OC1:** "The most frequent request we got from people was for online multiplayer" ([I] Push Square). De-Vine: "we knew we couldn't do the online play on our own, so we partnered with Team17" ([I] Red Bull). OC2 shipped with it.
- **Online in practice:** OC2 is peer-to-peer ([P] [Steam news](https://store.steampowered.com/news/app/728880/view/2444779293775819123)). Guests couldn't keep progress until the Shared Progress update ([P] [Team17](https://www.team17.com/overcooked-2-shared-progress-update-available-now/)). Mixing two couch players with two online players came in Dec 2018 ([P] [Team17](https://www.team17.com/overcooked-2-kevins-christmas-cracker-free-update-available-now)). A 2022 matchmaking update was rolled back after "multiple matchmaking and connection issues" ([P] [Steam news](https://store.steampowered.com/news/app/728880/view/4437747060830711964)).
- **Difficulty and accessibility:** AYCE launched with an assist mode (longer timers, orders that don't expire) and colour-blind indicators. An Oct 2022 update made assist togglable "at any point regardless of save file", and added level skip and an option to turn off fires from burnt food ([P] [Steam news](https://store.steampowered.com/news/app/1243830/view/6402455182504359246)).

---

## 5. Moving Out (2020) / Moving Out 2 (2023) (SMG Studio, DevM Games / Team17)

| Players | Local / online | Level length | Platform |
|---|---|---|---|
| 1–4 | MO1 local only: "Moving Out only supports 1-4 players in local co-op", with Remote Play Together as the workaround ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/3127131079124862476)). MO2 adds cross-play online co-op ([P] [Steam news](https://store.steampowered.com/news/app/1641700/view/4621277624686858150)). No versus | Timed for Bronze/Silver/Gold; a few minutes (**unverified**) | PC, Switch, PS4/5, Xbox ([P] [Steam MO1](https://store.steampowered.com/app/996770/), [MO2](https://store.steampowered.com/app/1641700/Moving_Out_2/)) |

### Loops

- **Moment to moment:** grab the tagged items, carry, drag or throw them, and pack the truck while dodging hazards.
- **Per level:** a race to load everything, ranked by time, with optional objectives such as breaking every window ([W] [Wikipedia](https://en.wikipedia.org/wiki/Moving_Out_(video_game))).
- **Meta:** a world map, unlockable characters, bonus arcade levels, DLC, and a free "Moving In" update with reversed levels ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/4483864038806104226)).

### Parallels to Team Buddies

| Moving Out | TB equivalent | Notes |
|---|---|---|
| "Objects have their own mass… Throw a box and it will fly through the air but the fridge will only rock ominously" ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/2609208878394583411)) | Crates and super crates | Weight as readable gameplay |
| Load the truck "in a very Tetris-like fashion" (same post) | Arranging crates on the pad | Packing as a spatial puzzle |
| Heavy items need two movers ([W]) | — | A shared carry forces cooperation; assist mode can make heavy items one-person ([R] [DualShockers](https://www.dualshockers.com/moving-out-accessibility-assist-mode-features-showcased-in-trailer/)) |
| **Co-op throw**: testers tried "to chuck a photocopier over the side of a balcony", so the team built a "cooperative throw… swing big ticket items and releasing at just the right moment" ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/3183419193841605208)) | Jump-throwing crates | A model for throwing heavy crates |
| "throw that box out the window… hurl that coffee table through that window right into your truck" ([P] physics post above) | Throwing crates downhill to teammates | Destruction as a shortcut |
| A hold/throw toggle "takes away the need to hold down triggers for extended periods" ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/2978509755359556468)) | Hold-to-carry | An accessibility fix for any carry game |

### What makes it fun (developers)

- "the physics in Moving Out are a blend of humour and destruction. We wanted success in the game to be unruly" ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/2609208878394583411)).
- "we try not to make a final call unless it's passed the 'people test'" ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/3183419193841605208)).
- Dual Mover Mode, one controller running two movers, started as a solo testing tool and shipped so that solo players get "access to the two-player content" ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/3112493747280951420)).

### Frustrations and responses

- **No online in MO1**, answered in MO2: "our most requested feature from the first game was to add online multiplayer… Creating an online physics game comes with a lot of challenges though" ([I] [GamingBolt](https://gamingbolt.com/moving-out-2-interview-characters-locations-accessibility-and-more)).
- **Assist mode at launch** removed hazards, lengthened timers and added level skip ([P] [Steam news](https://store.steampowered.com/news/app/996770/view/2978509755359556468)). For MO2: "We learnt a lot from Moving Out's assist mode, as well as the reaction it got… Reduced difficulty for example is now on a per level basis" ([I] GamingBolt).
- **MO2 online stability:** patch 1.3 made players recreate their Team17 account, losing friend lists, to fix "pesky issues… raised by the community" ([P] [Team17](https://www.team17.com/news/moving-out-2-update-1-3-coming-soon-important-multiplayer-information)).
- Game Informer's 6.5 review cited physics frustration ([R], reported second-hand by Wikipedia).

---

## 6. Unrailed! (Indoor Astronaut / Daedalic, EA 2019, 1.0 2020)

| Players | Local / online | Run length | Platform |
|---|---|---|---|
| Up to 4 co-op; versus is 2v2; local and online can be mixed ([P] [Steam](https://store.steampowered.com/app/1016920/)) | Both | Endless roguelike run (no official length) | PC, Switch, PS4, Xbox; Steam–Switch cross-play from Nov 2020 ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/3856732771968496900)) |

A sequel, *Unrailed 2: Back on Track*, entered Early Access in Nov 2024 with 4v4 versus ([R] [MP1st](https://mp1st.com/interviews/indoor-studios-talks-unrailed-2-back-on-track-their-roguelike-train-track-building-sequel)).

### Loops

- **Moment to moment:** chop wood and mine iron with carried tools, feed them to the train's storage wagon, and let the crafter wagon turn them into rails. Lay track ahead of a train that never stops. Storage and crafter capacity start at 3 per resource ([W] [Crafter](https://unrailed-wiki.com/page/Crafter)).
- **Per run:** station to station inside a biome. The train speeds up "in discrete steps after each station" ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/2504491307404130161)). Stations sell wagons and upgrades for bolts. A crash ends the run.
- **Meta:** leaderboards, unlockable characters, weekly challenges. *Unrailed 2* adds upgrades that survive a derailment ([R] MP1st).

### Parallels to Team Buddies

| Unrailed! | TB equivalent | Notes |
|---|---|---|
| Haul raw resources to the crafter wagon, which outputs rails | Haul crates to the pad, which outputs toys | The crafter **auto-builds** from what is delivered |
| A shop of wagons that cost bolts; a second wagon of the same type costs more ([P] [1st EA update](https://store.steampowered.com/news/app/1016920/view/2504491307404130161)) | Slot tiers | Rising cost per duplicate |
| Your role is the tool you are holding (axe, pickaxe, bucket) (**unverified**, recalled) | Buddy classes | Roles are swappable by picking up a tool |
| 2v2 race on mirrored seeded maps: "Who will get their train to the station first?" ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/2413289226969924753)); a Cannon Wagon is on by default in versus, and can be turned off ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/5589599109276994881)) | Rival teams | What the cannon does is unverified. No stealing |

### What makes it fun (developers)

- Lukas Rahmann: "you need to micromanage yourself, and your friends as well", and "your friends can, instead of helping you, be a burden" ([I] [Gamereactor](https://www.gamereactor.eu/in-unrailed-you-need-to-micromanage-yourself-and-friends/)).
- Hendrik Baatz wanted the core loop "a little bit chaotic in a way that a team must work together to succeed". **They dropped a first-person prototype because it was "difficult to observe what your teammates were doing, which led to less interaction within the team"** ([I] [MP1st](https://mp1st.com/interviews/indoor-studios-talks-unrailed-2-back-on-track-their-roguelike-train-track-building-sequel)).
- From PAX West: "having your teammate right next to you opens up a whole new layer of 'fun'" ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/3658535001306407563)).

### Frustrations and responses

- **Solo play** was "by far the most requested feature". The team "never felt that the single-player mode came close enough to the chaotic gameplay" of co-op, so they added a bot you command with emoji ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/3112493747267436705)).
- **Role camping:** tasks like "Each player has to mine at least one rock / chop at least one tree" ([P] [1st EA update](https://store.steampowered.com/news/app/1016920/view/2504491307404130161)).
- **Balance by player count:** "Train speed is now better adapted to 2 and 3 players (slower than before…)" (same post).
- **Permadeath:** optional checkpoints were added and kept off the leaderboards; savegames are "controversial… might take away from the roguelike core" ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/2587811707166325236)).
- **Griefing:** instant host kick, vote-kick and a block list ([P] [Steam news](https://store.steampowered.com/news/app/1016920/view/5124581515259146585)).

---

## 7. Lethal Company (Zeekerss, Early Access 2023)

| Players | Local / online | Session length | Platform |
|---|---|---|---|
| Up to 4 ([R] [Wikipedia](https://en.wikipedia.org/wiki/Lethal_Company)) | Online co-op; no local | No official figure | Windows ([P] [Steam](https://store.steampowered.com/app/1966720/)) |

### Loops

- **Moment to moment:** search derelict interiors for scrap, avoid monsters, and carry the scrap to the ship before night: "Communicate with your crewmates to carry all valuables to the ship before things get too dangerous" ([P] [Steam](https://store.steampowered.com/app/1966720/)). One player can stay on the ship and guide the others from the terminal.
- **Per run:** a deadline for each quota; sell scrap to the Company; the quota rises; missing it ends the run ([R] Wikipedia).
- **Meta:** credits buy tools, upgrades, furniture and suits; runs reset. Since V65, furniture kept on the ship gains "luck", which slows quota growth ([P] [V65](https://store.steampowered.com/news/app/1966720/view/6212245217911777747)). Weekly seeded Challenge Moons are meant "less for competition and more to give us all a shared experience" ([P] [V47](https://store.steampowered.com/news/app/1966720/view/5410576585196867953)).

### Parallels to Team Buddies

| Lethal Company | TB equivalent | Notes |
|---|---|---|
| Scrap carried by hand to the ship; some items are two-handed ([W] [Scrap](https://lethalcompany.wiki.gg/wiki/Scrap)); heavy scrap slows you: "carrying heavy scrap makes you harder to get around" ([P] [V80](https://store.steampowered.com/news/app/1966720/view/1828894815563326)) | Carrying crates | Weight is the cost, not a full weapon lock |
| The shovel's weight was raised "to make each player less of a self-sufficient killing machine" ([P] [V50](https://store.steampowered.com/news/app/1966720/view/6474562312189361059)) | Ammo shared between guns | Weight used to balance combat against hauling |
| The Company Cruiser, "capable of transporting an unimaginable amount of scrap all at once" ([P] [V55](https://store.steampowered.com/news/app/1966720/view/5842939267343376814)) | Vehicles carrying crates | — |
| Monsters that take scrap (Baboon hawks) | Crate-stealing monkeys | The raiders are AI, not rivals |
| Friendly fire: shovel damage on players raised "from its originally intended 10% to 20%" ([P] V55) | Friendly fire | — |

### What makes it fun (developer)

- "I want to release updates that have a nice handful of new things to discover each time, so you can come back and be surprised" ([P] [launch post](https://store.steampowered.com/news/app/1966720/view/5218041989060981182)).
- "Lethal Company started out as a completely indoor game with an elevator instead of a space ship" (same post).
- Fall damage was "intended to make verticality feel more important; for example I felt Titan's return trips were too simple for its level of reward" ([P] [V47](https://store.steampowered.com/news/app/1966720/view/5410576585196867953)). **The return trip is the tuned cost.**
- Exploits that bypassed the trip were cut because they "made it possible to bypass Lethal Company's design and mechanics without requiring any more skill or strategy" ([P] [V50](https://store.steampowered.com/news/app/1966720/view/6474562312189361059)).

### Frustrations and responses

- **The clearest carry-design lesson in this document.** V80 gave tools their own utility slot. Zeekerss: "While I liked the strategic tradeoff of having to use a scrap slot to hold tools, it meant that most players opted out of using items all-together" ([P] [V80](https://store.steampowered.com/news/app/1966720/view/1828894815563326)). V64's belt bag holds 15 non-scrap items that teammates can reach into ([P]).
- **New players overwhelmed:** V80 added a "diversity meter… to keep from overwhelming new players especially" ([P] V80).
- **Slow updates:** "I'm just not working like I have a quota, because that would be ironic" ([P] [V50](https://store.steampowered.com/news/app/1966720/view/6474562312189361059)).

---

## 8. R.E.P.O. (semiwork, Early Access 2025)

| Players | Local / online | Session length | Platform |
|---|---|---|---|
| "up to 6 players" ([P] [Steam](https://store.steampowered.com/app/3241660/)) | Online co-op with proximity voice; no local | No official figure | Windows; consoles planned for around 1.0 ([P] [Q&A](https://store.steampowered.com/news/app/3241660/view/1826362059924100)) |

It began as a single-player horror game about cleaning, later turned multiplayer ([P] [Steam news](https://store.steampowered.com/news/app/3241660/view/1814309641474831); [R] [Photon case study](https://blog.photonengine.com/r-e-p-o-multiplayer-success-powered-by-photon/)).

### Loops

- **Moment to moment:** find valuables, grab them with a physics "grabbing tool", and move them to an extraction point: "Transport anything from heavy pianos to fragile ceramics and use teamwork to make sure that the precious cargo will safely reach its destination" ([P] [Steam](https://store.steampowered.com/app/3241660/)). Impacts lower an item's value ([R] Wikipedia). Extraction points show "what valuables are at risk of being destroyed" ([P] [v0.2.0](https://store.steampowered.com/news/app/3241660/view/1803527891507631)).
- **Per run:** each level has a quota; valuables don't respawn: "If you smash everything and still can't make quota, you'll need to accept defeat" ([P] [50 Questions](https://store.steampowered.com/news/app/3241660/view/1802893906754534)). "Level size, enemy count and extraction goal increase per level" ([P] v0.2.0).
- **Meta:** a shop sells upgrades and weapons; v0.4.0 added 500+ cosmetic parts ([P] [v0.4.0](https://store.steampowered.com/news/app/3241660/view/1832065502811979)). No microtransactions: "You buy the game once, you have the game" ([P] 50 Questions).

### Parallels to Team Buddies

| R.E.P.O. | TB equivalent | Notes |
|---|---|---|
| Everything is carried physically; two or more players can share a load | Crate hauling | Shared carries are a cooperation verb |
| Damage in transit costs value | — | **A different carry cost:** fragility instead of a weapon lock |
| A shared cart for hauling, with cart upgrades (Cart Cannon, Cart Laser) ([W]; [P] names) | Vehicles carrying crates | — |
| Extraction point turns value into money; money buys upgrades and weapons | The pad turns crates into toys | A lump sum, not a shape recipe |
| Players and valuables can be grabbed and thrown; valuables used as weapons lose value ([P] v0.2.0) | Throwing crates; friendly-fire comedy | — |

**Gap:** no rival team or raiding.

### What makes it fun (developers)

- Deaths are comedy by design: the mic "Now stays active slightly longer on death for more death scream action" ([P] [v0.2.0](https://store.steampowered.com/news/app/3241660/view/1803527891507631)).
- Dead players stay involved: Death Head Possession (v0.3.0) lets spectators talk to and interact with the living ([P]).
- On networking: "Making all the physics feel smooth for clients was something we struggled with for a long time" ([I] [Photon case study](https://blog.photonengine.com/r-e-p-o-multiplayer-success-powered-by-photon/)).

### Frustrations and responses

- **Hackers:** lobbies forced to Friends Only at v0.1.2; then a server list, kick and passwords in v0.2.0 ([P] v0.2.0).
- **Late-game stun-lock** from grabbing enemies: the "Overcharge" system. "If multiple players grab the same enemy, the Overcharge is greatly reduced" ([P] [devlog](https://store.steampowered.com/news/app/3241660/view/1799088287863956); v0.2.0).
- Revive invulnerability "to prevent instant deaths" (v0.4.4), and an arachnophobia setting ("sorry!") in v0.3.1 ([P]).

---

## 9. Sea of Thieves (Rare / Xbox Game Studios, 2018)

| Players | Local / online | Session length | Platform |
|---|---|---|---|
| Crews of 1–4 (sloop 1–2, brigantine 3, galleon 4) ([R] [Wikipedia](https://en.wikipedia.org/wiki/Sea_of_Thieves)) | Online only, cross-platform ([P] [Steam](https://store.steampowered.com/app/1172620/)) | No official figure | PC, Xbox; PS5 from Apr 2024 |

Rare reported 40 million players by April 2024 ([P] [Steam news](https://store.steampowered.com/news/app/1172620/view/5762994032387571180)).

### Loops

- **Moment to moment:** sail as a crew (wheel, sails, cannons, bailing), dig up or loot treasure, carry it to the ship, fight skeletons and other crews. You hold one treasure "no matter its size" and "cannot fight or use any equipment or resources" while carrying ([W] [Loot](https://seaofthieves.wiki.gg/wiki/Loot)). Dropped chests float and sink after about 5 minutes ([W]).
- **Per session:** voyages for trading companies; sell at outposts. Emissary flags raise payouts and make your ship a target ([W]). In Hourglass battles your stake grows with each win, but "if you get sunk first, all that stored value will be lost to the waves!" ([P] [Season 8](https://store.steampowered.com/news/app/1172620/view/4836332027648310848)).
- **Meta:** cosmetic-only progression. Mike Chapman: "We want you to feel like you've always got a chance, so our progression system is not a single number" ([I] [Game Informer](https://www.gameinformer.com/b/features/archive/2017/11/17/how-rare-cast-away-its-developmental-process-for-sea-of-thieves.aspx)).

### Parallels to Team Buddies

| Sea of Thieves | TB equivalent | Notes |
|---|---|---|
| Carrying treasure means unarmed | "Can't shoot while carrying" | **Near-exact match.** Teammates must escort the carrier |
| Reaper's Bones buys treasure stolen from other crews ([W]) | Stealing crates from rival pads | **Stealing is rewarded**, not just allowed |
| Ship is base and vault; sinking it loses the loot aboard | Destroying the base disables the pad | Closer to TB's decapitation than any other game here |
| Season 14 raid tools: barrel or chest disguises, a Grapple Gun that pulls treasure, traps ([P] [Season 14](https://store.steampowered.com/news/app/1172620/view/6212244583179161994)) | Stealthy and Ninja Buddies | Specialist sabotage verbs |
| No friendly fire inside a crew; rewards are shared. Rare changed things after alpha players fought over who held the treasure ([I] Game Informer) | — | A contrast: TB's comedy friendly fire vs SoT's crew harmony |

**Gap:** no crafting or stacking; outposts convert loot into gold.

### What makes it fun (developers)

- Joe Neate: "Our game is a friendship creation tool", and "We wanted to make a game like that, where stories came out of the very fabric of players playing together" ([I] [Game Informer](https://www.gameinformer.com/b/features/archive/2017/11/17/how-rare-cast-away-its-developmental-process-for-sea-of-thieves.aspx)).
- **A cooperative item turned into sabotage.** Chapman on the Chest of Sorrow, which floods your ship: it "sort of bonds a crew together", yet "in the very third session after we put that chest into the game, people were already using it as a weapon", hiding it on rival ships (same source).
- The "tools not rules" motto is reported by press ([R] [GameSpot](https://www.gamespot.com/articles/sea-of-thieves-tools-not-rules-design-keeps-it-unique-four-years-later/1100-6506144/), page not readable) but has **no primary source** found, and no Sea of Thieves design talk turned up on GDC Vault.

### Frustrations and responses

- **Thin launch content:** Rare said it was "adjusting our roadmap based on feedback" and added closed crews to curb griefing through the brig ([R] [MCV](https://www.mcvuk.com/development-news/rare-responds-to-sea-of-thieves-player-feedback/)); content updates followed.
- **PvP stress:** *Safer Seas* (Dec 2023) gave single-crew servers at "30% of that awarded in High Seas", with Reaper's Bones and Hourglass excluded "to maintain a balance between Safer and High Seas"; it was later raised to full gold and reputation with a rank cap of 25 ([P] [Steam news](https://store.steampowered.com/news/app/1172620/view/5395938616795042774); [P] [Xbox Wire](https://news.xbox.com/en-us/2023/12/07/sea-of-thieves-safer-seas/)).
- **Toxicity:** Rare said it needs to "make encounters with other crews feel fairer and more fun" and formed a "Toxicity strike force" ([P] [Steam news](https://store.steampowered.com/news/app/1172620/view/1814309641476741), [Steam news](https://store.steampowered.com/news/app/1172620/view/1828441623110352)).
- **Hourglass queues:** fixed with same-faction fallback battles and cross-server matchmaking ([P] [Steam news](https://store.steampowered.com/news/app/1172620/view/4980448574127433038)).

---

## 10. Splatoon 3 (Nintendo, 2022): Salmon Run and Rainmaker

| Players | Local / online | Match length | Platform |
|---|---|---|---|
| Salmon Run: 4-player co-op; Rainmaker: 4v4 | Online, plus local wireless rooms; no split-screen ("Single system (1)") ([P] [store page](https://www.nintendo.com/us/store/products/splatoon-3-switch/)) | 3 waves plus an occasional boss wave (wave length **unverified**) | Switch |

Salmon Run "can be played at any time!", unlike Splatoon 2's scheduled windows ([P] [Splatoon Direct recap](https://splatoon.nintendo.com/en/news/catch-up-on-all-the-latest-from-the-splatoon-3-direct)).

### Loops

- **Moment to moment:** kill Boss Salmonids, which drop Golden Eggs; carry one egg at a time to the basket, or throw it in an arc (a throw that lands in the basket counts; a miss stays where it lands) ([W] [Inkipedia](https://splatoonwiki.org/wiki/Golden_Egg)). Weapons are loaned by rotation. Downed players become life rings to revive.
- **Per shift:** meet three egg quotas, then perhaps fight a King Salmonid by firing collected eggs from an "Egg Cannon" ([P] Direct recap).
- **Meta:** points, job titles, Hazard Level, rewards, plus events (Big Run on PvP maps, Eggstra Work with a leaderboard) ([P] [version history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257)).

### Parallels to Team Buddies

| Splatoon 3 | TB equivalent | Notes |
|---|---|---|
| Carry eggs to a shared basket; throw them in | Haul and jump-throw crates to the pad | Whether you can still fire while holding an egg is **unverified** |
| **Snatchers** take eggs left on the floor; in Mothership waves the Mothership "comes to steal Golden Eggs from the egg basket" ([P] Ver. 2.0.0, 3.0.0) | A rival raiding your pad | **The pad raid from the defender's side** |
| **Rainmaker:** the carrier's own weapon is *replaced* by the slow-firing Rainmaker, and they can't Super Jump ([W] [Inkipedia](https://splatoonwiki.org/wiki/Rainmaker_(weapon))) | "Can't shoot while carrying" | Cleanest version of the rule in a shooter |

No squad command or unit production; the four players are the squad.

### What makes it fun (developers)

From *Ask the Developer* Vol. 22, *Splatoon Raiders* (July 2026) ([P] [Nintendo](https://www.nintendo.com/en-ca/whatsnew/ask-the-developer-vol-22-splatoon-raiders-part-1/)):

- Inoue: "the satisfaction of playing Salmon Run comes from how you start moving the instant you think."
- Ito: "Salmonids in Salmon Run are designed with four-player co-op in mind." The series aims to "create an experience where players can feel that same sense of pleasant busyness."
- **A solo haul-to-pad loop didn't survive.** The *Raiders* solo prototype copied Salmon Run, collecting eggs and delivering "them to a container". It drifted towards trap-based base defence and "didn't really feel like a Splatoon game". They even tried "a conveyor belt-like gadget that could automatically transport a large number of Golden Eggs". In the end, "we kept the part where you collect the Golden Eggs, but removed the need to take them back to the container."

The reason Nintendo gave for capping Boss Salmonids at 15: when players are "chased down by bosses they cannot splat, the appearance of too many bosses can slow the game down and make it more difficult than necessary to get things back under control" ([P] Ver. 2.0.0, [version history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257)).

### Frustrations and responses

Reviews call high-level Salmon Run punishing and praise its constant availability ([R] [God is a Geek](https://godisageek.com/reviews/splatoon-3-review/)). The patch history ([P] [version history](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257)) shows:

| Version | Change | Problem |
|---|---|---|
| 1.2.0 | Shorter delay between touching an egg and picking it up | Pickup lag |
| 2.0.0 | Boss cap of 15; some bosses slowed, some specials stronger | Overload |
| 3.1.0, 7.1.0, 9.2.0 | Egg pickup timing made equal for host and non-host players | **Host advantage on a carry action** |
| 4.0.0 | Solo Eggstra Work with auto-fill to 4 | Solo access |
| 6.0.0 | At least two ranged specials guaranteed; Big Run medal thresholds announced up front, so players can play "with a goal in mind" | Weapon randomness |

---

## 11. Ultimate Chicken Horse (Clever Endeavour Games, 2016)

| Players | Local / online | Match length | Platform |
|---|---|---|---|
| Up to 4 ([P] [Steam](https://store.steampowered.com/app/386940/)) | "Online, cross-platform, and local play" (same) | Set by a points-to-win option ([P] [blog](https://www.cleverendeavourgames.com/blog/2018/11/21/switch-bug-fix-rule-presets-amp-events)); typical length **unverified** | PC, Mac, Linux, PS4, Xbox One, Switch |

It began as a 48-hour game jam whose themes were "ultimate" and "modularity" ([I] [NYU Game Center](https://gamecenter.nyu.edu/?p=13829)).

### Loops

- **Moment to moment:** run a platforming course while dodging traps, including ones you and your friends placed.
- **Per round:** each player drafts one item from the Party Box, everyone places it, then everyone races. "You score if you finish and your friends don't" ([P] [Steam](https://store.steampowered.com/app/386940/)). Pieces stay, so the level grows over the match.
- **Meta:** unlockables in the hub, a level editor and sharing, Challenge-level times.

### Parallels to Team Buddies

| UCH | TB equivalent | Notes |
|---|---|---|
| Drafting from the Party Box | Grabbing parachuted crates | A random pool you choose from |
| Placed pieces stay for the whole match | Stacking crates on the pad | The build is shared, not owned |
| Bonus points for a trap kill on a rival ([P] [blog](https://www.cleverendeavourgames.com/blog/2015/11/5/dont-fear-game-design-change)) | Sabotaging rival production | — |
| Your own traps can kill you (**inference**) | Friendly fire from your own toys | — |

### What makes it fun (developers)

- Richard Atlas: "the player was building the level, rather than being the person who was running in the level" ([I] [NYU](https://gamecenter.nyu.edu/?p=13829)), and "The discovery of new opportunities and possibilities of combinations is something that we want people to see" ([I] [CGMagazine](https://www.cgmagonline.com/interviews/making-the-ultimate-party-game-an-interview-with-richard-atlas)).
- **The scoring redesign.** In his "Don't Fear (Game Design) Change!" post (Oct 2015, about four months before release), Atlas explains that under the old rules "as long as someone was ahead, they would necessarily win as long as they were reaching the end", so "the last place person wanted to end the game quickly". The fix added bonuses for finishing first and for trap kills, plus a comeback bonus he likens to Mario Kart's lightning bolt. Internal feedback was "overwhelmingly positive". If everyone finishes, or no one does, no one scores, which pushes every build to be hard but possible ([P] [blog](https://www.cleverendeavourgames.com/blog/2015/11/5/dont-fear-game-design-change)).

### Frustrations and responses

- **Wanting control over the draft and the scoring:** the "Chimply Amazing" update added Party Box probability settings because "many people have requested" them, changed the Party Box to "better predict what blocks are necessary", made point values adjustable, and added a round timer after which unfinished players "will be killed" ([P] [update post](https://www.cleverendeavourgames.com/chimply-amazing-update)).
- **Online desync** (2023): fixes for "endless loading screens, getting stuck in the party box, placed blocks not showing up" through a "server refactor" ([R] [Nintendo Everything](https://nintendoeverything.com/ultimate-chicken-horse-update-out-now-version-1-10-06-patch-notes/), quoting the notes).

---

## 12. Tricky Towers (WeirdBeard, 2016)

| Players | Local / online | Match length | Platform |
|---|---|---|---|
| "2-4 players" ([P] [PlayStation](https://www.playstation.com/en-gb/games/tricky-towers/)) | Local and online | **Unverified** | PC, PS4, Xbox One, Switch ([P] [official site](https://www.trickytowers.com)) |

### Loops

- **Moment to moment:** rotate and drop physics-simulated tetrominoes; stacks wobble and topple.
- **Per round** (co-founder Niels Monshouwer, [P] [PlayStation Blog](https://blog.playstation.com/2015/10/21/introducing-tricky-towers-on-ps4/)):
  - Race: "your tower has to remain upright for three seconds before you'll be crowned".
  - Survival: "Each time you drop tetrominoes your character will lose a life."
  - Puzzle: fit as many blocks as possible under a line.
- **Spells:** light spells support your own tower, dark spells "hamper or even push over your competitors' towers" ([P] [Steam](https://store.steampowered.com/app/437920/)).
- **Meta:** XP levels, 50 single-player trials, and a Tournament update with leaderboards ([R] [GameWatcher](https://www.gamewatcher.com/2016-15-11-tricky-towers-receives-a-tournament-update-in-time-for-the-holidays)).

### Parallels to Team Buddies

| Tricky Towers | TB equivalent | Notes |
|---|---|---|
| Stack shape is the result: stable and space-filling, or collapsing | Stack shape picks the toy | **Closest modern stacking analogue**, though the shape decides survival, not a recipe |
| Dark magic attacks a rival's tower | Ninja splitting merged crates on a rival pad | Direct sabotage of rival production |
| Petrify: "When cast on a brick this brick will remain rock solid in your tower" ([I] [GamingBolt](https://gamingbolt.com/tricky-towers-interview-tetris-meets-spell-casting)) | Merging locks crates against theft | Defence of a stack |
| Collapse is both failure and the joke | Physics comedy | — |

### What makes it fun (developers)

- Monshouwer: "players can also use magic spells to either support their own structure or mess up the towers of their opponents", and "With spells flying everywhere and tetrominoes dropping like flies, Tricky Towers gives players a chaotically good time" ([P] [PlayStation Blog](https://blog.playstation.com/2015/10/21/introducing-tricky-towers-on-ps4/)).
- WeirdBeard: "players have to find a balance between building stable towers and attacking their opponents"; on the tech, "having a few hundred physics shapes can be quite tricky" ([I] [GamingBolt](https://gamingbolt.com/tricky-towers-interview-tetris-meets-spell-casting)).

### Frustrations and responses

Reviews ([R] [OpenCritic](https://opencritic.com/game/3038/-/reviews)) said luck decides too much ("the luck factor for success in a majority of the game modes can cause much more frustrating outcomes than a skill-based game would") and that matchmaking was random-only and slow on some platforms. **No developer response was found.**

---

## Synthesis

**Everything in this section is my own analysis**, built on the sourced facts above. Section numbers in brackets point back to the game breakdowns.

### Recurring patterns

#### 1. Carrying costs you your weapon, in graded steps

Six of the twelve games, plus Halo's CTF, restrict what a carrier can do:

| Game | Rule | Strength |
|---|---|---|
| Sea of Thieves [9] | Treasure in hand means no weapons or items | Full lock |
| Deep Rock Galactic [3] | Heavy object: no weapons, 25% slower, no sprint; can throw | Full lock with a throw |
| Splatoon Rainmaker [10] | Your weapon is replaced by a slow special one | Swap |
| Helldivers 2 [2] | One-handed load allows a one-handed weapon; two-handed allows none | Graded |
| Halo CTF (short entry) | Melee with the flag only | Melee-only |
| Lethal Company [7] | Weight slows you; hands fill up | Soft |
| R.E.P.O. [8] | Damage in transit costs value | Fragility |

The revisions all go the same way: **a harsh carry cost has to come with a way out.** Helldivers 2 later let you jog with two-handed items. Lethal Company gave tools a separate slot because players had stopped using tools at all. DRG and Overcooked 2 give you a throw.

**For a TB successor:** keep TB's "can't shoot while carrying", since it is what makes escorting matter. But give the carrier a verb: a throw (TB already has jump-throw), a shove, or a weak melee as in Halo. Consider a graded rule like Helldivers 2's, where a single crate allows a sidearm but a super crate needs both hands.

#### 2. Value only counts once it is delivered, and the trip is the game

Helldivers samples count only aboard the shuttle [2]. Sea of Thieves loot counts only when sold, and is lost if your ship sinks [9]. R.E.P.O. loot loses value on every bump [8]. Lethal Company's developer tuned *return trips* directly, adding fall damage because one moon's trip was "too simple for its level of reward" [7].

**For a TB successor:** TB crates already have this shape. Lean into it: long, readable routes from the drop zone to the pad. Super crates could drop farther away or be heavier, so the most valuable crate also means the riskiest trip.

#### 3. Stolen value should spill onto the map

The best raid designs don't just move points from one team to the other; they create a scramble:

- Pikmin 4's Sneak Bomb takes a proportional, capped share of the rival's score and spills it as weights **either** team can carry home [1].
- Bingo Battle makes stealing the rival's Macaroon a win condition [1].
- Splatoon's Mothership raids the basket [10], and Sea of Thieves *pays* for stolen loot through Reaper's Bones [9].
- Fall Guys' egg rounds (short entry) are the whole "raid the rival's nest" loop in a few minutes.

**For a TB successor:** when a pad is hit or a base destroyed, unmerged crates should burst out as loose crates anyone can grab, rather than vanish. That turns TB's "decapitation" into a fight over the leftovers, and capping the loss (Pikmin's 100-point cap) limits snowballing.

#### 4. Friendly fire is framed as consistency, then tuned item by item

Pilestedt: "Things should be consistent in the game world and therefore, we must have friendly fire," and the "chuckles" are the payoff [2]. DRG uses it as a difficulty dial (0.10 at Hazard 1 up to 0.70 at Hazard 5), and when particular gear made griefing easy, it cut friendly fire *for that item* rather than turning it off [3]. R.E.P.O. lengthened the death-scream mic window on purpose [8]. Moving Out wanted success to be "unruly" [5].

**For a TB successor:** keep TB's splash damage, since TB's own awards included "Friendly Fire" and "Kamikaze". Scale it by mode or difficulty, and expect to tune specific weapons (TB's Armageddon Barrel, napalm) rather than the global rule.

#### 5. A short time to the first fun moment

Overcooked builds each level around a single idea [4]. Pikmin 4 deliberately lowered "the barrier to entry for new players" [1]. Gang Beasts was built on one satisfying punch [short entry]. UCH came out of a 48-hour jam [11]. Splatoon's producer locates the fun in "how you start moving the instant you think" [10].

**For a TB successor:** TB's first lesson is already "one crate = Uzi". The first 30 seconds of a match should end with something built and opened. Drop the first crates beside the pad.

#### 6. Roles must rotate, or players go quiet

Overcooked's early testers would "just sit there in silence, do their job"; the moving kitchens exist to stop that [4]. Unrailed added tasks that require "each player" to mine and chop [6]. Overcooked's "First to Fun" warning is about one player taking over [4].

**For a TB successor:** a fixed "hauler" or "stacker" role would be boring. Rotating drop zones, events that drop megacrates, and raids that pull defenders away keep roles changing.

#### 7. Instant call-down as an alternative to slow building

Helldivers 2's stratagems deliver TB's output categories (weapons, turrets, vehicles) in seconds for a code and a cooldown [2]. DRG's resupply pod spends a pooled resource on a drop [3]. Unrailed's crafter builds automatically from what is delivered [6].

**For a TB successor:** keep the stack as the decision (which shape, when to commit), but make the build itself fast, or let allies finish it. A beacon-style drop could suit megacrates.

#### 8. Anti-snowball scoring and scaling with player count

UCH's redesign rewards first place, trap kills and comebacks, and gives no points if everyone or no one finishes [11]. Pikmin's capped proportional theft [1]. Unrailed slowed its train for 2–3 players [6]. DRG scaled a boss for solo [3]. Splatoon capped bosses at 15 so a struggling team could recover [10].

**For a TB successor:** scale crate drop rate and base health with team size, and add a comeback lever such as a megacrate that drops nearer the losing team.

#### 9. Solo needs a companion, and online is assumed

- Every co-op game here added or improved a solo path:
  - Bosco [3]
  - Unrailed's emoji-commanded bot [6]
  - Moving Out's one-controller Dual Mover [5]
  - Pikmin 4's CPU Dandori Battle [1]
  - Splatoon's auto-filled solo Eggstra Work [10]
- *Splatoon Raiders* went further and dropped the haul-to-container step in solo, because solo hauling drifted into base defence [10].
- The most requested feature for both Overcooked and Moving Out was online play [4][5].
- Physics online is expensive. It took Gang Beasts years [short entry], R.E.P.O. "struggled with for a long time" [8], and Splatoon needed three patches to make egg pickup fair between host and guests [10].

**For a TB successor:** plan both couch and online from day one, and budget for networked physics of the carried crates.

#### 10. Assist modes are now expected

Overcooked: AYCE's assist mode can be toggled any time [4]. Moving Out's assist mode was refined to work per level [5]. Pikmin 4 added creature activity levels [1]. Unrailed added optional checkpoints and a kids' difficulty [6].

### Which Team Buddies weaknesses the modern games solved

TB's problems are listed in the reference file's "Problems a new game should design around" section.

| TB weakness | How modern games handle it | Lesson for a successor |
|---|---|---|
| **Confusing controls** ("painfully confusing"; every button used) | Overcooked and Moving Out get by on move, grab/drop, use and throw [4][5]. Moving Out added a hold/throw toggle [5]. Pikmin 4 added an Auto Target Lock toggle and "Precise" aim [1]. Helldivers 2 puts every output behind one input pattern [2] | One context-sensitive interact button (grab, drop, kick open, enter), one throw, one command. Orders should depend on the target, as TB's already did, but without the binoculars step |
| **Weak ally AI** ("useless teammates"; allies idle unless ordered) | Bosco has a useful default job: mine, throw you carryables, revive [3]. Pikmin units haul on their own once pointed at something [1]. Oatchi takes over a whole task when you split up [1]. Unrailed's bot takes simple commands [6]. DRG fixed Bosco attacking the player [3] | Give each ally one default job with visible intent (haul to pad, guard pad), and let one button point them at a target. Never let allies idle |
| **Camera and readability** | Unrailed dropped first person because you couldn't see teammates [6]. Overcooked, Moving Out, Unrailed and Tricky Towers use one shared, fixed or overhead screen. Lethal Company gives one player a radar job on the ship [7] | Prefer a shared overhead camera for couch play, with split-screen only when teams separate. Make pads and carriers readable from a distance |
| **Slow stacking** ("too time consuming") | Helldivers 2 delivers output in seconds [2]. Unrailed's crafter builds automatically [6]. In Tricky Towers the stacking *is* the game, with immediate feedback [12]. Overcooked 2 added throwing so separated players could hand off [4] | Let players throw crates onto the pad; let allies finish a build; keep the ghost preview; shorten merge time |
| **Difficulty spikes** (*GameFan*: from "Piece of cake" to "Whoa Nelly!") | Assist modes [4][5], activity levels [1], checkpoints [6], and enemy caps that let a team recover [10] | Add an assist toggle and recovery tools (a guaranteed crate drop for a team that lost its base) |
| **Runaway leaders** (a destroyed base means a lost match) | UCH's comeback scoring [11]; Pikmin's capped theft and spill-out [1] | When a base is destroyed, crates spill out to be fought over; offer a rebuild path |

### Five ideas most worth stealing (ranked)

1. **Theft spills onto the map** (Pikmin 4's Sneak Bomb and Dandorium [1], Fall Guys' egg rounds). A raid or a destroyed base should scatter crates that either side can grab, with a capped loss. This keeps TB's best moment, the raid, and removes its worst outcome, an unrecoverable snowball.
2. **"Carrying disarms you", with a way out** (Sea of Thieves [9], DRG [3], Helldivers 2's graded hands and jog patch [2], Lethal Company's V80 lesson [7]). Keep the weapon lock and give the carrier a throw or pass. Consider a graded rule where a super crate takes both hands.
3. **Allies with one default job and readable intent** (Bosco [3], Pikmin's automatic carrying [1], Unrailed's commanded bot [6]). This answers TB's biggest complaint. Idle Buddies haul crates to the pad and finish builds; one button redirects them.
4. **Friendly fire as consistency, tuned item by item** (Helldivers 2 [2], DRG [3], R.E.P.O.'s death screams [8]). Comedy needs consistency. Scale it by mode, and fix griefable weapons one at a time.
5. **Comeback-aware scoring** (UCH's redesign [11], Pikmin's proportional cap [1], Unrailed's player-count scaling [6]). Give the trailing team a lever, such as megacrates landing closer to them, and scale drop rate and base health with team size.

Close runners-up: throwing as the fix for separated players (Overcooked 2 [4], Moving Out's co-op throw [5]); fragility as a second carry cost for one crate type (R.E.P.O. [8]); one new idea per level for the campaign (Overcooked [4]).

---

## Open questions

1. **No GDC talks found** for Overcooked, Helldivers 2, Deep Rock Galactic, UCH, Tricky Towers, Gang Beasts or Sea of Thieves' design. Searching GDC Vault directly by speaker might still turn some up.
2. **Match lengths** are unofficial or unverified for most games here.
3. **Exact carry rules still unverified:** Overcooked's one-item rule and which items can be thrown, Lethal Company's two-handed rules, whether Salmon Run lets you fire while holding an egg, DRG's M.U.L.E. deposit and Power Cell carry rules, and what Unrailed's versus cannon does.
4. **Unsourced mottos:** Arrowhead's "A Game for Everyone is a Game for No One" and Rare's "tools not rules" have no primary source yet.
5. **The Team Buddies remaster:** watch the Steam page for its player counts and whether it adds couch play.
6. **Teardown's "Treasure Thieves" mode** may be the closest new match for physics hauling plus stealing; its rules weren't checked.
