# Netcode and match-format budget: lightyear 0.30 + avian3d 0.7

Research ticket [#7](https://github.com/ALLiDoizCode/big_back/issues/7). It asks what the planned netcode stack allows for the match format: the server model, how many units, crates and projectiles a match can hold, how crates behave under prediction, lag compensation, and rejoin. It builds on [`bevy-crates-multiplayer-physics-particles.md`](bevy-crates-multiplayer-physics-particles.md) and does not repeat its crate comparison.

- **Research date:** 2026-10-18.
- **What I read:**
  - lightyear `main` at commit [`8abe004`](https://github.com/cBournhonesque/lightyear/commit/8abe00453b5272300db73c4f81999bfc1156e491) (2026-10-16). Its workspace version string is still `0.30.0`, and it still pins Bevy 0.19, avian 0.7 and bevy_replicon 0.44.1 ([Cargo.toml](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/Cargo.toml)). That is the 0.30 line the project plans to use.
  - bevy_replicon at tag `v0.44.1`.
  - avian at tag `v0.7.0`.
  - Issues in the lightyear and avian repos.
- **How to read this:**
  - Plain statements are sourced facts. Each one links to the file or issue it comes from.
  - **[Synthesis]** marks my own reasoning. No source states it.
  - **[Guess]** marks a number I estimated without a benchmark. No maintainer or benchmark publishes per-match numbers for this stack. Every [Guess] needs a measurement in the prototype before it becomes a decision.

Link shorthand used below:
- **LY** = `github.com/cBournhonesque/lightyear/blob/8abe004…`
- **RP** = `github.com/simgine/bevy_replicon/blob/v0.44.1`
- **AV** = `github.com/avianphysics/avian/blob/v0.7.0`

---

## 0. Answer in brief

1. **Server model.**
   - lightyear supports a dedicated headless server, a host-client mode (the server and one client in one process) and P2P.
   - Lightyear has no host migration.
   - A host client plays on the authoritative world with zero latency.
   - **[Synthesis]** Ranked play must use dedicated servers, one match per process. A host-client mode would only suit custom or private lobbies.
2. **Budget.**
   - The examples run a **64 Hz** fixed tick and replicate every **100 ms**.
   - A moving Avian body costs about **60 bytes per send** on the wire **[Guess, derived from the serialization code]**.
   - So bandwidth is roughly `60 B × moving bodies × send rate × clients`. At 30 Hz, 48 moving bodies cost each client about **0.7 Mbit/s** down **[Guess]**.
   - Bandwidth and the client's rollback CPU set the limit. Server CPU does not.
   - The most important finding is about **resting crates**. Lightyear's carry example turns Avian sleeping off for rollback. Avian then writes `Position` for every awake body every tick, so **every resting crate keeps replicating**.
   - **[Synthesis]** Crates on the pad and crates lying idle must not be awake dynamic bodies.
3. **Carried and stacked crates.**
   - The `timeline_switch` example shows the working pattern:
     - The **server decides pickup**.
     - A carried crate is made **kinematic** and posed at the holder every tick.
     - Crates near the local player switch to the predicted timeline, with a 3 m switch radius and a 5 m release radius.
     - Everything else stays interpolated.
   - Each switch to the predicted timeline forces a rollback.
   - Rollback re-runs the whole `FixedMain` schedule for the whole world. A stack of predicted crates is the costliest and most error-prone thing to predict.
   - **AI units run by the server are cheap to interpolate.** Their cost is the bandwidth they use, not client CPU.
4. **Lag compensation.**
   - It runs on the server only. It is a history of collider poses (35 ticks by default) plus **ray casts only**: there is no shape-cast or overlap query.
   - The rewind uses each client's latest interpolation delay, not a timestamp on each shot.
   - Hitscan fits it directly.
   - The projectiles demo, which is 2D only, compares three ways to send a projectile: a state entity, fire data, or a 32-slot shot buffer. The cheap options need deterministic trajectories.
5. **Rejoin.**
   - lightyear gives you the building blocks:
     - a stable `PeerId` across links;
     - a `Lifetime::Persistent` option, so a disconnecting client's units survive;
     - full-state replication to a newly connected client.
   - It has no rejoin feature: you rebind ownership yourself.
   - Netcode defaults are a **15 s** connection timeout and a **30 s** connect-token expiry. A rejoin needs a fresh token from your backend.

---

## 1. Dedicated server vs. listen/host server

### What lightyear supports (facts)

- **Topologies.** The README lists client-server, P2P and host-client topologies. This is covered in the earlier doc ([README](https://github.com/cBournhonesque/lightyear#features)).
- **Server entity.** A server is just an entity with the `Server` role, a connection component such as `NetcodeServer`, and a transport endpoint. Each client link is an entity with `LinkOf`. Game logic should start only when `Connected` is added, after netcode authentication succeeds ([LY book/…/bevy_integration/server.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/bevy_integration/server.md)).
- **Several transports at once.** One server can serve several transports at the same time, for example Steam, UDP and WebTransport ([LY book/…/connection/multi_connection.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/connection/multi_connection.md)).
- **Host-client mode:**
  - "the Client and the Server run in the same process … the in-process client talks to the server over local (crossbeam) channels; other clients can still connect" ([multi_connection.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/connection/multi_connection.md)).
  - A client counts as host when it is connected, is a `ClientOf` a server, and that server has started ([LY crates/connection/connection/src/host.rs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/connection/connection/src/host.rs)).
- **What the host sees.** The host does not predict or interpolate. In `timeline_switch`, "Timeline switching is client-local; the host-server world stays authoritative" ([LY examples/timeline_switch/src/client.rs#L141](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/client.rs#L141)). Replicon says the same: "The server always sees the entire world, even in listen-server mode" ([RP src/lib.rs#L572](https://github.com/simgine/bevy_replicon/blob/v0.44.1/src/lib.rs#L572)).
- **Changing topology at runtime.** The `lobby` example runs a dedicated lobby server. When a game starts, either the dedicated server hosts it (using rooms) or one client becomes the host in `HostServer` mode ([LY examples/lobby/README.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/lobby/README.md)).
- **No host migration.** I found no host-migration code: a search of `crates/` for "migrat" returned nothing.
- **Headless builds.** A headless server build is a feature flag: `cargo run --no-default-features --features=server` ([LY examples/README.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/README.md)).
- **Authentication for matchmade play:**
  - Netcode connections need a `ConnectToken`. A backend generates it with a private key shared with the game servers ([LY examples/auth/README.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/auth/README.md)).
  - A token carries up to 256 bytes of user data. One netcode server accepts up to 256 clients by default ([LY netcode lib.rs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/connection/netcode/src/lib.rs#L120-L125), [netcode server.rs#L32](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/connection/netcode/src/server.rs#L32)).
- **Scaling.** The maintainer's advice: "the standard is to have N server processes on a machine with N cores, with each server taking charge of one game room". For hosting he suggests Edgegap (through [bevygap](https://github.com/RJ/bevygap)) or Agones. Running several rooms in one process is also possible with rooms or visibility ([issue #834](https://github.com/cBournhonesque/lightyear/issues/834)).

### Trade-offs for a ranked game

**[Synthesis]** The comparison below is my own reasoning. The facts it rests on are cited above.

| | Dedicated server | Host client |
|---|---|---|
| Fairness | Every player sees the same delays | The host plays on the authoritative world with no latency and sees the whole world. That is a built-in advantage |
| Cheating | The server owns all game state | The host's process *is* the authority. Any hacked host can rewrite the match |
| Host leaves | Not applicable | The match ends. There is no host migration |
| Cost | You pay for servers, but at about 200 CCU the bill is small (see §2.5) | Free |
| NAT | Not needed | The host must be reachable, or you need a relay or Steam |

**Verdict [Synthesis]:**
- **Ranked:** dedicated, headless, one process per match. Matchmaking issues a `ConnectToken`.
- **Host client:** worth keeping for private or custom lobbies and local testing. The `lobby` example shows that one codebase can do both.

---

## 2. A realistic per-match budget

### 2.1 Tick rate and send rate (facts)

- **Fixed tick.** The examples' shared constants set `FIXED_TIMESTEP_HZ = 64.0` ([LY examples/common/src/shared.rs#L4](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/common/src/shared.rs#L4)).
- **Send interval.** The examples also set `SEND_INTERVAL = 100 ms` ([#L17](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/common/src/shared.rs#L17)). `timeline_switch` uses this value through `ReplicationMetadata::new(SEND_INTERVAL)` ([server.rs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/server.rs)).
- **Interpolation delay.** The delay is `max(1.2 × server send interval, 5 ms)`, plus a multiple of measured jitter ([LY crates/replication/interpolation/src/timeline.rs#L48-L74](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/replication/interpolation/src/timeline.rs#L48-L74)). With the examples' 100 ms send interval, interpolated entities are shown **at least 120 ms in the past**.
- **What a longer send interval trades.** "A longer interval means the Send systems run less often, which saves both bandwidth and server CPU" ([LY bandwidth_management.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/bandwidth_management.md)).
- **Avian substeps.** Avian runs 6 substeps per tick by default. This is covered in the earlier doc.

**[Synthesis]** 10 Hz is a demo value, not a competitive one. With a 30 Hz send rate the interpolation floor drops to about 40 ms; with 20 Hz it is about 60 ms. Hitscan against interpolated targets needs server rewind either way (§4). Keep the simulation at 64 Hz and tune the send rate between 20 and 30 Hz in the prototype.

### 2.2 Cost of one replicated rigid body (derived)

**Facts:**
- **What gets replicated.** `lightyear_avian3d` in `Position` mode replicates and predicts `Position`, `Rotation`, `LinearVelocity` and `AngularVelocity` on every entity with a `RigidBody` ([LY crates/integration/avian/src/plugin.rs#L161-L169](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/plugin.rs#L161-L169)).
- **Encoding.** Replicon serializes with postcard ([LY Cargo.toml](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/Cargo.toml)). It packs entity IDs as varints ([RP src/compact_entity.rs](https://github.com/simgine/bevy_replicon/blob/v0.44.1/src/compact_entity.rs)).
- **Resending.** "By default, all unacknowledged mutations are sent every tick" ([RP src/lib.rs#L576](https://github.com/simgine/bevy_replicon/blob/v0.44.1/src/lib.rs#L576)). Lost mutation messages are resent "using the last up-to-date mutations" ([RP mutations.rs](https://github.com/simgine/bevy_replicon/blob/v0.44.1/src/server/replication_messages/mutations.rs)).
- **Packet size.** Netcode caps a packet at **1200 bytes** (`MAX_PACKET_SIZE`) ([LY netcode lib.rs#L125](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/connection/netcode/src/lib.rs#L125)). Issue [#1489](https://github.com/cBournhonesque/lightyear/issues/1489) was a crash when a server with many entities overflowed this cap. It is now closed.
- **Smaller encodings.** You can register a component to replicate as a proxy type (`replicate_as::<T>()`) or as a diff (`replicate_diff`) ([LY registry/replication.rs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/replication/replication/src/registry/replication.rs#L196-L293)). That is where quantization would go. lightyear does not quantize anything by default.

**[Guess] Bytes per update:**
- In avian3d's default `f32` build, `Position` (Vec3) is 12 B, `Rotation` (Quat) is 16 B and each velocity is 12 B. That is **52 B of payload**.
- Add a varint entity ID, size and component headers, and the cost is **about 55–65 B per moving body per send**, before packet headers.
- Quantizing to 16-bit positions and velocities and a 32-bit smallest-three rotation would bring that to about **25–30 B**.
- An interpolated-only unit that drops its velocities would cost about 35 B. The trade-off is that it loses Hermite interpolation ([plugin.rs module docs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/plugin.rs)).

### 2.3 The resting-crate trap (facts → synthesis)

**Facts:**
- **Sleeping is off in the carry example.** `timeline_switch` disables Avian's `IslandPlugin` and `IslandSleepingPlugin` with the comment "disable Sleeping plugin as it can mess up physics rollbacks" ([LY examples/timeline_switch/src/shared.rs#L480-L489](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/shared.rs#L480-L489)).
- **Avian's view.** Avian's rollback tracking issue warns that sleeping "will need to be handled with care -- if the simulation is not large, I'd recommend disabling it" ([avian#478](https://github.com/avianphysics/avian/issues/478)).
- **What Avian writes each tick.** Avian's `writeback_solver_bodies` writes `Position`, `Rotation` and both velocities, through mutable access, for **every body that has a `SolverBody`**, every step ([AV src/dynamics/solver/solver_body/plugin.rs#L263-L290](https://github.com/avianphysics/avian/blob/v0.7.0/src/dynamics/solver/solver_body/plugin.rs#L263-L290)). Adding `Sleeping` or `RigidBodyDisabled` removes the `SolverBody` ([same file, L95](https://github.com/avianphysics/avian/blob/v0.7.0/src/dynamics/solver/solver_body/plugin.rs#L95)).
- **How replicon detects changes.** Replicon finds changes through Bevy change detection ([RP src/lib.rs#L225-L228](https://github.com/simgine/bevy_replicon/blob/v0.44.1/src/lib.rs#L225-L228)).

**[Synthesis]** With sleeping off, every dynamic crate counts as changed every tick and is re-sent every send, even when it sits still.

- A 2×2×2 pad holds 8 crates ([team-buddies-core-loops.md](team-buddies-core-loops.md)). With three or four teams plus loose crates, there can be 30–60 resting crates. Left as awake dynamic bodies, they would cost more than all the units together.
- Mitigations, roughly in order of preference:
  1. **Lock a crate when it lands on the pad.** Make it `Static` or kinematic, or merge the stack into one compound entity. That fits the "stack becomes a build" fantasy anyway.
  2. **Keep sleeping on in the server's world only.** The server never rolls back. Clients would still disable sleeping for predicted bodies. **[Guess]** This is untested, so measure it.
  3. **Give crates a low `ReplicatePriority`** under a per-link bandwidth cap ([bandwidth_management.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/bandwidth_management.md)).

### 2.4 Bandwidth table **[Guess]**

The table uses 60 B per moving body per send, down to each client, without packet headers. "Moving bodies" counts players, AI units and crates in flight. Resting crates count only if they are awake (see §2.3). Projectiles are sent as events (§4) and are not counted.

| Scenario | Moving bodies | 20 Hz send | 30 Hz send | Server upstream, 30 Hz |
|---|---|---|---|---|
| 3v3, one character each, about 12 crates in motion | 18 | 22 KB/s (0.17 Mbit/s) | 32 KB/s (0.26 Mbit/s) | ×6 clients ≈ 1.6 Mbit/s |
| 4v4, one character each, about 16 crates in motion | 24 | 29 KB/s (0.23 Mbit/s) | 43 KB/s (0.35 Mbit/s) | ×8 ≈ 2.8 Mbit/s |
| 4v4 hybrid, each player plus 3 AI buddies (32 units), about 16 crates | 48 | 58 KB/s (0.46 Mbit/s) | 86 KB/s (0.69 Mbit/s) | ×8 ≈ 5.5 Mbit/s |
| Same, but 40 pad and idle crates left awake | 88 | 106 KB/s (0.84 Mbit/s) | 158 KB/s (1.27 Mbit/s) | ×8 ≈ 10 Mbit/s |

- **Packets.** At 30 Hz, 48 bodies make about 2.9 KB per send. That is three 1200-byte packets each send, before the packet headers, which I have not measured.
- **[Synthesis]** Every row is affordable for native PC players on broadband. Bandwidth does **not** force a small squad. It does force the §2.3 rule: resting crates must stop replicating.

### 2.5 CPU: server vs. client prediction

**Facts:**
- **What a rollback replays.**
  - It restores predicted components to the confirmed tick. Then it calls `world.run_schedule(FixedMain)` once for each tick back to the present ([LY crates/replication/prediction/src/rollback.rs#L1236-L1276](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/replication/prediction/src/rollback.rs#L1236-L1276)).
  - So it replays **every fixed system and the whole Avian step**, not only the mispredicted entity. Only entities marked `DisableRollback` are left out of a replay (same file, L275).
- **Rollback depth.**
  - `RollbackPolicy::max_rollback_ticks` defaults to **20**. A non-zero `maximum_predicted_ticks` caps it further. Rollbacks deeper than the cap are aborted ([LY manager.rs#L38-L72](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/replication/prediction/src/manager.rs#L38-L72)).
  - The default `InputTimelineConfig` is `no_input_delay()`: prediction covers all of the latency, up to 100 ticks.
  - The `balanced()` preset covers the first 3 ticks with input delay and predicts at most 7 ticks ([LY crates/core/sync/src/timeline/input.rs#L239-L351](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/core/sync/src/timeline/input.rs#L239-L351)).
- **What triggers a rollback.**
  - With the default `Check` mode, a rollback happens when any predicted component differs from the server beyond its tolerance, or when a remote input differs from the guess.
  - `Always` mode "is also a good stress test: if your game can't handle the CPU load of constant rollbacks, you'll find out fast" ([LY prediction.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/prediction.md)).
  - The Avian tolerances are 0.01 m for position and 0.01 rad for rotation, both "small default rollback tolerances". Velocities use 0.15, matching Avian's sleep thresholds ([plugin.rs#L207-L240](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/plugin.rs#L207-L240)).
- **Published physics timings.** The only published Avian timing is a sample in the benchmark README: "Large Pyramid 2D", about 5,050 boxes, 12.3 ms per step on 1 thread and 8.9 ms on 6 ([AV benches/README.md](https://github.com/avianphysics/avian/blob/v0.7.0/benches/README.md)). The README calls this what the output "might look like". It is 2D and an illustration, not a benchmark claim.

**[Guess] What this means for a match:**
- **Server.** It steps physics once per tick and never rolls back. 100 bodies at 64 Hz should cost a few percent of one core. About 200 CCU in 6–8-player matches is about 25–35 concurrent matches across all regions. That is a handful of small instances per region.
- **Client.** Each rollback replays the whole fixed schedule. Roughly:
  `client fixed-step cost × (1 + rollback ticks per frame)`.
  - At 64 Hz and 80 ms RTT with no input delay, a rollback replays about 5–6 ticks.
  - If remote players are predicted, input rollbacks can happen on almost every update.
  - With about 100 bodies in the client's physics world, I expect each replay to cost 1–4 ms. Measure it with `RollbackMode::Always`.
- **[Synthesis] Consequence:** any AI or game-rule system placed in the shared `FixedUpdate` also runs on clients and is replayed in every rollback. Gate server-only systems (AI brains, crate spawning, build resolution) to the server.

### 2.6 Recommended prototype budget **[Synthesis / Guess]**

These are planning numbers for the squad-model prototype ([#10](https://github.com/ALLiDoizCode/big_back/issues/10)). They are not limits measured by any source.

- **Networked characters:** up to **32 per match**. That covers 4v4 with each player plus 3 AI buddies, or 4 teams of 1 player plus 3 buddies. Only **your own character** and things you are physically touching are predicted.
- **Crates in motion at once** (thrown, falling, carried): about **20–30**.
- **Crates at rest** (on the pad or idle): **unbounded**, as long as they are static, kinematic or asleep on the server (§2.3). If they are awake dynamic bodies, the budget shrinks fast.
- **Predicted set per client:** own character, carried crate, and crates or characters within about 3–5 m (the `timeline_switch` radii). **Keep it under about 10 bodies** so replays stay cheap.
- **Projectiles:** sent as fire data or a shot buffer, not as replicated entities (§4). Then the projectile count barely affects bandwidth.
- **Ticks:** 64 Hz simulation with a 20–30 Hz send rate.

---

## 3. Carried and stacked crates under prediction; AI units

### 3.1 How `timeline_switch` handles carrying (facts)

All of these are in [LY examples/timeline_switch/src/](https://github.com/cBournhonesque/lightyear/tree/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src).

- **Pickup is server-authoritative.**
  - The server's `handle_carry_toggle` grabs the nearest free block within `PICKUP_RADIUS = 2.0` m. It inserts `CarriedBy { holder }` plus `ControlledBy` pointing at the holder's link, so only the carrier's client gets `Controlled`.
  - Dropping waits on a `DROP_GRAB_COOLDOWN_SECS = 1.5` before the block can be grabbed again ([server.rs#L23-L130](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/server.rs)).
  - The client does not predict the pickup.
- **The carry pose is shared.**
  - `apply_carry_fixed` runs in `FixedUpdate` on the server and on the carrier's predicted copy. It teleports a carried cube to `holder + CARRY_OFFSET (0, 1.6, 0)` with zero velocity and switches it to `RigidBody::Kinematic`.
  - It runs in the fixed tick "so it is part of the simulated tick: a predicted block replays through rollback".
  - The body swap happens "only on a transition: re-inserting it every tick destroys and recreates the body (and its contacts), which panics the solver during rollback replay" ([shared.rs#L238-L295](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/shared.rs#L238-L295)).
  - Spheres stay dynamic and follow on a velocity "leash", `carry_spring_velocity`.
- **Other players' carried crates.** On other clients, a carried crate is re-posed in `Update` from the holder's *local* pose, after interpolation runs. Otherwise "a block posed purely from them trails a locally predicted holder by roughly delay × speed". It is also frozen kinematic so the physics step cannot push it ([shared.rs#L297-L380](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/shared.rs#L297-L380)).
- **Switch policy, on the client.**
  - A crate switches to predicted within `SWITCH_RADIUS = 3.0` m of the local player and back to interpolated beyond `RELEASE_RADIUS = 5.0` m (hysteresis).
  - Blends last `SWITCH_BLEND_SECS = 1.0`.
  - Newly replicated entities cannot switch for `SWITCH_SETTLE_SECS = 2.0`, because "Switching one inside that window corrupts its physics".
  - A crate someone else carries follows its holder's timeline ([client.rs#L17-L228](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/client.rs#L17-L228)).
  - Characters spawn predicted on every client (`PredictionTarget::to_clients(All)`), with `rebroadcast_inputs: true` ([server.rs#L216](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/server.rs#L216), [protocol.rs#L58](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/timeline_switch/src/protocol.rs#L58)).
- **How the switch works.**
  - A request to switch to predicted swaps the markers in `PreUpdate`, then "asks for a forced rollback, so that in this same frame the world rewinds … and replays under the new marker".
  - The blend reuses the visual-correction error, which halves every 200 ms by default ([LY crates/replication/prediction/src/switch.rs#L25-L120](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/replication/prediction/src/switch.rs#L25-L120)).

### 3.2 Stacks and rollback (facts)

- **Not every physics state is rolled back by default.**
  - `LightyearAvianPlugin::rollback_resources` defaults to `false`.
  - Setting it to `true` rolls back Avian's contact and constraint graphs, "including cached impulses" (warm starting), plus the collider tree, the clocks and island state.
  - The doc warns that the collider tree "is cloned into prediction history every tick, which can still be expensive for very large scenes". It recommends the option for deterministic or input-only replication ([plugin.rs#L170-L193](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/plugin.rs#L170-L193)).
- **Warm starting is what keeps stacks stable.** Avian's maintainer: contact caching "especially makes stacking scenarios converge on a stable solution much more effectively", but makes rollback "trickier and more expensive". A cache-free solver is still an open idea ([avian#734](https://github.com/avianphysics/avian/issues/734)).
- **Stacked blocks caused the rollbacks.** In an earlier avian3d rollback bug, the maintainer found "the rollbacks are happening on the block characters, not on the controllable character! When I remove the blocks completely, there are no rollbacks". He traced it to Avian resources that were not rolled back, and fixed it by "adding some leeway on the rollback checks" ([lightyear#795](https://github.com/cBournhonesque/lightyear/issues/795)).

**[Synthesis]**
- **Predicting a resting stack makes rollbacks the default state.** The client re-solves the stack without the server's contact cache, drifts past the 1 cm tolerance, and triggers rollbacks. Each rollback replays the whole world.
- **What to do instead:**
  - **Never predict a crate that sits on the pad.** Keep pad stacks interpolated or static, even when the local player is close.
  - **Predict only:** your own carried crate (kinematic, so it is trivially exact), crates you are about to grab, and thrown crates near you.
  - **Make stealing a server decision,** like pickup in the example. The client can play a grab animation at once. Ownership is confirmed about one RTT later.
- **Design consequence:** grab and steal show about 1 RTT of delay before the crate is in your hands. The game feel needs an animation lead-in, such as a reach or lunge, to hide it. That belongs in the core-feel and crate-economy prototypes.

### 3.3 Are AI units run by the server cheap to interpolate? (facts → synthesis)

**Facts:**
- An interpolated entity keeps a `ConfirmedHistory<C>`, and "Every frame the live value is re-sampled from that buffer, slightly in the past". Interpolation runs in `Update` ([LY interpolation.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/interpolation.md)).
- Position mode uses Hermite interpolation that takes velocity into account when all four Avian components are present ([plugin.rs docs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/plugin.rs)).
- The integration warns against putting `RigidBody` on interpolated entities ("you don't want any avian systems to run for `Interpolated` entities"). `timeline_switch` instead pins interpolated carried bodies to kinematic with zero velocity.

**[Synthesis]**
- **Yes, they are cheap.** On the client, an interpolated AI unit costs one history sample and blend per frame, and it never takes part in rollback. Its real cost is the bandwidth it uses (§2.4).
- **What interpolation does to gameplay:**
  - Clients see AI buddies about 1.2 send intervals plus half an RTT in the past. Your predicted character can bump into where a buddy *was*. The server corrects this, which can cause a small rollback on your character.
  - Shooting a buddy uses server rewind (§4).
- **Predicting AI units is unattractive.** AI units have no inputs to rebroadcast. A predicted AI unit only dead-reckons its last velocity, and it mispredicts whenever its AI changes its mind. Keep AI units **always interpolated**, give them kinematic proxy colliders on clients, and let the server own their physics.

---

## 4. Lag compensation for projectiles and hitscan

### Facts

- **What the README promises.** lightyear advertises lag compensation "so that predicted entities can interact with interpolated entities" ([README](https://github.com/cBournhonesque/lightyear#features)).
- **How the Avian implementation works:**
  - `LagCompensationPlugin` runs on the server. It keeps a `LagCompensationHistory` of `(Position, Rotation, ColliderAabb)` for each tracked entity, plus a broad-phase AABB envelope.
  - `max_collider_history_ticks` defaults to **35**, about 550 ms at 64 Hz. The doc comment says "Around 300ms should be enough for most cases" ([LY lag_compensation/history.rs#L24-L42](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/lag_compensation/history.rs#L24-L42)).
- **Ray casts only.** The query API `LagCompensationSpatialQuery` offers only `cast_ray`, `cast_ray_with_sample`, `cast_ray_predicate` and `cast_ray_predicate_with_sample`, plus `sample_collider`. There is **no shape-cast, sweep or overlap query** ([query.rs#L90-L171](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/integration/avian/src/lag_compensation/query.rs#L90-L171)).
- **Where the rewind time comes from.** It comes from the client's `InterpolationDelay`, which "the server also uses … as an estimate for lag compensation" ([interpolation.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/interpolation.md)).
- **The projectiles demo** is a 2D test bed with four independent settings ([LY demos/projectiles/README.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/demos/projectiles/README.md)):
  - **Trajectory:** hitscan or linear.
  - **Representation:**
    - *State entity*: replicates the projectile's position all the time.
    - *Fire-data entity*: sends only the firing tick, origin, direction and trajectory, and each peer fast-forwards the projectile. "requires a deterministic trajectory".
    - *Shot buffer*: a fixed 32-slot ring on the player; "Replicon diffs send only the changed slot".
  - **Hit policy:**
    - *Server current*: simple and cheat-resistant, but the client's interpolated target is older than what the server tests.
    - *Server rewound*: uses lag compensation, "at additional server CPU and history cost".
    - *Client reported*: "intentionally trusts the client and is therefore insecure".
  - **Timeline:** owner-predicted (described as "the usual responsive-client setup and is the main case for server rewind"), all predicted, or all interpolated.
- **Known limits of server rewind,** in its own words: it "Gives the shooter an intentional latency-related advantage over the target and therefore needs a bounded maximum rewind". It also says "The current Lightyear API uses the latest input interpolation delay. A future refinement should attach an exact view timestamp to each shot" ([server_rewound.rs#L1-L20](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/demos/projectiles/src/hit_detection/server_rewound.rs#L1-L20)).
- **3D status.** The `fps` example and the projectiles demo are both 2D. The 3D lag-compensation code exists behind the `3d` feature in the same module, but no 3D example uses it.

### [Synthesis] What this means for the weapons

- **Hitscan** (rifles, lasers): server rewind with `cast_ray`, with the rewind capped at about 150–200 ms so high-ping shooters do not get unbounded favour. This is the supported path.
- **Slow, visible projectiles** (rockets, grenades, thrown crates):
  - Use fire data or a shot buffer, and simulate the projectile on the server against **current** state. Players dodge what they see, and a rocket is visible for long enough that rewind matters less.
  - Lag-compensated shape sweeps for fat projectiles would need custom code: test the rewound `LagCompensationHistory` poses yourself, or approximate with several rays.
  - Explosions and area damage should resolve on the server's current state.
- **Thrown crates as weapons:** these are physics bodies, so they stay on the replicated-physics path. Predict them only when they are near the thrower.
- **Avoid hit policies the client decides.** That is the "Client reported" policy above, and the demo itself calls it insecure. It is wrong for ranked play.

---

## 5. Rejoin and reconnect

### Facts

- **Stable identity.** A `Connection` carries a stable `PeerId` (`LocalId` and `RemoteId`). "a client could get disconnected and reconnect with a different Link, but still have the same PeerId" ([LY book/…/build_client_server.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/tutorial/build_client_server.md)). Netcode and Steam provide this persistent identity ([LY book/concepts/title.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/title.md)).
- **Entities can outlive the disconnect.**
  - `ControlledBy { owner, lifetime }`: when the link disconnects, `Lifetime::SessionBased` despawns the controlled entities and `Lifetime::Persistent` keeps them.
  - `ControlledBy` is an immutable relationship to the link entity ([LY crates/replication/replication/src/control.rs#L42-L100](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/replication/replication/src/control.rs#L42-L100)).
  - **[Synthesis]** A new link means re-inserting `ControlledBy` with the new owner, matched by `RemoteId`.
- **Late join.**
  - Replicon replicates the visible world to each client.
  - Visibility can be per pair, or by rooms ([interest_management.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/interest_management.md)).
  - The time-sync controller has a reset "used when a client reconnects" ([LY sync.rs#L150](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/core/sync/src/timeline/sync.rs#L150)).
  - Deterministic (input-only) late join is a separate, still-open topic ([#1661](https://github.com/cBournhonesque/lightyear/issues/1661)). It does not apply to this state-replicated design.
- **Timeouts.** Netcode uses `CONNECTION_TIMEOUT_SEC = 15` and `TOKEN_EXPIRE_SEC = 30` ([LY netcode lib.rs#L115](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/connection/netcode/src/lib.rs#L115), [token.rs#L21](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/crates/connection/netcode/src/token.rs#L21)). Tokens are issued by your backend (§1).
- **Past reconnect bugs.**
  - A cluster of "Multiple errors when disconnecting/reconnecting" ([#1159](https://github.com/cBournhonesque/lightyear/issues/1159), closed 2025-12) included two problems. Packets from the previous session were dropped by netcode's replay protection. And there was possible undefined behaviour when an observer reconnects from inside `apply_world`.
  - The deterministic example warns that leaving the old player alive and spawning another on reconnect "creates duplicate `PlayerId(remote)` entities" ([examples/deterministic_replication/src/server.rs#L161-L166](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/deterministic_replication/src/server.rs#L161-L166)).

### [Synthesis] What this means for match rules

- **Rejoin is feasible, but it is game code.**
  1. Spawn each player's units with `Lifetime::Persistent`.
  2. On disconnect, keep the player's slot (`PeerId` → team, units) and hand the units to bot control.
  3. On reconnect, the matchmaker issues a new token. The server matches the `RemoteId` and re-inserts `ControlledBy` on the new link. The client receives the full world through normal replication.
- **Never trigger a reconnect from an observer during receive.** That is the #1159 hazard.
- **Leaver policy.** Bots taking over a disconnected player's squad, which is the same machinery as AI buddies, gives a natural leaver rule. That matters at about 200 CCU, where cancelling matches is costly.

---

## 6. Implications for the decisions this feeds

**[Synthesis]** This whole section is my reasoning.

1. **Squad model (#10): the netcode does not veto the hybrid model.** Up to about 32 interpolated, server-run characters per match fits a native PC budget, under 1 Mbit/s per client at 30 Hz. The binding constraints are elsewhere:
   - the *predicted* set: own character, carried crate, nearby loose crates;
   - the resting-crate rule.
   AI buddies should always be interpolated.
2. **Crate economy: pad stacks must "lock".** A crate placed on the pad should stop being an awake dynamic body: make it static or kinematic, or merge it into one build entity. This also gives clean, server-resolved theft. Pulling a crate off the pad turns it back into a dynamic crate owned by the thief. Theft should be decided by the server, with an animation that hides about 1 RTT.
3. **Server model: dedicated, one match per process, for ranked.** Host-client is only for private lobbies. At about 200 CCU, this is roughly 25–35 concurrent matches worldwide **[Guess]**.
4. **Tick and send rates:** 64 Hz simulation and a 20–30 Hz send rate, not the examples' 10 Hz. Lag compensation with a capped rewind is needed for hitscan, either way.
5. **Weapons should favour hitscan and slow, visible projectiles.** Rays are lag-compensated for free. Fat or swept projectiles need custom rewind code. Projectiles should be sent as fire data or a shot buffer, not as entities.
6. **Leavers and rejoin:** persistent units, bot takeover and token-based rejoin are all possible with the existing building blocks. This can inform the matchmaking ticket's leaver policy.

## 7. Measure in the prototype

Every **[Guess]** above should become a measured number. The cheapest way is a headless lightyear client-server pair over crossbeam with a link conditioner (the `RecvLinkConditioner` used in [examples/common](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/examples/common/src/client.rs)). Measure:

- bytes per send for 18, 48 and 88 bodies;
- client replay time with `RollbackMode::Always` at 80 and 150 ms RTT;
- server step time at 64 Hz;
- whether server-only sleeping is safe.

## 8. Open risks found

- **Visual-correction regression.** [lightyear#1780](https://github.com/cBournhonesque/lightyear/issues/1780) (open): visual correction keeps growing with no rollback, so the rendered entity trails its prediction. The reporter says it is a regression on the Bevy 0.20 branch, not on 0.19 `main`. Watch it before upgrading.
- **No published numbers.** Neither lightyear nor Avian publishes bandwidth or CPU numbers for scenes this size. lightyear's bandwidth bench measures 1,000 single-`f32` entities only ([benches/criterion/benches/bandwidth/replication.rs](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/benches/criterion/benches/bandwidth/replication.rs)), and it publishes no results.
- **Prespawned entities can leak.** A prespawned entity that the server never confirms is "not handled currently" for timeout cleanup ([prediction.md](https://github.com/cBournhonesque/lightyear/blob/8abe00453b5272300db73c4f81999bfc1156e491/book/src/concepts/advanced_replication/prediction.md)). Matters if crates or projectiles are prespawned.
