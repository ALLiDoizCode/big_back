//! PROTOTYPE — throwaway. Answers: should the arena be a bigger, League-of-Legends-like map
//! (bases at each end, three lanes, a walled jungle with entrances, a river) with PvE camps
//! that drop heals and shields? Not production code; lives on `prototype/arena-map`,
//! branched from `prototype/crate-grammar` (#17), whose rules all still work here.
//!
//!     cargo run --example arena_map
//!
//! The "map" setting in the panel switches back to the open #10 arena to compare.
//! See `docs/prototypes/arena-map.md` for what to try.
//!
//! Play: WASD / left stick move, mouse / right stick aim, Space / A jump, E / X grab or place,
//! RMB / RB throw, LMB / RT shoot, F / B kick (hold beside your pad to open it).
//! Rules: 1 what stacks build, 2 recipe shape, 3 pad readout, 4 super crates, 5 build time,
//! 6 vulnerability. Each restarts the match. Enter picks the rules you're playing.
//! C changes your class (matters when stacks build by class).
//! T drops a crate at your feet, Y a super crate (for trying recipes).
//! Tune: Up/Down pick, Left/Right change (Shift = x5).
//! R / Start resets, P prints settings, verdicts and results, H hides the rules and panel.
//!
//! Smoke tests: `CRATE_EXIT_AFTER=<s>`, `CRATE_SCREENSHOT=<file.png>`, `CRATE_SEAT=bot`,
//! `CRATE_RULES=<7 digits>` (the option index of each rule, e.g. `CRATE_RULES=0011211`),
//! `CRATE_ARENA=1` (whole-arena camera), `CRATE_SPEED=<1-4>` (time scale), `CRATE_MAP=open`.

use std::collections::VecDeque;
use std::f32::consts::FRAC_PI_2;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::ui::IsDefaultUiCamera;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

const G: f32 = 20.0;
const CRATE: f32 = 0.8;
const HALF: f32 = CRATE / 2.0;
const HEAD: f32 = 2.2;
const CHEST: f32 = 1.2;
const ACTOR_R: f32 = 0.45;
const YOU: usize = 0;
/// The pad is 2 x 2 crate cells (Team Buddies), at most 2 crates high.
const PAD_HALF: f32 = CRATE;
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

/// Same floor space per player at every team size: the 3v3 shape, scaled. The lanes map
/// is a fixed size.
fn set_arena(team_size: usize, space_per_player: f32) {
    let base_area = BASE_ARENA.x * BASE_ARENA.y * 4.0;
    let k = (2.0 * team_size as f32 * space_per_player / base_area).sqrt().max(0.5);
    let a = if map_on() { MAP_HALF } else { BASE_ARENA * k };
    ARENA_BITS.store(a.x.to_bits() as u64 | (a.y.to_bits() as u64) << 32, Ordering::Relaxed);
}

/// How far the arena is scaled from the 3v3 size.
fn arena_scale() -> f32 {
    arena().x / BASE_ARENA.x
}

/// Team pad centres, about 3.6 m in from each end. A pad's centre sits between crate
/// grid cells, so its 4 cells are on the grid.
fn pads() -> [Vec2; 2] {
    let x = ((arena().x - 3.6) / CRATE).round() * CRATE + HALF;
    [Vec2::new(-x, HALF), Vec2::new(x, HALF)]
}

/// The centres of a pad's 4 cells, indexed x + 2z.
fn pad_cells(team: usize) -> [Vec2; 4] {
    let p = pads()[team];
    [Vec2::new(-HALF, -HALF), Vec2::new(HALF, -HALF), Vec2::new(-HALF, HALF), Vec2::new(HALF, HALF)].map(|o| p + o)
}

// ---------------------------------------------------------------------------------------
// The lanes + jungle map (#20): bases at each end, three lanes, four walled jungle quadrants
// with a river between them. Walls are tall: players, crates and shots don't pass.
// ---------------------------------------------------------------------------------------

/// Whether the lanes map is on (else the open #10 arena). A static so the free physics
/// functions can read it.
static MAP_ON: AtomicBool = AtomicBool::new(false);
/// Lanes map half-extents: 90 x 50 m.
const MAP_HALF: Vec2 = Vec2::new(45.0, 25.0);
const BLOCK_H: f32 = 2.6;

fn map_on() -> bool {
    MAP_ON.load(Ordering::Relaxed)
}

/// A jungle wall, as a box on the floor.
#[derive(Clone, Copy)]
struct Block {
    min: Vec2,
    max: Vec2,
}

impl Block {
    fn hits(&self, p: Vec2, r: f32) -> bool {
        p.x > self.min.x - r && p.x < self.max.x + r && p.y > self.min.y - r && p.y < self.max.y + r
    }
    /// Pushes a circle of radius `r` at `p` out of the block. Returns the push normal.
    fn push_out(&self, p: &mut Vec2, r: f32) -> Option<Vec2> {
        if !self.hits(*p, r) {
            return None;
        }
        let pens = [
            (p.x - (self.min.x - r), Vec2::NEG_X),
            ((self.max.x + r) - p.x, Vec2::X),
            (p.y - (self.min.y - r), Vec2::NEG_Y),
            ((self.max.y + r) - p.y, Vec2::Y),
        ];
        let (pen, n) = pens.into_iter().min_by(|a, b| a.0.total_cmp(&b.0)).unwrap();
        *p += n * pen;
        Some(n)
    }
}

/// One quadrant's walls in +x, +z, mirrored into all four.
fn build_blocks() -> Vec<Block> {
    // (x0, x1, z0, z1) in the +x +z quadrant.
    let walls = [
        // Facing mid lane, with an entrance at x 14-18.
        (4.0, 14.0, 4.0, 5.0),
        (18.0, 30.0, 4.0, 5.0),
        // Facing the side lane, with an entrance at x 20-24.
        (4.0, 20.0, 17.0, 18.0),
        (24.0, 30.0, 17.0, 18.0),
        // Facing the base, with an entrance at z 10-12.
        (30.0, 31.0, 4.0, 10.0),
        (30.0, 31.0, 12.0, 18.0),
        // Pillars inside.
        (8.0, 11.0, 8.0, 10.0),
        (22.0, 25.0, 12.5, 14.0),
        (24.0, 26.0, 7.0, 9.0),
    ];
    let mut out = Vec::new();
    for sx in [-1.0f32, 1.0] {
        for sz in [-1.0f32, 1.0] {
            for (x0, x1, z0, z1) in walls {
                let (a, b) = (Vec2::new(x0 * sx, z0 * sz), Vec2::new(x1 * sx, z1 * sz));
                out.push(Block { min: a.min(b), max: a.max(b) });
            }
        }
    }
    out
}

fn blocks() -> &'static [Block] {
    static B: OnceLock<Vec<Block>> = OnceLock::new();
    if !map_on() {
        return &[];
    }
    B.get_or_init(build_blocks)
}

fn blocked(p: Vec2, r: f32) -> bool {
    blocks().iter().any(|b| b.hits(p, r))
}

/// Whether a circle of radius `r` can go straight from `a` to `b`.
fn clear(a: Vec2, b: Vec2, r: f32) -> bool {
    if !map_on() {
        return true;
    }
    let n = (a.distance(b) / 0.5).ceil().max(1.0) as usize;
    (0..=n).all(|k| !blocked(a.lerp(b, k as f32 / n as f32), r))
}

/// Navigation grid for bots: a node every 2.5 m off the walls, linked to its 8 neighbours
/// when the way is clear.
struct Nav {
    nodes: Vec<Vec2>,
    links: Vec<Vec<(usize, f32)>>,
}

fn nav() -> &'static Nav {
    static N: OnceLock<Nav> = OnceLock::new();
    N.get_or_init(|| {
        let step = 2.5;
        let mut nodes = Vec::new();
        let (nx, nz) = ((MAP_HALF.x * 2.0 / step) as i32, (MAP_HALF.y * 2.0 / step) as i32);
        let mut index = std::collections::HashMap::new();
        for i in 0..=nx {
            for j in 0..=nz {
                let p = Vec2::new(-MAP_HALF.x + 1.0 + i as f32 * step * 0.98, -MAP_HALF.y + 1.0 + j as f32 * step * 0.96);
                if !build_blocks().iter().any(|b| b.hits(p, 0.8)) {
                    index.insert((i, j), nodes.len());
                    nodes.push(p);
                }
            }
        }
        let walls = build_blocks();
        let free = |a: Vec2, b: Vec2| (0..=8).all(|k| !walls.iter().any(|w| w.hits(a.lerp(b, k as f32 / 8.0), 0.6)));
        let mut links = vec![Vec::new(); nodes.len()];
        for (&(i, j), &a) in &index {
            for (di, dj) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                if let Some(&b) = index.get(&(i + di, j + dj))
                    && free(nodes[a], nodes[b])
                {
                    let d = nodes[a].distance(nodes[b]);
                    links[a].push((b, d));
                    links[b].push((a, d));
                }
            }
        }
        Nav { nodes, links }
    })
}

/// The next point to walk to on the way from `from` to `goal`, around the walls.
fn route(from: Vec2, goal: Vec2) -> Vec2 {
    if clear(from, goal, ACTOR_R) {
        return goal;
    }
    let nav = nav();
    let nearest = |p: Vec2| {
        (0..nav.nodes.len())
            .filter(|&k| nav.nodes[k].distance(p) < 6.0 && clear(p, nav.nodes[k], ACTOR_R * 0.8))
            .min_by(|&a, &b| nav.nodes[a].distance(p).total_cmp(&nav.nodes[b].distance(p)))
    };
    let (Some(start), Some(end)) = (nearest(from), nearest(goal)) else { return goal };
    // A* over the grid.
    let n = nav.nodes.len();
    let mut cost = vec![f32::INFINITY; n];
    let mut prev = vec![usize::MAX; n];
    let mut open = std::collections::BinaryHeap::new();
    cost[start] = 0.0;
    open.push((std::cmp::Reverse((nav.nodes[start].distance(goal) * 100.0) as u32), start));
    while let Some((_, k)) = open.pop() {
        if k == end {
            break;
        }
        for &(m, d) in &nav.links[k] {
            let c = cost[k] + d;
            if c < cost[m] {
                cost[m] = c;
                prev[m] = k;
                open.push((std::cmp::Reverse(((c + nav.nodes[m].distance(goal)) * 100.0) as u32), m));
            }
        }
    }
    if cost[end].is_infinite() {
        return goal;
    }
    let mut path = vec![end];
    while let Some(&k) = path.last() {
        if k == start {
            break;
        }
        path.push(prev[k]);
    }
    // The furthest point along the path that's in a straight line from here.
    path.iter().map(|&k| nav.nodes[k]).find(|&p| clear(from, p, ACTOR_R)).unwrap_or(nav.nodes[start])
}

/// A random spot on the floor, off the walls.
fn open_spot(mut rand: impl FnMut() -> f32, span: Vec2) -> Vec2 {
    for _ in 0..30 {
        let p = Vec2::new((rand() * 2.0 - 1.0) * span.x, (rand() * 2.0 - 1.0) * span.y);
        if !blocked(p, 1.0) {
            return p;
        }
    }
    Vec2::ZERO
}

/// A recipe: what a pad's crates make when kicked open. The two player recipes are fixed
/// by ADR-0005; the others are what #17 is choosing.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Slot {
    /// 1 crate (count only: 1).
    Small,
    /// 2 side by side (count only: 3).
    Pair,
    /// 2 stacked: 1 new player (count only: 2).
    Player1,
    /// 2 wide x 2 high: 2 new players (count only: 4).
    Player2,
    /// 4 flat (count only: 5-7).
    Flat4,
    /// All 8 (count only: 8).
    All8,
}

/// What a pad's crates are right now.
#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Empty,
    Recipe(Slot),
    /// Lined up, but no recipe has this shape: add crates.
    Odd,
    /// A crate on the pad isn't sitting in a cell (or is still moving).
    Unaligned,
    /// Kicked open and building (build timer).
    Busy,
}

impl Shape {
    fn players(self) -> usize {
        match self {
            Shape::Recipe(Slot::Player1) => 1,
            Shape::Recipe(Slot::Player2) => 2,
            _ => 0,
        }
    }
}

impl Slot {
    fn label(self, count_only: bool) -> &'static str {
        match (self, count_only) {
            (Slot::Small, false) => "1 crate",
            (Slot::Pair, false) => "2 side by side",
            (Slot::Player1, false) => "2 stacked",
            (Slot::Player2, false) => "2 wide x 2 high",
            (Slot::Flat4, false) => "4 flat",
            (Slot::All8, false) => "all 8",
            (Slot::Small, true) => "1 crate",
            (Slot::Player1, true) => "2 crates",
            (Slot::Pair, true) => "3 crates",
            (Slot::Player2, true) => "4 crates",
            (Slot::Flat4, true) => "5-7 crates",
            (Slot::All8, true) => "8 crates",
        }
    }
}

/// What kicking a recipe open gives.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Make {
    Players(usize),
    /// Full health for whoever kicked it.
    Health,
    /// Full health for the whole team.
    TeamHeal,
    Gun(GunKind),
    /// A right-click item (class loadout): rockets, a turret to drop, or a heal pulse.
    Alt(AltKind),
    Tank,
    Turret,
    Wall,
    /// A turret and a wall that last longer.
    Fortress,
}

impl Make {
    fn name(self) -> &'static str {
        match self {
            Make::Players(1) => "1 new player",
            Make::Players(_) => "2 new players",
            Make::Health => "full health",
            Make::TeamHeal => "heal your team",
            Make::Gun(GunKind::Shotgun) => "shotgun",
            Make::Gun(GunKind::Rapid) => "rapid gun",
            Make::Gun(GunKind::Rocket) => "rocket launcher",
            Make::Gun(GunKind::Heal) => "heal gun (LMB)",
            Make::Alt(AltKind::Rockets) => "rockets (RMB)",
            Make::Alt(AltKind::Turret) => "turret drop (RMB)",
            Make::Alt(AltKind::Pulse) => "heal pulse (RMB)",
            Make::Tank => "tank (vehicle)",
            Make::Turret => "turret",
            Make::Wall => "shield wall",
            Make::Fortress => "fortress (turret + wall)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum GunKind {
    Shotgun,
    Rapid,
    Rocket,
    /// Medic, left click: heals the teammate you aim at, else a weak shot.
    Heal,
}

/// A right-click item from the class loadout (2 x 2 flat).
#[derive(Clone, Copy, PartialEq, Debug)]
enum AltKind {
    /// Soldier: rockets.
    Rockets,
    /// Engineer: drops a turret where you aim.
    Turret,
    /// Medic: heals you and every teammate close by.
    Pulse,
}

#[derive(Clone, Copy)]
struct Alt {
    kind: AltKind,
    charges: u32,
    gold: bool,
}

fn new_alt(kind: AltKind, gold: bool) -> Alt {
    let n = match kind {
        AltKind::Rockets => 4,
        AltKind::Turret => 2,
        AltKind::Pulse => 3,
    };
    Alt { kind, charges: if gold { n * 2 } else { n }, gold }
}

/// A player's class. With "what stacks build: by kicker's class", the class of whoever
/// kicks the pad open picks the recipe book.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Class {
    Soldier,
    Engineer,
    Medic,
}

const CLASSES: [Class; 3] = [Class::Soldier, Class::Engineer, Class::Medic];

impl Class {
    fn name(self) -> &'static str {
        match self {
            Class::Soldier => "soldier",
            Class::Engineer => "engineer",
            Class::Medic => "medic",
        }
    }
    fn book(self, loadout: bool) -> usize {
        match (self, loadout) {
            (Class::Soldier, false) => 1,
            (Class::Engineer, false) => 4,
            (Class::Medic, false) => 5,
            (Class::Soldier, true) => 6,
            (Class::Engineer, true) => 7,
            (Class::Medic, true) => 8,
        }
    }
    fn next(self) -> Class {
        CLASSES[(self as usize + 1) % 3]
    }
}

/// The "what stacks build" option that hands the choice to the kicker's class.
const BY_CLASS: usize = 3;
/// Like BY_CLASS, but each class's book is a loadout: 2 side by side makes its left-click
/// item, 2 x 2 flat its right-click item.
const BY_LOADOUT: usize = 4;

/// The recipe books: what 1 crate, 2 side by side, 4 flat and all 8 make.
const BOOKS: [(&str, [Make; 4]); 9] = [
    ("mix", [Make::Health, Make::Gun(GunKind::Rapid), Make::Turret, Make::Tank]),
    ("weapons", [Make::Gun(GunKind::Shotgun), Make::Gun(GunKind::Rapid), Make::Gun(GunKind::Rocket), Make::Tank]),
    ("base upgrades", [Make::TeamHeal, Make::Wall, Make::Turret, Make::Fortress]),
    // Only reached by a branching super crate in the mix book.
    ("mix (branched)", [Make::Gun(GunKind::Shotgun), Make::Wall, Make::Gun(GunKind::Rocket), Make::Fortress]),
    // Class books (soldier uses "weapons").
    ("engineer", [Make::Wall, Make::Turret, Make::Fortress, Make::Tank]),
    ("medic", [Make::Health, Make::TeamHeal, Make::Wall, Make::Fortress]),
    // Class loadouts: 1 crate full health, 2 side by side LMB, 2 x 2 flat RMB, all 8 tank.
    ("soldier loadout", [Make::Health, Make::Gun(GunKind::Rapid), Make::Alt(AltKind::Rockets), Make::Tank]),
    ("engineer loadout", [Make::Health, Make::Gun(GunKind::Shotgun), Make::Alt(AltKind::Turret), Make::Tank]),
    ("medic loadout", [Make::Health, Make::Gun(GunKind::Heal), Make::Alt(AltKind::Pulse), Make::Tank]),
];

/// The book a branching super crate switches to.
fn branch_of(book: usize) -> usize {
    match book {
        0 => 3,
        1 => 2,
        2 => 1,
        _ => 0,
    }
}

fn make_of(book: usize, slot: Slot) -> Make {
    let b = BOOKS[book].1;
    match slot {
        Slot::Player1 => Make::Players(1),
        Slot::Player2 => Make::Players(2),
        Slot::Small => b[0],
        Slot::Pair => b[1],
        Slot::Flat4 => b[2],
        Slot::All8 => b[3],
    }
}

fn adjacent(a: usize, b: usize) -> bool {
    ((a % 2) == (b % 2)) != ((a / 2) == (b / 2))
}

/// The recipe a set of cell heights makes: by shape (Team Buddies), or by count only.
fn shape_of(h: [u8; 4], count_only: bool) -> Shape {
    let total: u8 = h.iter().sum();
    if total == 0 {
        return Shape::Empty;
    }
    if count_only {
        return Shape::Recipe(match total {
            1 => Slot::Small,
            2 => Slot::Player1,
            3 => Slot::Pair,
            4 => Slot::Player2,
            5..=7 => Slot::Flat4,
            _ => Slot::All8,
        });
    }
    let cells = |n: u8| (0..4).filter(|&i| h[i] == n).collect::<Vec<_>>();
    match total {
        1 => Shape::Recipe(Slot::Small),
        2 if cells(2).len() == 1 => Shape::Recipe(Slot::Player1),
        2 if cells(1).len() == 2 && adjacent(cells(1)[0], cells(1)[1]) => Shape::Recipe(Slot::Pair),
        4 if cells(1).len() == 4 => Shape::Recipe(Slot::Flat4),
        4 if cells(2).len() == 2 && adjacent(cells(2)[0], cells(2)[1]) => Shape::Recipe(Slot::Player2),
        8 => Shape::Recipe(Slot::All8),
        _ => Shape::Odd,
    }
}

/// Hold-to-choose kick (rule 7): the first recipe is ready after this long, and each
/// further second step up to the next bigger one.
const HOLD_FIRST: f32 = 0.75;
const HOLD_STEP: f32 = 0.6;

/// The recipes that fit inside a pad, smallest first, each with how many crates it uses
/// from the top of each cell.
fn sub_recipes(h: [u8; 4], count_only: bool) -> Vec<(Slot, [u8; 4])> {
    let total: u8 = h.iter().sum();
    let mut out = Vec::new();
    if count_only {
        let five = total.min(7);
        for (slot, n) in [(Slot::Small, 1), (Slot::Player1, 2), (Slot::Pair, 3), (Slot::Player2, 4), (Slot::Flat4, five.max(5)), (Slot::All8, 8)] {
            if n > total {
                continue;
            }
            // From the top: the tallest cells first.
            let mut take = [0u8; 4];
            for _ in 0..n {
                let k = (0..4).filter(|&k| h[k] > take[k]).max_by_key(|&k| h[k] - take[k]).unwrap();
                take[k] += 1;
            }
            out.push((slot, take));
        }
        return out;
    }
    let one = |k: usize, n: u8| {
        let mut t = [0u8; 4];
        t[k] = n;
        t
    };
    const PAIRS: [(usize, usize); 4] = [(0, 1), (0, 2), (1, 3), (2, 3)];
    if let Some(k) = (0..4).filter(|&k| h[k] >= 1).min_by_key(|&k| h[k]) {
        out.push((Slot::Small, one(k, 1)));
    }
    if let Some(k) = (0..4).find(|&k| h[k] == 2) {
        out.push((Slot::Player1, one(k, 2)));
    }
    if let Some(&(a, b)) = PAIRS.iter().filter(|(a, b)| h[*a] >= 1 && h[*b] >= 1).min_by_key(|(a, b)| h[*a] + h[*b]) {
        let mut t = one(a, 1);
        t[b] = 1;
        out.push((Slot::Pair, t));
    }
    if let Some(&(a, b)) = PAIRS.iter().find(|(a, b)| h[*a] == 2 && h[*b] == 2) {
        let mut t = one(a, 2);
        t[b] = 2;
        out.push((Slot::Player2, t));
    }
    if h.iter().all(|&n| n >= 1) {
        out.push((Slot::Flat4, [1; 4]));
    }
    if h.iter().all(|&n| n == 2) {
        out.push((Slot::All8, [2; 4]));
    }
    out
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
    // The rules #17 compares (keys 1-7). Keep these first, in this order.
    Book,
    RecipeShape,
    Readout,
    Super,
    BuildTime,
    Vulnerable,
    Kick,
    // The rest.
    Map,
    TeamSize,
    AllyStacks,
    OnlineDelay,
    Seat,
    MatchLength,
    TimeScale,
    Camera,
}

/// How many of the params are rules (keys 1-7).
const RULES: usize = 7;

/// Build time options (seconds per crate) for key 5.
const BUILD_TIMES: [f32; 3] = [0.0, 1.0, 2.0];

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
        choice("1 what stacks build", 0, &["mix", "weapons", "base upgrades", "by kicker's class", "class loadout (LMB/RMB)"]),
        choice("2 recipe shape", 0, &["by shape (TB)", "by count only"]),
        choice("3 pad readout", 1, &["secret (learn the shapes)", "pad shows what it makes"]),
        choice("4 super crates", 0, &["off", "upgrade the recipe", "branch the recipe"]),
        num("5 build time", 0.0, 0.0, 3.0, 0.5, "s per crate"),
        choice("6 vulnerability", 0, &["steal only", "a hit splits the stack", "a hit spills the pad", "locks once placed"]),
        choice("7 kick", 1, &["uses the whole pad", "hold longer to use more crates"]),
        choice("map", 0, &["lanes + jungle (90 x 50 m)", "open arena (#10)"]),
        // 5v5, as in League of Legends (the #10 verdict was 3v3 on the small arena).
        num("team size", 5.0, 1.0, 8.0, 1.0, "v"),
        // Off: only you build and open blue's pad.
        choice("blue bots build", 1, &["off (only you)", "on"]),
        num("online delay", 0.0, 0.0, 250.0, 25.0, "ms"),
        choice("your seat", 0, &["you play", "bot (watch)"]),
        num("match length", 300.0, 30.0, 600.0, 30.0, "s"),
        num("time scale", 1.0, 1.0, 4.0, 1.0, "x"),
        choice("camera", 0, &["follow you", "whole arena"]),
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

/// The rules being played, on one line.
fn rules_desc(tune: &Tune) -> String {
    tune.vals[..RULES]
        .iter()
        .map(|p| format!("{}: {}", &p.name[2..], fmt_value(&p.value)))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn init_from_env(mut tune: ResMut<Tune>, mut sim: ResMut<Sim>) {
    if let Ok(rules) = std::env::var("CRATE_RULES") {
        for (k, ch) in rules.chars().take(RULES).enumerate() {
            if let Some(d) = ch.to_digit(10) {
                let x = if k == P::BuildTime as usize { BUILD_TIMES[(d as usize).min(2)] } else { d as f32 };
                set(&mut tune.vals, [P::Book, P::RecipeShape, P::Readout, P::Super, P::BuildTime, P::Vulnerable, P::Kick][k], x);
            }
        }
    }
    if let Some(x) = std::env::var("CRATE_SPEED").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::TimeScale, x);
    }
    if std::env::var("CRATE_MAP").as_deref() == Ok("open") {
        set(&mut tune.vals, P::Map, 1.0);
    }
    if std::env::var("CRATE_ARENA").is_ok() {
        set(&mut tune.vals, P::Camera, 1.0);
    }
    if std::env::var("CRATE_SEAT").as_deref() == Ok("bot") {
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
    /// Rare: upgrades or branches the recipe it joins.
    Super,
    Loot,
}

impl Kind {
    /// Crates that count on a pad.
    fn builds(self) -> bool {
        self != Kind::Loot
    }
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
    /// Locked into the pad: it can't be taken, knocked or moved, only kicked open.
    locked: bool,
    ent: Option<Entity>,
}

impl Crate {
    /// Free and not locked: can be grabbed, knocked and blown about.
    fn loose(&self) -> bool {
        self.state == CrateState::Free && !self.locked
    }
}

#[derive(Clone, Copy)]
struct Gun {
    kind: GunKind,
    ammo: u32,
    /// From a super crate: more damage and ammo.
    gold: bool,
}

#[derive(Clone, Copy)]
struct Tank {
    hp: f32,
    life: f32,
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
    gun: Option<Gun>,
    /// Right-click item from the class loadout.
    alt: Option<Alt>,
    alt_cd: f32,
    tank: Option<Tank>,
    class: Class,
    /// Absorbs damage before hp (from shield camps).
    shield: f32,
    /// Bots: the goal they're routing to, the next waypoint, and how old it is.
    nav: Option<(Vec2, Vec2, f32)>,
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
            gun: None,
            alt: None,
            alt_cd: 0.0,
            tank: None,
            // Bots by slot: 1st soldier, 2nd engineer, 3rd medic.
            class: CLASSES[slot % 3],
            shield: 0.0,
            nav: None,
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

/// Shoots the nearest enemy for a while, then expires.
struct Turret {
    team: usize,
    pos: Vec3,
    facing: Vec3,
    cd: f32,
    rate: f32,
    life: f32,
    ent: Option<Entity>,
}

/// A shield wall in front of a pad: blocks enemy shots until broken or expired.
struct Wall {
    team: usize,
    pos: Vec3,
    half_z: f32,
    hp: f32,
    life: f32,
    ent: Option<Entity>,
}

const WALL_H: f32 = 2.0;

/// A kicked-open pad that's still building (build timer). Its crates are merged into one
/// locked lump: enemy shots wear it down, and if it breaks the crates spill out.
struct Build {
    by: usize,
    make: Make,
    gold: bool,
    ids: Vec<u32>,
    left: f32,
    total: f32,
    hp: f32,
    max_hp: f32,
}

/// Build hp per crate in it.
const BUILD_HP: f32 = 30.0;

struct Shot {
    pos: Vec3,
    vel: Vec3,
    /// None for turret shots.
    owner: Option<usize>,
    team: usize,
    dmg: f32,
    /// Rockets: blast radius (0 for bullets).
    splash: f32,
    life: f32,
    ent: Option<Entity>,
}

/// What a PvE camp drops when cleared.
#[derive(Clone, Copy, PartialEq)]
enum Drop {
    Heal,
    Shield,
    /// River objectives: a super crate (3 crates when super crates are off).
    Super,
}

/// A neutral PvE monster guarding a camp. It fights whoever comes close or shoots it, chases
/// within its leash, then goes home and heals.
struct Monster {
    home: Vec3,
    pos: Vec3,
    hp: f32,
    max: f32,
    dmg: f32,
    drop: Drop,
    cd: f32,
    /// Seconds until it respawns (0 while alive).
    respawn: f32,
    target: Option<usize>,
    ent: Option<Entity>,
}

impl Monster {
    fn alive(&self) -> bool {
        self.respawn <= 0.0
    }
}

const LEASH: f32 = 9.0;

/// A heal or shield dropped by a camp, picked up by walking over it.
struct Pickup {
    pos: Vec3,
    drop: Drop,
    life: f32,
}

/// What the pad holds right now.
struct PadState {
    h: [u8; 4],
    shape: Shape,
    ids: Vec<u32>,
    supers: usize,
}

#[derive(Resource)]
struct Sim {
    /// Rule 7: holding the kick longer steps up through the recipes inside the pad.
    hold_kick: bool,
    /// Who's holding the kick on each pad (hold-to-choose fires on release).
    kick_by: [Option<usize>; 2],
    actors: Vec<Actor>,
    crates: Vec<Crate>,
    shots: Vec<Shot>,
    turrets: Vec<Turret>,
    walls: Vec<Wall>,
    monsters: Vec<Monster>,
    pickups: Vec<Pickup>,
    /// The lanes map is on.
    map: bool,
    builds: [Option<Build>; 2],
    pending: Option<Pending>,
    next_id: u32,
    rng: u32,
    t: f32,
    loot_respawn: f32,
    super_respawn: f32,
    msgs: Vec<(String, f32)>,
    despawn: Vec<Entity>,
    /// Rocket blasts to draw: where, radius, time left.
    booms: Vec<(Vec3, f32, f32)>,
    /// Players per team.
    team_size: usize,
    /// The rules, copied from `Tune` each tick.
    book: usize,
    count_only: bool,
    shown: bool,
    super_mode: usize,
    build_time: f32,
    vuln: usize,
    /// The class you play (C cycles it); survives resets.
    your_class: Class,
    /// Blue's bots build pad shapes and kick them open too.
    ally_stacks: bool,
    /// You're in your seat (not watching a bot play it).
    you_play: bool,
    /// Kick-open progress on each pad, and who kicked it this tick.
    pad_kick: [f32; 2],
    kicking: [Option<usize>; 2],
    /// How long each pad has held crates that make no recipe.
    odd_t: [f32; 2],
    /// How long each pad's recipe has stayed the same, and what it was.
    pad_t: [f32; 2],
    last_shape: [Shape; 2],
    /// Crates spent on pad recipes: they still count for the score.
    banked: [usize; 2],
    /// What each team made this match.
    made: [Vec<String>; 2],
    match_left: f32,
    /// Counts down the end-of-match result screen.
    over: f32,
    /// Match results, printed with P. Survives resets.
    log: Vec<String>,
    // Stats.
    fired: u32,
    hits: u32,
    kills: [u32; 2],
    /// Crates knocked or spilled off each team's pad by enemy hits.
    lost: [u32; 2],
    /// Builds broken on each team's pad.
    broken: [u32; 2],
    /// Players each team lost to PvE monsters.
    pve_deaths: [u32; 2],
}

/// Vulnerability options (key 6).
const STEAL: usize = 0;
const SPLIT: usize = 1;
const SPILL: usize = 2;
const LOCK: usize = 3;

impl Sim {
    fn new(tune: &Tune) -> Self {
        let mut s = Sim {
            actors: Vec::new(),
            crates: Vec::new(),
            shots: Vec::new(),
            turrets: Vec::new(),
            walls: Vec::new(),
            monsters: Vec::new(),
            pickups: Vec::new(),
            map: true,
            builds: [None, None],
            pending: None,
            next_id: 1,
            rng: 0x2545_f491,
            t: 0.0,
            loot_respawn: 0.0,
            super_respawn: 0.0,
            msgs: Vec::new(),
            despawn: Vec::new(),
            booms: Vec::new(),
            team_size: 0,
            book: 0,
            count_only: false,
            shown: true,
            super_mode: 0,
            build_time: 0.0,
            vuln: STEAL,
            your_class: Class::Soldier,
            ally_stacks: false,
            you_play: true,
            pad_kick: [0.0; 2],
            hold_kick: false,
            kick_by: [None; 2],
            kicking: [None; 2],
            odd_t: [0.0; 2],
            pad_t: [0.0; 2],
            last_shape: [Shape::Empty; 2],
            banked: [0; 2],
            made: [Vec::new(), Vec::new()],
            match_left: 0.0,
            over: 0.0,
            log: Vec::new(),
            fired: 0,
            hits: 0,
            kills: [0; 2],
            lost: [0; 2],
            broken: [0; 2],
            pve_deaths: [0; 2],
        };
        s.reset(tune);
        s
    }

    fn rules(&mut self, tune: &Tune) {
        self.book = tune.c(P::Book);
        self.count_only = tune.c(P::RecipeShape) == 1;
        self.shown = tune.c(P::Readout) == 1;
        self.super_mode = tune.c(P::Super);
        self.build_time = tune.f(P::BuildTime);
        self.vuln = tune.c(P::Vulnerable);
        self.hold_kick = tune.c(P::Kick) == 1;
    }

    fn reset(&mut self, tune: &Tune) {
        for c in self.crates.drain(..) {
            self.despawn.extend(c.ent);
        }
        for t in self.turrets.drain(..) {
            self.despawn.extend(t.ent);
        }
        for w in self.walls.drain(..) {
            self.despawn.extend(w.ent);
        }
        for m in self.monsters.drain(..) {
            self.despawn.extend(m.ent);
        }
        self.pickups.clear();
        self.map = tune.c(P::Map) == 0;
        MAP_ON.store(self.map, Ordering::Relaxed);
        for sh in self.shots.drain(..) {
            self.despawn.extend(sh.ent);
        }
        for a in self.actors.drain(..) {
            self.despawn.extend(a.ent);
        }
        self.rules(tune);
        self.builds = [None, None];
        self.pending = None;
        self.team_size = tune.c(P::TeamSize);
        set_arena(self.team_size, 130.0);
        self.banked = [0; 2];
        self.made = [Vec::new(), Vec::new()];
        self.pad_kick = [0.0; 2];
        self.odd_t = [0.0; 2];
        self.pad_t = [0.0; 2];
        self.super_respawn = 15.0;
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
            // Each team starts with one player; the rest join from pad recipes.
            if a.slot > 0 {
                a.dead = f32::INFINITY;
            }
        }
        self.actors[YOU].class = self.your_class;
        if self.map {
            self.reset_map();
        } else {
            self.reset_open();
        }
        self.match_left = tune.f(P::MatchLength);
        self.over = 0.0;
        self.fired = 0;
        self.hits = 0;
        self.kills = [0; 2];
        self.lost = [0; 2];
        self.broken = [0; 2];
        self.pve_deaths = [0; 2];
    }

    /// Lanes map: crates along the lanes, in the river and by the camps; a camp in each
    /// jungle quadrant (heal on the north side, shield on the south) and a river objective
    /// north and south.
    fn reset_map(&mut self) {
        let mut spots = Vec::new();
        for x in [-24.0, -12.0, 0.0, 12.0, 24.0] {
            spots.push(Vec2::new(x, 21.6));
            spots.push(Vec2::new(x, -21.6));
        }
        for x in [-22.0, -11.0, 11.0, 22.0] {
            spots.push(Vec2::new(x, 0.0));
        }
        for sx in [-1.0, 1.0] {
            for sz in [-1.0, 1.0] {
                spots.push(Vec2::new(14.0 * sx, 13.0 * sz));
                spots.push(Vec2::new(19.0 * sx, 8.0 * sz));
            }
        }
        spots.extend([Vec2::new(0.0, 6.0), Vec2::new(0.0, -6.0), Vec2::new(-1.6, 1.6), Vec2::new(1.6, -1.6)]);
        for p in spots {
            let p = (p / CRATE).round() * CRATE;
            self.spawn_crate(Kind::Normal, p.extend(HALF).xzy());
        }
        self.spawn_crate(Kind::Loot, Vec3::new(0.0, HALF, 12.0));
        self.spawn_crate(Kind::Loot, Vec3::new(0.0, HALF, -12.0));
        for sx in [-1.0f32, 1.0] {
            for sz in [-1.0f32, 1.0] {
                let drop = if sz > 0.0 { Drop::Heal } else { Drop::Shield };
                self.add_monster(Vec3::new(17.0 * sx, 0.0, 11.0 * sz), 200.0, 8.0, drop);
            }
        }
        self.add_monster(Vec3::new(0.0, 0.0, 15.0), 320.0, 12.0, Drop::Super);
        self.add_monster(Vec3::new(0.0, 0.0, -15.0), 320.0, 12.0, Drop::Super);
    }

    fn add_monster(&mut self, home: Vec3, hp: f32, dmg: f32, drop: Drop) {
        self.monsters.push(Monster { home, pos: home, hp, max: hp, dmg, drop, cd: 0.0, respawn: 0.0, target: None, ent: None });
    }

    /// Open arena (#10): starting crates in a grid across the middle.
    fn reset_open(&mut self) {
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
            locked: false,
            ent: None,
        });
        id
    }

    fn crate_mut(&mut self, id: u32) -> Option<&mut Crate> {
        self.crates.iter_mut().find(|c| c.id == id)
    }

    fn say(&mut self, s: impl Into<String>) {
        self.msgs.push((s.into(), 3.5));
        if self.msgs.len() > 6 {
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

    /// The heights of a pad's 4 cells, the recipe they make, and the crates in it.
    fn pad_state(&self, team: usize) -> PadState {
        let cells = pad_cells(team);
        let mut st = PadState { h: [0; 4], shape: Shape::Empty, ids: Vec::new(), supers: 0 };
        if self.builds[team].is_some() {
            st.shape = Shape::Busy;
            return st;
        }
        let mut odd = false;
        for c in self.crates.iter().filter(|c| c.state == CrateState::Free && c.kind.builds()) {
            if !pad_contains(pads()[team], c.pos) {
                continue;
            }
            st.ids.push(c.id);
            if c.kind == Kind::Super {
                st.supers += 1;
            }
            let cell = cells.iter().position(|q| q.distance(c.pos.xz()) < 0.1);
            let level = ((c.pos.y - HALF) / CRATE).round() as u8;
            match cell {
                Some(k) if c.vel.length() < 0.1 && (c.pos.y - HALF - level as f32 * CRATE).abs() < 0.05 && level < 2 => {
                    st.h[k] += 1;
                }
                _ => odd = true,
            }
        }
        st.shape = if odd { Shape::Unaligned } else { shape_of(st.h, self.count_only) };
        st
    }

    /// What `team`'s pad makes if `by` kicks it open now, and whether a super crate
    /// upgrades it.
    fn pad_make(&self, team: usize, by: usize) -> Option<(Make, bool)> {
        self.pad_make_as(team, self.actors[by].class)
    }

    /// What the pad makes if a player of `class` kicks it (the class only matters when
    /// stacks build by class).
    fn pad_make_as(&self, team: usize, class: Class) -> Option<(Make, bool)> {
        let st = self.pad_state(team);
        let Shape::Recipe(slot) = st.shape else { return None };
        Some(self.make_for(slot, st.supers, class))
    }

    /// What a recipe makes for a kicker of `class`, and whether a super crate upgrades it.
    fn make_for(&self, slot: Slot, supers: usize, class: Class) -> (Make, bool) {
        let by_class = self.book >= BY_CLASS;
        let loadout = self.book == BY_LOADOUT;
        let book = if by_class { class.book(loadout) } else { self.book };
        let sup = supers > 0 && self.super_mode > 0;
        let players = matches!(slot, Slot::Player1 | Slot::Player2);
        if sup && self.super_mode == 2 && !players {
            let other = if by_class { class.next().book(loadout) } else { branch_of(self.book) };
            return (make_of(other, slot), false);
        }
        // Branching doesn't touch player recipes; a super crate arms the new players instead.
        (make_of(book, slot), sup)
    }

    /// Hold-to-choose (rule 7): the recipes inside `team`'s pad that can open now, smallest
    /// first (player recipes only while the team has room).
    fn stages(&self, team: usize) -> Vec<(Slot, [u8; 4])> {
        let st = self.pad_state(team);
        if matches!(st.shape, Shape::Empty | Shape::Unaligned | Shape::Busy) {
            return Vec::new();
        }
        let room = self.players(team) < self.team_size;
        sub_recipes(st.h, self.count_only)
            .into_iter()
            .filter(|(slot, _)| room || !matches!(slot, Slot::Player1 | Slot::Player2))
            .collect()
    }

    /// The crates a stage uses: the top `take[k]` crates of each cell.
    fn stage_ids(&self, team: usize, take: [u8; 4]) -> Vec<u32> {
        let cells = pad_cells(team);
        let mut ids = Vec::new();
        for k in 0..4 {
            let mut here: Vec<&Crate> = self
                .crates
                .iter()
                .filter(|c| c.state == CrateState::Free && c.kind.builds() && c.pos.xz().distance(cells[k]) < 0.1)
                .collect();
            here.sort_by(|a, b| b.pos.y.total_cmp(&a.pos.y));
            ids.extend(here.iter().take(take[k] as usize).map(|c| c.id));
        }
        ids
    }

    /// Which stage a held kick has reached, if any.
    fn stage_at(&self, team: usize, n: usize) -> Option<usize> {
        let t = self.pad_kick[team];
        (n > 0 && t >= HOLD_FIRST).then(|| (((t - HOLD_FIRST) / HOLD_STEP) as usize).min(n - 1))
    }

    /// Whether `team` can open its pad: the shape is a recipe, and there's room for any
    /// players it brings.
    fn can_open(&self, team: usize, shape: Shape) -> bool {
        matches!(shape, Shape::Recipe(_)) && (shape.players() == 0 || self.players(team) < self.team_size)
    }

    /// Where bot `i` sets its crate down on its pad: the cell that best builds toward a
    /// recipe (players while the team has room, else a big recipe). None when the pad is full.
    fn pad_cell(&self, i: usize) -> Option<Vec3> {
        let team = self.actors[i].team;
        let st = self.pad_state(team);
        if matches!(st.shape, Shape::Unaligned | Shape::Busy) {
            return None;
        }
        let h = st.h;
        let builds = team == 1 || self.ally_stacks || i == YOU;
        let room = if builds { self.team_size - self.players(team) } else { 0 };
        let co = self.count_only;
        let score = |h: [u8; 4]| -> i32 {
            let total: u8 = h.iter().sum();
            match shape_of(h, co) {
                Shape::Recipe(Slot::Player1) if room >= 1 => 100,
                Shape::Recipe(Slot::Player2) if room >= 2 => 95,
                Shape::Recipe(Slot::All8) => 70,
                Shape::Recipe(Slot::Flat4) => 60,
                Shape::Recipe(Slot::Pair) => 20,
                Shape::Recipe(Slot::Small) => 10,
                _ if total == 3 && !co => {
                    let twos: Vec<usize> = (0..4).filter(|&k| h[k] == 2).collect();
                    let ones: Vec<usize> = (0..4).filter(|&k| h[k] == 1).collect();
                    match (twos.as_slice(), ones.as_slice()) {
                        ([t], [o]) if room >= 2 && adjacent(*t, *o) => 50,
                        ([], [_, _, _]) => 40,
                        _ => 1,
                    }
                }
                _ if total > 4 => 30 + total as i32,
                _ => 1,
            }
        };
        let cells = pad_cells(team);
        (0..4)
            .filter(|&k| h[k] < 2)
            .max_by_key(|&k| {
                let mut next = h;
                next[k] += 1;
                score(next)
            })
            .map(|k| Vec3::new(cells[k].x, HALF + h[k] as f32 * CRATE, cells[k].y))
    }

    /// Kicks `team`'s pad open. A recipe builds (instantly, or over the build timer); a pad
    /// with a crate out of line just breaks.
    fn open_pad(&mut self, team: usize, by: usize) {
        let st = self.pad_state(team);
        let side = ["BLUE", "RED"][team];
        let who = if by == YOU { "YOU".to_string() } else { self.describe(by) };
        let Some((make, gold)) = self.pad_make(team, by) else {
            // Unaligned: the crates are broken and used up.
            self.use_up(&st.ids, team);
            self.say(format!("{side} pad kicked by {who}: crates out of line, broken for nothing"));
            return;
        };
        let Shape::Recipe(slot) = st.shape else { return };
        self.open_recipe(team, by, slot, st.ids, make, gold);
    }

    /// Hold-to-choose (rule 7): opens one recipe inside the pad, using only its crates.
    fn open_stage(&mut self, team: usize, by: usize, slot: Slot, take: [u8; 4]) {
        let ids = self.stage_ids(team, take);
        let supers = self.crates.iter().filter(|c| ids.contains(&c.id) && c.kind == Kind::Super).count();
        let (make, gold) = self.make_for(slot, supers, self.actors[by].class);
        self.open_recipe(team, by, slot, ids, make, gold);
    }

    fn open_recipe(&mut self, team: usize, by: usize, slot: Slot, ids: Vec<u32>, make: Make, gold: bool) {
        let side = ["BLUE", "RED"][team];
        let who = if by == YOU { "YOU".to_string() } else { self.describe(by) };
        let label = slot.label(self.count_only);
        let class = if self.book >= BY_CLASS && !matches!(make, Make::Players(_)) { format!("{} ", self.actors[by].class.name()) } else { String::new() };
        let what = format!("{class}{}{}", if gold { "SUPER " } else { "" }, make.name());
        if self.build_time > 0.0 {
            let n = ids.len() as f32;
            for c in self.crates.iter_mut().filter(|c| ids.contains(&c.id)) {
                c.locked = true;
                c.vel = Vec3::ZERO;
            }
            let total = self.build_time * n;
            self.builds[team] = Some(Build {
                by,
                make,
                gold,
                ids,
                left: total,
                total,
                hp: BUILD_HP * n,
                max_hp: BUILD_HP * n,
            });
            let msg = format!("{side} pad kicked open by {who}: {label} -> building {what} ({total:.0} s)");
            println!("t={:.1}s {msg}", self.t);
            self.say(msg);
            return;
        }
        self.use_up(&ids, team);
        self.made[team].push(what.clone());
        let msg = format!("{side} pad kicked open by {who}: {label} -> {what}");
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
        self.apply(team, by, make, gold);
    }

    /// Removes a pad's crates once they're spent; they still count for the score.
    fn use_up(&mut self, ids: &[u32], team: usize) {
        self.crates.retain(|c| {
            let used = ids.contains(&c.id);
            if used {
                self.despawn.extend(c.ent);
            }
            !used
        });
        self.banked[team] += ids.len();
    }

    /// A build timer finished: the result comes out.
    fn finish_build(&mut self, team: usize) {
        let Some(b) = self.builds[team].take() else { return };
        self.use_up(&b.ids, team);
        let class = if self.book >= BY_CLASS && !matches!(b.make, Make::Players(_)) { format!("{} ", self.actors[b.by].class.name()) } else { String::new() };
        let what = format!("{class}{}{}", if b.gold { "SUPER " } else { "" }, b.make.name());
        self.made[team].push(what.clone());
        let msg = format!("{} pad finished building: {what}", ["BLUE", "RED"][team]);
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
        self.apply(team, b.by, b.make, b.gold);
    }

    /// A build was shot apart: its crates spill out, loose.
    fn break_build(&mut self, team: usize) {
        let Some(b) = self.builds[team].take() else { return };
        self.broken[team] += 1;
        let pad = pads()[team];
        for k in 0..b.ids.len() {
            let a = k as f32 * 2.4 + self.rand();
            if let Some(c) = self.crate_mut(b.ids[k]) {
                c.locked = false;
                let out = (c.pos.xz() - pad).normalize_or(Vec2::from_angle(a));
                c.vel = Vec3::new(out.x * 4.0, 4.0, out.y * 4.0);
            }
        }
        let msg = format!("{} build BROKEN before it finished ({} lost): crates spilled", ["BLUE", "RED"][team], b.make.name());
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
    }

    /// Gives `team` what a recipe makes. Weapons and vehicles go to whoever kicked it open
    /// (or a living teammate if they're down).
    fn apply(&mut self, team: usize, by: usize, make: Make, gold: bool) {
        let by = if self.actors[by].alive() {
            Some(by)
        } else {
            (0..self.actors.len()).find(|&j| self.actors[j].team == team && self.actors[j].alive())
        };
        let pad = pads()[team];
        let front = Vec3::new(pad.x + toward_centre(team) * 1.6, 0.0, pad.y);
        let k: f32 = if gold { 2.0 } else { 1.0 };
        match make {
            Make::Players(n) => {
                for m in 0..n {
                    let Some(slot) =
                        (0..self.actors.len()).find(|&j| self.actors[j].team == team && self.actors[j].dead.is_infinite())
                    else {
                        break;
                    };
                    let p = &mut self.actors[slot];
                    p.dead = 0.0;
                    p.hp = 100.0;
                    p.pos = front + Vec3::Z * (m as f32 * 1.2 - 0.6);
                    p.vel = Vec3::ZERO;
                    p.hist.clear();
                    if gold {
                        p.gun = Some(new_gun(GunKind::Rapid, true));
                    }
                }
                let n = self.players(team);
                self.say(format!("{} has {n}/{} players", ["BLUE", "RED"][team], self.team_size));
            }
            Make::Health => {
                if let Some(b) = by {
                    self.actors[b].hp = 100.0 * k.min(1.5);
                }
                if gold {
                    self.heal_team(team, 100.0);
                }
            }
            Make::TeamHeal => self.heal_team(team, if gold { 150.0 } else { 100.0 }),
            Make::Gun(kind) => {
                if let Some(b) = by {
                    self.actors[b].gun = Some(new_gun(kind, gold));
                }
            }
            Make::Alt(kind) => {
                if let Some(b) = by {
                    self.actors[b].alt = Some(new_alt(kind, gold));
                }
            }
            Make::Tank => {
                if let Some(b) = by {
                    let a = &mut self.actors[b];
                    a.tank = Some(Tank { hp: 300.0 * k, life: 45.0 });
                    a.gun = None;
                    if a.carrying.is_some() {
                        self.release(b, Vec3::Y, Vec3::ZERO);
                    }
                }
            }
            Make::Turret => self.add_turret(team, front, 40.0 * k, if gold { 0.4 } else { 0.8 }),
            Make::Wall => self.add_wall(team, 250.0 * k, 40.0 * k),
            Make::Fortress => {
                self.add_turret(team, front, 60.0 * k, if gold { 0.4 } else { 0.8 });
                self.add_wall(team, 400.0 * k, 60.0 * k);
            }
        }
    }

    fn heal_team(&mut self, team: usize, hp: f32) {
        for a in self.actors.iter_mut().filter(|a| a.team == team && a.alive()) {
            a.hp = a.hp.max(hp);
        }
    }

    fn add_turret(&mut self, team: usize, front: Vec3, life: f32, rate: f32) {
        let z = (self.turrets.iter().filter(|t| t.team == team).count() % 3) as f32 - 1.0;
        self.turrets.push(Turret {
            team,
            pos: front + Vec3::X * toward_centre(team) * 1.4 + Vec3::Z * z * 1.6,
            facing: Vec3::X * toward_centre(team),
            cd: 0.5,
            rate,
            life,
            ent: None,
        });
    }

    fn add_wall(&mut self, team: usize, hp: f32, life: f32) {
        for w in self.walls.iter_mut().filter(|w| w.team == team) {
            w.life = 0.0;
        }
        let pad = pads()[team];
        self.walls.push(Wall {
            team,
            pos: Vec3::new(pad.x + toward_centre(team) * 4.6, 0.0, pad.y),
            half_z: 2.0,
            hp,
            life,
            ent: None,
        });
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
            .filter(|c| c.loose() && c.kind.builds())
            .filter(|c| c.pos.y - a.pos.y < 3.0)
            .map(|c| (c.id, c.pos.xz().distance(a.pos.xz())))
            .filter(|(_, d)| *d <= reach + HALF)
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }

    fn hurt(&mut self, who: usize, amount: f32, knock: Vec3, by: Option<usize>, by_team: usize) {
        if !self.actors[who].alive() {
            return;
        }
        // A tank soaks hits until it's destroyed; the driver comes out unhurt.
        if let Some(tank) = &mut self.actors[who].tank {
            tank.hp -= amount;
            if tank.hp <= 0.0 {
                self.actors[who].tank = None;
                let what = self.describe(who);
                self.say(format!("{what}'s tank destroyed"));
            }
            return;
        }
        // A shield soaks damage first.
        let a = &mut self.actors[who];
        let soaked = a.shield.min(amount);
        a.shield -= soaked;
        let amount = amount - soaked;
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
        // A team's first player respawns; anyone who joined from the pad is out until the
        // team opens another player recipe.
        let first = a.slot == 0;
        a.dead = if first { 3.0 } else { f32::INFINITY };
        a.claim = None;
        a.gun = None;
        a.alt = None;
        a.shield = 0.0;
        // by_team 2 is a PvE monster.
        if by_team < 2 {
            self.kills[by_team] += 1;
        } else {
            self.pve_deaths[a.team] += 1;
        }
        let what = self.describe(who);
        if who == YOU {
            self.pending = None;
            self.say("you're down (3 s)");
        } else if !first {
            let how = if by == Some(YOU) { "you knocked out" } else { "out:" };
            self.say(format!("{how} {what}"));
        } else if by == Some(YOU) {
            self.say(format!("you downed {what}"));
        }
    }

    fn describe(&self, i: usize) -> String {
        let a = &self.actors[i];
        let team = ["blue", "red"][a.team];
        if i == YOU { "you".into() } else { format!("{team} player {}", a.slot + 1) }
    }

    /// Players on the team: everyone who has joined, alive or waiting to respawn.
    fn players(&self, team: usize) -> usize {
        self.actors.iter().filter(|a| a.team == team && !a.dead.is_infinite()).count()
    }

    /// Crates resting on the team's pad, plus crates spent there on recipes.
    fn score(&self, team: usize) -> usize {
        let on_pad = self
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind.builds() && pad_contains(pads()[team], c.pos))
            .filter(|c| self.builds[team].as_ref().is_none_or(|b| !b.ids.contains(&c.id)))
            .count();
        on_pad + self.banked[team]
    }

    /// An enemy hit lands on `team`'s pad at `at`: a build loses hp; otherwise the
    /// vulnerability rule decides what the crates do. Returns whether the hit was absorbed.
    fn pad_hit(&mut self, team: usize, at: Vec3, dmg: f32, radius: f32) -> bool {
        if let Some(b) = &mut self.builds[team] {
            b.hp -= dmg;
            if b.hp <= 0.0 {
                self.break_build(team);
            }
            return true;
        }
        if !matches!(self.vuln, SPLIT | SPILL) {
            return false;
        }
        let pad = pads()[team];
        let on_pad: Vec<usize> = (0..self.crates.len())
            .filter(|&k| self.crates[k].loose() && self.crates[k].kind.builds() && pad_contains(pad, self.crates[k].pos))
            .collect();
        let hit: Vec<usize> = if self.vuln == SPILL {
            if on_pad.iter().any(|&k| self.crates[k].pos.xz().distance(at.xz()) < HALF + radius) { on_pad } else { vec![] }
        } else {
            // Split: the top crate of each cell the hit reaches flies off.
            let mut tops = Vec::new();
            for cell in pad_cells(team) {
                if cell.distance(at.xz()) > HALF * 1.3 + radius {
                    continue;
                }
                if let Some(&k) = on_pad
                    .iter()
                    .filter(|&&k| self.crates[k].pos.xz().distance(cell) < 0.3)
                    .max_by(|&&x, &&y| self.crates[x].pos.y.total_cmp(&self.crates[y].pos.y))
                {
                    tops.push(k);
                }
            }
            tops
        };
        if hit.is_empty() {
            return false;
        }
        for &k in &hit {
            let a = self.rand() * 6.3;
            let c = &mut self.crates[k];
            let out = (c.pos.xz() - at.xz()).normalize_or(Vec2::from_angle(a)) + Vec2::from_angle(a) * 0.3;
            c.vel = Vec3::new(out.x * 4.5, 4.0 + c.pos.y, out.y * 4.5);
            c.settling = false;
        }
        self.lost[team] += hit.len() as u32;
        let how = if self.vuln == SPILL { "spilled" } else { "split" };
        self.say(format!("{} pad hit: {} crate{} {how} off", ["BLUE", "RED"][team], hit.len(), if hit.len() == 1 { "" } else { "s" }));
        true
    }

    /// A rocket goes off: hurts enemies, walls and builds near it, and blows loose crates
    /// about (crates on an enemy pad follow the vulnerability rule).
    fn explode(&mut self, at: Vec3, team: usize, dmg: f32, radius: f32, owner: Option<usize>) {
        for j in 0..self.actors.len() {
            let a = &self.actors[j];
            if a.team == team || !a.alive() {
                continue;
            }
            let d = a.pos.xz().distance(at.xz());
            if d < radius {
                let dir = (a.pos - at).with_y(0.0).normalize_or_zero();
                if owner == Some(YOU) {
                    self.hits += 1;
                }
                self.hurt(j, dmg * (1.0 - 0.5 * d / radius), dir * 5.0, owner, team);
            }
        }
        for w in self.walls.iter_mut().filter(|w| w.team != team) {
            if (w.pos.x - at.x).abs() < radius && (w.pos.z - at.z).abs() < w.half_z + radius {
                w.hp -= dmg;
            }
        }
        for m in 0..self.monsters.len() {
            if self.monsters[m].alive() && self.monsters[m].pos.xz().distance(at.xz()) < radius + 0.6 {
                self.hurt_monster(m, dmg, owner);
            }
        }
        let enemy = 1 - team;
        if pads()[enemy].distance(at.xz()) < PAD_HALF + radius {
            self.pad_hit(enemy, at, dmg, radius);
        }
        for c in self.crates.iter_mut().filter(|c| c.loose()) {
            if pad_contains(pads()[0], c.pos) || pad_contains(pads()[1], c.pos) {
                continue;
            }
            let d = c.pos.distance(at);
            if d < radius {
                let out = (c.pos - at).with_y(0.0).normalize_or(Vec3::X);
                c.vel += out * 5.0 * (1.0 - d / radius) + Vec3::Y * 3.0;
            }
        }
    }
}

impl Sim {
    /// A shot or blast hits monster `m`: it turns on whoever fired.
    fn hurt_monster(&mut self, m: usize, dmg: f32, by: Option<usize>) {
        let mon = &mut self.monsters[m];
        if !mon.alive() {
            return;
        }
        mon.hp -= dmg;
        if by.is_some() && mon.target.is_none() {
            mon.target = by;
        }
        if mon.hp > 0.0 {
            return;
        }
        mon.respawn = if mon.drop == Drop::Super { 60.0 } else { 40.0 };
        mon.target = None;
        let (at, drop) = (mon.pos, mon.drop);
        let who = by.map_or("a turret".to_string(), |b| if b == YOU { "YOU".into() } else { self.describe(b) });
        let what = match drop {
            Drop::Heal => "heal camp",
            Drop::Shield => "shield camp",
            Drop::Super => "river objective",
        };
        let msg = format!("{who} cleared the {what}");
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
        match drop {
            Drop::Heal | Drop::Shield => self.pickups.push(Pickup { pos: at.with_y(0.5), drop, life: 30.0 }),
            Drop::Super => {
                let kinds: &[Kind] = if self.super_mode > 0 { &[Kind::Super] } else { &[Kind::Normal; 3] };
                for (k, &kind) in kinds.iter().enumerate() {
                    let id = self.spawn_crate(kind, at + Vec3::Y * 1.0);
                    let a = k as f32 * 2.1;
                    if let Some(c) = self.crate_mut(id) {
                        c.vel = Vec3::new(a.cos() * 2.0, 6.0, a.sin() * 2.0);
                    }
                }
            }
        }
    }
}

fn new_gun(kind: GunKind, gold: bool) -> Gun {
    let ammo = match kind {
        GunKind::Shotgun => 12,
        GunKind::Rapid => 60,
        GunKind::Rocket => 6,
        GunKind::Heal => 80,
    };
    Gun { kind, ammo: if gold { ammo * 2 } else { ammo }, gold }
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
    // Jungle walls are tall: a crate bounces off them.
    for b in blocks() {
        let mut xz = p.xz();
        if let Some(n) = b.push_out(&mut xz, HALF) {
            p.x = xz.x;
            p.z = xz.y;
            let into = v.xz().dot(n);
            if into < 0.0 {
                let bounce = n * into * 1.4;
                v.x -= bounce.x;
                v.z -= bounce.y;
            }
            *settling = false;
        }
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
    if v.y == 0.0 && off < SNAP_RADIUS && (slowed < 1.5 || *settling) && !blocked(cell, HALF - 0.05) {
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
    /// Drop a crate (T) or a super crate (Y) at your feet, for trying recipes.
    drop: Option<Kind>,
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
    c.drop = if keys.just_pressed(KeyCode::KeyT) {
        Some(Kind::Normal)
    } else if keys.just_pressed(KeyCode::KeyY) {
        Some(Kind::Super)
    } else {
        None
    };
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
    // 1-7 step through each rule's options (restarts the match); Enter picks the rules.
    let rule_keys = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7];
    for (k, key) in rule_keys.iter().enumerate() {
        if keys.just_pressed(*key) {
            match &mut tune.vals[k].value {
                Value::Choice { i, opts } => *i = (*i + 1) % opts.len(),
                Value::Num { v, .. } => {
                    let at = BUILD_TIMES.iter().position(|b| (b - *v).abs() < 0.01).map_or(0, |p| p + 1);
                    *v = BUILD_TIMES[at % BUILD_TIMES.len()];
                }
            }
            tune.selected = k;
            sim.reset(&tune);
            let p = &tune.vals[k];
            sim.say(format!("RULE {}: {} (match restarted)", &p.name[2..], fmt_value(&p.value)));
        }
    }
    if keys.just_pressed(KeyCode::KeyC) {
        sim.your_class = sim.your_class.next();
        sim.actors[YOU].class = sim.your_class;
        let c = sim.your_class.name();
        sim.say(format!("you're a {c} now"));
    }
    if keys.just_pressed(KeyCode::Enter) {
        let line = rules_desc(&tune);
        println!("verdict: {line}");
        tune.verdicts.push(line);
        sim.say("picked these rules (P prints them)");
    }
    if keys.just_pressed(KeyCode::KeyP) {
        println!("# crate_grammar settings");
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
    if tune.c(P::TeamSize) != s.team_size || (tune.c(P::Map) == 0) != s.map {
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
    s.rules(tune);
    s.ally_stacks = tune.c(P::AllyStacks) == 1;
    s.you_play = tune.c(P::Seat) == 0;
    let t = s.t;
    for i in 0..s.actors.len() {
        let a = &mut s.actors[i];
        a.fire_cd -= dt;
        a.alt_cd -= dt;
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
    pads_step(s, dt);
    builds_step(s, dt);
    turrets_step(s, dt);
    walls_step(s, dt);
    vehicles_step(s, dt);
    monsters_step(s, dt);
    pickups_step(s, dt);
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
    let made = |t: usize| if s.made[t].is_empty() { "nothing".to_string() } else { s.made[t].join(", ") };
    let line = format!(
        "match: BLUE {b} - {r} RED, {winner} | {} | {} | blue player 1 = {seat} | kills {}-{} | lost to PvE {}-{} | crates knocked off pads {}-{} | builds broken {}-{} | blue made: {} | red made: {}",
        if s.map { "lanes + jungle" } else { "open arena" },
        rules_desc(tune),
        s.kills[0],
        s.kills[1],
        s.pve_deaths[0],
        s.pve_deaths[1],
        s.lost[0],
        s.lost[1],
        s.broken[0],
        s.broken[1],
        made(0),
        made(1),
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
        let speed = MOVE_SPEED * if carrying { CARRY_SPEED } else if a.tank.is_some() { 0.8 } else { 1.0 };
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
    let in_tank = s.actors[YOU].tank.is_some();

    if let Some(kind) = c.drop {
        let at = s.place_target(YOU);
        s.spawn_crate(kind, at + Vec3::Y * 0.5);
    }

    if c.grab && in_tank {
        s.say("you can't carry crates in a tank");
    } else if c.grab {
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

    // Right click with empty hands: the loadout's right-click item.
    if c.throw && !carrying && !in_tank {
        let range = c.aim_dist.map_or(8.0, |d| d.clamp(1.0, 30.0));
        use_alt(s, YOU, facing, range);
    }

    // Full lock: no shooting while carrying.
    if c.fire && !carrying && s.actors[YOU].fire_cd <= 0.0 {
        let range = c.aim_dist.map_or(30.0, |d| d.clamp(1.0, 30.0));
        fire(s, YOU, facing, range);
    }

    if !carrying && !in_tank {
        let pos = s.actors[YOU].pos;
        let my_pad = pads()[s.actors[YOU].team];
        // Hold kick beside your pad to open what's on it.
        if c.kick_held && pos.xz().distance(my_pad) < PAD_HALF + 1.4 {
            s.kicking[s.actors[YOU].team] = Some(YOU);
        }
        let mut opened = None;
        let mut punted = false;
        for cr in s.crates.iter_mut().filter(|cr| cr.loose() && !pad_contains(my_pad, cr.pos)) {
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

/// Fires `who`'s weapon: the tank's cannon, a crafted gun (until its ammo runs out), or the
/// standard gun. Sets the cooldown.
fn fire(s: &mut Sim, who: usize, dir: Vec3, range: f32) {
    let a = &s.actors[who];
    let from = a.pos + Vec3::Y * CHEST + dir * 0.5;
    let team = a.team;
    if who == YOU {
        s.fired += 1;
    }
    let shot = |vel: Vec3, dmg: f32, splash: f32, life: f32| Shot { pos: from, vel, owner: Some(who), team, dmg, splash, life, ent: None };
    if a.tank.is_some() {
        s.actors[who].fire_cd = 0.9;
        s.shots.push(shot(dir * 16.0, 80.0, 2.5, range.max(6.0) / 16.0));
        return;
    }
    let Some(mut gun) = a.gun else {
        s.actors[who].fire_cd = 1.0 / FIRE_RATE;
        s.shots.push(shot(dir * PROJ_SPEED, SHOT_DMG, 0.0, range / PROJ_SPEED));
        return;
    };
    let k = if gun.gold { 1.5 } else { 1.0 };
    match gun.kind {
        GunKind::Shotgun => {
            s.actors[who].fire_cd = 0.7;
            for p in -2..=2 {
                let d = Quat::from_rotation_y(p as f32 * 0.1) * dir;
                s.shots.push(shot(d * PROJ_SPEED, 14.0 * k, 0.0, 8.0 / PROJ_SPEED));
            }
        }
        GunKind::Rapid => {
            s.actors[who].fire_cd = 1.0 / 6.0;
            s.shots.push(shot(dir * PROJ_SPEED, 15.0 * k, 0.0, range / PROJ_SPEED));
        }
        GunKind::Rocket => {
            s.actors[who].fire_cd = 1.0;
            s.shots.push(shot(dir * 14.0, 70.0 * k, 2.5, range.max(4.0) / 14.0));
        }
        GunKind::Heal => {
            s.actors[who].fire_cd = 0.2;
            let pos = s.actors[who].pos;
            let mate = (0..s.actors.len())
                .filter(|&j| j != who && s.actors[j].team == team && s.actors[j].alive() && s.actors[j].hp < 100.0)
                .filter(|&j| {
                    let v = (s.actors[j].pos - pos).with_y(0.0);
                    v.length() < 10.0 && v.normalize_or_zero().dot(dir) > 0.9 && clear(pos.xz(), s.actors[j].pos.xz(), 0.1)
                })
                .min_by(|&x, &y| s.actors[x].pos.distance(pos).total_cmp(&s.actors[y].pos.distance(pos)));
            if let Some(j) = mate {
                s.actors[j].hp = (s.actors[j].hp + 8.0 * k).min(100.0);
                s.booms.push((s.actors[j].pos + Vec3::Y, 0.5, 0.15));
            } else {
                s.shots.push(shot(dir * PROJ_SPEED, 8.0 * k, 0.0, range / PROJ_SPEED));
            }
        }
    }
    gun.ammo -= 1;
    s.actors[who].gun = if gun.ammo == 0 { None } else { Some(gun) };
    if gun.ammo == 0 && who == YOU {
        s.say("out of ammo: back to your standard gun");
    }
}

/// Uses `who`'s right-click item (class loadout), if it has one and it's ready.
fn use_alt(s: &mut Sim, who: usize, dir: Vec3, range: f32) {
    let a = &s.actors[who];
    let Some(mut alt) = a.alt else { return };
    if a.alt_cd > 0.0 || a.tank.is_some() || a.carrying.is_some() || !a.alive() {
        return;
    }
    let (pos, team) = (a.pos, a.team);
    let k = if alt.gold { 1.5 } else { 1.0 };
    match alt.kind {
        AltKind::Rockets => {
            s.actors[who].alt_cd = 1.0;
            let from = pos + Vec3::Y * CHEST + dir * 0.5;
            s.shots.push(Shot { pos: from, vel: dir * 14.0, owner: Some(who), team, dmg: 70.0 * k, splash: 2.5, life: range.max(4.0) / 14.0, ent: None });
        }
        AltKind::Turret => {
            s.actors[who].alt_cd = 1.0;
            let mut at = pos + dir * range.clamp(1.5, 8.0);
            if !clear(pos.xz(), at.xz(), 0.5) {
                at = pos + dir * 1.5;
            }
            s.turrets.push(Turret { team, pos: at.with_y(0.0), facing: dir, cd: 0.5, rate: if alt.gold { 0.4 } else { 0.8 }, life: 30.0, ent: None });
        }
        AltKind::Pulse => {
            s.actors[who].alt_cd = 2.0;
            for b in s.actors.iter_mut().filter(|b| b.team == team && b.alive() && b.pos.distance(pos) < 7.0) {
                b.hp = (b.hp + 40.0 * k).min(100.0);
            }
            s.booms.push((pos + Vec3::Y * 0.2, 7.0, 0.4));
        }
    }
    alt.charges -= 1;
    s.actors[who].alt = if alt.charges == 0 { None } else { Some(alt) };
    if alt.charges == 0 && who == YOU {
        s.say("right-click item used up");
    }
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
        // Only enemies in sight: jungle walls block shots.
        .filter(|(j, ..)| clear(around.xz(), s.actors[*j].pos.xz(), 0.1))
        .min_by(|x, y| x.2.total_cmp(&y.2))
        .map(|(j, ..)| j)
}

/// Where a bot goes in the jungle, if anywhere: a dropped heal or shield close by, a camp
/// when it's hurt (heal camps first), or any camp for the team's 3rd player (its jungler).
fn jungle_goal(s: &Sim, i: usize) -> Option<Vec3> {
    let a = &s.actors[i];
    if !s.map || a.carrying.is_some() || a.tank.is_some() {
        return None;
    }
    let pick = s
        .pickups
        .iter()
        .filter(|p| p.pos.xz().distance(a.pos.xz()) < 12.0 && (p.drop != Drop::Heal || a.hp < 100.0))
        .min_by(|x, y| x.pos.distance(a.pos).total_cmp(&y.pos.distance(a.pos)));
    if let Some(p) = pick {
        return Some(p.pos.with_y(0.0));
    }
    let hurt = a.hp < 60.0;
    let jungler = a.slot == 2;
    if !hurt && !jungler {
        return None;
    }
    s.monsters
        .iter()
        .filter(|m| m.alive() && (!hurt || m.drop == Drop::Heal || jungler))
        // Hurt bots go for heals; junglers go for anything, river objectives first (they
        // drop crates, so both teams want them).
        .min_by(|x, y| {
            let score = |m: &Monster| m.pos.distance(a.pos) - if m.drop == Drop::Super && !hurt { 15.0 } else { 0.0 };
            score(x).total_cmp(&score(y))
        })
        .map(|m| m.pos.with_y(0.0))
}

/// A monster bot `i` should shoot: one fighting it, or one at a camp it's clearing.
fn monster_aim(s: &Sim, i: usize, jungling: bool) -> Option<Vec3> {
    let a = &s.actors[i];
    s.monsters
        .iter()
        .filter(|m| m.alive() && m.pos.xz().distance(a.pos.xz()) < 11.0 && clear(a.pos.xz(), m.pos.xz(), 0.1))
        .filter(|m| m.target == Some(i) || jungling)
        .min_by(|x, y| x.pos.distance(a.pos).total_cmp(&y.pos.distance(a.pos)))
        .map(|m| m.pos.with_y(CHEST))
}

/// The next point toward `goal` around the jungle walls, recomputed a few times a second.
fn steer(a: &mut Actor, goal: Vec3, dt: f32) -> Vec3 {
    if !map_on() {
        return goal;
    }
    if let Some((g, next, age)) = &mut a.nav {
        *age += dt;
        if g.distance(goal.xz()) < 1.0 && *age < 0.4 && next.distance(a.pos.xz()) > 0.6 {
            return next.extend(0.0).xzy();
        }
    }
    let next = route(a.pos.xz(), goal.xz());
    a.nav = Some((goal.xz(), next, 0.0));
    next.extend(0.0).xzy()
}

enum Act {
    None,
    Grab(u32),
    Place(Vec3),
    /// Set the crate down where you stand.
    Drop,
}

fn act_none(a: &Act) -> bool {
    matches!(a, Act::None)
}

/// Fetch the nearest crate no teammate is after (stealing from the enemy pad included) and
/// set it down on the team pad. Returns the goal and the action; `None` goal and no action
/// means there's nothing to haul. Bot grabs are instant: they stand in for the server.
fn haul_plan(s: &Sim, i: usize) -> (Option<Vec3>, Act, Option<u32>) {
    let a = &s.actors[i];
    if a.tank.is_some() {
        return (None, Act::None, None);
    }
    if a.carrying.is_some() {
        let shape = s.pad_state(a.team).shape;
        let builds = a.team == 1 || s.ally_stacks || i == YOU;
        // Hold on to it while the pad is ready to open (or full), so the shape survives.
        let ready = s.can_open(a.team, shape) && worth_opening(s, a.team, shape);
        let cell = s.pad_cell(i).filter(|_| !(builds && ready));
        let Some(cell) = cell else {
            let front = pad_front(a.team, a.slot);
            // Nobody free to kick it open: set the crate down beside the pad and kick it.
            let kicker = s.actors.iter().enumerate().any(|(j, b)| {
                j != i && b.team == a.team && b.alive() && b.carrying.is_none() && b.tank.is_none()
            });
            if builds && ready && !kicker && front.xz().distance(a.pos.xz()) < 1.0 {
                return (None, Act::Drop, None);
            }
            return (Some(front), Act::None, None);
        };
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
        .filter(|c| c.loose() && c.kind.builds() && c.pos.y < 3.0)
        .filter(|c| !pad_contains(pad, c.pos) && !claimed(c.id))
        // Super crates are worth a longer walk.
        .map(|c| (c, c.pos.distance(a.pos) - if c.kind == Kind::Super && s.super_mode > 0 { 8.0 } else { 0.0 }))
        .min_by(|x, y| x.1.total_cmp(&y.1))
        .map(|(c, _)| c);
    match pick {
        None => (None, Act::None, None),
        Some(c) if c.pos.xz().distance(a.pos.xz()) <= GRAB_REACH + HALF => (None, Act::Grab(c.id), Some(c.id)),
        Some(c) => (Some(c.pos.with_y(0.0)), Act::None, Some(c.id)),
    }
}

/// Recipes bots open: players while the team has room and the big recipes right away;
/// small ones (and 5-7 by count) only once the pad has sat unchanged a while; a pad stuck
/// out of line after a few seconds.
fn worth_opening(s: &Sim, team: usize, shape: Shape) -> bool {
    match shape {
        Shape::Recipe(Slot::Player1 | Slot::Player2 | Slot::All8) => true,
        Shape::Recipe(Slot::Flat4) if !s.count_only => true,
        // Class loadout: 2 side by side is the left-click item, worth it once the team is full
        // and nobody has added to the pad for a while (otherwise it grows to 2 x 2 flat).
        Shape::Recipe(Slot::Pair) if s.book == BY_LOADOUT && s.players(team) >= s.team_size => s.pad_t[team] > 8.0,
        // Small recipes once the pad has sat a while (longer on the big map: hauls are long).
        Shape::Recipe(_) => s.pad_t[team] > if s.map { 25.0 } else { 7.0 },
        Shape::Unaligned => s.odd_t[team] > 4.0,
        _ => false,
    }
}

/// A spot beside the pad, on the side facing the arena.
fn pad_front(team: usize, slot: usize) -> Vec3 {
    let pad = pads()[team];
    Vec3::new(pad.x + toward_centre(team) * 1.6, 0.0, pad.y + (slot as f32 % 3.0 - 1.0) * 0.9)
}

fn guard_post(s: &Sim, i: usize) -> Vec3 {
    let a = &s.actors[i];
    let pad = pads()[a.team];
    Vec3::new(pad.x + toward_centre(a.team) * 2.8, 0.0, pad.y + (a.slot as f32 - 1.0) * 1.6)
}

/// Whether bot `i` should go kick its pad open: the pad makes something worth opening,
/// and `i` is the nearest empty-handed teammate allowed to.
fn wants_to_kick(s: &Sim, i: usize) -> bool {
    let team = s.actors[i].team;
    if !(team == 1 || s.ally_stacks || i == YOU) {
        return false;
    }
    let shape = s.pad_state(team).shape;
    if !(s.can_open(team, shape) || shape == Shape::Unaligned) || !worth_opening(s, team, shape) {
        return false;
    }
    let pad = pads()[team];
    let me = s.actors[i].pos.xz().distance(pad);
    let human = tune_free_seat(s);
    !(0..s.actors.len()).any(|j| {
        let b = &s.actors[j];
        j != i && j != human && b.team == team && b.alive() && b.carrying.is_none() && b.pos.xz().distance(pad) < me
    })
}

/// The seat you're playing (bots don't wait for you to kick), or none when it's a bot.
fn tune_free_seat(s: &Sim) -> usize {
    if s.you_play { YOU } else { usize::MAX }
}

/// Where bot `i` would shoot the enemy pad: a build in progress, or (when hits split or
/// spill) the crates on it, if it's within 11 m.
fn pad_aim(s: &Sim, i: usize) -> Option<Vec3> {
    let a = &s.actors[i];
    let enemy = 1 - a.team;
    let pad = pads()[enemy];
    if a.carrying.is_some() || a.pos.xz().distance(pad) > 11.0 {
        return None;
    }
    let crates = s.crates.iter().any(|c| c.state == CrateState::Free && c.kind.builds() && pad_contains(pad, c.pos));
    (crates && (s.builds[enemy].is_some() || matches!(s.vuln, SPLIT | SPILL))).then(|| pad.extend(CHEST).xzy())
}

/// A bot player: carry home, fight what's close, chase enemy carriers to steal back,
/// otherwise haul (and steal). Builds pad shapes and kicks them open. Guards the pad when
/// there's nothing to haul.
fn ai_step(s: &mut Sim, i: usize, dt: f32) {
    if !s.actors[i].alive() {
        return;
    }
    let pos = s.actors[i].pos;
    let carrying = s.actors[i].carrying.is_some();
    let near = nearest_enemy(s, i, pos, if s.map { 10.0 } else { 8.0 });
    let carrier = nearest_enemy(s, i, pos, 14.0).filter(|&e| s.actors[e].carrying.is_some());
    let keep = s.actors[i].claim;
    let in_tank = s.actors[i].tank.is_some();
    let enemy_pad = pads()[1 - s.actors[i].team];
    let (target, goal, act, claim) = match (carrying, near, carrier) {
        // A tank pushes to the enemy pad and fights whatever it meets.
        (false, _, _) if in_tank => {
            let e = nearest_enemy(s, i, pos, 12.0);
            let goal = Vec3::new(enemy_pad.x - toward_centre(1 - s.actors[i].team) * 3.0, 0.0, enemy_pad.y);
            (e, Some(goal), Act::None, None)
        }
        (false, Some(e), _) => {
            // Strafe while fighting.
            let to = (s.actors[e].pos - pos).with_y(0.0).normalize_or_zero();
            let side = Vec3::new(-to.z, 0.0, to.x) * (s.t * 0.7 + i as f32).sin().signum();
            (Some(e), Some(pos + side * 2.0), Act::None, keep)
        }
        (false, None, Some(e)) => (Some(e), Some(s.actors[e].pos), Act::None, keep),
        (false, None, None) if wants_to_kick(s, i) => {
            // Go to the pad and kick it open.
            let team = s.actors[i].team;
            let spot = pad_front(team, s.actors[i].slot);
            let d = pos.xz().distance(pads()[team]);
            if d < PAD_HALF + 1.4 {
                s.kicking[team] = Some(i);
                (None, None, Act::None, None)
            } else if pos.xz().distance(spot.xz()) < 0.6 {
                // At the spot but held off the pad: walk straight at it.
                (None, Some(pads()[team].extend(0.0).xzy()), Act::None, None)
            } else {
                (None, Some(spot), Act::None, None)
            }
        }
        (false, None, None) if let Some(goal) = jungle_goal(s, i) => (None, Some(goal), Act::None, None),
        _ => {
            let (goal, act, claim) = haul_plan(s, i);
            let idle = goal.is_none() && matches!(act, Act::None);
            (None, if idle { Some(guard_post(s, i)) } else { goal }, act, claim)
        }
    };
    let speed = MOVE_SPEED * 0.85 * if carrying { CARRY_SPEED } else if in_tank { 0.8 } else { 1.0 };
    let spread = (s.rand() - 0.5) * 0.15;
    let jungling = target.is_none() && act_none(&act) && jungle_goal(s, i).is_some();
    {
        let a = &mut s.actors[i];
        a.claim = claim;
        let goal = goal.map(|g| steer(a, g, dt));
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
    // Medics: pulse when someone close is hurt, and aim the heal gun at hurt teammates.
    if !carrying && s.actors[i].alt.is_some_and(|a| a.kind == AltKind::Pulse) {
        let team = s.actors[i].team;
        if s.actors.iter().any(|b| b.team == team && b.alive() && b.hp < 60.0 && b.pos.distance(pos) < 6.0) {
            let f = s.actors[i].facing;
            use_alt(s, i, f, 0.0);
        }
    }
    let hurt_mate = if !carrying && s.actors[i].gun.is_some_and(|g| g.kind == GunKind::Heal) {
        let team = s.actors[i].team;
        (0..s.actors.len())
            .filter(|&j| j != i && s.actors[j].team == team && s.actors[j].alive() && s.actors[j].hp < 80.0)
            .filter(|&j| s.actors[j].pos.distance(pos) < 9.0 && clear(pos.xz(), s.actors[j].pos.xz(), 0.1))
            .min_by(|&x, &y| s.actors[x].hp.total_cmp(&s.actors[y].hp))
    } else {
        None
    };
    if let Some(m) = hurt_mate {
        let dir = (s.actors[m].pos - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        if s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, 10.0);
        }
    } else if let Some(e) = target {
        let (ep, ev) = (s.actors[e].pos, s.actors[e].vel.with_y(0.0));
        let d = ep.xz().distance(pos.xz());
        // Lead the target, then miss a little.
        let lead = ep + ev * (d / PROJ_SPEED);
        let dir = Quat::from_rotation_y(spread) * (lead - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        let alt = s.actors[i].alt.map(|a| a.kind);
        if !carrying && d < 12.0 && matches!(alt, Some(AltKind::Rockets | AltKind::Turret)) && s.actors[i].stun <= 0.0 {
            use_alt(s, i, dir, d.min(6.0));
        }
        if !carrying && d < 12.0 && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, 14.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    } else if let Some(at) = monster_aim(s, i, jungling) {
        let dir = Quat::from_rotation_y(spread) * (at - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        if !carrying && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, (at - pos).length() + 1.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    } else if let Some(at) = pad_aim(s, i) {
        // Nobody to fight: shoot the enemy pad when hits do something there.
        let dir = Quat::from_rotation_y(spread) * (at - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        if !carrying && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, (at - pos).length() + 1.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    }
    match act {
        Act::Grab(id) => s.take(i, id),
        Act::Place(at) => s.place(i, at),
        Act::Drop => {
            let at = s.actors[i].pos.with_y(HALF) + s.actors[i].facing * -0.2;
            s.place(i, at);
        }
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
    // Jungle walls block everyone.
    for a in s.actors.iter_mut().filter(|a| a.alive()) {
        for b in blocks() {
            let mut xz = a.pos.xz();
            if b.push_out(&mut xz, ACTOR_R).is_some() {
                a.pos.x = xz.x;
                a.pos.z = xz.y;
            }
        }
    }
    // Shield walls block enemy players.
    for a in s.actors.iter_mut().filter(|a| a.alive()) {
        for w in s.walls.iter().filter(|w| w.team != a.team) {
            let d = a.pos - w.pos;
            if d.x.abs() < 0.25 + ACTOR_R && d.z.abs() < w.half_z {
                a.pos.x = w.pos.x + d.x.signum() * (0.25 + ACTOR_R);
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
            // Locked crates are asleep: no motion to simulate or send.
            CrateState::Free if c.locked => {}
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
            if a.state != CrateState::Free || b.state != CrateState::Free || !side_hit(a.pos, b.pos) || (a.locked && b.locked) {
                continue;
            }
            // A locked crate doesn't budge: the other one bounces off it.
            if a.locked || b.locked {
                let (m, o) = if a.locked { (j, i) } else { (i, j) };
                let d = s.crates[m].pos.xz() - s.crates[o].pos.xz();
                let (axis, pen) = if (CRATE - d.x.abs()) < (CRATE - d.y.abs()) {
                    (Vec2::new(d.x.signum(), 0.0), CRATE - d.x.abs())
                } else {
                    (Vec2::new(0.0, d.y.signum()), CRATE - d.y.abs())
                };
                let c = &mut s.crates[m];
                c.pos += (axis * pen).extend(0.0).xzy();
                let into = c.vel.xz().dot(axis);
                if into < 0.0 {
                    c.vel -= (axis * into * 1.4).extend(0.0).xzy();
                }
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
    let mut booms = Vec::new();
    let mut pad_hits = Vec::new();
    let mut monster_hits = Vec::new();
    // Stack tops per enemy pad cell, for shots crossing a pad.
    let tops: [[f32; 4]; 2] = [0, 1].map(|team| {
        let cells = pad_cells(team);
        let mut top = [0.0f32; 4];
        for c in s.crates.iter().filter(|c| c.state == CrateState::Free && c.kind.builds() && pad_contains(pads()[team], c.pos)) {
            if let Some(k) = cells.iter().position(|q| q.distance(c.pos.xz()) < 0.3) {
                top[k] = top[k].max(c.pos.y + HALF);
            }
        }
        top
    });
    for sh in &mut s.shots {
        sh.pos += sh.vel * dt;
        sh.life -= dt;
        let hit = s.actors.iter().position(|a| {
            let r = if a.tank.is_some() { 0.9 } else { ACTOR_R + 0.15 };
            a.team != sh.team && a.alive() && sh.pos.xz().distance(a.pos.xz()) < r && sh.pos.y > a.pos.y && sh.pos.y < a.pos.y + 1.9
        });
        if let Some(j) = hit {
            if sh.splash > 0.0 {
                booms.push((sh.pos, sh.team, sh.dmg, sh.splash, sh.owner));
            } else {
                hits.push((j, sh.vel.normalize(), sh.owner, sh.team, sh.dmg));
            }
            sh.life = 0.0;
            continue;
        }
        if blocked(sh.pos.xz(), 0.0) {
            if sh.splash > 0.0 {
                booms.push((sh.pos, sh.team, sh.dmg, sh.splash, sh.owner));
            }
            sh.life = 0.0;
            continue;
        }
        if let Some(m) = s.monsters.iter().position(|m| m.alive() && m.pos.xz().distance(sh.pos.xz()) < 0.9 && sh.pos.y < 2.2) {
            if sh.splash > 0.0 {
                booms.push((sh.pos, sh.team, sh.dmg, sh.splash, sh.owner));
            } else {
                monster_hits.push((m, sh.dmg, sh.owner));
            }
            sh.life = 0.0;
            continue;
        }
        if let Some(w) = s.walls.iter_mut().find(|w| {
            w.team != sh.team && (sh.pos.x - w.pos.x).abs() < 0.3 && (sh.pos.z - w.pos.z).abs() < w.half_z && sh.pos.y < WALL_H
        }) {
            w.hp -= sh.dmg;
            if sh.splash > 0.0 {
                booms.push((sh.pos, sh.team, sh.dmg, sh.splash, sh.owner));
            }
            sh.life = 0.0;
            continue;
        }
        // Crossing the enemy pad over a stack (or a build) hits it.
        let enemy = 1 - sh.team;
        let cells = pad_cells(enemy);
        if let Some(k) = (0..4).find(|&k| (sh.pos.xz() - cells[k]).abs().max_element() < HALF && tops[enemy][k] > 0.0) {
            let building = s.builds[enemy].is_some();
            let rule = matches!(s.vuln, SPLIT | SPILL);
            if (building || rule) && sh.pos.y < tops[enemy][k] + 0.6 {
                if sh.splash > 0.0 {
                    booms.push((sh.pos, sh.team, sh.dmg, sh.splash, sh.owner));
                } else {
                    pad_hits.push((enemy, sh.pos, sh.dmg));
                }
                sh.life = 0.0;
                continue;
            }
        }
        if sh.life <= 0.0 && sh.splash > 0.0 {
            booms.push((sh.pos, sh.team, sh.dmg, sh.splash, sh.owner));
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
    for (m, dmg, owner) in monster_hits {
        s.hurt_monster(m, dmg, owner);
    }
    for (team, at, dmg) in pad_hits {
        s.pad_hit(team, at, dmg, 0.0);
    }
    for (at, team, dmg, r, owner) in booms {
        s.explode(at, team, dmg, r, owner);
        s.booms.push((at, r, 0.3));
    }
}

/// Turrets shoot the nearest enemy, then expire.
fn turrets_step(s: &mut Sim, dt: f32) {
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
                t.cd = t.rate;
                let from = t.pos + Vec3::Y * 1.2 + t.facing * 0.6;
                let vel = t.facing * PROJ_SPEED;
                s.shots.push(Shot { pos: from, vel, owner: None, team, dmg: SHOT_DMG, splash: 0.0, life: 1.0, ent: None });
            }
        }
    }
    s.turrets.retain(|t| {
        if t.life <= 0.0 { s.despawn.extend(t.ent); }
        t.life > 0.0
    });
}

/// Kick-open progress, locking crates into the pad (lock rule), and crates stacked higher
/// than 2 on a pad slide off.
fn pads_step(s: &mut Sim, dt: f32) {
    for team in 0..2 {
        let shape = s.pad_state(team).shape;
        s.odd_t[team] = if shape == Shape::Unaligned { s.odd_t[team] + dt } else { 0.0 };
        s.pad_t[team] = if shape == s.last_shape[team] { s.pad_t[team] + dt } else { 0.0 };
        s.last_shape[team] = shape;
        // With a secret readout you can try to kick any pad; only a recipe opens.
        let tryable = !s.shown && matches!(shape, Shape::Odd);
        let held = s.kicking[team].take();
        if s.hold_kick && shape != Shape::Unaligned && shape != Shape::Busy {
            let stages = s.stages(team);
            match held {
                Some(by) if !stages.is_empty() => {
                    s.pad_kick[team] += dt;
                    s.kick_by[team] = Some(by);
                    // Bots (and a pad with nothing bigger to reach) open at the last stage.
                    let at = s.stage_at(team, stages.len());
                    if (by != YOU || !s.you_play) && at == Some(stages.len() - 1) {
                        s.pad_kick[team] = 0.0;
                        let (slot, take) = stages[stages.len() - 1];
                        s.open_stage(team, by, slot, take);
                    }
                }
                Some(YOU) if shape.players() > 0 => {
                    if s.msgs.last().is_none_or(|m| !m.0.starts_with("team is full")) {
                        s.say("team is full: this shape brings players, so it won't open");
                    }
                }
                Some(_) => {}
                None => {
                    // Released: open the recipe the hold reached.
                    if let (Some(n), Some(by)) = (s.stage_at(team, stages.len()), s.kick_by[team]) {
                        let (slot, take) = stages[n];
                        s.open_stage(team, by, slot, take);
                    }
                    s.pad_kick[team] = 0.0;
                    s.kick_by[team] = None;
                }
            }
        } else {
        match held {
            // A pad with a crate out of line can only be cleared by breaking it.
            Some(by) if s.can_open(team, shape) || shape == Shape::Unaligned || tryable => {
                s.pad_kick[team] += dt;
                if s.pad_kick[team] >= KICK_TIME {
                    s.pad_kick[team] = 0.0;
                    if tryable {
                        if by == YOU {
                            s.say("nothing happened: that's not a recipe. Add crates");
                        }
                    } else {
                        s.open_pad(team, by);
                    }
                }
            }
            Some(YOU) if shape.players() > 0 => {
                s.pad_kick[team] = 0.0;
                if s.msgs.last().is_none_or(|m| !m.0.starts_with("team is full")) {
                    s.say("team is full: this shape brings players, so it won't open");
                }
            }
            _ => s.pad_kick[team] = (s.pad_kick[team] - dt).max(0.0),
        }
        }
        let pad = pads()[team];
        let cells = pad_cells(team);
        let lock = s.vuln == LOCK;
        for c in s.crates.iter_mut().filter(|c| c.loose() && c.kind.builds() && pad_contains(pad, c.pos)) {
            if c.pos.y > HALF + 2.0 * CRATE - 0.1 && c.vel.length() < 0.5 {
                let out = (c.pos.xz() - pad).normalize_or(Vec2::X);
                c.vel = Vec3::new(out.x * 3.0, 2.0, out.y * 3.0);
                continue;
            }
            let level = (c.pos.y - HALF) / CRATE;
            let in_cell = cells.iter().any(|q| q.distance(c.pos.xz()) < 0.02) && (level - level.round()).abs() < 0.03;
            if lock && in_cell && c.vel.length() < 0.05 && !c.settling {
                c.locked = true;
                c.vel = Vec3::ZERO;
            }
        }
    }
}

/// Build timers count down; a finished build gives its result.
fn builds_step(s: &mut Sim, dt: f32) {
    for team in 0..2 {
        if let Some(b) = &mut s.builds[team] {
            b.left -= dt;
            if b.left <= 0.0 {
                s.finish_build(team);
            }
        }
    }
}

fn walls_step(s: &mut Sim, dt: f32) {
    for w in &mut s.walls {
        w.life -= dt;
    }
    let mut broke = Vec::new();
    s.walls.retain(|w| {
        let gone = w.life <= 0.0 || w.hp <= 0.0;
        if gone {
            s.despawn.extend(w.ent);
            if w.hp <= 0.0 {
                broke.push(w.team);
            }
        }
        !gone
    });
    for team in broke {
        s.say(format!("{} shield wall broken", ["BLUE", "RED"][team]));
    }
    for b in &mut s.booms {
        b.2 -= dt;
    }
    s.booms.retain(|b| b.2 > 0.0);
}

fn vehicles_step(s: &mut Sim, dt: f32) {
    for i in 0..s.actors.len() {
        if let Some(t) = &mut s.actors[i].tank {
            t.life -= dt;
            if t.life <= 0.0 {
                s.actors[i].tank = None;
                if i == YOU {
                    s.say("your tank ran out of time");
                }
            }
        }
    }
}

/// PvE camps: monsters fight whoever comes close or shoots them, chase within their leash,
/// then walk home and heal. Cleared camps come back after a while.
fn monsters_step(s: &mut Sim, dt: f32) {
    for m in 0..s.monsters.len() {
        let mon = &mut s.monsters[m];
        if !mon.alive() {
            mon.respawn -= dt;
            if mon.respawn <= 0.0 {
                mon.respawn = 0.0;
                mon.hp = mon.max;
                mon.pos = mon.home;
            }
            continue;
        }
        mon.cd -= dt;
        // Keep a target while it's alive and within the leash; else pick up anyone close.
        let home = mon.home;
        let keep = mon.target.filter(|&t| s.actors[t].alive() && s.actors[t].pos.xz().distance(home.xz()) < LEASH + 2.0);
        let pos = mon.pos;
        let near = (0..s.actors.len())
            .filter(|&j| s.actors[j].alive() && s.actors[j].pos.xz().distance(pos.xz()) < 3.5)
            .min_by(|&a, &b| s.actors[a].pos.distance(pos).total_cmp(&s.actors[b].pos.distance(pos)));
        let target = keep.or(near);
        let mon = &mut s.monsters[m];
        mon.target = target;
        let goal = match target {
            Some(t) => s.actors[t].pos,
            None => mon.home,
        };
        let to = (goal - mon.pos).with_y(0.0);
        let reach = if target.is_some() { 1.5 } else { 0.2 };
        if to.length() > reach {
            let speed = if target.is_some() { 4.8 } else { 6.0 };
            mon.pos += to.normalize() * speed * dt;
        }
        if target.is_none() && to.length() < 0.5 {
            mon.hp = (mon.hp + mon.max * 0.5 * dt).min(mon.max);
        }
        if let Some(t) = target
            && to.length() < 1.7
            && mon.cd <= 0.0
        {
            mon.cd = 1.0;
            let (dmg, knock) = (mon.dmg, to.normalize_or_zero() * 3.0);
            s.hurt(t, dmg, knock, None, 2);
        }
    }
}

/// Heals and shields lying where a camp was cleared: whoever walks over one gets it.
fn pickups_step(s: &mut Sim, dt: f32) {
    let mut k = 0;
    while k < s.pickups.len() {
        s.pickups[k].life -= dt;
        let at = s.pickups[k].pos;
        let by = (0..s.actors.len()).find(|&j| s.actors[j].alive() && s.actors[j].pos.xz().distance(at.xz()) < 1.2);
        if let Some(j) = by {
            let drop = s.pickups.remove(k).drop;
            let a = &mut s.actors[j];
            let what = match drop {
                Drop::Heal => {
                    a.hp = (a.hp + 60.0).min(100.0);
                    "a heal (+60 hp)"
                }
                _ => {
                    a.shield = (a.shield + 60.0).min(100.0);
                    "a shield (+60)"
                }
            };
            let who = if j == YOU { "you".to_string() } else { s.describe(j) };
            s.say(format!("{who} picked up {what}"));
            continue;
        }
        if s.pickups[k].life <= 0.0 {
            s.pickups.remove(k);
            continue;
        }
        k += 1;
    }
}

fn refill(s: &mut Sim, dt: f32) {
    let loose = s
        .crates
        .iter()
        .filter(|c| c.kind == Kind::Normal)
        .filter(|c| c.state != CrateState::Free || !(pad_contains(pads()[0], c.pos) || pad_contains(pads()[1], c.pos)))
        .count();
    let want = if s.map { 14 + 2 * s.team_size } else { 3 + 2 * s.team_size };
    if loose < want {
        let a = arena();
        // On the lanes map new crates land in the contested middle: the river and mid lane.
        let span = Vec2::new(a.x * if s.map { 0.25 } else { 0.4 }, a.y * 0.8);
        let p = open_spot(|| s.rand(), span);
        s.spawn_crate(Kind::Normal, Vec3::new(p.x, 10.0, p.y));
    }
    // One super crate at a time drops somewhere in the middle third.
    if s.super_mode > 0 && !s.crates.iter().any(|c| c.kind == Kind::Super) {
        s.super_respawn -= dt;
        if s.super_respawn <= 0.0 {
            let span = Vec2::new(arena().x * if s.map { 0.1 } else { 0.3 }, arena().y * 0.7);
            let p = open_spot(|| s.rand(), span);
            s.spawn_crate(Kind::Super, Vec3::new(p.x, 10.0, p.y));
            s.super_respawn = 25.0;
            s.say("a SUPER crate dropped in the middle");
        }
    }
    if s.crates.iter().filter(|c| c.kind == Kind::Loot).count() < 2 {
        s.loot_respawn -= dt;
        if s.loot_respawn <= 0.0 {
            let p = open_spot(|| s.rand(), Vec2::new(3.0 * arena_scale().min(2.0), arena().y * 0.65));
            s.spawn_crate(Kind::Loot, Vec3::new(p.x, 10.0, p.y));
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
    sup: Handle<StandardMaterial>,
    monster_mesh: Handle<Mesh>,
    monster_mat: [Handle<StandardMaterial>; 2],
    marker: Handle<Mesh>,
    camp: Handle<StandardMaterial>,
    wall_mesh: Handle<Mesh>,
    wall_mat: [Handle<StandardMaterial>; 2],
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
struct Minimap;

/// A marker only the minimap sees: a player (actor index) or a camp (monster index).
#[derive(Component)]
enum Marker {
    Actor(usize),
    Monster(usize),
}

/// The lanes map's static meshes (walls, lane floors, river), rebuilt when the map changes.
#[derive(Resource, Default)]
struct MapView {
    built: Option<bool>,
    ents: Vec<Entity>,
}

fn sync_map(
    mut commands: Commands,
    sim: Res<Sim>,
    mut view: ResMut<MapView>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    if view.built == Some(sim.map) && !view.ents.is_empty() == sim.map {
        return;
    }
    for e in view.ents.drain(..) {
        commands.entity(e).despawn();
    }
    view.built = Some(sim.map);
    if !sim.map {
        return;
    }
    let wall = mats.add(Color::srgb(0.16, 0.3, 0.14));
    for b in blocks() {
        let size = b.max - b.min;
        let c = (b.min + b.max) / 2.0;
        let e = commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(size.x, BLOCK_H, size.y))),
                MeshMaterial3d(wall.clone()),
                Transform::from_xyz(c.x, BLOCK_H / 2.0, c.y),
            ))
            .id();
        view.ents.push(e);
    }
    // Floors: jungle darker, lanes lighter, river blue.
    let mut floor = |commands: &mut Commands, c: Vec2, size: Vec2, color: Color, y: f32| {
        let e = commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(size.x, 0.02, size.y))),
                MeshMaterial3d(mats.add(color)),
                Transform::from_xyz(c.x, y, c.y),
            ))
            .id();
        view.ents.push(e);
    };
    for sx in [-1.0, 1.0] {
        for sz in [-1.0, 1.0] {
            floor(&mut commands, Vec2::new(17.5 * sx, 11.0 * sz), Vec2::new(27.0, 13.0), Color::srgb(0.2, 0.27, 0.18), 0.005);
        }
    }
    let lane = Color::srgb(0.48, 0.45, 0.36);
    floor(&mut commands, Vec2::new(0.0, 21.5), Vec2::new(62.0, 7.0), lane, 0.006);
    floor(&mut commands, Vec2::new(0.0, -21.5), Vec2::new(62.0, 7.0), lane, 0.006);
    floor(&mut commands, Vec2::new(0.0, 0.0), Vec2::new(62.0, 8.0), lane, 0.006);
    floor(&mut commands, Vec2::new(0.0, 0.0), Vec2::new(8.0, 50.0), Color::srgb(0.22, 0.38, 0.55), 0.008);
    for (team, color) in TEAM_COLOR.into_iter().enumerate() {
        let x = if team == 0 { -38.0 } else { 38.0 };
        floor(&mut commands, Vec2::new(x, 0.0), Vec2::new(14.0, 50.0), color.mix(&Color::srgb(0.3, 0.33, 0.3), 0.75), 0.004);
    }
}

/// Sizes the minimap to the bottom-right corner, and turns it off in the open arena.
fn minimap_view(sim: Res<Sim>, window: Single<&Window>, mut cam: Single<&mut Camera, With<Minimap>>) {
    cam.is_active = sim.map;
    let size = window.physical_size();
    let w = (size.x as f32 * 0.3) as u32;
    let h = (w as f32 * (MAP_HALF.y + 1.0) / (MAP_HALF.x + 1.0)) as u32;
    if w < 10 || h + 10 > size.y {
        return;
    }
    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(size.x - w - 10, size.y - h - 10),
        physical_size: UVec2::new(w, h),
        ..default()
    });
}

/// Minimap markers follow the players and camps they stand for.
fn sync_markers(
    mut commands: Commands,
    sim: Res<Sim>,
    looks: Res<Looks>,
    mut markers: Query<(Entity, &Marker, &mut Transform, &mut Visibility)>,
    mut seen: Local<(usize, usize)>,
) {
    if *seen != (sim.actors.len(), sim.monsters.len()) {
        for (e, ..) in &markers {
            commands.entity(e).despawn();
        }
        *seen = (sim.actors.len(), sim.monsters.len());
        for (i, a) in sim.actors.iter().enumerate() {
            let mat = if i == YOU { looks.player[0].clone() } else { looks.team[a.team].clone() };
            commands.spawn((Marker::Actor(i), Mesh3d(looks.marker.clone()), MeshMaterial3d(mat), Transform::default(), RenderLayers::layer(1)));
        }
        for m in 0..sim.monsters.len() {
            commands.spawn((Marker::Monster(m), Mesh3d(looks.marker.clone()), MeshMaterial3d(looks.camp.clone()), Transform::default(), RenderLayers::layer(1)));
        }
        return;
    }
    for (_, marker, mut tf, mut vis) in &mut markers {
        let (pos, alive, scale) = match marker {
            Marker::Actor(i) => (sim.actors[*i].pos, sim.actors[*i].alive(), if *i == YOU { 1.3 } else { 1.0 }),
            Marker::Monster(m) => (sim.monsters[*m].pos, sim.monsters[*m].alive(), 0.8),
        };
        tf.translation = pos.with_y(30.0);
        tf.scale = Vec3::splat(scale);
        *vis = if alive && sim.map { Visibility::Visible } else { Visibility::Hidden };
    }
}

#[derive(Component)]
struct GuideText;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        MainCamera,
        IsDefaultUiCamera,
        Camera3d::default(),
        Transform::from_translation(CAM_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Minimap: a top-down camera in the bottom-right corner, north up. It also sees
    // layer 1, where the big player and camp markers live.
    commands.spawn((
        Minimap,
        Camera3d::default(),
        Camera { order: 1, clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.06, 0.08)), ..default() },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: MAP_HALF.x * 2.0 + 2.0, height: MAP_HALF.y * 2.0 + 2.0 },
            ..OrthographicProjection::default_3d()
        }),
        RenderLayers::from_layers(&[0, 1]),
        Transform::from_xyz(0.0, 80.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
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
        sup: mats.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.3, 1.0),
            emissive: LinearRgba::rgb(0.9, 0.3, 1.6),
            ..default()
        }),
        monster_mesh: meshes.add(Sphere::new(0.8)),
        monster_mat: [mats.add(Color::srgb(0.55, 0.4, 0.2)), mats.add(Color::srgb(0.6, 0.25, 0.7))],
        marker: meshes.add(Cylinder::new(1.1, 0.1)),
        camp: mats.add(StandardMaterial { base_color: Color::srgb(1.0, 0.85, 0.2), unlit: true, ..default() }),
        wall_mesh: meshes.add(Cuboid::new(0.4, WALL_H, 4.0)),
        wall_mat: TEAM_COLOR.map(|c| {
            mats.add(StandardMaterial { base_color: c.with_alpha(0.45), alpha_mode: AlphaMode::Blend, ..default() })
        }),
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
        TextFont::from_font_size(14.0),
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
                let mat = match c.kind {
                    Kind::Loot => looks.loot.clone(),
                    Kind::Super => looks.sup.clone(),
                    Kind::Normal => looks.normal.clone(),
                };
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
    for m in &mut sim.monsters {
        match m.ent {
            Some(e) => {
                if let Ok((mut tf, mut vis)) = q.get_mut(e) {
                    tf.translation = m.pos + Vec3::Y * 0.8;
                    *vis = if m.alive() { Visibility::Visible } else { Visibility::Hidden };
                }
            }
            None => {
                let big = m.drop == Drop::Super;
                let mat = looks.monster_mat[big as usize].clone();
                m.ent = Some(
                    commands
                        .spawn((
                            Mesh3d(looks.monster_mesh.clone()),
                            MeshMaterial3d(mat),
                            Transform::from_translation(m.pos + Vec3::Y * 0.8).with_scale(Vec3::splat(if big { 1.4 } else { 1.0 })),
                        ))
                        .id(),
                );
            }
        }
    }
    for w in &mut sim.walls {
        if w.ent.is_none() {
            let (m, mat) = (looks.wall_mesh.clone(), looks.wall_mat[w.team].clone());
            w.ent = Some(commands.spawn((Mesh3d(m), MeshMaterial3d(mat), Transform::from_translation(w.pos + Vec3::Y * WALL_H / 2.0))).id());
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
    let off = if sim.map { CAM_OFFSET * 1.25 } else { CAM_OFFSET };
    let want = sim.actors[YOU].pos.with_y(0.0) + off;
    cam.translation = cam.translation.lerp(want, k);
    let look = cam.translation - off;
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
    // Pads: 4 cells, gold when what's on them can be kicked open (pad readout on), the kick
    // progress, and a build's timer and health.
    for (team, pad) in pads().into_iter().enumerate() {
        let shape = sim.pad_state(team).shape;
        let ready = sim.shown && sim.can_open(team, shape);
        let color = if ready { Color::srgb(1.0, 0.8, 0.1) } else { Color::WHITE };
        let iso = Isometry3d::new(pad.extend(0.03).xzy(), Quat::from_rotation_x(FRAC_PI_2));
        g.rect(iso, Vec2::splat(PAD_HALF * 2.0), color);
        let c = pad.extend(0.03).xzy();
        g.line(c - Vec3::X * PAD_HALF, c + Vec3::X * PAD_HALF, color.with_alpha(0.6));
        g.line(c - Vec3::Z * PAD_HALF, c + Vec3::Z * PAD_HALF, color.with_alpha(0.6));
        if sim.pad_kick[team] > 0.0 {
            let k = (sim.pad_kick[team] / KICK_TIME).min(1.0);
            g.circle(ground_circle(c + Vec3::Y * 0.02), PAD_HALF * 1.5 + 0.6 * k, Color::srgb(1.0, 0.7, 0.1));
        }
        if let Some(b) = &sim.builds[team] {
            let done = 1.0 - b.left / b.total;
            g.circle(ground_circle(c + Vec3::Y * 0.05), PAD_HALF * 1.6, Color::srgb(0.3, 1.0, 1.0));
            g.circle(ground_circle(c + Vec3::Y * 0.05), 0.2 + PAD_HALF * 1.4 * done, Color::srgb(0.3, 1.0, 1.0));
            let top = c + Vec3::Y * 2.4 - Vec3::X * 0.8;
            g.line(top, top + Vec3::X * 1.6 * (b.hp / b.max_hp).max(0.0), Color::srgb(1.0, 0.3, 0.3));
        }
    }
    // Hold-to-choose: gold outlines on the crates the held kick would use.
    if sim.hold_kick {
        for team in 0..2 {
            let stages = sim.stages(team);
            if let Some(k) = sim.stage_at(team, stages.len()) {
                let ids = sim.stage_ids(team, stages[k].1);
                for c in sim.crates.iter().filter(|c| ids.contains(&c.id)) {
                    g.cube(Transform::from_translation(c.pos).with_scale(Vec3::splat(CRATE * 1.06)), Color::srgb(1.0, 0.8, 0.1));
                }
            }
        }
    }
    // Locked crates get a dark outline: asleep, untouchable until kicked open.
    for c in sim.crates.iter().filter(|c| c.locked) {
        g.cube(Transform::from_translation(c.pos).with_scale(Vec3::splat(CRATE * 1.02)), Color::srgb(0.1, 0.1, 0.1));
    }
    // Tanks: a hull around the driver, and its health.
    for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive()) {
        if let Some(t) = &b.tank {
            let p = pos_of(i);
            let rot = Transform::IDENTITY.looking_to(b.facing, Vec3::Y).rotation;
            g.cube(Transform::from_translation(p + Vec3::Y * 0.6).with_rotation(rot).with_scale(Vec3::new(1.8, 1.0, 2.4)), TEAM_COLOR[b.team]);
            g.line(p + Vec3::Y * 1.2, p + Vec3::Y * 1.2 + b.facing * 1.8, TEAM_COLOR[b.team]);
            let top = p + Vec3::Y * 2.8 - Vec3::X * 0.6;
            g.line(top, top + Vec3::X * 1.2 * (t.hp / 300.0).min(2.0), Color::srgb(0.3, 0.8, 1.0));
        }
        if let Some(gun) = &b.gun {
            let col = if gun.gold { Color::srgb(1.0, 0.85, 0.2) } else { Color::srgb(0.9, 0.9, 0.9) };
            let p = pos_of(i) + Vec3::Y * CHEST;
            g.line(p, p + b.facing * 1.0, col);
        }
    }
    // Class markers over heads when stacks build by class: soldier ^, engineer square,
    // medic +.
    if sim.book >= BY_CLASS {
        for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive()) {
            let p = pos_of(i) + Vec3::Y * 3.0;
            let col = PLAYER_COLOR[b.slot % PLAYER_COLOR.len()];
            match b.class {
                Class::Soldier => g.linestrip([p - Vec3::X * 0.3, p + Vec3::Y * 0.4, p + Vec3::X * 0.3, p - Vec3::X * 0.3], col),
                Class::Engineer => g.rect(Isometry3d::from_translation(p + Vec3::Y * 0.2), Vec2::splat(0.5), col),
                Class::Medic => {
                    g.line(p - Vec3::X * 0.3 + Vec3::Y * 0.2, p + Vec3::X * 0.3 + Vec3::Y * 0.2, col);
                    g.line(p - Vec3::Y * 0.1, p + Vec3::Y * 0.5, col);
                }
            }
        }
    }
    for m in sim.monsters.iter().filter(|m| m.alive()) {
        if m.hp < m.max {
            let top = m.pos + Vec3::Y * 2.6 - Vec3::X * 0.8;
            g.line(top, top + Vec3::X * 1.6 * (m.hp / m.max).max(0.0), Color::srgb(1.0, 0.85, 0.2));
        }
        if m.target.is_some() {
            g.circle(ground_circle(m.home.with_y(0.04)), LEASH, Color::srgba(1.0, 0.85, 0.2, 0.25));
        }
    }
    for p in &sim.pickups {
        let col = if p.drop == Drop::Heal { Color::srgb(0.2, 1.0, 0.3) } else { Color::srgb(0.3, 0.8, 1.0) };
        let bob = p.pos + Vec3::Y * (0.2 * (sim.t * 3.0).sin());
        g.sphere(Isometry3d::from_translation(bob), 0.35, col);
        g.circle(ground_circle(p.pos.with_y(0.04)), 0.6, col);
    }
    for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive() && b.shield > 0.0) {
        let top = pos_of(i) + Vec3::Y * 2.62 - Vec3::X * 0.4;
        g.line(top, top + Vec3::X * 0.8 * (b.shield / 100.0), Color::srgb(0.3, 0.8, 1.0));
    }
    for (at, r, left) in &sim.booms {
        g.sphere(Isometry3d::from_translation(*at), r * (1.0 - left / 0.3 * 0.5), Color::srgb(1.0, 0.6, 0.1));
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
    s += &format!(
        "players: BLUE {}/{n}  RED {}/{n}   crates knocked off pads: blue {} red {}   builds broken: blue {} red {}\n",
        sim.players(0),
        sim.players(1),
        sim.lost[0],
        sim.lost[1],
        sim.broken[0],
        sim.broken[1],
        n = sim.team_size,
    );
    s += &format!("your pad: {}\n", pad_readout(&sim, 0));
    s += &format!("red pad:  {}\n", pad_readout(&sim, 1).replace("   HOLD F BESIDE THE PAD TO KICK IT OPEN", ""));
    if tune.c(P::Seat) == 1 {
        s += "your seat is a BOT: watching\n";
    } else {
        s += &format!(
            "you ({}): {}{}  hp {:.0}{}{}\n",
            a.class.name(),
            if a.carrying.is_some() { "CARRYING" } else { "empty-handed" },
            if a.grounded { "" } else { ", airborne" },
            a.hp.max(0.0),
            if a.shield > 0.0 { format!("  shield {:.0}", a.shield) } else { String::new() },
            if a.alive() { String::new() } else { format!("  DOWN {:.1}s", a.dead) },
        );
        if let Some(t) = &a.tank {
            s += &format!("IN A TANK: hp {:.0}, {:.0} s left, cannon only, can't carry\n", t.hp, t.life);
        } else if let Some(gun) = &a.gun {
            s += &format!("weapon: {}{} ({} shots left)\n", if gun.gold { "SUPER " } else { "" }, Make::Gun(gun.kind).name(), gun.ammo);
        }
        if let Some(alt) = &a.alt {
            s += &format!("right click (empty hands): {}{} ({} left)\n", if alt.gold { "SUPER " } else { "" }, Make::Alt(alt.kind).name(), alt.charges);
        }
        if a.carrying.is_some() {
            let boost = throw_plan(&sim, controls.aim_dist).boost;
            s += &format!("throw reach: {:.1} m  (jump x{boost:.2})\n", THROW_DIST * boost);
        }
    }
    if sim.map {
        let camp = |m: &Monster| {
            let name = match (m.drop, m.home.z > 0.0, m.home.x < 0.0) {
                (Drop::Super, true, _) => "river N (super crate)".to_string(),
                (Drop::Super, false, _) => "river S (super crate)".to_string(),
                (d, _, west) => format!("{} {}", if west { "blue-side" } else { "red-side" }, if d == Drop::Heal { "heal" } else { "shield" }),
            };
            if m.alive() { format!("{name} up") } else { format!("{name} {:.0}s", m.respawn) }
        };
        s += &format!("camps: {}\n", sim.monsters.iter().map(camp).collect::<Vec<_>>().join(" | "));
    }
    let bodies = sim.players(0) + sim.players(1);
    let moving = sim.crates.iter().filter(|c| c.state != CrateState::Free || c.vel.length() > 0.05).count();
    let asleep = sim.crates.iter().filter(|c| c.locked).count();
    s += &format!(
        "netcode: {bodies} networked characters (budget ~32), {moving} moving crates (~20-30), {asleep} locked into pads (asleep, sent once)\n"
    );
    let acc = if sim.fired > 0 { 100.0 * sim.hits as f32 / sim.fired as f32 } else { 0.0 };
    s += &format!("your shots {} hits {} ({acc:.0}%)\n", sim.fired, sim.hits);
    for (m, _) in &sim.msgs {
        s += &format!("> {m}\n");
    }
    s += "\nWASD move  mouse aim  Space jump  E grab/place  RMB throw (or right-click item)  LMB shoot  F kick\n";
    s += "1-7 change a rule  C your class  ENTER pick these rules  T drop crate  Y drop super crate  R reset  P print  H hide boxes";
    hud.0 = s;
}

/// The first player on `team` (its starting player).
fn team_lead(sim: &Sim, team: usize) -> usize {
    (0..sim.actors.len()).find(|&j| sim.actors[j].team == team).unwrap_or(YOU)
}

/// What a pad says about itself: everything with the readout on, only the crate count
/// when it's secret.
fn pad_readout(sim: &Sim, team: usize) -> String {
    let st = sim.pad_state(team);
    if let Some(b) = &sim.builds[team] {
        return format!("BUILDING {}{} ({:.1} s left, hp {:.0}/{:.0})", if b.gold { "SUPER " } else { "" }, b.make.name(), b.left.max(0.0), b.hp, b.max_hp);
    }
    let n = st.ids.len();
    if sim.hold_kick && !matches!(st.shape, Shape::Empty | Shape::Unaligned) {
        let stages = sim.stages(team);
        if stages.is_empty() {
            return format!("{n} crate{}: no recipe inside yet, add crates", if n == 1 { "" } else { "s" });
        }
        let at = sim.stage_at(team, stages.len());
        let kicker = if team == 0 { YOU } else { team_lead(sim, 1) };
        let each: Vec<String> = stages
            .iter()
            .enumerate()
            .map(|(k, &(slot, take))| {
                let used: u8 = take.iter().sum();
                let what = if sim.shown {
                    let ids = sim.stage_ids(team, take);
                    let supers = sim.crates.iter().filter(|c| ids.contains(&c.id) && c.kind == Kind::Super).count();
                    let (make, gold) = sim.make_for(slot, supers, sim.actors[kicker].class);
                    format!("{} -> {}{}", slot.label(sim.count_only), if gold { "SUPER " } else { "" }, make.name())
                } else {
                    format!("{used} crate{}: ?", if used == 1 { "" } else { "s" })
                };
                if at == Some(k) { format!("[{what}]") } else { what }
            })
            .collect();
        let tip = if at.is_some() { "   RELEASE F TO OPEN" } else { "   HOLD F: LONGER = MORE CRATES" };
        return format!("{}{tip}", each.join(" > "));
    }
    let kick = "   HOLD F BESIDE THE PAD TO KICK IT OPEN";
    if !sim.shown {
        let sup = if st.supers > 0 { " (1 super)" } else { "" };
        return match st.shape {
            Shape::Empty => "empty".into(),
            Shape::Unaligned => format!("{n} crates, one out of line{kick}"),
            _ => format!("{n} crate{}{sup}: what will it make?{kick}", if n == 1 { "" } else { "s" }),
        };
    }
    match st.shape {
        Shape::Empty => "empty".into(),
        Shape::Odd => format!("{n} crates: no recipe yet, add crates"),
        Shape::Unaligned => format!("a crate is out of line: kicking breaks them for nothing{kick}"),
        Shape::Busy => "building".into(),
        Shape::Recipe(slot) if sim.book >= BY_CLASS && st.shape.players() == 0 => {
            // One shape, a result per class: whoever kicks it decides.
            let each: Vec<String> = CLASSES
                .iter()
                .map(|&c| {
                    let (make, gold) = sim.pad_make_as(team, c).unwrap_or((Make::Health, false));
                    let you = if team == 0 && c == sim.actors[YOU].class { " (you)" } else { "" };
                    format!("{}{you}: {}{}", c.name(), if gold { "SUPER " } else { "" }, make.name())
                })
                .collect();
            format!("{} -> {}{kick}", slot.label(sim.count_only), each.join(" | "))
        }
        Shape::Recipe(slot) => {
            let (make, gold) = sim.pad_make(team, if team == 0 { YOU } else { team_lead(sim, 1) }).unwrap_or((Make::Health, false));
            let full = st.shape.players() > 0 && !sim.can_open(team, st.shape);
            let branched = st.supers > 0 && sim.super_mode == 2 && !matches!(make, Make::Players(_));
            format!(
                "{} -> {}{}{}{}",
                slot.label(sim.count_only),
                if gold { "SUPER " } else { "" },
                make.name(),
                if branched { " (branched by the super crate)" } else { "" },
                if full { "   (team full)" } else { kick },
            )
        }
    }
}

fn guide_text(
    tune: Res<Tune>,
    sim: Res<Sim>,
    mut guide: Single<(&mut Text, &mut Visibility), (With<GuideText>, Without<Hud>, Without<Panel>)>,
) {
    let (text, vis) = &mut *guide;
    **vis = if tune.panel { Visibility::Visible } else { Visibility::Hidden };
    let mut s = "RULES YOU'RE PLAYING (press the number to change one; restarts the match)\n".to_string();
    for (k, p) in tune.vals[..RULES].iter().enumerate() {
        let opts = match &p.value {
            Value::Choice { i, opts } => opts
                .iter()
                .enumerate()
                .map(|(j, o)| if j == *i { format!("[{o}]") } else { o.to_string() })
                .collect::<Vec<_>>()
                .join("  "),
            Value::Num { v, .. } => BUILD_TIMES
                .iter()
                .map(|b| {
                    let o = if *b == 0.0 { "instant".to_string() } else { format!("{b:.0} s per crate") };
                    if (b - v).abs() < 0.01 { format!("[{o}]") } else { o }
                })
                .collect::<Vec<_>>()
                .join("  "),
        };
        s += &format!(" {} {:<14} {opts}\n", k + 1, &p.name[2..]);
    }
    // The recipe book: always listed (it's the manual); the pad readout is the variable.
    let co = sim.count_only;
    let line = |b: [Make; 4]| {
        format!(
            "{} = {}, {} = {}, {} = {}, {} = {}",
            Slot::Small.label(co),
            b[0].name(),
            Slot::Pair.label(co),
            b[1].name(),
            Slot::Flat4.label(co),
            b[2].name(),
            Slot::All8.label(co),
            b[3].name(),
        )
    };
    s += &format!("\nRECIPES: {} = 1 player, {} = 2 players", Slot::Player1.label(co), Slot::Player2.label(co));
    if sim.book >= BY_CLASS {
        s += &format!(", the rest by the KICKER'S CLASS (you: {}, C changes it):", sim.actors[YOU].class.name().to_uppercase());
        for c in CLASSES {
            s += &format!("\n  {:<8} {}", c.name(), line(BOOKS[c.book(sim.book == BY_LOADOUT)].1));
        }
    } else {
        s += &format!(", {}", line(BOOKS[sim.book].1));
    }
    match sim.super_mode {
        1 => s += "\nSUPER crate (purple) in a recipe: a stronger version of it",
        2 if sim.book >= BY_CLASS => s += "\nSUPER crate (purple) in a recipe: makes the next class's result (soldier > engineer > medic > soldier)",
        2 => s += &format!("\nSUPER crate (purple) in a recipe: makes the {} book's result instead", BOOKS[branch_of(sim.book)].0),
        _ => {}
    }
    s += "\nENTER = pick these rules";
    if !tune.verdicts.is_empty() {
        s += &format!(" ({} picked so far, P prints them)", tune.verdicts.len());
    }
    text.0 = s;
}

fn panel(tune: Res<Tune>, mut panel: Single<(&mut Text, &mut Visibility), (With<Panel>, Without<Hud>)>) {
    let (text, vis) = &mut *panel;
    **vis = if tune.panel { Visibility::Visible } else { Visibility::Hidden };
    let mut s = "TUNING (core feel #9 and pad joins #10 fixed)\n\n".to_string();
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
    let Some(secs) = std::env::var("CRATE_EXIT_AFTER").ok().and_then(|s| s.parse::<f32>().ok()) else {
        return;
    };
    let deadline = *smoke.exit_at.get_or_insert(Duration::from_secs_f32(secs));
    if let Ok(path) = std::env::var("CRATE_SCREENSHOT")
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
                title: "big_back arena map (PROTOTYPE)".into(),
                // Smoke runs often land on a hidden workspace, where vsync blocks forever.
                present_mode: if std::env::var("CRATE_EXIT_AFTER").is_ok() {
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
        .init_resource::<MapView>()
        .add_systems(Startup, (setup, init_from_env).chain())
        .add_systems(
            Update,
            (read_controls, tune_input, step, sync_arena, sync_map, sync_views, sync_markers, minimap_view, follow_camera, draw_gizmos, hud, guide_text, panel, smoke_exit)
                .chain(),
        )
        .run();
}
