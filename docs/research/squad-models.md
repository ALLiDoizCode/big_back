# Squad and commander models in competitive action games

Research notes for wayfinder ticket #8. The question: how have competitive PvP games handled one player controlling several units, compared with one character per player? What does that mean for a Team Buddies-like? Compiled 2026-10-18.

These notes build on two earlier files, and this file does not repeat them:

- [`team-buddies-core-loops.md`](team-buddies-core-loops.md), "Buddy AI and squad commands", covers TB's own model: binocular orders, which order a target gets, tap for one Buddy and hold for the whole team, build orders, and the complaint that allies are "useless".
- [`modern-team-buddies-likes.md`](modern-team-buddies-likes.md) §1 covers Pikmin 4 Dandori Battle's rules. Its synthesis ("Allies with one default job and readable intent") already recommends a useful default job for idle allies.

## How to read this

Source tags follow the convention of the earlier files:

| Tag | Meaning |
|---|---|
| **[P]** | Primary: the developer or publisher, through official patch notes, a dev blog, a store page, an official forum or Steam post by a developer, or an official video transcript |
| **[I]** | A named developer quoted in a press interview |
| **[R]** | Review or press summary. Secondary |
| **[W]** | Wiki, guide, forum, Reddit or stats site. Low trust; used for leads and numbers only |
| **unverified** | Seen only in a search snippet, or behind a page that would not load |

Dota 2 patch quotes are verbatim. They come from Valve's official patch-notes feed (`dota2.com/datafeed/patchnotes`), and links point to `dota2.com/patches/<version>`. Interview quotes were read through a page-summarising fetch tool, so treat them as close to verbatim but not exact.

Sections and lines marked **Analysis** are my own reasoning, not any source's.

**Coverage gaps.** I found no developer postmortem or GDC talk for NS2, Savage, Battlezone, AirMech or Sacrifice. The one relevant GDC talk I found, Halo Wars 2's UX postmortem, was not accessible. Opponent readability is the least-documented of the five questions.

---

## 1. Game-by-game evidence

### Natural Selection 2: one human commander per team

- **The model.** One player per team may become commander. The commander gets "a top-down view of the map", plays as an RTS, and gives orders to *human* players [W] [Wikipedia](https://en.wikipedia.org/wiki/Natural_Selection_2).
- **How orders reach players.** Orders arrive as waypoints. Build 198 added "New animated marine waypoints, to really grab your attention and make it clear what the Commander is telling you to do" [P] [Build 198](https://www.unknownworlds.com/news/build-198-released-2).
- **Default job.** The same build added: "marines get an automatic build order if they are near an unbuilt structure and don't already have an order" [P] (same link).
- **A single point of failure.**
  - Commander ejection by vote was added in Build 176 (2011) [P] [Build 176](https://unknownworlds.com/news/ns2-build-176-released-2).
  - Players blamed the role for the game's decline: "The commander just doesn't work for public games", and "most of the time in NS2 you're losing and not having fun due to the commander being crap or leaving half way through" [W] [Steam thread, 2015](https://steamcommunity.com/app/4920/discussions/0/611701999530939184).
  - A 2017 official-forum post says "many of even the most experienced players avoid commanding entirely" and calls it one of the most significant "bottlenecks the game faces" [W] [UWE forum](https://forums.unknownworlds.com/discussion/149300/reclaiming-the-value-in-rookie-servers-introduce-non-rookie-coaches-comms).
- **Developer responses.**
  - The 2016 Bootcamp update gated the chair: new players "cannot command until reaching Hive level 3". Bots "can do a passable job at acting as Commanders" on Bootcamp servers [P] [Bootcamp update](https://unknownworlds.com/news/bootcamp-update).
  - Matched Play (Thunderdome) is fixed at "a lobby of 12" and "will not proceed" without two commander volunteers [P] [Thunderdome Q&A](https://unknownworlds.com/news/thunderdome-matched-play-qa).
  - Developer BeigeAlert on training bots: "OUR bots suck" [W, developer's Steam post] [thread](https://steamcommunity.com/app/4920/discussions/0/133256240734281799).
- **Scaling.** Always one commander, whatever the team size. The NSL league requires "at least six players" per team [P] [NSL S10](https://unknownworlds.com/news/nsl-season-10).
- **Health.** All-time peak was 9,429 concurrent players (2013). Today it averages about 63 [W] [SteamCharts](https://steamcharts.com/app/4920).

### Savage (2003) and Savage 2 (2008)

- **Savage 1 is a true hybrid.** One human commander directs human players through waypoints that can be ignored without penalty. The commander also fully controls up to ten AI workers that build, repair and mine [W] [Wikipedia](https://en.wikipedia.org/wiki/Savage:_The_Battle_for_Newerth).
- **Savage 2 moved the load onto people.** Human officers give squad orders and place forward spawns. Commanders are voted in and "can be impeached via team vote". With no commander, builder units can still build [W] [Wikipedia](https://en.wikipedia.org/wiki/Savage_2:_A_Tortured_Soul).
- **The developer's reason, unverified.** A quote attributed to S2 Games reads: "In Savage 1 the commanders overly expansive responsibilities could ruin an entire team's experience" (seen via Softpedia only).
- **Scaling.** Savage 1 had one commander for up to 127 field players and capped workers at 10. Savage 2 reportedly scaled officers at about 1 per 5 players, but sources conflict [W].
- **Outcome.** Savage 1 was criticised for its "lack of any introductory tutorials" [R, via Wikipedia] and went freeware in 2006. Savage 2 went free-to-play within months of launch [W].

### Battlezone (1998), Battlezone 98 Redux (2016) and Battlezone II

- **The model.** Each player drives one vehicle *and* commands AI units, selecting them with the mouse or number keys [W] [Wikipedia](https://en.wikipedia.org/wiki/Battlezone_(1998_video_game)). This is the "everyone is a commander" model.
- **AI quality.** A review: "the AI for friendly units can be a little dicey" [R, quoted on Wikipedia].
- **Battlezone II team mode.** One commander per team, plus "thugs" (wingmen). The commander can hand units over to thugs "to improve their autonomy and allow finer control" [W, unverified: search snippet only] [BZ wiki](https://battlezone.fandom.com/wiki/Strategy).
  - Community advice caps teams at 4 players.
  - Unit-group hotkeys shrink as player count grows: "the number of possible players restricts the number of available F# keys for unit groups" [W] [BZ2 net tips](https://matesfamily.org/bz2/nettips.html).
  - In co-op against AI, the AI gets free scrap as more humans join [W] (same link).
- **Redux.** Supports 2–8 players online [P] [Steam](https://store.steampowered.com/app/301650). Rebellion rebuilt the netcode in 2017 [P] [dev post](https://steamcommunity.com/app/301650/discussions/0/143387886722042642/). All-time peak was 296 players; it now averages about 10 [W] [SteamCharts](https://steamcharts.com/app/301650).

### AirMech (Carbon Games, 2012–)

- **Lineage.** James Green on Herzog Zwei: "he could pick up tanks and give them orders and tell them where to go". Flying in to "grab a unit and move it somewhere else" is "so magic" [I] [Voices of VR](https://voicesofvr.com/?p=2547).
- **Orders ride on the carry.** Units get an order (Attack, Patrol, Capture and others) when they are dropped. Orders add "a key layer of depth", but on easy, plain "grab and drop" wins [R] [Shacknews](https://shacknews.com/article/72090/indie-jeffs-weekly-pick-airmech-alpha). The default infantry order captures the nearest outpost and then guards it [W, unverified snippet].
- **Ownership.** "Each player having their own army and AirMech/Pilot combination really opens up the complementary strategic choices" [I] (Shacknews).
- **Scaling.** The unit cap is per player: about 30 upkeep, raised by holding outposts [W, unverified]. Modes go up to 3v3, with AI filling empty slots [W] [Wikipedia](https://en.wikipedia.org/wiki/AirMech).
- **Outcome.** The game sat six years in Early Access, then split, with competitive PvP becoming the F2P *AirMech Strike* [R] [PC Gamer](https://www.pcgamer.com/au/transforming-mecha-action-rts-airmech-wastelands-leaves-early-access). The console version shut down in 2020 [W] [Delisted Games](https://delistedgames.com/?p=11073). Its all-time Steam peak is about 2,300 [W]. I found no postmortem.

### Halo Wars (2009) and Halo Wars 2 (2017)

- **Controls were the project.**
  - Devine: "one thing we had to get right was the controls." Pottinger: "I wanted one button to do one thing", and each rethink made the controls "simpler" [P] [ViDoc transcript](https://www.halopedia.org/Halo_Wars_ViDoc:_Strategy_on_Xbox).
  - The first "12 to 18 months" went on controls [I] [Worthplaying](https://www.worthplaying.com/article/2009/2/22/interviews/59199-halo-wars-x360-developer-interview-part-1/).
- **Auto-economy, no workers.** Pottinger: "I don't want to task individual peons", and "we don't want lots of really fine-grained micromanagement" [I] (same). Supply depots gather resources by themselves [P] (ViDoc).
- **Selection.** Tap, double-tap or paint selects units; the bumpers select everything on screen or every unit [R] [Wayward Strategy](https://waywardstrategy.com/2020/03/23/halo-wars-the-ultimate-design-for-console-rts/).
- **Criticism.**
  - Giant Bomb: the controls suit "broad tactical strokes" more than micromanagement [R] [review](https://giantbomb.com/reviews/halo-wars-review).
  - Soren Johnson on big armies with a gamepad: "looks like stress, not fun" [I] [designer-notes](https://www.designer-notes.com/?p=55).
- **Scaling.** Caps are per player: 30 or 40 in Halo Wars 1, 80–120 in Halo Wars 2 [W] [Halopedia](https://www.halopedia.org/Population). I found no evidence that caps change with team size.
- **Halo Wars 2 Blitz mode.** No base and no population cap, only energy and a 12-card deck, aiming for 7–12 minute matches [I/R] [Windows Central](https://www.windowscentral.com/halo-wars-2s-moba-blitz-mode-revealed). Reviewers liked the mode [R] [Game Informer](https://gameinformer.com/games/halo_wars_2/b/xboxone/archive/2017/02/17/halo-wars-2-game-informer-review). The fairness complaint was about pay-to-win cards, not AI [R] [PCWorld](https://www.pcworld.com/article/412122/halo-wars-2-review-spinning-its-warthog-wheels-in-a-stagnant-rts-genre.html).
- **Related talk.** "Halo Wars 2: A UX Postmortem" [P] [GDC Vault](https://gdcvault.com/play/1024961). I could not access it.

### Brütal Legend (Double Fine, 2009): hero plus army

- **Command set.** Schafer wanted nothing "so complicated that you couldn't do it with a console controller". After trying complex schemes, the team settled on four D-pad commands: charge, defend, follow, and move to a beacon [I] [Gaming Nexus](https://gamingnexus.com/Article/Brutal-Legend-Interview/Page2/Item2314.aspx).
- **Readability.** Anna Kipnis: "As the units around you get bloodier, that's how you notice that you're doing badly" [I] [Worthplaying](https://www.worthplaying.com/article/2009/8/18/interviews/64457-brutal-legend-ps3x360-developer-interview/).
- **Reception.** Game Informer called commanding "cumbersome" [R] [GI](https://gameinformer.com/games/brutal_legend/b/xbox360/archive/2009/10/13/review). Worthplaying said rushing the stage was the best tactic [R] [review](https://worthplaying.com/article/2009/10/15/reviews/69376-ps3x360-review-brtal-legend/).
- **Expectations failed.** Schafer: "There was definitely a messaging problem with that game" [I via R] [GamingBolt](https://gamingbolt.com/brutal-legend-suffered-because-of-poor-messaging-says-tim-schafer). In 2024 he reflected: "I thought the surprise would be okay because of ActRaiser" [P] [Double Fine](https://www.doublefine.com/news/brutal-memories).

### Sacrifice (Shiny, 2000)

- **The model.** You play a wizard in third person who summons and commands creatures, with formation commands. There are no workers: souls are the resource, and you win by desecrating the enemy altar [R] [GamesFirst](https://gamesfirst.com/articles/2001/preview-sacrifice/); [W] [Wikipedia](https://en.wikipedia.org/wiki/Sacrifice_(video_game)).
- **Gaps.** I found no developer source. Reports of long stalemates are unverified.

### Pikmin 4 Dandori Battle: the control model

The rules are already in `modern-team-buddies-likes.md` §1. This section adds only how the player commands.

- **Leader plus partner.** Kando: "players are able either to control both their character and Oatchi at the same time or to assign a different role to each" [P] [Ask the Developer Vol. 10, pt 2](https://www.nintendo.com/whatsnew/ask-the-developer-vol-10-pikmin-4-part-2/).
- **The partner as a heavy unit.** Oatchi can "throw and call Pikmin" and acts as a "mega Pikmin" [P] [pt 2 (SG)](https://www.nintendo.com/ph/interview/ampy/02.html).
- **Orders as throws.** Units take their job from where you throw them, then haul on their own (see the earlier file).
- **No online play.** Miyamoto reportedly ruled out online play because syncing many small Pikmin, each of which matters, is hard [I, unverified snippet] [Co-Optimus](https://co-optimus.com/article/8437/miyamoto-talks-pikmin-3-s-wii-u-co-op-features.html).

### MOBA lane creeps and pets

- **Creeps are unowned and uncontrollable.** Their numbers scale with the game clock, not with team size or skill.
  - League of Legends: a wave every 30, 25 or 20 seconds by phase, with more cannon minions over time [W] [LoL wiki](https://wiki.leagueoflegends.com/en-us/Minion_(League_of_Legends)).
  - Dota 2 adds siege creeps at fixed times [P] [7.22](https://www.dota2.com/patches/7.22), [7.41](https://www.dota2.com/patches/7.41).
- **Valve is moving pets toward following automatically.**
  - Visage (7.36): Familiars "always follow Visage and cannot receive direct attack or movement commands. They will attack the target Visage tried to attack last". The one exception is that they can still be ordered to cast Stone Form [P] [7.36](https://www.dota2.com/patches/7.36).
  - Meepo (7.33): Poof "can now be put on autocast… all other Meepos also cast Poof to the target automatically" [P] [7.33](https://www.dota2.com/patches/7.33).
  - Chen (7.36): "Only one convert can be summoned at a time" [P] (7.36).
- **Leashes and caps.**
  - Without Scepter, the Lone Druid's bear is limited by its distance from the Druid [P] [7.34](https://www.dota2.com/patches/7.34); [W] for the 1600 figure.
  - Yorick's Mist Walkers (max 4) "instantly die if they move too far" from him [P] [Riot Yorick update](https://na.leagueoflegends.com/en/featured/yorick-champion-update).
- **Riot's fairness principle: predictable AI.**
  - Old Yorick: "his enemy had a constant, obnoxious workload to deal with". A global-raise ultimate was cut as "totally unpredictable".
  - The rework "tests your ability to adapt to predictable AI actions" [P] (same).
  - The opposite worry also exists. Critics said automating Ivern's Daisy would "significantly lower his skill ceiling" [R] [esports.net](https://www.esports.net/news/lol/ivern-changes-step-wrong-direction).
- **Readability.**
  - Riot's clarity rule: "always preserve hierarchy", and "noise should be kept minimal" [P] [Clarity in League](https://www.leagueoflegends.com/en-us/news/dev/clarity-in-league/).
  - Patch 14.10 gave *allies* a clearer health bar for clones ("No more healing the Shaco clone!"). Enemies are still meant to be fooled [I via R] (Riot Yasuna, via riftfeed.gg).
  - Dota gave Spirit Bear "a minimap icon" [P] (7.36).
  - Arc Warden's double got distinct effects in 7.33 and 7.36, partly reverted in 7.38 ("same effect for both") [P] [7.38](https://www.dota2.com/patches/7.38).
- **Skill and rank.**
  - Valve rates 13 of 127 heroes at complexity 3. Five of the 13 are multi-unit heroes: Meepo, Lone Druid, Visage, Arc Warden and Chen [P] [hero list feed](https://www.dota2.com/datafeed/herolist).
  - Win rates by rank, Herald to Divine:
    - Meepo rises from 51.2% to 55.1%, and Visage from 48.4% to 54.9%.
    - Broodmother stays at about 49% and Enchantress at 46–48%. These two have pets that mostly act alone, and they are picked two to three times as often at the lowest rank [W] [OpenDota heroStats](https://api.opendota.com/api/heroStats).
  - Lone Druid went through five rounds of reworks across 7.34 to 7.41 [P] (patches).
  - Azir (LoL) was top-three mid in pro play but sat at about a 45% win rate in average solo queue [I via R] (Phreak, via Dot Esports).

### Awesomenauts (Ronimo): low player counts

This section has little to do with squads, but it is the strongest evidence for the ~200 CCU constraint.

- **The maths.** Joost van Dongen: "tens of thousands of concurrent players are needed for truly awesome matchmaking". With 1,000 players online and filters for region, skill, premades and ranked, "it will take 30 minutes to fill a match!" [P] [Joost dev blog, 2014](https://joostdevblog.blogspot.com/2014/11/why-good-matchmaking-requires-enormous.html).
- **The fallback.** "If the number of players is too low then at some point we just perform matchmaking anyway" [P] [2018 post](https://joostdevblog.blogspot.com/2018/09/the-awesomenauts-matchmaking-algorithm.html).
- **Modes split the population.** "Adding more gamemodes often means splitting the playerbase" [P] [Game Developer repost](https://www.gamedeveloper.com/design/the-requirements-of-good-matchmaking).
- **Outcome.** Awesomenauts went free-to-play in 2017 [P via R] [MCV](https://mcvuk.com/awesomenauts-is-becoming-free-to-play/). Its servers went offline in 2023 [W] [Wikipedia](https://en.wikipedia.org/wiki/Awesomenauts).

---

## 2. Cross-cutting findings by question

### How AI units are controlled

**Analysis:** every successful controller-friendly design settled on a small set of coarse orders with a default job:

| Game | Orders | Default job |
|---|---|---|
| Brütal Legend | 4 commands | — |
| Halo Wars | "one button one thing", plus select local or all | Auto-economy |
| AirMech | One order per unit, given when it is dropped | Capture nearest outpost |
| Pikmin 4 | Orders are throws | Auto-haul |
| Dota Visage (7.36) | Only Stone Form can be ordered | Follow, attack the last target |
| NS2 | Waypoints | Auto-build when idle near a structure |

TB's binocular orders and its "do nothing unless ordered" allies are the outlier.

### Fairness complaints about AI doing the work

- **The direct "AI is doing the work" complaint is rarer than expected.** The best-documented version is Riot's: unpredictable AI hands the *enemy* an "obnoxious workload" (Yorick).
- The data points the other way. In Dota, the heroes whose pets mostly act alone are over-picked at low ranks and win no more often. The heroes that need heavy manual control win more as rank rises.
- **Analysis:** in practice the risk is less "AI carries bad players" and more "self-running pets are a noob trap and a chore to fight". Heavy micro, on the other hand, opens a skill gap that average players lose (Azir, Meepo).
- In commander games, the fairness complaint is about **one human controlling the other humans' fate**: NS2's "commander being crap or leaving", and Savage 1's commander who "could ruin an entire team's experience".

### Readability for opponents

- Sources are thin.
- Riot's rule is hierarchy and minimal noise. Pets get minimap icons (Spirit Bear) and allied tells on clones.
- Deception is reserved for specific kits (the Shaco clone).
- Brütal Legend used the units' visible state (bloodiness) in place of a HUD.
- **Analysis:** for a TB-like, it should be obvious at a glance who owns which unit, and whether a figure is a human or an AI.

### How unit count scales with team size

- **Every per-player unit cap I found is fixed, whatever the team size:** Halo Wars, AirMech, Yorick, Chen and Savage's 10 workers.
- Commander games keep **one** commander per team, whatever the team size, and that seat becomes the bottleneck.
- Battlezone II shows a side effect: as player count grows, each player gets fewer unit-group hotkeys.
- Creep counts scale with the game clock (MOBAs).
- AI difficulty scales with the number of humans (BZ2 co-op gives the AI free scrap).

### What made hybrids succeed or fail in ranked play

- **Success pattern:**
  - a locked format (NS2 Matched Play is 6v6, with commanders required);
  - coarse, readable orders;
  - shorter, base-free variants (Halo Wars 2 Blitz);
  - ownership per player (AirMech, Battlezone).
- **Failure pattern:**
  - a single high-skill seat nobody wants (NS2 and Savage);
  - controls too clumsy for a gamepad (Brütal Legend);
  - expectations not set before launch (Brütal Legend);
  - bad bots, which hurt both training and backfill (NS2);
  - a population too small for the queues the game offered (Awesomenauts).
- **Analysis:** none of these games kept a healthy ranked population for long. Even at their peaks, NS2, Battlezone Redux and AirMech all had small populations in the thousands or fewer.

---

## 3. Candidate squad models for a Team Buddies-like

**This whole section is synthesis.**

### A. Leader plus a small personal squad (TB-faithful, Pikmin- and Visage-style)

- **What it is.** Each player is one hero with 2–3 AI Buddies of their own. Idle Buddies have a default job: follow and guard, then haul crates to the pad and finish builds.
- **Orders.** One context-sensitive point-and-tap button (TB's target table, without binoculars). Optionally hold to send the whole squad.
- **Caps.** Squad size is a fixed per-player cap. Buddies are leashed to their leader or the pad, and owner-coloured.
- **Main risk: readability and noise.** At 3v3 that is 18–24 bodies, and opponents can't tell humans from AI or who owns what. Players who micro well (Meepo, Azir) win far more than average players, and the self-running units are a noob trap (Brood, Enchantress). The squad also amplifies netcode cost and visual clutter.

### B. One character per player; AI only as neutral, team-owned haulers (creep model)

- **What it is.** Players control only themselves, as in Brawl Stars or Splatoon. The team pad produces "worker" Buddies that haul and guard on fixed rules, like lane creeps. No player commands them; they are shaped by what you build and where crates are.
- **Main risk: losing the TB identity.** Pillar S (squad command) goes away. Workers become background, and their AI must be fully predictable or they become the enemy's "obnoxious workload" (Yorick).

### C. Hero plus one partner unit (the Oatchi or Spirit Bear model)

- **What it is.** Each player controls one character and **one** companion. The companion follows by default, can take one coarse order (hold the pad, haul this crate, attack my target), and can be possessed or swapped into.
- **Main risk: a shallow middle.** It may be too little army to feel like TB, yet still double the bodies and add a micro skill gap (the Lone Druid rework churn shows how hard a hero plus one strong pet is to balance). A tight leash and a cap on the partner's power are needed.

### D. Rotating role: one "Sarge" commands the team's pooled AI (NS2-lite)

- **What it is.** The team shares a pool of AI Buddies. Whoever holds a token or seat on the pad gives orders, and the seat can be swapped mid-match.
- **Main risk: the commander bottleneck.** NS2 and Savage document nobody wanting the seat, one bad or quitting player ruining the match, eject-vote drama, and the need for good bots. At about 200 CCU, there is no slack to require a commander volunteer in every match.

**Recommendation.** Prototype **A**, and include **C** as its "squad size = 1" setting, since the same code can test both. Keep **B** as the fallback if readability fails. Drop **D**.

---

## 4. Implications for the squad-model prototype (ticket #10)

**Analysis**, derived from the findings above:

1. **Make squad size a tuning knob** (0, 1, 2, 3 per player), and test at the target team size (3v3 or 4v4), not 1v1. The main risk only appears once there are many bodies on screen.
2. **Default jobs from the first build.** Follow, then guard; auto-haul crates near the pad; finish builds. Never idle. Every order must have a visible tell, such as a salute or a line to the target.
3. **One order button that depends on the target**, with a tap/hold split. Measure whether testers use it at all, given Brütal Legend's "cumbersome" and AirMech's finding that "grab and drop" alone wins.
4. **A readability test.** In freeze-frame screenshots, can an observer tell humans from AI and identify owners in under a second? Try owner-colour rims, AI Buddies at a smaller scale, and minimap pips.
5. **A fairness probe.** Compare a player who never gives orders with one who micros constantly. Target: micro wins, but an idle squad is neither useless nor dominant. This is the Brood/Enchantress check.
6. **Leash and cap** squad units to the leader or the pad (Yorick, Lone Druid). Decide what happens to a dead leader's Buddies.
7. **Bots must exist anyway.** At ~200 CCU, bots fill empty seats (NS2 Bootcamp, AirMech's AI fill-ins, Awesomenauts' "matchmake anyway"). Squad AI and backfill-bot AI should share one brain. NS2's "OUR bots suck" is the warning.
8. **Netcode budget.** Each squad unit is another replicated, physics-carrying entity (Pikmin's online concern, Miyamoto, unverified). Price this into the Bevy netcode choice.

---

## Open questions

1. **Brawl Stars pets** (Nita's bear, Jessie's turret): Supercell's balance reasoning was not researched because the search budget ran out. It is the closest example of an accessible, competitive game with pets.
2. **The Halo Wars 2 UX postmortem** (GDC 2018) was not accessible. It may cover gamepad unit control in Blitz.
3. **No primary source on opponent readability** for NS2, Savage, Battlezone or AirMech.
4. Whether the Ivern Daisy autonomy change shipped, and whether Visage's 7.36 follow-only rule is tied to one Facet: unverified.
5. Battlezone 98's exact order menu and scrap or unit caps: unverified.
