# Dedicated server cost per match-hour

Research notes for [#40](https://github.com/ALLiDoizCode/big_back/issues/40). Research date: 2026-10-19.

**Question.** Every rated match runs on a dedicated server, and all matchmaking goes through one queue pooled across regions ([ADR 0012](https://github.com/ALLiDoizCode/big_back/blob/main/docs/adr/0012-one-rated-queue-with-short-starts.md)). What does a 5v5 match-hour cost on real hosting? What does that come to at about 200 players online across 3 regions, and at what population does it stop being affordable for a small indie? What did small games do when their servers grew too costly, and should the spec include an end-of-life fallback?

Already covered elsewhere and not redone here:

- The server model, tick and bandwidth budget: [netcode-budget research](https://github.com/ALLiDoizCode/big_back/blob/research/netcode-budget/docs/research/netcode-budget.md) (§1 dedicated vs. host client, §2.4 bandwidth table, §2.5 CPU).
- Queue, regions and population curve: [ADR 0012](https://github.com/ALLiDoizCode/big_back/blob/main/docs/adr/0012-one-rated-queue-with-short-starts.md) and the [low-CCU matchmaking research](https://github.com/ALLiDoizCode/big_back/blob/research/low-ccu-matchmaking/docs/research/low-ccu-matchmaking.md).

## How to read this doc

- Plain statements are sourced facts and link to their source. Prices come from provider pricing pages or AWS's machine-readable price list, read on 2026-10-19.
- **[Estimate]** marks a number I worked out from those prices plus the assumptions in §1. No provider or benchmark publishes per-match numbers for this stack, so every [Estimate] needs a measurement from a real server build before it becomes a decision.
- **[Synthesis]** marks my own reasoning or recommendation.
- **[Press]** marks a claim that rests on a press report because the primary page could not be read.
- Pages were read through a page-summarising fetch tool, so quotes are close to verbatim but not character-perfect.
- EUR prices are converted at €1 = $1.16 **[Estimate]**.

---

## 0. Answer in brief

1. **What dominates the bill is egress, not CPU.** A 5v5 server sends about 2–4.5 GB an hour (10 clients at 0.5–1 Mbit/s). At cloud egress prices ($0.09–0.10/GB) that is $0.20–0.45 a match-hour. The server's CPU, at 0.5–1 vCPU, costs $0.03–0.07 **[Estimate]**. So pick hosting where bandwidth is free or bundled.
2. **About 200 peak CCU across 3 regions is about 7,400 match-hours a month** (§1) **[Estimate]**:

   | Option | $ per match-hour | $ per month at ~200 peak CCU |
   |---|---|---|
   | Edgegap Edge Cloud, a container per match (pay per use) | ~0.40–0.43 | **~2,900–3,200** |
   | Edgegap Private Fleet, 3 reserved hosts | — | **~1,700** |
   | Amazon GameLift Servers, managed EC2 `c6a.large`, on-demand, autoscaled | ~0.09 | **~650** |
   | Bare VMs per region (Hetzner CCX, dedicated vCPU), fixed size | ~0.11 | **~850** |
   | Agones on GKE (cluster per region, hyperscaler egress) | ~0.43 | **~3,200** |
   | Agones on bare VMs (k3s on Hetzner) | ~0.12 | **~900 plus ops time** |

   Hathora and Unity's Multiplay hosting are not options: both ended in 2026 (§2.5).
3. **Affordability.** On VMs or GameLift a server costs about **$0.01 per player-hour**, or about $3–4.5 per peak CCU a month. A bill of about $2,000 a month comes at about **500 peak CCU on VMs**, but at only about **125 peak CCU on Edgegap's per-use cloud**. The bill shrinks as players leave, down to a floor of about **$160–240 a month** for 3 always-on regions. So a small indie loses its servers when **revenue** falls below that, not when the population does (§3) **[Estimate / Synthesis]**.
4. **Precedents** (§4). Splitgate (2025) and Splitgate: Arena Reloaded and Empulse (2026-09-03) moved to player hosting with a server browser, because revenue no longer covered dedicated servers. Evolve (2018) kept peer-to-peer play. Knockout City (2023) and Spellbreak (2022) released standalone server builds. Rumbleverse just shut down. In June 2026 the European Commission declined to require games stay playable, but it will draft an industry **code of conduct on "end of life"**.
5. **End-of-life fallback: yes, put it in the spec** (§5). With lightyear it costs very little if designed in now: a headless server binary that can run without the backend, a host-client lobby mode, direct connect, and a server browser. Ranked play ends at end of life, and unrated custom play goes on.

---

## 1. The per-match load **[Estimate]**

The inputs come from the ticket and the netcode research:

- **Tick:** 64 Hz fixed simulation, sending 20–30 Hz ([netcode-budget §2.1](https://github.com/ALLiDoizCode/big_back/blob/research/netcode-budget/docs/research/netcode-budget.md#21-tick-rate-and-send-rate-facts)).
- **Bodies:** about 32 characters the server simulates (10 players plus server-run characters such as camps or bots, per the ticket) and about 20–30 crates moving at once ([§2.6](https://github.com/ALLiDoizCode/big_back/blob/research/netcode-budget/docs/research/netcode-budget.md#26-recommended-prototype-budget-synthesis--guess)). Resting crates are static or asleep and do not replicate (§2.3 there).
- **Bandwidth:** about 60 B per moving body per send ([§2.4](https://github.com/ALLiDoizCode/big_back/blob/research/netcode-budget/docs/research/netcode-budget.md#24-bandwidth-table-guess)). 57 bodies × 60 B × 30 Hz ≈ 103 KB/s ≈ **0.8 Mbit/s per client**, or about 0.55 Mbit/s at 20 Hz. The ticket caps it at under 1 Mbit/s.
- **CPU:** the server "steps physics once per tick and never rolls back", and 100 bodies at 64 Hz "should cost a few percent of one core" ([§2.5](https://github.com/ALLiDoizCode/big_back/blob/research/netcode-budget/docs/research/netcode-budget.md#25-cpu-server-vs-client-prediction)). The lightyear maintainer's advice is "N server processes on a machine with N cores, with each server taking charge of one game room" ([lightyear #834](https://github.com/cBournhonesque/lightyear/issues/834)). I budget **0.5–1 vCPU per match**, with 1 vCPU as the safe case.

**Egress per match-hour** (server to 10 clients, payload only, no packet headers):

| Per client | GB per match-hour | Monthly egress at ~7,400 match-hours |
|---|---|---|
| 0.5 Mbit/s | 2.25 | ~17 TB |
| **0.8 Mbit/s (central case)** | **3.6** | **~27 TB** |
| 1.0 Mbit/s (cap) | 4.5 | ~34 TB |

**Population to match-hours:**

- "About 200 CCU" is read as ADR 0012's **200 players online at the busiest hour**.
- Off-peak CCU is about ⅓ of peak ([Joost van Dongen, 2014](https://joostdevblog.blogspot.com/2014/11/why-good-matchmaking-requires-enormous.html), cited in the low-CCU research). I take the daily average as 0.6 × peak, which is **120 CCU**.
- I assume 85% of online players are in a match, the rest in menus or the queue.
- That gives about **17 matches at peak and about 10 on average**, or **~7,400 match-hours a month** (10.2 × 730 h) and ~87,600 player-hours.
- Short starts ([ADR 0012](https://github.com/ALLiDoizCode/big_back/blob/main/docs/adr/0012-one-rated-queue-with-short-starts.md)) make some matches run below 10 players. That lowers egress a little but not the server count, so the estimate stays conservative.

---

## 2. Prices per option

### 2.1 Per-match containers: Edgegap

- **Edge Cloud:** "$0.00115 / vCPU / minute" ($0.069 per vCPU-hour), a "2:1 RAM-vCPU Ratio", egress at "$0.10 / GB / month", no commitment, $0 with no servers running, and a "$1 minimum" in any month with a deployment. Deployments are 0.25 to 4 vCPU. The rate card was last reviewed 2026-09-04 ([Edgegap pricing](https://edgegap.com/pricing), [rate card](https://edgegap.com/pricing.md)).
- **Private Fleet** (reserved bare metal): "$280 per host / month" on a 12-month term ($350 billed monthly). A host has 16 vCPU (15 usable) and includes 6 TB of egress a month. Egress beyond that is billed at the public cloud rate, which the page does not state (same pages).
- **Bevy fit:** the lightyear maintainer suggests Edgegap through [bevygap](https://github.com/RJ/bevygap) ([#834](https://github.com/cBournhonesque/lightyear/issues/834)). bevygap describes itself as "(WIP)", and its last push was 2024-12-23, so expect to maintain the glue yourself.
- **[Estimate]** At 1 vCPU: compute 7,446 × $0.069 ≈ $514, plus egress 26,800 GB × $0.10 ≈ $2,680, for **≈ $3,190 a month, or $0.43 a match-hour**. At 0.5 vCPU it is ≈ $2,940. Private Fleet with 3 hosts (one per region): $840 plus about 8.8 TB of overage. At an assumed $0.10/GB that is **≈ $1,720**.
- **[Synthesis]** Edgegap's selling point is placing each match's server near its players. That suits ADR 0012, where 94% of matches mix regions. But egress at $0.10/GB makes it the most expensive option at this size.

### 2.2 Managed fleets: Amazon GameLift Servers

- **Bandwidth is free:** "Network bandwidth is free for all instance types generation 6 and later." Existing customers get this "automatically" ([GameLift Servers pricing](https://aws.amazon.com/gamelift/servers/pricing/)).
- **Instance prices.** On-demand Linux prices from the AWS price list (`AmazonGameLift` offer, published 2026-10-13):

  | Instance | US East (N. Virginia) | EU (Frankfurt) | Asia Pacific (Singapore) |
  |---|---|---|---|
  | `c6a.large` (2 vCPU, 4 GB) | $0.099/h | $0.113/h | $0.114/h |
  | `c6a.xlarge` (4 vCPU) | $0.19/h | $0.218/h | $0.220/h |

  Each price is described as an instance hour "with free network bandwidth" ([price list](https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonGameLift/current/us-east-1/index.json)). The pricing page's own example puts `c6a.large` at "$.099 OD / $.03 Spot" in US East (Ohio) ([pricing page](https://aws.amazon.com/gamelift/servers/pricing/)).
- **Integrating a Rust server:** AWS's open-source [Game Server Wrapper](https://github.com/amazon-gamelift/amazon-gamelift-servers-game-server-wrapper) runs "any game server" executable on managed EC2, container or Anywhere fleets "without directly integrating your game server code with the server SDK". It is written in Go and was last pushed 2026-09-29. Its limits: no custom SDK callbacks and no player-session management.
- **[Estimate]** Two matches per `c6a.large`. Autoscaling to the average load (10.2 matches) plus one spare instance per region gives about 8.1 instances × 730 h × $0.109 ≈ **$640 a month, or ≈ $0.09 a match-hour**, with no egress line. Spot instances would cut the compute by up to about 70% but can be reclaimed mid-match.

### 2.3 Bare VMs per region: Hetzner Cloud (dedicated vCPU)

- **Prices:** Hetzner repriced on **15 June 2026** (for new orders and rescales). Monthly prices, excluding VAT and IPv4 ([Hetzner price adjustment](https://docs.hetzner.com/general/infrastructure-and-availability/price-adjustment/)):

  | Plan | Germany/Finland | USA (Ashburn, Hillsboro) | Singapore |
  |---|---|---|---|
  | CCX13 (2 dedicated vCPU) | €42.99 | €43.49 | €53.99 |
  | CCX23 (4 dedicated vCPU) | €85.99 | €87.49 | €108.49 |

  Press reports put CCX13 at €15.99 before the change, so the increase was about 2.7× **[Press]** ([wz-it](https://wz-it.com/en/blog/hetzner-price-increase-june-2026-cpx-ccx-alternatives/)).
- **Traffic:** CCX includes "20-60 TB for EU locations" and "1-8 TB for US & SIN, depending on plan". Overage is "€ 1 ($1.20)/TB" ([Hetzner traffic docs](https://docs.hetzner.com/robot/general/traffic/)). Hetzner's Singapore page reportedly prices overage at $8.49/TB **[Press]**. Even at that rate, the egress line is tens of dollars.
- **[Estimate]** Each region is sized for its own peak at 1 vCPU per match: 3 × CCX23 in the US, 3 in the EU and 2 in Singapore, which is 32 vCPU. That costs **€737 ≈ $855 a month, or ≈ $0.11 a match-hour** at average load. At 0.5 vCPU per match, half the servers do. The floor of one CCX13 per region is **€140 ≈ $165 a month**.
- **[Synthesis] Caveats:**
  - Use dedicated vCPU (CCX), not shared (CPX/CX), so a 64 Hz tick does not jitter from noisy neighbours.
  - Hetzner offers no game-specific DDoS filtering. OVHcloud's GAME bare-metal line includes "Game DDoS Protection" and "1Gbps unmetered public bandwidth", but starts at $397/month ([OVHcloud Game](https://www.ovhcloud.com/en/bare-metal/game/)). That is a step up for later, not for launch.
  - The June 2026 change shows that cheap VM prices can move by 2–3× in one step.

### 2.4 Self-run orchestration: Agones

- **What it is:** Agones is Apache-2.0 open source. It now lives at `agones-dev/agones`, and v1.61.0 was released 2026-09-24 ([GitHub](https://github.com/agones-dev/agones)). It costs nothing itself. You pay for the Kubernetes cluster, its nodes and egress, plus the time to run it.
- **GKE:** "A flat cluster management fee of $0.10 per cluster per hour" applies to every cluster. The free tier gives "$74.40 in monthly credits per billing account", which covers one zonal or Autopilot cluster ([GKE pricing](https://cloud.google.com/kubernetes-engine/pricing)). I could not read GCP's egress table through the fetch tool. As a hyperscaler reference, AWS EC2 internet egress from US East is $0.09/GB for the first 10 TB a month after a 100 GB free tier, then $0.085/GB ([AWS price list, `AWSDataTransfer`](https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AWSDataTransfer/current/us-east-1/index.json)).
- **[Estimate]**
  - **Agones on a hyperscaler:** about $146 in cluster fees for 3 clusters with one free, plus about $640 in nodes, plus about $2,400 of egress at $0.09/GB. That is **≈ $3,200 a month**, the same egress problem as Edgegap.
  - **Agones on Hetzner nodes** (k3s): the §2.3 VMs plus a small control-plane VM per region, **≈ $900 a month plus operations time**.
- **[Synthesis]** Agones buys allocation, health checks and fleet autoscaling, not cheaper hardware. At about 20 concurrent matches, a small allocator over a static VM pool does the same job with less to run.

### 2.5 Gone in 2026: Hathora and Unity Multiplay

- **Hathora:** `hathora.dev/pricing` now redirects to Nitrado's `gamefabric.com`. Fireworks AI acquired Hathora, and its game-hosting customers were moved to Nitrado's GameFabric (Nitrado press release, 4 March 2026) ([GamesMarket](https://www.gamesmarket.global/nitrado-and-hathora-collaborate-on-customer-transition-to-gamefabric/)). One vendor blog says hosting ended on 5 May 2026 **[Press]**. GameFabric publishes no prices ("Individual Pricing") ([GameFabric](https://gamefabric.com/pricing)).
- **Unity Multiplay Game Server Hosting:** Unity ended direct support on 31 March 2026 and licensed the software to Rocket Science Group. From 1 April 2026, customers could no longer scale or make new allocations ([Unity status notice](https://status.unity.com/info_notices/362941)).
- **[Synthesis]** Two of the four "Agones/Edgegap/Hathora-style" vendors left the market within six weeks of each other. Keep orchestration behind a thin seam (allocate a server, get `ip:port`, hand out a lightyear `ConnectToken`), so that moving vendors means changing that seam, not the server.

---

## 3. When does it stop being affordable? **[Estimate / Synthesis]**

Costs scale roughly linearly with player-hours, above a fixed floor per region:

| | Per player-hour | Per peak CCU, per month | Floor, 3 regions always on |
|---|---|---|---|
| Edgegap Edge Cloud | ~$0.037 | ~$16 | ~$0 (scales to zero) |
| GameLift `c6a.large` | ~$0.007 | ~$3.2 | ~$240 (one instance per region) |
| Hetzner CCX | ~$0.010 | ~$4.3 (in steps of one VM) | ~$165 (one CCX13 per region) |

Monthly bill by peak CCU:

| Peak CCU | Edgegap Edge Cloud | GameLift / Hetzner VMs |
|---|---|---|
| 20–50 | $300–800 | $165–300 (the floor) |
| **200** | **~$3,200** | **~$650–850** |
| 500 | ~$8,000 | ~$1,700–2,200 |
| 1,000 | ~$16,000 | ~$3,500–4,300 |

What this means:

- **A per-use cloud that bills egress stops being cheap early.** At this game's bandwidth, Edgegap Edge Cloud passes $1,000 a month at about 60 peak CCU. Its only advantage is that it costs nothing at zero players, which helps for playtests and a soft launch.
- **On VMs or GameLift, servers are not what sinks a small game.** The bill is about $0.01 per player-hour. At 200 peak CCU that is about $850 a month, about the cost of one month of a cheap contractor. It reaches about $2,000 a month at about 500 peak CCU, and by then the population is a revenue base, not a burden. Below about 50 peak CCU the bill sits at the $165–240 floor.
- **The real threshold is revenue, not CCU.** A premium indie earns most of its money at launch, and its long-tail revenue falls faster than its player count. Splitgate's servers had cost "hundreds of thousands of dollars over the past couple of years" (§4.1). That is a game that was briefly huge, with costs that lagged its revenue. The rule for this project: **keep the always-on floor under about $250 a month, and switch to the end-of-life fallback (§5) before monthly revenue drops below the bill.**
- **The biggest saving is bandwidth.** Every 20 Hz instead of 30 Hz, and every byte of quantization in lightyear's `replicate_as`/`replicate_diff` (netcode-budget §2.2), lowers the egress-billed options in proportion. Measure real bytes per match before choosing a vendor.
- **Recommendation for launch:**
  - Run a fixed VM pool per region (Hetzner CCX, or GameLift `c6a.large` for free bandwidth plus managed autoscaling), sized for each region's peak.
  - Use a small allocator in the matchmaking backend.
  - Keep Edgegap as overflow or for playtests if needed.

---

## 4. What small-population games did when servers grew too costly

### 4.1 Splitgate (1047 Games), 2025 and 2026

- **2025-07-22:** "we're also going to be shutting down OG Splitgate servers in a month. While we'd love to keep servers online indefinitely, it's cost us hundreds of thousands of dollars over the past couple of years, and we have to prioritize our team… we are exploring the possibility of supporting offline or peer-to-peer matches" ([Steam, "Building the Splitgate you deserve"](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1805431065504926)).
- **2025-08-22:** "Starting August 29, the original Splitgate will be moving to peer-to-peer… A player will host a match and act as the server. Other players can view the game in the server browser and join from there." Everyone got every cosmetic, and the Battle Pass, Store and Drops were sunset ([Steam, "Splitgate 1 Lives On Through Peer-To-Peer Support!"](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1808601382462987)). Press reports that anti-cheat was dropped **[Press]** ([Generación Xbox](https://en.generacionxbox.com/good-news-splitgate-is-not-closing-it-will-remain-active-thanks-to-player-support/)).
- **2026-08-26:** 1047 Games posted "An Update on Empulse, Splitgate: Arena Reloaded, and What's Next" ([Steam](https://steamcommunity.com/ogg/2918300/announcements/detail/698774255287926897)). I could not read the post's body. As quoted by GamingOnLinux: "neither game is generating enough revenue to support the ongoing cost of dedicated servers… On Thursday, September 3, both games will move to peer-to-peer hosting with a server browser. Dedicated servers and matchmaking will shut down… players will still be able to host matches, find games, and play with friends forever" **[Press]** ([GamingOnLinux](https://www.gamingonlinux.com/2026/08/1047-games-stop-development-on-empulse-and-splitgate-arena-reloaded/)). The developers' last matchmaking session was posted on Steam on 2026-09-01, "before the switch to P2P servers later this week" ([Steam](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1842212951314446)).

### 4.2 Others

| Game | What happened | Fallback |
|---|---|---|
| **Evolve Stage 2** (2K / Turtle Rock), 2018 | Dedicated servers shut down on 3 September 2018. Stage 2 was delisted and ranked play and leaderboards ended **[Press]** ([TheSixthAxis](https://www.thesixthaxis.com/2018/06/04/evolves-dedicated-pc-servers-to-be-shut/), [MP1st](https://mp1st.com/news/evolve-dedicated-servers-are-shutting-down-free-to-play-version-being-pulled-from-pc-stores)) | Peer-to-peer play went on in "Legacy Evolve", and DLC stayed. A single source says all services ended in 2023 **[Press]** |
| **Knockout City** (Velan Studios), 2023 | Service ended on 6 June 2023 **[Press]** ([Gematsu](https://gematsu.com/2023/02/knockout-city-to-end-service-on-june-6-private-hosted-server-edition-to-be-released-for-pc)) | Free "Knockout City – Private Server Edition" on Steam (app 2915930), which invites players to keep "hosting servers for their Crews" ([Steam app details](https://store.steampowered.com/app/2915930/)). Live progression did not carry over **[Press]** |
| **Spellbreak** (Proletariat), 2022–23 | Development ended in June 2022, and servers closed in early 2023 **[Press]** ([Shacknews](https://www.shacknews.com/article/131137/spellbreak-server-shutdown-2023)) | "Spellbreak Community Version" on itch.io, signed "The Spellbreak Dev Team", with a client and a Windows server download plus a hosting guide (v2, 14 Dec 2022) ([itch.io](https://sbcommunity.itch.io/spellbreak-community-version)) |
| **Omega Strikers** (Odyssey), 2025 | "We spun down a couple rarely-used servers in some regions" (cited in the low-CCU research, [Steam](https://store.steampowered.com/news/app/1869590/view/1810503566589084)) | Shrank its region footprint instead of shutting down |
| **Rumbleverse** (Iron Galaxy / Epic), 2023 | Shut down after about six months **[Press]** ([PC Gamer](https://pcgamer.com/all-the-big-games-that-shut-down-in-2023)) | None |

### 4.3 Regulation

- The "Stop Destroying Videogames" European citizens' initiative gathered "over 1.29 million valid signatures" in 24 member states. The Commission adopted its reply on 16 June 2026 ([ECI news](https://citizens-initiative.europa.eu/news/european-commission-replies-stop-destroying-videogames-initiative-2026-06-16_en)).
- The reply ([press release IP/26/1369](https://ec.europa.eu/commission/presscorner/detail/en/ip_26_1369)):
  - "The Commission considers that at this stage it cannot propose a legal obligation to keep video games playable after they stop being provided commercially."
  - Under existing law, providers "must inform consumers about the duration and the conditions for terminating the contract".
  - The Commission will start an exchange with industry and consumer groups "to draw up an industry code of conduct on managing video games' 'end of life'", and will report on the Digital Content Directive before the end of 2026.
- **[Synthesis]** There is no legal duty yet, but a code of conduct is coming. A game that ships with a written end-of-life plan is ahead of it, and stating that plan on the store page meets the duty to tell buyers how long the service runs.

---

## 5. Is an end-of-life fallback worth putting in the spec?

**Yes. [Synthesis]** Every precedent that kept a game alive used one of two paths: player hosting with a server browser (Splitgate ×2, Evolve), or released server builds (Knockout City, Spellbreak). Both cost little when designed in, and a lot when bolted on during a wind-down. The stack already supports both:

- lightyear has a host-client mode ("the Client and the Server run in the same process… other clients can still connect"). Its `lobby` example switches between a dedicated server and a client host at runtime.
- A headless server build is a feature flag (netcode-budget §1).

The netcode research already ruled host-client out for rated play, because the host has zero latency and is the authority. That ruling holds: **ranked ends at end of life, and unrated play goes on.**

What to put in the spec:

1. **The dedicated server must run without our backend.** Today a netcode `ConnectToken` comes from a backend holding a private key shared with servers (netcode-budget §1). Add an "offline" mode in which the server makes its own key and tokens, or accepts direct connections, and reads a plain config file (map rotation, team size). This is the one piece that is expensive to add late.
2. **Keep host-client lobbies and direct connect working through development.** That means custom or private matches hosted by a player, and a direct-connect field (Knockout City's "point it at a server address"). Once players can host, a server browser or a Steam lobby list is a small extra.
3. **No vendor SDK in the server binary.** Orchestration stays behind a seam (§2.5). The released binary must not depend on Edgegap, GameLift or any other vendor existing.
4. **A wind-down checklist**, modelled on Splitgate: announce at least 60–90 days ahead **[Synthesis]**, freeze ranks and leaderboards as a final season, unlock all cosmetics (progression is identity only, per ADR 0009, so nothing breaks), release the headless server for Linux and Windows, and switch the client to the server browser.
5. **State the plan on the store page** ("rated matchmaking runs while the game is supported; at end of life, player-hosted and self-hosted servers stay available"). This meets the EU duty to inform buyers and gets ahead of the coming code of conduct (§4.3).

**Not worth it:**

- Host migration (lightyear has none, netcode-budget §1).
- Anti-cheat for player-hosted matches. Splitgate dropped it too, and unrated play among friends tolerates that.

---

## Gaps

- No server build exists yet, so CPU and bytes per match are estimates. Measure one 5v5 bot match (bytes sent per client, CPU per process) before choosing a vendor.
- I could not read GCP's egress table or GameFabric's and Rocket Science's prices. Neither GameFabric nor Rocket Science publishes prices.
- I could not read the body of the Splitgate 2026 Steam post, so its quotes come from GamingOnLinux.
- I could not confirm the regional split of peak CCU (NA/EU/APAC), so the per-region VM sizing in §2.3 is a guess.
