# Win conditions and match structure in competitive arena games

Research notes for ticket #3 (map #2). Compiled 2026-10-18. The question: how do competitive team arena games structure a match, and what does each structure do to fairness, pacing and comebacks? The games covered are Brawl Stars, Splitgate, Rocket League, Overwatch 2, Splatoon 3, Valorant and CS2, Pikmin 4's Dandori Battle, and Team Buddies itself. The notes end with the patterns that matter for a crate-economy game whose production base might be destructible.

Team Buddies' own modes are covered in [`team-buddies-core-loops.md`](team-buddies-core-loops.md) (§2 Match loop, §3 Multiplayer), and the Dandori Battle and Splatoon Rainmaker/Salmon Run parallels are in [`modern-team-buddies-likes.md`](modern-team-buddies-likes.md) (§1, §10). Those facts are summarised here, not researched again.

## How to read this

| Tag | Meaning | Trust |
|---|---|---|
| **[P]** | Primary: the developer's own blog, patch notes, dev posts or interviews, the store page, or shipped game config files | High |
| **[P via repost]** / **[P via mirror]** | Official text read through a verbatim mirror (devtrackers.gg, Liquipedia, press reposts), because the official site blocked the fetch tool | High for wording; check the date |
| **[S]** | Secondary: a wiki (Fandom, Pikmin Wiki, wiki.gg), a guide (Game8, TheGamer) or press | Medium. Good for rule numbers, weak for intent |
| **[U]** / **[UNVERIFIED]** | Unverified: a single weak source, conflicting sources, or nothing found | Low. Check before relying on it |

Paragraphs headed **Synthesis**, **Interpretation** or **Takeaway** are my own analysis, not sourced fact.

**Access limits.** rocketleague.com, Inkipedia, Fandom (for direct fetches), Liquipedia (except through its API) and Icy Veins blocked the fetch tool, and the web-search budget ran out in places. The gaps this left are marked [U] and listed in [Open questions](#open-questions).

Sections 2–4 cite sources inline. Sections 5–7 use numbered references `[n]` that point to [Sources for sections 5–7](#sources-for-sections-57).

---

## 0. The answer in brief

**The structures, and what each does:**

| Structure | Games | Pacing | Fairness and snowball profile |
|---|---|---|---|
| **Score race on a clock** (most points at time-out, often with an early "knockout" target) | Brawl Stars Bounty, Hot Zone; Splatoon ranked (count to 100); Dandori Battle; RL; TB Domination and Capture | Bounded and predictable: 2–5 min per match, plus overtime | A lead accumulates. Needs explicit brakes (bounties, penalties, score-proportional theft) or a late swing, or the last minute is dead |
| **Hold-to-win countdown** (reach a threshold, then survive a timer) | Brawl Stars Gem Grab | Short, with a tense ending | Built-in comeback: dying drops every carried gem and resets the countdown, so no lead is safe until the timer ends |
| **Base destruction / HP race** | Brawl Stars Heist and Siege; TB base kills; RL Dropshot (floor) | Fast and bursty | Prone to snowball and burst. Supercell added 3 s safe immunity at every 25% of HP. **Siege (carry bolts → build a robot) had the highest quit rate of any Brawl Stars mode and was removed** |
| **Rounds with a reset** (Bo3, Bo7, first to 13) | Brawl Stars Knockout; OW Control and Stadium; Valorant/CS2; Splitgate Takedown, Firecracker | Longer (Knockout ~3 rounds; tactical shooters 15–40 min) | Each round resets position. Snowball moves into whatever carries over between rounds (economy, Powers), so those systems need loss bonuses, bounties or trailing-team cash |
| **Symmetric tug-of-war** | OW Push, Clash; Splatoon Tower Control | Medium (8 min plus OT) | Push's robot snaps back to the losing side, a built-in catch-up. Clash snowballed ("two fights into a final fight") and was pulled from ranked |
| **Goal plus kickoff reset** | Rocket League; Brawl Ball; TB Bomb-Ball; Splitgate Splitball | 5 min (RL) | Every goal resets all positions and resources. Only the score carries forward, which is the simplest anti-snowball there is |

**Three findings that bear most on our decisions:**

1. **Production snowball is the documented failure mode.** The closest shipped analogue to a crate economy feeding a production base is Brawl Stars **Siege**: teams carried bolts to feed a robot, and the team with more bolts spawned it. Supercell's Frank: "Siege also had the highest (!) termination rate of all modes, regardless of the active maps". It was hard to make work with solo teammates. Removing it cut termination rates by about 15–20% ([P via devtrackers](https://devtrackers.gg/brawl-stars/p/d838a599-tweets-by-frank-supercell-on-21-sep)). Overwatch dropped 2CP because "the attacking team could win in two team fights", and pulled Clash from Competitive for steamrolls (§4).
2. **Carried value should drop on death and be stealable until it "locks in".** Gem Grab (drop all gems on death; the countdown resets), Splitgate Splitball (a delivered ball can be stolen off the plate before it locks in), Dandori Sneak Bomb (theft proportional to score, capped at 100, and spilled as loose weights either side can carry) and Splatoon Rainmaker/Clam Blitz all turn a lead into something physical and contestable. That is the mechanism a crate pad already has.
3. **At low concurrency, consolidate modes hard.** Splatoon rotates one ranked ruleset every 2 hours. Brawl Stars Ranked uses six modes, Bo1 with no bans at low ranks, and merged solo and team queues; it got "around 10 times more players" than Power League. Splitgate forced a "3 modes minimum" casual selection "to ensure healthy player numbers", cut ranked to a single poll-chosen mode, and dropped whole mode families. Rocket League alternates its niche ranked modes by season. Splitgate still shut down its dedicated servers in Sep 2026 ("neither game is generating enough revenue to support the ongoing costs of dedicated servers").

The full implications are in [§9](#9-patterns-for-a-crate-economy-game-with-destructible-production).

---

## 1. Brawl Stars (Supercell)

### 1.1 Modes and win conditions (3v3 versions)

| Mode | Win condition | Timer | Time-out, tiebreak and overtime | Comeback or anti-snowball levers |
|---|---|---|---|---|
| **Gem Grab** | Hold **10 gems** as a team for a **15 s countdown**. The mine spawns 1 gem every 7 s. | **3:30** since 15/12/2020 | The countdown "will stop and reset" if a death drops the team below 10, or if the other team ties the count. If both teams have 10 or more and are tied, no countdown runs until one leads. At 3:30 the team holding the most gems wins. At most 29 gems can be in play, and the mine pauses when 15 lie on the map. | **A defeated player drops ALL carried gems.** A full reset is always possible until the countdown ends. Because of the countdown, the game ends 18 s after the last gem is picked up. |
| **Heist** | Destroy the enemy safe. | 2:30 [S/U] (older guides; not confirmed officially) | No sudden-death overtime. If neither safe is destroyed, the higher remaining **health %** wins. Equal % or simultaneous destruction is a **draw**. | Since 23/04/2025 the safe becomes **immune for 3 s** and knocks back nearby enemies after **every 25%** of its HP is lost. This is an anti-burst, anti-snowball pacing brake. Safe HP scales with the average Brawler power level (since 10/09/2020). |
| **Bounty** | First to **20 stars**, or the most stars at time-out. The first-to-20 rule was added in a rework on 25/04/2023. | **2:00** (cut from 2:30 on 29/01/2019) | A **blue star** sits mid-map. Whoever collects it gets 1 star and the tiebreak, and killing the holder passes it to the other team. Equal stars and an uncollected blue star means a **draw**. | Each player starts with a **2-star** bounty, and each kill adds **+1, up to 7**. When killed, the victim's bounty goes to the enemy team and resets to 2. Leaders become high-value targets, so a team that is behind can swing the score quickly by killing a streaking player. The respawn is 3 s instead of the usual 5 s. |
| **Knockout** | Best of 3 rounds. In each round there is no respawn, and wiping the enemy team wins the round. | Per-round timer not documented [U] | Since 16/06/2021 a **poison gas** closes in from the top and bottom of the map after a while, or when it is 1v1. It replaced the earlier **damage-based** tiebreaker. Since 25/04/2023 gas damage scales with max HP, and since 04/09/2024 it halves healing. | Round resets give a team that lost round 1 a clean slate. In **5v5**, survivors get buffed as teammates die: +20% stats at 1 dead, 153% at 2, 219% at 3 and 418% at 4. This is an explicit catch-up mechanic. |
| **Brawl Ball** | First to **2 goals**, or the most goals at time-out. | ~2:00 to 2:30 [U] | A tie at time-out goes to **1 min overtime** in which **all map obstacles are destroyed**. If still tied, it is a **draw**. | The carrier drops the ball when stunned, knocked back or killed. Respawn has been 7 s since 06/02/2019, and spawns are inside your own goal, which favors the defender. |
| **Hot Zone** | Fully capture all zones. 60 s of occupancy captures a zone, split proportionally on multi-zone maps. | **3:00** | At time-out the higher total capture % wins. Equal % is a **draw**. Both teams can capture at once, and entering a zone does not erase the enemy's progress. | Progress is never lost (since the 13/05/2020 redesign). The early 2020 version let teams disrupt each other's progress, and that was removed. |
| **Siege** (removed) | Destroy the enemy IKE turret, or deal it more damage by time-out. | Cut from 4:30 to **2:30** on 26/06/2019 | The team with more damage dealt wins. | Teams **carry bolts** from mid-map and hold each one for 3 s to "feed" their IKE. At intervals the team with **more bolts** spawns a Siege Bot. Each bolt adds +1 level to the bot (+50 dmg per punch per level). This is the closest Brawl Stars analogue to a crate economy, and it was removed (see 1.4). |

Sources for the table:
- Rule text and dated change logs come from the Fandom wiki [S], read via the MediaWiki API: <https://brawlstars.fandom.com/wiki/Gem_Grab>, <https://brawlstars.fandom.com/wiki/Heist>, <https://brawlstars.fandom.com/wiki/Bounty>, <https://brawlstars.fandom.com/wiki/Knockout>, <https://brawlstars.fandom.com/wiki/Brawl_Ball>, <https://brawlstars.fandom.com/wiki/Hot_Zone>, <https://brawlstars.fandom.com/wiki/Siege>.
- Wikipedia [S] cross-checks Gem Grab ("hold at least ten Gems for 15 seconds"), Bounty (20 stars and blue star), Knockout (first to 2 rounds) and Hot Zone: <https://en.wikipedia.org/wiki/Brawl_Stars>.
- The Heist 2:30 timer comes from iMore and GameWith [S/U]: <https://www.imore.com/brawl-stars-tips-tricks-and-cheats>, <https://gamewith.net/brawlstars/article/show/2222>.

### 1.2 Match length summary
- Most 3v3 matches run **2 to 3.5 minutes**: Bounty 2:00, Heist ~2:30, Hot Zone 3:00, Gem Grab 3:30.
- Knockout matches are 2 or 3 short rounds.
- Siege was cut from 4:30 to 2:30 specifically "rebalanced for the shorter duration," with trophy rewards aligned to other modes' durations (wiki change log [S]).
- Takeaway: Supercell standardizes on short matches and has shortened outliers.

### 1.3 Overtime and draw philosophy
- **Draws are allowed** in Heist, Bounty, Hot Zone and Brawl Ball after overtime.
- Overtime that changes the map exists only in Brawl Ball, where obstacles are destroyed.
- Each mode has its own deterministic tiebreak: Gem Grab uses gems held, Heist uses safe %, Bounty uses the blue star, and Knockout uses gas.
- In Ranked (from 25/03/2024): "Draws no longer trigger replays; points are split based on the Ranked Score difference." In Bo3 at Mythic and above, "if rounds end in a draw, a winner is automatically decided after the third round; there is no rematch."
  - Patch notes [P]: <https://supercell.com/en/games/brawlstars/blog/release-notes/patch-notes-ranked-update/>
  - Ranked wiki [S]: <https://brawlstars.fandom.com/wiki/Ranked>

### 1.4 Snowball vs comeback: what designers did and said

**Siege removal is the clearest data-driven mode removal.**
- Frank (Supercell) posted on 23 Sep 2021 that Siege was dropped from Power League S3.
- His stated reasons:
  - Players reported it was especially hard to make work with teams in Solo.
  - "Factually Siege also had the highest (!) termination rate of all modes, regardless of the active maps."
- Hypothesis: "Would remove Siege improve stats?"
- Result: termination rates fell across the board by about 15–20%. For low-tier Solo players, about 30 of 100 games were terminated before and about 25 of 100 after.
- Source [P, via devtrackers mirror of dev tweets]: <https://devtrackers.gg/brawl-stars/p/d838a599-tweets-by-frank-supercell-on-21-sep>
- Timeline (wiki [S]):
  - Removed from Power League 25/08/2021.
  - Removed from normal rotation 07/03/2022.
  - Removed from the game 31/08/2022.
  - Brief return as a Community Event 05/09/2023 to 24/10/2023.
- **Relevance:** the one resource-carry and production mode (bolts feeding a robot) had the worst quit rate. It was hardest with uncoordinated solo teammates, and a robot that snowballs from the bolt lead is a plausible factor. That factor is our inference, not a quoted reason.

**Engagement data on modes (Frank, "2025 in review" blog [P]):**
- The Brawl Arena test (a longer mode with power scaling and MOBA-style mechanics) "temporarily boosted playtime in general, but quickly normalized." Its popularity fell to roughly the level of **Heist and Hot Zone**. That implies Heist and Hot Zone are low-engagement modes. This is paraphrased from the summary, so confirm the exact wording.
- Frank on Ranked: "Overall we were (and are not) happy with the state of Ranked today."
- Source: <https://supercell.com/en/games/brawlstars/blog/community/2025-in-review-franks-blog-post>

**Rotation pruning (wiki change logs [S]; official reasoning not retrieved [U]):**
- On **05/09/2023**, Heist, Bounty and Hot Zone were all removed from normal rotation and Power League and became "special events." The rotation was reversed within weeks:
  - Heist and Hot Zone went back into Power League on 07/09/2023 and back into normal rotation on 03/10/2023.
  - Bounty went back into Power League on 24/10/2023, but returned to normal rotation only on 10/12/2024.
- This was a rapid reversal of a mode-pool consolidation. A Supercell statement on the reason was not found.

**Mode-mechanic changes that target snowball or stalemate:**
- Heist safe immunity every 25% (23/04/2025). This brakes burst snowballs.
- Knockout tiebreak changed from damage to closing gas (16/06/2021). This forces engagement instead of rewarding a team that is ahead on damage for stalling.
- Bounty "first to 20" (25/04/2023) adds an early-end condition for a dominant team.
- Bounty's +1 per kill up to 7 is a built-in comeback lever: a streaking player carries 7 stars.
- Gem Grab's full drop on death plus the countdown reset means no lead is safe until the 15 s countdown completes.
- All of the above come from wiki change logs [S]; designer rationale was not found [U].

### 1.5 Ranked vs casual, and why

**Power League (14/03/2021) [P]** <https://supercell.com/en/games/brawlstars/blog/release-notes/powerleague-update-patch-note>
- "Best of 3 match format."
- Blind captain bans, then alternating picks. The stated reason for turns: "Pick phase goes in turns - so you can react accordingly."
- A random mode and map per match.

**Power League changes (7 Apr 2022) [P]** <https://supercell.com/en/games/brawlstars/blog/news/power-league-changes-and-improvements-2/>
- Each player gets an individual ban, for 3 to 6 bans total.
- Reasons:
  - "It will make the matches more versatile once the strongest Brawlers for a specific map or mode are out of the way."
  - "We wanted to add more bans, but we didn't want to increase the banning phase time."

**Ranked replaced Power League (27/02/2024 to March 2024) [P]** <https://supercell.com/en/games/brawlstars/blog/release-notes/patch-notes-ranked-update/>
- Format:
  - "From Bronze to Gold III, matches will be in a best-of-1 format without a ban phase."
  - "Diamond I and above continue having a best-of-3 format with a ban phase."
- Separate solo and team queues were removed. Players queue solo, duo or trio.
- **Mode pool (also used for BSC esports):** Bounty, Brawl Ball, Gem Grab, Heist, Hot Zone, Knockout, with 3 maps each.
- So Ranked uses exactly the six core 3v3 modes. Showdown and Siege are excluded.

**"Ranked is dead and Ranked is back" (13 Feb 2025) [P]** <https://supercell.com/en/games/brawlstars/blog/news/ranked-is-dead-and-ranked-is-back/>
- "Ranked has around 10 times more players than its previous version (Power League)". This is evidence that a lower-friction, merged-queue competitive mode boosts population.
- Ranked had become "TOO casual for players who were looking for a challenge."
- "We felt modifiers were complex for casual players to understand" (modifiers removed).
- "At lower Ranks, matchmaking time should become faster"; "At higher Ranks, matchmaking will become much more restrictive to your current Rank."
- "we'll be experimenting with a new game mode every month in Ranked."

**Current structure (Ranked wiki [S])** <https://brawlstars.fandom.com/wiki/Ranked>
- Bronze to Gold: Bo1, blind pick, no bans.
- Diamond: Bo1 with bans, and 9 Power-9 Brawlers required.
- Mythic and above:
  - Bo3 with a snake draft (B3, R3, R2, B2, B1, R1), captain picks last, and a coin flip for first pick.
  - 12 Power-11 Brawlers required.
- Masters and above: solo queue only (17/07/2025).
- The mode pool has churned:
  - Bounty and Heist were removed on 25/02/2025, then returned.
  - Maps per mode rose from 3 to 4 (15 to 24 maps) on 03/03/2025.
  - Hot Zone was swapped with Heist on 17/04/2025, then returned.
  - A "featured mode" adds extra maps each season (19/06/2025).
- Current featured modes [P]: Brawl Ball, then Hot Zone. <https://supercell.com/en/games/brawlstars/blog/release-notes/release-notes-august-2026/>

**Why (synthesis):**
- Ranked keeps only modes with clean, deterministic win conditions and bounded length.
- Drafting is added only at ranks where players have deep rosters and coordination.
- Low ranks stay Bo1 with no bans to keep queues and sessions short.

---

## 2. Splitgate (1047 Games)

### 2.1 Timeline and business outcome (relevant to fragmentation at low concurrency)

| Date | Event | Source |
|---|---|---|
| 2019 / July 2021 | Splitgate (original) breaks out in 2021. | Background |
| 2 Sep 2022 | 1047 ends **new feature development** on the original to go "all-in" on a UE5 sequel. Statement: Splitgate "achieved a level of success that we could not have anticipated"; the studio was spending too much time fixing legacy code. | [S] <https://shacknews.com/article/132135/splitgate-ending-development-1047-games-ue5>, <https://www.thegamer.com/spitgate-ends-development-1047-new-game/> |
| 22 May / 6 Jun 2025 | Splitgate 2 open beta (25 weapons, 15 maps, **10 modes**), then full launch. Launch used **Factions** with abilities: Aeros (mobility), Meridian (support/intel), Sabrask (defensive walls). | [S] <https://en.wikipedia.org/wiki/Splitgate_2> |
| 3 Jul 2025 | **Ranked Arena** launches with **Splitball, Firecracker and Hotzone** only. Domination and Takedown were reportedly excluded as "too unbalanced for competitive settings." | [S] <https://gamingtrend.com/news/splitgate-2-adds-ranked-mode-and-more/>, <https://www.sportskeeda.com/esports/splitgate-2-ranked-mode-all-details-explored> (official post not retrieved [U]) |
| 22 Jul 2025 | "**Unlaunch**": back to beta. Founders Ian Proulx and Nicholas Bagamian: "We launched too early," "we bit off more than we could chew." Plans: rebuild progression, add portals, simplify monetization, **refocus on classic modes**. About 45 layoffs. Steam CCU fell from a 25,785 peak to ~1,839. **Not delisted**: it stayed playable. | [S] <https://www.shacknews.com/article/145217/splitgate-2-developer-apology>, <https://gameinformer.com/2025/07/22/splitgate-2-returning-to-beta-following-poor-reception-as-developer-1047-games-lays-off> |
| 29 Aug 2025 | Original Splitgate moves to **peer-to-peer**. Dedicated servers were sunset: "we can't keep the full online infrastructure running indefinitely." | [P] <https://www.splitgate.com/news/splitgate-1-lives-on-through-peer-to-peer-support> |
| 11 Sep 2025 | "Arena First": **Onslaught** (pulled 18 Sep) and **Battle Royale** (pulled 25 Sep) were removed. Quotes: "Arena combat is the foundation of Splitgate," "we cannot afford to split our focus when it comes to delivering the best possible core experience," "these modes will not be available at relaunch." | [P] <https://www.splitgate.com/news/an-update-on-splitgate-2-arena-first/> |
| 19 Nov 2025 | Final beta playtest. "We removed factions entirely and reimagined abilities as power-ups." Brought back **roundless** King of the Hill, Domination and TDM, plus **Classic Arena**: "an even-starts mode that captures the spirit of original Splitgate." | [P] <https://www.splitgate.com/news/our-final-beta-playtest> |
| 17 Dec 2025 | Relaunch as **SPLITGATE: Arena Reloaded**, free-to-play, with Factions, abilities and some equipment removed. | [P] <https://www.splitgate.com/news/splitgate-arena-reloaded-launch-overview> |
| Jan 2026 | Steam peak around 1–2.3k. Dev response: "Steam Charts don't measure fun." | [S] <https://www.gamingonlinux.com/2026/01/after-a-poor-relaunch-splitgate-arena-reloaded-devs-release-a-statement/page=1/>, <https://hitmarker.net/news/splitgate-2-reboot-arena-reloaded-launches-with-underwhelming-player-numbers-1610517> |
| 27 Mar 2026 | Season 2 adds Arena Royale Duos, Races, Laser Tag and **Classic Arena Ranked**. | [P] <https://www.splitgate.com/news/season-2-patch-notes>, <https://www.splitgate.com/news/season-2-arrives-friday-march-27> |
| **26 Aug 2026** | **End of active development** for Arena Reloaded and Empulse. Dedicated servers and matchmaking shut down **3 Sep 2026**, with peer-to-peer and a server browser remaining. Quotes: "neither game is generating enough revenue to support the ongoing costs of dedicated servers"; "After ten years of focusing on live service shooters, our focus is changing." | [S] <https://www.shacknews.com/article/150495/splitgate-arena-reloaded-empulse-discontinued>, <https://www.destructoid.com/empulse-splitgate-shut-down-devs-pivot-live-service-shooters/> |

**Status as of 2026-10-18:** no dedicated servers or matchmaking, and no further development. The game is playable only via player-hosted P2P lobbies.

### 2.2 Modes and win conditions

**Original Splitgate ranked playlists** (guide [S]): <https://www.esports.net/wiki/guides/splitgate-ranks/>

| Mode | Team | Win condition |
|---|---|---|
| Ranked Takedown | 3v3 | Win **4 of 7 rounds**. A round is won by wiping the enemy team at once. Respawn timers grow with each death. Pistol and carbine start. |
| Ranked Domination | 4v4 | 3 zones; first to **600** points. |
| Ranked King of the Hill | 4v4 | Moving hill; most points at the end. A KOTH limit of 200 is cited elsewhere [U]. |
| Ranked TDM | 4v4 | First to **50 kills**. |

- All ranked 4v4 modes used even starts (pistol and carbine) with radar disabled.
- Each mode had its own separate ELO.
- Casual-only modes included Oddball and others.

**Splitgate 2 / Arena Reloaded** (official where marked; otherwise the Arena Reloaded wiki [S] <https://splitgatearenareloaded.wiki.gg/wiki/Arena>)

| Mode | Win condition and structure | Overtime / tiebreak | Notes |
|---|---|---|---|
| **TDM** (2025 launch) | **Best of 5 rounds; a round ends at 20 kills** [S, Shacknews preview] | — | The roundless TDM returned in Nov 2025 [P] |
| **Hotzone** | Zones charge by occupancy. Each full bar scores, and the zone moves; **first to 2** (described as "three rounds") | Competitive variant is "best of seven rounds" [S] | Wikipedia: respawn time **rises during a match** [S] |
| **Firecracker** | Plant or defuse at 2 sites, **with respawns**; Bo5; sides swap each round | Round 5 casual: **no-respawn sudden death**. Round 5 Ranked: **"Neutral Firecracker"**, with one bomb mid-map, a site at each base, and both teams able to attack | The ranked tiebreaker is symmetric attack/defend |
| **Splitball** | Carry a neutral ball to your **capture plate**; **first to 3**; **no match timer** | No timer means no overtime | **"Balls can be stolen from the enemy's plate before they lock in and score"**. Carriers can't shoot but can throw the ball. Directly analogous to stealable crates on a pad. <https://splitgatearenareloaded.wiki.gg/wiki/Splitball> |
| **Domination / KOTH** | Point-race on zones | — | Excluded from SG2's first Ranked; later formed Classic Arena Ranked (KOTH, DOM, TDM) |
| **Takedown** | Round-based wipe; respawn delay grows over time | — | Became the Ranked mode on 13 Feb 2026 |
| **Arena Royale** (24 players) | "Be the last team standing - respawns are enabled, but your team is knocked out if everyone is killed at once." Respawns are disabled at the very end. | Ring closes in stages | **Kills shorten your respawn timer; deaths lengthen it; timers grow as the match goes on** [P] <https://www.splitgate.com/news/update-2-2-patch-notes> |
| Onslaught, Battle Royale | Large-scale modes | — | **Cut 18 and 25 Sep 2025** to focus on Arena [P] |
| Showdown | A June 2025 creator team event: "you pick a team and earn points by getting kills" | — | [P] <https://www.splitgate.com/news/splitgate-showdown> |

Unverified [U]: exact time limits and score caps for SG2 TDM, Domination and KOTH. A 50-kill TDM cap is cited in some places.

### 2.3 Snowball, comeback and fairness: designer statements

"No Heroes. No Abilities. No Limits." [P] <https://www.splitgate.com/news/no-heroes-no-abilities-no-limits>
- On Factions: they "made it harder for players to play the way they wanted and did not support our arena-focused gameplay."
- On abilities:
  - "They could create big moments, but they were often frustrating to play against."
  - Abilities "sometimes felt like a get out of jail free card when someone made a mistake."
- The principle: **"any spike in power should come from map knowledge and positioning, not from a hidden loadout choice."**
- On Classic Arena:
  - "Every player starts on equal footing."
  - "Power is decided by positioning, timing, and how well you can use portals to outplay the other team."

Mechanical anti-snowball levers seen in Splitgate:
- **Escalating respawn timers.** Takedown's respawn grows with each death, and Arena Royale timers grow over the match while kills shorten them and deaths lengthen them. These shorten the endgame. Note that the "kills shorten" rule is pro-snowball.
- **Round resets** in Takedown, Firecracker and the 2025 TDM.
- **A symmetric neutral-bomb final round** in Ranked Firecracker.
- **Stealing from the scoring plate before lock-in** in Splitball.

No direct designer quote on comebacks or snowballing was found [U].

### 2.4 Ranked vs casual, and why (including playlist consolidation at low population)

**Mode Select with a forced minimum (Dec 2025) [P]** <https://www.splitgate.com/news/splitgate-arena-reloaded-launch-overview>
- "Players will be required to select 3 modes minimum at launch to ensure healthy player numbers across available playlists". The plan was to relax this once matchmaking is fast.
- Multi-queue across playlists gives a **cumulative XP bonus**.
- This is a direct low-concurrency tool: it lets players choose modes while preventing them from fragmenting the pool.
- Stated goal: "Play what you want instead of hoping you get your preferred mode in a random rotation" [P] (No Heroes post).

**Ranked composition history:**
1. Original Splitgate: Takedown 3v3 plus a 4v4 Ranked queue (Domination, KOTH, TDM), all even-start [S].
2. SG2 July 2025: Splitball, Firecracker and Hotzone. Domination and Takedown were excluded as "unbalanced" [S].
3. Arena Reloaded launch: the Ranked overhaul added 10 placements, a soft reset and leaderboards [P]. The Ranked playlist was "KOTH, DOM, and TDM with loadouts" [P, Ranked Refresh].
4. **13 Feb 2026, "Ranked Refresh"** [P] <https://www.splitgate.com/news/ranked-refresh>
   - The Ranked playlist became **Takedown only**: 3v3, even starts (Cadence Carbine and Blink Pistol), no custom loadouts.
   - It was chosen by **monthly community poll**: "Takedown was the winner of this month's poll!!!"
   - Reasons: "intended to keep the core arena experience feeling fresh and challenging"; "Takedown is equally competitive in its own right."
5. Season 2 (27 Mar 2026): **Classic Arena Ranked** (KOTH, DOM, TDM with carbine and pistol starts) [P] <https://www.splitgate.com/news/season-2-patch-notes>.

**Interpretation (ours, not quoted):** the studio repeatedly shrank and rotated the ranked pool to one mode or a narrow set. It forced multi-mode queueing and cut whole mode families (BR, Onslaught) to concentrate players and development. This did not rescue the game: 1047 still cited revenue that could not cover dedicated servers.


---

## 3. Rocket League (Psyonix)

### 3.1 Modes and win conditions

| Mode / playlist | Team size | Win condition | Ranked status (history) |
|---|---|---|---|
| Soccar: Duel, Doubles, Standard | 1v1 / 2v2 / 3v3 | Most goals after a 5:00 clock; if tied, golden-goal overtime | Always ranked, each with its own MMR |
| Hoops | 2v2 | Goals into a raised hoop | Ranked under "Extra Modes" since Season 9 (2018) [S] |
| Rumble | 3v3 | Soccar plus random power-ups | Ranked under Extra Modes since S9 [S] |
| Snow Day | 3v3 | Soccar with a hockey puck | Ranked since S9. From Nov 2023 it **alternates each season with Dropshot** [P via repost] |
| Dropshot | 3v3 | Break the floor panels on the enemy half, then drop the ball through a hole (see 3.6) | Ranked since S9. Alternates with Snow Day from S14 on [P via repost] |
| 4v4 Chaos | 4v4 | Soccar | Casual only. In Nov 2023 it was folded into a weekly-rotating "Big Party" playlist with Knockout [P via repost] |
| 4v4 "Quads" | 4v4 | Soccar | Ranked for **Season 20 only** (Sept 2025). In S22 (Mar 2026) it left ranked for Casual only [P via repost / S] |
| Heatseeker (v2) | — | Ball homes toward goals | A new ranked v2 Heatseeker mode replaced Quads in S22 [S] |
| Tournaments | 2v2 / 3v3 | Bracket events. S20 and later award Tournament Credits; S22 added Weekly Cash Cups | [P via repost] |

Sources:
- Season 9 Extra Modes ranked rules: separate MMR per playlist, 10 placement matches, the modes removed from the Casual tab, no leaderboards in S9. https://mynintendonews.com/2018/09/24/rocket-league-season-9-patch-notes/ [S repost of official notes]; https://www.rocketleague.com/news/season-8-rewards-season-9-details [P, 403 to the fetch tool, cited from the search index]
- Esports Tales says the modes left Casual because offering them in both Casual and Ranked "could make matches hard to find". This is the closest thing found to a population-split rationale. https://www.esportstales.com/rocket-league/seasonal-rank-distribution-and-players-percentage-by-tier [S, no official quote found]
- Play Menu post by Psyonix, Nov 21 2023. https://rocketleague.com/news/first-look-play-menu-changes-coming-to-rocket-league [P]. Mirror: https://devtrackers.gg/rocket-league/p/5f93f4a3-first-look-play-menu-changes-coming-to-rocket-league. Quotes:
  - "4v4 Chaos is evolving into a new Big Party Playlist, rotating weekly between 4v4 Chaos and Knockout."
  - "As part of this consolidation, we are going to alternate between Competitive Snow Day and Dropshot each Season."
  - Note the word *consolidation*. **The mirror text gives no explicit reason (queue times or population).** The population-split motive is inferred [U].
- Season 20 (v2.56, Sept 17 2025). https://devtrackers.gg/rocket-league/p/7d89106d-rocket-league-patch-notes-v2-56-season-20-live (mirror of https://www.rocketleague.com/news/rocket-league-patch-notes-v2-56-season-20-live) [P]. Quotes:
  - "For Season 20, Competitive Dropshot replaces Competitive Snowday."
  - "For Season 20 only, we've also added the new 4v4 (Quads) to the Competitive rotation."
  - "a soft rank reset has been applied to all Competitive Playlists."
  - New Driver Challenges gate ranked play, and "should also reduce instances of smurfing".
- Season 22 (v2.66, Mar 11 2026). https://www.rocketleague.com/news/rocket-league-patch-notes-v2-66-season-22-live/ [P, 403 to the fetch tool]; https://www.strafe.com/news/read/rocket-league-patch-v2-66-visible-mmr-flip-reset-indicator-and-all-key-changes/ [S]. Details:
  - Snowday stays and Ranked Dropshot returns.
  - Quads moves to Casual and v2 Heatseeker enters ranked.
  - Soft reset.
  - **Opt-in visible MMR**: shown only to yourself, per playlist.
  - The S22 rotation seems to break the strict alternation, since both Snowday and Dropshot are listed. **No reasoning was given for any of these playlist moves.**
- Season 23 and later (mid-2026 onward): not checked [U].

### 3.2 Match length

- Each match has a 5:00 regulation clock. This is the default setting in every ruleset found: https://athletics.carleton.ca/wp-content/uploads/Rocket-League-Ruleset.pdf ; https://recsports.ufl.edu/wp-content/uploads/2021/02/Esports-Sport-Rules.pdf [S]
- When time expires, the match does not end until the ball touches the ground. Source is a player report: https://steamcommunity.com/app/252950/discussions/0/528398719789904135 [S]
- The clock stops during goal replays and kickoff countdowns, so a real match lasts about 6–8 minutes [U].
- **No Psyonix designer statement on why 5 minutes was chosen was found** [U].

### 3.3 Overtime and tiebreak

- If the score is tied at 0:00, the match goes to sudden-death overtime: the next goal wins. https://recsports.ufl.edu/wp-content/uploads/2021/02/Esports-Sport-Rules.pdf [S]
- Overtime has no time cap in online play. There are no draws, and the OT clock counts upward [U, consistent with the rulesets above, but no official text was found].

### 3.4 Kickoff reset (built-in anti-snowball)

After every goal there is a replay, and then all cars return to fixed, mirrored kickoff spawns. The ball returns to centre and boost resets. A team's possession or positional advantage is wiped every time it scores. The only state that carries forward is the scoreline and the clock [U, from game knowledge; no official text found because rocketleague.com is blocked]. In Dropshot, damage to the floor may carry across goals (see 3.6; unverified).

### 3.5 Forfeit rules

- **v1.04 (2015):** Psyonix introduced the forfeit vote: "the players left in the game can now vote to forfeit the match when playing in 'Ranked' online matches." https://attackofthefanboy.com/news/rocket-league-update-1-04-released-adds-free-map-3v3-team-ranked-forfeit-option-and-spectate-mode/ [S]
- **v1.50 (29 Aug 2018):**
  - "A forfeit cannot be triggered until 90 seconds have elapsed in any competitive match. (Excluding 1v1 matches.)"
  - A forfeit counts as a completed match, so XP is still earned.
  - Coverage frames the 90-second floor as a fix for teams forfeiting right after an early goal.
  - Sources: https://upcomer.com/rocket-league-update-1-50-notable-changes ; https://www.newsweek.com/entertainment/video-games/rocket-league-15-update-patch-notes-progression-rocket-pass-1096490 [S reposts of the official notes]
- **Casual concede:** NME reported Psyonix planned to let a team concede a casual match "if all players on a team vote to concede", alongside bans for quitting casual matches. https://www.nme.com/news/psyonix-will-soon-ban-rocket-league-players-who-quit-casual-matches-3015538 [S, ship date not verified]
- The vote threshold (unanimous versus majority in 3v3) and any change after 2018 could not be verified [U].

### 3.6 Dropshot: destructible floor as the win condition

Sources: official announcement, Mar 2017, https://www.rocketleague.com/news/dropshot-new-game-mode-march-22 [P, 403 to the fetch tool, details taken from the search index]; Epic help page, https://www.epicgames.com/help/en-US/c-202300000001622/c-202300000001682/a202300000017643 [P, 403]; https://www.dualshockers.com/rocket-league-new-free-mode-dropshot-march-22nd/ [S].

- **Arena and goal:** The arena is the hexagonal "Core 707" and has no goals. To score, you damage the floor panels on the opponent's half and then knock the ball down through an opened hole.
- **Panel states:**
  - A panel takes two hits.
  - The first hit "activates" (cracks) it.
  - The second hit breaks it open.
- **Ball charge:**
  - The ball has 3 phases, and it moves up a phase as players keep hitting it while it stays in the air.
  - The phase sets how much damage it does when it lands on the enemy floor: 1 tile in phase 1, up to 7 in phase 2, and up to 19 in phase 3.
- **Scoring:** "works just as it would in a standard match", so the same 5:00 clock and golden-goal OT apply. Damage is a new tracked stat. The per-tile point value is 10 according to a user-compiled list [U]: https://steamcommunity.com/app/252950/discussions/0/1627412171722898765
- **What happens after a goal** [U]:
  - After a goal, the ball returns to neutral charge.
  - The floor of the side that conceded is reportedly repaired, while damage on the other side persists.
  - This needs verifying before we use Dropshot as a precedent.
- **Relevance to our design:** Dropshot is the clearest shipped example of "destroy the enemy's terrain to open the win condition". Its offence builds up across hits (charge), and its defence is spatial (covering the damaged tiles).

### 3.7 Snowball versus comeback (Rocket League)

- Three mechanisms resist snowballing: the kickoff reset after every goal, a short fixed clock, and golden-goal OT. Goal state never compounds into stronger cars or more resources. Psyonix offers no catch-up mechanic beyond these.
- The forfeit vote is the counterweight for hopeless matches. The 90-second minimum was added so teams don't forfeit after a single early goal.
- **No direct Psyonix designer quotes on comebacks, match length or overtime philosophy were found** before the search budget ran out [U]. A follow-up should look for GDC talks by Psyonix (Corey Davis, Jeremy Dunham).

---

## 4. Overwatch 2 (Blizzard)

### 4.1 Modes and win conditions

The detailed numbers (capture rates, time added per checkpoint) come from wikis that blocked the fetch tool. Unless a row says otherwise, the rules below are [S/U] and should be checked against the game itself.

| Mode | Win condition | Round structure | Timer / overtime | Ranked? |
|---|---|---|---|---|
| **Control** | First team to win 2 rounds. A round is won by filling the capture meter to 100% while holding the single point | Best of 3. Each round is on a different sub-map | When one team is at 99% and the point is contested, the round goes to overtime. **S2 (Dec 2022) [P]:** the team that loses control of the point is treated as contesting it, so retaking at 99% triggers OT unless the other team touches the point again | QP + Comp |
| **Escort** | Attackers push the payload through checkpoints to the end. Defenders stall | 2 halves (attack / defend). If both teams complete the map, a time bank gives extra rounds | Reaching a checkpoint adds time. OT continues while attackers are near the payload | QP + Comp |
| **Hybrid** | Capture point A, which unlocks the payload, then escort it through the remaining checkpoints | Same as Escort, with a time bank | Same as Escort | QP + Comp |
| **Push** (replaced Assault, 2022) | Escort the TS-1 robot pushing *your* barricade toward the enemy spawn. Furthest push wins | Single 8-minute round on a symmetric map | OT runs while TS-1 is contested, or while the team holding TS-1 is not furthest. It ends when the team holding TS-1 loses control or overtakes the opponent's distance [P] | QP + Comp |
| **Flashpoint** (S6, Aug 2023) | First team to capture 3 of 5 points | One continuous round. The centre point unlocks first, and later points are drawn at random | OT works like Control (contested at 99%). Spawns are re-placed so they are about equidistant from the active point [S] | QP + Comp (Comp from 18 Aug 2023) [S] |
| **Clash** (trial S10, Apr 2024; full release S12) | Score 5 points, or hold all 5 objectives at once | Linear tug-of-war over 5 points on a mirrored map, one point active at a time. In S12 the end points A and E were split into 3 segments [P] | Has a time limit [S] | QP + Comp at release, then **removed from Comp in S15 (2025)** [P] |
| **Assault / 2CP** | Attackers capture A then B | Halves plus a time bank | OT on contest | **Removed** from QP and Comp at OW2 launch [P] |
| **Stadium** (S16, Apr 2025) | Round-based. **Best of 7 (first to 4)** in Competitive | Each round is short Control, Push or Clash. Buy phase between rounds | See 2.5 | Separate Stadium ranked ladder [P] |

Sources:
- Push rules, Assault removal and 5v5 change: OW2 PvP beta patch notes, Apr 2022. https://overwatch.blizzard.com/en-us/news/patch-notes/beta/2022/04/ [P]
  - Push timer and win rule: "Players have 8 minutes to push their team's barricade to the end goal and win" / "the team who pushed their barricade the farthest distance AND has control of TS-1 wins."
  - Push OT start and end: "The game enters overtime if control of TS-1 is contested" / "Overtime ends when the team with TS-1 loses control of it" or "pushes their barricade to a distance exceeding the opposition's."
  - **Push catch-up mechanic:** when a team takes TS-1, it "quickly moves back along the path until it reaches the opposing team's barricade". A team that has fallen behind doesn't have to walk the robot back.
  - Push has a forward checkpoint, and reaching it activates a closer respawn. That respawn deactivates if the other team retakes TS-1.
- Control OT change: Season 2 patch notes. https://overwatch.blizzard.com/en-us/news/patch-notes/live/2022/12/ [P]
- Flashpoint rules:
  - https://www.dexerto.com/overwatch/overwatch-2s-flashpoint-how-it-works-2176234/ [S]
  - https://www.sportskeeda.com/esports/overwatch-2-flashpoint-mode-features-play-maps [S]
  - Patch notes, Aug 2023: https://overwatch.blizzard.com/en-us/news/patch-notes/live/2023/08 [P, not fetched]
  - One guide puts the point capture at about 70 seconds, compared with 120 in Control [S/U].
- Clash rules:
  - S10 notes via https://mp1st.com/news/overwatch-2-season-10-patch-notes-new-mode-new-ways-to-unlock-rewards-detailed [S]
  - https://www.techradar.com/gaming/consoles-pc/overwatch-2-season-10-makes-all-heroes-free-adds-new-clash-mode-and-map [S]
  - Details from the notes: "five total objectives placed in a linear pattern on a mirrored map". Matches start at the centre point. If a team is pushed off a point and the other team captures it, the next active point is in the opposite direction.
- Escort/Hybrid time bank and draws: https://www.start.gg/tournament/b-u-gamerz-fall-spring/event/ow2-5v5-round-robin/overview/rules [S]
  - The time bank sets "the team's remaining attack time after the first round".
  - Draw rules are inconsistent across sources. Older wiki text describes a draw if neither team captures past 33% on Hybrid/Assault, and some guides describe an OW1-era "Sudden Death". **The current OW2 draw rule is unverified [U].**
- **Overtime decay:** a July 2016 OW1 patch made the overtime fuse deplete "much faster after 20s of Overtime". https://www.over.gg/post/23373 [S]
  - The fuse drains only while no attacker is contesting, and refills on contest [S/U].

### 4.2 Match length targets

- Alec Dawson in a Director's Take on Clash (S12): "No one likes it when queue times exceed your match time, and some Clash matches end very quickly." https://news.blizzard.com/en-us/article/24140358/directors-take-juno-clash-and-a-look-ahead-to-midseason [P]
  - Blizzard listed its adjustment priorities in order: reduce the stomp rate and very short matches, get attackers to go for A and E, improve fairness on B and D, and keep the mode clear.
  - Dawson: "Giving teams an additional chance or two to not get blown out will be quite healthy for Clash overall."
- Keller in a Director's Take (early 2025): Clash had "some matches having a team steamroll and ending much faster than a typical Overwatch game." https://news.blizzard.com/en-us/article/24167662/directors-take-looking-back-and-looking-ahead [P]
- **No official target length in minutes was found** [U]. Commonly cited figures are about 10–20 minutes for a standard match, and 8 minutes plus OT for Push.

### 4.3 Snowball versus comeback: designer statements

**Why Assault/2CP was removed**
- Beta patch notes, Apr 2022 [P]: "the Assault game mode has always been a source of discontent for many members of our community." Fixing it would have required major changes and full rebuilds of the maps, so the team decided to "remove all the maps using the Assault game mode from both the Quick Play and Competitive map rotations." https://overwatch.blizzard.com/en-us/news/patch-notes/beta/2022/04/
- Aaron Keller, developer panel, as reported Aug 2023 [S with direct quotes]. https://www.ggrecon.com/articles/overwatch-devs-reveal-why-controversial-mode-was-cut-from-overwatch-2
  - "If you think about 2CP, the attacking team could win in two team fights."
  - "As far as 2CP goes, that to me wasn't a high-skill or a low-skill issue."
  - "We really weren't able to solve it without totally redoing those maps."
- Paraphrase via BlizzardWatch [S]: 2CP was too favourable to defenders, yet an offence that snowballed the first point almost always took the second. https://blizzardwatch.com/?p=176198
- **Takeaway:** 2CP failed in both directions. Either the defence stalemated the attack, or one won fight let the attack snowball through two fights and end the map.

**Why Push and Flashpoint were made**
- **Push** replaced Assault and uses a symmetric tug-of-war. The robot snapping back to the enemy barricade is a built-in catch-up mechanic, and the forward-checkpoint respawn rewards progress [P, beta notes above].
- **Flashpoint** was a response to Push feedback. It uses double symmetry (left-right and top-bottom), many flank routes, and spawns re-placed so they are equidistant from each new point. https://comicbook.com/gaming/news/overwatch-2-devs-negative-push-feedback-crafting-flashpoint-mode [S with direct quotes]
  - Daniel McGowan, Principal Environment Artist: "The ability to turn it around mid-game, late-game is definitely there with (Flashpoint)"
  - McGowan, on balance: "Even if there isn't an advantage, is there the perception of an advantage?"
  - McGowan, on Push: "In Push there's a very clear front, right and it's pretty linear"
  - Ryan Smith, Lead Level Designer: "The game modes aren't created equally."
  - Smith: "we learn as we go, and these game modes are ever evolving."

**Why Clash was made, and why it left Competitive**
- Keller said Clash was inspired by the original Assault maps, but lets both teams attack and defend throughout the match, with "back-and-forth gameplay and a clear time limit". https://cogconnected.com/2024/03/overwatch-2-navigating-changes-and-renewed-direction/ [S paraphrase]
- Alec Dawson, Director's Take, S12 [P]: "we wanted it to contain more constant action than other modes" and "A defending team can snowball two fights into a final fight at the center point". The fix was splitting A and E into 3 segments "to make it easier to acquire at least a single point on A or E." https://news.blizzard.com/en-us/article/24140358/directors-take-juno-clash-and-a-look-ahead-to-midseason
- Keller, Director's Take, early 2025 [P]: "starting in Season 15, Clash maps are going to be removed out of Competitive Play." The reasons were steamrolls and "It can also be confusing as to which point is active for some players." Clash stays: "we are going to keep Clash in Quick Play and other unranked game modes." https://news.blizzard.com/en-us/article/24167662/directors-take-looking-back-and-looking-ahead
  - **Pattern:** a mode that snowballs is demoted to casual, not deleted.

**Control best of 3**
- Each round is on a different sub-map and starts fresh at 0–0%, so it works as a full reset in the same way as a Rocket League kickoff.
- The team that loses round 1 has a whole new round to come back.
- No designer quote on this was found [U].

**The 5v5 change (2022)**
- Beta notes [P]: with fewer players, "individual contribution means more to the overall success of the team". It "increases gameplay clarity for all hero roles." https://overwatch.blizzard.com/en-us/news/patch-notes/beta/2022/04/
- Keller at the 2021 reveal, paraphrased by press [S]: it was "just too hard to read what 11 other players are doing". The team also tested 4v4, 3-1-1 and 7v7. https://www.invenglobal.com/articles/14083/overwatch-2-multiplayer-pvp-reveal-5v5-format-with-only-one-tank
- Blizzard later reopened the question: 6v6 tests in 2024 and a 6v6 mini-season in mid-S15. https://www.thesixthaxis.com/2024/07/26/overwatch-2-could-be-going-back-to-6v6-as-blizzard-plans-gameplay-tests/ [S]; Director's Take above [P]

### 4.4 Ranked versus Quick Play mode pools

- Competitive uses Control, Escort, Hybrid, Push and Flashpoint. Clash was in Competitive only from S12 to S14 [P].
- Quick Play adds Clash and arcade modes [P, Director's Take 2025].
- **Stated principle (Clash):** a mode with a high stomp rate or unclear objectives is pulled from ranked so it can be reworked, while it stays in QP to keep collecting data.
- Stadium has its own ranked ladder, separate from Competitive Play. Ranks reset to the bottom of Rookie League each season, and "Each League will become more difficult as the risk of losing progress increases." https://overwatch.blizzard.com/en-us/news/24188046/enter-the-stadium-get-ready-to-battle/ [P]

### 4.5 Stadium (S16, Apr 2025): round-based buy economy (most relevant to us)

Sources:
- Launch blog [P]: https://overwatch.blizzard.com/en-us/news/24188046/enter-the-stadium-get-ready-to-battle/
- Strategy blog [P]: https://news.blizzard.com/en-us/article/24191424/
- Oct 2025 patch notes [P]: https://overwatch.blizzard.com/en-us/news/patch-notes/live/2025/10/

**Format**
- "Stadium is the new best-of-7, multi-round way to play Overwatch 2."
- The format is 5v5 (1-2-2), with each player locked to one hero for the whole match.
- Rounds are played on 9 maps using Control (one round, with faster capture), Push (about half the push distance) or Clash ("will play until a team wins two of those objective points"), each in a modified form.

**Powers (free, permanent)**
- Players choose a Power at the start of rounds 1, 3, 5 and 7, from 12 per hero.
- "Powers don't cost you Stadium Cash to equip, but once you equip a Power, it cannot be removed, so plan carefully."

**Items (bought with cash)**
- Players buy Items in the Armory between rounds, with up to 6 slots.
- Items come in three tiers: Common, Rare and Epic.
- Items fall into three categories: Weapon, Ability and Survival.
- Items can be sold back at full price, so a player can rebuild their loadout freely.

**Cash**
- Players start with 3,500 Stadium Cash.
- Cash is earned from damage, healing, assists and eliminations, plus a round-completion payout "regardless of a win or loss".
- Every 500 cash earned gives +1 max HP, up to +300. Tanks also gain speed and strength per 250 earned.

**Comeback and catch-up mechanics**
- **Bounties:**
  - Every hero starts with a bounty of 300.
  - Each elimination raises it by 75 (briefly 100 in Oct 2025), up to a cap of 2,000.
  - A bounty of 750 or more is shown on the team bar.
  - On death the bounty resets to 300, or to 200 if it was already 300.
  - Killing a high-bounty player pays out "a large cash boost", split by contribution.
- **Round MVP:** the top earner on each team gets a +1,000 bonus (1,500 for part of Oct 2025).
- **Trailing-team bonus:** teams with fewer round wins earn more cash, so an early lead doesn't become insurmountable. https://www.inverse.com/gaming/overwatch-2-stadium-mode-season-16-launch-most-important-moment-ever [S]
  - A secondary summary says a team "significantly behind" at the start of a round gets a bonus multiplier on its earnings.
  - **The exact formula was not found in primary text [U].**
  - Blizzard's Oct 16 2025 note mentions "strengthening comeback mechanics to make momentum shifts feel more impactful." [P]
- **No separate loss bonus:** the official blog describes no CS-style loss bonus. Round-completion cash is paid to both teams.

**Mercy rule**
- At first, a match ended at 3–0.
- Before launch this was changed: the match now ends early only if a team wins the first three rounds **and** has 15,000 more cash than the losing team. The reason was that the trailing team could still win the four remaining rounds. https://www.inverse.com/gaming/overwatch-2-stadium-mode-season-16-launch-most-important-moment-ever [S]
- In Oct 2025 the mercy rule was disabled on Oct 14 and re-enabled on Oct 30 [P].

**Format experiment (Oct 2025)** [P, patch notes]
- On Oct 14, Competitive Stadium moved to **best of 5** (first to 3, with Powers on rounds 1, 2, 3 and 5).
- On Oct 30 it was reverted to best of 7. Blizzard: "This is a direct result of both player feedback and internal data" and "Best of 7 provides players with more space to let their builds breathe".
- **Round-completion cash in Oct 2025:**
  - Round 1: 5k → 10k → 5k
  - Round 2: 4k → 7.5k → 4k
  - Round 3 and later: 3.5k → 5k → 3.5k
  - Damage and healing cash was raised, then lowered again.
  - Takeaway: Blizzard tunes the economy aggressively.


---

## 5. Splatoon 3 (Nintendo)

### Modes and win conditions

| Mode | Queue | Team | Win condition | Match length | Overtime |
|---|---|---|---|---|---|
| Turf War | Regular Battle (casual); also Splatfest Battle | 4v4 | "the team that covers the most area ('turf') in ink wins" [P][1] | "within three minutes" [P][2] | None. The area is judged once, at the end |
| Tricolor Turf War | Splatfest, second half only | 4 vs 2 vs 2 | "ink the most turf to win", plus a fight over the Ultra Signal [P][3] | Same as Turf War [UNVERIFIED: 3 min] | None |
| Splat Zones | Anarchy / X Battle | 4v4 | Hold the zone and your countdown clock drops. "Whichever team lowers the countdown clock the most wins" [P][1]. The clock starts at 100 and reaching 0 wins outright (knockout) [S][4] | Five minutes [S][4] | Yes. See below |
| Tower Control | Anarchy / X | 4v4 | "Whichever team gets their tower to the goal first wins" [P][1]. Otherwise the team that pushed it furthest wins [S][5] | Five minutes [S][4 by analogy; UNVERIFIED for this mode] | Yes, while the trailing team holds the tower [S][5] |
| Rainmaker | Anarchy / X | 4v4 | Carry the Rainmaker into the enemy goal [P][1]. Otherwise "the team with the lower goal distance will win" [S][6] | Five minutes [UNVERIFIED for this mode] | Yes [S][6] |
| Clam Blitz | Anarchy / X | 4v4 | "Whichever team throws the most clams into the opponents' basket wins." A Power Clam (made from 8 clams) breaks the barrier [P][1] | Five minutes [UNVERIFIED for this mode] | Yes [S][7] |

The game shows ranked progress as a count to 100: reaching 100, or 0 for Splat Zones, is a knockout and ends the match early. Nintendo's own pages describe these modes only qualitatively, so the numeric "100 count / 5 minutes" framing comes from secondary guides [S][4]. A Nintendo patch note confirms that a count exists in Rainmaker: "Adjusted Rainmaker so that if the player is splatted right at the moment they place the Rainmaker at the goal, the count will be at 1" (Ver. 1.2.0, Oct 25 2022) [P][8].

### Overtime and tiebreak rules (the anti-snowball part)

- **Overall principle.** Ranked overtime keeps the match going past the clock while the trailing team is in control of the objective. The trailing team either takes the lead or loses control.
- **Splat Zones overtime.** "If the losing team is currently in control when the match timer ends, the game will enter overtime." The game stays in overtime until the trailing team loses control or its counter drops below the other team's, and the second case is a win for them [S][4]. Game8 adds that the leading team can end overtime by retaking the zone, or by "neutralizing or capturing at least one Splat Zone for ten seconds" [S][9].
- **Splat Zones penalty count (designed comeback brake).** When a team loses the zone, it receives a penalty "increasing their timer by a varying amount". When it retakes the zone, its counter has to work through the penalty first. "The amount of penalty counter a team receives increases the lower the normal counter is" [S][4]. A team close to winning therefore pays the most for losing the zone, which slows runaway leads. No Nintendo patch note changed this rule through Ver. 9.3.0 (the history page was searched for penalty and overtime) [P][8].
- **Tower Control overtime.** "Overtime in Tower Control occurs when the losing team has control of the Tower." To win, the trailing team has to push the tower further than the leader's best distance. When the last rider gets off, the tower stops for 5 s and then slowly rolls back toward the center [S][5]. Checkpoint pauses on the route are [UNVERIFIED].
- **Rainmaker overtime.** It starts if the trailing team holds the Rainmaker at time-up. It also starts if the leader lost control "15 seconds before the end of regulation", in which case the trailing team gets 10 s to grab it. The trailing team wins by carrying it "past the winning team's score", and the leader wins if the trailing team loses control. The carrier has a 60 s timer and then explodes; the timer runs faster when retreating toward your own side. Splatoon 3 adds mid-route checkpoints [S][6].
- **Clam Blitz overtime.** It starts if the trailing team holds a Power Clam, has just dropped one that has not yet despawned, or if the leader's basket is open [S][9b]. A failed basket opening carries a penalty of "around half the progress you made" [S][7]. Normal clams score 3 and Power Clams 20 [S][7]. An opened barrier lasts 10 s and is extended by scoring [S][9b].
- **Turf War tiebreak.** No overtime. The result comes from Judd's end-of-match area count. Players can check territory mid-match on the map ("Press the X Button to open the map and see how much territory each team has covered in ink") [P][1], so the score is not hidden, but the exact percentage only appears at the end. **[UNVERIFIED]**: what happens when the area is exactly tied.

### Snowball vs comeback: designer quotes

- **Turf War lets non-fighters contribute.** Sakaguchi: "Even if you're the type of player who may not be the best in defeating your opponents, there are gameplay elements where you could contribute to your team. We'd like to have all kinds of people play using their own play style." Sato: "Because it's a team battle game, we made a ranking of who inked the most, and careful people like him usually get number 1." (Iwata Asks: Splatoon, "This Depth Has No End") [P][10]
- **Turf War comeback lever.** Nintendo: "The key to Turf War is to take back territory the opponent has inked. Not only does this expand your own turf, but it also narrows the opponents' turf, and helps turn the tide of battle in your favor." [P][1] Every repaint counts double in the margin (your area goes up and theirs goes down), so a lead built early can be reversed late.
- **Why Tricolor is asymmetric (3-team design).** Director Shintaro Sato (Famitsu, translated by Nintendo Everything): "if we simply added an extra team to the existing Turf War ruleset, we expected various problems to arise, such as one team getting ganged up on by the other two. At the same time, while developing [Splatoon] 2, we were considering whether we could implement asymmetric battles with an attacking side and a defending side. We thought perhaps we could solve both problems by mixing them together." Producer Nogami: "we ended up having to create a Tricolor Turf War map for every Splatfest." [S, translated dev quote][11]
- **Tricolor rule changes after launch.** Originally the halftime leader always defended: "Can the first-place team defend themselves in this unique 4-v-2-v-2 match type?" [P][2]. Ver. 2.0.0 (Nov 29 2022) changed it so that "all players can play on the attacking or defending team regardless of the halftime results", with a 50/50 random role. It also "Increased the victory bonus for the defending team", paying 1.5x Clout when a first-place defender beats 2nd/3rd-place attackers [P][8][3]. Nintendo let the leader defend, then removed that rule, because it tied role assignment to standings.
- **Built-in "Comeback" gear ability.** Splatoon ships a gear ability called Comeback (a boost after respawn). It is named in the patch notes ("Fixed an issue where the gear ability Comeback was also activating for respawns from the player falling off the stage") [P][8]. Its exact effect is [S/UNVERIFIED].
- **Splatfest outcomes are unpredictable even to developers.** Inoue (Ask the Developer Vol. 7) said the team watched the final Splatoon 2 Splatfest without knowing who would win [P][12, via search summary; exact wording not captured].

### Ranked vs casual, and why

- **Structure.** Regular Battle is Turf War only and pays XP; "As your level increases, you will unlock additional game modes". "Anarchy Battle is unlocked at player level 10." Anarchy has Open (solo or with friends; "every battle will raise or lower your Rank Points") and Series (solo only; you pay an entry fee and play "5 wins or 3 losses"). X Battle requires rank S+0 and uses X Power "calculated per rule set" [P][1].
- **Why Turf War is the casual mode (designer intent).** The Iwata Asks quote above (non-fighters can contribute) is the strongest primary statement. Sakaguchi also: "We also made a single-player mode called Octo Valley for players who had never played competitive games before and were hesitant to start right off competing online." [P][10] Producer Nogami, Famitsu 2016, as reported: new players concentrate on Turf War while long-time players focus on Ranked [S, reported, original not retrieved][13].
- **Rotation (2-hour schedule).** "The available rulesets change every two hours" [P][1]. The patch notes also refer to "the timing of stage changes that happen every two hours" (Ver. 3.0.0) [P][8]. The launch Direct described League Battle the same way: "players can compete every two hours" [P][2].
- **Effect on population (Nintendo's own matchmaking notes).** Nintendo repeatedly patched queue health. Ver. 1.2.0: "Improved the efficiency of matchmaking for Anarchy Battles (Open), Anarchy Battles (Series), Splatfest Battles (Open), and Tricolor Attack." Ver. 4.0.0: "Greatly changed the matchmaking mechanism in Anarchy Battle (Open) to improve matchmaking efficiency for groups of 2 players." Ver. 6.0.0: "Players with X Power of less than 2000.0 and players with X Power of 2000.0 or higher will no longer be matched together." Ver. 8.0.0/9.2.0: the top 1,000 and then top 3,000 X players start the season at S+0, because "matchmaking can take a while in some regions right after the start of a season." Ver. 1.1.1: "there will be somewhat more instances of players selecting Tricolor Battle but being placed in a Turf War battle instead" [P][8]. Rotating one ranked ruleset at a time keeps each ranked queue to one mode per two-hour window, so the ranked population is concentrated instead of split four ways. That is an inference: Nintendo has not stated it as the reason [UNVERIFIED rationale]. Series and Open showing different modes at the same time is [UNVERIFIED].

---

## 6. Valorant (Riot) and Counter-Strike 2 (Valve)

### Round structure

| | Valorant Competitive | Valorant Premier | Valorant Swiftplay | CS2 Premier / Competitive | CS2 Casual |
|---|---|---|---|---|---|
| Regulation | First to 13; 12-round halves (24 max). "the game will no longer end when a team reaches 13 wins" describes the custom-game option that turns this off [P][14] | Same | "First to 5 rounds, 4-round half" [P][15] | "maximum of 24 rounds in regulation time" = MR12, first to 13 [P][16]. Config: `mp_maxrounds 24`, `mp_match_can_clinch 1` [P][17] | `mp_maxrounds 15`, `mp_halftime 0` [P][17b] |
| Overtime | Win by 2, sides alternate each round, everyone reset to 5,000 credits. A draw vote fires after every 2 OT rounds and gets easier to pass: 6, then 3, then 1 player(s) needed to end in a draw. "Players might gain rank off a draw but will never lose rank" (Patch 1.03, Jul 2020) [P][18] | Overtime Priority goes to the first team to 12 and picks the starting side. Win by 2. "If 3 sets of Overtime have been played, play one final round of Sudden Death" [P][19] | "Sudden death overtime" [P][15] | "a 6 round overtime in case of a tie" (MR3 per side, repeated until decided) [P][16]. Survivors "no longer carry their items into overtime" (9/1/2023) [P][20]. 3 tactical timeouts in regulation and 1 in OT [P][20b]. OT start money $12,500 [UNVERIFIED] | N/A |
| Approx. length | — | — | "Match time is approx. 15 minutes" [P][15] | CS:GO MR15 averaged about 40 min, per Valve as reported [S][21] | — |

**History of Valorant overtime.** In Patch 1.02 Riot shipped sudden death and called it "intrinsically unfair. We're actively working on replacing this with a more fair solution." [P][22] Patch 1.03 replaced it with win-by-2 plus draw votes. "Unrated will still use the original Sudden Death format" [P][18]: the casual queue kept the faster, less fair resolution. Whether the draw-vote thresholds have changed since 2020 is [UNVERIFIED].

### Economy

| | Valorant | CS2 (Competitive config) |
|---|---|---|
| Start | 800 (pistol rounds 1 and 13) [S][23] | `mp_startmoney 800` [P][17] |
| Round win | 3,000 per player [S][23] | 3,250 for elimination or time; 3,500 for bomb detonation or defuse [P][17] |
| Loss bonus | 1,900, then 2,400, then 2,900 (cap). Resets on a win [S][23] | Base `cash_team_loser_bonus 1400` + `500` per consecutive loss, so 1,400 / 1,900 / 2,400 / 2,900 / 3,400 [P][17][24] |
| Loss-streak reset | Full reset on any win [S][23] | **Decrements by 1 per win (does not reset)**. Since Mar 13 2019: "Previously, a win would reset the round loss bonus. Now, a win reduces the loss counter by one." "Teams still begin the half with a loss count of 1." [P][24] |
| Plant bonus | +300 per attacker for planting [P][15][S][23] | +300 to the planter (`cash_player_bomb_planted`); +600 team bonus if planted but defused (`cash_team_planted_bomb_but_defused`) [P][17] |
| Kill | +200 [P][15] | +300 default (`cash_player_killed_enemy_default`; varies by weapon) [P][17] |
| Survivor penalty | Since Patch 1.11, attackers who lose and survive without planting, or defenders who lose and survive after detonation, get only 1,000 [P][25] | `cash_team_bonus_shorthanded 1000` exists, but its meaning is [UNVERIFIED] |
| Cap | 9,000 [UNVERIFIED] | `mp_maxmoney 16000` [P][17] |
| Overtime | Flat 5,000 every OT round, no carryover [P][18] | Reset to OT start money [UNVERIFIED amount] |
| Casual variant | Swiftplay uses "Set Credit amounts granted at the start of the round: 800 / 2400 / 4250 / 4250", plus +600 for winning the pistol round [P][15] | Casual: **flat** `cash_team_loser_bonus 2400` with `cash_team_loser_bonus_consecutive_rounds 0`, so there is no streak escalation at all; start money 1,000 [P][17b] |

### Snowball vs comeback: designer quotes

- **Valve on loss bonus (anti-snowball).** "Adjusted round loss bonuses with the goal of reducing a negative feedback loop following streaks of losses." Changing reset-on-win to minus-one-per-win means a single scrappy round win does not throw a losing team back to minimum income. (CS:GO, Mar 13 2019) [P][24]
- **Valve on MR15 to MR12.** "Over the past decade, we've shipped updates to the economy and weapon balance to trim the fat and reduce the number of uncontested rounds in a match of CS. Because of these changes, exciting competitive matches can be resolved with fewer rounds. And shorter matches mean players can play more, and more often. So with CS2, we're moving to a maximum of 24 rounds in regulation time (with a 6 round overtime in case of a tie) for Premier, Competitive, and the Majors. We expect the structure and flow of matches to evolve over time as the community adapts." ("Your Time is Now", Aug 31 2023) [P][16] Their argument: once the economy produces fewer foregone ("uncontested") rounds, the match can be shorter without losing drama.
- **Riot on economy decision-making (anti-hoarding).** Patch 1.11 context: "These changes are intended to increase richness in decision making regarding when it is worthwhile to save your gear vs. go for the round win. Additionally, this will allow teams that can secure round wins to more effectively chip away at the economy of opponents that opt to save out expensive weapons on a round loss." [P][25] This is a pro-winner lever: it punishes "save" play so the winning team can press its advantage.
- **Riot on casual vs ranked stakes.** On the surrender vote (Patch 1.03): Unrated "is meant to be lower stakes than Competitive (no rank on the line)", so 80% can surrender to "find a fairer fight sooner". Competitive keeps unanimity "because surrendering has an impact on your rank" [P][18].
- **Riot on match flow (patch statement).** Patch 1.11: "attacking teams can have a really hard time breaking entrenched defenders" (Max Grossman, Lead Character Designer) [P][25]. This is a side-balance fix, not an economy one.
- **No standalone Riot "economy philosophy" dev blog was found.** The playvalorant.com /news/dev index currently lists mostly YouTube dev updates [P][26]. Riot's economy rationale is spread across patch-note "Context" lines.

### Casual alternates and why

- **Valorant Swiftplay.** "Similar to the standard 5v5 Unrated mode, but condensed". It uses fixed per-round credits, which removes most economy snowball, plus sudden-death OT and about 15 minutes per match. It was announced as a beta for Dec 6 2022 to Jan 10 2023 and kept because "Everyone decided to just keep it on" (Patch 6.03) [P][15][27]. Riot's only stated rationale is a shorter Unrated.
- **Valorant Spike Rush and Team Deathmatch.** Both exist as standard queues; Team Deathmatch went live in Patch 7.0 (Jun 27 2023) [P][28]. No Riot design rationale was retrieved for either [UNVERIFIED].
- **CS2 Casual.** 15 rounds, no halftime side swap, flat 2,400 loss bonus, friendly fire config present [P][17b]. The flat loss bonus is a concrete example of switching off streak economics for the casual audience.

---

## 7. Pikmin 4: Dandori Battle

### Format

| Stage | Time limit [S][29] | 2nd Bonus Find added [S][30] | Sneak Bomb spawns [S][30] |
|---|---|---|---|
| Trial Run | 4:00 | 2:30 remaining | 1:30 remaining |
| Battle in a Box | 5:00 | 3:00 remaining | 2:30 remaining |
| Dandori Castle | 5:00 | 3:00 remaining | 2:30 remaining |
| Leafy Showdown | 7:00 | 4:00 remaining | 3:00 remaining |
| Hot Sandy Duel | 5:00 | 3:00 remaining | 2:30 remaining |
| Final Battle | 7:00 | 4:00 remaining | 3:00 remaining |

Golden Sniffers (30 points, the heaviest single item) appear at a fixed time and place in Dandori Castle and later stages. In Final Battle that is 2:00 remaining (wiki strategy text) [S][30].

- **Winning.** "The player with the most points when the timer runs out is declared the winner." Points equal the weight of whatever is carried to your Onion [S][29]. Nintendo's store page: "Take on—or team up with—a second player to collect the most items before time runs out." [P][31]
- **Margin-based medals.** Bronze for a win by at least 1, silver by 30 or more, gold by 60 or more, platinum by 100 or more [S][29]. The game rewards margin of victory, not just the win.
- **Tiebreak.** Not documented on Pikmin Wiki or on Nintendo pages [UNVERIFIED: what happens at an exact tie].
- **Score transparency.** No source says scores are hidden late in the match [UNVERIFIED]. The wiki does not mention any hidden-score mechanic.

### End-of-match and comeback mechanics

- **Late Bonus Find.** One x2 Bonus Find runs from the start, and a second (x3) is added partway through. "When the second Bonus Find is added or when a Bonus Find is re-rolled, if one player's score is considerably lower than the other, the lower-scoring player will get a larger multiplier than the higher-scoring player (x3 instead of x2, or x5 instead of x3)." The exact score gap is marked as an open question on the wiki [S][29]. This is a direct rubber-band.
- **Sneak Bomb (late-match swing).** It spawns "a few minutes into the battle" (at 1:30 to 3:00 remaining depending on stage, so in the last 30 to 43% of the match). It can be carried into the opponent's Onion, and "the number of points lost to a Sneak Bomb is proportional to the victim's score". The loss is capped: "the player whose Onion was detonated will not lose more than 100 points." The lost points pop out as Dandorium weights (5, 10 or 20) that **either player** can carry home [S][29]. Because the penalty scales with score, it hits the leader hardest, and the dropped points become a fresh contest.
- **Golden Sniffer.** A fixed late-match 30-point spawn [S][30].
- **Item reels (Mystery Capsules).** Capsules can spawn every 30 s, and a player can queue up to 5 items. The items include stuns, Rock Storm, a Warp that sends the opponent somewhere random, and Quick Pik (+10 Pikmin) [S][29]. **[UNVERIFIED]**: whether item odds favor the trailing player.
- **In-game designer framing (stage-select text, transcribed on the wiki).** Leafy Showdown: "Whoever gathers the most Gold Nuggets is the true Dandori master. But with Sneak Bombs, you won't know the true winner until the last second!" [S, in-game text][32] Nintendo's own description of the mode presents late uncertainty as intended.
- **Handicap instead of matchmaking.** Difficulty presets change starting Pikmin (50 / 25 / 10 / 5 / 1), free capsule spins, the Charging Horn and the dog ability per side [S][29]. This local-versus mode balances skill gaps with explicit handicaps.

---

## 8. Team Buddies (PS1, 2000): its own modes, summarised

Everything here is taken from [`team-buddies-core-loops.md`](team-buddies-core-loops.md) §2–§3, which cites the disc data [D], the press kit [P] and the 2001 FAQ [F]. Nothing was researched again.

| Mode | Win condition | Length and limit | Comeback / snowball |
|---|---|---|---|
| **Deathmatch** (skirmish) | Last team standing. Destroying a base disables that team's pad, so it can no longer build Buddies or weapons | "Battle Limit": Infinite, Short, Medium, Long or Points [D] | **Strong snowball.** Losing the base removes production entirely. The team fights on with leftover crates and scavenged weapons. The **Ghost** option lets a wiped team's last Buddy come back by touching "smoking boots" [F]; it is the only comeback lever |
| **Domination** | 1 point each time a point stays your colour for 5 s, so more points held means faster scoring [D] | Time limit (most points) or points target (first to N) [D] | Dead teams respawn, and no team can be wiped out before time runs out [D]. A team without a base "will struggle to arm itself" [D] |
| **Capture** | Carry a stunned enemy pen animal to your pen: 10 points, or 15 if your own animal is also in your pen [D] | Time or points [D] | Respawns are on. The 15-point bonus rewards defending your own animal, which is mildly pro-leader |
| **Bomb-Ball** | Throw the bomb between the enemy posts: 1 point, plus 5 if it explodes in their goal area (3 s fuse) [D] | Time or points [D] | No building. Weapons can be found on the map, so there is no economy snowball |

**Synthesis.** Team Buddies already uses three of the structures above: base destruction with an elimination finish (Deathmatch), a point-hold score race (Domination) and goal-scoring (Bomb-Ball). It has none of the brakes modern games added: no overtime, no draw rule, no bounty on the leader, and no rebuilding a base. Its pillar mode (Deathmatch with base kills) is structurally the same as Brawl Stars Siege, the mode with the worst quit rate.

---

## 9. Patterns for a crate-economy game with destructible production

### 9.1 Cross-game comparison

| Game / mode | Structure | Typical length | Overtime / tiebreak | Main comeback lever | Ranked? |
|---|---|---|---|---|---|
| Brawl Stars Gem Grab | Hold-to-win countdown | 3:30 cap | Most gems at time-out; casual allows draws | All carried gems drop on death; the 15 s countdown resets | Yes |
| Brawl Stars Heist | Base HP race | ~2:30 [U] | Higher safe HP %; otherwise a draw | 3 s safe immunity at every 25% (since 2025) | Yes |
| Brawl Stars Bounty | Kill score race | 2:00, or first to 20 | Blue star breaks ties | Bounty on a streaking player rises +1 per kill, up to 7 | Yes |
| Brawl Stars Knockout | Bo3 elimination rounds | 2–3 short rounds | Closing gas (replaced a damage tiebreak) | Round reset; 5v5 survivors get buffed | Yes |
| Brawl Stars Siege (removed) | Carry → production → base HP | 2:30 (cut from 4:30) | Damage dealt | None effective; **highest termination rate** | Removed 2021–22 |
| Splitgate Splitball | Carry to plate, first to 3 | No timer | None needed | **Steal from the plate before lock-in** | SG2 Ranked 2025 |
| Splitgate Takedown | Wipe rounds, 4 of 7 | Rounds | — | Respawn delay grows with each death; round reset | Sole ranked mode, Feb 2026 |
| Rocket League | Goal race | 5:00 plus OT | Unlimited golden goal; no draws | Kickoff reset after every goal | Yes (1s/2s/3s plus rotating extras) |
| RL Dropshot | Destroy enemy floor, then score | 5:00 plus OT | Golden goal | Unclear whether floor damage persists [U] | Alternating seasons |
| OW2 Control | Bo3 point rounds | ~10–15 min [U] | OT while contested at 99% | Fresh round on a new sub-map | Yes |
| OW2 Push | Tug-of-war | 8 min plus OT | OT while contested or not furthest | **The robot snaps back to the losing side's barricade** | Yes |
| OW2 Clash | 5-point tug-of-war | Short (too short) | Time limit | End points split into 3 segments | **Pulled from Comp in S15**, kept in QP |
| OW2 Stadium | Bo7 rounds plus cash economy | Long | Mercy rule only at 3–0 **and** a 15k cash gap | Bounties, round MVP cash, trailing-team bonus, full-refund respecs | Its own ladder |
| Splatoon Turf War | Area at the end | 3:00 | None; judged once at the end | Repainting counts double in the margin | Casual only |
| Splatoon ranked (4 modes) | Count to 100 or best at 5:00 | 5:00 plus OT | **OT while the trailing team holds the objective** | Splat Zones penalty grows as you near victory | Yes, one mode per 2 h |
| Valorant / CS2 | First to 13 rounds | 30–40+ min | Win by 2 (Val); MR3 sets (CS2) | Loss bonus; CS decrements it instead of resetting | Yes; casual flattens the economy |
| Valorant Swiftplay | First to 5 | ~15 min | Sudden death | Fixed credits each round, so no economy snowball | No |
| Dandori Battle | Score race | 4–7 min | [U] | **Bigger Bonus Find multiplier for the trailing player; Sneak Bomb theft proportional to score, spilled as loose weights** | Local only |
| Team Buddies Deathmatch | Base kill → elimination | Configurable | None | Ghost option only | n/a |

### 9.2 Patterns (synthesis)

Each pattern cites the sections that support it. The recommendations are mine.

1. **Snowball lives in whatever persists.** RL resets everything except the score, and Valorant resets position every round but carries the economy. OW Stadium carries Powers and cash. Siege carried the bolt lead into a robot. In each game the comeback tools sit on whatever persists (§3.4, §4.5, §6, §2). **For us:** a crate economy persists, and destructible production makes it compound twice: more crates give a better army, and a better army kills the rival's production. Every brake has to act on the economy, not just on the score.

2. **Do not let destroyed production be permanent within a single scoring period.** Team Buddies Deathmatch and Brawl Stars Siege both make losing production snowball outright, and Siege was the one Supercell removed for quitting. Shipped answers include Heist's immunity at every 25% (a pacing brake on the base), OW Push's snap-back (a lost position is cheap to recover), and rounds (production resets each round). **Options for us:** a base that rebuilds or respawns after N seconds; a pad that can be "knocked out" (a scoring event plus a temporary production stop) rather than destroyed; or base destruction as the round-ending event in a best-of-N, so production resets with the round.

3. **Make carried and banked value physical, droppable and stealable until it locks in.** Gem Grab, Splitball, Sneak Bomb, Rainmaker and Clam Blitz all do this (§2.1, §2.2, §7, §5). Theft scaled to the victim's score with a cap (Sneak Bomb: proportional, at most 100) is a ready-made leader brake. **For us:** stacked crates on a pad stay stealable, and theft should spill crates onto the map, not transfer them silently. That matches the earlier finding in `modern-team-buddies-likes.md` §Synthesis 3. A "lock-in" moment (a crate becomes points or units after a short timer on the pad) gives defenders a window and a reason to fight.

4. **End on a fight, not a buzzer.** Overtime that lasts while the trailing team holds the objective (Splatoon), while the objective is contested (OW), or until the next goal (RL golden goal) means the leader has to win the last exchange. **For us:** at time-up, if the trailing team is carrying a crate or standing on a contested pad, extend the match until the carry resolves.

5. **Leader brakes beat handouts.** The levers players accept are bounties on streakers (Bounty +1 per kill up to 7; Stadium bounties capped at 2,000), penalties that grow as you near victory (Splat Zones), bigger multipliers for the trailing side (Dandori Bonus Find; Stadium's trailing-team cash) and loss bonuses that decay instead of resetting (CS 2019: "reducing a negative feedback loop"). Riot did the opposite in Patch 1.11, pushing winners on by punishing saves. That is a reminder that some snowball counts as skill expression (§6). **For us:** the economy lever has to stay visible and legible, not hidden rubber-banding. Blizzard itself flagged "confusing" objectives as a reason to pull Clash.

6. **Ranked wants fewer, cleaner modes, and low concurrency wants fewer still.** Ranked pools keep the modes with deterministic win conditions and bounded length: Brawl Stars' six core modes, OW without Clash, Splatoon one mode per 2 h, Splitgate a single mode. Casual modes flatten the economy (CS2 Casual's flat 2,400 loss bonus; Swiftplay's fixed credits) and keep the faster, less fair tiebreaks (Valorant Unrated kept sudden death). **For us, at about 200 CCU across regions:** launch with **one** ranked mode, the crate-economy mode, and at most one or two casual modes in a single mixed playlist (Splitgate's forced "3 modes minimum" and Rocket League's "Big Party" rotation are the models). Treat any new mode as a casual trial first, the way OW demotes rather than deletes.

7. **Short matches and short sessions.** Brawl Stars standardises on 2–3.5 min and cut Siege from 4:30. Splatoon runs 3–5 min, RL 5 min, and Dandori 4–7 min. Valve shortened CS so "players can play more, and more often", and Dawson said "No one likes it when queue times exceed your match time." **For us:** about 5–8 minutes per match (Team Buddies' own 5–10 min timers sit at the long end). That keeps the match-to-queue ratio healthy when queues are slow.

8. **Draws and forfeits are design decisions.** Brawl Stars allows draws in casual and splits points in Ranked. Valorant added draw votes that get easier to pass, after calling sudden death "intrinsically unfair". RL bans forfeits before 90 s. Stadium's mercy rule needs both 3–0 and a 15k cash gap. **For us:** decide on draws (or golden-crate overtime) and an early-surrender rule from the start. A crate-economy game can measure "hopeless" directly, as an economy gap, the way Stadium does.

---

## Open questions

These are gaps the sources did not close. Each is marked [U] in the body.

- **Brawl Stars:** the official reason for the Sept 2023 removal and return of Heist, Bounty and Hot Zone; exact timers for Brawl Ball and Heist; the exact wording of Frank's "2025 in review" remark on Heist and Hot Zone engagement.
- **Splitgate:** the official Splitgate 2 Ranked Arena post from July 2025 (why Domination and Takedown were excluded); any designer quote on comebacks.
- **Rocket League:** the forfeit vote threshold and any change after 2018; whether Dropshot floor damage persists after a goal; any Psyonix talk (GDC) on match length or comebacks; the ranked lineup from S23.
- **Overwatch 2:** the current draw rule; the time-bank arithmetic; overtime fuse numbers; the formula for Stadium's trailing-team bonus.
- **Splatoon 3:** an exact Turf War tie; whether Series and Open Anarchy show different modes at the same time; a primary source for the Splat Zones penalty formula (Inkipedia was blocked).
- **Tactical shooters:** CS2 overtime start money (believed $12,500); Valorant draw-vote rules after 2020; Riot's reasoning for Spike Rush and Team Deathmatch.
- **Dandori Battle:** the tiebreak; the score gap that triggers the trailing-player Bonus Find multiplier; whether item odds favour the trailing player.

---

## Sources for sections 5–7

1. [P] Nintendo, "Beginner Basics for Splatoon 3: The ins and outs of playing online" (info as of Apr 1 2024): https://splatoon.nintendo.com/en/news/beginner-basics-for-splatoon-3-the-ins-and-outs-of-playing-online
2. [P] Nintendo, "Catch up on all the latest from the Splatoon 3 Direct" (2022): https://splatoon.nintendo.com/en/news/catch-up-on-all-the-latest-from-the-splatoon-3-direct
3. [P] Nintendo, "Squid Research Lab makes new discoveries about Big Run, Tricolor Turf War, and X Battles" (Dec 1 2022): https://splatoon.nintendo.com/en/news/squid-research-lab-makes-new-discoveries-about-big-run-tricolor-turf-war-and
4. [S] TheGamer, Splatoon 3 Splat Zones guide: https://www.thegamer.com/splatoon-3-splat-zones-ranked-anarchy-battle-guide-tips/
5. [S] Game8, Splatoon 3 Tower Control: https://game8.co/games/Splatoon-3/archives/388958
6. [S] Game8, Splatoon 3 Rainmaker: https://game8.co/games/Splatoon-3/archives/388874
7. [S] Nintendo Everything, "How to play Clam Blitz in Splatoon 3": https://nintendoeverything.com/how-to-play-clam-blitz-in-splatoon-3/
8. [P] Nintendo Support, Splatoon 3 Update History (Ver. 1.1.0 to 9.3.0): https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257
9. [S] Game8, Splatoon 3 Splat Zones: https://game8.co/games/Splatoon-3/archives/388301 · 9b. [S] Game8, Clam Blitz: https://game8.co/games/Splatoon-3/archives/388042
10. [P] Iwata Asks: Splatoon, "This Depth Has No End": https://iwataasks.nintendo.com/interviews/wiiu/splatoon/0/4/ (series index: https://iwataasks.nintendo.com/interviews/wiiu/splatoon/0/0/)
11. [S, translated Famitsu interview] Nintendo Everything, "Splatoon 3 developers explain how Tricolor Turf Wars came to be" (Jun 24 2023): https://nintendoeverything.com/splatoon-3-developers-explain-how-tricolor-turf-wars-came-to-be/
12. [P] Nintendo, Ask the Developer Vol. 7: Splatoon 3, Parts 1 to 4: https://www.nintendo.com/us/whatsnew/ask-the-developer-vol-7-splatoon-3-part-1/ (no direct statements on mode or win-condition design were found in Parts 1 to 4)
13. [S] Nintendo Everything report of the 2016 Famitsu Nogami interview (via search summary; original not retrieved): https://nintendoeverything.com/?p=443197
14. [P via Liquipedia mirror] VALORANT Patch Notes 1.11 ("Play Out All Rounds"): https://playvalorant.com/en-us/news/game-updates/valorant-patch-notes-1-11/ · mirror https://liquipedia.net/valorant/Patch_1.11
15. [P] VALORANT Patch Notes 5.12 (Swiftplay Beta, Dec 6 2022): https://playvalorant.com/en-us/news/game-updates/valorant-patch-notes-5-12/
16. [P] Valve, "Your Time is Now" (CS2 Limited Test, Aug 31 2023): https://store.steampowered.com/news/app/730/view/5141476355659151610 (also https://www.counter-strike.net/newsentry/3644028846252008595, linked by https://dust2.us/news/39261/valve-officially-confirms-mr12-and-overtime-in-competitive-play)
17. [P, shipped game config tracked by SteamDB] CS2 `gamemode_competitive.cfg`: https://raw.githubusercontent.com/SteamDatabase/GameTracking-CS2/master/game/csgo/cfg/gamemode_competitive.cfg · 17b. `gamemode_casual.cfg`: https://raw.githubusercontent.com/SteamDatabase/GameTracking-CS2/master/game/csgo/cfg/gamemode_casual.cfg (values as of fetch date 2026-10-18)
18. [P] VALORANT Patch Notes 1.03 (Jul 10 2020): https://playvalorant.com/en-us/news/game-updates/valorant-patch-notes-1-03/
19. [P] Riot, Premier Launch FAQ: https://playvalorant.com/en-us/news/game-updates/premier-launch-faq/
20. [P] Valve, CS2 Release Notes 9/1/2023: https://store.steampowered.com/news/app/730/view/5141476355663002229 · 20b. Release Notes 9/6/2023: https://store.steampowered.com/news/app/730/view/5141476355680929074
21. [S] csdb.gg, "What is MR12" (the ~13% shorter figure and the "~40 min average" attributed to Valve): https://csdb.gg/blog/insights/what-is-mr12 ; also https://game-tournaments.com/en/csgo/news/50505
22. [P] VALORANT Patch Notes 1.02 (Jun 23 2020): https://playvalorant.com/en-us/news/game-updates/valorant-patch-notes-1-02/
23. [S] ONE Esports, Valorant economy guide (Jun 2023): https://www.oneesports.gg/valorant/valorant-economy-guide ; corroborated by https://wecoach.gg/blog/article/a-detailed-guide-for-mastering-the-valorant-economy
24. [P via Liquipedia mirror] CS:GO patch, Mar 13 2019 (economy / loss-bonus change): https://blog.counter-strike.net/index.php/2019/03/23488/ · mirror https://liquipedia.net/counterstrike/2019-03-13_Patch
25. [P via Liquipedia mirror] VALORANT Patch 1.11 (Nov 2 2020), economy ruleset and Grossman note: https://liquipedia.net/valorant/Patch_1.11
26. [P] playvalorant.com dev news index: https://playvalorant.com/en-us/news/dev/
27. [P via Liquipedia mirror] VALORANT Patch Notes 6.03: https://playvalorant.com/en-us/news/game-updates/valorant-patch-notes-6-03/
28. [P via Liquipedia mirror] VALORANT Patch 7.0 (Team Deathmatch): https://liquipedia.net/valorant/Patch_7.0
29. [S] Pikmin Wiki, Dandori Battle: https://www.pikminwiki.com/Dandori_Battle
30. [S] Pikmin Wiki stage pages: https://www.pikminwiki.com/Trial_Run, https://www.pikminwiki.com/Battle_in_a_Box, https://www.pikminwiki.com/Dandori_Castle, https://pikminwiki.com/Leafy_Showdown, https://pikminwiki.com/Hot_Sandy_Duel, https://www.pikminwiki.com/Final_Battle
31. [P] Nintendo, Pikmin 4 store page: https://www.nintendo.com/us/store/products/pikmin-4-switch/
32. [S, transcribed in-game text] Pikmin Wiki, Leafy Showdown: https://pikminwiki.com/Leafy_Showdown
