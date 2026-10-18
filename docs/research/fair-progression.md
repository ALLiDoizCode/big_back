# Progression and rewards that keep competition fair

Research notes for wayfinder ticket [#6](https://github.com/ALLiDoizCode/big_back/issues/6). Compiled 2026-10-18.

**Question.** What progression and reward systems make playing feel rewarding without hurting competitive fairness? This covers cosmetic-only progression, sidegrade unlocks, Brawl Stars power levels, battle-pass *structure* (Overwatch 2, Splitgate), ranked season rewards (Rocket League, OW2, Valorant), mastery and challenge systems, and per-match awards. Monetization is out of scope. It appears below only where it explains a design.

Already covered elsewhere, and not redone here:

- Team Buddies' own meta loop (campaign unlocks feeding multiplayer, medals, joke awards): [`team-buddies-core-loops.md` §3](team-buddies-core-loops.md).
- Sea of Thieves' cosmetic-only progression and Deep Rock Galactic's free, replayable seasons: [`modern-team-buddies-likes.md` §8, §9](modern-team-buddies-likes.md).

## How to read this

The source tags match the other research files:

| Tag | Meaning |
|---|---|
| **[P]** | Primary: the developer or publisher speaking for itself (official blog, patch notes, support article), or a peer-reviewed paper |
| **[I]** | A named developer quoted in a press interview |
| **[R]** | Press reporting or opinion. Used only where no primary source turned up |
| **[W]** | Fan wiki or guide site. Leads and numbers only |

Sections marked **Analysis** are my own reasoning, not a source's.

Caveat: pages were read through a page-summarising fetch tool. Quotes should be close to verbatim but not character-perfect, so recheck one before quoting it in public. The web-search budget ran out partway through, so a few points below rest on [R] or [W] sources where a primary one probably exists. They are flagged.

---

## TL;DR

1. **The games converged on one rule: anything that changes a match is free and available to everyone; progression buys only identity.** Overwatch 2 launched with heroes locked behind the battle pass, then reversed that in Season 10 and later removed the new-player hero gate too. Valve concluded in 2009 that random drops were "a bad way to deliver items" to competitive players. Rocket League forced every car body onto one of six standardized hitbox and handling presets. Supercell, whose Brawl Stars keeps real stat power levels, has to keep working around that power in competitive modes: a Power 9 floor in Ranked, Power capped at 9 in Classic events, and "We are not adding Power Level 12".
2. **What players reward is proof of skill, not time.** The ranked rewards examined here require *wins at a rank*: Rocket League needs 10, Valorant 9, and OW2 pays Competitive Points per win. Riot rebuilt Champion Mastery so that it "reflects real accomplishment, not just grind", driven by performance grades. Overwatch rebuilt hero progression because the old one "emphasized overall time played".
3. **Grind complaints were about complexity and friction more than length.** Splitgate dropped tickets, chapters and manual claims for "a traditional, linear track". Supercell called its trophy rewards "unnecessarily complex and not fun". Overwatch called hero progression "unnecessarily complex".
4. **Leaks into match outcomes came from four places:** stat power levels (Brawl Stars), time-gated character access (OW2 Season 1, which needed a competitive delay of "a few weeks"), unlock pacing that puts new players against fully-kitted veterans (Splitgate's 75 Rookie Levels unlock weapons), and farmable mastery metrics (Brawl Stars' "mastery bot" farming).
5. **In-match progression is where fair power lives.** OW2's Perks, and Team Buddies' crates, give real power that resets every match.

---

## 1. Cosmetic-only progression

- **Rocket League.** All cars share the same stats. Each car body uses one of six hitboxes (Breakout, Dominus, Hybrid, Merc, Octane, Plank), which differ slightly in length, width and height ([P] [RL Support: Car Hitboxes](https://www.epicgames.com/help/c-202300000001622/c-202300000001682/rocket-league-car-hitboxes-a202300000018007)). This came about by retrofit. Patch v1.35 introduced "car standardization that minimizes differences in handling and hitbox between cars of the same type" ([P] [v1.35](https://www.rocketleague.com/en/news/patch-notes-v1-35-anniversary-update)), and v1.37 moved "all cars" onto those presets ([P] [v1.37](https://www.rocketleague.com/en/news/patch-notes-v1-37)). Earlier patches had fixed outlier cars, such as the Scarab and Grog hitboxes in 1.16 ([R] [GameStar](https://www.gamestar.de/artikel/rocket-league-update-116-verbessert-hitboxen-und-entfernt-jump-exploit,3271106.html)).
  - **Lesson:** even purely cosmetic bodies leaked into outcomes until the hitboxes were normalized.
- **Overwatch 2 hero progression.** A Progression 2.0 rework (around August 2025) replaced a system described as "unnecessarily complex" that "wasn't resonating correctly" and "emphasized overall time played". The new one awards an Ascended portrait border every 20 hero levels, badges between borders, loot boxes at milestones, and hero-specific name cards and titles. Levels were granted retroactively. Starter kits give hero-specific cosmetics for the first 20 levels, because new players found it "unfun" to be "the only person in the match lobby without a cute outfit". Veterans raised a concern about the information this displays being visible to the enemy team ([P] [Weekly Recall: Gaining Progress](https://overwatch.blizzard.com/en-us/news/24224138/weekly-recall-gaining-progress/)).
- **Sea of Thieves and Deep Rock Galactic** (from the earlier file): Rare made progression cosmetic so that "you've always got a chance". DRG keeps its seasons free and replayable, with no FOMO ([`modern-team-buddies-likes.md`](modern-team-buddies-likes.md)).
- **Splitgate (original)** closed its battle pass in August 2025, gifted every cosmetic to every player and moved to player-hosted P2P servers because 1047 "can no longer afford" the running costs ([R] [Mein-MMO](https://mein-mmo.de/en/popular-shooter-on-steam-releases-the-battle-pass-and-gives-away-all-skins-players-are-now-taking-over-the-servers,1521642/)).

**Analysis.** Cosmetic-only progression is fair only once cosmetics are prevented from changing hitboxes, silhouettes or readability. Rocket League had to add that rule after launch. For a crate game, a skin must not hide what a player is carrying or change what their crate stack reads as.

## 2. Sidegrade and content unlocks

- **Team Fortress 2.** Robin Walker described unlockables as trade-offs: a Sniper unlock "gives up some of his primary advantage" to pull the class closer to its targets ([I] [Shacknews 2009](https://www.shacknews.com/article/58501/valve-talks-team-fortress-2)). He described an unlockable as a cheap way to create a "subclass", since a tenth class is "an order of magnitude more work" ([I] same). On delivery, he said random drops are "only good for some types of things, like the rare cosmetic hats" and that "for competitive players, it's obviously a bad way to deliver items". He floated "a mode where we just give them to you" ([I] [Shacknews, 21 Jul 2009](https://www.shacknews.com/article/59625/valve-still-experimenting-with-team)). When item sales came, the goal was that all gameplay-affecting items are findable, so no one can buy an advantage ([I] [PC Gamer](https://www.pcgamer.com/uk/interview-valve-on-why-theyre-selling-team-fortress-2-items-for-real-money)).
- **Overwatch 2 heroes, a full reversal in three steps.**
  1. **Launch (Sept 2022).** New heroes debuted "at Tier 55 on the free track" of an 80-tier pass and unlocked immediately on premium. To offset this, Blizzard would "delay introducing them into Competitive modes for a few weeks", to tune balance and "give players time to experiment" ([P] [Overwatch 2 Explained](https://overwatch.blizzard.com/en-us/news/23824003/overwatch-2-launches-october-4-as-a-free-to-play-game/)). Ramattra was held out of competitive for two weeks ([P] [Patch notes Dec 2022](https://overwatch.blizzard.com/en-us/news/patch-notes/live/2022/12/)).
  2. **Season 10 (April 2024).** "Heroes will no longer need to be unlocked through the Battle Pass to be playable in all game modes." This was retroactive, and new players still unlocked heroes through the first-time user experience ([P] [Developer Update](https://overwatch.blizzard.com/en-us/news/24062312/developer-update-hero-releases-mythics-and-gameplay-updates/)). Venture went into Competitive immediately ([P] [Patch notes Apr 2024](https://overwatch.blizzard.com/en-us/news/patch-notes/live/2024/04/)). Aaron Keller reportedly said any gameplay behind a paywall "can be a competitive advantage" and that the mitigations were not as "effective as we wanted them to be" ([R] TechRadar, as summarised in search results; I could not read the article).
  3. **August 2025.** "New players will now have access to every Hero in the game from the start!" ([P] [Patch notes Aug 26, 2025](https://overwatch.blizzard.com/en-us/news/patch-notes/live/2025/08/)).
- **Splatoon 3.** Weapons are bought with Sheldon Licenses, earned one per player level-up or weapon Freshness star. The level gate is soft: weapons above your level cost 3 licenses instead of 1 ([W] [Inkipedia](https://splatoonwiki.org/wiki/Sheldon_License)).
  - **Lesson:** a gentle onboarding gate that nobody is locked behind for long.
- **Splitgate: Arena Reloaded (Dec 2025).** 75 "Rookie Levels" replace the faction tracks and unlock "weapons, equipment, character skins"; for example, the Blitz Shotgun now unlocks at level 53. Weapon levels unlock "attachments and new camos". Career Level "is not tied to skill and does not affect matchmaking" ([P] [Diving Deep on Progression](https://www.splitgate.com/news/diving-deep-on-progression/)). Splitgate 2 had been pulled back to beta in July 2025 with progression to be rebuilt "from the ground up" ([R] [Game Informer](https://gameinformer.com/2025/07/22/splitgate-2-returning-to-beta-following-poor-reception-as-developer-1047-games-lays-off)).

**Analysis.** Sidegrades are fair as *design* (TF2) but unfair as *gating*. OW2 needed a competitive delay to cover locked heroes and then removed the locks. Splitgate's weapons at level 53 mean a level-1 player meets level-200 players who hold tools they lack, and at low concurrency they will meet them, because the queue can't be split by level.

## 3. Power levels: Brawl Stars

- **What power is.** Brawlers level from Power 1 to 11. Each level raises stats by roughly 5–10% ([R] [Esports.net](https://www.esports.net/news/mobile-games/brawl-stars-sept-2023-update-2-brawlers-hypercharge)); there was no primary figure in reach. Power 11 unlocks Hypercharges. GM Frank Keienburg said the old Power 11 Gear slot was "Functional, but not exciting by any means". Asked whether Hypercharge was an "I win!" button, he answered "Hypercharge is far from that", and added "we do not strive to have a perfectly balanced game - ever!" ([I] [Pocket Gamer, 20 Sep 2023](https://pocketgamer.com/brawl-stars/frank-keineburg-interview-hypercharge-update/)).
- **The workarounds Supercell keeps adding** (primary evidence that power leaks into outcomes):
  - **Ranked:** "Only Brawlers Power Level 9 and above are allowed", and Diamond I and above needs "at least 12 Brawlers at Power Level 9" ([P] [Ranked Update, 25 Mar 2024](https://supercell.com/en/games/brawlstars/blog/release-notes/patch-notes-ranked-update/)). Ranked 2.0 reportedly requires 12 Brawlers at Power 11 for Mythic and above ([W] [TrophyCoach](https://trophycoach.com/brawl-stars/guides/brawl-stars-ranked-mode-guide)).
  - **Classic Brawl event:** "Brawlers are capped at level 9", first Star Power only, "No Hypercharges, Gadgets or Gears!" ([P] [Classic Brawl patch notes, 25 Jun 2024](https://supercell.com/en/games/brawlstars/blog/game-updates/patch-notes-classic-brawl-update/)).
  - **Nov 2025:** "We are not adding Power Level 12". New power arrives instead as "Buffies" to Gadgets, Star Powers and Hypercharges ([P] [Supercell blog, 26 Nov 2025](https://supercell.com/en/games/brawlstars/blog/news/new-power-brawl-pass-changes-and-a-new-starr-drop-2)).
- **Matchmaking ignores power.**
  - **2024:** matching was on trophies, with "After 1000 Brawler Trophies, players will all be in the same matchmaking pool" because "matchmaking times get too long at high Trophy matches" ([P] [Trophy Season Rework, 15 Oct 2024](https://supercell.com/en/games/brawlstars/blog/news/trophy-season-rework-is-coming/)).
  - **Feb 2026:** "Matchmaking will now be based on hidden MMR decoupled from Trophies", with bots and underdog trophies switched off from 2000 trophies ([P] [Release Notes Feb 2026](https://supercell.com/en/games/brawlstars/blog/release-notes/release-notes-february-2026/)).
  - So two players at the same MMR can bring very different stat totals.
- **Player sentiment.** No official post names a power-level "backlash" outright. Player reports such as "even skilled players still have less hp and dps because they are lower power level" exist on app-store and complaint sites ([W] [Sikayetvar](https://www.sikayetvar.com/en/brawl-stars-us/i-face-unfair-matchmaking-in-brawl-stars-against-much-higher-trophy-players)). The strongest evidence is what Supercell itself built, listed above.

**Analysis.** Power levels work for Brawl Stars only because of mobile-scale concurrency. Supercell can afford floors, caps and separate modes. A ~200-CCU game cannot split its queue by power, so every power gap goes straight into match outcomes.

## 4. Battle-pass structure (not price)

- **Overwatch 2 (launch).** "Up to 80+ unlockable tiers of cosmetics" per season. The free track had 20 tiers spread across the 80, and premium had all 80 ([P] [Overwatch 2 Explained](https://overwatch.blizzard.com/en-us/news/23824003/overwatch-2-launches-october-4-as-a-free-to-play-game/)). In Season 10, weekly challenges began paying Battle Pass XP, and coins appeared on the free track ([P] [Developer Update](https://overwatch.blizzard.com/en-us/news/24062312/developer-update-hero-releases-mythics-and-gameplay-updates/)).
- **Splitgate 2, then Arena Reloaded.** Season 1 of Splitgate 2 split the pass into three time-gated chapters ([R] [Mezha](https://mezha.media/en/reviews/oglyad-bagatokoristuvackogo-shutera-splitgate-2-302827)). The Dec 2025 rebuild changed several things ([P] [Battle Pass Update, 14 Dec 2025](https://www.splitgate.com/news/splitgate-arena-reloaded-battle-pass-update/)):
  - It became "a traditional, linear track that progresses naturally as you play".
  - Tickets now work like XP, and challenges "simply auto-claim".
  - Monthly chapters were removed.
  - There is an optional Epilogue after the main track.
  - The stated reason was that the old pass "asked players to juggle tickets, chapters, and manual unlocks", and the goal was "removing friction and putting the focus back where it belongs: in the Arena".
- **Deep Rock Galactic:** free seasons, all past seasons replayable (earlier file).

**Analysis.** As a progression *structure*, the pass that held up is one linear, auto-advancing, cosmetic track that pays XP for simply playing. Weekly challenges top it up, and the season can be replayed later.

## 5. Ranked season rewards

| Game | Reward gate | Shows | Source |
|---|---|---|---|
| Rocket League | **10 wins** in a tier to earn that tier's reward. To move up, 10 more wins at a higher rank | Season reward items (wheels), title | [P] [Epic support](https://www.epicgames.com/help/en-US/c-Category_RocketLeague/c-RocketLeague_Gameplay/what-are-rocket-league-season-rewards-a000084540) (fetch returned 403; the text is from search extracts) |
| Valorant | **9 wins** in any Act of the season for the rank Gun Buddy. The Act Rank badge fills a triangle per win, at the rank you ended that game with; its border upgrades at 9, 25, 50, 75 and 100 wins. Competitive needs Account Level 20 | Badge on player card, buddy | [P] [VALORANT Wiki](https://wiki.playvalorant.com/en-us/Competitive) |
| Valorant (intent) | Rank buddies "display their best proven skill"; "You need a better reward for your time playing and earning ranked" | | [P] [Ep2 Act I changes, 12 Jan 2021](https://playvalorant.com/en-us/news/game-updates/valorant-episode-2-act-i-competitive-changes/) |
| Overwatch 2 | Competitive Points **per win** (15 for a win and 5 for a draw at launch, later 25 per win), plus an end-of-season bonus by highest tier; 3,000 points buys a Golden Weapon. Limited titles at season end, usable only the next season | Golden weapons, titles | [P] [Initializing systems](https://overwatch.blizzard.com/en-us/news/23857518/initializing-systems-updating-competitive-play-for-overwatch-2/); [P] [Patch notes Apr 2023](https://overwatch.blizzard.com/en-us/news/patch-notes/live/2023/04/) |
| OW2 Competitive Drives | A weekend event: points = 10× rank-percent gained per win, 5× lost per loss. Checkpoints can't be lost; rewards are Competitive Points and cosmetic "Signatures" | | [P] [Competitive Drives](https://overwatch.blizzard.com/en-us/news/24146089/driven-to-victory-introducing-competitive-drives/) |
| Brawl Stars | A Ranked rewards track (Starr Drops with cosmetics), plus ranked card backgrounds that last one season and upgrade if you repeat the same rank, up to 7 times | | [P] [Ranked Update](https://supercell.com/en/games/brawlstars/blog/release-notes/patch-notes-ranked-update/) |

OW2 also batched rank feedback, showing an update "every seven wins, or 20 losses, instead of every single game". This avoids "putting a ton of pressure on each individual game", because "losing games shouldn't be a purely negative experience" ([P] [Initializing systems](https://overwatch.blizzard.com/en-us/news/23857518/initializing-systems-updating-competitive-play-for-overwatch-2/)). The commemorative per-season sprays and icons were removed at the same time (same source).

**Analysis.** The common pattern is a reward for *sustained wins at the rank you reached*, so one lucky climb or a single placement isn't enough. The reward is cosmetic and dated to the season, which proves "I was there". OW2's Drives show the other use of ranked rewards: pulling people into the ranked queue at a chosen time. At low concurrency that is a real tool for filling queues.

## 6. Mastery and challenge systems

- **League of Legends Champion Mastery (Jan 2024 dev blog, live in Patch 14.10).** The old system was capped ("many players are already maxed on their favorite champions") and static ("no way to show off as a master of a particular window in time"). The new goals were "Mastery is for all players, regardless of mode or rank" and "Mastery reflects real accomplishment, not just grind". Score is earned "by playing games as usual (more for higher grades)". Each split gives every champion "a 4-milestone reward track" gated on higher grades, plus a title for the split and a repeating bonus milestone. Riot also admits that "an infinite system can sometimes feel like a grind" ([P] [/dev: Updating Champion Mastery](https://www.leagueoflegends.com/en-us/news/dev/dev-updating-champion-mastery/)).
- **Splitgate weapon mastery.** Weapon Mastery challenges unlock Mastery camos, but "many players never got to experience progressing through these challenges" before the rebuild ([P] [Diving Deep on Progression](https://www.splitgate.com/news/diving-deep-on-progression/)).
- **Brawl Stars.** The June 2024 notes say "We are actively monitoring the 'mastery bot' farming situation", and accounts were being suspended ([P] [Classic Brawl patch notes](https://supercell.com/en/games/brawlstars/blog/game-updates/patch-notes-classic-brawl-update/)). In Feb 2026, win-trading protection was made "more strict at Prestige 2+", and a trophy exploit where "intentionally losing Free Play matches could result in bot matches" was fixed ([P] [Release Notes Feb 2026](https://supercell.com/en/games/brawlstars/blog/release-notes/release-notes-february-2026/)).
- **Theory.** Ryan, Rigby & Przybylski (2006) found that perceived in-game autonomy, competence and relatedness independently predict enjoyment and future play ([P] *Motivation and Emotion* 30(4):347–363, [PDF](https://www.rochester.edu/warner/lida/wp-content/uploads/2022/11/02bfe513dd59366750000000.pdf)). **Analysis:** this is why grade-based mastery ("I got better") lands as rewarding where time-based levels ("I was here a lot") read as grind.

**Analysis.** A mastery track needs (a) a performance signal, not hours; (b) a seasonal reset, so mastery stays current and maxed players have something to chase; and (c) to count only in matchmade games with anti-farm checks. Any countable stat becomes a farming target.

## 7. Per-match awards

- **Team Buddies:** joke awards on the results screen (Friendly Fire, Crate Hog, Kamikaze, Chicken, Panzer General and others) ([`team-buddies-core-loops.md`](team-buddies-core-loops.md), from the game's own data).
- **Splatoon 3:** post-match medals rate a player against their own teammates. At most three are shown, gold ranking above silver (for example, most splats, or "Popular Target" for the player shot at the most). They are visible only to the player who earned them, and they affect nothing except Anarchy Series, where they add points (+1 per silver, +5 per gold) ([W] [Inkipedia](https://splatoonwiki.org/wiki/Medal); [R] [NintendoWire, May 2022](https://nintendowire.com/news/2022/05/27/splatoon-3-matches-offer-medals-as-acknowledgment-of-accomplishments-post-match/)).
- **Overwatch:** OW1's end-of-match voting cards were removed in OW2. Players then asked for them back, and later reports say Blizzard is bringing a reworked version back. I found no primary statement of the reason for removal ([R] [Dexerto](https://www.dexerto.com/overwatch/overwatch-2-blizzard-removed-aspects-no-sense-2161280); [R] [EGW](https://egw.news/overwatch/news/28566/overwatch-2-devs-planning-return-of-ow1s-iconic-en-TjvyxV7uU)).

**Analysis.** Per-match awards are the cheapest "rewarding" layer and almost risk-free for fairness, because they pay nothing that carries into the next match. Splatoon's private medals avoid public shaming. Team Buddies' joke awards celebrate failure ("Kamikaze", "Chicken"), which suits an accessible game where many players lose. Awards can also drive the mastery or challenge track ("earn Crate Hog 5 times this season").

## 8. In-match progression: where power belongs

- **OW2 Season 15 Perks.** Each hero has two Minor and two Major Perks, chosen through two level-ups *within a match*, earned by "dealing damage, securing eliminations, and healing your allies" ([P] [Season 15](https://overwatch.blizzard.com/en-us/news/24175767/season-15-honor-glory-overwatch-2-s-boldest-gameplay-evolution-yet/); per-match detail from search extracts).
- **Team Buddies:** crates turn into weapons and vehicles during the match, and everything resets afterwards ([`team-buddies-core-loops.md`](team-buddies-core-loops.md)).

**Analysis.** The power-growth pleasure that Brawl Stars sells across months can be sold inside one match, where it resets and both teams start equal. The crate economy already is that system.

---

## What players found rewarding vs. grindy (summary)

| Rewarding (evidence) | Grindy or bad (evidence) |
|---|---|
| Proof of skill at a rank: wins-at-rank rewards and the Valorant triangle (§5) | Time-played progression: OW2 "emphasized overall time played" (§1) |
| Performance-graded mastery with seasonal titles: LoL (§6) | Capped or static mastery that was "already maxed": LoL (§6) |
| A linear, auto-claiming pass: Splitgate rebuild (§4) | Tickets, chapters and manual claims: Splitgate 2; "unnecessarily complex and not fun", "the rewards suck!", "Yes, it is grindy": Brawl Stars trophies (§4; [P] [Trophy Season Rework](https://supercell.com/en/games/brawlstars/blog/news/trophy-season-rework-is-coming/)) |
| Starter cosmetics so new players aren't the plain one in the lobby: OW2 (§1) | Hero access behind pass tiers: OW2 S1 (§2) |
| Batched rank updates and lost games still counting toward something: OW2 (§5) | Per-game rank swings: "a ton of pressure on each individual game" (§5) |

## Where progression leaked into match outcomes (summary)

| Leak | Example | The fix the developer shipped |
|---|---|---|
| Stat power levels | Brawl Stars Power 1–11 plus Hypercharges | Power floors in Ranked, power caps in Classic, no Power 12 (§3) |
| Character access gated by time | OW2 Season 1 heroes at tier 55 | A competitive delay, then all heroes free, then no new-player gate (§2) |
| Unlock pacing (new player vs. full kit) | Splitgate weapons unlocking up to Rookie Level 75 | None yet; Career Level kept out of matchmaking (§2) |
| Cosmetic bodies with different hitboxes | Rocket League before v1.35 | Six standardized presets (§1) |
| Farmable progress metrics | Brawl Stars mastery bots, win-trading | Bans, win-trade protection (§6) |
| Rewards that pay out power | TF2 random weapon drops | Valve: "a bad way to deliver items" to competitive players (§2) |

---

## Implications for this game (Analysis)

1. **No persistent power. All power is earned inside the match through crates.** At ~200 CCU we can't bracket the queue by power the way Supercell does, so any persistent stat gap lands directly in outcomes. The crate economy *is* the progression players feel within a match.
2. **Every character, loadout and crate set that affects play is available from the start, or after a short tutorial at most.** OW2 took three steps (pass lock, then the FTUE gate, then nothing) to arrive here. Team Buddies-style unlocks (maps, modes, crate sets) can still unlock through play if they are *playlist variety*, not personal power. Unlocks that open lobby options are fine; unlocks that change your own kit are not.
3. **Cosmetics only, with a readability rule from day one.** Use one shared hitbox per unit class, and keep cosmetics off carried crates and stack shapes, so a skin never hides what someone is carrying or changes what their stack reads as. Rocket League had to retrofit this. Ship a free starter set so new players don't stand out.
4. **Ranked rewards: dated cosmetics for wins at your final rank** (around 10 wins, following RL and Valorant), a visible win history (Valorant's triangle), and batched rank updates (OW2's 7 wins or 20 losses). Use a Drives-style timed event as a lever to fill the ranked queue at low CCU.
5. **Mastery based on grades and resetting per season, counted only in matchmade games.** Base it on performance and on objective play (crates delivered, steals, defences) rather than hours. Guard it against farming and win-trading from the start: a crate game invites "steal 50 crates" farming in collusion lobbies.
6. **Joke awards at the end of every match, private or comedic, which also feed challenges.** This is Team Buddies' own idea, close to free and fair, and it gives losing players something to enjoy. Any battle-pass-like structure should be one linear, auto-claiming track with past seasons kept earnable (Splitgate's rebuild, DRG), with no time-gated chapters.

## Gaps and unverified points

- Brawl Stars' per-level stat percentage is [R], and the Ranked 2.0 Power 11 requirement is [W]. Supercell's Ranked 2.0 patch notes should confirm both.
- The Keller quote on "competitive advantage" is [R] from a search extract. The TechRadar article did not load.
- Rocket League's 10-wins rule comes from search extracts of the Epic support page (the fetch returned 403).
- The reason OW2 removed end-of-match cards has no primary source.
- No GDC talk on progression fairness was reached before the search budget ran out. Candidates for a follow-up are talks on Rocket League's Rocket Pass, Riot's mastery design, and Supercell's progression design.
