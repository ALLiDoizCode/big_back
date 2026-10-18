//! PROTOTYPE — throwaway. Answers issue #10: which squad model makes the most fun and fair
//! competitive game? Not production code; lives on `prototype/squad-model`.
//!
//!     cargo run --example squad_model
//!
//! Built on the core-feel baseline from #9 (`prototype/core-feel`), which is fixed here as
//! constants: curling jump-throws, full-lock carrying, 18 m/s projectiles, reach-masked grabs.
//! Two teams of leaders (you plus bot leaders standing in for humans) and their AI.
//! Variant A: every leader has their own AI Buddies (squad size 0..3; 1 = hero + partner,
//! 0 = one character per player). Variant B: no squad, team-owned neutral haulers.
//! See `docs/prototypes/squad-model.md` for what to try.
//!
//! Play: WASD / left stick move, mouse / right stick aim, Space / A jump, E / X grab or place,
//! RMB / RB throw, LMB / RT shoot, F / B kick (hold on an orange crate to open it).
//! Orders: Q / LB context order (tap = one buddy, hold = whole squad) or, with "4 verbs",
//! Z follow, X haul, C guard pad, V attack / hold at the reticle (gamepad: hold LB + D-pad).
//! G / left-stick click recalls everyone to their default job.
//! T runs the readability test (freeze-frames; click the highlighted unit's owner).
//! Tune: Up/Down pick, Left/Right change (Shift = x5). N / 1-4 / Enter: guided comparisons.
//! R / Start resets, P prints settings, verdicts and results, H hides the panel.
//!
//! Smoke tests: `SQUAD_EXIT_AFTER=<s>`, `SQUAD_SCREENSHOT=<file.png>`, `SQUAD_SEAT=bot`,
//! `SQUAD_COMPARISON=<n>`, `SQUAD_READTEST=1` (starts the readability test after 3 s).

use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

const G: f32 = 20.0;
const ARENA: Vec2 = Vec2::new(18.0, 11.0);
const CRATE: f32 = 0.8;
const HALF: f32 = CRATE / 2.0;
const HEAD: f32 = 2.2;
const CHEST: f32 = 1.2;
const ACTOR_R: f32 = 0.45;
const YOU: usize = 0;
/// Team pads, on the crate grid (18 cells from the centre).
const PADS: [Vec2; 2] = [Vec2::new(-14.4, 0.0), Vec2::new(14.4, 0.0)];
const PAD_HALF: f32 = 2.0;
const CAM_OFFSET: Vec3 = Vec3::new(0.0, 21.0, 14.0);
const ARENA_CAM: Vec3 = Vec3::new(0.0, 46.0, 28.0);
const TURRET_H: f32 = 1.1;
const PLACE_REACH: f32 = 1.6;
const READ_QUESTIONS: usize = 8;

// The core-feel baseline picked by play in #9 (docs/prototypes/core-feel.md), fixed here.
const MOVE_SPEED: f32 = 6.0;
const CARRY_SPEED: f32 = 0.6;
const JUMP_VEL: f32 = 6.0;
const THROW_ANGLE: f32 = 35.0;
const THROW_DIST: f32 = 9.0;
const JUMP_BOOST: f32 = 1.5;
const SLIDE_KEEP: f32 = 0.8;
const SLIDE_FRICTION: f32 = 4.0;
const SNAP_RADIUS: f32 = 0.35;
const FIRE_RATE: f32 = 3.0;
const PROJ_SPEED: f32 = 18.0;
/// 4 shots kill a 100 hp leader.
const SHOT_DMG: f32 = 25.0;
const KICK_TIME: f32 = 1.5;
const GRAB_LATENCY: f32 = 0.12;
const GRAB_REACH: f32 = 1.2;

const TEAM_COLOR: [Color; 2] = [Color::srgb(0.3, 0.55, 1.0), Color::srgb(1.0, 0.35, 0.3)];
/// Owner colours by leader slot: a leader and their buddies share one.
const OWNER_COLOR: [Color; 4] = [
    Color::srgb(1.0, 0.9, 0.15),
    Color::srgb(0.25, 1.0, 0.35),
    Color::srgb(0.97, 0.97, 0.97),
    Color::srgb(0.85, 0.3, 1.0),
];

// ---------------------------------------------------------------------------------------
// Tunables
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum P {
    Model,
    TeamSize,
    SquadSize,
    Haulers,
    DefaultJob,
    Orders,
    Leash,
    BuddyHp,
    BuddyDamage,
    OnlineDelay,
    BlueBots,
    RedBots,
    Seat,
    OwnerBand,
    BuddyScale,
    HumanHalo,
    Tethers,
    OrderLines,
    MatchLength,
    TimeScale,
    Camera,
    Craft,
}

#[derive(Clone)]
enum Value {
    Num { v: f32, min: f32, max: f32, step: f32, unit: &'static str },
    Choice { i: usize, opts: &'static [&'static str] },
}

#[derive(Clone)]
struct Param {
    name: &'static str,
    value: Value,
}

fn num(name: &'static str, v: f32, min: f32, max: f32, step: f32, unit: &'static str) -> Param {
    Param { name, value: Value::Num { v, min, max, step, unit } }
}

fn choice(name: &'static str, i: usize, opts: &'static [&'static str]) -> Param {
    Param { name, value: Value::Choice { i, opts } }
}

const JOBS: &[&str] = &["follow & guard me", "haul to pad", "guard pad"];
const ORDER_STYLES: &[&str] = &["never", "micro"];

fn defaults() -> Vec<Param> {
    // Order must match `P`.
    vec![
        choice("squad model", 0, &["A: leader + own buddies", "B: team neutral haulers"]),
        num("team size", 3.0, 1.0, 4.0, 1.0, "v"),
        num("squad size (A)", 2.0, 0.0, 3.0, 1.0, "buddies"),
        num("haulers/player (B)", 1.0, 0.0, 3.0, 1.0, ""),
        choice("default job", 1, JOBS),
        choice("orders", 1, &["none", "one context button", "4 verbs"]),
        num("leash", 14.0, 6.0, 30.0, 2.0, "m"),
        num("buddy hp", 50.0, 25.0, 100.0, 25.0, ""),
        num("buddy damage", 0.5, 0.25, 1.0, 0.25, "x"),
        num("online delay", 0.0, 0.0, 250.0, 25.0, "ms"),
        choice("blue bots order", 0, ORDER_STYLES),
        choice("red bots order", 0, ORDER_STYLES),
        choice("your seat", 0, &["you play", "bot (watch)"]),
        choice("owner band", 1, &["off", "on"]),
        num("buddy scale", 0.75, 0.5, 1.0, 0.05, "x"),
        choice("human halo", 0, &["off", "on"]),
        choice("tethers", 0, &["off", "own squad", "everyone"]),
        choice("order lines", 1, &["off", "own squad"]),
        num("match length", 120.0, 30.0, 300.0, 30.0, "s"),
        num("time scale", 1.0, 1.0, 4.0, 1.0, "x"),
        choice("camera", 0, &["follow you", "whole arena"]),
        // PLACEHOLDER only, so stacking does something. The real grammar is issue #17.
        choice("placeholder craft", 0, &["2x2 = turret", "off"]),
    ]
}

#[derive(Resource)]
struct Tune {
    vals: Vec<Param>,
    selected: usize,
    panel: bool,
    guide: Option<Guide>,
    settled: Vec<bool>,
    verdicts: Vec<String>,
}

impl Tune {
    fn f(&self, p: P) -> f32 {
        match self.vals[p as usize].value {
            Value::Num { v, .. } => v,
            Value::Choice { i, .. } => i as f32,
        }
    }
    fn c(&self, p: P) -> usize {
        self.f(p) as usize
    }
    fn delay(&self) -> f32 {
        self.f(P::OnlineDelay) / 1000.0
    }
}

fn set(vals: &mut [Param], p: P, x: f32) {
    match &mut vals[p as usize].value {
        Value::Num { v, min, max, .. } => *v = x.clamp(*min, *max),
        Value::Choice { i, opts } => *i = (x as usize).min(opts.len() - 1),
    }
}

fn fmt_value(v: &Value) -> String {
    match v {
        Value::Num { v, step, unit, .. } => {
            if *step < 1.0 { format!("{v:.2} {unit}") } else { format!("{v:.0} {unit}") }
        }
        Value::Choice { i, opts } => opts[*i].to_string(),
    }
}

/// Model, team size, squad size, haulers per team: changing any of these rebuilds the match.
fn roster_of(tune: &Tune) -> [usize; 4] {
    let model = tune.c(P::Model);
    let n = tune.c(P::TeamSize);
    let squad = if model == 0 { tune.c(P::SquadSize) } else { 0 };
    let haulers = if model == 1 { tune.c(P::Haulers) * n } else { 0 };
    [model, n, squad, haulers]
}

fn roster_desc(tune: &Tune) -> String {
    let n = tune.c(P::TeamSize);
    if tune.c(P::Model) == 1 {
        format!("B {n}v{n}, {} haulers per player", tune.c(P::Haulers))
    } else {
        format!(
            "A {n}v{n}, {} buddies each, default job {}, orders {}",
            tune.c(P::SquadSize),
            JOBS[tune.c(P::DefaultJob)],
            fmt_value(&tune.vals[P::Orders as usize].value)
        )
    }
}

fn looks_desc(tune: &Tune) -> String {
    format!(
        "band {}, buddy scale {:.2}, halo {}, tethers {}",
        fmt_value(&tune.vals[P::OwnerBand as usize].value),
        tune.f(P::BuddyScale),
        fmt_value(&tune.vals[P::HumanHalo as usize].value),
        fmt_value(&tune.vals[P::Tethers as usize].value),
    )
}

// ---------------------------------------------------------------------------------------
// Guided comparisons. N starts the next one, 1-4 pick the variant you're playing, Enter
// keeps it (it becomes the baseline) and moves on.
// ---------------------------------------------------------------------------------------

struct Comparison {
    row: &'static str,
    title: &'static str,
    question: &'static str,
    /// Applied to every variant.
    both: &'static [(P, f32)],
    variants: &'static [(&'static str, &'static [(P, f32)])],
    /// Set back to the baseline when the comparison ends, so a test setup doesn't leak.
    restore: &'static [P],
}

const COMPARISONS: &[Comparison] = &[
    Comparison {
        row: "1", title: "squad size (3v3, model A)",
        question: "How many buddies should you own? 0 = one character per player, 1 = hero + partner, 3 = Team Buddies.",
        both: &[(P::Model, 0.0), (P::TeamSize, 3.0)],
        variants: &[
            ("0: solo", &[(P::SquadSize, 0.0)]),
            ("1: hero + partner", &[(P::SquadSize, 1.0)]),
            ("2 buddies", &[(P::SquadSize, 2.0)]),
            ("3 buddies", &[(P::SquadSize, 3.0)]),
        ],
        restore: &[],
    },
    Comparison {
        row: "2", title: "A own squad vs B neutral haulers (3v3)",
        question: "Is owning and ordering buddies more fun than team haulers nobody commands? Is B still Team Buddies?",
        both: &[(P::TeamSize, 3.0)],
        variants: &[
            ("A: 2 buddies each", &[(P::Model, 0.0), (P::SquadSize, 2.0)]),
            ("B: 2 haulers per player", &[(P::Model, 1.0), (P::Haulers, 2.0)]),
        ],
        restore: &[],
    },
    Comparison {
        row: "3", title: "default job",
        question: "What should a buddy do with no order? It must never idle. Which job helps without playing for you?",
        both: &[(P::Model, 0.0)],
        variants: &[
            ("follow & guard me", &[(P::DefaultJob, 0.0)]),
            ("haul to pad", &[(P::DefaultJob, 1.0)]),
            ("guard pad", &[(P::DefaultJob, 2.0)]),
        ],
        restore: &[],
    },
    Comparison {
        row: "4", title: "order verbs",
        question: "Do you give orders mid-fight at all? One context button (tap = one buddy, hold = all) vs four verbs vs none.",
        both: &[(P::Model, 0.0)],
        variants: &[
            ("none (default job only)", &[(P::Orders, 0.0)]),
            ("one context button (Q / LB)", &[(P::Orders, 1.0)]),
            ("4 verbs (Z X C V)", &[(P::Orders, 2.0)]),
        ],
        restore: &[],
    },
    Comparison {
        row: "5", title: "readability at full size (3v3, 3 buddies = 24 bodies)",
        question: "Press T in each variant: 8 freeze-frames. Click the highlighted unit's OWNER: the unit itself if it's a human, \
                   its leader if it's a buddy, its pad if it's a hauler. Target: right, in under 1 s.",
        both: &[(P::Model, 0.0), (P::TeamSize, 3.0), (P::SquadSize, 3.0), (P::Tethers, 0.0), (P::Camera, 0.0)],
        variants: &[
            ("team colour only", &[(P::OwnerBand, 0.0), (P::BuddyScale, 1.0), (P::HumanHalo, 0.0)]),
            ("owner bands", &[(P::OwnerBand, 1.0), (P::BuddyScale, 1.0), (P::HumanHalo, 0.0)]),
            ("bands + small buddies", &[(P::OwnerBand, 1.0), (P::BuddyScale, 0.7), (P::HumanHalo, 0.0)]),
            ("bands + small + human halo", &[(P::OwnerBand, 1.0), (P::BuddyScale, 0.7), (P::HumanHalo, 1.0)]),
        ],
        restore: &[],
    },
    Comparison {
        row: "6", title: "fairness probe (bots only: you watch)",
        question: "Blue's bots never order, red's micro. A match per variant, results print at the end. \
                   Keep the squad size where an idle squad is neither useless nor dominant, and micro still wins.",
        both: &[
            (P::Model, 0.0), (P::TeamSize, 3.0), (P::Seat, 1.0), (P::Camera, 1.0), (P::TimeScale, 2.0),
            (P::BlueBots, 0.0), (P::RedBots, 1.0),
        ],
        variants: &[
            ("1 buddy", &[(P::SquadSize, 1.0)]),
            ("2 buddies", &[(P::SquadSize, 2.0)]),
            ("3 buddies", &[(P::SquadSize, 3.0)]),
        ],
        restore: &[P::Seat, P::Camera, P::TimeScale, P::BlueBots, P::RedBots],
    },
    Comparison {
        row: "7", title: "online delay on buddies",
        question: "Online, buddies are always interpolated (shown in the past) and orders take a round trip. Does the squad still feel responsive?",
        both: &[(P::Model, 0.0)],
        variants: &[
            ("local 0 ms", &[(P::OnlineDelay, 0.0)]),
            ("100 ms", &[(P::OnlineDelay, 100.0)]),
            ("200 ms", &[(P::OnlineDelay, 200.0)]),
        ],
        restore: &[P::OnlineDelay],
    },
    Comparison {
        row: "8", title: "team size",
        question: "With the squad you picked, how many players per team? This sets the range of team sizes. Budget: ~32 networked bodies.",
        both: &[],
        variants: &[
            ("2v2", &[(P::TeamSize, 2.0)]),
            ("3v3", &[(P::TeamSize, 3.0)]),
            ("4v4", &[(P::TeamSize, 4.0)]),
        ],
        restore: &[],
    },
];

struct Guide {
    idx: usize,
    variant: usize,
    base: Vec<Param>,
}

fn apply_variant(tune: &mut Tune, v: usize) {
    let Some(g) = &mut tune.guide else { return };
    let cmp = &COMPARISONS[g.idx];
    let v = v.min(cmp.variants.len() - 1);
    g.variant = v;
    let mut vals = g.base.clone();
    for (p, x) in cmp.both.iter().chain(cmp.variants[v].1) {
        set(&mut vals, *p, *x);
    }
    tune.vals = vals;
}

fn start_next(tune: &mut Tune, sim: &mut Sim, from: Option<usize>) {
    let n = COMPARISONS.len();
    let first = from.map_or(0, |i| i + 1);
    let Some(idx) = (0..n).map(|k| (first + k) % n).find(|&i| !tune.settled[i]) else {
        tune.guide = None;
        sim.say("all comparisons done: press P and paste the output");
        return;
    };
    let base = tune.vals.clone();
    tune.guide = Some(Guide { idx, variant: 0, base });
    apply_variant(tune, 0);
    sim.reset(tune);
}

fn guide_from_env(mut tune: ResMut<Tune>, mut sim: ResMut<Sim>) {
    if std::env::var("SQUAD_SEAT").as_deref() == Ok("bot") {
        set(&mut tune.vals, P::Seat, 1.0);
    }
    if let Some(i) = std::env::var("SQUAD_COMPARISON").ok().and_then(|v| v.parse::<usize>().ok()) {
        // Row numbers start at 1.
        start_next(&mut tune, &mut sim, i.checked_sub(2));
    }
}

// ---------------------------------------------------------------------------------------
// Simulation state (plain data; entities only mirror it)
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Normal,
    Loot,
}

#[derive(Clone, Copy, PartialEq)]
enum CrateState {
    Free,
    Carried { by: usize, lift_from: Vec3, lift_t: f32 },
}

struct Crate {
    id: u32,
    kind: Kind,
    pos: Vec3,
    vel: Vec3,
    state: CrateState,
    kick: f32,
    thrown_by: Option<usize>,
    settling: bool,
    ent: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq)]
enum Role {
    /// A player seat: you, or a bot standing in for a human.
    Leader,
    /// An AI Buddy owned by the leader at this index.
    Buddy(usize),
    /// Variant B: owned by the team, takes no orders.
    Hauler,
}

#[derive(Clone, Copy, PartialEq)]
enum Job {
    /// No order: the "default job" knob decides.
    Default,
    Follow,
    Haul,
    GuardPad,
    HaulThis(u32),
    Attack(usize),
    Hold(Vec3),
}

impl Job {
    fn label(self) -> &'static str {
        match self {
            Job::Default => "default",
            Job::Follow => "follow",
            Job::Haul => "haul",
            Job::GuardPad => "guard pad",
            Job::HaulThis(_) => "haul THIS",
            Job::Attack(_) => "ATTACK",
            Job::Hold(_) => "HOLD",
        }
    }
}

struct Actor {
    role: Role,
    team: usize,
    /// Leader slot in the team (buddies copy their leader's): picks the owner colour.
    slot: usize,
    /// Index within the squad (or among the team's haulers).
    spot: usize,
    pos: Vec3,
    vel: Vec3,
    facing: Vec3,
    grounded: bool,
    carrying: Option<u32>,
    /// The crate this AI is heading for, so teammates pick different ones.
    claim: Option<u32>,
    fire_cd: f32,
    hp: f32,
    max_hp: f32,
    dead: f32,
    stun: f32,
    job: Job,
    job_t: f32,
    /// Order tell: counts down from 1 after an order lands.
    told: f32,
    /// Bot leaders: time to the next squad re-think.
    think: f32,
    /// Recent positions, to show remote units slightly in the past (online interpolation).
    hist: VecDeque<(f32, Vec3)>,
    ent: Option<Entity>,
}

impl Actor {
    fn new(role: Role, team: usize, slot: usize, spot: usize, hp: f32) -> Self {
        Actor {
            role,
            team,
            slot,
            spot,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            facing: Vec3::X,
            grounded: true,
            carrying: None,
            claim: None,
            fire_cd: 0.5,
            hp,
            max_hp: hp,
            dead: 0.0,
            stun: 0.0,
            job: Job::Default,
            job_t: 0.0,
            told: 0.0,
            think: 1.0,
            hist: VecDeque::new(),
            ent: None,
        }
    }
    fn alive(&self) -> bool {
        self.dead <= 0.0
    }
    fn is_ai(&self) -> bool {
        self.role != Role::Leader
    }
}

struct Pending {
    crate_id: u32,
    left: f32,
    total: f32,
}

/// PLACEHOLDER craft result (see #17): shoots the nearest enemy for a while, then expires.
struct Turret {
    team: usize,
    pos: Vec3,
    facing: Vec3,
    cd: f32,
    life: f32,
    ent: Option<Entity>,
}

struct Shot {
    pos: Vec3,
    vel: Vec3,
    /// None for turret shots.
    owner: Option<usize>,
    team: usize,
    dmg: f32,
    life: f32,
    ent: Option<Entity>,
}

#[derive(Resource)]
struct Sim {
    actors: Vec<Actor>,
    crates: Vec<Crate>,
    shots: Vec<Shot>,
    turrets: Vec<Turret>,
    pending: Option<Pending>,
    /// Orders in flight: (lands at, buddy, job). Online they take a round trip.
    orders: Vec<(f32, usize, Job)>,
    next_id: u32,
    rng: u32,
    t: f32,
    loot_respawn: f32,
    msgs: Vec<(String, f32)>,
    despawn: Vec<Entity>,
    roster: [usize; 4],
    match_left: f32,
    /// Counts down the end-of-match result screen.
    over: f32,
    /// Freeze-frame for the readability test.
    frozen: bool,
    /// Match results and readability results, printed with P. Survives resets.
    log: Vec<String>,
    view_key: u64,
    // Stats.
    fired: u32,
    hits: u32,
    kills: [u32; 2],
    orders_given: u32,
}

impl Sim {
    fn new(tune: &Tune) -> Self {
        let mut s = Sim {
            actors: Vec::new(),
            crates: Vec::new(),
            shots: Vec::new(),
            turrets: Vec::new(),
            pending: None,
            orders: Vec::new(),
            next_id: 1,
            rng: 0x2545_f491,
            t: 0.0,
            loot_respawn: 0.0,
            msgs: Vec::new(),
            despawn: Vec::new(),
            roster: [0; 4],
            match_left: 0.0,
            over: 0.0,
            frozen: false,
            log: Vec::new(),
            view_key: 0,
            fired: 0,
            hits: 0,
            kills: [0; 2],
            orders_given: 0,
        };
        s.reset(tune);
        s
    }

    fn reset(&mut self, tune: &Tune) {
        for c in self.crates.drain(..) {
            self.despawn.extend(c.ent);
        }
        for t in self.turrets.drain(..) {
            self.despawn.extend(t.ent);
        }
        for sh in self.shots.drain(..) {
            self.despawn.extend(sh.ent);
        }
        for a in self.actors.drain(..) {
            self.despawn.extend(a.ent);
        }
        self.pending = None;
        self.orders.clear();
        self.roster = roster_of(tune);
        let [_, n, squad, haulers] = self.roster;
        let buddy_hp = tune.f(P::BuddyHp);
        for team in 0..2 {
            for slot in 0..n {
                self.actors.push(Actor::new(Role::Leader, team, slot, 0, 100.0));
            }
        }
        for li in 0..2 * n {
            let (team, slot) = (self.actors[li].team, self.actors[li].slot);
            for k in 0..squad {
                self.actors.push(Actor::new(Role::Buddy(li), team, slot, k, buddy_hp));
            }
        }
        for team in 0..2 {
            for k in 0..haulers {
                self.actors.push(Actor::new(Role::Hauler, team, k % n.max(1), k, buddy_hp));
            }
        }
        for i in 0..self.actors.len() {
            let p = self.spawn_point(i);
            let a = &mut self.actors[i];
            a.pos = p;
            a.facing = Vec3::X * toward_centre(a.team);
        }
        let count = 6 + 3 * n;
        for i in 0..count {
            let x = -4.8 + (i % 4) as f32 * 3.2;
            let z = -8.0 + (i / 4) as f32 * 3.2;
            self.spawn_crate(Kind::Normal, Vec3::new(x, HALF, z));
        }
        self.spawn_crate(Kind::Loot, Vec3::new(0.0, HALF, -0.4));
        self.spawn_crate(Kind::Loot, Vec3::new(0.0, HALF, 8.0));
        self.match_left = tune.f(P::MatchLength);
        self.over = 0.0;
        self.fired = 0;
        self.hits = 0;
        self.kills = [0; 2];
        self.orders_given = 0;
    }

    /// Leaders start in a line in front of their pad; buddies beside their leader;
    /// haulers on the pad's front edge.
    fn spawn_point(&self, i: usize) -> Vec3 {
        let a = &self.actors[i];
        let n = self.roster[1].max(1);
        let dir = toward_centre(a.team);
        let pad = PADS[a.team];
        let lane = (a.slot as f32 - (n as f32 - 1.0) / 2.0) * 3.2;
        match a.role {
            Role::Leader => Vec3::new(pad.x + dir * 4.0, 0.0, lane),
            Role::Buddy(_) => Vec3::new(pad.x + dir * 2.8, 0.0, lane + (a.spot as f32 - 1.0) * 1.0),
            Role::Hauler => Vec3::new(pad.x + dir * 2.6, 0.0, (a.spot as f32 - 2.0) * 1.1),
        }
    }

    fn rand(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        (self.rng % 10_000) as f32 / 10_000.0
    }

    fn spawn_crate(&mut self, kind: Kind, pos: Vec3) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.crates.push(Crate {
            id,
            kind,
            pos,
            vel: Vec3::ZERO,
            state: CrateState::Free,
            kick: 0.0,
            thrown_by: None,
            settling: false,
            ent: None,
        });
        id
    }

    fn crate_mut(&mut self, id: u32) -> Option<&mut Crate> {
        self.crates.iter_mut().find(|c| c.id == id)
    }

    fn free_crate(&self, id: u32) -> Option<&Crate> {
        self.crates.iter().find(|c| c.id == id && c.state == CrateState::Free)
    }

    fn say(&mut self, s: impl Into<String>) {
        self.msgs.push((s.into(), 3.0));
        if self.msgs.len() > 5 {
            self.msgs.remove(0);
        }
    }

    fn release(&mut self, who: usize, vel: Vec3, offset: Vec3) {
        let Some(id) = self.actors[who].carrying.take() else { return };
        let head = self.actors[who].pos + Vec3::Y * HEAD + offset;
        if let Some(c) = self.crate_mut(id) {
            c.state = CrateState::Free;
            c.pos = head;
            c.vel = vel;
            c.thrown_by = if vel.length() > 4.0 { Some(who) } else { None };
        }
    }

    fn place_target(&self, who: usize) -> Vec3 {
        let a = &self.actors[who];
        self.target_at((a.pos + a.facing * 1.1).xz())
    }

    /// The landing centre for a crate aimed at `want`: its grid cell, on top of whatever is in it.
    fn target_at(&self, want: Vec2) -> Vec3 {
        let free = || self.crates.iter().filter(|c| c.state == CrateState::Free);
        let cell = (want / CRATE).round() * CRATE;
        let xz = free()
            .map(|c| c.pos.xz())
            .filter(|p| p.distance(want) < HALF && p.distance(cell) > 0.05)
            .min_by(|x, y| x.distance(want).total_cmp(&y.distance(want)))
            .unwrap_or(cell);
        let top = free()
            .filter(|c| (c.pos.x - xz.x).abs() < CRATE * 0.9 && (c.pos.z - xz.y).abs() < CRATE * 0.9)
            .map(|c| c.pos.y + HALF)
            .chain(self.turrets.iter().filter(|t| t.pos.xz().distance(xz) < 1.1).map(|_| TURRET_H))
            .fold(0.0, f32::max);
        Vec3::new(xz.x, top + HALF, xz.y)
    }

    /// Where an AI hauler sets its crate down on its pad: the lowest cell, then the nearest.
    fn pad_cell(&self, team: usize, from: Vec3) -> Vec3 {
        let pad = PADS[team];
        let mut best: Option<(f32, Vec3)> = None;
        for gx in -2..=2 {
            for gz in -2..=2 {
                let at = self.target_at(pad + Vec2::new(gx as f32, gz as f32) * CRATE);
                let score = at.y * 10.0 + at.xz().distance(from.xz());
                if best.is_none_or(|(b, _)| score < b) {
                    best = Some((score, at));
                }
            }
        }
        best.map_or(pad.extend(HALF).xzy(), |(_, at)| at)
    }

    fn place(&mut self, who: usize, at: Vec3) {
        let Some(id) = self.actors[who].carrying.take() else { return };
        if let Some(c) = self.crate_mut(id) {
            c.state = CrateState::Free;
            c.pos = at;
            c.vel = Vec3::ZERO;
            c.thrown_by = None;
        }
    }

    fn take(&mut self, who: usize, id: u32) {
        let from = self.crate_mut(id).map(|c| c.pos).unwrap_or_default();
        if let Some(c) = self.crate_mut(id) {
            c.state = CrateState::Carried { by: who, lift_from: from, lift_t: 0.0 };
            c.vel = Vec3::ZERO;
            c.thrown_by = None;
        }
        self.actors[who].carrying = Some(id);
        self.actors[who].claim = None;
    }

    fn nearest_free(&self, who: usize, reach: f32) -> Option<(u32, f32)> {
        let a = &self.actors[who];
        self.crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal)
            .filter(|c| c.pos.y - a.pos.y < 3.0)
            .map(|c| (c.id, c.pos.xz().distance(a.pos.xz())))
            .filter(|(_, d)| *d <= reach + HALF)
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }

    fn hurt(&mut self, who: usize, amount: f32, knock: Vec3, by: Option<usize>, by_team: usize) {
        if !self.actors[who].alive() {
            return;
        }
        // Core feel: a hit knocks the crate loose.
        if self.actors[who].carrying.is_some() {
            self.release(who, knock * 0.3 + Vec3::Y * 2.0, Vec3::ZERO);
        }
        let a = &mut self.actors[who];
        a.vel += knock;
        a.stun = a.stun.max(0.15);
        a.hp -= amount;
        if a.hp > 0.0 {
            return;
        }
        a.dead = if a.role == Role::Leader { 3.0 } else { 5.0 };
        a.claim = None;
        self.kills[by_team] += 1;
        if who == YOU {
            self.pending = None;
            self.say("you're down (3 s)");
        } else if by == Some(YOU) {
            let what = self.describe(who);
            self.say(format!("you downed {what}"));
        }
    }

    fn describe(&self, i: usize) -> String {
        let a = &self.actors[i];
        let team = ["blue", "red"][a.team];
        match a.role {
            Role::Leader if i == YOU => "you".into(),
            Role::Leader => format!("{team} leader {}", a.slot + 1),
            Role::Buddy(o) => format!("{team} leader {}'s buddy", self.actors[o].slot + 1),
            Role::Hauler => format!("a {team} hauler"),
        }
    }

    fn squad(&self, leader: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.actors.len()).filter(move |&j| self.actors[j].role == Role::Buddy(leader))
    }

    /// Crates resting on the team's pad, plus 4 per placeholder turret built from them.
    fn score(&self, team: usize) -> usize {
        let on_pad = self
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && pad_contains(PADS[team], c.pos))
            .count();
        on_pad + 4 * self.turrets.iter().filter(|t| t.team == team).count()
    }

    fn order(&mut self, b: usize, job: Job, delay: f32) {
        self.orders.retain(|o| o.1 != b);
        self.orders.push((self.t + delay, b, job));
    }
}

fn toward_centre(team: usize) -> f32 {
    if team == 0 { 1.0 } else { -1.0 }
}

fn pad_contains(pad: Vec2, p: Vec3) -> bool {
    (p.x - pad.x).abs() < PAD_HALF && (p.z - pad.y).abs() < PAD_HALF
}

/// Where actor `a` is drawn: in the past by the online delay, except for you (predicted).
fn shown(a: &Actor, t: f32, delay: f32) -> Vec3 {
    if delay <= 0.0 || a.hist.is_empty() {
        return a.pos;
    }
    let want = t - delay;
    let mut prev = a.hist[0];
    for &(ht, hp) in &a.hist {
        if ht >= want {
            let span = ht - prev.0;
            let k = if span > 0.0 { (want - prev.0) / span } else { 1.0 };
            return prev.1.lerp(hp, k.clamp(0.0, 1.0));
        }
        prev = (ht, hp);
    }
    a.pos
}

fn shown_at(s: &Sim, i: usize, tune: &Tune) -> Vec3 {
    let seat_bot = tune.c(P::Seat) == 1;
    let delay = if i == YOU && !seat_bot { 0.0 } else { tune.delay() };
    shown(&s.actors[i], s.t, delay)
}

// ---------------------------------------------------------------------------------------
// Crate physics and throws (the #9 baseline: curling landings, apex jump-throws)
// ---------------------------------------------------------------------------------------

fn velocity_to(from: Vec3, to: Vec3, angle: f32) -> Vec3 {
    let flat = (to - from).xz();
    let d = flat.length().max(0.05);
    let dir = (flat / d).extend(0.0).xzy();
    let drop = from.y - to.y;
    let denom = (d * angle.tan() + drop).max(0.05);
    let speed = (G * d * d / (2.0 * angle.cos().powi(2) * denom)).sqrt();
    dir * speed * angle.cos() + Vec3::Y * speed * angle.sin()
}

struct ThrowPlan {
    from: Vec3,
    vel: Vec3,
    target: Option<Vec3>,
    short: bool,
    path: Vec<Vec3>,
    landing: Vec3,
    knock: bool,
    boost: f32,
}

/// Reach grows with jump height, peaking at the apex.
fn jump_boost(a: &Actor) -> f32 {
    if a.grounded {
        return 1.0;
    }
    let apex = JUMP_VEL.powi(2) / (2.0 * G);
    1.0 + (JUMP_BOOST - 1.0) * (a.pos.y / apex).clamp(0.0, 1.0)
}

fn floor_under(p: Vec3, others: impl Iterator<Item = Vec3>, turrets: impl Iterator<Item = Vec2>) -> f32 {
    let mut floor: f32 = 0.0;
    for t in turrets {
        if t.distance(p.xz()) < 1.1 && p.y > TURRET_H {
            floor = TURRET_H;
        }
    }
    for o in others {
        let overlap = (o.x - p.x).abs() < CRATE * 0.9 && (o.z - p.z).abs() < CRATE * 0.9;
        if overlap && o.y + 0.5 < p.y {
            floor = floor.max(o.y + HALF);
        }
    }
    floor
}

/// One step of a free crate: gravity, then a curling landing (keep some speed, slide with
/// friction, click into a cell once slow).
fn crate_motion(p: &mut Vec3, v: &mut Vec3, settling: &mut bool, floor: impl Fn(Vec3) -> f32, dt: f32) -> bool {
    v.y -= G * dt;
    *p += *v * dt;
    if p.x.abs() > ARENA.x - HALF {
        p.x = p.x.clamp(-ARENA.x + HALF, ARENA.x - HALF);
        v.x *= -0.4;
    }
    if p.z.abs() > ARENA.y - HALF {
        p.z = p.z.clamp(-ARENA.y + HALF, ARENA.y - HALF);
        v.z *= -0.4;
    }
    let floor = floor(*p);
    if p.y - HALF > floor {
        *settling = false;
        return false;
    }
    p.y = floor + HALF;
    if v.y < -2.0 {
        v.x *= SLIDE_KEEP;
        v.z *= SLIDE_KEEP;
    }
    v.y = if v.y < -6.0 { -v.y * 0.25 } else { 0.0 };
    let h = v.xz();
    let speed = h.length();
    let slowed = (speed - SLIDE_FRICTION * dt).max(0.0);
    let h = if speed > 0.0 { h * (slowed / speed) } else { h };
    v.x = h.x;
    v.z = h.y;
    let cell = (p.xz() / CRATE).round() * CRATE;
    let off = cell.distance(p.xz());
    if v.y == 0.0 && off < SNAP_RADIUS && (slowed < 1.5 || *settling) {
        *settling = true;
        let pull = (cell - p.xz()) * 20.0;
        v.x = pull.x;
        v.z = pull.y;
        if off < 0.02 {
            p.x = cell.x;
            p.z = cell.y;
            v.x = 0.0;
            v.z = 0.0;
            *settling = false;
        }
    } else {
        *settling = false;
    }
    true
}

fn side_hit(a: Vec3, b: Vec3) -> bool {
    (a.y - b.y).abs() < 0.3 && (a.x - b.x).abs() < CRATE * 0.95 && (a.z - b.z).abs() < CRATE * 0.95
}

fn predict(s: &Sim, from: Vec3, vel: Vec3) -> (Vec<Vec3>, Vec3, bool) {
    let (mut p, mut v, mut settling) = (from, vel, false);
    let others: Vec<Vec3> = s.crates.iter().filter(|c| c.state == CrateState::Free).map(|c| c.pos).collect();
    let turrets: Vec<Vec2> = s.turrets.iter().map(|t| t.pos.xz()).collect();
    let mut path = vec![p];
    let dt = 1.0 / 60.0;
    for _ in 0..600 {
        let grounded = crate_motion(
            &mut p,
            &mut v,
            &mut settling,
            |q| floor_under(q, others.iter().copied(), turrets.iter().copied()),
            dt,
        );
        path.push(p);
        if others.iter().any(|o| side_hit(p, *o)) && v.xz().length() > 0.3 {
            return (path, p, true);
        }
        if grounded && v.length() < 0.05 && !settling {
            break;
        }
    }
    (path, p, false)
}

fn throw_plan(s: &Sim, aim_dist: Option<f32>) -> ThrowPlan {
    let a = &s.actors[YOU];
    if a.grounded {
        // Jump-throw: on the ground you can only toss it a short way.
        let from = a.pos + Vec3::Y * HEAD + a.facing * 0.5;
        let vel = a.facing * 3.0 + Vec3::Y * 3.0;
        let (path, landing, knock) = predict(s, from, vel);
        return ThrowPlan { from, vel, target: None, short: true, path, landing, knock, boost: 1.0 };
    }
    let from = a.pos + Vec3::Y * HEAD + a.facing * 0.3;
    let boost = jump_boost(a);
    let max = THROW_DIST * boost;
    let d = aim_dist.unwrap_or(max).clamp(1.0, max);
    let want = (a.pos + a.facing * d).xz();
    let free = s.crates.iter().filter(|c| c.state == CrateState::Free).map(|c| c.pos);
    let top = floor_under(want.extend(99.0).xzy(), free, s.turrets.iter().map(|t| t.pos.xz()));
    let target = Vec3::new(want.x, top + HALF, want.y);
    let vel = velocity_to(from, target, THROW_ANGLE.to_radians());
    let (path, landing, knock) = predict(s, from, vel);
    ThrowPlan { from, vel, target: Some(target), short: false, path, landing, knock, boost }
}

// ---------------------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------------------

#[derive(Resource, Default, Clone)]
struct Controls {
    mv: Vec2,
    aim: Option<Vec3>,
    jump: bool,
    grab: bool,
    throw: bool,
    fire: bool,
    kick_held: bool,
    kick: bool,
    use_mouse: bool,
    aim_point: Option<Vec3>,
    aim_dist: Option<f32>,
    /// Context order: when the button went down, and how long it was held on release.
    order_down: Option<f32>,
    order_release: Option<f32>,
    /// 4 verbs: 0 follow, 1 haul, 2 guard pad, 3 attack / hold at the reticle.
    verb: Option<u8>,
    recall: bool,
}

#[derive(Component)]
struct MainCamera;

fn read_controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    // CursorMoved, not MouseMotion: raw device motion often never arrives on Wayland.
    mut moved: MessageReader<CursorMoved>,
    gamepads: Query<&Gamepad>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    sim: Res<Sim>,
    mut c: ResMut<Controls>,
) {
    let now = time.elapsed_secs();
    let mut mv = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { mv.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { mv.y -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { mv.x += 1.0; }
    if keys.pressed(KeyCode::KeyA) { mv.x -= 1.0; }
    c.jump = keys.just_pressed(KeyCode::Space);
    c.grab = keys.just_pressed(KeyCode::KeyE);
    c.throw = mouse.just_pressed(MouseButton::Right);
    c.fire = mouse.pressed(MouseButton::Left);
    c.kick_held = keys.pressed(KeyCode::KeyF);
    c.kick = keys.just_pressed(KeyCode::KeyF);
    c.recall = keys.just_pressed(KeyCode::KeyG);
    c.verb = [KeyCode::KeyZ, KeyCode::KeyX, KeyCode::KeyC, KeyCode::KeyV]
        .iter()
        .position(|k| keys.just_pressed(*k))
        .map(|i| i as u8);
    let mut order_press = keys.just_pressed(KeyCode::KeyQ);
    let mut order_up = keys.just_released(KeyCode::KeyQ);
    if moved.read().count() > 0 || mouse.just_pressed(MouseButton::Left) {
        c.use_mouse = true;
    }
    let mut stick_aim = None;
    let mut tilt = 0.0;
    for pad in &gamepads {
        let ls = pad.left_stick();
        if ls.length() > 0.15 { mv = ls; }
        let rs = pad.right_stick();
        if rs.length() > 0.3 {
            stick_aim = Some(Vec3::new(rs.x, 0.0, -rs.y).normalize());
            tilt = ((rs.length() - 0.3) / 0.65).clamp(0.0, 1.0);
            c.use_mouse = false;
        }
        c.jump |= pad.just_pressed(GamepadButton::South);
        c.grab |= pad.just_pressed(GamepadButton::West);
        c.throw |= pad.just_pressed(GamepadButton::RightTrigger);
        c.fire |= pad.pressed(GamepadButton::RightTrigger2);
        c.kick_held |= pad.pressed(GamepadButton::East);
        c.kick |= pad.just_pressed(GamepadButton::East);
        c.recall |= pad.just_pressed(GamepadButton::LeftThumb);
        order_press |= pad.just_pressed(GamepadButton::LeftTrigger);
        order_up |= pad.just_released(GamepadButton::LeftTrigger);
        if pad.pressed(GamepadButton::LeftTrigger) {
            let dpad = [GamepadButton::DPadUp, GamepadButton::DPadLeft, GamepadButton::DPadDown, GamepadButton::DPadRight];
            if let Some(i) = dpad.iter().position(|b| pad.just_pressed(*b)) {
                c.verb = Some(i as u8);
            }
        }
    }
    c.order_release = None;
    if order_press {
        c.order_down = Some(now);
    }
    if order_up && let Some(t0) = c.order_down.take() {
        c.order_release = Some(now - t0);
    }
    c.mv = mv.clamp_length_max(1.0);
    c.aim = stick_aim;
    c.aim_point = None;
    c.aim_dist = None;
    let you = &sim.actors[YOU];
    if let Some(dir) = stick_aim {
        let max = if you.carrying.is_some() { THROW_DIST * JUMP_BOOST } else { 30.0 };
        let d = 1.0 + tilt * (max - 1.0);
        c.aim_dist = Some(d);
        c.aim_point = Some((you.pos + dir * d).with_y(CHEST));
    }
    if c.use_mouse && stick_aim.is_none() {
        let (cam, cam_tf) = *camera;
        if let Some(cursor) = window.cursor_position()
            && let Ok(ray) = cam.viewport_to_world(cam_tf, cursor)
            && let Some(d) = ray.intersect_plane(Vec3::Y * CHEST, InfinitePlane3d::new(Vec3::Y))
        {
            let p = ray.get_point(d);
            let to = (p - you.pos).with_y(0.0);
            c.aim_point = Some(p.with_y(CHEST));
            c.aim_dist = Some(to.length());
            if to.length() > 0.2 {
                c.aim = Some(to.normalize());
            }
        }
    }
}

fn tune_input(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut tune: ResMut<Tune>,
    mut sim: ResMut<Sim>,
) {
    // Holding LB turns the D-pad into order verbs.
    let lb = gamepads.iter().any(|g| g.pressed(GamepadButton::LeftTrigger));
    let any = |k: KeyCode, b: GamepadButton| keys.just_pressed(k) || (!lb && gamepads.iter().any(|g| g.just_pressed(b)));
    let n = tune.vals.len();
    if any(KeyCode::ArrowUp, GamepadButton::DPadUp) {
        tune.selected = (tune.selected + n - 1) % n;
    }
    if any(KeyCode::ArrowDown, GamepadButton::DPadDown) {
        tune.selected = (tune.selected + 1) % n;
    }
    let mut dir = 0i32;
    if any(KeyCode::ArrowLeft, GamepadButton::DPadLeft) { dir -= 1; }
    if any(KeyCode::ArrowRight, GamepadButton::DPadRight) { dir += 1; }
    if dir != 0 {
        let mult = if keys.pressed(KeyCode::ShiftLeft) { 5.0 } else { 1.0 };
        let sel = tune.selected;
        match &mut tune.vals[sel].value {
            Value::Num { v, min, max, step, .. } => {
                *v = (*v + *step * mult * dir as f32).clamp(*min, *max);
            }
            Value::Choice { i, opts } => {
                *i = (*i as i32 + dir).rem_euclid(opts.len() as i32) as usize;
            }
        }
        // Match length only takes effect on a reset; apply it now.
        if sel == P::MatchLength as usize {
            sim.match_left = tune.f(P::MatchLength);
        }
    }
    if keys.just_pressed(KeyCode::KeyH) {
        tune.panel = !tune.panel;
    }
    if any(KeyCode::KeyR, GamepadButton::Start) {
        sim.reset(&tune);
        sim.say("match reset");
    }
    if keys.just_pressed(KeyCode::KeyN) {
        let from = tune.guide.take().map(|g| {
            tune.vals = g.base;
            g.idx
        });
        start_next(&mut tune, &mut sim, from);
    }
    if tune.guide.is_some() {
        for (v, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4].iter().enumerate() {
            if keys.just_pressed(*key) {
                apply_variant(&mut tune, v);
                sim.reset(&tune);
            }
        }
    }
    if keys.just_pressed(KeyCode::Enter)
        && let Some(g) = tune.guide.take()
    {
        let cmp = &COMPARISONS[g.idx];
        let line = format!("row {} {}: {}", cmp.row, cmp.title, cmp.variants[g.variant].0);
        println!("verdict: {line}");
        tune.verdicts.push(line);
        tune.settled[g.idx] = true;
        for p in cmp.restore {
            tune.vals[*p as usize] = g.base[*p as usize].clone();
        }
        start_next(&mut tune, &mut sim, Some(g.idx));
    }
    if keys.just_pressed(KeyCode::KeyP) {
        println!("# squad_model settings");
        for p in &tune.vals {
            println!("{:<20} = {}", p.name, fmt_value(&p.value));
        }
        if !tune.verdicts.is_empty() {
            println!("# verdicts");
            for v in &tune.verdicts {
                println!("- {v}");
            }
        }
        if !sim.log.is_empty() {
            println!("# results");
            for l in &sim.log {
                println!("- {l}");
            }
        }
        sim.say("printed settings, verdicts and results to the terminal");
    }
}

// ---------------------------------------------------------------------------------------
// Simulation step
// ---------------------------------------------------------------------------------------

fn step(time: Res<Time>, controls: Res<Controls>, tune: Res<Tune>, mut sim: ResMut<Sim>) {
    let s = &mut *sim;
    if roster_of(&tune) != s.roster {
        s.reset(&tune);
    }
    let dt = time.delta_secs().min(1.0 / 30.0);
    for m in &mut s.msgs { m.1 -= dt; }
    s.msgs.retain(|m| m.1 > 0.0);
    if s.frozen {
        return;
    }
    // Faster-than-real-time for watching bot matches; edge inputs only on the first substep.
    let quiet = Controls {
        mv: controls.mv,
        aim: controls.aim,
        aim_point: controls.aim_point,
        aim_dist: controls.aim_dist,
        fire: controls.fire,
        kick_held: controls.kick_held,
        ..default()
    };
    for k in 0..tune.c(P::TimeScale).max(1) {
        tick(s, if k == 0 { &controls } else { &quiet }, &tune, dt);
    }
}

fn tick(s: &mut Sim, c: &Controls, tune: &Tune, dt: f32) {
    if s.over > 0.0 {
        s.over -= dt;
        if s.over <= 0.0 {
            s.reset(tune);
        }
        return;
    }
    s.t += dt;
    let t = s.t;
    // Orders land after the round trip.
    let due: Vec<(f32, usize, Job)> = s.orders.iter().filter(|o| o.0 <= t).copied().collect();
    s.orders.retain(|o| o.0 > t);
    for (_, b, job) in due {
        if let Some(a) = s.actors.get_mut(b)
            && a.alive()
        {
            a.job = job;
            a.job_t = 0.0;
            a.told = 1.0;
            a.claim = None;
        }
    }
    for i in 0..s.actors.len() {
        let a = &mut s.actors[i];
        a.fire_cd -= dt;
        a.stun -= dt;
        a.told -= dt;
        a.job_t += dt;
        a.think -= dt;
        if a.dead > 0.0 {
            a.dead -= dt;
            if a.dead <= 0.0 {
                let p = s.spawn_point(i);
                let a = &mut s.actors[i];
                a.pos = p;
                a.vel = Vec3::ZERO;
                a.hp = a.max_hp;
                a.job = Job::Default;
                a.hist.clear();
            }
        }
    }
    if tune.c(P::Seat) == 1 {
        ai_step(s, YOU, tune, dt);
    } else {
        player_step(s, c, tune, dt);
    }
    for i in 1..s.actors.len() {
        ai_step(s, i, tune, dt);
    }
    bot_orders(s, tune);
    actors_physics(s, dt);
    crates_physics(s);
    crates_step(s, dt);
    shots_step(s, dt);
    craft(s, tune, dt);
    refill(s, dt);
    for a in &mut s.actors {
        a.hist.push_back((t, a.pos));
        while a.hist.front().is_some_and(|h| h.0 < t - 0.5) {
            a.hist.pop_front();
        }
    }
    s.match_left -= dt;
    if s.match_left <= 0.0 {
        end_match(s, tune);
    }
}

fn end_match(s: &mut Sim, tune: &Tune) {
    let (b, r) = (s.score(0), s.score(1));
    let winner = match b.cmp(&r) {
        std::cmp::Ordering::Greater => "BLUE wins",
        std::cmp::Ordering::Less => "RED wins",
        std::cmp::Ordering::Equal => "draw",
    };
    let seat = if tune.c(P::Seat) == 1 { "bot" } else { "you" };
    let line = format!(
        "match: BLUE {b} - {r} RED, {winner} | {} | blue bots {}, red bots {}, blue seat 1 = {seat} | kills {}-{} | your orders {}",
        roster_desc(tune),
        ORDER_STYLES[tune.c(P::BlueBots)],
        ORDER_STYLES[tune.c(P::RedBots)],
        s.kills[0],
        s.kills[1],
        s.orders_given,
    );
    println!("{line}");
    s.log.push(line);
    s.say(format!("MATCH OVER: blue {b} - {r} red, {winner}"));
    s.over = 4.0;
}

fn player_step(s: &mut Sim, c: &Controls, tune: &Tune, dt: f32) {
    if !s.actors[YOU].alive() {
        s.pending = None;
        return;
    }
    let carrying = s.actors[YOU].carrying.is_some();
    {
        let a = &mut s.actors[YOU];
        if let Some(aim) = c.aim {
            a.facing = aim;
        } else if c.mv.length() > 0.1 {
            a.facing = Vec3::new(c.mv.x, 0.0, -c.mv.y).normalize();
        }
        let speed = MOVE_SPEED * if carrying { CARRY_SPEED } else { 1.0 };
        let mut target = Vec3::new(c.mv.x, 0.0, -c.mv.y) * speed;
        if a.stun > 0.0 { target = Vec3::ZERO; }
        let k = 1.0 - (-dt * 18.0).exp();
        let h = a.vel.with_y(0.0).lerp(target, k);
        a.vel = h.with_y(a.vel.y);
        if c.jump && a.grounded {
            a.vel.y = JUMP_VEL;
            a.grounded = false;
        }
    }

    // Grab confirmation after the round trip (the reach mask covers it).
    if let Some(p) = &mut s.pending {
        p.left -= dt;
        if p.left <= 0.0 {
            let id = p.crate_id;
            s.pending = None;
            let a_pos = s.actors[YOU].pos;
            match s.crates.iter().find(|cr| cr.id == id) {
                Some(cr) if cr.state == CrateState::Free && cr.pos.xz().distance(a_pos.xz()) <= GRAB_REACH + HALF + 0.5 => {
                    s.take(YOU, id);
                }
                _ => s.say("GRAB DENIED"),
            }
        }
    }

    player_orders(s, c, tune);

    if s.actors[YOU].stun > 0.0 {
        return;
    }
    let facing = s.actors[YOU].facing;
    let carrying = s.actors[YOU].carrying.is_some();

    if c.grab {
        if carrying {
            let plan = throw_plan(s, c.aim_dist);
            let pos = s.actors[YOU].pos;
            let at = match plan.target {
                Some(t) if t.xz().distance(pos.xz()) <= PLACE_REACH => t,
                _ => s.place_target(YOU),
            };
            s.place(YOU, at);
        } else if s.pending.is_none()
            && let Some((id, _)) = s.nearest_free(YOU, GRAB_REACH)
        {
            s.pending = Some(Pending { crate_id: id, left: GRAB_LATENCY, total: GRAB_LATENCY });
        }
    }

    if c.throw && carrying {
        let plan = throw_plan(s, c.aim_dist);
        let head = s.actors[YOU].pos + Vec3::Y * HEAD;
        s.release(YOU, plan.vel, plan.from - head);
    }

    // Full lock: no shooting while carrying.
    if c.fire && !carrying && s.actors[YOU].fire_cd <= 0.0 {
        s.actors[YOU].fire_cd = 1.0 / FIRE_RATE;
        let range = c.aim_dist.map_or(30.0, |d| d.clamp(1.0, 30.0));
        fire(s, YOU, facing, range, tune);
    }

    if !carrying {
        let pos = s.actors[YOU].pos;
        let mut opened = None;
        let mut punted = false;
        for cr in s.crates.iter_mut().filter(|cr| cr.state == CrateState::Free) {
            let near = cr.pos.xz().distance(pos.xz()) < 1.6 && cr.pos.y - pos.y < 1.5;
            match cr.kind {
                Kind::Loot if near && c.kick_held => {
                    cr.kick += dt;
                    if cr.kick >= KICK_TIME {
                        opened = Some((cr.id, cr.pos));
                    }
                }
                Kind::Loot => cr.kick = (cr.kick - dt * 0.5).max(0.0),
                Kind::Normal if near && c.kick && !punted => {
                    cr.vel = facing * 9.0 + Vec3::Y * 3.0;
                    punted = true;
                }
                _ => {}
            }
        }
        if let Some((id, at)) = opened {
            s.crates.retain(|cr| {
                if cr.id == id { s.despawn.extend(cr.ent); }
                cr.id != id
            });
            for i in 0..3 {
                let a = i as f32 * 2.1 + s.rand();
                let id = s.spawn_crate(Kind::Normal, at + Vec3::Y * 0.5);
                if let Some(cr) = s.crate_mut(id) {
                    cr.vel = Vec3::new(a.cos() * 3.0, 7.0, a.sin() * 3.0);
                }
            }
            s.loot_respawn = 6.0;
            s.say("loot crate opened: 3 crates");
        }
    }
}

/// Your orders: one context button (target decides the verb) or four verbs.
fn player_orders(s: &mut Sim, c: &Controls, tune: &Tune) {
    let wants = c.order_release.is_some() || c.verb.is_some() || c.recall;
    if !wants {
        return;
    }
    let scheme = tune.c(P::Orders);
    let mine: Vec<usize> = s.squad(YOU).filter(|&j| s.actors[j].alive()).collect();
    if scheme == 0 {
        s.say("orders are off: buddies run their default job");
        return;
    }
    if mine.is_empty() {
        s.say("no squad to order");
        return;
    }
    let delay = tune.delay();
    let p = c.aim_point.unwrap_or(s.actors[YOU].pos + s.actors[YOU].facing * 4.0).with_y(0.0);
    if c.recall {
        for &b in &mine {
            s.order(b, Job::Default, delay);
        }
        s.orders_given += 1;
        s.say(format!("RECALL: {} back to {}", mine.len(), JOBS[tune.c(P::DefaultJob)]));
        return;
    }
    match (scheme, c.order_release, c.verb) {
        (1, Some(held), _) => context_order(s, &mine, p, held >= 0.3, delay),
        (2, _, Some(v)) if v < 3 => {
            let job = [Job::Follow, Job::Haul, Job::GuardPad][v as usize];
            for &b in &mine {
                s.order(b, job, delay);
            }
            s.orders_given += 1;
            s.say(format!("order: all {}", job.label()));
        }
        (2, _, Some(_)) => context_order(s, &mine, p, true, delay),
        _ => {}
    }
}

/// The target under the reticle picks the verb: an enemy = attack, a crate = haul it,
/// your pad = guard it, ground = hold there. Tap = the nearest buddy, hold = all of them.
fn context_order(s: &mut Sim, mine: &[usize], p: Vec3, all: bool, delay: f32) {
    let team = s.actors[YOU].team;
    let enemy = (0..s.actors.len())
        .filter(|&j| s.actors[j].team != team && s.actors[j].alive())
        .filter(|&j| s.actors[j].pos.xz().distance(p.xz()) < 1.3)
        .min_by(|&x, &y| s.actors[x].pos.distance(p).total_cmp(&s.actors[y].pos.distance(p)));
    let near_crates: Vec<u32> = {
        let mut v: Vec<(u32, f32)> = s
            .crates
            .iter()
            .filter(|cr| cr.state == CrateState::Free && cr.kind == Kind::Normal && !pad_contains(PADS[team], cr.pos))
            .map(|cr| (cr.id, cr.pos.xz().distance(p.xz())))
            .filter(|(_, d)| *d < 4.0)
            .collect();
        v.sort_by(|a, b| a.1.total_cmp(&b.1));
        v.into_iter().map(|(id, _)| id).collect()
    };
    let crate_hit = near_crates.first().copied().filter(|&id| {
        s.crates.iter().any(|cr| cr.id == id && cr.pos.xz().distance(p.xz()) < 1.0)
    });
    let who: Vec<usize> = if all {
        mine.to_vec()
    } else {
        // Prefer an empty-handed buddy, then the nearest to the target.
        let pick = mine.iter().copied().min_by(|&x, &y| {
            let k = |j: usize| s.actors[j].carrying.is_some() as u32 as f32 * 100.0 + s.actors[j].pos.distance(p);
            k(x).total_cmp(&k(y))
        });
        pick.into_iter().collect()
    };
    let mut verb = "";
    for (k, &b) in who.iter().enumerate() {
        let job = if let Some(e) = enemy {
            verb = "ATTACK";
            Job::Attack(e)
        } else if crate_hit.is_some() {
            verb = "HAUL that";
            match near_crates.get(k) {
                Some(&id) => Job::HaulThis(id),
                None => Job::Haul,
            }
        } else if pad_contains(PADS[team], p) {
            verb = "GUARD PAD";
            Job::GuardPad
        } else {
            verb = "HOLD there";
            let a = k as f32 * 2.1;
            Job::Hold(p + Vec3::new(a.cos(), 0.0, a.sin()) * if k == 0 { 0.0 } else { 1.0 })
        };
        s.order(b, job, delay);
    }
    s.orders_given += 1;
    let n = who.len();
    s.say(format!("order: {verb} ({} buddy{})", n, if n == 1 { "" } else { "s" }));
}

fn fire(s: &mut Sim, who: usize, dir: Vec3, range: f32, tune: &Tune) {
    let a = &s.actors[who];
    let from = a.pos + Vec3::Y * CHEST + dir * 0.5;
    let dmg = if a.is_ai() { SHOT_DMG * tune.f(P::BuddyDamage) } else { SHOT_DMG };
    let team = a.team;
    if who == YOU {
        s.fired += 1;
    }
    s.shots.push(Shot { pos: from, vel: dir * PROJ_SPEED, owner: Some(who), team, dmg, life: range / PROJ_SPEED, ent: None });
}

// ---------------------------------------------------------------------------------------
// AI: one brain for bot leaders, buddies and haulers
// ---------------------------------------------------------------------------------------

/// What a buddy is really doing: its order, or the default job when it has none.
fn effective_job(s: &Sim, i: usize, tune: &Tune) -> Job {
    let a = &s.actors[i];
    match a.role {
        Role::Hauler => Job::Haul,
        Role::Leader => Job::Default,
        Role::Buddy(o) => {
            let j = if a.job == Job::Default { [Job::Follow, Job::Haul, Job::GuardPad][tune.c(P::DefaultJob)] } else { a.job };
            if j == Job::Follow && !s.actors[o].alive() { Job::GuardPad } else { j }
        }
    }
}

/// The nearest live enemy within `radius` of `around`, carriers first (they hold crates).
fn nearest_enemy(s: &Sim, i: usize, around: Vec3, radius: f32) -> Option<usize> {
    let team = s.actors[i].team;
    (0..s.actors.len())
        .filter(|&j| s.actors[j].team != team && s.actors[j].alive())
        .map(|j| {
            let d = s.actors[j].pos.xz().distance(around.xz());
            (j, d, d - if s.actors[j].carrying.is_some() { 4.0 } else { 0.0 })
        })
        .filter(|(_, d, _)| *d <= radius)
        .min_by(|x, y| x.2.total_cmp(&y.2))
        .map(|(j, ..)| j)
}

enum Act {
    None,
    Grab(u32),
    Place(Vec3),
}

struct Haul {
    goal: Option<Vec3>,
    act: Act,
    claim: Option<u32>,
    /// A one-off haul order is done (or the crate is gone).
    done: bool,
    /// Nothing to haul.
    idle: bool,
}

/// Fetch a crate (a given one, or the nearest unclaimed one within `leash` of the pad) and
/// set it down on the team pad. AI grabs are instant: the AI runs on the server.
fn haul_plan(s: &Sim, i: usize, only: Option<u32>, leash: Option<f32>) -> Haul {
    let a = &s.actors[i];
    let none = Haul { goal: None, act: Act::None, claim: None, done: false, idle: false };
    if a.carrying.is_some() {
        let cell = s.pad_cell(a.team, a.pos);
        if cell.xz().distance(a.pos.xz()) < 2.0 {
            return Haul { act: Act::Place(cell), done: only.is_some(), ..none };
        }
        return Haul { goal: Some(cell.with_y(0.0)), ..none };
    }
    let pad = PADS[a.team];
    let claimed = |id: u32| s.actors.iter().enumerate().any(|(j, b)| j != i && b.team == a.team && b.claim == Some(id));
    let pick = match only {
        Some(id) => s.free_crate(id).map(|c| (c.id, c.pos)),
        None => s
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && c.pos.y < 3.0)
            .filter(|c| !pad_contains(pad, c.pos))
            .filter(|c| leash.is_none_or(|l| c.pos.xz().distance(pad) <= l))
            .filter(|c| !claimed(c.id))
            .min_by(|x, y| x.pos.distance(a.pos).total_cmp(&y.pos.distance(a.pos)))
            .map(|c| (c.id, c.pos)),
    };
    match pick {
        None => Haul { done: only.is_some(), idle: only.is_none(), ..none },
        Some((id, at)) if at.xz().distance(a.pos.xz()) <= GRAB_REACH + HALF => {
            Haul { act: Act::Grab(id), claim: Some(id), ..none }
        }
        Some((id, at)) => Haul { goal: Some(at.with_y(0.0)), claim: Some(id), ..none },
    }
}

fn guard_post(s: &Sim, i: usize) -> Vec3 {
    let a = &s.actors[i];
    let pad = PADS[a.team];
    let side = (a.spot as f32 - 1.0) * 1.6 + (a.slot as f32 - 1.0) * 0.5;
    Vec3::new(pad.x + toward_centre(a.team) * 2.8, 0.0, pad.y + side)
}

fn ai_step(s: &mut Sim, i: usize, tune: &Tune, dt: f32) {
    if !s.actors[i].alive() {
        return;
    }
    let job = effective_job(s, i, tune);
    let (pos, team, role) = (s.actors[i].pos, s.actors[i].team, s.actors[i].role);
    let carrying = s.actors[i].carrying.is_some();
    let leash = tune.f(P::Leash);
    let mut goal: Option<Vec3> = None;
    let mut target: Option<usize> = None;
    let mut act = Act::None;
    let mut claim = s.actors[i].claim;
    let mut revert = false;

    let haul = |s: &Sim, only: Option<u32>, leash: Option<f32>, goal: &mut Option<Vec3>, act: &mut Act, claim: &mut Option<u32>| {
        let h = haul_plan(s, i, only, leash);
        *goal = h.goal;
        *act = h.act;
        *claim = h.claim;
        (h.done, h.idle)
    };

    match role {
        Role::Leader => {
            // Bot leader: haul, fight what's close, chase carriers to steal back.
            if carrying {
                haul(s, None, None, &mut goal, &mut act, &mut claim);
            } else if let Some(e) = nearest_enemy(s, i, pos, 8.0) {
                target = Some(e);
                let to = (s.actors[e].pos - pos).with_y(0.0).normalize_or_zero();
                let side = Vec3::new(-to.z, 0.0, to.x) * (s.t * 0.7 + i as f32).sin().signum();
                goal = Some(pos + side * 2.0);
            } else if let Some(e) = nearest_enemy(s, i, pos, 14.0).filter(|&e| s.actors[e].carrying.is_some()) {
                target = Some(e);
                goal = Some(s.actors[e].pos);
            } else {
                let (_, idle) = haul(s, None, None, &mut goal, &mut act, &mut claim);
                if idle {
                    goal = Some(guard_post(s, i));
                }
            }
        }
        _ => match job {
            Job::Haul => {
                let (_, idle) = haul(s, None, Some(leash), &mut goal, &mut act, &mut claim);
                if idle {
                    // Nothing in range: guard the pad meanwhile.
                    goal = Some(guard_post(s, i));
                    target = nearest_enemy(s, i, PADS[team].extend(0.0).xzy(), 10.0);
                } else if !carrying {
                    target = nearest_enemy(s, i, pos, 6.0);
                }
            }
            Job::HaulThis(id) => {
                let (done, _) = haul(s, Some(id), None, &mut goal, &mut act, &mut claim);
                revert = done || s.actors[i].job_t > 20.0;
            }
            Job::Follow => {
                let Role::Buddy(o) = role else { unreachable!() };
                let l = &s.actors[o];
                if carrying {
                    haul(s, None, None, &mut goal, &mut act, &mut claim);
                } else {
                    let n = s.roster[2].max(1) as f32;
                    let ang = PI + (s.actors[i].spot as f32 - (n - 1.0) / 2.0) * 0.8;
                    let spot = l.pos + Quat::from_rotation_y(ang) * l.facing * 2.0;
                    if spot.xz().distance(pos.xz()) > 0.6 {
                        goal = Some(spot);
                    }
                    target = nearest_enemy(s, i, pos, 9.0);
                }
            }
            Job::GuardPad => {
                if carrying {
                    haul(s, None, None, &mut goal, &mut act, &mut claim);
                } else {
                    goal = Some(guard_post(s, i));
                    target = nearest_enemy(s, i, PADS[team].extend(0.0).xzy(), 10.0);
                }
            }
            Job::Attack(e) => {
                if !s.actors[e].alive() || s.actors[i].job_t > 15.0 || carrying {
                    revert = true;
                } else {
                    let d = s.actors[e].pos.xz().distance(pos.xz());
                    if d > 6.0 {
                        goal = Some(s.actors[e].pos);
                    }
                    target = Some(e);
                }
            }
            Job::Hold(p) => {
                if s.actors[i].job_t > 20.0 {
                    revert = true;
                }
                if p.xz().distance(pos.xz()) > 0.5 {
                    goal = Some(p);
                }
                target = nearest_enemy(s, i, pos, 8.0);
            }
            Job::Default => {}
        },
    }
    // Buddies stay on a leash to their leader while following or holding.
    if let Role::Buddy(o) = role
        && matches!(job, Job::Follow | Job::Hold(_))
        && s.actors[o].alive()
        && let Some(g) = goal
    {
        let l = s.actors[o].pos;
        if g.xz().distance(l.xz()) > leash {
            goal = Some(l + (g - l).with_y(0.0).normalize_or_zero() * leash);
        }
    }

    let speed = MOVE_SPEED
        * match role {
            Role::Leader => 0.85,
            Role::Buddy(_) => 0.9,
            Role::Hauler => 0.8,
        }
        * if carrying { CARRY_SPEED } else { 1.0 };
    let spread = (s.rand() - 0.5) * 0.15;
    {
        let a = &mut s.actors[i];
        a.claim = claim;
        if revert {
            a.job = Job::Default;
        }
        let want = match goal.map(|g| (g - a.pos).with_y(0.0)) {
            Some(v) if v.length() > 0.4 && a.stun <= 0.0 => v.normalize() * speed,
            _ => Vec3::ZERO,
        };
        let k = 1.0 - (-dt * 12.0).exp();
        a.vel = a.vel.with_y(0.0).lerp(want, k).with_y(a.vel.y);
        if want.length() > 0.1 {
            a.facing = want.normalize();
        }
    }
    if let Some(e) = target {
        let (ep, ev) = (s.actors[e].pos, s.actors[e].vel.with_y(0.0));
        let d = ep.xz().distance(pos.xz());
        // Lead the target, then miss a little.
        let lead = ep + ev * (d / PROJ_SPEED);
        let dir = Quat::from_rotation_y(spread) * (lead - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        let rate = match role {
            Role::Leader => 2.0,
            Role::Buddy(_) => 1.5,
            Role::Hauler => 1.0,
        };
        if !carrying && d < 12.0 && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            s.actors[i].fire_cd = 1.0 / rate;
            fire(s, i, dir, 14.0, tune);
        }
    }
    match act {
        Act::Grab(id) => s.take(i, id),
        Act::Place(at) => {
            s.place(i, at);
            if matches!(s.actors[i].job, Job::HaulThis(_)) {
                s.actors[i].job = Job::Default;
            }
        }
        Act::None => {}
    }
}

/// Bot leaders set to "micro" re-order their squad every 2.5 s: attack a threat to the pad
/// or to the leader, else send each buddy for a different crate anywhere (stealing included).
fn bot_orders(s: &mut Sim, tune: &Tune) {
    let delay = tune.delay();
    let seat_bot = tune.c(P::Seat) == 1;
    for l in 0..s.actors.len() {
        let leader = &s.actors[l];
        if leader.role != Role::Leader || !leader.alive() || (l == YOU && !seat_bot) || leader.think > 0.0 {
            continue;
        }
        let team = leader.team;
        if tune.c(if team == 0 { P::BlueBots } else { P::RedBots }) == 0 {
            continue;
        }
        s.actors[l].think = 2.5;
        let squad: Vec<usize> = s.squad(l).filter(|&j| s.actors[j].alive()).collect();
        if squad.is_empty() {
            continue;
        }
        let pad = PADS[team];
        let lpos = s.actors[l].pos;
        let threat = (0..s.actors.len())
            .filter(|&j| s.actors[j].team != team && s.actors[j].alive())
            .filter(|&j| {
                let e = &s.actors[j];
                (e.carrying.is_some() && e.pos.xz().distance(pad) < 12.0) || e.pos.xz().distance(lpos.xz()) < 7.0
            })
            .min_by(|&x, &y| s.actors[x].pos.xz().distance(pad).total_cmp(&s.actors[y].pos.xz().distance(pad)));
        let mut taken: Vec<u32> = s.actors.iter().filter(|a| a.team == team).filter_map(|a| a.claim).collect();
        for b in squad {
            if s.actors[b].carrying.is_some() || s.orders.iter().any(|o| o.1 == b) {
                continue;
            }
            let job = match threat {
                Some(e) => Job::Attack(e),
                None => {
                    if let Job::HaulThis(id) = s.actors[b].job
                        && s.free_crate(id).is_some()
                    {
                        continue;
                    }
                    let bp = s.actors[b].pos;
                    let pick = s
                        .crates
                        .iter()
                        .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && c.pos.y < 3.0)
                        .filter(|c| !pad_contains(pad, c.pos) && !taken.contains(&c.id))
                        .min_by(|x, y| x.pos.distance(bp).total_cmp(&y.pos.distance(bp)))
                        .map(|c| c.id);
                    match pick {
                        Some(id) => {
                            taken.push(id);
                            Job::HaulThis(id)
                        }
                        None => Job::Default,
                    }
                }
            };
            if s.actors[b].job != job {
                s.order(b, job, delay);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// Physics
// ---------------------------------------------------------------------------------------

fn actors_physics(s: &mut Sim, dt: f32) {
    let blockers: Vec<Vec3> = s
        .crates
        .iter()
        .filter(|c| c.state == CrateState::Free && c.pos.y < CRATE * 1.5)
        .map(|c| c.pos)
        .chain(s.turrets.iter().map(|t| t.pos + Vec3::Y * HALF))
        .collect();
    for a in s.actors.iter_mut().filter(|a| a.alive()) {
        a.vel.y -= G * dt;
        a.pos += a.vel * dt;
        if a.pos.y <= 0.0 {
            a.pos.y = 0.0;
            a.vel.y = 0.0;
            a.grounded = true;
        } else {
            a.grounded = false;
        }
        for b in &blockers {
            let d = a.pos.xz() - b.xz();
            let min = ACTOR_R + HALF;
            if d.length() < min && a.pos.y < b.y + HALF - 0.1 {
                let n = d.normalize_or(Vec2::X);
                let p = b.xz() + n * min;
                a.pos.x = p.x;
                a.pos.z = p.y;
            }
        }
    }
    // Bodies don't overlap each other.
    let n = s.actors.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if !s.actors[i].alive() || !s.actors[j].alive() {
                continue;
            }
            let d = s.actors[j].pos.xz() - s.actors[i].pos.xz();
            let min = ACTOR_R * 1.8;
            if d.length() < min {
                let n = d.normalize_or(Vec2::X) * (min - d.length()) * 0.5;
                s.actors[i].pos -= n.extend(0.0).xzy();
                s.actors[j].pos += n.extend(0.0).xzy();
            }
        }
    }
    for a in &mut s.actors {
        a.pos.x = a.pos.x.clamp(-ARENA.x + ACTOR_R, ARENA.x - ACTOR_R);
        a.pos.z = a.pos.z.clamp(-ARENA.y + ACTOR_R, ARENA.y - ACTOR_R);
    }
}

/// Carried crates ride on the carrier's head.
fn crates_physics(s: &mut Sim) {
    for c in &mut s.crates {
        if let CrateState::Carried { by, lift_from, lift_t } = &mut c.state {
            let head = s.actors[*by].pos + Vec3::Y * HEAD;
            c.pos = lift_from.lerp(head, (*lift_t / 0.1).min(1.0));
        }
    }
}

fn crates_step(s: &mut Sim, dt: f32) {
    let snapshot: Vec<(u32, Vec3, bool)> =
        s.crates.iter().map(|c| (c.id, c.pos, c.state == CrateState::Free)).collect();
    let turrets: Vec<Vec2> = s.turrets.iter().map(|t| t.pos.xz()).collect();
    let mut thrown_hits = Vec::new();
    for c in &mut s.crates {
        match &mut c.state {
            CrateState::Carried { lift_t, .. } => *lift_t += dt,
            CrateState::Free => {
                let me = c.id;
                let others = || snapshot.iter().filter(move |(id, _, free)| *id != me && *free).map(|(_, p, _)| *p);
                let grounded = crate_motion(
                    &mut c.pos,
                    &mut c.vel,
                    &mut c.settling,
                    |q| floor_under(q, others(), turrets.iter().copied()),
                    dt,
                );
                if grounded {
                    c.thrown_by = None;
                }
                if let Some(by) = c.thrown_by {
                    let team = s.actors.get(by).map_or(0, |a| a.team);
                    let hit = s.actors.iter().position(|o| {
                        o.team != team
                            && o.alive()
                            && c.pos.xz().distance(o.pos.xz()) < ACTOR_R + HALF
                            && c.pos.y < o.pos.y + 2.0
                    });
                    if let Some(j) = hit {
                        thrown_hits.push((j, c.vel.with_y(0.0).normalize_or_zero(), by, team));
                        c.thrown_by = None;
                        c.vel *= -0.2;
                    }
                }
            }
        }
    }
    // Curling knocks.
    let n = s.crates.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, b) = (&s.crates[i], &s.crates[j]);
            if a.state != CrateState::Free || b.state != CrateState::Free || !side_hit(a.pos, b.pos) {
                continue;
            }
            let rel = a.vel.xz() - b.vel.xz();
            if rel.length() < 0.3 {
                continue;
            }
            let d = b.pos.xz() - a.pos.xz();
            let (axis, pen) = if (CRATE - d.x.abs()) < (CRATE - d.y.abs()) {
                (Vec2::new(d.x.signum(), 0.0), CRATE - d.x.abs())
            } else {
                (Vec2::new(0.0, d.y.signum()), CRATE - d.y.abs())
            };
            let closing = rel.dot(axis);
            let push = (axis * pen * 0.5).extend(0.0).xzy();
            s.crates[i].pos -= push;
            s.crates[j].pos += push;
            if closing > 0.0 {
                let dv = (axis * closing * 0.9).extend(0.0).xzy();
                s.crates[i].vel -= dv;
                s.crates[j].vel += dv;
                s.crates[i].settling = false;
                s.crates[j].settling = false;
            }
        }
    }
    for (who, dir, by, team) in thrown_hits {
        if by == YOU {
            s.hits += 1;
        }
        s.hurt(who, SHOT_DMG * 2.0, dir * 6.0, Some(by), team);
    }
}

fn shots_step(s: &mut Sim, dt: f32) {
    let mut hits = Vec::new();
    for sh in &mut s.shots {
        sh.pos += sh.vel * dt;
        sh.life -= dt;
        // Hitboxes don't scale with the buddy look: it's readability only.
        let hit = s.actors.iter().position(|a| {
            a.team != sh.team
                && a.alive()
                && sh.pos.xz().distance(a.pos.xz()) < ACTOR_R + 0.15
                && sh.pos.y > a.pos.y
                && sh.pos.y < a.pos.y + 1.9
        });
        if let Some(j) = hit {
            hits.push((j, sh.vel.normalize(), sh.owner, sh.team, sh.dmg));
            sh.life = 0.0;
        }
    }
    s.shots.retain(|sh| {
        if sh.life <= 0.0 { s.despawn.extend(sh.ent); }
        sh.life > 0.0
    });
    for (who, dir, owner, team, dmg) in hits {
        if owner == Some(YOU) {
            s.hits += 1;
        }
        s.hurt(who, dmg, dir * 1.5, owner, team);
    }
}

/// PLACEHOLDER (not a grammar decision, see #17): four resting crates on a team's pad in a
/// 2x2 block on the same level merge into a turret for that team.
fn craft(s: &mut Sim, tune: &Tune, dt: f32) {
    for k in 0..s.turrets.len() {
        s.turrets[k].life -= dt;
        s.turrets[k].cd -= dt;
        let (team, tpos) = (s.turrets[k].team, s.turrets[k].pos);
        let enemy = (0..s.actors.len())
            .filter(|&j| s.actors[j].team != team && s.actors[j].alive())
            .map(|j| (j, s.actors[j].pos.xz().distance(tpos.xz())))
            .filter(|(_, d)| *d < 14.0)
            .min_by(|x, y| x.1.total_cmp(&y.1));
        if let Some((j, _)) = enemy {
            let t = &mut s.turrets[k];
            t.facing = (s.actors[j].pos - t.pos).with_y(0.0).normalize_or(Vec3::X);
            if t.cd <= 0.0 {
                t.cd = 0.8;
                let from = t.pos + Vec3::Y * 1.2 + t.facing * 0.6;
                let vel = t.facing * PROJ_SPEED;
                s.shots.push(Shot { pos: from, vel, owner: None, team, dmg: SHOT_DMG, life: 1.0, ent: None });
            }
        }
    }
    s.turrets.retain(|t| {
        if t.life <= 0.0 { s.despawn.extend(t.ent); }
        t.life > 0.0
    });
    if tune.c(P::Craft) != 0 {
        return;
    }
    for team in 0..2 {
        let cells: Vec<(IVec3, u32)> = s
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && c.vel.length() < 0.1)
            .filter(|c| pad_contains(PADS[team], c.pos))
            .filter(|c| (c.pos.xz() / CRATE - (c.pos.xz() / CRATE).round()).length() < 0.05)
            .map(|c| ((c.pos / CRATE - Vec3::Y * 0.5).round().as_ivec3(), c.id))
            .collect();
        let at = |g: IVec3| cells.iter().find(|(k, _)| *k == g).map(|(_, id)| *id);
        for (g, _) in &cells {
            let block = [*g, *g + IVec3::X, *g + IVec3::Z, *g + IVec3::X + IVec3::Z].map(at);
            let centre = (g.as_vec3() + Vec3::new(0.5, 0.0, 0.5)) * CRATE;
            let occupied = s.turrets.iter().any(|t| t.pos.xz().distance(centre.xz()) < 1.1);
            if block.iter().all(Option::is_some) && !occupied {
                let ids: Vec<u32> = block.into_iter().flatten().collect();
                s.crates.retain(|c| {
                    let used = ids.contains(&c.id);
                    if used { s.despawn.extend(c.ent); }
                    !used
                });
                s.turrets.push(Turret {
                    team,
                    pos: Vec3::new(centre.x, 0.0, centre.z),
                    facing: Vec3::X,
                    cd: 0.5,
                    life: 40.0,
                    ent: None,
                });
                s.say(format!("{} built a TURRET (placeholder, 40 s)", ["blue", "red"][team]));
                return;
            }
        }
    }
}

fn refill(s: &mut Sim, dt: f32) {
    let loose = s
        .crates
        .iter()
        .filter(|c| c.kind == Kind::Normal)
        .filter(|c| c.state != CrateState::Free || !(pad_contains(PADS[0], c.pos) || pad_contains(PADS[1], c.pos)))
        .count();
    if loose < 3 + 2 * s.roster[1] {
        let x = -7.0 + s.rand() * 14.0;
        let z = -9.0 + s.rand() * 18.0;
        s.spawn_crate(Kind::Normal, Vec3::new(x, 10.0, z));
    }
    if s.crates.iter().filter(|c| c.kind == Kind::Loot).count() < 2 {
        s.loot_respawn -= dt;
        if s.loot_respawn <= 0.0 {
            let x = -3.0 + s.rand() * 6.0;
            let z = -7.0 + s.rand() * 14.0;
            s.spawn_crate(Kind::Loot, Vec3::new(x, 10.0, z));
            s.loot_respawn = 6.0;
        }
    }
}

// ---------------------------------------------------------------------------------------
// Readability test: freeze-frame, highlight a unit, click its owner
// ---------------------------------------------------------------------------------------

#[derive(Resource, Default)]
struct ReadTest {
    on: bool,
    /// 0 = playing between questions, 1 = asking (frozen), 2 = showing the answer (frozen).
    phase: u8,
    timer: f32,
    target: Option<usize>,
    asked: f32,
    answers: Vec<(bool, f32)>,
    right: bool,
    answer_at: Vec3,
    auto_started: bool,
}

fn read_test(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    tune: Res<Tune>,
    mut sim: ResMut<Sim>,
    mut rt: ResMut<ReadTest>,
) {
    let now = time.elapsed_secs();
    let auto = std::env::var("SQUAD_READTEST").is_ok() && !rt.auto_started && now > 3.0;
    if keys.just_pressed(KeyCode::KeyT) || auto {
        let was_on = rt.on;
        *rt = ReadTest { auto_started: true, ..default() };
        sim.frozen = false;
        if was_on {
            sim.say("readability test stopped");
            return;
        }
        rt.on = true;
        rt.timer = now;
    }
    if !rt.on {
        return;
    }
    let (cam, cam_tf) = *camera;
    match rt.phase {
        0 => {
            sim.frozen = false;
            if now < rt.timer {
                return;
            }
            // Any unit but you that's alive and on screen; two times in three, an AI one.
            let size = window.size();
            let on_screen = |i: usize| {
                cam.world_to_viewport(cam_tf, shown_at(&sim, i, &tune) + Vec3::Y)
                    .is_ok_and(|p| p.x > 60.0 && p.y > 60.0 && p.x < size.x - 60.0 && p.y < size.y - 60.0)
            };
            let seat_bot = tune.c(P::Seat) == 1;
            let cands: Vec<usize> = (0..sim.actors.len())
                .filter(|&i| (i != YOU || seat_bot) && sim.actors[i].alive() && on_screen(i))
                .collect();
            let ai: Vec<usize> = cands.iter().copied().filter(|&i| sim.actors[i].is_ai()).collect();
            let pool = if !ai.is_empty() && sim.rand() < 0.67 { ai } else { cands };
            if pool.is_empty() {
                rt.timer = now + 0.5;
                return;
            }
            let k = ((sim.rand() * pool.len() as f32) as usize).min(pool.len() - 1);
            rt.target = Some(pool[k]);
            rt.phase = 1;
            rt.asked = now;
            sim.frozen = true;
        }
        1 => {
            let Some(t) = rt.target else { return };
            if !mouse.just_pressed(MouseButton::Left) {
                return;
            }
            let Some(p) = window
                .cursor_position()
                .and_then(|c| cam.viewport_to_world(cam_tf, c).ok())
                .and_then(|ray| ray.intersect_plane(Vec3::Y, InfinitePlane3d::new(Vec3::Y)).map(|d| ray.get_point(d)))
            else {
                return;
            };
            let clicked = (0..sim.actors.len())
                .filter(|&j| sim.actors[j].alive())
                .map(|j| (j, shown_at(&sim, j, &tune).xz().distance(p.xz())))
                .filter(|(_, d)| *d < 0.9)
                .min_by(|x, y| x.1.total_cmp(&y.1))
                .map(|(j, _)| j);
            let tgt = &sim.actors[t];
            let (right, at) = match tgt.role {
                Role::Leader => (clicked == Some(t), shown_at(&sim, t, &tune)),
                Role::Buddy(o) => (clicked == Some(o), shown_at(&sim, o, &tune)),
                Role::Hauler => {
                    let pad = PADS[tgt.team];
                    (clicked.is_none() && pad_contains(pad, p.with_y(0.0)), pad.extend(0.0).xzy())
                }
            };
            let took = now - rt.asked;
            rt.answers.push((right, took));
            rt.right = right;
            rt.answer_at = at;
            rt.phase = 2;
            rt.timer = now + 1.2;
            let what = sim.describe(t);
            sim.say(format!("{} in {took:.2} s: it was {what}", if right { "RIGHT" } else { "WRONG" }));
        }
        _ => {
            if now < rt.timer {
                return;
            }
            sim.frozen = false;
            if rt.answers.len() < READ_QUESTIONS {
                rt.phase = 0;
                rt.timer = now + 1.5;
                return;
            }
            let n = rt.answers.len();
            let right = rt.answers.iter().filter(|a| a.0).count();
            let fast = rt.answers.iter().filter(|a| a.0 && a.1 < 1.0).count();
            let mut times: Vec<f32> = rt.answers.iter().map(|a| a.1).collect();
            times.sort_by(f32::total_cmp);
            let median = times[n / 2];
            let bodies = sim.actors.len();
            let line = format!(
                "readability: {right}/{n} right, {fast}/{n} right in under 1 s, median {median:.2} s | {} | {} ({bodies} bodies)",
                looks_desc(&tune),
                roster_desc(&tune),
            );
            println!("{line}");
            sim.log.push(line);
            sim.say(format!("readability: {right}/{n} right, {fast} under 1 s, median {median:.2} s"));
            *rt = ReadTest { auto_started: true, ..default() };
        }
    }
}

// ---------------------------------------------------------------------------------------
// Presentation
// ---------------------------------------------------------------------------------------

#[derive(Resource)]
struct Looks {
    crate_mesh: Handle<Mesh>,
    normal: Handle<StandardMaterial>,
    loot: Handle<StandardMaterial>,
    shot_mesh: Handle<Mesh>,
    shot_mat: [Handle<StandardMaterial>; 2],
    turret_base: Handle<Mesh>,
    turret_barrel: Handle<Mesh>,
    body: Handle<Mesh>,
    nose: Handle<Mesh>,
    band: Handle<Mesh>,
    halo: Handle<Mesh>,
    team: [Handle<StandardMaterial>; 2],
    hauler: [Handle<StandardMaterial>; 2],
    owner: [Handle<StandardMaterial>; 4],
    halo_mat: Handle<StandardMaterial>,
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Panel;

#[derive(Component)]
struct GuideText;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_translation(CAM_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight { illuminance: 9_000.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(8.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(ARENA.x * 2.0, 0.1, ARENA.y * 2.0))),
        MeshMaterial3d(mats.add(Color::srgb(0.32, 0.36, 0.3))),
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));
    for (pad, color) in PADS.iter().zip(TEAM_COLOR) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(PAD_HALF * 2.0, 0.02, PAD_HALF * 2.0))),
            MeshMaterial3d(mats.add(color.with_alpha(0.6))),
            Transform::from_xyz(pad.x, 0.01, pad.y),
        ));
    }
    let dark = |c: Color| {
        let l = c.to_linear();
        Color::linear_rgb(l.red * 0.35, l.green * 0.35, l.blue * 0.35)
    };
    commands.insert_resource(Looks {
        crate_mesh: meshes.add(Cuboid::from_length(CRATE)),
        normal: mats.add(Color::srgb(0.75, 0.6, 0.35)),
        loot: mats.add(Color::srgb(1.0, 0.5, 0.05)),
        shot_mesh: meshes.add(Sphere::new(0.15)),
        shot_mat: [
            mats.add(StandardMaterial { emissive: LinearRgba::rgb(0.6, 1.4, 3.0), ..default() }),
            mats.add(StandardMaterial { emissive: LinearRgba::rgb(3.0, 0.6, 0.3), ..default() }),
        ],
        turret_base: meshes.add(Cylinder::new(0.7, 0.8)),
        turret_barrel: meshes.add(Cuboid::new(0.25, 0.25, 1.2)),
        body: meshes.add(Capsule3d::new(ACTOR_R, 1.0)),
        nose: meshes.add(Cuboid::new(0.25, 0.25, 0.4)),
        band: meshes.add(Cylinder::new(ACTOR_R + 0.06, 0.22)),
        halo: meshes.add(Torus::new(0.3, 0.42)),
        team: TEAM_COLOR.map(|c| mats.add(c)),
        hauler: TEAM_COLOR.map(|c| mats.add(dark(c))),
        owner: OWNER_COLOR.map(|c| mats.add(StandardMaterial { base_color: c, emissive: c.to_linear() * 0.4, ..default() })),
        halo_mat: mats.add(StandardMaterial { base_color: Color::WHITE, emissive: LinearRgba::rgb(2.0, 2.0, 2.0), ..default() }),
    });
    commands.spawn((
        Hud,
        Text::new(""),
        TextFont::from_font_size(15.0),
        Node { position_type: PositionType::Absolute, bottom: px(10.0), left: px(10.0), ..default() },
        BackgroundColor(Color::BLACK.with_alpha(0.5)),
    ));
    commands.spawn((
        GuideText,
        Text::new(""),
        TextFont::from_font_size(20.0),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            padding: UiRect::all(px(10.0)),
            max_width: percent(55.0),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.7)),
    ));
    commands.spawn((
        Panel,
        Text::new(""),
        TextFont::from_font_size(14.0),
        Node { position_type: PositionType::Absolute, top: px(10.0), right: px(10.0), padding: UiRect::all(px(8.0)), ..default() },
        BackgroundColor(Color::BLACK.with_alpha(0.6)),
    ));
}

fn sync_views(
    mut commands: Commands,
    mut sim: ResMut<Sim>,
    tune: Res<Tune>,
    looks: Res<Looks>,
    mut q: Query<(&mut Transform, &mut Visibility)>,
) {
    for e in sim.despawn.drain(..) {
        commands.entity(e).despawn();
    }
    // Rebuild every unit's body when the look changes.
    let band = tune.c(P::OwnerBand) == 1;
    let halo = tune.c(P::HumanHalo) == 1;
    let scale = tune.f(P::BuddyScale);
    let key = band as u64 | (halo as u64) << 1 | ((scale * 100.0) as u64) << 2;
    if key != sim.view_key {
        sim.view_key = key;
        for a in &mut sim.actors {
            if let Some(e) = a.ent.take() {
                commands.entity(e).despawn();
            }
        }
    }
    let shown_pos: Vec<Vec3> = (0..sim.actors.len()).map(|i| shown_at(&sim, i, &tune)).collect();
    for (i, a) in sim.actors.iter_mut().enumerate() {
        let rot = Transform::IDENTITY.looking_to(a.facing, Vec3::Y).rotation;
        match a.ent {
            Some(e) => {
                if let Ok((mut tf, mut vis)) = q.get_mut(e) {
                    tf.translation = shown_pos[i];
                    tf.rotation = rot;
                    *vis = if a.alive() { Visibility::Visible } else { Visibility::Hidden };
                }
            }
            None => {
                let s = if a.is_ai() { scale } else { 1.0 };
                let body = if a.role == Role::Hauler { looks.hauler[a.team].clone() } else { looks.team[a.team].clone() };
                let owner = looks.owner[a.slot % 4].clone();
                let (bm, nm, band_m, halo_m, halo_mat) =
                    (looks.body.clone(), looks.nose.clone(), looks.band.clone(), looks.halo.clone(), looks.halo_mat.clone());
                let with_band = band && a.role != Role::Hauler;
                let with_halo = halo && a.role == Role::Leader;
                a.ent = Some(
                    commands
                        .spawn((
                            Transform::from_translation(shown_pos[i]).with_rotation(rot).with_scale(Vec3::splat(s)),
                            Visibility::default(),
                        ))
                        .with_children(|p| {
                            p.spawn((Mesh3d(bm), MeshMaterial3d(body.clone()), Transform::from_xyz(0.0, 0.95, 0.0)));
                            p.spawn((Mesh3d(nm), MeshMaterial3d(body), Transform::from_xyz(0.0, 1.3, -0.5)));
                            if with_band {
                                p.spawn((Mesh3d(band_m), MeshMaterial3d(owner), Transform::from_xyz(0.0, 1.0, 0.0)));
                            }
                            if with_halo {
                                p.spawn((Mesh3d(halo_m), MeshMaterial3d(halo_mat), Transform::from_xyz(0.0, 2.95, 0.0)));
                            }
                        })
                        .id(),
                );
            }
        }
    }
    for c in &mut sim.crates {
        match c.ent {
            Some(e) => {
                if let Ok((mut tf, _)) = q.get_mut(e) {
                    tf.translation = c.pos;
                }
            }
            None => {
                let mat = if c.kind == Kind::Loot { looks.loot.clone() } else { looks.normal.clone() };
                c.ent = Some(
                    commands
                        .spawn((Mesh3d(looks.crate_mesh.clone()), MeshMaterial3d(mat), Transform::from_translation(c.pos)))
                        .id(),
                );
            }
        }
    }
    for t in &mut sim.turrets {
        let rot = Transform::IDENTITY.looking_to(t.facing, Vec3::Y).rotation;
        match t.ent {
            Some(e) => {
                if let Ok((mut tf, _)) = q.get_mut(e) {
                    tf.rotation = rot;
                }
            }
            None => {
                let (base, barrel, mat) = (looks.turret_base.clone(), looks.turret_barrel.clone(), looks.team[t.team].clone());
                t.ent = Some(
                    commands
                        .spawn((Transform::from_translation(t.pos).with_rotation(rot), Visibility::default()))
                        .with_children(|p| {
                            p.spawn((Mesh3d(base), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.0, 0.4, 0.0)));
                            p.spawn((Mesh3d(barrel), MeshMaterial3d(mat), Transform::from_xyz(0.0, 0.9, -0.5)));
                        })
                        .id(),
                );
            }
        }
    }
    for sh in &mut sim.shots {
        match sh.ent {
            Some(e) => {
                if let Ok((mut tf, _)) = q.get_mut(e) {
                    tf.translation = sh.pos;
                }
            }
            None => {
                let mat = looks.shot_mat[sh.team].clone();
                sh.ent = Some(
                    commands
                        .spawn((Mesh3d(looks.shot_mesh.clone()), MeshMaterial3d(mat), Transform::from_translation(sh.pos)))
                        .id(),
                );
            }
        }
    }
}

fn follow_camera(time: Res<Time>, sim: Res<Sim>, tune: Res<Tune>, mut cam: Single<&mut Transform, With<MainCamera>>) {
    let k = 1.0 - (-time.delta_secs() * 6.0).exp();
    if tune.c(P::Camera) == 1 {
        cam.translation = cam.translation.lerp(ARENA_CAM, k);
        let look = cam.translation - ARENA_CAM;
        cam.look_at(look, Vec3::Y);
        return;
    }
    let want = sim.actors[YOU].pos.with_y(0.0) + CAM_OFFSET;
    cam.translation = cam.translation.lerp(want, k);
    let look = cam.translation - CAM_OFFSET;
    cam.look_at(look, Vec3::Y);
}

fn ground_circle(p: Vec3) -> Isometry3d {
    Isometry3d::new(p, Quat::from_rotation_x(FRAC_PI_2))
}

fn draw_gizmos(mut g: Gizmos, sim: Res<Sim>, tune: Res<Tune>, controls: Res<Controls>, rt: Res<ReadTest>, time: Res<Time>) {
    let you_play = tune.c(P::Seat) == 0;
    let a = &sim.actors[YOU];
    let pos_of = |i: usize| shown_at(&sim, i, &tune);
    if you_play && a.alive() {
        let chest = a.pos + Vec3::Y * CHEST;
        g.line(chest, chest + a.facing * 8.0, Color::srgba(1.0, 1.0, 1.0, 0.5));
        let plan = a.carrying.map(|_| throw_plan(&sim, controls.aim_dist));
        if let Some(p) = controls.aim_point
            && plan.as_ref().is_none_or(|p| p.target.is_none())
        {
            g.circle(ground_circle(p), 0.35, Color::srgb(1.0, 0.3, 0.3));
            g.circle(ground_circle(p.with_y(0.03)), 0.35, Color::srgba(1.0, 0.3, 0.3, 0.4));
            g.line(p.with_y(0.03), p, Color::srgba(1.0, 0.3, 0.3, 0.4));
        }
        if a.carrying.is_none() {
            g.circle(ground_circle(a.pos + Vec3::Y * 0.03), GRAB_REACH + HALF, Color::srgba(1.0, 1.0, 1.0, 0.15));
            if sim.pending.is_none()
                && let Some((id, _)) = sim.nearest_free(YOU, GRAB_REACH)
                && let Some(c) = sim.crates.iter().find(|c| c.id == id)
            {
                g.cube(Transform::from_translation(c.pos).with_scale(Vec3::splat(CRATE * 1.08)), Color::WHITE);
            }
        }
        // Reach mask over the grab round trip.
        if let Some(p) = &sim.pending
            && let Some(c) = sim.crates.iter().find(|c| c.id == p.crate_id)
        {
            let progress = 1.0 - (p.left / p.total).clamp(0.0, 1.0);
            let hand = chest.lerp(c.pos, progress);
            g.line(chest, hand, TEAM_COLOR[0]);
            g.sphere(Isometry3d::from_translation(hand), 0.15, TEAM_COLOR[0]);
        }
        if let Some(plan) = &plan {
            let ghost = |g: &mut Gizmos, at: Vec3, color: Color| {
                g.cube(Transform::from_translation(at).with_scale(Vec3::splat(CRATE)), color);
            };
            let near = sim.place_target(YOU);
            match plan.target {
                Some(t) => {
                    g.circle(ground_circle(t.with_y(t.y - HALF + 0.03)), 0.55, Color::srgb(0.6, 1.0, 1.0));
                    g.circle(ground_circle(t.with_y(t.y - HALF + 0.03)), 0.1, Color::srgb(0.6, 1.0, 1.0));
                    let color = if plan.knock { Color::srgb(1.0, 0.55, 0.1) } else { Color::srgb(0.6, 1.0, 1.0) };
                    ghost(&mut g, plan.landing.with_y(plan.landing.y.max(HALF)), color);
                    if t.xz().distance(a.pos.xz()) > PLACE_REACH {
                        ghost(&mut g, near, Color::srgba(0.6, 1.0, 1.0, 0.25));
                    }
                }
                None => ghost(&mut g, near, Color::srgb(0.6, 1.0, 1.0)),
            }
            let color = if plan.short { Color::srgb(1.0, 0.4, 0.2) } else { Color::srgb(0.4, 1.0, 0.4) };
            g.linestrip(plan.path.iter().copied(), color);
        }
        // Context order: a ring at the reticle that fills while held (full = whole squad).
        if tune.c(P::Orders) == 1
            && let (Some(t0), Some(p)) = (controls.order_down, controls.aim_point)
        {
            let held = ((time.elapsed_secs() - t0) / 0.3).min(1.0);
            let color = if held >= 1.0 { OWNER_COLOR[0] } else { Color::WHITE };
            g.circle(ground_circle(p.with_y(0.05)), 0.4 + 0.6 * held, color);
        }
    }
    let kt = KICK_TIME;
    for c in sim.crates.iter().filter(|c| c.kind == Kind::Loot && c.kick > 0.0) {
        g.circle(ground_circle(c.pos.with_y(0.04)), 0.2 + 0.6 * (c.kick / kt).min(1.0), Color::srgb(1.0, 0.7, 0.1));
    }
    for pad in PADS {
        g.rect(
            Isometry3d::new(pad.extend(0.03).xzy(), Quat::from_rotation_x(FRAC_PI_2)),
            Vec2::splat(PAD_HALF * 2.0),
            Color::WHITE,
        );
    }
    let tethers = tune.c(P::Tethers);
    let lines = tune.c(P::OrderLines) == 1;
    for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive()) {
        let at = pos_of(i);
        let owner_c = OWNER_COLOR[b.slot % 4];
        // Damage shows as a bar over the head.
        if b.hp < b.max_hp {
            let s = if b.is_ai() { tune.f(P::BuddyScale) } else { 1.0 };
            let top = at + Vec3::Y * (2.2 * s + 0.3) - Vec3::X * 0.4;
            g.line(top, top + Vec3::X * 0.8 * (b.hp / b.max_hp).max(0.0), Color::srgb(0.2, 1.0, 0.2));
        }
        // Order tell, seen by everyone: a pip pops over the head when an order lands.
        if b.told > 0.0 {
            let s = if b.is_ai() { tune.f(P::BuddyScale) } else { 1.0 };
            g.sphere(Isometry3d::from_translation(at + Vec3::Y * (2.4 * s + 0.4)), 0.12 + 0.2 * b.told, owner_c);
        }
        let Role::Buddy(o) = b.role else { continue };
        if tethers == 2 || (tethers == 1 && o == YOU) {
            g.line(pos_of(o) + Vec3::Y * 0.2, at + Vec3::Y * 0.2, owner_c.with_alpha(0.45));
        }
        if lines && o == YOU && you_play {
            let to = match b.job {
                Job::HaulThis(id) => sim.crates.iter().find(|c| c.id == id).map(|c| c.pos),
                Job::Attack(e) => Some(pos_of(e) + Vec3::Y),
                Job::Hold(p) => {
                    g.circle(ground_circle(p.with_y(0.05)), 0.5, owner_c);
                    Some(p)
                }
                Job::GuardPad => Some(PADS[b.team].extend(0.0).xzy()),
                _ => None,
            };
            if let Some(to) = to {
                g.line(at + Vec3::Y, to, owner_c.with_alpha(0.7));
            }
        }
    }
    // Readability test: the highlighted unit, then the right answer.
    if rt.on
        && let Some(t) = rt.target
        && rt.phase > 0
    {
        let at = pos_of(t);
        let pulse = 0.9 + 0.15 * (time.elapsed_secs() * 8.0).sin();
        let y = Color::srgb(1.0, 1.0, 0.0);
        g.circle(ground_circle(at.with_y(0.05)), pulse, y);
        g.circle(ground_circle(at.with_y(0.05)), pulse + 0.15, y);
        g.line(at + Vec3::Y * 2.6, at + Vec3::Y * 5.0, y);
        if rt.phase == 2 {
            let c = if rt.right { Color::srgb(0.2, 1.0, 0.2) } else { Color::srgb(1.0, 0.2, 0.2) };
            g.line(at + Vec3::Y, rt.answer_at + Vec3::Y, c);
            g.circle(ground_circle(rt.answer_at.with_y(0.06)), 1.0, c);
        }
    }
}

fn hud(sim: Res<Sim>, tune: Res<Tune>, controls: Res<Controls>, mut hud: Single<&mut Text, (With<Hud>, Without<Panel>)>) {
    let a = &sim.actors[YOU];
    let mut s = String::new();
    let left = sim.match_left.max(0.0);
    s += &format!(
        "MATCH {}:{:02}   BLUE {} - {} RED   (crates on pad)   kills {}-{}\n",
        (left / 60.0) as u32,
        (left % 60.0) as u32,
        sim.score(0),
        sim.score(1),
        sim.kills[0],
        sim.kills[1],
    );
    s += &format!("{}\n", roster_desc(&tune));
    if tune.c(P::Seat) == 1 {
        s += "your seat is a BOT: watching\n";
    } else {
        s += &format!(
            "you: {}{}  hp {:.0}{}\n",
            if a.carrying.is_some() { "CARRYING" } else { "empty-handed" },
            if a.grounded { "" } else { ", airborne" },
            a.hp.max(0.0),
            if a.alive() { String::new() } else { format!("  DOWN {:.1}s", a.dead) },
        );
        if a.carrying.is_some() {
            let boost = throw_plan(&sim, controls.aim_dist).boost;
            s += &format!("throw reach: {:.1} m  (jump x{boost:.2})\n", THROW_DIST * boost);
        }
    }
    let mine: Vec<String> = sim
        .squad(YOU)
        .map(|j| {
            let b = &sim.actors[j];
            if !b.alive() {
                "down".to_string()
            } else {
                let job = effective_job(&sim, j, &tune);
                let label = if b.job == Job::Default { format!("{} (default)", job.label()) } else { job.label().to_string() };
                if b.carrying.is_some() { format!("{label}, carrying") } else { label }
            }
        })
        .collect();
    if !mine.is_empty() {
        s += &format!("your squad: {}\n", mine.join(" | "));
    }
    let bodies = sim.actors.len();
    let moving = sim.crates.iter().filter(|c| c.state != CrateState::Free || c.vel.length() > 0.05).count();
    s += &format!(
        "netcode: {bodies} networked bodies (budget ~32), {moving} moving crates (~20-30), predicted here: you + crates near you\n"
    );
    let acc = if sim.fired > 0 { 100.0 * sim.hits as f32 / sim.fired as f32 } else { 0.0 };
    s += &format!("your shots {} hits {} ({acc:.0}%)   orders given {}\n", sim.fired, sim.hits, sim.orders_given);
    for (m, _) in &sim.msgs {
        s += &format!("> {m}\n");
    }
    s += "\nWASD move  mouse aim  Space jump  E grab/place  RMB throw  LMB shoot  F kick\n";
    s += match tune.c(P::Orders) {
        0 => "orders: off",
        1 => "orders: Q / LB at the reticle (enemy = attack, crate = haul, your pad = guard, ground = hold); tap = 1 buddy, hold = all.  G recall",
        _ => "orders: Z follow  X haul  C guard pad  V attack/hold at reticle (pad: LB + D-pad).  G recall",
    };
    s += "\nT readability test  R reset  P print  H panel  N next comparison";
    hud.0 = s;
}

fn guide_text(tune: Res<Tune>, rt: Res<ReadTest>, mut text: Single<&mut Text, (With<GuideText>, Without<Hud>, Without<Panel>)>) {
    if rt.on {
        let k = rt.answers.len();
        text.0 = match rt.phase {
            1 => format!(
                "READABILITY {}/{READ_QUESTIONS}: who owns the YELLOW-ringed unit?\n\
                 Click the unit itself if it's a human (leader), its leader if it's a buddy, \
                 its pad if it's a hauler. Be quick: the clock is running.",
                k + 1
            ),
            2 => format!("{} ({}/{READ_QUESTIONS})", if rt.right { "RIGHT" } else { "WRONG: the line shows the owner" }, k),
            _ => format!("READABILITY test: playing a moment, then the next freeze-frame ({k}/{READ_QUESTIONS}). T stops."),
        };
        return;
    }
    let done = tune.settled.iter().filter(|s| **s).count();
    let total = COMPARISONS.len();
    text.0 = match &tune.guide {
        Some(g) => {
            let cmp = &COMPARISONS[g.idx];
            let mut s = format!("COMPARISON {} of {total}: {}\n{}\n\n", g.idx + 1, cmp.title, cmp.question);
            for (i, (label, _)) in cmp.variants.iter().enumerate() {
                if i == g.variant {
                    s += &format!("  >> [{}] {} <<  PLAYING\n", i + 1, label.to_uppercase());
                } else {
                    s += &format!("     [{}] {}\n", i + 1, label);
                }
            }
            s += &format!(
                "\nPress 1-{} to switch (restarts the match).  ENTER = keep the one you're playing, next.  N = skip.",
                cmp.variants.len()
            );
            s
        }
        None if done == total => "All comparisons done. Press P and paste the output.".into(),
        None => format!("Press N to start the next comparison ({done} of {total} done)."),
    };
}

fn panel(tune: Res<Tune>, mut panel: Single<(&mut Text, &mut Visibility), (With<Panel>, Without<Hud>)>) {
    let (text, vis) = &mut *panel;
    **vis = if tune.panel { Visibility::Visible } else { Visibility::Hidden };
    let mut s = "TUNING (core feel fixed at the #9 baseline)\n\n".to_string();
    for (i, p) in tune.vals.iter().enumerate() {
        let cursor = if i == tune.selected { ">" } else { " " };
        s += &format!("{cursor} {:<20} {}\n", p.name, fmt_value(&p.value));
    }
    s += "\nUp/Down pick  Left/Right change (Shift x5)";
    text.0 = s;
}

#[derive(Resource, Default)]
struct Smoke {
    exit_at: Option<Duration>,
    shot: bool,
}

fn smoke_exit(time: Res<Time>, mut smoke: ResMut<Smoke>, mut exit: MessageWriter<AppExit>, mut commands: Commands) {
    let Some(secs) = std::env::var("SQUAD_EXIT_AFTER").ok().and_then(|s| s.parse::<f32>().ok()) else {
        return;
    };
    let deadline = *smoke.exit_at.get_or_insert(Duration::from_secs_f32(secs));
    if let Ok(path) = std::env::var("SQUAD_SCREENSHOT")
        && !smoke.shot
        && time.elapsed() + Duration::from_secs(1) >= deadline
    {
        smoke.shot = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    if time.elapsed() >= deadline {
        exit.write(AppExit::Success);
    }
}

fn main() {
    let tune = Tune {
        vals: defaults(),
        selected: 0,
        panel: true,
        guide: None,
        settled: vec![false; COMPARISONS.len()],
        verdicts: Vec::new(),
    };
    let sim = Sim::new(&tune);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "big_back squad model (PROTOTYPE)".into(),
                // Smoke runs often land on a hidden workspace, where vsync blocks forever.
                present_mode: if std::env::var("SQUAD_EXIT_AFTER").is_ok() {
                    bevy::window::PresentMode::AutoNoVsync
                } else {
                    bevy::window::PresentMode::AutoVsync
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GlobalAmbientLight { color: Color::WHITE, brightness: 500.0, ..default() })
        .insert_resource(ClearColor(Color::srgb(0.12, 0.13, 0.16)))
        .insert_resource(tune)
        .insert_resource(sim)
        .init_resource::<Controls>()
        .init_resource::<ReadTest>()
        .init_resource::<Smoke>()
        .add_systems(Startup, (setup, guide_from_env).chain())
        .add_systems(
            Update,
            (read_controls, tune_input, read_test, step, sync_views, follow_camera, draw_gizmos, hud, guide_text, panel, smoke_exit)
                .chain(),
        )
        .run();
}
