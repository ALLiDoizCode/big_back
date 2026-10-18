# Matchmaking at low player concurrency

Research notes for ticket #5. The question: how do games with small populations (hundreds of concurrent players, spread across regions) keep matchmaking healthy? Compiled 2026-10-18. The target is a competitive team arena game, native PC, online-only, that has to stay healthy at about **200 CCU across regions**.

## How to read this

The source tags follow [`modern-team-buddies-likes.md`](modern-team-buddies-likes.md):

| Tag | Meaning |
|---|---|
| **[P]** | Primary: the developer speaking for itself (Steam announcements and patch notes, official blogs, support pages, developer replies on forums, official docs) |
| **[I]** | A named developer quoted in press |
| **[R]** | Press reporting or summary. Use for facts with no primary source |
| **[W]** | Wiki, fan guide, forum or Reddit. Leads only |
| **Synthesis** | My own reasoning or arithmetic, not a source's claim |

Steam patch notes were read in full through Steam's news API (`ISteamNews/GetNewsForApp`). Links use `store.steampowered.com/news/app/<app>/view/<post>`.

**Gaps.** Three sources could not be read. Velan's Knockout City matchmaking blog ("Making Magical Matches") now redirects to the game's homepage. The 1047 Games 2022 matchmaker blog returns 404. The Véron et al. NOSSDAV 2014 paper (League of Legends queue traces) refused the connection. Spellbreak's announcements say almost nothing about matchmaking. I found **no primary source for Brawl Stars bots**, and no official statement on Marvel Rivals bots either. The search budget for this session ran out before I could chase those further.

---

## 1. The core arithmetic: why small games break

The most useful primary source here is the developer of a mid-size game who published the numbers: Joost van Dongen of Ronimo (Awesomenauts, 3v3).

- **The filters multiply.** Awesomenauts "often" had "well over 1,000" CCU. With ~20-minute matches, that gives about **50 players per minute** entering matchmaking. Joost then applies a filter for each requirement: a good connection (÷3), close skill (÷3), ranked/unranked split (÷3), premade/solo split (÷4), and tighter skill (÷3). That leaves **0.15 suitable players per minute**: "With 1,000 concurrent players it will take 30 minutes to fill a match!" He estimates "perfect" matchmaking would need about **19,000 CCU** for a 2-minute wait, or about **58,000** at peak once off-peak hours (about 3× lower) are allowed for ([P] [Joost, 2014-11-22](https://joostdevblog.blogspot.com/2014/11/why-good-matchmaking-requires-enormous.html)).
- **Small games.** "It is pretty decent to have 40 simultaneous players a few weeks after launch" for an indie game. At 40 CCU, "a player starts looking for a match once every 45 seconds." With three modes, each mode gets one searcher every 135 seconds, so "waiting for a complete match of four players then takes almost 7 minutes." He adds that "removing two game modes would have reduced the waiting time to 2:15 minutes" ([P] [Joost, 2015-09-06](https://joostdevblog.blogspot.com/2015/09/designing-matchmaking-for-smaller.html)).
- **What counts as a small game.** "1000 concurrent players isn't much from a matchmaking perspective." He gives a triple-A game, LawBreakers, as having "only a few hundred" simultaneous players ([P] [Joost, 2017-09-03](https://joostdevblog.blogspot.com/2017/09/the-requirements-of-good-matchmaking.html), via the [matchmaking label index](https://joostdevblog.blogspot.com/search/label/matchmaking)).
- **A real game near our target.** Lethal League Blaze developer YinYin (2020): "It peaks at 100 concurrent players every day around NA/EU midnight … more in ranked than free for all … The community is pretty global, so you can end up with Japan, Europe, Australia, etc" ([P] [Steam thread](https://steamcommunity.com/app/553310/discussions/0/2150972948824214686)).
- **The trade-off is acknowledged at large scale too.** Blizzard says it "often comes down to a hard tradeoff between making worse matches or dramatically increasing queue times" ([P] [Overwatch 2 dev blog, part 2](https://overwatch.blizzard.com/news/23910161)). 343 says more playlists mean "overall match quality declines as the player population is spread across more options". The results are "unhealthy" playlists, "degraded online performance due to smaller populations around specific data centers", and "larger than desired variances in player skill levels" ([P] [Halo Infinite September playlist update, 2022](https://www.halowaypoint.com/news/halo-infinite-september-playlist-update)).

### Synthesis: a rule of thumb for players per queue

From Joost's arithmetic (it matches his 40-CCU, 7-minute example):

> **expected fill time T ≈ (m − 1) / (r · f)**, where **r = N / L**.

- *N* is the CCU cycling through one queue.
- *L* is the minutes per loop: match plus lobby plus menus.
- *m* is the players per match.
- *f* is the fraction of the pool the matchmaker will accept. That is the skill band × the ping band, and it widens over time (§3).

Solving for *N* at a target fill time of **T = 90 s**, with **L = 10 min**:

| Match size *m* | Accept anyone (*f* = 1) | Loose skill band (*f* = ⅓) | Tight band (*f* = ⅙) |
|---|---|---|---|
| 4 (2v2) | ~20 CCU | ~60 | ~120 |
| 6 (3v3) | ~33 | ~100 | ~200 |
| 8 (4v4 or 4×2) | ~47 | ~140 | ~280 |

**Off-peak CCU is about ⅓ of peak** ([P] Joost 2014). So "200 CCU across regions" is roughly **70 CCU globally at the trough**, and only a few dozen in any one region. **Synthesis:** at that size the game can sustain **one matchmade queue, pooled globally with ping-aware scoring**. With a 3v3 or smaller format, a second queue might be sustainable at peak hours only. Splitting by region, by ranked and casual, or by solo and party each costs roughly ×2–4 in fill time. The numbers are rough, but they show which levers matter: **number of queues, match size *m*, loop length *L*, and how fast *f* widens.**

---

## 2. Queue splitting: how many queues, and rotation vs. choice

### What studios did

- **Rocket League (2015–2026), the long-running example.**
  - Its first extra-mode slot was a single rotating playlist. "Snow Day" "will officially replace the 'Mutator Mashup' online playlist" ([P] [2015-11-25](https://store.steampowered.com/news/app/252950/view/377537016001736844)).
  - In 2018 it cut casual extra modes when ranked versions arrived: "Due to population split, keeping Extra Modes in both Casual and Competitive could lead to great difficulty finding matches". It let players "hide your Rank for these modes" instead ([P] [Season 9 details, 2018-09-13](https://store.steampowered.com/news/app/252950/view/2454914931399746345)).
  - "The Solo Standard Competitive Playlist is no longer available" ([P] [Sept 2020 patch](https://store.steampowered.com/news/app/252950/view/3819572206264408365)).
  - In 2023 it merged Competitive and Extra Modes into one card so players "can queue for all six Competitive Playlists at once". Snow Day and Dropshot now alternate by season instead of running side by side ([P] [v2.34](https://store.steampowered.com/news/app/252950/view/5395938616788809965); [preview](https://store.steampowered.com/news/app/252950/view/5382426458970660898)).
  - In 2025, the mode rotating out of Competitive moves to Casual ([P] [v2.56](https://store.steampowered.com/news/app/252950/view/1811138915363363)).
- **Splitgate: Arena Reloaded (Dec 2025), a relaunch after a weak launch.** It added Mode Select, with a hard floor: "Players will be required to select 3 modes minimum at launch to ensure that we see healthy player numbers across available playlists. We will monitor this closely and loosen this restriction as soon as we can ensure players are finding matches quickly". It also added multi-queue and "a cumulative XP bonus for players who queue for multiple playlists" ([P] [launch notes](https://store.steampowered.com/news/app/677620/view/1819386365087873)).
- **Halo Infinite Match Composer.** Every mode is on by default. Deselecting "adds more constraints to the matchmaking search parameters" and can mean "longer wait times". Ranked does not support it ([P] [Match Composer preview](https://www.halowaypoint.com/news/match-composer-preview)).
- **Brawlhalla.** Brawlhalla has a very large population, but it still runs extras as **limited-time single queues** rather than permanent ones: a 3v3 Brawlball queue for two weeks, map queues for one week each, and an "Experimental queue" for testing mechanics ([P] [Patch 10.02](https://store.steampowered.com/news/app/291550/view/1822556746157471); [10.03 preview](https://store.steampowered.com/news/app/291550/view/1824644522840411)). It also runs a rotating "Brawl of the Week" ([P] [Patch 11.00](https://store.steampowered.com/news/app/291550/view/1846651706672605)).
- **Brawl Stars, at the opposite extreme.** Hundreds of thousands of players, but still curated rotation. A fan guide reports that Season 20 cut simultaneous event slots from 6 to 4, citing "Match making times being inconsistent due to scattering of the player base especially for high trophy players" ([W] [AFK Gaming](https://afkgaming.com/mobileesports/guide/all-available-maps-and-game-modes-in-brawl-stars-season-20); no Supercell primary found). Supercell confirmed the same high-end pressure in 2024: "matchmaking times get too long at high Trophy matches", so "after 1000 Brawler Trophies, players will all be in the same matchmaking pool". Pros and casuals may meet, "but it should be rare" ([P] [Trophy Season rework, 2024-10-15](https://supercell.com/en/games/brawlstars/blog/news/trophy-season-rework-is-coming)).
- **Awesomenauts' advice for small games:** keep one core matchmade mode and offer the rest only through friend invites, or let players state a preference but expect complaints ([P] [Joost 2015](https://joostdevblog.blogspot.com/2015/09/designing-matchmaking-for-smaller.html)).
- **Fall Guys** cut Solos lobbies from 60 to 40 because "This will speed up matchmaking times" ([P] quoted in [MP1st](https://mp1st.com/title-updates-and-patches/fall-guys-update-1-20-jumps-out-for-lobby-size-changes-this-february-21)). **Synthesis:** match size *m* is itself a matchmaking lever.

### Synthesis

Mode choice at low population follows a clear progression, from most to least friendly to a small pool:

1. **One queue, curated rotation:** the server picks mode and map (Brawl Stars, RL's Mutator Mashup, Brawlhalla's weekly queue).
2. **One queue with multi-select and a minimum** (Splitgate's 3-mode floor, Halo's all-on default, RL's six-at-once).
3. **Free choice of separate queues.** This is what every game here moved *away from* when population fell.

Ranked and casual are the most expensive split, because ranked also narrows *f*.

---

## 3. Widening the search over time

- **Standard mechanism (PlayFab).** "A rule gradually adjusts its threshold over time." Example: a skill threshold of 0.2 "expand[s] by 0.1 every 5 seconds", eventually reaching 0.5. "After a configured time, the rule stops filtering entirely", yet "even when a rule becomes optional, it still contributes distance to the sorting step." Tickets past `GiveUpAfterSeconds` are cancelled. Under load, "partitions merge back together" as volume drops ([P] [PlayFab: How matchmaking works](https://devdocs.xbox.com/services/playfab/multiplayer/matchmaking/how-matchmaking-works)). AWS recommends you "relax your rules over time so that you can be sure that your players eventually find a match" ([P] [AWS GameTech, 2017](https://aws.amazon.com/blogs/gametech/questions-of-matchmaking/)).
- **Concrete values in a shipped game.** Awesomenauts used a start window of "at most 500 skillpoints" (average 10,000), would "double the allowed geographical distance and skill difference every 20 seconds", and "stretched the allowed league range a bit further when few players were playing" ([P] [Joost 2018-01-07](https://joostdevblog.blogspot.com/2018/01/the-limitations-of-matchmaking-without.html)).
- **Batch instead of first-come.** Awesomenauts' later "Galactron" gathers players and matches "once every couple of minutes". It wants "more than 100 people at the same time" per round and improves "up to as many as 300". At low population, "we just perform matchmaking anyway so that waiting times never go beyond a maximum." A round every 5 minutes meant "a waiting time of 2.5 minutes" on average, but "average ping has indeed increased significantly" ([P] [Joost 2018-09-16](https://joostdevblog.blogspot.com/2018/09/the-awesomenauts-matchmaking-algorithm.html); [2016-10-08](https://joostdevblog.blogspot.com/2016/10/the-first-results-of-new-awesomenauts.html)).
- **Widening has a cost.** Overwatch tried "extending the MMR range allowed in matches to mitigate longer queue times" and "the matches ended up less balanced and less fun as a result." It now reports a floor of 40% win probability, with 80% of Competitive Role Queue matches at 45–55% ([P] [Weekly Recall: Meet Your Matchmaker](https://overwatch.blizzard.com/en-us/news/24224365/weekly-recall-meet-your-matchmaker/)).
- **Some small games tightened instead.** Omega Strikers (2023): "Matchmaker will take a bit longer to find games, but there should be fewer lobbies with high rank disparity", with rules set per rank band ([P] [July 13 micropatch](https://store.steampowered.com/news/app/1869590/view/5124582150182709997)).
- **Show players the wait.** Rocket League replaced exact counts with "Amazing / Great / Good" labels in 2021 ([P] [v2.01](https://store.steampowered.com/news/app/252950/view/4032396530960362986)). It then added an "Average Wait Time" counter ([P] [v2.26](https://store.steampowered.com/news/app/252950/view/5070528810286473208)), and in 2026 restored total and per-playlist population numbers ([P] [v2.66](https://store.steampowered.com/news/app/252950/view/1826992588588691)).
- **Give players something to do while they wait.** "Allow players to play *all* single player content while waiting for an opponent … players are willing to wait much longer" ([P] [Joost 2015](https://joostdevblog.blogspot.com/2015/09/designing-matchmaking-for-smaller.html)).

### Compensate in the rating when the match is bad

When the matchmaker had to compromise, several games adjust rating gains and losses afterwards:

- **Brawl Stars "Underdog/Outsider" (2020).** If teammates differ by more than 200 trophies, or the opponents average more than 200 above you, you "lose fewer trophies" on a loss and gain more on a win. It is shown at match start ([P] [Supercell, 2020-03-10](https://supercell.com/en/games/brawlstars/fr/blog/game-updates/underdog-nouveau-système-de-matchmaking)). It was removed above 1,000 trophies when those pools merged ([P] 2024 rework).
- **Omega Strikers.** Solo queuers "benefit from higher minimum rating gains if they win a match to balance out cases where they had to fill into a less than ideal match" ([P] [2.2 notes](https://store.steampowered.com/news/app/1869590/view/5129083846358102150)).

---

## 4. Region pooling

- **Global pools with ping scoring are what small games actually run.**
  - LLB's community is global and connection "depends a lot on who you play with" ([P] YinYin).
  - Awesomenauts' Galactron batches globally and weights ping. It ignores ping to teammates so that "the ping with opponents [is] a lot more important." Off-hours players from smaller regions suffer: at 20:00 CET "the vast majority … will be from Europe" ([P] [Joost 2018](https://joostdevblog.blogspot.com/2018/09/the-awesomenauts-matchmaking-algorithm.html)).
  - Knockout City ran its cross-play beta as "a global dodgebrawl event", not limited to regions ([P] [2021-03-31](https://store.steampowered.com/news/app/1301210/view/4023377846272214355)).
- **Let players choose a region.** Rocket Arena (Nov 2020): "Players can now switch regions if they'd like to matchmake using servers in a different location" ([P] [Update 2.3](https://www.ea.com/games/rocket-arena/amp/news/update-2-3)).
- **Shrink the server footprint as population falls.** Omega Strikers: "We spun down a couple rarely-used servers in some regions" (Oregon, Montreal, Ireland, Seoul) ([P] [Season ∞, 2025-09-16](https://store.steampowered.com/news/app/1869590/view/1810503566589084)). Halo names "smaller populations around specific data centers" as a cause of unhealthy playlists ([P]).
- **Synthesis.** At 200 CCU, hard region queues mean a few dozen players per region at the trough, which falls below the 3v3 thresholds in §1. Use **one global queue, with ping as a widening rule**: start with "same region", then relax to neighbouring regions after N seconds. Keep data centres few (NA-East, EU-West, maybe one APAC) and put the netcode budget into tolerating 100–150 ms cross-region play.

---

## 5. Bot backfill and bot-filled lobbies

### What studios did

| Game | Where bots appear | Disclosure | Rating |
|---|---|---|---|
| **Rocket Arena** (2.3, Nov 2020) | "Bots will now fill in for players in all standard playlists if not enough players are found during matchmaking"; "bots will scale in difficulty based on the average skill of the players" ([P] [Update 2.3](https://www.ea.com/games/rocket-arena/amp/news/update-2-3)) | Not stated | Not stated |
| **Knockout City** (3.10, Oct 2022) | "Restricted to some NEW players in Street Play ONLY, and only for their first few matches"; not in League Play ([P] notes quoted in [MP1st](https://mp1st.com/news/knockout-city-update-3-10-blasts-out-this-october-28)) | "Bots are always marked with a new bot icon on their Player Card" | Kept out of ranked |
| **Splitgate** (2022) | Per 1047's blog as summarised: outside the new-player experience, "we don't intentionally match you with bots". Bots appeared because the matchmaker was "slow" and "forgetful", and bots replace leavers ([R] [GamingOnLinux](https://www.gamingonlinux.com/2022/04/splitgate-is-getting-a-new-matchmaking-system-that-wont-suck); original blog 404) | — | — |
| **Rocket League** (2021) | Casual only: "Bots will replace any players who leave the match"; players can join "when you replace a bot in a Casual match" ([P] [v2.01](https://store.steampowered.com/news/app/252950/view/4032396530960362986); [v1.50](https://store.steampowered.com/news/app/252950/view/2447032361184402601)) | Bots are visible as bots | Competitive never backfills |
| **Awesomenauts** | When the timer ran out, "start with fewer players and add bots to complete the teams"; "missing spots are filled with bots until more players are found" ([P] [Joost 2018-01-28](https://joostdevblog.blogspot.com/2018/01/entertaining-players-while-waiting-for.html); [2015](https://joostdevblog.blogspot.com/2015/09/designing-matchmaking-for-smaller.html)) | Visible | — |
| **Fortnite** (2019) | Bots in non-competitive modes for newer players, fewer as you improve, none in competitive ([R] [PC Gamer](https://www.pcgamer.com/uk/fortnite-is-getting-skill-based-matchmaking-and-bots)) | Not disclosed per match | Not in competitive |
| **Marvel Rivals** | Players allege hidden bot lobbies in Quick Match after losing streaks. NetEase has not confirmed ([W]/[R] [Dexerto](https://www.dexerto.com/gaming/marvel-rivals-bots-infested-quickplay-matches-3014882), [MP1st](https://mp1st.com/news/marvel-rivals-bots-snuck-in-quick-match-to-ensure-win-after-multiple-losses)) | Undisclosed: the cause of the backlash | — |

**The downsides are on record.** With Awesomenauts' early bot-start, joining a match in progress was unfair ("it feels unfair if bots already lost a tower before you even joined"). "Matchmaking issues quickly became the number one complaint." Ronimo raised the pre-match wait to 2 minutes, then 4, before letting bots fill in ([P] Joost 2015, 2018).

### Synthesis

The pattern is consistent:

- **Disclose bots.** Use an icon, as Knockout City did. Undisclosed bots (Marvel Rivals) become a trust scandal.
- **Keep them out of ranked and rating.** None of the primary sources puts bots in ranked.
- **Use them for onboarding and casual backfill**, ideally scaled to lobby skill (Rocket Arena).
- **Let a human replace a bot mid-match.**

This game has a specific problem: **a crate-economy bot must haul, stack and steal competently, or the match is broken.** Bot quality is therefore a gameplay dependency for any backfill plan, not just an AI nicety.

---

## 6. Party vs. solo

- **Hard premade segregation is expensive.**
  - Omega Strikers (Oct 2026): trios had been matched "only into other groups of two or more", but "this hard restriction on matchmaking is not improving match quality enough to justify the increase in queue times it causes", so the plan is to drop it ([P] [Oct 12 patch](https://store.steampowered.com/news/app/1869590/view/1846018067942627)).
  - Earlier, Omega Strikers made players above Challenger "queue up solo" in Competitive ([P] [2.2](https://store.steampowered.com/news/app/1869590/view/5129083846358102150)).
  - Overwatch found two-stacks waited longest because each needed a matching stack, and that "allow[ing] players to group together even when there's a difference in the MMR" was a major cause of skill disparity ([P] [OW2 part 2](https://overwatch.blizzard.com/news/23910161); [R] [esports.gg on Season 11](https://esports.gg/news/overwatch/overwatch-2-season-11-matchmaking-update)).
- **Restrict party spread instead of splitting queues.** Rocket League requires undersized parties of two in 3v3 Competitive to be "within three Ranks of each other … so the third teammate … will join a team that is roughly the same Rank" ([P] [2021-08-09](https://store.steampowered.com/news/app/252950/view/4032396530955817609)). It cut Solo Standard as a separate queue (2020).
- **Score premades rather than forbid them.** Galactron scores "a 3 player premade versus 3 solo players" at 0%, so it avoids that pairing when it can and accepts it when it can't. "We can't split premades so this can't be solved" ([P] Joost 2016, 2018).
- **Solo-filling a team queue.** Brawlhalla added solo queue for Ranked 2v2 in 2026 with its own leaderboard: "Solo Ranked 2v2 experiences may vary, we still recommend queuing with a friend" ([P] [Season 40](https://store.steampowered.com/news/app/291550/view/1827626365768768)).
- **Party rating rule.** Brawl Stars puts a game room in "the highest pool that a player has in the room" ([P] [Supercell 2019-02-06](https://supercell.com/en/games/brawlstars/blog/release-notes/maintenance-changes)). Rocket League reportedly moved party skill toward the highest member to stop boosting ([R] [TheSixthAxis](https://www.thesixthaxis.com/?p=272147)).
- **Synthesis.** At 200 CCU: **one queue that mixes solos and parties.** Cap the skill spread within a party, rate a party at or near its top member, penalise premade-vs-solo pairings in the score rather than forbidding them, and compensate solos in rating (§3).

---

## 7. Leaver and rejoin policy

- **Rocket League, the most fully documented.**
  - Competitive: a warning, then escalating matchmaking bans starting at 5 minutes, with reconnect attempts while the match is live ([R] summary of [Epic help](https://www.epicgames.com/help/c-202300000001622/c-202300000001682/what-happens-when-i-leave-or-get-disconnected-from-a-competitive-match-in-rocket-league-a202300000010289); the page refused direct fetch).
  - Casual (2021): "Leaving a single match per day won't result in a penalty". After that the ladder runs 5, 10, 20, 40, 60 and 120 minutes, with "a 12-hour cooldown before the ban level de-escalates by one level". Teams can "Vote to Concede". Leavers are not backfilled into the same match, and "Casual matches with a score difference of three goals or more will not be backfilled" ([P] [v2.01](https://store.steampowered.com/news/app/252950/view/4032396530960362986)).
  - Forfeit votes are allowed only after 90 seconds in team Competitive ([P] [v1.50](https://store.steampowered.com/news/app/252950/view/2447032361184402601)).
  - Under a DDoS, the server disconnects everyone and awards "a No Contest … attack victims no longer see an MMR loss" ([P] [2024-08-23](https://store.steampowered.com/news/app/252950/view/5875595438692278335)).
  - "Players can now reconnect more than once if they crash or disconnect multiple times during a Ranked match" ([P] [v1.61](https://store.steampowered.com/news/app/252950/view/3763229740660993803)).
- **Omega Strikers (2023).**
  - Early inactivity cancels the match: "A player that causes an automatic match cancellation will be penalized as if they had lost", and so is their premade. The first offence in 2 weeks is free.
  - Teams may surrender if 2 sets down or a teammate is inactive (inactive = disconnected or no input for 40 s) ([P] [2.2](https://store.steampowered.com/news/app/1869590/view/5129083846358102150)).
- **Brawlhalla** lets you report "leavers who don't reconnect" from the post-match screen ([P] [11.00](https://store.steampowered.com/news/app/291550/view/1846651706672605)).
- **Synthesis.** For a small population, a leaver costs the *remaining* players a whole re-queue, so policy should protect the people who stay:
  - an early-match "remake" (cancel with no rating change for the others);
  - a reconnect slot held for the leaver;
  - bot backfill in casual, which a human can then take over;
  - no backfill into lopsided matches;
  - no rating loss for teammates of a leaver in ranked;
  - escalating bans that decay, plus one free casual leave per day, because harsh bans also shrink the pool.

---

## 8. What else small games did to survive

- **Gather the population at set times.** LLB tells new competitors: "entering the ranked queue during a sale, when other new players are joining in". It runs monthly "New Player Challenge" tournaments ([P] [Ranked Season 17](https://store.steampowered.com/news/app/553310/view/1834602721184140)). Joost reports one developer "taking the multiplayer functionality offline altogether during weekdays" ([P] 2015). LLB players also matchmake through the community Discord ([P] YinYin).
- **Lower the barrier to entry.** Joost recommends pricing multiplayer-only games low or going F2P, and running sales, to grow the pool ([P] 2015).
- **Plan for end of life.** When Splitgate 1's dedicated servers were sunset, it moved to peer-to-peer with a server browser: "A player will host a match and act as the server", with no anti-cheat ([P] [2025-08-22](https://store.steampowered.com/news/app/677620/view/1808601382462987)). Knockout City closed after failing to "attract and retain" enough players ([R] [Nintendo Life](https://www.nintendolife.com/news/2023/02/live-service-game-knockout-city-will-shut-down-this-june)); Spellbreak closed with about 165 players on Steam in its last 30 days ([R] [TheGamer](https://www.thegamer.com/spellbreak-servers-shut-down-developer-bought-by-blizzard/)). Spectre Divide shut down despite a peak of ~10,000 CCU ([R] [PCGamesN](https://www.pcgamesn.com/spectre-divide/shutting-down)).

---

## 9. Implications for this game (Synthesis)

1. **One matchmade queue at launch, pooled globally.** Modes and maps are picked by server-side rotation or multi-select with a minimum. No player-chosen per-mode queues. Ranked, if any, should start as a hidden MMR on that same queue, plus timed events or tournaments, not a parallel queue.
2. **Choose the match format with the queue in mind.** Every extra player per match, and every extra minute per match, raises the CCU needed per queue (§1 table). A 3v3 (or 2v2v2) of 6–10 minutes fills in about 1–2 minutes at about 100 CCU with a loose skill band. Team Buddies-style 4 teams × N Buddies needs this check before it is locked.
3. **Widen by time, cap the wait, show the wait.** Start with a tight skill and same-region window, double it every ~20 s, make it optional by ~90–120 s, and show an estimated wait. Let players practise solo while they wait.
4. **Bots: disclosed, casual and onboarding only, never rated, replaceable by humans.** The crate economy needs bots that haul, stack and steal properly. This is a hard dependency if bots are used for backfill.
5. **Parties and leavers protect the shared pool.** Mix solos and parties under a party-spread cap with rating compensation. Use early remakes, reconnect slots, casual bot backfill, decaying bans, and no rating loss for teammates who stay.
6. **Plan for low population from day one.** Few data centres, global play tolerated by the netcode, scheduled peak events, and a peer-to-peer or community-server fallback for end of life.
