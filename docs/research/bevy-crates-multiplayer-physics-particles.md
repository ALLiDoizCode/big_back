# Bevy crates: multiplayer, physics and particles

Research for a Team Buddies-style 3D squad game: teams of up to 4 units, 1–4 players, crates you carry and stack with physics, projectiles, explosions, vehicles and a lot of particles. Multiplayer is real-time online, and split-screen or local co-op is a possibility.

- **Research date:** 2026-10-18.
- **Newest Bevy:** **0.20.0**, released 2026-10-08 ([crates.io](https://crates.io/crates/bevy/0.20.0), [release notes](https://bevy.org/news/bevy-0-20/)). The previous line was 0.19.1, released 2026-08-13 ([crates.io](https://crates.io/crates/bevy/versions)).

**How to read this doc**
- Plain statements are sourced facts. Each one links to its source: crates.io, docs.rs, the crate's GitHub repo (README, CHANGELOG, source, issues or PRs), or the Bevy docs.
- Text marked **[Synthesis]** is my own reasoning or recommendation. No source states it.
- Text marked **[Lower confidence]** comes from a non-primary source.
- The "open issues" counts come from GitHub's `open_issues_count`, which counts **open issues and open PRs together**.
- Every version and date comes from the crates.io API and was checked on 2026-10-18.

---

## 1. Recommendation summary

**[Synthesis]** This whole section is my recommendation. The facts behind it are cited in sections 2–4.

| Category | Pick | One-line reason |
|---|---|---|
| Netcode | **lightyear 0.30** | It is the only crate here with server-authoritative replication, client prediction with rollback, snapshot interpolation, lag compensation, WebTransport, WebSocket and Steam transports, and a maintained Avian integration. Its `timeline_switch` example is a 3D Avian demo of carrying physics objects. |
| Physics | **avian3d 0.7** | It is ECS-native and is the physics engine lightyear integrates with. Its solver is TGS Soft, with substeps, warm starting and relaxation, and it has an `enhanced-determinism` feature. Fallback: **bevy_rapier3d 0.37**, which already supports Bevy 0.20 and has a built-in ray-cast vehicle controller. |
| Particles, GPU | **bevy_hanabi 0.19** | It is the most mature GPU particle crate. It supports burst and command-driven spawning, trails, ribbons and split-screen viewports. It has no world collision, and on the web it needs WebGPU. |
| Particles, CPU / WebGL2 | **bevy_firework 0.10** | It runs on the CPU in 3D, renders in batches, collides with Avian colliders, works with WASM and WebGL, and supports nested emission, for example smoke trailing behind debris. |

### Combinations that work together

- **lightyear + avian3d.** This is the proven pairing. Lightyear has first-party integration crates, `lightyear_avian3d` and `lightyear_avian2d`, which cover replication, prediction, rollback, frame interpolation and lag compensation ([lightyear_avian lib.rs](https://github.com/cBournhonesque/lightyear/blob/main/crates/integration/avian/src/lib.rs)). Lightyear 0.30.1 pins `avian` 0.7 and `bevy_replicon` 0.44 ([workspace Cargo.toml](https://github.com/cBournhonesque/lightyear/blob/main/Cargo.toml)).
- **lightyear + bevy_rapier3d.** Lightyear has no integration crate for Rapier. Its integration directory holds only the Avian crates ([crates/integration](https://github.com/cBournhonesque/lightyear/tree/main/crates/integration)).
- **bevy_replicon + avian or rapier.** Replicon only replicates state. Prediction, interpolation and rollback have to come from a third-party crate. Its README lists `bevy_rewind`, plus several crates it marks unmaintained ([replicon README](https://github.com/simgine/bevy_replicon#interpolation-andor-rollback)).
- **bevy_ggrs + matchbox + a deterministic physics engine.** This is the classic P2P rollback stack for the browser ([bevy_ggrs README](https://github.com/gschup/bevy_ggrs#community)). Neither Avian nor Rapier ships a GGRS integration. You register the physics components for snapshots yourself ([bevy_ggrs quickstart](https://github.com/gschup/bevy_ggrs#quickstart)).
- **Character controller.** Use bevy-tnua with Avian or Rapier ([tnua README](https://github.com/idanarye/bevy-tnua)), or `bevy_ahoy`, a kinematic character controller for Avian that pulls in `avian_pickup`. `avian_pickup` picks up rigid bodies in the style of Half-Life 2's gravity gun ([crates.io](https://crates.io/crates/avian_pickup)) and may be useful for carrying crates.
- **Particles and physics.** bevy_firework collides with Avian colliders. Hanabi and Sprinkles do not use physics-engine colliders.

### Is rollback possible?

**[Synthesis]** Yes, in two ways. Neither is free.

1. **Server-authoritative with client-side prediction and rollback (recommended).** Lightyear predicts locally controlled entities and rolls them back when server state arrives. Its `timeline_switch` example predicts the physics blocks near the local player, and interpolates distant players and blocks carried by others. It moves entities between the two timelines at runtime ([examples README](https://github.com/cBournhonesque/lightyear/blob/main/examples/README.md), [timeline_switch README](https://github.com/cBournhonesque/lightyear/blob/main/examples/timeline_switch/README.md)).
   - This fits a crate-carrying game with up to 16 units: you predict only what is near the player, not the whole world.
   - It does not need cross-platform determinism, because the server corrects any drift.
2. **Full deterministic rollback or lockstep, with GGRS or lightyear's input-only deterministic mode.** This needs bit-identical physics on every peer.
   - Avian and Rapier both ship an `enhanced-determinism` feature for this.
   - Avian's cross-platform determinism test in CI is **2D only** ([src/tests/mod.rs](https://github.com/avianphysics/avian/blob/main/src/tests/mod.rs)).
   - Rapier documents the exact conditions it needs ([determinism guide](https://github.com/dimforge/rapier/blob/master/website/docs/user_guides/templates/determinism.mdx)).
   - Lightyear's deterministic examples use Avian 2D ([deterministic_replication Cargo.toml](https://github.com/cBournhonesque/lightyear/blob/main/examples/deterministic_replication/Cargo.toml)).
   - **[Synthesis]** For a 3D world with many stacked rigid bodies, snapshotting and re-simulating up to 8 frames ([GGRS default prediction window](https://github.com/gschup/ggrs/blob/main/docs/sessions.md)) is expensive, and nobody has published evidence that it works in 3D. Treat it as a research spike, not the default.

### Split-screen and local co-op

- **GGRS** supports several local players in one session (`PlayerType::Local` per handle) ([sessions.md](https://github.com/gschup/ggrs/blob/main/docs/sessions.md#multiple-local-players)).
- **Hanabi** lists "Multiple viewports (split screen)" as supported ([README feature list](https://github.com/djeedai/bevy_hanabi#feature-list)).
- **Lightyear** keeps input state on each entity: `InputMarker` "identify[ies] the ActionState that the player is actively updating" ([action_state.rs](https://github.com/cBournhonesque/lightyear/blob/main/crates/inputs/inputs_native/src/action_state.rs)). I found no documented feature for several local players on one client.
  - **[Synthesis]** One client should be able to drive several input-marked entities, one per local player. Test this early. If it fails, the fallback is a composite input type that carries every local player's actions.
- **[Synthesis]** In any setup, local co-op multiplies the number of predicted entities on one machine. That adds rollback cost per client, not cost on the server.

### Version reality check (Bevy 0.20 is 10 days old)

**[Synthesis]** The recommended stack (lightyear + avian3d + hanabi + firework) **does not build on Bevy 0.20 today.** Start the project on **Bevy 0.19.1**, which every pick supports, and plan one upgrade once the following land:
- Lightyear's Bevy 0.20 PR ([#1779](https://github.com/cBournhonesque/lightyear/pull/1779), open);
- Avian's Bevy 0.20 release. The maintainer promised it "this weekend" on 2026-10-14 ([avian#1082](https://github.com/avianphysics/avian/issues/1082)); PR [#1070](https://github.com/avianphysics/avian/pull/1070) is still open.
- A Hanabi release. `main` has already migrated ([#576](https://github.com/djeedai/bevy_hanabi/pull/576), merged 2026-10-15).

---

## 2. Comparison tables

### 2.1 Networking

| Crate (latest) | Released | Bevy | License | Model | Prediction / interpolation / lag comp | Transports | WASM | Physics integration | Activity (repo) |
|---|---|---|---|---|---|---|---|---|---|
| [lightyear 0.30.1](https://crates.io/crates/lightyear) | 2026-09-16 | 0.19 ([table](https://github.com/cBournhonesque/lightyear#supported-bevy-version)) | MIT OR Apache-2.0 | Server-authoritative, host-client and P2P. Optional deterministic input-only mode, for lockstep or rollback | Built in: prediction with rollback, snapshot interpolation, input delay, lag compensation | UDP, WebTransport, WebSocket, Steam, crossbeam | Yes, through WebTransport ([README](https://github.com/cBournhonesque/lightyear)) | **Avian 2D/3D, first-party** | Pushed 2026-10-16. 63 open. 1,190★ |
| [bevy_replicon 0.45.2](https://crates.io/crates/bevy_replicon) | 2026-10-15 | **0.20** ([table](https://github.com/simgine/bevy_replicon#bevy-compatibility)) | MIT OR Apache-2.0 | Server-authoritative replication, with no I/O of its own | None built in. Third-party: [bevy_rewind](https://github.com/NiseVoid/bevy_rewind) 0.0.1 | Comes from the backend: renet, renet2, quinnet, aeronet, matchbox | Depends on the backend | None. Physics state is replicated like any other component | Pushed 2026-10-15. 22 open. 653★ |
| ↳ [bevy_replicon_renet2 1.0.0](https://crates.io/crates/bevy_replicon_renet2) | 2026-10-16 | 0.20 | MIT OR Apache-2.0 | Backend | — | UDP, in-memory, WebTransport, WebSocket, Steam ([renet2 README](https://github.com/UkoeHB/renet2)) | Yes, as a WT or WS client | — | Pushed 2026-10-16. 62★ |
| ↳ [bevy_replicon_renet 0.20.0](https://crates.io/crates/bevy_replicon_renet) | 2026-09-01 | 0.19 (replicon 0.44) | MIT OR Apache-2.0 | Backend | — | renet: UDP netcode, Steam | Not documented | — | — |
| ↳ [bevy_replicon_quinnet 0.21.0](https://crates.io/crates/bevy_replicon_quinnet) | 2026-10-13 | 0.20 | MIT OR Apache-2.0 | Backend | — | QUIC, through quinn | Not documented | — | quinnet pushed 2026-10-13 |
| ↳ [aeronet_replicon 0.21.0](https://crates.io/crates/aeronet_replicon) | 2026-06-24 (0.22.0-rc.1: 2026-09-23) | 0.19 (replicon 0.41) | MIT OR Apache-2.0 | Backend | — | WebSocket, WebTransport, Steam, channels ([aeronet README](https://github.com/aecsocket/aeronet)) | Yes, for WS and WT | — | Pushed 2026-09-23 |
| [naia-bevy-server/client 0.25.0](https://crates.io/crates/naia-bevy-server) | 2026-05-12 | 0.18 (`dev` branch is on 0.19) | MIT OR Apache-2.0 | Server-authoritative. Can delegate authority over an entity to a client | Not built in. Gives you tick-buffered input, and you write the prediction yourself ([README](https://github.com/naia-lib/naia)) | UDP natively, WebRTC in the browser | Yes, as a WebRTC client | None | Pushes to `dev` through 2026-10-17. 18 open |
| [bevy_ggrs 0.22.0](https://crates.io/crates/bevy_ggrs) + [ggrs 0.13.0](https://crates.io/crates/ggrs) | 2026-06-26 | 0.19 (0.20 PR [#135](https://github.com/gschup/bevy_ggrs/pull/135) open) | MIT OR Apache-2.0 | P2P rollback in the GGPO style. Set the prediction window to 0 for lockstep | Rollback over the whole world. No interpolation | Any socket. Matchbox with the `ggrs` feature | Yes, through matchbox | None. You register the physics components for snapshots | bevy_ggrs pushed 2026-07-26 (7 open). ggrs pushed 2026-08-25 (6 open) |
| [bevy_matchbox 0.14.0](https://crates.io/crates/bevy_matchbox) | 2026-02-13 | **0.18** (0.19 port, PR [#557](https://github.com/johanhelsing/matchbox/pull/557), open) | MIT OR Apache-2.0 | Transport only: P2P mesh over WebRTC, with a signaling server | — | WebRTC data channels, reliable or unreliable | Yes | — | Pushed 2026-06-02. 43 open |

### 2.2 Physics

| Crate (latest) | Released | Bevy | License | Solver | Determinism | Character / vehicles / joints | ECS model | Activity |
|---|---|---|---|---|---|---|---|---|
| [avian3d 0.7.0](https://crates.io/crates/avian3d) | 2026-06-20 | 0.19 (0.20: [#1070](https://github.com/avianphysics/avian/pull/1070) open, [#1082](https://github.com/avianphysics/avian/issues/1082)) | MIT OR Apache-2.0 | Impulse-based, with substeps (6 by default), soft constraints, warm starting and relaxation. XPBD joints are optional ([plugin.rs](https://github.com/avianphysics/avian/blob/main/src/dynamics/solver/plugin.rs), [schedule.rs](https://github.com/avianphysics/avian/blob/main/src/dynamics/solver/schedule.rs)) | Locally deterministic. The `enhanced-determinism` feature uses libm math for cross-platform results ([lib.rs](https://github.com/avianphysics/avian/blob/main/src/lib.rs)). Cross-platform hash test is 2D only | Move-and-slide primitive and example KCC and dynamic character controllers. No vehicle controller. Fixed, distance, prismatic, revolute and spherical joints, plus motors | Native ECS, with no separate physics world ([README](https://github.com/avianphysics/avian#design)) | Pushed 2026-10-10. 214 open. 3,238★ |
| [bevy_rapier3d 0.37.0](https://crates.io/crates/bevy_rapier3d) (rapier3d 0.36.1) | 2026-10-10 | **0.20** | **Apache-2.0** only | PGS iterations (4 by default; the docs suggest 8–12 for stacks) with substeps and soft constraints. Optional `block-solver`, off by default in 3D ([integration_parameters.rs](https://github.com/dimforge/rapier/blob/master/src/dynamics/integration_parameters.rs), [Cargo.toml](https://github.com/dimforge/rapier/blob/master/crates/rapier3d/Cargo.toml)) | `enhanced-determinism` gives cross-platform determinism when its documented conditions are met. It is compatible with `parallel` and not with `simd8` ([determinism.mdx](https://github.com/dimforge/rapier/blob/master/website/docs/user_guides/templates/determinism.mdx)) | Built-in `KinematicCharacterController`. **`RayCastVehicleController`** is new in 0.37. Impulse joints, multibodies, PID controllers, soft bodies ([CHANGELOG](https://github.com/dimforge/rapier/blob/master/bindings/bevy_rapier/CHANGELOG.md)) | Bevy plugin over a separate Rapier world | Rapier pushed 2026-10-10. 75 open. The old `dimforge/bevy_rapier` repo is **archived** and the code moved into the rapier repo |

Supporting crates:
- [bevy_tnua 0.33.0](https://crates.io/crates/bevy_tnua): Bevy 0.20, MIT OR Apache-2.0, pushed 2026-10-15. It is a floating character controller.
  - [bevy-tnua-rapier3d 0.19](https://crates.io/crates/bevy-tnua-rapier3d) already supports Bevy 0.20.
  - [bevy-tnua-avian3d 0.12.1](https://crates.io/crates/bevy-tnua-avian3d) is still on Bevy 0.19 and integration layer 0.13, so you must pair it with bevy_tnua **0.32**.
- [bevy_ahoy 0.2.0](https://crates.io/crates/bevy_ahoy): Bevy 0.19, avian3d 0.7.
- [avian_pickup 0.6.0](https://crates.io/crates/avian_pickup): Bevy 0.19, avian3d 0.7.

### 2.3 Particles

| Crate (latest) | Released | Bevy | License | Sim | 3D | WASM / WebGL2 | World collision | Gameplay-event spawning | Maintained? |
|---|---|---|---|---|---|---|---|---|---|
| [bevy_hanabi 0.19.0](https://crates.io/crates/bevy_hanabi) | 2026-06-27 | 0.19 (`main` is on 0.20) | MIT OR Apache-2.0 | GPU compute | Yes | WASM **with WebGPU only**. WebGL2 is not supported ([docs/wasm.md](https://github.com/djeedai/bevy_hanabi/blob/main/docs/wasm.md)) | **No.** Collision is unchecked in the feature list | Yes: one-shot or repeated bursts, `EffectSpawner::reset()`, properties, GPU spawn events and child effects | Yes. Pushed 2026-10-15. 60 open. 1,426★ |
| [bevy_sprinkles 0.3.0](https://crates.io/crates/bevy_sprinkles) | 2026-06-22 | 0.19 (from the crates.io dependency; the README table is stale) | MIT OR Apache-2.0 | GPU compute | Yes (2D is planned) | Not documented. **[Synthesis]** Compute shaders imply WebGPU only | **Yes**, against its own `ParticlesCollider3D` shapes, not physics colliders ([runtime.rs](https://github.com/doceazedo/sprinkles/blob/main/crates/bevy_sprinkles/src/runtime.rs)). Sub-emitters can fire on collision | RON assets with a visual editor | Yes, young. Pushed 2026-10-14. 198★ |
| [bevy_firework 0.10.0](https://crates.io/crates/bevy_firework) | 2026-07-08 | 0.19 | MIT OR Apache-2.0 | CPU, rendered in batches | Yes, billboards with PBR | **Yes, WASM and WebGL** ([README](https://github.com/mbrea-c/bevy_firework)) | **Yes, any Avian collider** | One-shot or continuous emission. Nested emission | Yes, small. Pushed 2026-07-25. 82★ |
| [bevy_enoki 0.8.0](https://crates.io/crates/bevy_enoki) | 2026-10-11 | **0.20** | MIT | CPU (SIMD), GPU-instanced | **No, 2D only** ([README](https://github.com/Lommix/bevy_enoki)) | Yes, "portability is the highest priority" | No | RON effect assets with hot reload | Yes. Pushed 2026-10-12 |
| [bevy_particle_systems 0.13.0](https://crates.io/crates/bevy_particle_systems) | **2024-07-18** | **0.14** | MIT | CPU | **No, 2D** ([README](https://github.com/abnormalbrain/bevy_particle_systems)) | Yes | No | Spawn a component | **No.** Six Bevy releases behind. Last push 2024-12-02 |

---

## 3. Detailed notes

### 3.1 Networking

#### lightyear
- **Facts:**
  - Latest is 0.30.1 (2026-09-16), with 0.28–0.30 on Bevy 0.19 ([README table](https://github.com/cBournhonesque/lightyear#supported-bevy-version)). `main` still pins Bevy 0.19 ([Cargo.toml](https://github.com/cBournhonesque/lightyear/blob/main/Cargo.toml)). The 0.20 bump is open as [#1779](https://github.com/cBournhonesque/lightyear/pull/1779).
  - It releases often: 0.28 came out 2026-06-26, 0.29 on 2026-08-10 and 0.30 on 2026-09-13 ([crates.io versions](https://crates.io/crates/lightyear/versions)).
- **Model:** the README describes it as "a library for writing server-authoritative multiplayer games". It supports client-server, P2P and host-client topologies. It uses `bevy_replicon` for world replication, interest management and pre-spawning ([README](https://github.com/cBournhonesque/lightyear#features)).
- **Prediction and interpolation:** the README lists client-side prediction with rollback, snapshot interpolation, input delay, priority-based bandwidth management and lag compensation "so that predicted entities can interact with interpolated entities" ([README](https://github.com/cBournhonesque/lightyear#features)). It also has deterministic replication, "compatible with both lockstep and prediction/rollback".
- **Inputs:** inputs are buffered every tick and the last few frames are resent in each packet. It integrates with leafwing-input-manager and bevy_enhanced_input ([README](https://github.com/cBournhonesque/lightyear#features)).
- **Transports:** UDP, plus WebSocket, Steam and WebTransport through aeronet. Each one is a crate feature: `udp`, `websocket`, `webtransport` and `steam` ([crates.io features](https://crates.io/crates/lightyear/0.30.1/features)). WASM works through WebTransport.
- **Physics:** `lightyear_avian3d` integrates "replication, prediction, rollback, frame interpolation, and lag compensation with the Avian physics engine". You disable Avian's `PhysicsTransformPlugin` and `PhysicsInterpolationPlugin`. The default `AvianReplicationMode::Position` replicates `Position`, `Rotation`, `LinearVelocity` and `AngularVelocity` ([lib.rs](https://github.com/cBournhonesque/lightyear/blob/main/crates/integration/avian/src/lib.rs)).
- **Relevant examples:** all of these are in the [examples README](https://github.com/cBournhonesque/lightyear/blob/main/examples/README.md) and the [demos README](https://github.com/cBournhonesque/lightyear/blob/main/demos/README.md).
  - `avian_3d`: server-authoritative 3D characters with a predicted dynamic block.
  - `timeline_switch`: a 3D Avian demo where players pick up physics blocks ([Cargo.toml](https://github.com/cBournhonesque/lightyear/blob/main/examples/timeline_switch/Cargo.toml)).
  - `fps`: lag compensation, in 2D.
  - `demos/projectiles`: a test bed for hitscan and linear projectiles, hit policies and timelines.
  - `demos/spaceships`: predicted physics, pre-spawned bullets and deterministic P2P, in 2D.
- **Users named in the README:** [Lumina](https://github.com/nixon-voxell/lumina) and [cycles.io](https://github.com/cBournhonesque/jam5).
- **Open issue to watch:** [#1780](https://github.com/cBournhonesque/lightyear/issues/1780), "Visual correction keeps growing with no rollbacks".

#### bevy_replicon and its backends
- **Facts:**
  - Latest is 0.45.2 (2026-10-15). Version 0.45 supports Bevy 0.20 ([compat table](https://github.com/simgine/bevy_replicon#bevy-compatibility)).
  - It is server-authoritative, with "no builtin I/O". It supports singleplayer, client, dedicated-server and listen-server setups, visibility control, and remote events and triggers ([README](https://github.com/simgine/bevy_replicon#features)).
  - The repo moved to `simgine/bevy_replicon`.
- **Prediction and rollback:** none built in. The only maintained add-on the README lists is `bevy_rewind`, "server-authoritative rollback inspired by Rocket League". Its crate is at 0.0.1 and the repo was last pushed 2026-01-17 ([GitHub](https://github.com/NiseVoid/bevy_rewind)). `bevy_replicon_snap` and `bevy_timewarp` are listed as unmaintained ([README](https://github.com/simgine/bevy_replicon#ecosystem)).
- **Backends on Bevy 0.20 today:**
  - `bevy_replicon_renet2` 1.0.0 supports UDP, in-memory, WebTransport, WebSocket and Steam ([renet2 README](https://github.com/UkoeHB/renet2)).
  - `bevy_replicon_quinnet` 0.21.0 supports QUIC.
- **Backends still on Bevy 0.19:** `bevy_replicon_renet` 0.20.0 and `aeronet_replicon` 0.21.0.
- **[Synthesis]** Use replicon directly if you want Bevy 0.20 right now and are willing to write prediction and interpolation yourself. Lightyear is built on replicon, so the replication concepts carry over.

#### naia
- **Facts:**
  - The Bevy adapters are at 0.25.0 (2026-05-12), on Bevy 0.18. The previous release, 0.24, came out 2024-12-13 ([crates.io](https://crates.io/crates/naia-bevy-server/versions)).
  - The `dev` branch already targets Bevy 0.19 and had commits up to 2026-10-17 ([dev Cargo.toml](https://github.com/naia-lib/naia/blob/dev/adapters/bevy/server/Cargo.toml)).
  - It is server-authoritative and follows the Tribes 2 networking model. It works with any ECS. Transports are UDP natively and WebRTC in the browser.
  - Prediction and interpolation are "not built-in — naia supplies the data; you write the prediction/interpolation logic" ([README](https://github.com/naia-lib/naia)).
- **[Synthesis]** It is two Bevy releases behind and has a slow release cadence. Not recommended.

#### GGRS / bevy_ggrs
- **Facts:**
  - bevy_ggrs 0.22.0 and ggrs 0.13.0 were both released 2026-06-26 and target Bevy 0.19. The Bevy 0.20 PR is [#135](https://github.com/gschup/bevy_ggrs/pull/135), opened 2026-10-10.
  - Its model is P2P rollback in the style of GGPO. Rollback snapshots only the components and resources you register ([README](https://github.com/gschup/bevy_ggrs)).
  - The default prediction window is 8 frames. Setting it to 0 gives "conservative lockstep mode" ([sessions.md](https://github.com/gschup/ggrs/blob/main/docs/sessions.md)).
  - It supports several local players per client ([sessions.md](https://github.com/gschup/ggrs/blob/main/docs/sessions.md#multiple-local-players)).
  - The docs on pitfalls cover non-deterministic query iteration, events, `Local<T>`, entity ID instability, `GlobalTransform` and change detection ([pitfalls.md](https://github.com/gschup/bevy_ggrs/blob/main/docs/pitfalls.md)).
- **Users named in the ggrs README:** Jumpy, Gamercade, NES-bundler, Girls with Swords, NieR Automata: Online, and [Signs of Danger](https://store.steampowered.com/app/3816900/Signs_of_Danger/), which uses a fork ([ggrs README](https://github.com/gschup/ggrs#projects-using-ggrs)).
- **Physics:** there is no built-in integration. **[Synthesis]** You would register all Avian or Rapier body, collider and contact state for rollback, and enable `enhanced-determinism`. Bevy 0.20's new "schedule randomization" debug feature, which randomizes the order of unordered conflicting systems ([release notes](https://bevy.org/news/bevy-0-20/#schedule-randomization)), would help find desyncs.

#### bevy_matchbox
- **Facts:**
  - 0.14.0 was released 2026-02-13 and targets **Bevy 0.18** ([README table](https://github.com/johanhelsing/matchbox)).
  - The Bevy 0.19 port, [#557](https://github.com/johanhelsing/matchbox/pull/557) (opened 2026-08-18), and its follow-ups [#558](https://github.com/johanhelsing/matchbox/pull/558) and [#559](https://github.com/johanhelsing/matchbox/pull/559) are all still open.
  - It provides P2P over WebRTC for native and WASM, with reliable and unreliable channels, and comes with a signaling server. The `ggrs` feature gives a GGRS-compatible socket.
  - There is also a replicon backend, [bevy_replicon_matchbox](https://github.com/Occuros/bevy_replicon_matchbox), which the replicon README lists. I did not check its version.
- **[Synthesis]** It lags two Bevy versions behind, and it is the weakest link in a GGRS stack.

### 3.2 Physics

#### Avian (avian3d)
- **Facts:**
  - 0.7.0 was released 2026-06-20 and is "primarily just an update to Bevy 0.19" ([release](https://github.com/avianphysics/avian/releases/tag/v0.7.0)).
  - Bevy 0.20 support is in PR [#1070](https://github.com/avianphysics/avian/pull/1070), which also moves to Parry 0.31. On 2026-10-14 the maintainer said a 0.20-compatible release would come "this weekend" ([#1082](https://github.com/avianphysics/avian/issues/1082)). As of 2026-10-18, crates.io still shows 0.7.0.
  - The repo has 214 open issues and PRs, and the last push was 2026-10-10.
- **Solver:**
  - "Avian uses an impulse-based solver with substepping and soft constraints. Warm starting is used to improve convergence, along with a relaxation pass to reduce overshooting." Speculative collision is on by default, and swept CCD is optional ([plugin.rs](https://github.com/avianphysics/avian/blob/main/src/dynamics/solver/plugin.rs)).
  - The default is 6 substeps ([schedule.rs](https://github.com/avianphysics/avian/blob/main/src/dynamics/solver/schedule.rs)).
  - The maintainer's Avian 0.1 post names the solver "TGS Soft", modeled on Box2D v3 ([joonaa.dev](https://joonaa.dev/blog/06/avian-0-1)).
  - Joints are solved with XPBD behind the `xpbd_joints` feature.
- **Determinism:**
  - `enhanced-determinism` "enables cross-platform deterministic math" through libm ([lib.rs feature table](https://github.com/avianphysics/avian/blob/main/src/lib.rs), [avian3d Cargo.toml](https://github.com/avianphysics/avian/blob/main/crates/avian3d/Cargo.toml)).
  - The cross-platform test hashes a chaotic **2D** scene on Windows, macOS and Linux. That job only runs on pushes, or on PRs with the `Cross-Platform` label ([determinism_2d.rs](https://github.com/avianphysics/avian/blob/main/src/tests/determinism_2d.rs), [rust.yaml](https://github.com/avianphysics/avian/blob/main/.github/workflows/rust.yaml)).
  - There is a separate test for local determinism.
- **Features:**
  - From 0.6 onward: move-and-slide for kinematic character controllers, joint motors, a BVH broad phase using OBVHS, and faster spatial queries ([v0.6.0 release](https://github.com/avianphysics/avian/releases/tag/v0.6.0)).
  - 3D examples include `kinematic_character_3d`, `dynamic_character_3d`, `move_and_slide_3d`, `joint_motors_3d`, `voxels_3d` and `conveyor_belt` ([examples](https://github.com/avianphysics/avian/tree/main/crates/avian3d/examples)).
  - There is **no vehicle controller and no vehicle example**.
- **ECS:** the README says "Made with Bevy, for Bevy. No wrappers… shouldn't need to maintain a separate physics world" ([README](https://github.com/avianphysics/avian#design)).
- **Benchmarks:** none published in the repo README. The 0.6 post describes the BVH broad phase as a "massive" improvement but gives no comparison table I could cite.

#### Rapier (bevy_rapier3d)
- **Facts:**
  - bevy_rapier3d 0.37.0 (rapier3d 0.36.1) was released 2026-10-10 and already targets **Bevy 0.20**.
  - The plugin moved into the main rapier repo, and `dimforge/bevy_rapier` is **archived** ([old repo](https://github.com/dimforge/bevy_rapier), [CHANGELOG](https://github.com/dimforge/rapier/blob/master/bindings/bevy_rapier/CHANGELOG.md)).
  - The license is **Apache-2.0 only**, not dual-licensed ([crates.io](https://crates.io/crates/bevy_rapier3d)).
- **Solver:**
  - It runs PGS iterations inside substeps, with `num_solver_iterations` defaulting to 4. The docs say: "`8-12`: Use for demanding scenarios (stacks of objects, complex machinery)" ([integration_parameters.rs](https://github.com/dimforge/rapier/blob/master/src/dynamics/integration_parameters.rs)).
  - 0.36 added `additional_pgs_iterations` per body ([CHANGELOG](https://github.com/dimforge/rapier/blob/master/CHANGELOG.md)).
  - The optional `block-solver` is "NOT enabled by default in 3D: it introduces jitter in the 3D domino demo" ([rapier3d Cargo.toml](https://github.com/dimforge/rapier/blob/master/crates/rapier3d/Cargo.toml)).
- **Determinism:**
  - It is locally deterministic by default. `enhanced-determinism` makes it cross-platform deterministic if the platform is IEEE 754-2008 compliant, initial conditions are the same, and you use nalgebra's math functions. The feature works with `parallel` (results don't depend on thread count) but not with `simd8`.
  - In Bevy you also need a fixed timestep, the same spawn order, and Bevy's `libm` feature ([determinism.mdx](https://github.com/dimforge/rapier/blob/master/website/docs/user_guides/templates/determinism.mdx)).
  - **[Synthesis]** These are clearer and more complete determinism docs than Avian has for 3D.
- **Features new in 0.37:** `RayCastVehicleController` (3D), PID and PD controllers, soft bodies, the `PinSlotJoint`, joint impulses, and URDF and MJCF loaders. The `KinematicCharacterController` is still there ([CHANGELOG](https://github.com/dimforge/rapier/blob/master/bindings/bevy_rapier/CHANGELOG.md)).
- **Benchmarks:** the maintainers publish them at [rapier.rs/benchmarks](https://rapier.rs/benchmarks). I am not copying any numbers.
- **Lower confidence:** Avian's maintainer showed bevy_rapier3d 0.33's built-in KCC "jittering and getting stuck" in a video. That is a competitor's opinion about an old version ([joonaa.dev](https://joonaa.dev/blog/12/avian-0-6)).

#### Character controller helpers
- **bevy_tnua 0.33.0** (2026-10-15) is a floating controller with jumps, coyote time, slopes, moving platforms and climbing ([README](https://github.com/idanarye/bevy-tnua)). The Avian 3D integration still lags on Bevy 0.19, as described in section 2.2.
- **bevy_ahoy 0.2.0** is a kinematic character controller for Avian and BEI ([crates.io](https://crates.io/crates/bevy_ahoy)).
- **avian_pickup 0.6.0** picks up dynamic rigid bodies in Avian, modeled on Half-Life 2's gravity gun ([crates.io](https://crates.io/crates/avian_pickup)).

#### Anything newer?
- A crates.io search for "bevy physics" sorted by recent downloads found nothing newer that is production-oriented. The results beyond Avian and Rapier are helper crates (`bevy_transform_interpolation`, `bevy_heavy`) and the unrelated `gizmo-*` engine ([search](https://crates.io/search?q=bevy%20physics&sort=recent-downloads)).

### 3.3 Particles

#### bevy_hanabi (GPU)
- **Facts:**
  - 0.19.0 was released 2026-06-27 for Bevy 0.19.
  - `main` has moved to Bevy 0.20, wgpu 30 and **WESL shaders**, and its version is `0.20.0-dev` ([CHANGELOG Unreleased](https://github.com/djeedai/bevy_hanabi/blob/main/CHANGELOG.md), [#576](https://github.com/djeedai/bevy_hanabi/pull/576)).
  - The README still says "under heavy development… for moderate-size effects, it can already be used" ([README](https://github.com/djeedai/bevy_hanabi)).
- **Features** ([feature list](https://github.com/djeedai/bevy_hanabi#feature-list)):
  - Spawning: constant rate, one-shot burst and repeated burst.
  - Forces: force fields and drag.
  - Rendering: stretching along velocity, trails and ribbons, 3D meshes, render layers, **multiple viewports (split screen)**, HDR and bloom.
  - **Collision: not implemented.** Neither shapes nor the depth buffer are supported.
  - `main` adds depth-texture sampling in expressions and a `KillFrustumModifier`.
- **Gameplay events:** the `spawn_on_command` example spawns particles "manually with `EffectSpawner::reset()`" when a ball hits a wall, using properties to set direction and color ([example](https://github.com/djeedai/bevy_hanabi/blob/main/examples/spawn_on_command.rs)).
- **WASM:** it is supported, but it "**requires using the WebGPU render backend of Bevy instead of the WebGL2 one**", plus the `bevy/webgpu` feature ([docs/wasm.md](https://github.com/djeedai/bevy_hanabi/blob/main/docs/wasm.md)).

#### bevy_sprinkles (GPU, newer)
- **Facts:**
  - 0.3.0 was released 2026-06-22. It depends on `bevy ^0.19` according to the crates.io dependency data, while its README table still says 0.18 for 0.1–0.2 and 0.19 for main.
  - The last commit on `main` is from 2026-09-08 ([repo](https://github.com/doceazedo/sprinkles)).
  - Features: GPU simulation and sorting, a visual editor, 3D only, PBR materials, automatic instancing, **particle collision**, curl noise and sub-emitters. It takes inspiration from Godot's particle system ([README](https://github.com/doceazedo/sprinkles#features)).
  - Collision is against `ParticlesCollider3D` shapes that you place yourself, with rigid or hide-on-contact modes, plus sub-emitters that fire at collision ([runtime.rs](https://github.com/doceazedo/sprinkles/blob/main/crates/bevy_sprinkles/src/runtime.rs), [extract.rs](https://github.com/doceazedo/sprinkles/blob/main/crates/bevy_sprinkles/src/extract.rs)).
- **[Synthesis]** It is promising for debris that bounces off the floor and crates, but it is young and has a single maintainer. Evaluate it after Hanabi.

#### bevy_firework (CPU, 3D)
- **Facts:**
  - 0.10.0 was released 2026-07-08 for Bevy 0.19 and Avian 0.7 ([version table](https://github.com/mbrea-c/bevy_firework#version-table)).
  - It simulates on the CPU and renders each system in one draw call, aiming at "tens of thousands of particles without noticeable framerate drops". The README gives that as the maintainer's claim, with no benchmark attached.
  - Particles are billboards with PBR textures that receive shadows and fog.
  - It collides with **arbitrary Avian colliders**, and it is **WASM and WebGL compatible**.
  - It supports soft edges, curves over lifetime, point, disk and sphere emitters, one-shot or continuous emission, and **nested emission**, for example smoke from flying debris ([README](https://github.com/mbrea-c/bevy_firework)).
  - Limitation: "Can't use custom images for particles."

#### bevy_enoki (CPU)
- **Facts:**
  - 0.8.0 was released 2026-10-11 and is **already on Bevy 0.20**.
  - It is a "2D particle system". It uses SIMD on the CPU and GPU instancing, with no compute shaders, and runs on web and mobile.
  - It has RON effect assets with hot reload, plus an editor ([README](https://github.com/Lommix/bevy_enoki)).
- **[Synthesis]** It is **2D only**, so it doesn't fit a 3D game.

#### bevy_particle_systems (CPU)
- **Facts:**
  - 0.13.0 was released 2024-07-18 for Bevy 0.14, and the last push was 2024-12-02 ([crates.io](https://crates.io/crates/bevy_particle_systems), [repo](https://github.com/abnormalbrain/bevy_particle_systems)).
  - It is described as "a native and WASM-compatible **2D** particle system".
- **[Synthesis]** Effectively **abandoned**. Six Bevy versions behind, and 2D only. Do not use.

#### Particles and netcode
- **[Synthesis]** Never replicate particles. Spawn them on each client from replicated gameplay events, such as explosions, shots and crate breaks, or from changes in predicted or replicated components.
- With prediction and rollback, guard the spawning so that re-simulated frames don't trigger the effect again. For example, spawn only on confirmed ticks, or deduplicate by event ID.

---

## 4. Risks

### Version lag behind Bevy 0.20 (released 2026-10-08)

| Crate | Supports 0.20 on crates.io? | Status |
|---|---|---|
| bevy_replicon 0.45, bevy_replicon_renet2 1.0, bevy_replicon_quinnet 0.21 | Yes | — |
| bevy_rapier3d 0.37, bevy_tnua 0.33, bevy-tnua-rapier3d 0.19 | Yes | — |
| bevy_enoki 0.8 | Yes | 2D only |
| **lightyear** 0.30 | No (0.19) | PR [#1779](https://github.com/cBournhonesque/lightyear/pull/1779) is open |
| **avian3d** 0.7 | No (0.19) | PR [#1070](https://github.com/avianphysics/avian/pull/1070) is open. A release was promised for around 2026-10-17/18 ([#1082](https://github.com/avianphysics/avian/issues/1082)) |
| bevy_hanabi 0.19 | No (0.19) | Already on `main`, not yet released |
| bevy_ggrs 0.22 | No (0.19) | PR [#135](https://github.com/gschup/bevy_ggrs/pull/135) is open |
| bevy_firework 0.10, bevy_sprinkles 0.3, bevy-tnua-avian3d 0.12, bevy_ahoy, avian_pickup | No (0.19) | No PR found. They usually follow Avian |
| bevy_matchbox 0.14 | No (**0.18**) | The 0.19 port, PR [#557](https://github.com/johanhelsing/matchbox/pull/557), is still open after 2 months |
| naia 0.25 | No (**0.18**) | `dev` branch is on 0.19 |
| bevy_particle_systems 0.13 | No (**0.14**) | Abandoned |

**[Synthesis]**
- The Avian-centric stack typically trails a Bevy release by about 1–4 weeks. The chain is Bevy, then Avian, then lightyear and the Avian add-ons.
- Pin Bevy 0.19.1 now, and expect one coordinated upgrade per Bevy release, roughly every 3–4 months, judging by 0.19.0 on 2026-06-19 and 0.20.0 on 2026-10-08.
- Bevy 0.20 moved all shaders to WESL ([release notes](https://bevy.org/news/bevy-0-20/#wesl-shaders)). That made upgrades unusually costly for rendering crates such as particle systems.

### Abandoned or at-risk crates
- **bevy_particle_systems** is abandoned.
- **bevy_rapier's old repo** is archived. That is not abandonment: the code moved into `dimforge/rapier`. Update any git dependencies ([notice](https://github.com/dimforge/bevy_rapier)).
- **bevy_matchbox** is slow to follow Bevy releases. Its open-items count is relatively high (43) and the last push was 2026-06-02.
- **naia** releases rarely: 0.24 came out 2024-12 and 0.25 in 2026-05.
- **Replicon add-ons:** `bevy_rewind` is at 0.0.1, and several others are marked unmaintained in replicon's own README.
- **Bus factor:** **[Synthesis, not verified]** each of the following looks like it depends heavily on one lead maintainer. This is based on whose repo it is and who comments as maintainer. I did not audit the contributor statistics.
  - lightyear (cBournhonesque);
  - avian (Jondolf);
  - hanabi (djeedai);
  - firework, sprinkles and enoki, which have small communities (82–198★).

### Technical risks
- **[Synthesis]** For **3D deterministic rollback with Avian**, the only published cross-platform test is 2D. If full rollback or lockstep matters, prototype a 3D crate-stack hash test across Windows, macOS, Linux and WASM early, or consider Rapier, whose determinism conditions are documented in more detail.
- **[Synthesis]** **Stacking stability under prediction:** stacks that sleep and wake, and contact warm-start state, are hard to roll back exactly. Lightyear's `timeline_switch` approach predicts only nearby objects and interpolates the rest, which limits the problem. Verify it with tall crate stacks and four predicted local units.
- **Vehicles:** Avian has no vehicle controller, so you would build ray-cast suspension yourself. Rapier 0.37 has one ([CHANGELOG](https://github.com/dimforge/rapier/blob/master/bindings/bevy_rapier/CHANGELOG.md)). **[Synthesis]** This is the main reason to weigh Rapier, against losing lightyear's Avian integration.
- **Web builds:** Hanabi and Sprinkles need WebGPU, while firework works on WebGL2. Lightyear's WASM client uses WebTransport. **[Synthesis]** If browser play is a goal, check which browsers you are targeting for both WebGPU and WebTransport, or ship firework as the web fallback for particles.
