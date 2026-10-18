//! PROTOTYPE — throwaway. Answers issue #9: what should moving, carrying, throwing and
//! shooting feel like for one character? Not production code; lives on `prototype/core-feel`.
//!
//!     cargo run --example core_feel
//!
//! Primitive shapes only (capsules and cubes) and hand-rolled kinematics instead of avian3d,
//! so feel can be tuned without fighting physics. See `docs/prototypes/core-feel.md` for
//! what to try.
//!
//! Play: WASD / left stick move, mouse / right stick aim, Space / A jump, E / X grab or place,
//! RMB / RB throw, LMB / RT shoot, F / B kick (hold on an orange crate to open it).
//! Tune: Up/Down (D-pad) pick a parameter, Left/Right change it (Shift = x5), Tab / Select
//! swaps between slot A and B, R / Start resets the arena, P prints the active slot,
//! H hides the panel.
//!
//! `CORE_FEEL_EXIT_AFTER=<seconds>` quits automatically; with `CORE_FEEL_SCREENSHOT=<file.png>`
//! it saves a screenshot just before quitting (smoke tests).

use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

const G: f32 = 20.0;
const ARENA: Vec2 = Vec2::new(16.0, 10.0);
const CRATE: f32 = 0.8;
const HALF: f32 = CRATE / 2.0;
const HEAD: f32 = 2.2;
const CHEST: f32 = 1.2;
const ACTOR_R: f32 = 0.45;
const PLAYER: usize = 0;
const BOT: usize = 1;
const PLAYER_PAD: Vec2 = Vec2::new(-12.0, 0.0);
const BOT_PAD: Vec2 = Vec2::new(12.0, 0.0);
const PAD_HALF: f32 = 2.0;
const CAM_OFFSET: Vec3 = Vec3::new(0.0, 21.0, 14.0);
const TURRET_H: f32 = 1.1;
const BOT_SPAWN: Vec3 = Vec3::new(5.0, 0.0, 0.0);
const PLAYER_SPAWN: Vec3 = Vec3::new(-6.0, 0.0, 0.0);

// ---------------------------------------------------------------------------------------
// Tunables
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum P {
    MoveSpeed,
    CarrySpeed,
    CarryDisarm,
    HitDropsCrate,
    JumpVel,
    ThrowStyle,
    ThrowAngle,
    ThrowDist,
    ArcPreview,
    Weapon,
    FireRate,
    ProjSpeed,
    ShotsToKill,
    KickTime,
    GrabLatency,
    GrabMask,
    GrabReach,
    BotMode,
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
        num("move speed", 7.0, 3.0, 14.0, 0.5, "m/s"),
        num("carry speed", 0.7, 0.3, 1.0, 0.05, "x"),
        choice("carrying disarms", 0, &["full lock", "sidearm (half rate)", "melee only"]),
        choice("hit drops crate", 0, &["yes", "no"]),
        num("jump velocity", 6.0, 3.0, 10.0, 0.5, "m/s"),
        choice("throw style", 0, &["ground throw", "jump-throw"]),
        num("throw angle", 35.0, 10.0, 70.0, 5.0, "deg"),
        num("throw distance", 9.0, 3.0, 20.0, 1.0, "m"),
        choice("arc preview", 0, &["on", "off"]),
        choice("weapon", 0, &["hitscan", "projectile"]),
        num("fire rate", 5.0, 1.0, 15.0, 1.0, "/s"),
        num("projectile speed", 30.0, 8.0, 80.0, 2.0, "m/s"),
        num("shots to kill", 8.0, 1.0, 20.0, 1.0, ""),
        num("kick-open time", 1.0, 0.0, 3.0, 0.25, "s"),
        num("fake grab latency", 0.0, 0.0, 150.0, 10.0, "ms"),
        choice("latency mask", 0, &["none", "reach", "lunge"]),
        num("grab reach", 1.6, 0.8, 3.0, 0.1, "m"),
        choice("bot", 1, &["dummy", "strafer", "hauler", "duelist"]),
        // PLACEHOLDER only, so stacking does something. The real grammar is issue #17.
        choice("placeholder craft", 0, &["2x2 = turret", "off"]),
    ]
}

#[derive(Resource)]
struct Tune {
    slots: [Vec<Param>; 2],
    active: usize,
    selected: usize,
    panel: bool,
}

impl Tune {
    fn f(&self, p: P) -> f32 {
        match self.slots[self.active][p as usize].value {
            Value::Num { v, .. } => v,
            Value::Choice { i, .. } => i as f32,
        }
    }
    fn c(&self, p: P) -> usize {
        self.f(p) as usize
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

fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Num { v: x, .. }, Value::Num { v: y, .. }) => (x - y).abs() < 1e-4,
        (Value::Choice { i: x, .. }, Value::Choice { i: y, .. }) => x == y,
        _ => false,
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
    /// Set while airborne after a throw, so it can hurt the other actor.
    thrown_by: Option<usize>,
    ent: Option<Entity>,
}

#[derive(Default)]
struct Actor {
    pos: Vec3,
    vel: Vec3,
    facing: Vec3,
    grounded: bool,
    carrying: Option<u32>,
    fire_cd: f32,
    hp: f32,
    dead: f32,
    stun: f32,
    first_hit: Option<f32>,
}

struct Pending {
    crate_id: u32,
    left: f32,
    total: f32,
}

/// PLACEHOLDER craft result: shoots at the bot for a while, then expires.
struct Turret {
    pos: Vec3,
    facing: Vec3,
    cd: f32,
    life: f32,
    ent: Option<Entity>,
}

struct Shot {
    pos: Vec3,
    vel: Vec3,
    owner: usize,
    life: f32,
    /// False for turret shots, so they stay out of your accuracy.
    counted: bool,
    ent: Option<Entity>,
}

#[derive(Resource)]
struct Sim {
    actors: [Actor; 2],
    crates: Vec<Crate>,
    shots: Vec<Shot>,
    turrets: Vec<Turret>,
    tracers: Vec<(Vec3, Vec3, f32)>,
    swings: Vec<(Vec3, Vec3, f32)>,
    pending: Option<Pending>,
    next_id: u32,
    rng: u32,
    t: f32,
    loot_respawn: f32,
    bot_fire_cd: f32,
    msgs: Vec<(String, f32)>,
    despawn: Vec<Entity>,
    // Stats.
    fired: u32,
    hits: u32,
    times_hit: u32,
    last_ttk: Option<f32>,
    grabs: u32,
    denied: u32,
}

impl Sim {
    fn new() -> Self {
        let mut s = Sim {
            actors: [Actor::default(), Actor::default()],
            crates: Vec::new(),
            shots: Vec::new(),
            turrets: Vec::new(),
            tracers: Vec::new(),
            swings: Vec::new(),
            pending: None,
            next_id: 1,
            rng: 0x2545_f491,
            t: 0.0,
            loot_respawn: 0.0,
            bot_fire_cd: 1.0,
            msgs: Vec::new(),
            despawn: Vec::new(),
            fired: 0,
            hits: 0,
            times_hit: 0,
            last_ttk: None,
            grabs: 0,
            denied: 0,
        };
        s.reset();
        s
    }

    fn reset(&mut self) {
        for c in self.crates.drain(..) {
            self.despawn.extend(c.ent);
        }
        for t in self.turrets.drain(..) {
            self.despawn.extend(t.ent);
        }
        for sh in self.shots.drain(..) {
            self.despawn.extend(sh.ent);
        }
        self.pending = None;
        self.actors[PLAYER] = Actor { pos: PLAYER_SPAWN, facing: Vec3::X, hp: 100.0, ..default() };
        self.actors[BOT] = Actor { pos: BOT_SPAWN, facing: -Vec3::X, hp: 100.0, ..default() };
        for i in 0..8 {
            let x = -4.0 + (i % 4) as f32 * 2.5;
            let z = if i < 4 { -4.0 } else { 4.0 };
            self.spawn_crate(Kind::Normal, Vec3::new(x, HALF, z));
        }
        self.spawn_crate(Kind::Loot, Vec3::new(-1.0, HALF, 0.0));
        self.spawn_crate(Kind::Loot, Vec3::new(1.5, HALF, 7.0));
        self.fired = 0;
        self.hits = 0;
        self.times_hit = 0;
        self.last_ttk = None;
        self.grabs = 0;
        self.denied = 0;
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
            ent: None,
        });
        id
    }

    fn crate_mut(&mut self, id: u32) -> Option<&mut Crate> {
        self.crates.iter_mut().find(|c| c.id == id)
    }

    fn say(&mut self, s: impl Into<String>) {
        self.msgs.push((s.into(), 2.5));
        if self.msgs.len() > 5 {
            self.msgs.remove(0);
        }
    }

    /// Lets go of whatever `who` carries, with the given velocity, from head height.
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

    /// Where a placed crate lands: the crate-sized grid cell in front (the grid lines up with
    /// the pads), on top of whatever is already in it. A loose, off-grid crate right under the
    /// target is stacked on instead. Returns the landing centre.
    fn place_target(&self, who: usize) -> Vec3 {
        let a = &self.actors[who];
        let want = (a.pos + a.facing * 1.1).xz();
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

    fn place(&mut self, who: usize) {
        let at = self.place_target(who);
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
    }

    /// The nearest free, carryable crate in reach of `who`.
    fn nearest_free(&self, who: usize, reach: f32, kind: Kind) -> Option<(u32, f32)> {
        let a = &self.actors[who];
        self.crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == kind)
            .filter(|c| c.pos.y - a.pos.y < 3.0)
            .map(|c| (c.id, c.pos.xz().distance(a.pos.xz())))
            .filter(|(_, d)| *d <= reach + HALF)
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }

    fn hurt(&mut self, who: usize, amount: f32, knock: Vec3, tune: &Tune) {
        if self.actors[who].dead > 0.0 {
            return;
        }
        if tune.c(P::HitDropsCrate) == 0 && self.actors[who].carrying.is_some() {
            self.release(who, knock * 0.3 + Vec3::Y * 2.0, Vec3::ZERO);
            if who == PLAYER {
                self.pending = None;
            }
        }
        let a = &mut self.actors[who];
        a.vel += knock;
        a.stun = a.stun.max(0.15);
        if who == PLAYER {
            self.times_hit += 1;
            return;
        }
        let t = self.t;
        let a = &mut self.actors[who];
        a.first_hit.get_or_insert(t);
        a.hp -= amount;
        if a.hp <= 0.0 {
            let ttk = t - a.first_hit.take().unwrap_or(t);
            a.dead = 2.0;
            self.last_ttk = Some(ttk);
            self.say(format!("bot down, time to kill {ttk:.2}s"));
        }
    }
}

fn pad_contains(pad: Vec2, p: Vec3) -> bool {
    (p.x - pad.x).abs() < PAD_HALF && (p.z - pad.y).abs() < PAD_HALF
}

fn throw_velocity(dir: Vec3, tune: &Tune) -> Vec3 {
    let angle = tune.f(P::ThrowAngle).to_radians();
    let dist = tune.f(P::ThrowDist);
    // Level-ground range formula: d = v^2 sin(2θ) / g.
    let speed = (dist * G / (2.0 * angle).sin()).sqrt();
    dir * speed * angle.cos() + Vec3::Y * speed * angle.sin()
}

// ---------------------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------------------

#[derive(Resource, Default)]
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
}

#[derive(Component)]
struct MainCamera;

fn read_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
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
    if motion.read().count() > 0 || mouse.just_pressed(MouseButton::Left) {
        c.use_mouse = true;
    }
    let mut stick_aim = None;
    for pad in &gamepads {
        let ls = pad.left_stick();
        if ls.length() > 0.15 { mv = ls; }
        let rs = pad.right_stick();
        if rs.length() > 0.3 {
            stick_aim = Some(Vec3::new(rs.x, 0.0, -rs.y).normalize());
            c.use_mouse = false;
        }
        c.jump |= pad.just_pressed(GamepadButton::South);
        c.grab |= pad.just_pressed(GamepadButton::West);
        c.throw |= pad.just_pressed(GamepadButton::RightTrigger);
        c.fire |= pad.pressed(GamepadButton::RightTrigger2);
        c.kick_held |= pad.pressed(GamepadButton::East);
        c.kick |= pad.just_pressed(GamepadButton::East);
    }
    if std::env::var("CORE_FEEL_AUTOPLAY").is_ok() {
        autoplay(&sim, &mut c);
        return;
    }
    c.mv = mv.clamp_length_max(1.0);
    c.aim = stick_aim;
    if c.use_mouse && stick_aim.is_none() {
        let (cam, cam_tf) = *camera;
        if let Some(cursor) = window.cursor_position()
            && let Ok(ray) = cam.viewport_to_world(cam_tf, cursor)
            && let Some(d) = ray.intersect_plane(Vec3::Y * CHEST, InfinitePlane3d::new(Vec3::Y))
        {
            let p = ray.get_point(d);
            let to = (p - sim.actors[PLAYER].pos).with_y(0.0);
            if to.length() > 0.2 {
                c.aim = Some(to.normalize());
            }
        }
    }
}

/// Smoke-test pilot: walk to the nearest crate, grab it, throw it at the bot, shoot,
/// and kick open loot crates. Only with `CORE_FEEL_AUTOPLAY` set.
fn autoplay(sim: &Sim, c: &mut Controls) {
    let a = &sim.actors[PLAYER];
    let phase = (sim.t * 4.0) as u32;
    let edge = (sim.t * 4.0).fract() < 0.1;
    let target = sim
        .crates
        .iter()
        .filter(|cr| cr.state == CrateState::Free && !pad_contains(PLAYER_PAD, cr.pos))
        .filter(|cr| cr.kind == Kind::Normal || std::env::var("CORE_FEEL_AUTOPLAY").as_deref() != Ok("stack"))
        .min_by(|x, y| x.pos.distance(a.pos).total_cmp(&y.pos.distance(a.pos)));
    let to_bot = (sim.actors[BOT].pos - a.pos).with_y(0.0).normalize_or_zero();
    *c = Controls::default();
    if a.carrying.is_some() && std::env::var("CORE_FEEL_AUTOPLAY").as_deref() == Ok("stack") {
        // Fill a 2x2 block of grid cells on the pad, one crate per cell, from the side.
        let placed = sim
            .crates
            .iter()
            .filter(|cr| cr.state == CrateState::Free && pad_contains(PLAYER_PAD, cr.pos))
            .count();
        let cells = [(-12.0, 0.0, -1.0), (-12.8, 0.0, -1.0), (-12.0, 0.8, 1.0), (-12.8, 0.8, 1.0)];
        let (x, z, side) = cells[placed % 4];
        let stand = Vec3::new(x, 0.0, z + side * 1.1);
        let to = (stand - a.pos).with_y(0.0);
        c.aim = Some(Vec3::Z * -side);
        if to.length() > 0.2 {
            c.mv = Vec2::new(to.x, -to.z).normalize_or_zero();
        } else {
            c.grab = edge;
        }
    } else if a.carrying.is_some() {
        c.aim = Some(to_bot);
        c.throw = edge && phase % 4 == 0;
        c.jump = edge && phase % 4 == 3;
    } else if let Some(cr) = target {
        let to = (cr.pos - a.pos).with_y(0.0);
        c.mv = Vec2::new(to.x, -to.z).normalize_or_zero();
        c.aim = Some(to_bot);
        c.grab = edge && to.length() < 1.5;
        c.fire = std::env::var("CORE_FEEL_AUTOPLAY").as_deref() != Ok("stack");
        c.kick_held = to.length() < 1.6;
        c.kick = c.kick_held && edge;
    }
}

fn tune_input(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut tune: ResMut<Tune>,
    mut sim: ResMut<Sim>,
) {
    let any = |k: KeyCode, b: GamepadButton| keys.just_pressed(k) || gamepads.iter().any(|g| g.just_pressed(b));
    let n = tune.slots[0].len();
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
        let (active, sel) = (tune.active, tune.selected);
        match &mut tune.slots[active][sel].value {
            Value::Num { v, min, max, step, .. } => {
                *v = (*v + *step * mult * dir as f32).clamp(*min, *max);
            }
            Value::Choice { i, opts } => {
                *i = (*i as i32 + dir).rem_euclid(opts.len() as i32) as usize;
            }
        }
    }
    if any(KeyCode::Tab, GamepadButton::Select) {
        tune.active = 1 - tune.active;
        let slot = if tune.active == 0 { "A" } else { "B" };
        sim.say(format!("switched to slot {slot}"));
    }
    if keys.just_pressed(KeyCode::KeyH) {
        tune.panel = !tune.panel;
    }
    if any(KeyCode::KeyR, GamepadButton::Start) {
        sim.reset();
        sim.say("arena reset");
    }
    if keys.just_pressed(KeyCode::KeyP) {
        let slot = if tune.active == 0 { "A" } else { "B" };
        println!("# core_feel baseline, slot {slot}");
        for p in &tune.slots[tune.active] {
            println!("{:<20} = {}", p.name, fmt_value(&p.value));
        }
        sim.say("printed active slot to the terminal");
    }
}

// ---------------------------------------------------------------------------------------
// Simulation step
// ---------------------------------------------------------------------------------------

fn step(time: Res<Time>, controls: Res<Controls>, tune: Res<Tune>, mut sim: ResMut<Sim>) {
    let dt = time.delta_secs().min(1.0 / 30.0);
    sim.t += dt;
    let s = &mut *sim;

    for m in &mut s.msgs { m.1 -= dt; }
    s.msgs.retain(|m| m.1 > 0.0);
    for t in &mut s.tracers { t.2 -= dt; }
    s.tracers.retain(|t| t.2 > 0.0);
    for t in &mut s.swings { t.2 -= dt; }
    s.swings.retain(|t| t.2 > 0.0);

    player_step(s, &controls, &tune, dt);
    bot_step(s, &tune, dt);
    actors_physics(s, &tune, dt);
    crates_physics(s, &tune, dt);
    shots_step(s, &tune, dt);
    craft(s, &tune, dt);
    refill(s, dt);
}

fn player_step(s: &mut Sim, c: &Controls, tune: &Tune, dt: f32) {
    let carrying = s.actors[PLAYER].carrying.is_some();
    {
        let a = &mut s.actors[PLAYER];
        a.fire_cd -= dt;
        a.stun -= dt;
        if let Some(aim) = c.aim {
            a.facing = aim;
        } else if c.mv.length() > 0.1 {
            a.facing = Vec3::new(c.mv.x, 0.0, -c.mv.y).normalize();
        }
        let speed = tune.f(P::MoveSpeed) * if carrying { tune.f(P::CarrySpeed) } else { 1.0 };
        let mut target = Vec3::new(c.mv.x, 0.0, -c.mv.y) * speed;
        if a.stun > 0.0 { target = Vec3::ZERO; }
        // Lunge mask: during the grab round trip, step towards the crate instead.
        if let Some(p) = &s.pending
            && tune.c(P::GrabMask) == 2
            && let Some(cr) = s.crates.iter().find(|cr| cr.id == p.crate_id)
        {
            let to = (cr.pos - a.pos).with_y(0.0);
            let d = to.length() - (HALF + ACTOR_R + 0.1);
            target = if d > 0.0 { to.normalize() * (d / p.left.max(0.02)).min(18.0) } else { Vec3::ZERO };
        }
        let k = 1.0 - (-dt * 18.0).exp();
        let h = a.vel.with_y(0.0).lerp(target, k);
        a.vel = h.with_y(a.vel.y);
        if c.jump && a.grounded {
            a.vel.y = tune.f(P::JumpVel);
            a.grounded = false;
        }
    }

    // Grab confirmation after the fake round trip: the "server" re-checks.
    if let Some(p) = &mut s.pending {
        p.left -= dt;
        if p.left <= 0.0 {
            let id = p.crate_id;
            s.pending = None;
            let reach = tune.f(P::GrabReach);
            let a_pos = s.actors[PLAYER].pos;
            match s.crates.iter().find(|cr| cr.id == id) {
                Some(cr) if cr.state == CrateState::Free && cr.pos.xz().distance(a_pos.xz()) <= reach + HALF + 0.5 => {
                    s.take(PLAYER, id);
                    s.grabs += 1;
                }
                Some(cr) if cr.state != CrateState::Free => {
                    s.denied += 1;
                    s.say("GRAB DENIED: the bot got it first");
                }
                _ => {
                    s.denied += 1;
                    s.say("GRAB DENIED: out of reach when the server checked");
                }
            }
        }
    }

    if s.actors[PLAYER].dead > 0.0 || s.actors[PLAYER].stun > 0.0 {
        return;
    }
    let facing = s.actors[PLAYER].facing;
    let carrying = s.actors[PLAYER].carrying.is_some();

    if c.grab {
        if carrying {
            // Place: set it down on the ghost shown in front (snaps onto stacks).
            s.place(PLAYER);
        } else if s.pending.is_none()
            && let Some((id, _)) = s.nearest_free(PLAYER, tune.f(P::GrabReach), Kind::Normal)
        {
            let lat = tune.f(P::GrabLatency) / 1000.0;
            if lat <= 0.0 {
                s.take(PLAYER, id);
                s.grabs += 1;
            } else {
                s.pending = Some(Pending { crate_id: id, left: lat, total: lat });
            }
        }
    }

    if c.throw && carrying {
        let a = &s.actors[PLAYER];
        let jump_mode = tune.c(P::ThrowStyle) == 1;
        if jump_mode && a.grounded {
            // Jump-throw: on the ground you can only toss it a short way.
            s.release(PLAYER, facing * 3.0 + Vec3::Y * 3.0, facing * 0.5);
        } else {
            let inherit = a.vel.with_y(0.0);
            s.release(PLAYER, throw_velocity(facing, tune) + inherit, facing * 0.3);
        }
    }

    if c.fire && s.actors[PLAYER].fire_cd <= 0.0 {
        let disarm = tune.c(P::CarryDisarm);
        let rate = tune.f(P::FireRate);
        match (carrying, disarm) {
            (true, 0) => {}
            (true, 2) => {
                // Melee shove in front.
                s.actors[PLAYER].fire_cd = 0.5;
                let from = s.actors[PLAYER].pos + Vec3::Y * CHEST;
                s.swings.push((from, facing, 0.15));
                let to_bot = (s.actors[BOT].pos - s.actors[PLAYER].pos).with_y(0.0);
                if to_bot.length() < 2.2 && to_bot.normalize_or_zero().dot(facing) > 0.4 {
                    s.hits += 1;
                    let dmg = 200.0 / tune.f(P::ShotsToKill);
                    s.hurt(BOT, dmg, facing * 9.0 + Vec3::Y * 3.0, tune);
                }
                s.fired += 1;
            }
            _ => {
                let rate = if carrying { rate * 0.5 } else { rate };
                s.actors[PLAYER].fire_cd = 1.0 / rate;
                fire(s, PLAYER, facing, tune);
            }
        }
    }

    // Kick: hold on a loot crate to open it, tap a normal crate to punt it.
    if !carrying {
        let pos = s.actors[PLAYER].pos;
        let kick_time = tune.f(P::KickTime);
        let mut opened = None;
        let mut punted = false;
        for cr in s.crates.iter_mut().filter(|cr| cr.state == CrateState::Free) {
            let near = cr.pos.xz().distance(pos.xz()) < 1.6 && cr.pos.y - pos.y < 1.5;
            match cr.kind {
                Kind::Loot if near && c.kick_held => {
                    cr.kick += dt;
                    if c.kick || cr.kick >= kick_time {
                        if cr.kick >= kick_time || kick_time <= 0.0 {
                            opened = Some((cr.id, cr.pos));
                        }
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

fn fire(s: &mut Sim, who: usize, dir: Vec3, tune: &Tune) {
    let from = s.actors[who].pos + Vec3::Y * CHEST + dir * 0.5;
    s.fired += (who == PLAYER) as u32;
    if tune.c(P::Weapon) == 0 && who == PLAYER {
        let range = 30.0;
        let b = &s.actors[BOT];
        let rel = (b.pos - s.actors[who].pos).with_y(0.0);
        let along = rel.dot(dir);
        let side = (rel - dir * along).length();
        let hit = b.dead <= 0.0 && along > 0.0 && along < range && side < ACTOR_R + 0.15;
        let end = if hit { from + dir * along } else { from + dir * range };
        s.tracers.push((from, end, 0.06));
        if hit {
            s.hits += 1;
            let dmg = 100.0 / tune.f(P::ShotsToKill);
            s.hurt(BOT, dmg, dir * 1.5, tune);
        }
    } else {
        let speed = if who == PLAYER { tune.f(P::ProjSpeed) } else { 14.0 };
        s.shots.push(Shot { pos: from, vel: dir * speed, owner: who, life: 30.0 / speed, counted: true, ent: None });
    }
}

fn bot_step(s: &mut Sim, tune: &Tune, dt: f32) {
    let mode = tune.c(P::BotMode);
    let t = s.t;
    {
        let b = &mut s.actors[BOT];
        b.stun -= dt;
        if b.dead > 0.0 {
            b.dead -= dt;
            if b.dead <= 0.0 {
                *b = Actor { pos: BOT_SPAWN, facing: -Vec3::X, hp: 100.0, ..default() };
            }
            return;
        }
    }
    let carrying = s.actors[BOT].carrying.is_some();
    let speed = tune.f(P::MoveSpeed) * 0.75 * if carrying { tune.f(P::CarrySpeed) } else { 1.0 };
    let pos = s.actors[BOT].pos;
    let goal = match mode {
        0 => BOT_SPAWN,
        1 | 3 => Vec3::new(BOT_SPAWN.x, 0.0, 6.0 * (t * 0.5).sin()),
        _ => {
            if carrying {
                let pad = Vec3::new(BOT_PAD.x, 0.0, BOT_PAD.y);
                if pos.xz().distance(pad.xz()) < 1.0 {
                    s.release(BOT, Vec3::ZERO, -Vec3::X * 0.2);
                }
                pad
            } else {
                // Nearest free crate not already on its own pad (so it steals from yours).
                let target = s
                    .crates
                    .iter()
                    .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal)
                    .filter(|c| !pad_contains(BOT_PAD, c.pos))
                    .min_by(|a, b| a.pos.distance(pos).total_cmp(&b.pos.distance(pos)))
                    .map(|c| (c.id, c.pos));
                match target {
                    Some((id, at)) if at.xz().distance(pos.xz()) < 1.2 && at.y < 3.0 => {
                        // The bot is the server: its grabs are instant.
                        s.take(BOT, id);
                        at
                    }
                    Some((_, at)) => at,
                    None => BOT_SPAWN,
                }
            }
        }
    };
    let b = &mut s.actors[BOT];
    let to = (goal - b.pos).with_y(0.0);
    let target = if to.length() > 0.3 && b.stun <= 0.0 { to.normalize() * speed } else { Vec3::ZERO };
    let k = 1.0 - (-dt * 10.0).exp();
    b.vel = b.vel.with_y(0.0).lerp(target, k).with_y(b.vel.y);
    if to.length() > 0.3 {
        b.facing = to.normalize();
    }
    if mode == 3 {
        let to_p = (s.actors[PLAYER].pos - s.actors[BOT].pos).with_y(0.0).normalize_or_zero();
        s.actors[BOT].facing = to_p;
        s.bot_fire_cd -= dt;
        if s.bot_fire_cd <= 0.0 {
            s.bot_fire_cd = 1.4;
            fire(s, BOT, to_p, tune);
        }
    }
}

fn actors_physics(s: &mut Sim, _tune: &Tune, dt: f32) {
    let blockers: Vec<Vec3> = s
        .crates
        .iter()
        .filter(|c| c.state == CrateState::Free && c.pos.y < CRATE * 1.5)
        .map(|c| c.pos)
        .chain(s.turrets.iter().map(|t| t.pos + Vec3::Y * HALF))
        .collect();
    for a in &mut s.actors {
        a.vel.y -= G * dt;
        a.pos += a.vel * dt;
        if a.pos.y <= 0.0 {
            a.pos.y = 0.0;
            a.vel.y = 0.0;
            a.grounded = true;
        } else {
            a.grounded = false;
        }
        // Push out of resting crates (treated as circles; good enough to feel them).
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
        a.pos.x = a.pos.x.clamp(-ARENA.x + ACTOR_R, ARENA.x - ACTOR_R);
        a.pos.z = a.pos.z.clamp(-ARENA.y + ACTOR_R, ARENA.y - ACTOR_R);
    }
}

fn crates_physics(s: &mut Sim, tune: &Tune, dt: f32) {
    let snapshot: Vec<(u32, Vec3, bool)> =
        s.crates.iter().map(|c| (c.id, c.pos, c.state == CrateState::Free)).collect();
    let turrets: Vec<Vec2> = s.turrets.iter().map(|t| t.pos.xz()).collect();
    let mut thrown_hits = Vec::new();
    for c in &mut s.crates {
        match &mut c.state {
            CrateState::Carried { by, lift_from, lift_t } => {
                *lift_t += dt;
                let head = s.actors[*by].pos + Vec3::Y * HEAD;
                let k = (*lift_t / 0.1).min(1.0);
                c.pos = lift_from.lerp(head, k);
            }
            CrateState::Free => {
                c.vel.y -= G * dt;
                c.pos += c.vel * dt;
                let mut floor: f32 = 0.0;
                if turrets.iter().any(|t| t.distance(c.pos.xz()) < 1.1) && c.pos.y > TURRET_H {
                    floor = TURRET_H;
                }
                for (id, p, free) in &snapshot {
                    if *id == c.id || !*free {
                        continue;
                    }
                    let overlap = (p.x - c.pos.x).abs() < CRATE * 0.9 && (p.z - c.pos.z).abs() < CRATE * 0.9;
                    if overlap && p.y < c.pos.y {
                        floor = floor.max(p.y + HALF);
                    }
                }
                if c.pos.y - HALF <= floor {
                    c.pos.y = floor + HALF;
                    c.vel.y = if c.vel.y < -6.0 { -c.vel.y * 0.25 } else { 0.0 };
                    let f = (-dt * 8.0).exp();
                    c.vel.x *= f;
                    c.vel.z *= f;
                    c.thrown_by = None;
                }
                if c.pos.x.abs() > ARENA.x - HALF {
                    c.pos.x = c.pos.x.clamp(-ARENA.x + HALF, ARENA.x - HALF);
                    c.vel.x *= -0.4;
                }
                if c.pos.z.abs() > ARENA.y - HALF {
                    c.pos.z = c.pos.z.clamp(-ARENA.y + HALF, ARENA.y - HALF);
                    c.vel.z *= -0.4;
                }
                if let Some(by) = c.thrown_by {
                    let other = 1 - by;
                    let o = &s.actors[other];
                    if o.dead <= 0.0
                        && c.pos.xz().distance(o.pos.xz()) < ACTOR_R + HALF
                        && c.pos.y < o.pos.y + 2.0
                    {
                        thrown_hits.push((other, c.vel.with_y(0.0).normalize_or_zero()));
                        c.thrown_by = None;
                        c.vel *= -0.2;
                    }
                }
            }
        }
    }
    for (who, dir) in thrown_hits {
        if who == BOT { s.hits += 1; }
        let dmg = 200.0 / tune.f(P::ShotsToKill);
        s.hurt(who, dmg, dir * 6.0, tune);
        s.say(if who == BOT { "crate hit the bot" } else { "the bot hit you with a crate" });
    }
}

fn shots_step(s: &mut Sim, tune: &Tune, dt: f32) {
    let mut hits = Vec::new();
    for sh in &mut s.shots {
        sh.pos += sh.vel * dt;
        sh.life -= dt;
        let target = 1 - sh.owner;
        let a = &s.actors[target];
        if a.dead <= 0.0
            && sh.pos.xz().distance(a.pos.xz()) < ACTOR_R + 0.15
            && sh.pos.y > a.pos.y
            && sh.pos.y < a.pos.y + 1.9
        {
            hits.push((target, sh.vel.normalize(), sh.counted));
            sh.life = 0.0;
        }
    }
    s.shots.retain(|sh| {
        if sh.life <= 0.0 { s.despawn.extend(sh.ent); }
        sh.life > 0.0
    });
    for (who, dir, counted) in hits {
        if who == BOT && counted { s.hits += 1; }
        let dmg = 100.0 / tune.f(P::ShotsToKill);
        s.hurt(who, dmg, dir * 1.5, tune);
    }
}

/// PLACEHOLDER (not a grammar decision, see #17): four resting crates on your pad in a 2x2
/// block on the same level merge into a turret.
fn craft(s: &mut Sim, tune: &Tune, dt: f32) {
    for t in &mut s.turrets {
        t.life -= dt;
        t.cd -= dt;
        let b = &s.actors[BOT];
        let to = (b.pos - t.pos).with_y(0.0);
        if b.dead <= 0.0 && to.length() < 14.0 {
            t.facing = to.normalize_or(Vec3::X);
            if t.cd <= 0.0 {
                t.cd = 0.8;
                let from = t.pos + Vec3::Y * 0.4 + t.facing * 0.6;
                s.shots.push(Shot { pos: from, vel: t.facing * 18.0, owner: PLAYER, life: 1.0, counted: false, ent: None });
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
    let cells: Vec<(IVec3, u32)> = s
        .crates
        .iter()
        .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && c.vel.length() < 0.1)
        .filter(|c| pad_contains(PLAYER_PAD, c.pos))
        .filter(|c| (c.pos.xz() / CRATE - (c.pos.xz() / CRATE).round()).length() < 0.05)
        .map(|c| {
            let g = (c.pos / CRATE - Vec3::Y * 0.5).round().as_ivec3();
            (g, c.id)
        })
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
                pos: Vec3::new(centre.x, 0.0, centre.z),
                facing: Vec3::X,
                cd: 0.5,
                life: 40.0,
                ent: None,
            });
            println!("craft: 2x2 -> turret at ({:.1}, {:.1}) t={:.1}s", centre.x, centre.z, s.t);
            s.say("2x2 crafted a TURRET (placeholder, 40 s)");
            return;
        }
    }
}

fn refill(s: &mut Sim, dt: f32) {
    let loose = s
        .crates
        .iter()
        .filter(|c| c.kind == Kind::Normal)
        .filter(|c| c.state != CrateState::Free || !(pad_contains(PLAYER_PAD, c.pos) || pad_contains(BOT_PAD, c.pos)))
        .count();
    if loose < 4 {
        let x = -6.0 + s.rand() * 12.0;
        let z = -8.0 + s.rand() * 16.0;
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
// Presentation
// ---------------------------------------------------------------------------------------

#[derive(Resource)]
struct Looks {
    crate_mesh: Handle<Mesh>,
    normal: Handle<StandardMaterial>,
    loot: Handle<StandardMaterial>,
    shot_mesh: Handle<Mesh>,
    shot_mat: Handle<StandardMaterial>,
    bot_shot_mat: Handle<StandardMaterial>,
    turret_base: Handle<Mesh>,
    turret_barrel: Handle<Mesh>,
    turret_mat: Handle<StandardMaterial>,
}

#[derive(Component)]
struct ActorView(usize);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Panel;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_translation(PLAYER_SPAWN + CAM_OFFSET).looking_at(PLAYER_SPAWN, Vec3::Y),
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
    for (pad, color) in [(PLAYER_PAD, Color::srgb(0.2, 0.4, 0.9)), (BOT_PAD, Color::srgb(0.9, 0.25, 0.2))] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(PAD_HALF * 2.0, 0.02, PAD_HALF * 2.0))),
            MeshMaterial3d(mats.add(color.with_alpha(0.6))),
            Transform::from_xyz(pad.x, 0.01, pad.y),
        ));
    }
    let body = meshes.add(Capsule3d::new(ACTOR_R, 1.0));
    let nose = meshes.add(Cuboid::new(0.25, 0.25, 0.4));
    for (i, color) in [(PLAYER, Color::srgb(0.3, 0.55, 1.0)), (BOT, Color::srgb(1.0, 0.35, 0.3))] {
        let mat = mats.add(color);
        commands
            .spawn((ActorView(i), Transform::default(), Visibility::default()))
            .with_children(|p| {
                p.spawn((Mesh3d(body.clone()), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.0, 0.95, 0.0)));
                p.spawn((Mesh3d(nose.clone()), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.0, 1.3, -0.5)));
            });
    }
    commands.insert_resource(Looks {
        crate_mesh: meshes.add(Cuboid::from_length(CRATE)),
        normal: mats.add(Color::srgb(0.75, 0.6, 0.35)),
        loot: mats.add(Color::srgb(1.0, 0.5, 0.05)),
        shot_mesh: meshes.add(Sphere::new(0.15)),
        shot_mat: mats.add(StandardMaterial { emissive: LinearRgba::rgb(2.0, 2.0, 0.5), ..default() }),
        bot_shot_mat: mats.add(StandardMaterial { emissive: LinearRgba::rgb(3.0, 0.4, 0.2), ..default() }),
        turret_base: meshes.add(Cylinder::new(0.7, 0.8)),
        turret_barrel: meshes.add(Cuboid::new(0.25, 0.25, 1.2)),
        turret_mat: mats.add(Color::srgb(0.25, 0.45, 0.95)),
    });
    commands.spawn((
        Hud,
        Text::new(""),
        TextFont::from_font_size(15.0),
        Node { position_type: PositionType::Absolute, top: px(10.0), left: px(10.0), ..default() },
    ));
    commands.spawn((
        Panel,
        Text::new(""),
        TextFont::from_font_size(14.0),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            right: px(10.0),
            padding: UiRect::all(px(8.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.6)),
    ));
}

fn sync_views(
    mut commands: Commands,
    mut sim: ResMut<Sim>,
    looks: Res<Looks>,
    mut actors: Query<(&ActorView, &mut Transform, &mut Visibility)>,
    mut tfs: Query<&mut Transform, Without<ActorView>>,
) {
    for e in sim.despawn.drain(..) {
        commands.entity(e).despawn();
    }
    for (v, mut tf, mut vis) in &mut actors {
        let a = &sim.actors[v.0];
        tf.translation = a.pos;
        tf.rotation = Transform::IDENTITY.looking_to(a.facing, Vec3::Y).rotation;
        *vis = if a.dead > 0.0 { Visibility::Hidden } else { Visibility::Visible };
    }
    for c in &mut sim.crates {
        match c.ent {
            Some(e) => {
                if let Ok(mut tf) = tfs.get_mut(e) {
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
                if let Ok(mut tf) = tfs.get_mut(e) {
                    tf.rotation = rot;
                }
            }
            None => {
                let (base, barrel, mat) = (looks.turret_base.clone(), looks.turret_barrel.clone(), looks.turret_mat.clone());
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
                if let Ok(mut tf) = tfs.get_mut(e) {
                    tf.translation = sh.pos;
                }
            }
            None => {
                let mat = if sh.owner == PLAYER { looks.shot_mat.clone() } else { looks.bot_shot_mat.clone() };
                sh.ent = Some(
                    commands
                        .spawn((Mesh3d(looks.shot_mesh.clone()), MeshMaterial3d(mat), Transform::from_translation(sh.pos)))
                        .id(),
                );
            }
        }
    }
}

fn follow_camera(time: Res<Time>, sim: Res<Sim>, mut cam: Single<&mut Transform, With<MainCamera>>) {
    let p = sim.actors[PLAYER].pos.with_y(0.0);
    let want = p + CAM_OFFSET;
    let k = 1.0 - (-time.delta_secs() * 6.0).exp();
    cam.translation = cam.translation.lerp(want, k);
    let look = cam.translation - CAM_OFFSET;
    cam.look_at(look, Vec3::Y);
}

fn ground_circle(p: Vec3) -> Isometry3d {
    Isometry3d::new(p, Quat::from_rotation_x(FRAC_PI_2))
}

fn draw_gizmos(mut g: Gizmos, sim: Res<Sim>, tune: Res<Tune>, controls: Res<Controls>) {
    let a = &sim.actors[PLAYER];
    let chest = a.pos + Vec3::Y * CHEST;
    // Aim line.
    g.line(chest, chest + a.facing * 3.0, Color::srgba(1.0, 1.0, 1.0, 0.4));
    // Grab reach ring.
    if a.carrying.is_none() {
        g.circle(ground_circle(a.pos + Vec3::Y * 0.03), tune.f(P::GrabReach) + HALF, Color::srgba(1.0, 1.0, 1.0, 0.15));
        if sim.pending.is_none()
            && let Some((id, _)) = sim.nearest_free(PLAYER, tune.f(P::GrabReach), Kind::Normal)
            && let Some(c) = sim.crates.iter().find(|c| c.id == id)
        {
            g.cube(Transform::from_translation(c.pos).with_scale(Vec3::splat(CRATE * 1.08)), Color::WHITE);
        }
    }
    // Pending grab (fake round trip) and its mask.
    if let Some(p) = &sim.pending
        && let Some(c) = sim.crates.iter().find(|c| c.id == p.crate_id)
    {
        let progress = 1.0 - (p.left / p.total).clamp(0.0, 1.0);
        match tune.c(P::GrabMask) {
            1 => {
                let hand = chest.lerp(c.pos, progress);
                g.line(chest, hand, Color::srgb(0.3, 0.55, 1.0));
                g.sphere(Isometry3d::from_translation(hand), 0.15, Color::srgb(0.3, 0.55, 1.0));
            }
            _ => {
                g.circle(ground_circle(a.pos + Vec3::Y * 0.05), 0.8 * (1.0 - progress) + 0.1, Color::srgb(1.0, 1.0, 0.2));
            }
        }
    }
    // Placement ghost: where E / X will set the carried crate down.
    if a.carrying.is_some() {
        let at = sim.place_target(PLAYER);
        g.cube(Transform::from_translation(at).with_scale(Vec3::splat(CRATE)), Color::srgb(0.6, 1.0, 1.0));
        g.line(a.pos + Vec3::Y * HEAD, at, Color::srgba(0.6, 1.0, 1.0, 0.4));
    }
    // Throw arc preview.
    if a.carrying.is_some() && tune.c(P::ArcPreview) == 0 {
        let short = tune.c(P::ThrowStyle) == 1 && a.grounded;
        let (mut p, mut v) = if short {
            (a.pos + Vec3::Y * HEAD + a.facing * 0.5, a.facing * 3.0 + Vec3::Y * 3.0)
        } else {
            (a.pos + Vec3::Y * HEAD + a.facing * 0.3, throw_velocity(a.facing, &tune) + a.vel.with_y(0.0))
        };
        let color = if short { Color::srgb(1.0, 0.4, 0.2) } else { Color::srgb(0.4, 1.0, 0.4) };
        for _ in 0..80 {
            let dt = 1.0 / 60.0;
            let nv = v - Vec3::Y * G * dt;
            let np = p + nv * dt;
            g.line(p, np, color);
            p = np;
            v = nv;
            if p.y < HALF {
                g.circle(ground_circle(p.with_y(0.03)), HALF, color);
                break;
            }
        }
    }
    // Kick-open progress on loot crates.
    let kt = tune.f(P::KickTime).max(0.01);
    for c in sim.crates.iter().filter(|c| c.kind == Kind::Loot && c.kick > 0.0) {
        let r = 0.2 + 0.6 * (c.kick / kt).min(1.0);
        g.circle(ground_circle(c.pos.with_y(0.04)), r, Color::srgb(1.0, 0.7, 0.1));
    }
    for (from, to, _) in &sim.tracers {
        g.line(*from, *to, Color::srgb(1.0, 1.0, 0.5));
    }
    for (from, dir, _) in &sim.swings {
        g.arc_3d(
            1.4,
            2.0,
            Isometry3d::new(*from, Quat::from_rotation_arc(Vec3::Z, *dir) * Quat::from_rotation_y(-0.7)),
            Color::WHITE,
        );
    }
    for pad in [PLAYER_PAD, BOT_PAD] {
        g.rect(
            Isometry3d::new(Vec3::new(pad.x, 0.03, pad.y), Quat::from_rotation_x(FRAC_PI_2)),
            Vec2::splat(PAD_HALF * 2.0),
            Color::WHITE,
        );
    }
    let _ = &controls;
}

fn hud(sim: Res<Sim>, tune: Res<Tune>, mut hud: Single<&mut Text, (With<Hud>, Without<Panel>)>) {
    let a = &sim.actors[PLAYER];
    let b = &sim.actors[BOT];
    let pad_count = |pad: Vec2| {
        sim.crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind == Kind::Normal && pad_contains(pad, c.pos))
            .count()
    };
    let slot = if tune.active == 0 { "A" } else { "B" };
    let mut s = format!("SLOT {slot}  (Tab / Select swaps)\n");
    s += &format!(
        "you: {}{}{}\n",
        if a.carrying.is_some() { "CARRYING" } else { "empty-handed" },
        if a.grounded { "" } else { ", airborne" },
        match &sim.pending {
            Some(p) => format!(", grab in flight {:.0} ms", p.left * 1000.0),
            None => String::new(),
        }
    );
    let tallest = |pad: Vec2| {
        sim.crates
            .iter()
            .filter(|c| c.state == CrateState::Free && pad_contains(pad, c.pos))
            .map(|c| ((c.pos.y + HALF) / CRATE).round() as u32)
            .max()
            .unwrap_or(0)
    };
    s += &format!(
        "your pad: {} crates, tallest stack {}   bot pad: {} crates\n",
        pad_count(PLAYER_PAD),
        tallest(PLAYER_PAD),
        pad_count(BOT_PAD)
    );
    s += &format!(
        "bot: {} hp {:.0}{}   last TTK: {}\n",
        ["dummy", "strafer", "hauler", "duelist"][tune.c(P::BotMode)],
        b.hp.max(0.0),
        if b.dead > 0.0 { " (respawning)" } else { "" },
        sim.last_ttk.map_or("-".into(), |t| format!("{t:.2}s")),
    );
    let acc = if sim.fired > 0 { 100.0 * sim.hits as f32 / sim.fired as f32 } else { 0.0 };
    s += &format!(
        "shots {} hits {} ({acc:.0}%)   you were hit {}   grabs {} denied {}\n",
        sim.fired, sim.hits, sim.times_hit, sim.grabs, sim.denied
    );
    for (m, _) in &sim.msgs {
        s += &format!("> {m}\n");
    }
    s += "\nWASD/stick move  mouse/RS aim  Space/A jump  E/X grab / place on ghost  RMB/RB throw\n";
    s += "LMB/RT shoot  F/B kick (hold on orange)  R/Start reset  P print slot  H panel";
    hud.0 = s;
}

fn panel(tune: Res<Tune>, mut panel: Single<(&mut Text, &mut Visibility), (With<Panel>, Without<Hud>)>) {
    let (text, vis) = &mut *panel;
    **vis = if tune.panel { Visibility::Visible } else { Visibility::Hidden };
    let slot = if tune.active == 0 { "A" } else { "B" };
    let other = 1 - tune.active;
    let mut s = format!("TUNING slot {slot}   (* = differs from other slot)\n\n");
    for (i, p) in tune.slots[tune.active].iter().enumerate() {
        let cursor = if i == tune.selected { ">" } else { " " };
        let diff = if same(&p.value, &tune.slots[other][i].value) { " " } else { "*" };
        s += &format!("{cursor}{diff} {:<18} {}\n", p.name, fmt_value(&p.value));
    }
    s += "\nUp/Down pick  Left/Right change (Shift x5)";
    text.0 = s;
}

#[derive(Resource, Default)]
struct Smoke {
    exit_at: Option<Duration>,
    shot: bool,
}

fn smoke_exit(
    time: Res<Time>,
    mut smoke: ResMut<Smoke>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    let Some(secs) = std::env::var("CORE_FEEL_EXIT_AFTER").ok().and_then(|s| s.parse::<f32>().ok()) else {
        return;
    };
    let deadline = *smoke.exit_at.get_or_insert(Duration::from_secs_f32(secs));
    if let Ok(path) = std::env::var("CORE_FEEL_SCREENSHOT")
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
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "big_back core feel (PROTOTYPE)".into(),
                // Smoke runs often land on a hidden workspace, where vsync blocks forever.
                present_mode: if std::env::var("CORE_FEEL_EXIT_AFTER").is_ok() {
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
        .insert_resource(Tune { slots: [defaults(), defaults()], active: 0, selected: 0, panel: true })
        .insert_resource(Sim::new())
        .init_resource::<Controls>()
        .init_resource::<Smoke>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (read_controls, tune_input, step, sync_views, follow_camera, draw_gizmos, hud, panel, smoke_exit).chain(),
        )
        .run();
}
