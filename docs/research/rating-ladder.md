# Rating algorithms and ranked ladder design

Research notes for the ranked mode of a small online team arena game: native PC, online only, crate economy, and roughly 200 CCU across regions. Compiled 2026-10-18 for issue #4. It feeds #11 (prototype: choose the rating system) and #12 (prototype: ladder presentation, placements, decay and seasons).

## How to read this

| Tag | Meaning |
|---|---|
| **[P]** | Primary: a paper, the algorithm author's own document, the developer's official blog, patch notes, or a support page |
| **[P·s]** | Primary page that blocks direct fetches (rocketleague.com, support.rocketleague.com). I read its text through a search-engine excerpt, not the page itself. Recheck it before quoting in public |
| **[W]** | Third-party guide or wiki. Used only where no primary source turned up, and flagged |
| **Synthesis** | My own reasoning. Not a source's claim |

Two limits on this research:

- **Dates.** Most ladder rules change every season. Each claim gives the date of the source it comes from, and newer patches may have changed the rule.
- **Search budget.** The session's web-search budget ran out near the end. Remaining gaps are listed in [Open questions](#open-questions).

---

## Part 1: Rating algorithms

### Summary table

| | Elo | Glicko-2 | TrueSkill | TrueSkill 2 | OpenSkill (Weng-Lin) |
|---|---|---|---|---|---|
| State per player | rating | rating, RD, volatility | μ, σ | μ, σ, plus tuned per-mode offsets | μ, σ |
| Uncertainty | none | yes (RD) | yes (σ) | yes | yes |
| Native teams | no | no (pairwise; see below) | yes, team = sum of members | yes | yes, plus multi-team and free-for-all |
| Parties | no | no | no (assumes sum) | **yes: squad offset** | no (assumes sum) |
| Quits, individual stats, experience | no | no | no | **yes** | no |
| Implementation | trivial | small, but batched rating periods | factor graph and message passing | needs offline parameter fitting on your own match data | about 100 lines of analytic updates |
| Rust crate | `skillratings` | `skillratings` | `skillratings` | none found | `skillratings` |
| Legal | free | free | **patented by Microsoft, trademarked name** | Microsoft research | MIT / open |

### Elo

Elo needs no source here: it is one rating and a fixed K-factor, with no uncertainty term. Its main gap for this game, the lack of an uncertainty term, is exactly what Glicko and the Bayesian systems below add. **Synthesis:** without uncertainty, new players move as slowly as veterans, which is the worst case for a small population where every bad early match is a large share of someone's experience.

### Glicko-2

- Each player has a rating r, a rating deviation RD and a volatility σ. A 95% interval is roughly r ± 2·RD ([P] [Glickman, *Example of the Glicko-2 system*, 2022](https://www.glicko.net/glicko/glicko2.pdf)).
- Unrated players start at r = 1500, RD = 350 and σ = 0.06; "this value depends on the particular application" ([P] same).
- The system constant τ should be "between 0.3 and 1.2", tuned on real data for predictive accuracy ([P] same).
- **The batching requirement.** Games are grouped into a "rating period" and treated as simultaneous. Glicko-2 "works best when the number of games in a rating period is moderate to large, say an average of at least 10-15 games per player in a rating period" ([P] same).
- The update is written for one player against m opponents with scores of 0, 0.5 or 1 ([P] same). There is no team model in the paper.

**Synthesis.** Glicko-2 is designed for chess-like pairwise play with batched periods. Using it for 3v3 or 4v4 means inventing a team composite (an average, or rating each player against each opponent), which is exactly the job TrueSkill and Weng-Lin do natively. The batching advice fits poorly with per-match rank updates, which every modern ladder below now shows.

### TrueSkill (2007)

- A team's skill "is assumed to be the sum of the skills of the players". Updates use only which team won, not scores ([P] [Microsoft Research, TrueSkill project page](https://www.microsoft.com/en-us/research/project/trueskill-ranking-system/)).
- New players start at μ = 25, σ = 8.333. The leaderboard value is the conservative estimate μ − 3σ ([P] same).
- **Minimum games per player to identify skill** ([P] same, which says the real number can be up to 3× higher):

| Mode | Games |
|---|---|
| 8-player free-for-all | 3 |
| 4-player free-for-all | 5 |
| 2-player (1v1) | 12 |
| 4 teams × 2 | 10 |
| 4 teams × 4 | 20 |
| **2 teams × 4** | **46** |
| 2 teams × 8 | 91 |

- **Patent.** US 8,538,910 B2, "Determining relative skills of players", is assigned to Microsoft Corporation, inventors Graepel, Herbrich and Minka, and Google Patents lists it as active ([P] [Google Patents](https://patents.google.com/patent/US8538910B2/en)). I did not read the claims, so I cannot say exactly what it covers.

**Synthesis.** The table is the single most important number in this file for a team game. Two-team play carries far less information per match than free-for-all, because one bit (win or loss) is shared across all teammates. At 4v4, TrueSkill needs about 46 games, possibly up to about 140, before a player's rating is reliable. That is the convergence problem every ladder below works around with placements, convergence multipliers and smurf detection.

### TrueSkill 2 (2018)

Source for this whole section: [P] Minka, Cleven and Zaykov, [*TrueSkill 2: An improved Bayesian skill rating system*](https://www.microsoft.com/en-us/research/uploads/prod/2018/03/trueskill2.pdf), Microsoft Research, 2018, built on Halo 5 and Gears of War 4 data.

- **Accuracy.** "TrueSkill2 predicts historical match outcomes with 68% accuracy, compared to 52% accuracy for TrueSkill" on Halo 5.
- **What it adds:**
  1. Skill is inferred from individual statistics such as kills and deaths, as well as from win or loss.
  2. A player who quits is "treated as a surrender", and is updated as a loss regardless of the result.
  3. Skill in one mode is correlated with skill in other modes, so a new mode borrows the player's existing rating.
  4. Skill drifts upward with experience: a small per-match increment, largest in the first matches, capped at 200 matches.
  5. Squad members are assumed to perform better than their individual skill.
- **The squad finding.** With the usual "squad = sum of its players" assumption, solo players won 47% and squads of 4 won 57% against a predicted 52%; squads of 8 won 68% against a predicted 49%. "The skill rating of a squad should be larger than the sum of its players, and the effect grows with the size of the squad." The fix is a tuned `squadOffset` per squad size per mode, used in matchmaking and in the post-match update.
- The paper names the alternatives it rejects: Halo 5 and Gears 4 playlists that "prohibit large squads", and games that force squads to meet squads, which "has the problem of shrinking the matchmaking pool".
- **New players.** Players with 0–9 matches won about 45–47% while TrueSkill expected 48–49%, which is the motivation for the experience offset.
- **Adoption.** Riot reports testing TrueSkill 2 in League of Legends' unranked queues: high-skill players reached "correctly skilled games in around 5 games … about 3 times faster than our current system" ([P] [Riot, /dev: Banning Bots, Boosters & More in 2025, 2025-08-11](https://leagueoflegends.com/en-us/news/dev/dev-banning-bots-boosters-and-more-in-2025)).

**Synthesis.** TrueSkill 2 is a recipe more than a library. Its offsets are fitted to a large match history the game will not have at launch. The ideas still transfer cheaply to any Gaussian rating system: a party-size offset, quit-as-loss, an experience drift, and borrowing a rating across modes.

### OpenSkill / Weng-Lin (2011)

- Weng and Lin derive "simple analytic update rules" that need no iterative message passing. Accuracy is "competitive with state of the art systems such as TrueSkill" while "the running time as well as the code is much shorter" ([P] [Weng & Lin, JMLR 12, 2011](https://www.jmlr.org/papers/v12/weng11a.html)).
- Like TrueSkill, they assume "a team's skill is the sum of its members' skills" ([P] [paper PDF](https://www.jmlr.org/papers/volume12/weng11a/weng11a.pdf), §4).
- **Numbers from the paper:** on the Halo 2 "Free for All" data, TrueSkill took 13 s and their Bradley-Terry full-pair model 1.2 s. Their F# code was under 100 lines against more than 500 for TrueSkill. The Bradley-Terry and Plackett-Luce models were "competitive with TrueSkill" on prediction error, while their Thurstone-Mosteller model, which uses TrueSkill's likelihood, was "consistently worse" ([P] same, §5).
- **OpenSkill** is the maintained implementation. It supports "multiple teams, factions" and asymmetric team sizes, offering Plackett-Luce, Bradley-Terry and Thurstone-Mosteller models ([P] [openskill.py docs](https://openskill.me/en/stable/)). Its README claims it is "150% faster than TrueSkill" with "accuracy [that] matches TrueSkill", and it is MIT-licensed ([P] [openskill.py repo](https://github.com/OpenDebates/openskill.py)). These are the maintainers' own claims, with no benchmark shown on the page.

### Rust implementation

- The `skillratings` crate (0.29.2, MIT OR Apache-2.0) implements Elo, Glicko, Glicko-2, TrueSkill and Weng-Lin, among others. "Some algorithms like TrueSkill or Weng-Lin allow you to rate team-based games as well"; Weng-Lin also handles multi-team and free-for-all ([P] [docs.rs/skillratings](https://docs.rs/skillratings/latest/skillratings/)).
- It does not list TrueSkill 2 ([P] same).

**Synthesis: recommendation for #11.** Use **Weng-Lin (Plackett-Luce or Bradley-Terry) through `skillratings`**. It is native to team games, has uncertainty, is MIT/Apache, is cheap to run, and avoids the TrueSkill patent and trademark question. Add the TrueSkill 2 ideas as thin layers on top, each one cheap: a per-party-size offset, quit counted as a loss, and per-mode ratings seeded from the player's other modes. Keep the option to fit those offsets from real data once there is a match history. Do not use Glicko-2 for a team game that updates per match. Elo has no uncertainty term at all.

---

## Part 2: Real ladders

### StarCraft II

- **Leagues and divisions.** Seven ranked leagues, Bronze to Grandmaster. Five placement matches set the starting league. Each league is split into divisions of up to 100 players. Placement uses only wins, losses and the skill of the participants, "not in-match stats like APM" ([P] [Blizzard, Leagues and Ladders FAQ](https://news.blizzard.com/en-us/article/110519/leagues-and-ladders-faq), undated).
- **Points and the bonus pool.** Visible points go up for wins and down for losses, scaled by opponent strength. Every player accrues a capped Bonus Pool "whether online or not", which pays out on wins to help players "catch up after time away" ([P] same).
- **Patch 3.4 revamp.** MMR is now displayed, for "increased accuracy and increased transparency". Bronze to Master are split into three tiers each, and league sizes were adjusted so Silver to Diamond are equal. Grandmaster is based purely on MMR, needs at least 10 games every 3 weeks, and demotes the bottom 5% daily ([P] [Blizzard, Patch 3.4: The StarCraft II Ladder Revamp](https://news.blizzard.com/en-us/starcraft2/20166423)).
- **What it rewards:** skill (MMR) for league placement, plus activity through the Bonus Pool and Grandmaster's game-count requirement.

### Rocket League

- **Placements and resets.** Ten placement games per ranked playlist were part of the original system ([P·s] [Patch Notes v1.11](https://www.rocketleague.com/news/patch-notes-v1-11?lang=en)). Every recent season starts with "a soft rank reset … applied to all Ranked Playlists" ([P·s] [v2.66, Season 22](https://www.rocketleague.com/news/rocket-league-patch-notes-v2-66), [v2.63, Season 21](https://www.rocketleague.com/news/rocket-league-patch-notes-v2-63-season-21-live), [v2.56](https://www.rocketleague.com/news/rocket-league-patch-notes-v2-56-season-20-live)). In v2.63 Psyonix also changed starting ranks for Quads and Heatseeker, because placement in Quads gave higher-skilled players lower-than-expected ranks ([P·s] v2.63).
- **MMR display.** Since v2.66 players can opt in to seeing their MMR on Competitive playlist tiles and the end-of-match scoreboard. It is off by default and does not affect Casual ([P·s] [v2.66](https://www.rocketleague.com/news/rocket-league-patch-notes-v2-66)).
- **Playlist pruning.** Season 22 removed 4v4 (Quads) from Ranked ([P·s] v2.66).
- **Parties.** Party skill is "based on the highest-skilled player in the group" rather than an average. It is used both for matchmaking and after the match, which stops players ranking up by teaming with lower-ranked friends. Friends can queue together "regardless of their current skill tier" ([P·s] [Party Matchmaking Update](https://www.rocketleague.com/en/news/party-matchmaking-update)). One exception: from Season 4, a 2-player party in 3v3 must be within 3 ranks of each other, so that the solo third player joins a team near their own rank ([P·s] [support: rank restrictions for parties](https://support.rocketleague.com/hc/en-us/articles/4405011171483-What-Are-Rank-Restrictions-for-Parties-in-Rocket-League-)).
- **Boosting.** The same post set an enforcement threshold for players at Rising Star and above who play more than 25% of matches, and more than 30 matches, with a teammate more than eight tiers below them ([P·s] Party Matchmaking Update).
- **Structure.** Bronze to Grand Champion with tiers I–III and four divisions each, then Supersonic Legend; ten placements per playlist; season rewards based on peak rank. These details are [W] only ([Shacknews](https://www.shacknews.com/article/124436/all-rocket-league-ranks-and-divisions-explained), [esportstalk](https://www.esportstalk.com/blog/rocket-league-ranks-ranking-up-guide/)).

### Overwatch 2

- **Season 9 rework (February 2024).** Placements returned as 10 matches, with a predicted rank shown after each, and every player's Skill Rank was reset. Players get "one chance this year" to run placements, so placements happen yearly, not every season. Rank updates "after every match" again, with a bar showing progress between Skill Divisions and modifiers listed beneath it (for example, a boost for beating a favoured team). A new top rank, Champion, was added. Competitive Points for cosmetics are awarded for playing, with more for wins ([P] [Blizzard, Revitalizing the Overwatch 2 Experience](https://overwatch.blizzard.com/en-us/news/24056255/revitalizing-the-overwatch-2-experience/)).
- The reset was "a soft reset of everyone's rank", and placement results "substantially" influence the starting rank ([P] [Blizzard, Rise as a Champion in Season 9](https://overwatch.blizzard.com/en-us/news/24064844/rise-as-a-champion-in-overwatch-2-season-9/)).
- **Groups.** Season 10 removed grouping restrictions: "We'd like people to be able to play with their friends in every area of the game." Instead, "groups with a large range of player skill — what we call wide groups — will only play against other wide groups," and narrow groups only against narrow groups ([P] [Aaron Keller, Director's Take, 2024](https://overwatch.blizzard.com/en-us/news/24069288/director-s-take-looking-ahead-to-improvements-and-experiments/)). The old rule was two ranks apart, or one at Master and above [W] ([Fragster](https://www.fragster.com/overwatch-2-wide-match-what-is-it-and-whats-going-on/)).
- **Population signal.** After the Season 9 rework, "Competitive Play now makes up about 45% of match hours, with Quick Play around 32%" ([P] Director's Take).
- **What it rewards:** wins and MMR for rank, and participation for the cosmetic currency, which is weighted toward wins.

### Valorant

- **Hidden MMR, visible RR.** "You will never see your MMR." Rank and RR are kept "slightly decoupled" from MMR so the visible rank does not swing wildly. New or reset players are placed "on the low end" of the estimate, and then **convergence** gives more RR per win than per loss until rank reaches MMR. The leaderboard follows MMR, not win count ([P] [Ask VALORANT – Rank Rating Edition, 2021-03-18](https://playvalorant.com/en-us/news/dev/ask-valorant-rank-rating-edition/)). Patch 2.02 sped up convergence because it "wasn't pushing you fast enough" ([P] [Competitive Changes: 2.02](https://playvalorant.com/en-gb/news/game-updates/competitive-changes-2-02-and-what-s-next/)).
- **Party policy as an anti-smurf tool.** "In an effort to reduce smurfing, we're removing all ranked restrictions from 5-stack parties." In the same patch, 4-stacks were removed from Competitive because they gave the solo fifth player a poor experience. To compensate, wide 5-stacks get their RR changes cut by 50–90% depending on the rank spread ([P] [Patch 3.10](https://playvalorant.com/en-us/news/game-updates/valorant-patch-notes-3-10/)).
- **Episode 5 (2022).** Added the Ascendant rank. The seasonal reset meant players "may not see a rank increase at the start of the Episode without putting in some work". The highest possible placement rose to Ascendant 1. "MMR targets determine your RR gains and help push your rank to match your MMR" ([P] [Patch 5.0](https://playvalorant.com/en-gb/news/game-updates/valorant-patch-notes-5-0/)).
- **Smurf detection.** Machine learning flags likely alternate accounts and moves their MMR toward the main account's level. Before the change, smurfs "stomped" (won by 8+ rounds) 32% of the time. Afterwards, adjusted smurfs were within 1% of the target stomp rate and reached the right MMR "in as few as 4 matches", while the control group was still about 3 divisions off after 10. Smurf numbers fell about 17% ([P] [VALORANT Systems Health Series – Smurf Detection, 2022-12-20](https://playvalorant.com/en-us/news/dev/valorant-systems-health-series-smurf-detection/)).

### League of Legends

- **Hidden MMR.** MMR "has been custom-tailored to work with the backend systems that help place you into fair games", so Riot shows rank instead. LP is the bridge: if MMR is above rank, a player gains more LP than they lose, and the reverse if it is below. MMR "remains at exactly the same spot" over a season reset while rank resets lower. MMR is separate per queue. Queuing with friends can mean "slightly more difficult opponents or reduced LP gains" ([P] [Riot Support, MMR, Rank, and LP](https://support.riotgames.com/league-of-legends/gameplay/mmr-rank-and-lp)).
- **Placements.** Five placement matches per ranked queue. A placement loss gives 0 LP rather than losing LP. Placement is capped at Diamond III ([P] [Riot Support, Placements, Promotions, Demotions and Decay](https://support.riotgames.com/en-us/league-of-legends/gameplay/placements-promotions-series-demotions-and-decay/)).
- **Divisions.** Each division promotes at 100 LP with carry-over. A drop between tiers lands the player at 25, 50 or 75 LP, depending on how far their skill is estimated to be from the tier ([P] same).
- **Decay applies only from Diamond up.** In Diamond, each game banks 7 days (28 maximum) and each decay costs 50 LP. In Master and above, each game banks 1 day (14 maximum) and each decay costs 75 LP. Decaying apex players fall to Diamond II ([P] same).
- **New accounts.** Riot is testing TrueSkill 2 to place accounts faster (see [TrueSkill 2](#trueskill-2-2018)), and says it is "working on how to better place these accounts' skill levels accurately" ([P] [Riot, 2025-08-11](https://leagueoflegends.com/en-us/news/dev/dev-banning-bots-boosters-and-more-in-2025)).

### Brawl Stars Ranked

Source for this section: [P] [Supercell, *Ranked is dead and Ranked is back*, 2025-02-13](https://supercell.com/en/games/brawlstars/blog/news/ranked-is-dead-and-ranked-is-back/).

- **History.** Ranked replaced Power League and had about 10× its players, but Supercell says it became too casual for players seeking a challenge. The rework adds a Pro rank above Masters. Masters is split into three tiers of 1,000 Elo each, and Legendary tiers are 750 Elo each.
- **Fighting Elo inflation.** Floor protection was removed from Mythic and above. Elo losses are harsher from Legendary up. "Rank Boost" catch-up was cut sharply and applies only up to Mythic. Duo and Trio team-ups become more restricted as rank rises.
- **Resets.** Players are no longer reset to Bronze I each month. Diamond 1 and above drop six minor ranks; Gold 3 and below reset to Silver I. Seasons start on the third Thursday of the month; the first new season ran 25 February to 16 April.
- **Matchmaking.** Faster at low ranks and stricter at high ranks: "searching no longer widens the Elo range beyond your rank restriction."
- **Entry gate.** Mythic and above require 12 Brawlers at Power Level 11. This is a progression gate on the ladder.
- **What it rewards.** A four-month Pro Pass (free and paid tracks, $24.99) whose XP comes from ranked wins (refreshed weekly), rank-ups and watching esports. It is tied to the esports circuit.

### Splatoon 3 X Battle

All from [P] [Nintendo, Splatoon 3 Update History](https://en-americas-support.nintendo.com/app/answers/detail/a_id/61257/~/splatoon-3-update-history).

- **Ver. 2.0.0 (2022-11).** X Battle is open from rank S+0 in Anarchy Battle. **X Power is tracked separately per mode** (Splat Zones, Tower Control and so on). Players play sets until three wins or three losses, then X Power changes. The world is split into two divisions for rankings: Takoroka (Japan and the Asia-Pacific) and Tentatek (the Americas and Europe). The choice is locked for the season. Separately, Anarchy Battle gained a once-per-season rank reset, available from rank B, which sets rank "from the player's estimated ability".
- **Ver. 4.0.0 (2023-05).** Added Anarchy Battle (Open), so friends can play ranked together at any time. When playing with friends, "Anarchy Power will be calculated for your team as a whole." Solo, duo and team divisions exist for Challenges.
- **Ver. 6.0.0 (2023-11).** Players under 2000 X Power and players at 2000 or above "will no longer be matched together". The minimum X Power change per set was raised and now depends on pre-set X Power and the set's win/loss record. Badges were added for X Power thresholds, for win streaks at 2000+, and for end-of-season X Rankings.
- **Ver. 7.2.0 to 9.2.0 (2024).** To "alleviate a problem with matchmaking being difficult immediately after the season switch", players who finished in the previous season's top 1,000 (later the top 3,000) start the new season at S+0 and can enter X Battle at once. Ver. 9.2.0 says the reason is that "matchmaking can take a while in some regions right after the start of a season". Ver. 9.0.0 made rematches against the same players less likely.
- **Seasons** are three months, starting in March, June, September and December (from the season data dates in the same update notes).
- **What it rewards:** X Power per mode and season-end ranking badges.

---

## Part 3: Cross-cutting comparison

| Topic | What the ladders do |
|---|---|
| **Hidden MMR vs. visible rank** | Every ladder separates them. LoL and Valorant hide MMR deliberately. SC2 (patch 3.4) shows it, and Rocket League (v2.66) shows it as an opt-in. OW2 shows rank plus the modifiers behind each change |
| **Bridge from rank to MMR** | Convergence: more points per win than per loss until rank reaches MMR (Valorant, LoL) |
| **Placements** | 5 (SC2, LoL), 10 (OW2, Rocket League). Placements are capped below the top (LoL at Diamond III, Valorant at Ascendant 1). Placing low and climbing quickly is deliberate (Valorant) |
| **Tiers and divisions** | 6–9 named tiers with 3–4 divisions each, and an apex tier that is a pure MMR leaderboard (SC2 Grandmaster, LoL Master+, Splatoon X Rankings) |
| **Decay** | Only at the top, and based on activity: SC2 Grandmaster (10 games every 3 weeks), LoL Diamond+ (banked days). None for ordinary players |
| **Soft resets** | Universal. MMR is kept and only the visible rank drops (LoL, OW2, Rocket League, Brawl Stars, Valorant). Splatoon seeds its top players straight back in to keep the high-end queue alive |
| **Season length** | Monthly to about 7 weeks (Brawl Stars), 3 months (Splatoon), a year for OW2 placements |
| **Parties** | Three approaches: (a) restrict party spread (Rocket League 3v3 duos, Brawl Stars at high rank, OW2 before Season 10); (b) allow wide parties but separate or penalise them (OW2 wide pool, Valorant 5-stack RR cut, Rocket League rates a party as its best player); (c) model the party advantage (TrueSkill 2 squad offset) |
| **Smurfs and new accounts** | Faster placement through better priors (TrueSkill 2 in LoL, about 5 games), ML smurf detection (Valorant, about 4 games), and removing the friend-queue motive for smurfing (Valorant 3.10) |
| **What it rewards** | Skill determines rank everywhere. Participation and wins earn cosmetics (OW2 points, Brawl Stars Pro Pass). Peak and season-end placement earn prestige (Splatoon badges, Rocket League season rewards [W]) |

## Implications for this game (Synthesis)

These are my own conclusions, aimed at #11 and #12, for a game with about 200 CCU.

1. **Rating system: Weng-Lin through `skillratings`**, with uncertainty. Add three TrueSkill 2 layers: a party-size offset, quit counted as a loss, and per-mode ratings seeded from the player's other modes. Avoid TrueSkill (patent) and Glicko-2 (no team model, batching). Do not rate from kills or crate counts until the match data shows they predict wins (TrueSkill 2's method). Rating from them early would reward farming stats over winning, and here crates are the core objective.
2. **Convergence is the real problem at low CCU.** TrueSkill needs about 46 games at 4v4. A small population cannot absorb that many mismatched games per new player. Use a strong prior (for example, from bot or tutorial results and a short placement run), high initial σ, and a Valorant-style convergence multiplier. Consider 3v3 over 4v4 partly for this reason: fewer teammates share each win, so each match tells you more.
3. **One ranked queue, very few modes.** Splatoon tracks X Power per mode and still had to patch season-start matchmaking delays "in some regions". Rocket League dropped Quads from Ranked. At 200 CCU, split ranked by nothing except region, if even that, and keep one rating across modes.
4. **Parties: allow them, and model them; do not restrict or segregate.** Separate pools (OW2's wide/narrow, Valorant's 5-stack-vs-5-stack) and party restrictions (Rocket League 3v3) all shrink the pool, which TrueSkill 2 names as the cost. Rocket League's "party = its best player" rule and a squad offset are pool-neutral. Valorant shows that blocking friend queues creates smurfs.
5. **Ladder shape: hidden MMR, visible tiers, soft reset, decay only at the top.** Few tiers with coarse divisions, an apex leaderboard driven by MMR, soft resets of the visible rank only, and seasons long enough that the ladder repopulates (about 3 months). Seed the previous season's top players back in so the high-end queue is not empty at season start (Splatoon 9.2.0). At 200 CCU, a "top 500" tier is most of the active population, so define the apex tier by MMR threshold, not by rank count.
6. **Rewards: rank from winning, cosmetics from participation.** Follow OW2 and Brawl Stars: rank tracks MMR, and a separate currency or track rewards playing, with more for wins. This keeps a small population queuing without inflating rank (Brawl Stars had to undo inflation from floor protection and Rank Boost).

## Open questions

- **Rocket League** pages could not be fetched directly (HTTP 403). Its claims rest on search excerpts of rocketleague.com pages ([P·s]) and guides ([W]). Psyonix's "Competitive Rank Recalibration" and "Changes to Rocket League's Competitive Rank System" posts were found but not read.
- **TrueSkill patent scope:** the claims were not read. Get legal advice before shipping TrueSkill itself.
- **Squad offset sizes, LoL season and split lengths, Valorant's current act and episode cadence, and OW2's current decay rules** were not confirmed from primary sources in this pass.
- **No published numbers** on how much rank confidence a small-population game can reach. This is a candidate for simulation in #11: simulate a few hundred players with Weng-Lin under 3v3 and 4v4 and measure games to convergence.
