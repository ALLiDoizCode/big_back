//! PROTOTYPE — throwaway. Answers issue #10: which squad model makes the most fun and fair
//! competitive game? Not production code; lives on `prototype/squad-model`.
//!
//!     cargo run --example squad_model
//!
//! Verdict so far (2026-10-18): one character per player, no AI Buddies and no AI haulers.
//! What's left is the range of team sizes. Teams of players (you plus bots standing in for
//! other players) on the core-feel baseline from #9, fixed here as constants: curling
//! jump-throws, full-lock carrying, 18 m/s projectiles, reach-masked grabs.
//! See `docs/prototypes/squad-model.md` for what to try.
//!
//! Play: WASD / left stick move, mouse / right stick aim, Space / A jump, E / X grab or place,
//! RMB / RB throw, LMB / RT shoot, F / B kick (hold on an orange crate to open it).
//! Team size: 1-4 switch between 2v2 / 3v3 / 4v4 / 6v6, Enter picks the one you're playing.
//! Tune: Up/Down pick, Left/Right change (Shift = x5).
//! R / Start resets, P prints settings, verdicts and results, H hides the panel.
//!
//! Smoke tests: `SQUAD_EXIT_AFTER=<s>`, `SQUAD_SCREENSHOT=<file.png>`, `SQUAD_SEAT=bot`,
//! `SQUAD_TEAM=<n>` (team size, whole-arena camera).

use std::collections::VecDeque;
use std::f32::consts::FRAC_PI_2;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

const G: f32 = 20.0;
const CRATE: f32 = 0.8;
const HALF: f32 = CRATE / 2.0;
const HEAD: f32 = 2.2;
const CHEST: f32 = 1.2;
const ACTOR_R: f32 = 0.45;
const YOU: usize = 0;
const PAD_HALF: f32 = 2.0;
const CAM_OFFSET: Vec3 = Vec3::new(0.0, 21.0, 14.0);
/// The whole-arena camera at 3v3 size; it moves out with the arena.
const ARENA_CAM: Vec3 = Vec3::new(0.0, 46.0, 28.0);
/// Arena half-extents at 3v3 (36 x 22 m): the shape every team size keeps.
const BASE_ARENA: Vec2 = Vec2::new(18.0, 11.0);

/// Arena half-extents, set from team size on every reset. A static so the free physics
/// functions can read it; there's one arena at a time.
static ARENA_BITS: AtomicU64 = AtomicU64::new(0);

fn arena() -> Vec2 {
    let b = ARENA_BITS.load(Ordering::Relaxed);
    Vec2::new(f32::from_bits(b as u32), f32::from_bits((b >> 32) as u32))
}

/// Same floor space per player at every team size: the 3v3 shape, scaled.
fn set_arena(team_size: usize, space_per_player: f32) {
    let base_area = BASE_ARENA.x * BASE_ARENA.y * 4.0;
    let k = (2.0 * team_size as f32 * space_per_player / base_area).sqrt().max(0.5);
    let a = BASE_ARENA * k;
    ARENA_BITS.store(a.x.to_bits() as u64 | (a.y.to_bits() as u64) << 32, Ordering::Relaxed);
}

/// How far the arena is scaled from the 3v3 size.
fn arena_scale() -> f32 {
    arena().x / BASE_ARENA.x
}

/// Team pads, on the crate grid, 3.6 m in from each end.
fn pads() -> [Vec2; 2] {
    let x = ((arena().x - 3.6) / CRATE).round() * CRATE;
    [Vec2::new(-x, 0.0), Vec2::new(x, 0.0)]
}
const TURRET_H: f32 = 1.1;
const PLACE_REACH: f32 = 1.6;

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
/// 4 shots kill a 100 hp player.
const SHOT_DMG: f32 = 25.0;
const KICK_TIME: f32 = 1.5;
const GRAB_LATENCY: f32 = 0.12;
const GRAB_REACH: f32 = 1.2;

const TEAM_COLOR: [Color; 2] = [Color::srgb(0.3, 0.55, 1.0), Color::srgb(1.0, 0.35, 0.3)];
/// Player colours by slot, worn as a band: tells teammates apart.
const PLAYER_COLOR: [Color; 8] = [
    Color::srgb(1.0, 0.9, 0.15),
    Color::srgb(0.25, 1.0, 0.35),
    Color::srgb(0.97, 0.97, 0.97),
    Color::srgb(0.85, 0.3, 1.0),
    Color::srgb(1.0, 0.55, 0.1),
    Color::srgb(0.2, 0.95, 0.95),
    Color::srgb(1.0, 0.5, 0.75),
    Color::srgb(0.15, 0.15, 0.15),
];

// ---------------------------------------------------------------------------------------
// Tunables
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum P {
    TeamSize,
    Space,
    AllyStacks,
    OnlineDelay,
    Seat,
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

fn defaults() -> Vec<Param> {
    // Order must match `P`.
    vec![
        num("team size", 3.0, 1.0, 8.0, 1.0, "v"),
        // 3v3 in 36 x 22 m is 132 m2 per player.
        num("space per player", 130.0, 60.0, 250.0, 10.0, "m2"),
        // Off: only you build blue's stacks, so you can see each join is yours.
        choice("blue bots stack", 0, &["off (only you)", "on"]),
        num("online delay", 0.0, 0.0, 250.0, 25.0, "ms"),
        choice("your seat", 0, &["you play", "bot (watch)"]),
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
    /// Team sizes picked with Enter, printed with P.
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

fn roster_desc(tune: &Tune) -> String {
    let n = tune.c(P::TeamSize);
    let a = arena() * 2.0;
    format!("up to {n}v{n} players (start 1v1, +1 per 2-high stack), no AI units, arena {:.0} x {:.0} m", a.x, a.y)
}

/// The team sizes keys 1-4 switch between.
const SIZES: [usize; 4] = [2, 3, 4, 6];

fn init_from_env(mut tune: ResMut<Tune>, mut sim: ResMut<Sim>) {
    if let Some(n) = std::env::var("SQUAD_TEAM").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::TeamSize, n);
        set(&mut tune.vals, P::Camera, 1.0);
    }
    if std::env::var("SQUAD_SEAT").as_deref() == Ok("bot") {
        set(&mut tune.vals, P::Seat, 1.0);
    }
    sim.reset(&tune);
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
    /// Who last set this crate down: the one who completes a stack.
    placed_by: Option<usize>,
    settling: bool,
    ent: Option<Entity>,
}

/// A player: you, or a bot standing in for another player.
struct Actor {
    team: usize,
    /// Slot in the team: picks the player colour.
    slot: usize,
    pos: Vec3,
    vel: Vec3,
    facing: Vec3,
    grounded: bool,
    carrying: Option<u32>,
    /// The crate this bot is heading for, so teammates pick different ones.
    claim: Option<u32>,
    fire_cd: f32,
    hp: f32,
    dead: f32,
    stun: f32,
    /// Recent positions, to show other players slightly in the past (online interpolation).
    hist: VecDeque<(f32, Vec3)>,
    ent: Option<Entity>,
}

impl Actor {
    fn new(team: usize, slot: usize) -> Self {
        Actor {
            team,
            slot,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            facing: Vec3::X,
            grounded: true,
            carrying: None,
            claim: None,
            fire_cd: 0.5,
            hp: 100.0,
            dead: 0.0,
            stun: 0.0,
            hist: VecDeque::new(),
            ent: None,
        }
    }
    fn alive(&self) -> bool {
        self.dead <= 0.0
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
    next_id: u32,
    rng: u32,
    t: f32,
    loot_respawn: f32,
    msgs: Vec<(String, f32)>,
    despawn: Vec<Entity>,
    /// Players per team.
    team_size: usize,
    /// The space per player the arena was sized with.
    space: f32,
    /// Blue's bots build 2-high stacks too.
    ally_stacks: bool,
    /// Crates spent on placeholder turrets: they still count for the score.
    banked: [usize; 2],
    match_left: f32,
    /// Counts down the end-of-match result screen.
    over: f32,
    /// Match results, printed with P. Survives resets.
    log: Vec<String>,
    // Stats.
    fired: u32,
    hits: u32,
    kills: [u32; 2],
}

impl Sim {
    fn new(tune: &Tune) -> Self {
        let mut s = Sim {
            actors: Vec::new(),
            crates: Vec::new(),
            shots: Vec::new(),
            turrets: Vec::new(),
            pending: None,
            next_id: 1,
            rng: 0x2545_f491,
            t: 0.0,
            loot_respawn: 0.0,
            msgs: Vec::new(),
            despawn: Vec::new(),
            team_size: 0,
            space: 0.0,
            ally_stacks: false,
            banked: [0; 2],
            match_left: 0.0,
            over: 0.0,
            log: Vec::new(),
            fired: 0,
            hits: 0,
            kills: [0; 2],
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
        self.team_size = tune.c(P::TeamSize);
        self.space = tune.f(P::Space);
        set_arena(self.team_size, self.space);
        self.banked = [0; 2];
        for team in 0..2 {
            for slot in 0..self.team_size {
                self.actors.push(Actor::new(team, slot));
            }
        }
        for i in 0..self.actors.len() {
            let p = self.spawn_point(i);
            let a = &mut self.actors[i];
            a.pos = p;
            a.facing = Vec3::X * toward_centre(a.team);
            // Each team starts with one player; the rest join from 2-high stacks.
            if a.slot > 0 {
                a.dead = f32::INFINITY;
            }
        }
        // Starting crates in a grid across the middle, spread to fit the arena.
        let (k, a) = (arena_scale(), arena());
        let count = 6 + 3 * self.team_size;
        let rows = count.div_ceil(6);
        let dz = (3.2 * k).min(a.y * 1.6 / rows as f32);
        for i in 0..count {
            let x = ((i % 6) as f32 - 2.5) * 3.2 * k;
            let z = ((i / 6) as f32 - (rows as f32 - 1.0) / 2.0) * dz;
            self.spawn_crate(Kind::Normal, Vec3::new(x, HALF, z));
        }
        self.spawn_crate(Kind::Loot, Vec3::new(0.0, HALF, -0.4));
        self.spawn_crate(Kind::Loot, Vec3::new(0.0, HALF, a.y * 0.75));
        self.match_left = tune.f(P::MatchLength);
        self.over = 0.0;
        self.fired = 0;
        self.hits = 0;
        self.kills = [0; 2];
    }

    /// Players start in a line in front of their pad.
    fn spawn_point(&self, i: usize) -> Vec3 {
        let a = &self.actors[i];
        let n = self.team_size.max(1) as f32;
        let gap = (arena().y * 1.6 / n).min(3.2);
        let lane = (a.slot as f32 - (n - 1.0) / 2.0) * gap;
        Vec3::new(pads()[a.team].x + toward_centre(a.team) * 4.0, 0.0, lane)
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
            placed_by: None,
            settling: false,
            ent: None,
        });
        id
    }

    fn crate_mut(&mut self, id: u32) -> Option<&mut Crate> {
        self.crates.iter_mut().find(|c| c.id == id)
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
            c.placed_by = Some(who);
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

    /// Where bot `i` sets its crate down on its pad: the lowest cell, then the nearest.
    /// While the team has room for another player, on top of a lone crate first, to build
    /// a 2-high stack (blue's bots only with "blue bots stack" on).
    fn pad_cell(&self, i: usize) -> Vec3 {
        let (team, from) = (self.actors[i].team, self.actors[i].pos);
        let column = self.players(team) < self.team_size && (team == 1 || self.ally_stacks || i == YOU);
        let pad = pads()[team];
        let mut best: Option<(f32, Vec3)> = None;
        for gx in -2..=2 {
            for gz in -2..=2 {
                let at = self.target_at(pad + Vec2::new(gx as f32, gz as f32) * CRATE);
                let second = (at.y - CRATE * 1.5).abs() < 0.05;
                let score = if column && second { -10.0 } else { at.y * 10.0 } + at.xz().distance(from.xz());
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
            c.placed_by = Some(who);
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
        // A team's first player respawns; anyone who joined from a stack is out until the
        // team stacks 2 crates again.
        let first = a.slot == 0;
        a.dead = if first { 3.0 } else { f32::INFINITY };
        a.claim = None;
        self.kills[by_team] += 1;
        let what = self.describe(who);
        if who == YOU {
            self.pending = None;
            self.say("you're down (3 s)");
        } else if !first {
            let how = if by == Some(YOU) { "you knocked out" } else { "out:" };
            self.say(format!("{how} {what}. Stack 2 crates to bring a player back"));
        } else if by == Some(YOU) {
            self.say(format!("you downed {what}"));
        }
    }

    fn describe(&self, i: usize) -> String {
        let a = &self.actors[i];
        let team = ["blue", "red"][a.team];
        if i == YOU { "you".into() } else { format!("{team} player {}", a.slot + 1) }
    }

    /// Crates resting on the team's pad, plus crates spent there on new players and
    /// placeholder turrets.
    /// Players on the team: everyone who has joined, alive or waiting to respawn.
    fn players(&self, team: usize) -> usize {
        self.actors.iter().filter(|a| a.team == team && !a.dead.is_infinite()).count()
    }

    fn score(&self, team: usize) -> usize {
        let on_pad = self
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && pad_contains(pads()[team], c.pos))
            .count();
        on_pad + self.banked[team]
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
    let arena = arena();
    if p.x.abs() > arena.x - HALF {
        p.x = p.x.clamp(-arena.x + HALF, arena.x - HALF);
        v.x *= -0.4;
    }
    if p.z.abs() > arena.y - HALF {
        p.z = p.z.clamp(-arena.y + HALF, arena.y - HALF);
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
}

#[derive(Component)]
struct MainCamera;

fn read_controls(
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
    let any = |k: KeyCode, b: GamepadButton| keys.just_pressed(k) || gamepads.iter().any(|g| g.just_pressed(b));
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
    // 1-4 switch team size; Enter picks the one you're playing.
    for (k, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4].iter().enumerate() {
        if keys.just_pressed(*key) {
            let n = SIZES[k];
            set(&mut tune.vals, P::TeamSize, n as f32);
            sim.reset(&tune);
            let a = arena() * 2.0;
            sim.say(format!("NOW PLAYING {n}v{n} (arena {:.0} x {:.0} m)", a.x, a.y));
        }
    }
    if keys.just_pressed(KeyCode::Enter) {
        let line = format!("team size: {}", roster_desc(&tune));
        println!("verdict: {line}");
        tune.verdicts.push(line);
        let n = tune.c(P::TeamSize);
        sim.say(format!("picked {n}v{n} (P prints it)"));
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
    if tune.c(P::TeamSize) != s.team_size || tune.f(P::Space) != s.space {
        s.reset(&tune);
    }
    let dt = time.delta_secs().min(1.0 / 30.0);
    for m in &mut s.msgs { m.1 -= dt; }
    s.msgs.retain(|m| m.1 > 0.0);
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
    s.ally_stacks = tune.c(P::AllyStacks) == 1;
    let t = s.t;
    for i in 0..s.actors.len() {
        let a = &mut s.actors[i];
        a.fire_cd -= dt;
        a.stun -= dt;
        if a.dead > 0.0 {
            a.dead -= dt;
            if a.dead <= 0.0 {
                let p = s.spawn_point(i);
                let a = &mut s.actors[i];
                a.pos = p;
                a.vel = Vec3::ZERO;
                a.hp = 100.0;
                a.hist.clear();
            }
        }
    }
    if tune.c(P::Seat) == 1 {
        ai_step(s, YOU, dt);
    } else {
        player_step(s, c, dt);
    }
    for i in 1..s.actors.len() {
        ai_step(s, i, dt);
    }
    actors_physics(s, dt);
    crates_physics(s);
    crates_step(s, dt);
    shots_step(s, dt);
    join_players(s);
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
        "match: BLUE {b} - {r} RED, {winner} | {} | blue player 1 = {seat} | kills {}-{}",
        roster_desc(tune),
        s.kills[0],
        s.kills[1],
    );
    println!("{line}");
    s.log.push(line);
    s.say(format!("MATCH OVER: blue {b} - {r} red, {winner}"));
    s.over = 4.0;
}

fn player_step(s: &mut Sim, c: &Controls, dt: f32) {
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
        fire(s, YOU, facing, range);
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

fn fire(s: &mut Sim, who: usize, dir: Vec3, range: f32) {
    let a = &s.actors[who];
    let from = a.pos + Vec3::Y * CHEST + dir * 0.5;
    let team = a.team;
    if who == YOU {
        s.fired += 1;
    }
    s.shots.push(Shot { pos: from, vel: dir * PROJ_SPEED, owner: Some(who), team, dmg: SHOT_DMG, life: range / PROJ_SPEED, ent: None });
}

// ---------------------------------------------------------------------------------------
// Bots standing in for other players
// ---------------------------------------------------------------------------------------

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

/// Fetch the nearest crate no teammate is after (stealing from the enemy pad included) and
/// set it down on the team pad. Returns the goal and the action; `None` goal and no action
/// means there's nothing to haul. Bot grabs are instant: they stand in for the server.
fn haul_plan(s: &Sim, i: usize) -> (Option<Vec3>, Act, Option<u32>) {
    let a = &s.actors[i];
    if a.carrying.is_some() {
        let cell = s.pad_cell(i);
        if cell.xz().distance(a.pos.xz()) < 2.0 {
            return (None, Act::Place(cell), None);
        }
        return (Some(cell.with_y(0.0)), Act::None, None);
    }
    let pad = pads()[a.team];
    let claimed = |id: u32| s.actors.iter().enumerate().any(|(j, b)| j != i && b.team == a.team && b.claim == Some(id));
    let pick = s
        .crates
        .iter()
        .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && c.pos.y < 3.0)
        .filter(|c| !pad_contains(pad, c.pos) && !claimed(c.id))
        .min_by(|x, y| x.pos.distance(a.pos).total_cmp(&y.pos.distance(a.pos)));
    match pick {
        None => (None, Act::None, None),
        Some(c) if c.pos.xz().distance(a.pos.xz()) <= GRAB_REACH + HALF => (None, Act::Grab(c.id), Some(c.id)),
        Some(c) => (Some(c.pos.with_y(0.0)), Act::None, Some(c.id)),
    }
}

fn guard_post(s: &Sim, i: usize) -> Vec3 {
    let a = &s.actors[i];
    let pad = pads()[a.team];
    Vec3::new(pad.x + toward_centre(a.team) * 2.8, 0.0, pad.y + (a.slot as f32 - 1.0) * 1.6)
}

/// A bot player: carry home, fight what's close, chase enemy carriers to steal back,
/// otherwise haul (and steal). Guards the pad when there's nothing to haul.
fn ai_step(s: &mut Sim, i: usize, dt: f32) {
    if !s.actors[i].alive() {
        return;
    }
    let pos = s.actors[i].pos;
    let carrying = s.actors[i].carrying.is_some();
    let near = nearest_enemy(s, i, pos, 8.0);
    let carrier = nearest_enemy(s, i, pos, 14.0).filter(|&e| s.actors[e].carrying.is_some());
    let keep = s.actors[i].claim;
    let (target, goal, act, claim) = match (carrying, near, carrier) {
        (false, Some(e), _) => {
            // Strafe while fighting.
            let to = (s.actors[e].pos - pos).with_y(0.0).normalize_or_zero();
            let side = Vec3::new(-to.z, 0.0, to.x) * (s.t * 0.7 + i as f32).sin().signum();
            (Some(e), Some(pos + side * 2.0), Act::None, keep)
        }
        (false, None, Some(e)) => (Some(e), Some(s.actors[e].pos), Act::None, keep),
        _ => {
            let (goal, act, claim) = haul_plan(s, i);
            let idle = goal.is_none() && matches!(act, Act::None);
            (None, if idle { Some(guard_post(s, i)) } else { goal }, act, claim)
        }
    };
    let speed = MOVE_SPEED * 0.85 * if carrying { CARRY_SPEED } else { 1.0 };
    let spread = (s.rand() - 0.5) * 0.15;
    {
        let a = &mut s.actors[i];
        a.claim = claim;
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
        if !carrying && d < 12.0 && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            s.actors[i].fire_cd = 0.5;
            fire(s, i, dir, 14.0);
        }
    }
    match act {
        Act::Grab(id) => s.take(i, id),
        Act::Place(at) => s.place(i, at),
        Act::None => {}
    }
}

// ---------------------------------------------------------------------------------------
// Physics
// ---------------------------------------------------------------------------------------

fn actors_physics(s: &mut Sim, dt: f32) {
    let arena = arena();
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
        a.pos.x = a.pos.x.clamp(-arena.x + ACTOR_R, arena.x - ACTOR_R);
        a.pos.z = a.pos.z.clamp(-arena.y + ACTOR_R, arena.y - ACTOR_R);
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

/// Team Buddies' rule, with players: a 2-high stack resting on a team's pad brings in that
/// team's next player at the stack, up to the team size. PLACEHOLDER shape (see #17).
fn join_players(s: &mut Sim) {
    let on_grid = |p: Vec3| (p.xz() / CRATE - (p.xz() / CRATE).round()).length() < 0.05;
    let resting = |c: &Crate| c.state == CrateState::Free && c.kind == Kind::Normal && c.vel.length() < 0.1 && on_grid(c.pos);
    for team in 0..2 {
        let Some(slot) = (0..s.actors.len()).find(|&i| s.actors[i].team == team && s.actors[i].dead.is_infinite()) else {
            continue;
        };
        let pad = pads()[team];
        let stack = s.crates.iter().filter(|c| resting(c) && pad_contains(pad, c.pos)).find_map(|top| {
            if (top.pos.y - CRATE * 1.5).abs() > 0.05 {
                return None;
            }
            let bottom = s
                .crates
                .iter()
                .find(|c| resting(c) && (c.pos.y - HALF).abs() < 0.05 && c.pos.xz().distance(top.pos.xz()) < 0.1)?;
            Some((top.id, bottom.id, top.pos.with_y(0.0), top.placed_by))
        });
        let Some((a, b, at, by)) = stack else { continue };
        s.crates.retain(|c| {
            let used = c.id == a || c.id == b;
            if used { s.despawn.extend(c.ent); }
            !used
        });
        s.banked[team] += 2;
        let p = &mut s.actors[slot];
        p.dead = 0.0;
        p.hp = 100.0;
        p.pos = at;
        p.vel = Vec3::ZERO;
        p.hist.clear();
        let (n, cap) = (s.players(team), s.team_size);
        let who = match by {
            Some(YOU) => "YOU".to_string(),
            Some(j) => s.describe(j),
            None => "a knocked crate".to_string(),
        };
        let msg = format!("{} stack by {who}: a player joins ({n}/{cap})", ["BLUE", "RED"][team]);
        println!("t={:.1}s {msg}", s.t);
        s.say(msg);
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
            .filter(|c| pad_contains(pads()[team], c.pos))
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
                s.banked[team] += 4;
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
        .filter(|c| c.state != CrateState::Free || !(pad_contains(pads()[0], c.pos) || pad_contains(pads()[1], c.pos)))
        .count();
    if loose < 3 + 2 * s.team_size {
        let a = arena();
        let x = (s.rand() * 2.0 - 1.0) * a.x * 0.4;
        let z = (s.rand() * 2.0 - 1.0) * a.y * 0.8;
        s.spawn_crate(Kind::Normal, Vec3::new(x, 10.0, z));
    }
    if s.crates.iter().filter(|c| c.kind == Kind::Loot).count() < 2 {
        s.loot_respawn -= dt;
        if s.loot_respawn <= 0.0 {
            let x = (s.rand() * 2.0 - 1.0) * 3.0 * arena_scale();
            let z = (s.rand() * 2.0 - 1.0) * arena().y * 0.65;
            s.spawn_crate(Kind::Loot, Vec3::new(x, 10.0, z));
            s.loot_respawn = 6.0;
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
    team: [Handle<StandardMaterial>; 2],
    player: [Handle<StandardMaterial>; 8],
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct PadView(usize);

fn sync_arena(
    mut ground: Single<&mut Transform, With<Ground>>,
    mut pad_views: Query<(&PadView, &mut Transform), Without<Ground>>,
) {
    let a = arena();
    ground.scale = Vec3::new(a.x * 2.0, 1.0, a.y * 2.0);
    for (p, mut tf) in &mut pad_views {
        let pad = pads()[p.0];
        tf.translation = Vec3::new(pad.x, 0.01, pad.y);
    }
}

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
    // Ground and pads are resized and moved by `sync_arena`.
    commands.spawn((
        Ground,
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.1, 1.0))),
        MeshMaterial3d(mats.add(Color::srgb(0.32, 0.36, 0.3))),
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));
    for (team, color) in TEAM_COLOR.into_iter().enumerate() {
        commands.spawn((
            PadView(team),
            Mesh3d(meshes.add(Cuboid::new(PAD_HALF * 2.0, 0.02, PAD_HALF * 2.0))),
            MeshMaterial3d(mats.add(color.with_alpha(0.6))),
            Transform::from_xyz(0.0, 0.01, 0.0),
        ));
    }
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
        team: TEAM_COLOR.map(|c| mats.add(c)),
        player: PLAYER_COLOR.map(|c| mats.add(StandardMaterial { base_color: c, emissive: c.to_linear() * 0.4, ..default() })),
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
        TextFont::from_font_size(24.0),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            padding: UiRect::all(px(10.0)),
            max_width: percent(70.0),
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
                let body = looks.team[a.team].clone();
                let band = looks.player[a.slot % PLAYER_COLOR.len()].clone();
                let (bm, nm, band_m) = (looks.body.clone(), looks.nose.clone(), looks.band.clone());
                a.ent = Some(
                    commands
                        .spawn((
                            Transform::from_translation(shown_pos[i]).with_rotation(rot),
                            Visibility::default(),
                        ))
                        .with_children(|p| {
                            p.spawn((Mesh3d(bm), MeshMaterial3d(body.clone()), Transform::from_xyz(0.0, 0.95, 0.0)));
                            p.spawn((Mesh3d(nm), MeshMaterial3d(body), Transform::from_xyz(0.0, 1.3, -0.5)));
                            p.spawn((Mesh3d(band_m), MeshMaterial3d(band), Transform::from_xyz(0.0, 1.0, 0.0)));
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
        let want = ARENA_CAM * arena_scale().max(1.0);
        cam.translation = cam.translation.lerp(want, k);
        let look = cam.translation - want;
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

fn draw_gizmos(mut g: Gizmos, sim: Res<Sim>, tune: Res<Tune>, controls: Res<Controls>) {
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
    }
    let kt = KICK_TIME;
    for c in sim.crates.iter().filter(|c| c.kind == Kind::Loot && c.kick > 0.0) {
        g.circle(ground_circle(c.pos.with_y(0.04)), 0.2 + 0.6 * (c.kick / kt).min(1.0), Color::srgb(1.0, 0.7, 0.1));
    }
    for pad in pads() {
        g.rect(
            Isometry3d::new(pad.extend(0.03).xzy(), Quat::from_rotation_x(FRAC_PI_2)),
            Vec2::splat(PAD_HALF * 2.0),
            Color::WHITE,
        );
    }
    // Damage shows as a bar over the head.
    for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive() && b.hp < 100.0) {
        let top = pos_of(i) + Vec3::Y * 2.5 - Vec3::X * 0.4;
        g.line(top, top + Vec3::X * 0.8 * (b.hp / 100.0).max(0.0), Color::srgb(0.2, 1.0, 0.2));
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
    s += &format!(
        "players: BLUE {}/{n}  RED {}/{n}   stack 2 crates on your pad (one on top of another) to bring in a teammate\n",
        sim.players(0),
        sim.players(1),
        n = sim.team_size,
    );
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
    let bodies = sim.players(0) + sim.players(1);
    let moving = sim.crates.iter().filter(|c| c.state != CrateState::Free || c.vel.length() > 0.05).count();
    s += &format!(
        "netcode: {bodies} networked characters (budget ~32), {moving} moving crates (~20-30), predicted here: you + crates near you\n"
    );
    let acc = if sim.fired > 0 { 100.0 * sim.hits as f32 / sim.fired as f32 } else { 0.0 };
    s += &format!("your shots {} hits {} ({acc:.0}%)\n", sim.fired, sim.hits);
    for (m, _) in &sim.msgs {
        s += &format!("> {m}\n");
    }
    s += "\nWASD move  mouse aim  Space jump  E grab/place  RMB throw  LMB shoot  F kick\n";
    s += "1-4 team size  ENTER pick it  R reset  P print  H panel";
    hud.0 = s;
}

fn guide_text(tune: Res<Tune>, mut text: Single<&mut Text, (With<GuideText>, Without<Hud>, Without<Panel>)>) {
    let n = tune.c(P::TeamSize);
    let a = arena() * 2.0;
    let mut s = format!(
        "YOU ARE PLAYING {n}v{n}   (arena {:.0} x {:.0} m)\n\
         Teams start with 1 player. Stack 2 crates on your pad to add one, up to {n}.\n\n\
         Press a number to switch (restarts the match):\n",
        a.x,
        a.y
    );
    for (k, size) in SIZES.iter().enumerate() {
        if *size == n {
            s += &format!("  >> {} = {size}v{size}  <<  PLAYING\n", k + 1);
        } else {
            s += &format!("     {} = {size}v{size}\n", k + 1);
        }
    }
    s += &format!("ENTER = pick {n}v{n} as the size you like");
    if !tune.verdicts.is_empty() {
        let picks: Vec<&str> = tune.verdicts.iter().map(|v| v.trim_start_matches("team size: ").split(',').next().unwrap_or("")).collect();
        s += &format!("\nPicked so far: {}", picks.join(", "));
    }
    text.0 = s;
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
        .init_resource::<Smoke>()
        .add_systems(Startup, (setup, init_from_env).chain())
        .add_systems(
            Update,
            (read_controls, tune_input, step, sync_arena, sync_views, follow_camera, draw_gizmos, hud, guide_text, panel, smoke_exit)
                .chain(),
        )
        .run();
}
