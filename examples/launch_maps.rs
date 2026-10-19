//! PROTOTYPE — throwaway. Answers #42: Season 1 has one mode (ADR 0014), so the maps carry
//! the variety. How many maps ship, and what are they: ADR 0006's lanes + jungle map alone,
//! plus its ~60 x 34 m smaller version (ADR 0012's 3v3 fallback), or distinct layouts? Mirror
//! or rotational symmetry? How does the queue pick among them? Each new map is weighed
//! against the work of tuning bots and the comeback set for it.
//! Built on `prototype/latency` (#36, its verdict fixed: server grabs judged on what you saw,
//! at 0 ms here so maps compare clean). Not production code; lives on `prototype/launch-maps`.
//!
//!     cargo run --example launch_maps
//!
//! See `docs/prototypes/launch-maps.md` for what to try.
//!
//! Every map is drawn as blue's half and turned into red's by the symmetry switch: mirrored
//! east-west, or rotated 180 degrees. The small map is the full one, scaled.
//!
//! Play: WASD / left stick move, mouse / right stick aim, Space / A jump, E / X grab or place,
//! RMB / RB throw (empty hands: right-click item), LMB / RT shoot, Q / LB throw your 1-crate
//! item, F / B kick (hold beside your pad, let go to open; tap beside a loose crate to punt
//! it). Shoot the red base (the block behind red's pad) to attack its production.
//! Map switch: 1 map, 2 symmetry (both restart), 3 how the queue picks the next map, 4 the
//! queue's pool (both act when a match ends), 5 how many per team on the small map.
//! C changes your class. T drops a crate at your feet, Y a super crate, U a megacrate. K
//! knocks red's base down to 1 hp (Shift+K: yours), to try a base kill quickly. V gives you
//! your class's all 8 now (Shift+V a super one).
//! Tune: Up/Down pick, Left/Right change (Shift = x5); every number is in the panel.
//! R / Start resets, Enter picks the current map, P prints settings, picks and results,
//! H hides the rules and panel.
//!
//! Smoke tests: `CRATE_EXIT_AFTER=<s>`, `CRATE_SCREENSHOT=<file.png>`, `CRATE_SEAT=bot`,
//! `CRATE_RULES=<5 digits>` (the option index of each map switch), `CRATE_MAP=<0-3|open>`,
//! `CRATE_SYM=<0|1>` (1: rotational), `CRATE_ARENA=1` (whole-arena camera),
//! `CRATE_SPEED=<1-10>` (time scale), `CRATE_CLASS=<0-2>` (your class),
//! `CRATE_MATCHES=<n>` (exit after n matches), `CRATE_LENGTH=<s>` (match length),
//! `CRATE_SEED=<n>` (bot-match randomness), `CRATE_HEADLESS=1` (no window or rendering, sim
//! only). Each match prints a `maps` line: the map's fights, hauls, closeness and bots.

use std::collections::VecDeque;
use std::f32::consts::FRAC_PI_2;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
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
    let a = if map_on() { layout().half } else { BASE_ARENA * k };
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
// #42: the launch maps. Each is drawn as blue's half (x <= 0) and turned into red's by the
// symmetry switch: mirrored east-west, or rotated 180 degrees about the centre. Walls are
// tall: players, crates and shots don't pass. The lanes + jungle map is #20's (ADR 0006).
// ---------------------------------------------------------------------------------------

/// Whether a walled map is on (else the open #10 arena). A static so the free physics
/// functions can read it.
static MAP_ON: AtomicBool = AtomicBool::new(false);
/// Which layout is on: map index x 2 + symmetry (see `layout`).
static LAYOUT_KEY: AtomicUsize = AtomicUsize::new(0);
const BLOCK_H: f32 = 2.6;

/// Key 1. The last is the open #10 arena, kept to compare.
const MAP_NAMES: [&str; 6] = [
    "lanes + jungle (90 x 50 m, ADR 0006)",
    "small lanes + jungle (60 x 34 m)",
    "crossroads (72 x 44 m): 2 lanes, a jungle on one flank",
    "canal (76 x 40 m): 3 lanes, 3 bridges, no jungle",
    "big lanes + jungle (120 x 72 m): League scale, big bases",
    "open arena (#10)",
];
/// Short names for the HUD and the result lines.
const MAP_SHORT: [&str; 6] = ["full", "small", "crossroads", "canal", "big", "open"];
const MAPS: usize = 5;
const OPEN_MAP: usize = 5;
const SMALL_MAP: usize = 1;
const BIG_MAP: usize = 4;
/// Key 4: which maps the queue picks from.
const POOL_NAMES: [&str; 6] = ["full only", "full + small", "full + small + crossroads", "full + small + crossroads + canal", "big only", "big + full"];
const POOLS: [&[usize]; 6] = [&[0], &[0, 1], &[0, 1, 2], &[0, 1, 2, 3], &[BIG_MAP], &[BIG_MAP, 0]];

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

/// How a floor patch is drawn.
#[derive(Clone, Copy, PartialEq)]
enum Floor {
    Jungle,
    Lane,
    /// The river (lanes maps), the plaza (crossroads), a bridge (canal).
    Middle,
    Field,
    Water,
}

/// A whole map, both halves.
struct Layout {
    half: Vec2,
    blocks: Vec<Block>,
    /// Which blocks are canal (drawn as water) rather than walls.
    water: Vec<bool>,
    floors: Vec<(Block, Floor)>,
    crates: Vec<Vec2>,
    loot: Vec<Vec2>,
    camps: Vec<(Vec2, Drop)>,
    /// Lane chokes and entrances a gate can shut: (centre, the way it faces).
    chokes: Vec<(Vec2, Vec2)>,
    /// New crates land within this share of the half-width of the centre line.
    mid_span: f32,
    /// What the HUD calls the middle (camps there are "river objectives").
    middle: &'static str,
    /// How deep each base's tinted floor is, from the end wall.
    base: f32,
}

/// Blue's half of a map, in blue's coordinates (x <= 0). Rects are (x0, x1, z0, z1).
#[derive(Default)]
struct HalfSpec {
    walls: Vec<(f32, f32, f32, f32)>,
    water: Vec<(f32, f32, f32, f32)>,
    floors: Vec<((f32, f32, f32, f32), Floor)>,
    crates: Vec<(f32, f32)>,
    loot: Vec<(f32, f32)>,
    camps: Vec<((f32, f32), Drop)>,
    chokes: Vec<((f32, f32), Vec2)>,
}

fn rect(r: (f32, f32, f32, f32), k: Vec2) -> Block {
    let (a, b) = (Vec2::new(r.0 * k.x, r.2 * k.y), Vec2::new(r.1 * k.x, r.3 * k.y));
    Block { min: a.min(b), max: a.max(b) }
}

/// Blue's half plus red's: mirrored (x -> -x) or rotated (x, z -> -x, -z). `k` scales the
/// spec (the small map is the full one, scaled). Points that land on each other are kept once.
fn finish(spec: HalfSpec, half: Vec2, k: Vec2, rotate: bool, mid_span: f32, middle: &'static str) -> Layout {
    let flip = |p: Vec2| if rotate { -p } else { Vec2::new(-p.x, p.y) };
    let flip_block = |b: Block| {
        let (a, c) = (flip(b.min), flip(b.max));
        Block { min: a.min(c), max: a.max(c) }
    };
    let both = |pts: Vec<Vec2>| {
        let mut out: Vec<Vec2> = Vec::new();
        for p in pts.iter().copied().chain(pts.iter().map(|&p| flip(p))) {
            if !out.iter().any(|q| q.distance(p) < 0.5) {
                out.push(p);
            }
        }
        out
    };
    let pt = |(x, z): (f32, f32)| Vec2::new(x * k.x, z * k.y);
    let mut blocks = Vec::new();
    let mut water = Vec::new();
    for (list, wet) in [(&spec.walls, false), (&spec.water, true)] {
        for &r in list {
            let b = rect(r, k);
            blocks.extend([b, flip_block(b)]);
            water.extend([wet, wet]);
        }
    }
    let floors = spec.floors.iter().flat_map(|&(r, f)| {
        let b = rect(r, k);
        [(b, f), (flip_block(b), f)]
    });
    let mut camps: Vec<(Vec2, Drop)> = Vec::new();
    for &(p, d) in &spec.camps {
        for q in [pt(p), flip(pt(p))] {
            if !camps.iter().any(|c| c.0.distance(q) < 0.5) {
                camps.push((q, d));
            }
        }
    }
    let mut chokes: Vec<(Vec2, Vec2)> = Vec::new();
    for &(p, n) in &spec.chokes {
        for q in [pt(p), flip(pt(p))] {
            if !chokes.iter().any(|c| c.0.distance(q) < 0.5) {
                chokes.push((q, n));
            }
        }
    }
    Layout {
        half,
        blocks,
        water,
        floors: floors.collect(),
        crates: both(spec.crates.iter().map(|&p| pt(p)).collect()),
        loot: both(spec.loot.iter().map(|&p| pt(p)).collect()),
        camps,
        chokes,
        mid_span,
        middle,
        base: 14.0,
    }
}

/// #20's lanes + jungle map, blue's half: bases at each end, three lanes, two walled jungle
/// quadrants (heal camp north, shield camp south) and the river, with an objective north
/// and south.
fn lanes_spec() -> HalfSpec {
    let mut s = HalfSpec::default();
    // One quadrant's walls in -x, +z (x0, x1, z0, z1), mirrored north-south.
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
    for sz in [-1.0f32, 1.0] {
        for (x0, x1, z0, z1) in walls {
            s.walls.push((-x0, -x1, z0 * sz, z1 * sz));
        }
        s.floors.push(((-4.0, -31.0, 4.5 * sz, 17.5 * sz), Floor::Jungle));
        s.floors.push(((0.0, -31.0, 18.0 * sz, 25.0 * sz), Floor::Lane));
        s.chokes.push(((-26.0, 21.5 * sz), Vec2::X));
        s.chokes.push(((-16.0, 4.5 * sz), Vec2::Y));
        s.chokes.push(((-22.0, 17.5 * sz), Vec2::Y));
        s.chokes.push(((-30.5, 11.0 * sz), Vec2::X));
        for x in [-24.0, -12.0, 0.0] {
            s.crates.push((x, 21.6 * sz));
        }
        s.crates.push((-14.0, 13.0 * sz));
        s.crates.push((-19.0, 8.0 * sz));
        s.crates.push((0.0, 6.0 * sz));
        s.loot.push((0.0, 12.0 * sz));
        s.camps.push(((0.0, 15.0 * sz), Drop::Super));
    }
    s.floors.push(((0.0, -31.0, -4.0, 4.0), Floor::Lane));
    s.floors.push(((0.0, -4.0, -25.0, 25.0), Floor::Middle));
    s.chokes.push(((-26.0, 0.0), Vec2::X));
    s.crates.extend([(-22.0, 0.0), (-11.0, 0.0), (-1.6, 1.6)]);
    s.camps.push(((-17.0, 11.0), Drop::Heal));
    s.camps.push(((-17.0, -11.0), Drop::Shield));
    s
}

/// Crossroads, blue's half: no mid jungle. A walled jungle on blue's north flank (heal camp),
/// an open field with scattered cover on its south flank (shield camp), a wide mid lane with
/// a pillar at the centre, and a plaza between them. Rotated, red's jungle is south: each
/// side lane has a jungle on one end and a field on the other.
fn crossroads_spec() -> HalfSpec {
    let mut s = HalfSpec::default();
    s.walls = vec![
        // The jungle: south wall (entrance x -18..-14), north wall (entrance x -12..-8),
        // base wall (entrance z 11..14), a stub on the plaza side, a pillar inside.
        (-26.0, -18.0, 7.0, 8.0),
        (-14.0, -6.0, 7.0, 8.0),
        (-26.0, -12.0, 17.0, 18.0),
        (-8.0, -6.0, 17.0, 18.0),
        (-27.0, -26.0, 7.0, 11.0),
        (-27.0, -26.0, 14.0, 18.0),
        (-7.0, -6.0, 8.0, 11.0),
        (-19.0, -16.0, 11.5, 13.5),
        // The field: low cover.
        (-22.0, -19.0, -12.0, -10.0),
        (-14.0, -12.0, -18.0, -15.0),
        (-10.0, -7.0, -12.0, -10.0),
        (-26.0, -25.0, -20.0, -16.0),
        // Mid lane cover, and the centre pillar.
        (-13.0, -11.0, 3.0, 5.0),
        (-1.5, 0.0, -1.5, 1.5),
    ];
    s.floors = vec![
        ((-27.0, 0.0, 18.0, 22.0), Floor::Lane),
        ((-26.0, -6.0, 8.0, 17.0), Floor::Jungle),
        ((-27.0, 0.0, -7.0, 7.0), Floor::Lane),
        ((-27.0, 0.0, -22.0, -7.0), Floor::Field),
        ((-6.0, 0.0, -7.0, 18.0), Floor::Middle),
    ];
    s.crates = vec![
        (-20.0, 20.0),
        (-8.0, 20.0),
        (-22.0, 0.0),
        (-11.0, -3.0),
        (-16.0, -18.0),
        (-6.0, -17.0),
        (-20.0, -5.0),
        (-14.0, 10.0),
        (-3.0, 4.0),
        (-3.0, 12.0),
    ];
    s.loot = vec![(0.0, 9.0)];
    s.camps = vec![((-11.0, 12.5), Drop::Heal), ((-17.0, -15.0), Drop::Shield), ((0.0, 13.0), Drop::Super), ((0.0, -13.0), Drop::Super)];
    s.chokes = vec![
        ((-26.0, 0.0), Vec2::X),
        ((-26.0, 20.0), Vec2::X),
        ((-26.0, -14.0), Vec2::X),
        ((-16.0, 7.5), Vec2::Y),
        ((-10.0, 17.5), Vec2::Y),
        ((-26.5, 12.5), Vec2::X),
    ];
    s
}

/// Canal, blue's half: a canal down the middle with three bridges (the objectives sit on
/// the side bridges), and two long walls splitting each half into three lanes that merge at
/// the base and at the canal. No jungle: the heal and shield camps sit in the side lanes.
fn canal_spec() -> HalfSpec {
    let mut s = HalfSpec::default();
    s.water = vec![(-2.5, 0.0, -20.0, -14.0), (-2.5, 0.0, -10.0, -2.0), (-2.5, 0.0, 2.0, 10.0), (-2.5, 0.0, 14.0, 20.0)];
    s.walls = vec![
        (-24.0, -8.0, 6.0, 7.0),
        (-24.0, -8.0, -7.0, -6.0),
        (-18.0, -16.0, 12.5, 15.5),
        (-12.0, -10.0, -16.0, -13.0),
        (-15.0, -13.0, -2.0, 2.0),
    ];
    s.floors = vec![
        ((-27.0, -2.5, -20.0, 20.0), Floor::Lane),
        ((-2.5, 0.0, -20.0, 20.0), Floor::Water),
        ((-2.5, 0.0, -14.0, -10.0), Floor::Middle),
        ((-2.5, 0.0, -2.0, 2.0), Floor::Middle),
        ((-2.5, 0.0, 10.0, 14.0), Floor::Middle),
    ];
    s.crates = vec![
        (-20.0, 10.0),
        (-8.0, 16.0),
        (-20.0, -10.0),
        (-8.0, -17.0),
        (-20.0, 3.0),
        (-8.0, 0.0),
        (-5.0, 8.0),
        (-5.0, -8.0),
        (-26.0, 12.0),
        (-26.0, -12.0),
        (0.0, 0.0),
    ];
    s.loot = vec![(-5.0, 3.0)];
    s.camps = vec![((-12.0, 13.0), Drop::Heal), ((-18.0, -13.0), Drop::Shield), ((0.0, 12.0), Drop::Super), ((0.0, -12.0), Drop::Super)];
    s.chokes = vec![
        ((0.0, 12.0), Vec2::X),
        ((0.0, 0.0), Vec2::X),
        ((0.0, -12.0), Vec2::X),
        ((-26.0, 13.0), Vec2::X),
        ((-26.0, 0.0), Vec2::X),
        ((-26.0, -13.0), Vec2::X),
    ];
    s
}

/// Big lanes + jungle, blue's half: League scale. A walled base 20 m deep with exits onto the
/// three lanes and a gate into each jungle; two big jungle quadrants (35 x 23 m, against #20's
/// 27 x 13) split into rooms by inner walls, each with a heal and a shield camp; a 10 m river
/// with an objective north and south.
fn big_spec() -> HalfSpec {
    let mut s = HalfSpec::default();
    for sz in [-1.0f32, 1.0] {
        let z = |v: f32| v * sz;
        for (x0, x1, z0, z1) in [
            // The base wall: lane exits at z 29-36 and -5..5, a jungle gate at z 14-18.
            (-41.0, -40.0, 5.0, 14.0),
            (-41.0, -40.0, 18.0, 29.0),
            // Jungle, facing mid lane: entrances at x -26..-22 and -12..-8.
            (-40.0, -26.0, 5.0, 6.0),
            (-22.0, -12.0, 5.0, 6.0),
            (-8.0, -5.0, 5.0, 6.0),
            // Facing the side lane: entrances at x -30..-26 and -14..-10.
            (-40.0, -30.0, 28.0, 29.0),
            (-26.0, -14.0, 28.0, 29.0),
            (-10.0, -5.0, 28.0, 29.0),
            // Facing the river: an entrance at z 12-17.
            (-6.0, -5.0, 6.0, 12.0),
            (-6.0, -5.0, 17.0, 28.0),
            // Rooms: a long inner wall, a ledge, and pillars.
            (-30.0, -28.0, 10.0, 22.0),
            (-20.0, -12.0, 16.0, 18.0),
            (-36.0, -33.0, 21.0, 24.0),
            (-16.0, -14.0, 9.0, 12.0),
            (-24.0, -21.0, 23.0, 25.0),
        ] {
            s.walls.push((x0, x1, z(z0), z(z1)));
        }
        s.floors.push(((-40.0, -5.0, z(6.0), z(28.0)), Floor::Jungle));
        s.floors.push(((-40.0, 0.0, z(29.0), z(36.0)), Floor::Lane));
        // Heal camps in the north rooms' far corners, shields in the near ones (swapped south).
        let (far, near) = if sz > 0.0 { (Drop::Heal, Drop::Shield) } else { (Drop::Shield, Drop::Heal) };
        s.camps.push(((-35.0, z(14.0)), far));
        s.camps.push(((-17.0, z(23.0)), near));
        s.camps.push(((0.0, z(18.0)), Drop::Super));
        s.chokes.extend([
            ((-40.0, z(32.5)), Vec2::X),
            ((-40.5, z(16.0)), Vec2::X),
            ((-24.0, z(5.5)), Vec2::Y),
            ((-10.0, z(5.5)), Vec2::Y),
            ((-28.0, z(28.5)), Vec2::Y),
            ((-12.0, z(28.5)), Vec2::Y),
            ((-5.5, z(14.5)), Vec2::X),
        ]);
        for x in [-32.0, -20.0, -8.0] {
            s.crates.push((x, z(32.5)));
        }
        s.crates.extend([(-24.0, z(12.0)), (-12.0, z(22.0)), (-36.0, z(9.0)), (0.0, z(8.0)), (-2.0, z(28.0))]);
        s.loot.push((0.0, z(11.0)));
    }
    s.floors.push(((-40.0, 0.0, -5.0, 5.0), Floor::Lane));
    s.floors.push(((-5.0, 0.0, -36.0, 36.0), Floor::Middle));
    s.chokes.push(((-40.0, 0.0), Vec2::X));
    s.crates.extend([(-30.0, 0.0), (-18.0, 0.0), (-8.0, 0.0)]);
    s
}

fn build_layout(map: usize, rotate: bool) -> Layout {
    match map {
        0 => finish(lanes_spec(), Vec2::new(45.0, 25.0), Vec2::ONE, rotate, 0.25, "river"),
        // ADR 0006's smaller version: the same layout scaled to about 60 x 34 m.
        1 => finish(lanes_spec(), Vec2::new(30.0, 17.0), Vec2::new(30.0 / 45.0, 17.0 / 25.0), rotate, 0.25, "river"),
        2 => finish(crossroads_spec(), Vec2::new(36.0, 22.0), Vec2::ONE, rotate, 0.25, "plaza"),
        3 => finish(canal_spec(), Vec2::new(38.0, 20.0), Vec2::ONE, rotate, 0.3, "bridges"),
        _ => Layout { base: 20.0, ..finish(big_spec(), Vec2::new(60.0, 36.0), Vec2::ONE, rotate, 0.25, "river") },
    }
}

fn layout() -> &'static Layout {
    static L: [OnceLock<Layout>; MAPS * 2] = [const { OnceLock::new() }; MAPS * 2];
    let key = LAYOUT_KEY.load(Ordering::Relaxed);
    L[key].get_or_init(|| build_layout(key / 2, key % 2 == 1))
}

/// Puts a map on (or the open arena, `OPEN_MAP`).
fn set_layout(map: usize, rotate: bool) {
    MAP_ON.store(map < MAPS, Ordering::Relaxed);
    if map < MAPS {
        LAYOUT_KEY.store(map * 2 + rotate as usize, Ordering::Relaxed);
    }
}

fn blocks() -> &'static [Block] {
    if !map_on() {
        return &[];
    }
    &layout().blocks
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

/// Navigation grid for bots: a node every 2 m off the walls, linked to its 8 neighbours
/// when the way is clear. One per layout.
struct Nav {
    nodes: Vec<Vec2>,
    links: Vec<Vec<(usize, f32)>>,
}

fn nav() -> &'static Nav {
    static N: [OnceLock<Nav>; MAPS * 2] = [const { OnceLock::new() }; MAPS * 2];
    N[LAYOUT_KEY.load(Ordering::Relaxed)].get_or_init(|| {
        let half = layout().half;
        let walls = &layout().blocks;
        // The small map's lanes and entrances are narrower: a finer grid.
        let step = if half.x < 40.0 { 2.0 } else { 2.5 };
        let mut nodes = Vec::new();
        let (nx, nz) = ((half.x * 2.0 / step) as i32, (half.y * 2.0 / step) as i32);
        let mut index = std::collections::HashMap::new();
        for i in 0..=nx {
            for j in 0..=nz {
                let p = Vec2::new(-half.x + 1.0 + i as f32 * step * 0.98, -half.y + 1.0 + j as f32 * step * 0.96);
                if !walls.iter().any(|b| b.hits(p, 0.8)) {
                    index.insert((i, j), nodes.len());
                    nodes.push(p);
                }
            }
        }
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
    /// A left-click item (1 x 2): replaces the gun until its ammo runs out.
    Gun(GunKind),
    /// A right-click item (2 x 2 flat): used with empty hands, by charges.
    Alt(AltKind),
    /// 1 crate, Set A: a thrown item, kept until you throw it with Q.
    Toss(TossKind),
    /// 1 crate, Set A engineer: +1 right-click charge, or hp back on your nearest wall.
    PatchKit,
    /// 1 crate, Set B: tops up what you hold (left click to full, right click +1), else +25 hp.
    Refill,
    /// All 8, Set A and Set B soldier.
    Tank,
    /// All 8, Set B engineer: a vehicle that carries several crates, with no weapon.
    Mech,
    /// All 8, Set B medic: a vehicle that heals allies close by (an aura).
    Ambulance,
    /// All 8, Set B variant: the ambulance with a heal shot its driver fires instead.
    AmbulanceShot,
    /// All 8, Set A engineer: a horseshoe of walls round your pad, with one gap.
    Bunker,
    /// All 8, Set A medic: every living teammate to full health with a shield.
    Rally,
    /// 1 crate, Set C: a class self-buff (or the engineer's barricade), used with Q.
    Kit(KitKind),
    /// All 8, Set C soldier: mark a circle; shells fall on it after a delay. `true` is the
    /// sparser variant.
    Barrage(bool),
    /// All 8, Set C engineer: two long walls your team passes and enemies don't.
    Gate,
}

impl Make {
    fn name(self) -> &'static str {
        match self {
            Make::Players(1) => "1 new player",
            Make::Players(_) => "2 new players",
            Make::Gun(GunKind::Rapid) => "rapid gun (LMB)",
            Make::Gun(GunKind::Rivet) => "rivet gun (LMB)",
            Make::Gun(GunKind::Heal) => "heal gun (LMB)",
            Make::Gun(GunKind::Shotgun) => "shotgun (LMB)",
            Make::Gun(GunKind::Mines) => "nail-mine layer (LMB)",
            Make::Alt(AltKind::Rockets) => "rockets (RMB)",
            Make::Alt(AltKind::ShieldWall) => "shield wall (RMB)",
            Make::Alt(AltKind::Pulse) => "heal pulse (RMB)",
            Make::Toss(TossKind::Grenade) => "grenade (Q)",
            Make::Toss(TossKind::MedPack) => "med pack (Q)",
            Make::PatchKit => "patch kit",
            Make::Refill => "refill",
            Make::Tank => "tank",
            Make::Mech => "hauler mech",
            Make::Ambulance => "ambulance (heal aura)",
            Make::AmbulanceShot => "ambulance (heal shot)",
            Make::Bunker => "bunker",
            Make::Rally => "rally",
            Make::Alt(AltKind::Launcher) => "grenade launcher (RMB)",
            Make::Alt(AltKind::ShieldBurst) => "shield burst (RMB)",
            Make::Toss(_) => "grenade",
            Make::Kit(KitKind::Adrenaline) => "adrenaline (Q)",
            Make::Kit(KitKind::Barricade) => "barricade (Q)",
            Make::Kit(KitKind::SelfPatch) => "self-patch (Q)",
            Make::Barrage(false) => "barrage (LMB marks it)",
            Make::Barrage(true) => "sparse barrage (LMB marks it)",
            Make::Gate => "gate (LMB places 2 walls)",
        }
    }
    /// What a super crate adds: one behaviour twist (research section 7). The pad readout
    /// names it so the enemy can read the threat.
    fn twist(self) -> &'static str {
        match self {
            Make::Players(_) => "each joins with their class's super left click",
            Make::Gun(GunKind::Rapid) => "shots pierce one target",
            Make::Gun(GunKind::Rivet) => "rivets stagger",
            Make::Gun(GunKind::Heal) => "heals bounce to a 2nd ally",
            Make::Gun(GunKind::Shotgun) => "7 pellets, tighter spread",
            Make::Gun(GunKind::Mines) => "mines show the victim on the minimap",
            Make::Alt(AltKind::Rockets) => "rockets split into 3",
            Make::Alt(AltKind::ShieldWall) => "allies shoot through",
            Make::Alt(AltKind::Pulse) => "pulse adds a 25 shield",
            Make::Toss(TossKind::Grenade) => "sticks to what it touches",
            Make::Toss(TossKind::MedPack) => "adds a 25 shield",
            Make::PatchKit => "+2 charges",
            Make::Refill => "both to full",
            Make::Tank => "cannon leaves a slow zone, 450 hp",
            Make::Mech => "carries 4, stomp knocks crates off carriers",
            Make::Ambulance | Make::AmbulanceShot => "+25 shield every 5 s",
            Make::Bunker => "the gap is one-way",
            Make::Rally => "100 shield, clears slows",
            Make::Alt(AltKind::Launcher) => "grenades split on the bounce",
            Make::Alt(AltKind::ShieldBurst) => "shield also cleanses slows",
            Make::Toss(_) => "",
            Make::Kit(KitKind::Adrenaline) => "8 s, reload on kill",
            Make::Kit(KitKind::Barricade) => "allies can climb it",
            Make::Kit(KitKind::SelfPatch) => "adds a 25 shield",
            Make::Barrage(_) => "shells leave a slow zone",
            Make::Gate => "enemies touching it are slowed",
        }
    }
    /// The pad readout's name: "SUPER rockets (RMB) [rockets split into 3]".
    fn label(self, gold: bool) -> String {
        if gold { format!("SUPER {} [{}]", self.name(), self.twist()) } else { self.name().to_string() }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum GunKind {
    /// Soldier: 6 shots/s.
    Rapid,
    /// Engineer: slow, hard-hitting; a hit on a friendly wall repairs it.
    Rivet,
    /// Medic: a heal shot that heals the first hurt teammate it touches, weak against enemies.
    Heal,
    /// Set B soldier: a spread of pellets, short range.
    Shotgun,
    /// Set B engineer: each click drops a visible floor mine at your feet.
    Mines,
}

/// An all-8 vehicle, driven by its kicker. It soaks hits until destroyed; the driver comes
/// out unhurt.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Ride {
    /// Rocket cannon, can't carry.
    Tank,
    /// Carries several crates, no weapon; left click stomps.
    Mech,
    /// Heals allies within its aura; a weak blaster.
    Ambulance,
    /// Variant: no aura; the driver fires heal shots.
    AmbulanceShot,
}

impl Ride {
    fn name(self) -> &'static str {
        match self {
            Ride::Tank => "tank",
            Ride::Mech => "hauler mech",
            Ride::Ambulance | Ride::AmbulanceShot => "ambulance",
        }
    }
}

/// A right-click item from the class loadout (2 x 2 flat).
#[derive(Clone, Copy, PartialEq, Debug)]
enum AltKind {
    /// Soldier: rockets.
    Rockets,
    /// Engineer: a 4 m wall where you aim.
    ShieldWall,
    /// Medic: heals you and every teammate close by.
    Pulse,
    /// Set C soldier: grenades lobbed on a visible arc, bouncing once.
    Launcher,
    /// Set C medic: a shot that gives the first ally it touches a shield.
    ShieldBurst,
}

/// A Set C 1-crate item, used with Q.
#[derive(Clone, Copy, PartialEq, Debug)]
enum KitKind {
    /// Soldier: a few seconds of faster fire, lost if you pick up a crate.
    Adrenaline,
    /// Engineer: a short chest-high block where you aim.
    Barricade,
    /// Medic: hp now and more over a few seconds.
    SelfPatch,
}

/// A Set C all 8 waiting to be placed with left click (a barrage's circle, or gate walls).
#[derive(Clone, Copy)]
struct Mark {
    make: Make,
    gold: bool,
    /// Placements left (a gate has 2 walls).
    left: u32,
    /// Lost if not placed in this long.
    life: f32,
}

/// A 1-crate item thrown with Q, on the jump-throw arc.
#[derive(Clone, Copy, PartialEq, Debug)]
enum TossKind {
    Grenade,
    MedPack,
    /// Set C: a grenade from the launcher (right click), bouncing once.
    Launched,
    /// Set C: a barrage shell, falling from the sky.
    Shell,
}

#[derive(Clone, Copy)]
struct Alt {
    kind: AltKind,
    charges: u32,
    max: u32,
    gold: bool,
}

/// A player's class: the class of whoever kicks the pad open decides what it makes.
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
    fn next(self) -> Class {
        CLASSES[(self as usize + 1) % 3]
    }
}

// ---------------------------------------------------------------------------------------
// The item sets (#24-#26). Each slot picks which set its items come from; key 1 picks a
// whole set at once. Set B and Set C add their options to these lists.
// ---------------------------------------------------------------------------------------

/// Whole sets for key 1: (name, option index for left click, right click, 1 crate, all 8).
const SETS: [(&str, [usize; 4]); 7] = [
    ("A: fix the placeholders", [0, 0, 0, 0]),
    ("A + Set B's 1-crate refill", [0, 0, 1, 0]),
    ("B: Team Buddies ladder", [1, 0, 1, 1]),
    ("B, ambulance fires heal shots", [1, 0, 1, 2]),
    ("C: a capstone per class", [0, 1, 2, 3]),
    ("C, sparser barrage", [0, 1, 2, 4]),
    // The kit picked in the #22 grilling, mixed from the three sets.
    ("Final (#22)", [2, 2, 0, 5]),
];
/// Production-loss rules (key 1, #15).
const LOSS_OPTS: [&str; 4] = [
    "permanent (Team Buddies: base dies, no more players)",
    "knockout (pad down N s, then it rebuilds)",
    "rounds (a base kill wins the round, best of N)",
    "no base: theft only",
];
const PERMANENT: usize = 0;
const KNOCKOUT: usize = 1;
const ROUNDS: usize = 2;
const THEFT: usize = 3;
const SET_NAMES: [&str; 7] = [SETS[0].0, SETS[1].0, SETS[2].0, SETS[3].0, SETS[4].0, SETS[5].0, SETS[6].0];
/// Set C's left click is Set A's.
const LEFT_OPTS: [&str; 3] =
    ["A, C: rapid / rivet / heal gun", "B: shotgun / nail-mine layer / heal gun", "Final: shotgun / rivet / heal gun"];
/// Set B's right click is Set A's.
const RIGHT_OPTS: [&str; 3] = [
    "A, B: rockets / shield wall / heal pulse",
    "C: grenade launcher / shield wall / shield burst",
    "Final: rockets / shield wall / shield burst",
];
const SMALL_OPTS: [&str; 3] = ["A: grenade / patch kit / med pack", "B: refill", "C: adrenaline / barricade / self-patch"];
const ALL8_OPTS: [&str; 6] = [
    "A: tank / bunker / rally",
    "B: tank / hauler mech / ambulance (heal aura)",
    "B: tank / hauler mech / ambulance (heal shot)",
    "C: barrage / gate / rally",
    "C: sparser barrage / gate / rally",
    "Final: tank / gate / rally",
];

/// What a class makes from a recipe, given which set each slot is on.
fn item_of(slot: Slot, class: Class, sets: [usize; 4]) -> Make {
    use Class::*;
    match slot {
        Slot::Player1 => Make::Players(1),
        Slot::Player2 => Make::Players(2),
        Slot::Pair => match (sets[0], class) {
            (1 | 2, Soldier) => Make::Gun(GunKind::Shotgun),
            (1, Engineer) => Make::Gun(GunKind::Mines),
            (_, Soldier) => Make::Gun(GunKind::Rapid),
            (_, Engineer) => Make::Gun(GunKind::Rivet),
            (_, Medic) => Make::Gun(GunKind::Heal),
        },
        Slot::Flat4 => match (sets[1], class) {
            (1, Soldier) => Make::Alt(AltKind::Launcher),
            (1 | 2, Medic) => Make::Alt(AltKind::ShieldBurst),
            (_, Soldier) => Make::Alt(AltKind::Rockets),
            (_, Engineer) => Make::Alt(AltKind::ShieldWall),
            (_, Medic) => Make::Alt(AltKind::Pulse),
        },
        Slot::Small => match (sets[2], class) {
            (1, _) => Make::Refill,
            (2, Soldier) => Make::Kit(KitKind::Adrenaline),
            (2, Engineer) => Make::Kit(KitKind::Barricade),
            (2, Medic) => Make::Kit(KitKind::SelfPatch),
            (_, Soldier) => Make::Toss(TossKind::Grenade),
            (_, Engineer) => Make::PatchKit,
            (_, Medic) => Make::Toss(TossKind::MedPack),
        },
        Slot::All8 => match (sets[3], class) {
            (1 | 2, Engineer) => Make::Mech,
            (1, Medic) => Make::Ambulance,
            (2, Medic) => Make::AmbulanceShot,
            // ADR 0005 stands (ruling on #26): the medic's second wind is out; Set C plays
            // Set A's rally.
            (n @ (3 | 4), Soldier) => Make::Barrage(n == 4),
            (3 | 4 | 5, Engineer) => Make::Gate,
            (_, Soldier) => Make::Tank,
            (_, Engineer) => Make::Bunker,
            (_, Medic) => Make::Rally,
        },
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

/// Which stage a held kick of `t` seconds has reached, if any.
fn stage_at_k(t: f32, n: usize) -> Option<usize> {
    (n > 0 && t >= HOLD_FIRST).then(|| (((t - HOLD_FIRST) / HOLD_STEP) as usize).min(n - 1))
}

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
    // #42: the map switch (keys 1-5). Keep these first, in this order.
    Map,
    Symmetry,
    Queue,
    Pool,
    SmallTeam,
    // #36's switches, fixed at its verdict: server grabs, judged on what you saw.
    Ping,
    GrabMode,
    ActMode,
    Reach,
    Rewind,
    Preset,
    // #36: numbers for the switches above.
    Interp,
    RewindCap,
    DrillRival,
    DrillLock,
    DrillSlide,
    // #35's switches, fixed at its verdict (the defaults).
    FfShots,
    FfBlasts,
    FfCrates,
    FfPad,
    RandSpawns,
    RandRiver,
    RandMega,
    RandSpread,
    // #35: numbers for the switches above.
    FfDamage,
    BotsMind,
    BotsThrow,
    SpreadCone,
    SpreadRapid,
    SpawnWarn,
    MegaFrom,
    MegaTo,
    MegaWarn,
    RiverFrom,
    RiverTo,
    // #15 and #34's switches, fixed at ADR 0013 and ADR 0015's picks.
    Loss,
    LockIn,
    Spill,
    Mega,
    Overtime,
    Mercy,
    // #34: open seats, and whether you take one. The timer is ADR 0015's 8 min.
    OpenSeats,
    YouLate,
    // Production loss and comeback numbers (#15).
    BaseHp,
    KnockoutTime,
    KnockoutBonus,
    BestOf,
    RoundLength,
    LockInTime,
    SpillCap,
    MegaEvery,
    MegaWorth,
    MegaPull,
    OvertimeCap,
    MercyGap,
    MercyHold,
    Raiders,
    HeadStart,
    // #34: open seats (ADR 0012).
    SeatCutoff,
    SeatWindow,
    FightWindow,
    // The rest.
    TeamSize,
    AllyStacks,
    Seat,
    MatchLength,
    TimeScale,
    Camera,
    // The item sets from #24-#26, fixed at the Final kit (#22) by default.
    Set,
    LeftSet,
    RightSet,
    SmallSet,
    All8Set,
    Super,
    // Item numbers (research section 8: starting points, not tuning).
    SuperStrength,
    RapidRate,
    RapidRounds,
    RapidDmg,
    RocketCharges,
    RocketDmg,
    RocketBlast,
    GrenadeFuse,
    GrenadeBlast,
    GrenadeDmg,
    TankTime,
    TankHp,
    SlowZone,
    RivetRate,
    Rivets,
    RivetDmg,
    RivetRepair,
    WallCharges,
    WallLen,
    WallHp,
    WallLife,
    PatchHp,
    BunkerHp,
    BunkerLife,
    HealShot,
    HealShots,
    HealRange,
    PulseCharges,
    PulseHp,
    PulseRadius,
    MedPackHp,
    RallyShield,
    RallyInvuln,
    RefillHp,
    // Set B (#25).
    EnemyMap,
    ShotgunPellets,
    ShotgunDmg,
    ShotgunShells,
    ShotgunRate,
    ShotgunSpread,
    ShotgunRange,
    MineDmg,
    Mines,
    MinesOut,
    MineRoot,
    MineReveal,
    MechTime,
    MechHp,
    MechCrates,
    MechSpeed,
    StompRadius,
    StompPush,
    AmbTime,
    AmbHp,
    AmbRadius,
    AmbHeal,
    AmbShotHeal,
    AmbBlaster,
    // Set C (#26).
    LauncherCharges,
    LauncherDmg,
    LauncherBlast,
    LauncherRange,
    AdrenRate,
    AdrenTime,
    AdrenSuperTime,
    BarricadeLen,
    BarricadeHp,
    BarricadeLife,
    GateLen,
    GateHp,
    GateLife,
    GateSlow,
    MarkRange,
    MarkTime,
    BarrageRadius,
    BarrageDelay,
    BarrageShells,
    BarrageTime,
    SparseShells,
    SparseTime,
    ShellDmg,
    ShellBlast,
    ShellFall,
    BurstCharges,
    BurstShield,
    BurstRange,
    PatchNow,
    PatchOver,
    PatchSecs,
}

/// How many of the params are the rule switch (keys 1-5).
const RULES: usize = 5;

/// Key 6: presets for keys 2-5 (the ping stays). "as switched" leaves them alone.
const PRESET_OPTS: [&str; 5] = [
    "as switched",
    "the spec now: the server decides everything",
    "the research's plan: server grabs, predicted throws",
    "predict everything",
    "server grabs, judged on what you saw, wider reach",
];
const PRESETS: [[usize; 4]; 4] = [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 1, 1]];

/// Key 1: the ping (round trip), in ms.
const PINGS: [f32; 4] = [0.0, 95.0, 165.0, 250.0];
/// Key 4: grab reach, in m (from your centre to the crate's edge).
const REACHES: [f32; 3] = [1.2, 1.7, 2.2];

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
        // #42: the map switch.
        choice("1 map", BIG_MAP, &MAP_NAMES),
        choice("2 symmetry", 0, &["mirror: red's half is blue's, flipped east-west", "rotational: red's half is blue's, turned 180 deg"]),
        choice("3 queue picks", 0, &[
            "you pick (key 1)",
            "rotate through the pool",
            "random from the pool",
            "random, never the same map twice running",
        ]),
        choice("4 map pool", 1, &POOL_NAMES),
        choice("5 small map plays", 0, &["5v5", "4v4", "3v3 (ADR 0012's fallback)"]),
        // #36's verdict: server grabs, judged on what you saw. Ping 0 so maps compare clean.
        choice("ping", 0, &["0 ms (no latency)", "95 ms (busiest hours)", "165 ms (quietest hours)", "250 ms (worse than the sim saw)"]),
        choice("grabs and steals", 0, &[
            "server decides: a reach, then the crate after 1 round trip",
            "predicted: in your hands at once, snaps back if lost",
        ]),
        choice("throws, set-downs, kicks", 0, &[
            "server decides: shown 1 round trip (+ interpolation) late",
            "predicted: shown at once",
        ]),
        choice("grab reach", 0, &["1.2 m (now)", "1.7 m", "2.2 m"]),
        choice("server rewind", 1, &[
            "off: judged on where things are now",
            "on: judged on what you saw (capped)",
        ]),
        choice("latency preset", 0, &PRESET_OPTS),
        // What a 30 Hz send rate costs on top of the ping (netcode research: 1.2 send intervals).
        num("net: interpolation", 40.0, 0.0, 150.0, 10.0, "ms"),
        // Lightyear's demo caps the rewind so high pings don't get unbounded favour.
        num("net: rewind at most", 200.0, 50.0, 400.0, 25.0, "ms"),
        // Drills: how far the rival starts beyond the crate (Z, Shift+X), how long a stolen
        // crate has before it locks in (X), and how fast the slider goes (N).
        num("drill: rival starts", 2.5, 0.0, 10.0, 0.5, "m out"),
        num("drill: locks in after", 0.8, 0.2, 3.0, 0.1, "s"),
        num("drill: slider speed", 6.0, 1.0, 12.0, 0.5, "m/s"),
        // #35's verdict: the defaults.
        choice("ff shots", 0, &["off: bullets pass through allies", "on: bullets hurt allies"]),
        choice("ff blasts", 0, &["off: blasts spare you and allies", "your own blasts hurt you", "on: blasts hurt you and allies"]),
        choice("ff thrown crates", 0, &["off: thrown crates pass allies", "on: thrown crates hit allies"]),
        choice("ff own pad", 1, &["off: your team's crates can't knock your pad", "on: any crate knocks any pad crate"]),
        choice("crate spawns", 0, &["random spot, unmarked", "random spot, marked ahead", "fixed spots in turn, marked ahead"]),
        choice("river drops", 0, &["fixed: a super crate, back in 60 s", "random: a roll, scattered, back in 40-80 s"]),
        choice("megacrate timing", 0, &["fixed: every 45 s, countdown shown", "random: no warning", "random: marked ahead"]),
        choice("weapon spread", 0, &["none: shots go where you aim", "random cone", "bloom: first shot exact, held fire spreads"]),
        // Friendly damage is a share of the full hit, so it hurts but rarely kills.
        num("ff: damage", 50.0, 10.0, 100.0, 10.0, "% of a hit"),
        // Bots hold fire when an ally is in the way (only while that friendly fire is on).
        choice("ff: bots mind allies", 1, &["no: they fire anyway", "yes: they hold fire"]),
        // Bots used to walk every crate onto the pad. Throwing from 3-7 m out tries thrown
        // crates and own-pad knocks in bot matches.
        choice("bots throw onto the pad", 0, &["no: they set crates down", "yes: jump-throws from 3-7 m"]),
        num("spread: cone", 5.0, 1.0, 15.0, 1.0, "deg"),
        num("spread: rapid cone", 8.0, 1.0, 20.0, 1.0, "deg"),
        num("spawns: marked", 3.0, 1.0, 10.0, 1.0, "s ahead"),
        num("mega: random from", 25.0, 5.0, 60.0, 5.0, "s"),
        num("mega: random to", 65.0, 30.0, 120.0, 5.0, "s"),
        num("mega: marked", 5.0, 1.0, 15.0, 1.0, "s ahead"),
        num("river: random from", 40.0, 10.0, 60.0, 5.0, "s"),
        num("river: random to", 80.0, 60.0, 150.0, 5.0, "s"),
        // ADR 0013's pick: knockout, with all five comeback knobs on.
        choice("production loss", 1, &LOSS_OPTS),
        choice("lock-in", 1, &["off: pad crates score at once", "on: score after the lock-in timer"]),
        choice("spill", 1, &["off: a fallen pad's crates are destroyed", "on: they spill onto the map, loss capped"]),
        choice("megacrates", 2, &["off", "in the river", "nearer the trailing team"]),
        choice("overtime", 1, &["off: ends on the buzzer", "on: while the trailing team carries or contests"]),
        choice("mercy", 1, &["off", "on: ends early on a held economy gap"]),
        choice("open seats", 1, &["none: every seat filled at the start", "1 per team", "2 per team"]),
        choice("you", 0, &["start the match", "take blue's open seat (late joiner)"]),
        num("base: hp", 750.0, 250.0, 6000.0, 250.0, ""),
        num("knockout: pad down", 20.0, 5.0, 60.0, 5.0, "s"),
        num("knockout: bonus", 3.0, 0.0, 10.0, 1.0, "pts"),
        num("rounds: best of", 3.0, 1.0, 7.0, 2.0, ""),
        num("rounds: length", 150.0, 30.0, 300.0, 30.0, "s"),
        num("lock-in: timer", 8.0, 1.0, 30.0, 1.0, "s"),
        num("spill: most lost", 3.0, 0.0, 10.0, 1.0, "crates"),
        num("mega: every", 45.0, 10.0, 120.0, 5.0, "s"),
        num("mega: worth", 3.0, 2.0, 6.0, 1.0, "pts"),
        num("mega: pull at gap 10", 22.0, 0.0, 36.0, 2.0, "m"),
        num("overtime: at most", 60.0, 10.0, 180.0, 10.0, "s"),
        num("mercy: gap", 15.0, 5.0, 40.0, 1.0, "pts"),
        num("mercy: held for", 20.0, 5.0, 60.0, 5.0, "s"),
        num("bots raiding per team", 1.0, 0.0, 4.0, 1.0, ""),
        // Puts blue behind on purpose, to see whether it can come back.
        num("red head start", 0.0, 0.0, 4.0, 1.0, "players"),
        // ADR 0012: open seats are taken until about 40% of the match. A seat nobody takes
        // by then goes to a bot, so every open seat is filled by the cutoff.
        num("open seats: until", 40.0, 10.0, 100.0, 5.0, "% of timer"),
        num("open seats: filled within", 120.0, 0.0, 600.0, 15.0, "s"),
        // A match "ends on a fight" if a kill, steal or base hit came this close to the end.
        num("fight: within", 10.0, 2.0, 30.0, 1.0, "s"),
        num("team size", 5.0, 1.0, 8.0, 1.0, "v"),
        // Off: only you build and open blue's pad.
        choice("blue bots build", 1, &["off (only you)", "on"]),
        choice("your seat", 0, &["you play", "bot (watch)"]),
        num("match length", 480.0, 30.0, 900.0, 30.0, "s"),
        num("time scale", 1.0, 1.0, 10.0, 1.0, "x"),
        choice("camera", 0, &["follow you", "whole arena"]),
        // Fixed at the Final kit (main() applies it); change here to compare.
        choice("item set", 6, &SET_NAMES),
        choice("left click", 0, &LEFT_OPTS),
        choice("right click", 0, &RIGHT_OPTS),
        choice("1 crate", 0, &SMALL_OPTS),
        choice("all 8", 0, &ALL8_OPTS),
        // ADR 0007: a super crate upgrades the result.
        choice("super crates", 1, &["off", "upgrade (one twist)"]),
        num("super strength", 1.25, 1.0, 2.0, 0.05, "x"),
        num("rapid: rate", 6.0, 3.0, 12.0, 1.0, "/s"),
        num("rapid: rounds", 60.0, 10.0, 200.0, 10.0, ""),
        num("rapid: damage", 15.0, 5.0, 40.0, 1.0, ""),
        num("rockets: charges", 4.0, 1.0, 10.0, 1.0, ""),
        num("rockets: damage", 70.0, 20.0, 150.0, 5.0, ""),
        num("rockets: blast", 2.5, 1.0, 5.0, 0.25, "m"),
        num("grenade: fuse", 1.5, 0.5, 4.0, 0.25, "s"),
        num("grenade: blast", 2.0, 1.0, 5.0, 0.25, "m"),
        num("grenade: damage", 70.0, 20.0, 150.0, 5.0, ""),
        num("tank: time", 45.0, 10.0, 120.0, 5.0, "s"),
        num("tank: hp", 300.0, 100.0, 1000.0, 25.0, ""),
        num("tank: slow zone", 3.0, 1.0, 8.0, 0.5, "s"),
        num("rivet: rate", 2.0, 1.0, 6.0, 0.5, "/s"),
        num("rivet: rivets", 40.0, 10.0, 100.0, 5.0, ""),
        num("rivet: damage", 30.0, 10.0, 60.0, 1.0, ""),
        num("rivet: wall repair", 20.0, 0.0, 60.0, 5.0, "hp"),
        num("wall: charges", 2.0, 1.0, 6.0, 1.0, ""),
        num("wall: length", 4.0, 2.0, 8.0, 0.5, "m"),
        num("wall: hp", 250.0, 50.0, 800.0, 25.0, ""),
        num("wall: life", 20.0, 5.0, 60.0, 5.0, "s"),
        num("patch kit: wall hp", 60.0, 10.0, 200.0, 10.0, ""),
        num("bunker: hp each wall", 600.0, 100.0, 2000.0, 50.0, ""),
        num("bunker: life", 40.0, 10.0, 90.0, 5.0, "s"),
        num("heal gun: per shot", 8.0, 2.0, 20.0, 1.0, "hp"),
        num("heal gun: shots", 80.0, 20.0, 200.0, 10.0, ""),
        num("heal gun: range", 10.0, 4.0, 20.0, 1.0, "m"),
        num("pulse: charges", 3.0, 1.0, 8.0, 1.0, ""),
        num("pulse: heal", 40.0, 10.0, 100.0, 5.0, "hp"),
        num("pulse: radius", 7.0, 3.0, 12.0, 0.5, "m"),
        num("med pack: heal", 40.0, 10.0, 100.0, 5.0, "hp"),
        num("rally: shield", 50.0, 0.0, 100.0, 5.0, ""),
        num("rally: invulnerable", 4.0, 0.0, 10.0, 0.5, "s"),
        num("refill: hp if empty", 25.0, 0.0, 100.0, 5.0, ""),
        // Set B (#25). Enemies show on the minimap only when your team can see them, so a
        // super mine's reveal means something.
        choice("minimap enemies", 0, &["only in sight", "always shown"]),
        num("shotgun: pellets", 5.0, 1.0, 12.0, 1.0, ""),
        num("shotgun: dmg/pellet", 12.0, 4.0, 40.0, 1.0, ""),
        num("shotgun: shells", 12.0, 4.0, 40.0, 1.0, ""),
        num("shotgun: rate", 1.25, 0.5, 4.0, 0.25, "/s"),
        num("shotgun: spread", 30.0, 5.0, 60.0, 5.0, "deg"),
        num("shotgun: range", 7.0, 3.0, 15.0, 0.5, "m"),
        num("mine: damage", 40.0, 10.0, 100.0, 5.0, ""),
        num("mine: mines", 6.0, 1.0, 20.0, 1.0, ""),
        num("mine: max out", 3.0, 1.0, 10.0, 1.0, ""),
        num("mine: root", 1.0, 0.0, 3.0, 0.25, "s"),
        num("mine: super reveal", 3.0, 0.0, 10.0, 0.5, "s"),
        num("mech: time", 40.0, 10.0, 120.0, 5.0, "s"),
        num("mech: hp", 400.0, 100.0, 1000.0, 25.0, ""),
        num("mech: crates", 3.0, 1.0, 8.0, 1.0, ""),
        num("mech: speed", 0.8, 0.4, 1.2, 0.05, "x"),
        num("mech: stomp radius", 3.0, 1.0, 6.0, 0.25, "m"),
        num("mech: stomp push", 12.0, 2.0, 30.0, 1.0, "m/s"),
        num("ambulance: time", 40.0, 10.0, 120.0, 5.0, "s"),
        num("ambulance: hp", 350.0, 100.0, 1000.0, 25.0, ""),
        num("ambulance: aura", 5.0, 2.0, 10.0, 0.5, "m"),
        num("ambulance: aura heal", 15.0, 2.0, 40.0, 1.0, "hp/s"),
        num("ambulance: heal shot", 12.0, 2.0, 40.0, 1.0, "hp"),
        num("ambulance: blaster", 8.0, 0.0, 25.0, 1.0, "dmg"),
        // Set C (#26).
        num("launcher: charges", 3.0, 1.0, 10.0, 1.0, ""),
        num("launcher: damage", 60.0, 20.0, 150.0, 5.0, ""),
        num("launcher: blast", 2.0, 1.0, 5.0, 0.25, "m"),
        num("launcher: range", 14.0, 5.0, 25.0, 1.0, "m"),
        num("adrenaline: fire rate", 1.3, 1.0, 2.5, 0.05, "x"),
        num("adrenaline: time", 6.0, 1.0, 20.0, 0.5, "s"),
        num("adrenaline: super time", 8.0, 1.0, 20.0, 0.5, "s"),
        num("barricade: length", 2.0, 1.0, 6.0, 0.5, "m"),
        num("barricade: hp", 150.0, 25.0, 600.0, 25.0, ""),
        num("barricade: life", 20.0, 5.0, 60.0, 5.0, "s"),
        num("gate: wall length", 8.0, 2.0, 14.0, 0.5, "m"),
        num("gate: hp each wall", 500.0, 100.0, 2000.0, 50.0, ""),
        num("gate: life", 45.0, 10.0, 120.0, 5.0, "s"),
        num("gate: super slow", 1.5, 0.0, 5.0, 0.25, "s"),
        num("mark: range", 25.0, 8.0, 45.0, 1.0, "m"),
        num("mark: time to place", 30.0, 5.0, 90.0, 5.0, "s"),
        num("barrage: radius", 5.0, 2.0, 10.0, 0.5, "m"),
        num("barrage: delay", 3.0, 0.0, 8.0, 0.5, "s"),
        num("barrage: shells", 12.0, 1.0, 40.0, 1.0, ""),
        num("barrage: over", 4.0, 0.5, 12.0, 0.5, "s"),
        num("sparse: shells", 6.0, 1.0, 40.0, 1.0, ""),
        num("sparse: over", 4.0, 0.5, 12.0, 0.5, "s"),
        num("shell: damage", 50.0, 10.0, 150.0, 5.0, ""),
        num("shell: blast", 2.0, 0.5, 5.0, 0.25, "m"),
        num("shell: fall speed", 12.0, 4.0, 40.0, 1.0, "m/s"),
        num("shield burst: charges", 3.0, 1.0, 8.0, 1.0, ""),
        num("shield burst: shield", 40.0, 10.0, 100.0, 5.0, ""),
        num("shield burst: range", 10.0, 4.0, 20.0, 1.0, "m"),
        num("self-patch: now", 30.0, 0.0, 100.0, 5.0, "hp"),
        num("self-patch: over time", 30.0, 0.0, 100.0, 5.0, "hp"),
        num("self-patch: over", 3.0, 0.5, 10.0, 0.5, "s"),
    ]
}

#[derive(Resource)]
struct Tune {
    vals: Vec<Param>,
    selected: usize,
    panel: bool,
    /// Team sizes picked with Enter, printed with P.
    verdicts: Vec<String>,
    /// #36: the ping L swaps back to.
    last_ping: usize,
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
    /// How far in the past you see everything but yourself: the round trip plus the
    /// interpolation delay (none at 0 ms, the no-latency reference).
    fn delay(&self) -> f32 {
        let ping = PINGS[self.c(P::Ping)];
        if ping <= 0.0 { 0.0 } else { (ping + self.f(P::Interp)) / 1000.0 }
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

/// A switch's option index (`CRATE_RULES` digits).
fn fmt_value_index(v: &Value) -> String {
    match v {
        Value::Choice { i, .. } => i.to_string(),
        Value::Num { v, .. } => format!("{v:.0}"),
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
                set(&mut tune.vals, RULE_PS[k], d as f32);
            }
        }
    }
    if std::env::var("CRATE_BOTS_THROW").as_deref() == Ok("1") {
        set(&mut tune.vals, P::BotsThrow, 1.0);
    }
    if let Ok(v) = std::env::var("CRATE_BOTS_MIND") {
        set(&mut tune.vals, P::BotsMind, if v == "0" { 0.0 } else { 1.0 });
    }
    if let Some(x) = std::env::var("CRATE_SEED").ok().and_then(|v| v.parse::<u32>().ok()) {
        sim.rng = x.max(1).wrapping_mul(0x9e37_79b9) | 1;
    }
    if let Some(x) = std::env::var("CRATE_SPEED").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::TimeScale, x);
    }
    if let Some(x) = std::env::var("CRATE_HEADSTART").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::HeadStart, x);
    }
    if let Some(x) = std::env::var("CRATE_LENGTH").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::MatchLength, x);
        set(&mut tune.vals, P::RoundLength, x);
    }
    if let Ok(m) = std::env::var("CRATE_MAP") {
        let i = if m == "open" { OPEN_MAP } else { m.parse::<usize>().unwrap_or(0) };
        set(&mut tune.vals, P::Map, i as f32);
    }
    if let Ok(v) = std::env::var("CRATE_SYM") {
        set(&mut tune.vals, P::Symmetry, if v == "1" || v == "rot" { 1.0 } else { 0.0 });
    }
    if std::env::var("CRATE_ARENA").is_ok() {
        set(&mut tune.vals, P::Camera, 1.0);
    }
    if std::env::var("CRATE_SEAT").as_deref() == Ok("bot") {
        set(&mut tune.vals, P::Seat, 1.0);
    }
    if std::env::var("CRATE_LAG_BOT").as_deref() == Ok("1") {
        sim.lag_bot = true;
        set(&mut tune.vals, P::Seat, 1.0);
    }
    if let Some(x) = std::env::var("CRATE_RIVAL").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::DrillRival, x);
    }
    if let Some(x) = std::env::var("CRATE_LOCK").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::DrillLock, x);
    }
    if let Some(x) = std::env::var("CRATE_SLIDE").ok().and_then(|v| v.parse::<f32>().ok()) {
        set(&mut tune.vals, P::DrillSlide, x);
    }
    if let Ok(k) = std::env::var("CRATE_BENCH") {
        let (key, shift) = match k.as_str() {
            "z" => (KeyCode::KeyZ, false),
            "x" => (KeyCode::KeyX, false),
            "X" => (KeyCode::KeyX, true),
            _ => (KeyCode::KeyN, false),
        };
        let num = |v: &str, d: f32| std::env::var(v).ok().and_then(|x| x.parse::<f32>().ok()).unwrap_or(d);
        sim.bench = Some(Bench {
            key: Some(key),
            shift,
            name: k,
            reaction: num("CRATE_REACTION", 250.0) / 1000.0,
            trials: num("CRATE_TRIALS", 100.0) as u32,
            next: 3.0,
            ..default()
        });
    }
    if std::env::var("CRATE_TRUTH").as_deref() == Ok("1") {
        sim.truth = true;
    }
    if let Some(c) = std::env::var("CRATE_CLASS").ok().and_then(|v| v.parse::<usize>().ok()) {
        sim.your_class = CLASSES[c.min(2)];
    }
    sim.reset(&tune);
}

/// The rule switch, keys 1-5 in order.
const RULE_PS: [P; RULES] = [P::Map, P::Symmetry, P::Queue, P::Pool, P::SmallTeam];
/// #36's latency switch, now panel rows: ping, then the four the preset sets.
const LAT_PS: [P; 5] = [P::Ping, P::GrabMode, P::ActMode, P::Reach, P::Rewind];

/// Key 6: puts keys 2-5 on a preset.
fn apply_preset(vals: &mut [Param], i: usize) {
    if i == 0 {
        return;
    }
    for (k, o) in PRESETS[i - 1].into_iter().enumerate() {
        set(vals, LAT_PS[k + 1], o as f32);
    }
}

/// Key 1: puts every slot on the chosen set's options.
fn apply_set(vals: &mut [Param], i: usize) {
    let (_, opts) = SETS[i.min(SETS.len() - 1)];
    for (p, o) in [P::LeftSet, P::RightSet, P::SmallSet, P::All8Set].into_iter().zip(opts) {
        set(vals, p, o as f32);
    }
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
    /// #15: a rare crate worth several points on the pad; one cell in a recipe.
    Mega,
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
    /// Locked into the pad: it can't be taken, knocked or moved, only kicked open. With
    /// lock-in on (#15), a crate locks (and scores) once it has rested on its pad long enough.
    locked: bool,
    /// Seconds this crate has rested on a pad (the lock-in timer).
    pad_t: f32,
    ent: Option<Entity>,
    /// #36: what the server sent about it, to draw it as you'd see it online.
    hist: VecDeque<Seen>,
    /// #36: drawn where it really is (predicted) until this time.
    pred_until: f32,
    /// #36: you threw, set down, punted or opened it with prediction on: predicted while it
    /// moves, and for one delay after it stops (so the view catches up before it switches).
    pred_mine: bool,
    /// #36: when you learn it's in your hands (your grab's answer).
    mine_at: f32,
}

/// #36: one sample of a crate as the server had it.
#[derive(Clone, Copy)]
struct Seen {
    t: f32,
    pos: Vec3,
    loose: bool,
    /// In your hands: drawn at your hands as they are now (your character is predicted).
    mine: bool,
    locked: bool,
    /// Where it sat relative to you, when it was in your hands.
    off: Vec3,
}

/// #36: sample `hist` at time `want`. None while `want` is before the first sample (you
/// can't see it yet).
fn seen(hist: &VecDeque<Seen>, want: f32) -> Option<Seen> {
    let first = *hist.front()?;
    if want < first.t - 1e-4 {
        return None;
    }
    let mut prev = first;
    for &h in hist {
        if h.t >= want {
            let span = h.t - prev.t;
            let k = if span > 0.0 { ((want - prev.t) / span).clamp(0.0, 1.0) } else { 1.0 };
            let mut out = if k < 0.5 { prev } else { h };
            // Don't slide a crate from the floor into your hands: it pops, as it would.
            if prev.mine == h.mine {
                out.pos = prev.pos.lerp(h.pos, k);
                out.off = prev.off.lerp(h.off, k);
            }
            return Some(out);
        }
        prev = h;
    }
    hist.back().copied()
}

/// #36: a crate the server has removed (kicked open, destroyed) that you still see.
struct Ghost {
    id: u32,
    kind: Kind,
    ent: Option<Entity>,
    hist: VecDeque<Seen>,
    gone: f32,
}

/// #36: your grab, waiting for the server's answer to reach you.
struct GrabReq {
    id: u32,
    /// When you pressed, and when the answer reaches you (one round trip later).
    at: f32,
    answer_at: f32,
    /// Where you saw the crate when you pressed.
    seen: Vec3,
    ok: bool,
    steal: bool,
    why: &'static str,
    /// Shown in your hands at once (prediction on).
    predicted: bool,
}

/// #36: what latency did to your grabs, for one setup of the switch.
#[derive(Default, Clone)]
struct NetStats {
    grabs: u32,
    grabs_lost: u32,
    steals: u32,
    steals_lost: u32,
    /// Lost because someone took it first, it moved off, it locked in, or it was gone.
    taken: u32,
    moved: u32,
    locked: u32,
    gone: u32,
    /// Granted only because the server judged on what you saw.
    rewound: u32,
    /// Of those, pulled out of someone's hands.
    yanked: u32,
    /// Predicted grabs shown in your hands, then snapped back.
    snapbacks: u32,
    throws: u32,
    kicks: u32,
}

impl NetStats {
    fn line(&self) -> String {
        format!(
            "grabs {} ({} lost), steals {} ({} lost); lost: taken first {}, moved off {}, locked in {}, gone {}; rewind saved {} (yanked {}); snap-backs {}; throws/set-downs {}; kick-opens {}",
            self.grabs, self.grabs_lost, self.steals, self.steals_lost, self.taken, self.moved, self.locked, self.gone,
            self.rewound, self.yanked, self.snapbacks, self.throws, self.kicks,
        )
    }
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
    max: u32,
    /// From a super crate: its twist and more strength, never more ammo.
    gold: bool,
}

/// The all-8 vehicle you're driving (tank, hauler mech or ambulance).
#[derive(Clone, Copy)]
struct Tank {
    ride: Ride,
    hp: f32,
    max: f32,
    life: f32,
    gold: bool,
    /// Super ambulance: counts to the next 25 shield (every 5 s).
    beat: f32,
}

/// A nail mine on the floor: one-shot, set off by the first enemy to step on it.
struct Mine {
    pos: Vec3,
    team: usize,
    owner: usize,
    gold: bool,
    /// Arms this long after it's laid.
    arm: f32,
}

const MINE_R: f32 = 0.7;
/// How high a hauler mech carries its bottom crate.
const MECH_TOP: f32 = 2.6;

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
    /// 1-crate item to throw with Q (grenade or med pack), and whether it's super.
    toss: Option<(TossKind, bool)>,
    tank: Option<Tank>,
    /// The crates a hauler mech carries, bottom first.
    load: Vec<u32>,
    class: Class,
    /// Slowed while this is above 0 (a super tank's slow zone).
    slow: f32,
    /// Immune to slows (super rally).
    slow_immune: f32,
    /// Takes no damage (the medic after a rally).
    invuln: f32,
    /// Can't move while this is above 0 (a nail mine); can still shoot.
    root: f32,
    /// A shove (the mech's stomp) that slides you on its own, dying away.
    shove: Vec3,
    /// Shown on the enemy's minimap while this is above 0 (a super nail mine).
    reveal: f32,
    /// #36 drills: rushing this crate to grab it, until this time.
    rush: Option<(u32, f32)>,
    /// Absorbs damage before hp (from shield camps).
    shield: f32,
    /// Set C: the 1-crate item to use with Q, and whether it's super.
    kit: Option<(KitKind, bool)>,
    /// Adrenaline seconds left (faster fire), and whether it's super (reload on kill).
    adren: f32,
    adren_gold: bool,
    /// Self-patch: hp a second, for this many seconds more.
    regen: f32,
    regen_t: f32,
    /// A barrage or gate waiting to be placed with left click.
    mark: Option<Mark>,
    /// Bots: the goal they're routing to, the next waypoint, and how old it is.
    nav: Option<(Vec2, Vec2, f32)>,
    /// Recent positions, to show other players slightly in the past (online interpolation).
    hist: VecDeque<(f32, Vec3)>,
    ent: Option<Entity>,
    /// #34: an open seat (ADR 0012), filled from the queue at match time `seat_at`. Until
    /// then a player recipe can't bring this slot in.
    open: bool,
    seat_at: f32,
    /// When this player first came into the match, and seconds spent alive in it.
    entered: Option<f32>,
    played: f32,
    /// Match time this player went out (or, for an open seat, was seated): a player recipe
    /// brings in whoever has waited longest.
    out_since: f32,
    /// #35 bloom: how far the cone has opened (0 exact, 1 full), and seconds since the last
    /// shot (it closes again once you stop firing).
    bloom: f32,
    since_shot: f32,
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
            rush: None,
            fire_cd: 0.5,
            hp: 100.0,
            dead: 0.0,
            stun: 0.0,
            gun: None,
            alt: None,
            alt_cd: 0.0,
            toss: None,
            tank: None,
            load: Vec::new(),
            root: 0.0,
            shove: Vec3::ZERO,
            reveal: 0.0,
            slow: 0.0,
            slow_immune: 0.0,
            invuln: 0.0,
            // Bots by slot: 1st soldier, 2nd engineer, 3rd medic.
            class: CLASSES[slot % 3],
            shield: 0.0,
            kit: None,
            adren: 0.0,
            adren_gold: false,
            regen: 0.0,
            regen_t: 0.0,
            mark: None,
            nav: None,
            hist: VecDeque::new(),
            ent: None,
            open: false,
            seat_at: 0.0,
            entered: None,
            played: 0.0,
            out_since: 0.0,
            bloom: 0.0,
            since_shot: 9.0,
        }
    }
    fn alive(&self) -> bool {
        self.dead <= 0.0
    }
}

/// A wall: a static object with hp and a timer (allowed by the ADR 0004 ruling). A shield
/// wall goes where the engineer aims; a bunker is a horseshoe of them round the pad. Walls
/// block players, crates and shots from both teams (a super shield wall lets its own team's
/// shots through). A bunker gate fills the gap of a super bunker: it stops enemies walking in
/// and nothing else.
struct Wall {
    team: usize,
    /// Centre, on the floor.
    pos: Vec3,
    /// The way the wall faces (unit, flat). The wall runs across it.
    normal: Vec3,
    half_len: f32,
    hp: f32,
    max_hp: f32,
    life: f32,
    gold: bool,
    bunker: bool,
    gate: bool,
    /// Set C gate: your team's bodies and shots pass, enemies' don't.
    pass: bool,
    /// Set C barricade: low and short (a super one your team can climb).
    barricade: bool,
    /// How tall it is: shots, lobs and crates below this stop.
    h: f32,
    ent: Option<Entity>,
}

const WALL_H: f32 = 2.0;
/// A barricade is chest-high: shots stop, nobody jumps it (a super one, your team can).
const BARRICADE_H: f32 = 1.3;
const WALL_HALF_T: f32 = 0.2;
/// Bunker walls stand this far from the pad's centre.
const BUNKER_R: f32 = 2.6;
/// The bunker's gap, in the side facing the arena.
const GAP_HALF: f32 = 1.0;

impl Wall {
    /// (along the wall, out from its face) for a floor point.
    fn local(&self, p: Vec2) -> (f32, f32) {
        let d = p - self.pos.xz();
        let n = self.normal.xz();
        (d.dot(Vec2::new(-n.y, n.x)), d.dot(n))
    }
    fn touches(&self, p: Vec2, r: f32) -> bool {
        let (along, across) = self.local(p);
        along.abs() < self.half_len + r * 0.5 && across.abs() < WALL_HALF_T + r
    }
    /// Pushes a circle out of the wall's face. Returns the push normal.
    fn push_out(&self, p: &mut Vec2, r: f32) -> Option<Vec2> {
        if !self.touches(*p, r) {
            return None;
        }
        let (_, across) = self.local(*p);
        let n = self.normal.xz() * if across >= 0.0 { 1.0 } else { -1.0 };
        *p += n * (WALL_HALF_T + r - across.abs());
        Some(n)
    }
    /// Whether it stops a body on `team`.
    fn blocks_body(&self, team: usize) -> bool {
        !(self.gate || self.pass) || team != self.team
    }
    /// Whether the straight walk from `a` to `b` crosses it.
    fn crosses(&self, a: Vec2, b: Vec2, r: f32) -> bool {
        let n = (a.distance(b) / 0.3).ceil().max(1.0) as usize;
        (0..=n).any(|k| self.touches(a.lerp(b, k as f32 / n as f32), r))
    }
    /// Distance from a floor point to the wall.
    fn dist(&self, p: Vec2) -> f32 {
        let (along, across) = self.local(p);
        Vec2::new((along.abs() - self.half_len).max(0.0), (across.abs() - WALL_HALF_T).max(0.0)).length()
    }
}

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
    owner: Option<usize>,
    team: usize,
    dmg: f32,
    /// Rockets: blast radius (0 for bullets).
    splash: f32,
    life: f32,
    /// Super rapid gun: passes through this many more players.
    pierce: u8,
    /// The last player it hit, so a piercing shot doesn't hit them twice.
    last: Option<usize>,
    /// Heal shots: heal the first hurt teammate they touch (enemies take `dmg`).
    heal: f32,
    /// Super heal shot: bounces on to a second ally.
    bounce: bool,
    /// Rivets repair friendly walls.
    rivet: bool,
    /// Extra stun on a hit (super rivets).
    stagger: f32,
    /// Super rockets: split into 3 small blasts.
    split: bool,
    /// Super tank cannon: leaves a slow zone.
    slow: bool,
    /// Heal shot from an ambulance (counted as ambulance healing).
    amb: bool,
    /// Shield burst: the first ally it touches gets this much shield (a super one also
    /// clears slows).
    shield: f32,
    cleanse: bool,
    /// #36: seconds since it was fired (others' shots show a delay late).
    age: f32,
    ent: Option<Entity>,
}

impl Shot {
    fn new(pos: Vec3, vel: Vec3, owner: Option<usize>, team: usize, dmg: f32, life: f32) -> Shot {
        Shot {
            pos,
            vel,
            owner,
            team,
            dmg,
            splash: 0.0,
            life,
            pierce: 0,
            last: None,
            heal: 0.0,
            bounce: false,
            rivet: false,
            stagger: 0.0,
            split: false,
            slow: false,
            amb: false,
            shield: 0.0,
            cleanse: false,
            age: 0.0,
            ent: None,
        }
    }
}

/// A grenade or med pack in flight (or a grenade lying or stuck, waiting to go off).
#[derive(Clone, Copy)]
struct Lob {
    kind: TossKind,
    pos: Vec3,
    vel: Vec3,
    team: usize,
    owner: usize,
    gold: bool,
    fuse: f32,
    /// A super grenade sticks to the first thing it touches: a player (and their offset),
    /// or the floor or a wall.
    stuck: Option<Option<(usize, Vec3)>>,
    /// Launcher grenades: bounces so far (they go off on the second touch), and whether it's
    /// one of the 3 a super one splits into.
    hops: u8,
    small: bool,
    /// Barrage shells: the barrage they belong to.
    wave: u32,
}

/// A Set C barrage: a marked circle that shells fall on after a delay. Both teams see it.
struct Barrage {
    id: u32,
    team: usize,
    owner: usize,
    at: Vec3,
    r: f32,
    /// Seconds until the first shell.
    delay: f32,
    shells_left: u32,
    every: f32,
    next: f32,
    gold: bool,
    /// Enemies (by actor index) any of its shells hit.
    hit: u64,
}

/// A blast: rockets, grenades and the tank cannon.
struct Boom {
    at: Vec3,
    team: usize,
    dmg: f32,
    r: f32,
    owner: Option<usize>,
    split: bool,
    slow: bool,
}

/// What a PvE camp drops when cleared.
#[derive(Clone, Copy, PartialEq)]
enum Drop {
    Heal,
    Shield,
    /// A thrown med pack (super: it adds a shield).
    MedPack(bool),
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
    lobs: Vec<Lob>,
    walls: Vec<Wall>,
    mines: Vec<Mine>,
    /// Slow zones from a super tank's cannon: where, radius, time left, whose.
    zones: Vec<(Vec3, f32, f32, usize)>,
    /// Each team's rally: seconds left (it locks out all 8 while it lasts).
    rally: [f32; 2],
    barrages: Vec<Barrage>,
    monsters: Vec<Monster>,
    pickups: Vec<Pickup>,
    /// A walled map is on (else the open arena).
    map: bool,
    /// #42: the map being played (`MAP_NAMES`), and whether red's half is rotated.
    map_i: usize,
    rotate: bool,
    /// Keys 1 and 2 as last seen: changing either restarts on that map.
    tune_map: usize,
    tune_sym: usize,
    /// The match ended on its own, so the queue picks the next map.
    queued: bool,
    /// Maps played this run, in order.
    played_maps: Vec<usize>,
    /// Where each bot last stood still, and for how long (bots stuck on walls).
    still: Vec<(Vec2, f32)>,
    builds: [Option<Build>; 2],
    /// #36: your grab waiting on the server's answer.
    grab: Option<GrabReq>,
    /// #36: crates the server removed that you still see.
    ghosts: Vec<Ghost>,
    /// #36: each pad's kick progress over time, to draw it as you'd see it.
    kick_hist: VecDeque<(f32, [f32; 2])>,
    /// #36: grab stats for each setup of the latency switch (the last one is current).
    net: Vec<(String, NetStats)>,
    /// #36 (key O): draw where things really are over what you see.
    truth: bool,
    /// #36 (`CRATE_LAG_BOT=1`): your seat's bot sees the world late and grabs through the
    /// server like you do, to measure lost grabs without a human.
    lag_bot: bool,
    /// #36 (`CRATE_BENCH`): a drill run over and over, grabbing a fixed reaction time after
    /// the crate shows up in your view.
    bench: Option<Bench>,
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
    /// The tunables, copied from `Tune` each tick (read with `n`).
    tv: Vec<f32>,
    /// Which set each slot's items come from: left click, right click, 1 crate, all 8.
    sets: [usize; 4],
    /// The crate grammar picked in #17 (ADR 0007), fixed: by shape, readout shown, instant,
    /// steal only, hold to choose.
    count_only: bool,
    shown: bool,
    super_mode: usize,
    build_time: f32,
    vuln: usize,
    /// The class you play (C cycles it); survives resets.
    your_class: Class,
    /// Smoke runs: you get your class's all 8 at this match time, once it's free.
    all8_at: Option<f32>,
    /// Smoke runs: red marks a barrage on you (B), or shuts two lanes with gates (J), at
    /// these match times.
    red_barrage_at: Option<f32>,
    red_gates_at: Option<f32>,
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
    /// Crates stolen off each team's pad, and how many of those while its bunker was up.
    stolen: [u32; 2],
    stolen_bunkered: [u32; 2],
    /// Seconds each team's bunker stood, and the damage its walls took.
    bunker_time: [f32; 2],
    wall_dmg: [f32; 2],
    /// Wall hp repaired by rivets and patch kits.
    repaired: [f32; 2],
    /// Heals picked up from heal camps, and from med packs.
    camp_heals: [u32; 2],
    pack_heals: [u32; 2],
    /// Shots fired with each team's crafted left-click guns, and with the standard gun.
    crafted_shots: [u32; 2],
    plain_shots: [u32; 2],
    // Set B.
    /// Mines each team laid, and how many went off under an enemy.
    mines_laid: [u32; 2],
    mine_hits: [u32; 2],
    /// Crates each team's hauler mechs picked up, and set down on their own pad.
    mech_grabbed: [u32; 2],
    mech_banked: [u32; 2],
    /// Seconds each team had a vehicle out, and the most out at once (both teams).
    ride_time: [f32; 2],
    rides_max: usize,
    /// Hp each team's ambulances healed.
    amb_healed: [f32; 2],
    /// Seconds any ambulance was out, and the kills made in that time.
    amb_time: f32,
    amb_kills: u32,
    // Set C.
    /// Barrages each team marked; enemies in the circle at the mark, still in it when the
    /// shells began, and hit by a shell.
    barrage_n: [u32; 2],
    barrage_in_mark: [u32; 2],
    barrage_in_fall: [u32; 2],
    barrage_hit: [u32; 2],
    /// Seconds each team had a gate up, and seconds each team's players were held at an
    /// enemy gate.
    gate_time: [f32; 2],
    gate_held: [f32; 2],
    /// Left-click items each team ran dry, and lost on a knockout.
    dry: [u32; 2],
    gun_lost: [u32; 2],
    /// Adrenaline cancelled by picking up a crate.
    adren_cut: [u32; 2],
    /// Shield each team's shield bursts gave.
    burst_given: [f32; 2],
    // #15: production loss and comebacks.
    /// The production-loss rule (key 1; changing it restarts the match).
    loss: usize,
    /// The comeback knobs (keys 2-6, live).
    lock_in: bool,
    spill: bool,
    mega: usize,
    overtime: bool,
    mercy: bool,
    /// Each team's base hp, and how long its pad stays down (infinite once destroyed).
    base_hp: [f32; 2],
    base_down: [f32; 2],
    /// Seconds since each base was last hit (bots come home to defend it).
    base_hit_t: [f32; 2],
    /// Seconds to the next megacrate.
    mega_t: f32,
    /// Seconds of overtime played, and how long it lasts once nobody contests.
    ot: f32,
    ot_grace: f32,
    /// Seconds the mercy gap has been held.
    mercy_t: f32,
    /// Rounds: the round being played, and each team's round wins.
    round: usize,
    wins: [usize; 2],
    /// When `over` runs out: the next round starts (else a new match).
    next_round: bool,
    /// A finished rounds match, waiting for `end_match` (winner, how).
    match_end: Option<(Option<usize>, String)>,
    /// Match-long stats that survive round resets.
    track: Track,
    /// Matches finished this run (smoke runs exit after `CRATE_MATCHES`).
    matches_done: usize,
    // #35: friendly fire and randomness.
    /// Keys 1-4: shots, blasts, thrown crates, own pad.
    ff: [usize; 4],
    /// Keys 5-8: crate spawns, river drops, megacrate timing, weapon spread.
    luck: [usize; 4],
    /// Drops marked on the floor ahead of time: where, what, seconds to go.
    marked: Vec<(Vec3, Kind, f32)>,
    /// Fixed crate spawns: the next spot in turn.
    spawn_turn: usize,
}

/// #15: what a match did to production and the lead. Survives round resets.
#[derive(Default)]
struct Track {
    time: f32,
    leader: Option<usize>,
    lead_changes: u32,
    /// The biggest lead each team had.
    max_lead: [usize; 2],
    /// Seconds each team spent behind by 10 or more.
    hopeless: [f32; 2],
    base_dmg: [f32; 2],
    /// Times each team's base fell.
    falls: [u32; 2],
    /// Crates each team lost when its base fell: spilled onto the map, and destroyed.
    spilled: [u32; 2],
    destroyed: [u32; 2],
    /// Megacrates each team picked up, and where they dropped (blue side, middle, red side).
    mega_grabbed: [u32; 2],
    mega_dropped: [u32; 3],
    overtime: f32,
    kills: [u32; 2],
    stolen: [u32; 2],
    rounds: Vec<String>,
    /// Bot-seconds each team spent raiding the enemy base, and players each team had up
    /// (summed over time, for the average).
    raiding: [f32; 2],
    up: [f32; 2],
    /// Bot-seconds at the raid post, and shots fired at the enemy base.
    posted: [f32; 2],
    base_shots: [u32; 2],
    // #34: pacing.
    /// Match time of the last kill, steal or base hit (a fight), and of each kind.
    last_fight: f32,
    last_kill: f32,
    last_steal: f32,
    last_base_hit: f32,
    /// Kills, steals and base hits (counted once a second at most) in the last minute
    /// before the end.
    fights: Vec<f32>,
    /// Who led (None: tied) and since when; the winner was decided from this point on.
    lead_now: Option<usize>,
    lead_since: f32,
    /// The scores and who led when the timer ran out.
    buzzer: Option<(usize, usize)>,
    /// Each team's score at every quarter of the timer.
    quarters: Vec<(usize, usize)>,
    /// When the mercy gap was first reached, in the stretch that fired it.
    mercy_from: f32,
    /// The last time either score changed, and the scores then.
    last_score: f32,
    scores: (usize, usize),
    // #35: friendly fire, per team that dealt it.
    /// Damage to teammates, and teammates knocked out.
    ff_dmg: [f32; 2],
    ff_ko: [u32; 2],
    /// Damage to yourself (your own blasts), and players who knocked themselves out.
    self_dmg: [f32; 2],
    self_ko: [u32; 2],
    /// Teammates hit by your team's thrown crates, and crates your team carried that
    /// friendly fire knocked loose.
    crate_ff: [u32; 2],
    ff_drops: [u32; 2],
    /// Your team's crates that knocked your own pad's crates, and knocks the rule stopped.
    own_knocks: [u32; 2],
    knocks_stopped: [u32; 2],
    /// Shots bots held because an ally was in the way.
    held: [u32; 2],
    /// Shots fired (a trigger pull) and enemy players hit.
    fired: [u32; 2],
    hit: [u32; 2],
    /// #35 randomness: normal crates that spawned on each team's half, crates each team got
    /// from river objectives, and the gaps between megacrates.
    spawn_side: [u32; 2],
    river_crates: [u32; 2],
    river_cleared: [u32; 2],
    mega_at: Vec<f32>,
    // #42: per map.
    /// Seconds each team spent carrying crates, and crates picked up.
    carry_t: [f32; 2],
    carries: [u32; 2],
    /// Match time of the first kill.
    first_kill: f32,
    /// Bot-seconds stood still for 5 s+ outside the bases (stuck on walls, or idle).
    stuck: f32,
}

/// #15: each team's base, a block behind its pad that enemy shots and blasts hurt.
const BASE_HALF: f32 = 0.7;
const BASE_H: f32 = 1.8;

fn base_pos(team: usize) -> Vec2 {
    let pad = pads()[team];
    Vec2::new(pad.x - toward_centre(team) * (PAD_HALF + 1.0), pad.y)
}

fn base_block(team: usize) -> Block {
    let c = base_pos(team);
    Block { min: c - Vec2::splat(BASE_HALF), max: c + Vec2::splat(BASE_HALF) }
}

/// Vulnerability options (key 6).
const STEAL: usize = 0;
const SPLIT: usize = 1;
const SPILL: usize = 2;

impl Sim {
    fn new(tune: &Tune) -> Self {
        let mut s = Sim {
            actors: Vec::new(),
            crates: Vec::new(),
            shots: Vec::new(),
            lobs: Vec::new(),
            walls: Vec::new(),
            mines: Vec::new(),
            zones: Vec::new(),
            rally: [0.0; 2],
            monsters: Vec::new(),
            pickups: Vec::new(),
            map: true,
            map_i: 0,
            rotate: false,
            tune_map: 0,
            tune_sym: 0,
            queued: false,
            played_maps: Vec::new(),
            still: Vec::new(),
            builds: [None, None],
            grab: None,
            ghosts: Vec::new(),
            kick_hist: VecDeque::new(),
            net: vec![(String::new(), NetStats::default())],
            truth: false,
            lag_bot: false,
            bench: None,
            next_id: 1,
            rng: 0x2545_f491,
            t: 0.0,
            loot_respawn: 0.0,
            super_respawn: 0.0,
            msgs: Vec::new(),
            despawn: Vec::new(),
            booms: Vec::new(),
            team_size: 0,
            tv: Vec::new(),
            sets: [0; 4],
            count_only: false,
            shown: true,
            super_mode: 0,
            build_time: 0.0,
            vuln: STEAL,
            your_class: Class::Soldier,
            all8_at: None,
            red_barrage_at: None,
            red_gates_at: None,
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
            stolen: [0; 2],
            stolen_bunkered: [0; 2],
            bunker_time: [0.0; 2],
            wall_dmg: [0.0; 2],
            repaired: [0.0; 2],
            camp_heals: [0; 2],
            pack_heals: [0; 2],
            crafted_shots: [0; 2],
            plain_shots: [0; 2],
            mines_laid: [0; 2],
            mine_hits: [0; 2],
            mech_grabbed: [0; 2],
            mech_banked: [0; 2],
            ride_time: [0.0; 2],
            rides_max: 0,
            amb_healed: [0.0; 2],
            amb_time: 0.0,
            amb_kills: 0,
            barrages: Vec::new(),
            barrage_n: [0; 2],
            barrage_in_mark: [0; 2],
            barrage_in_fall: [0; 2],
            barrage_hit: [0; 2],
            gate_time: [0.0; 2],
            gate_held: [0.0; 2],
            dry: [0; 2],
            gun_lost: [0; 2],
            adren_cut: [0; 2],
            burst_given: [0.0; 2],
            loss: KNOCKOUT,
            lock_in: false,
            spill: false,
            mega: 0,
            overtime: false,
            mercy: false,
            base_hp: [0.0; 2],
            base_down: [0.0; 2],
            base_hit_t: [99.0; 2],
            mega_t: 0.0,
            ot: 0.0,
            ot_grace: 0.0,
            mercy_t: 0.0,
            round: 1,
            wins: [0; 2],
            next_round: false,
            match_end: None,
            track: Track::default(),
            matches_done: 0,
            ff: [0, 0, 0, 1],
            luck: [0; 4],
            marked: Vec::new(),
            spawn_turn: 0,
        };
        s.reset(tune);
        s
    }

    fn rules(&mut self, tune: &Tune) {
        self.tv = tune.vals.iter().map(|p| match p.value {
            Value::Num { v, .. } => v,
            Value::Choice { i, .. } => i as f32,
        }).collect();
        self.sets = [P::LeftSet, P::RightSet, P::SmallSet, P::All8Set].map(|p| tune.c(p));
        self.count_only = false;
        self.shown = true;
        self.super_mode = tune.c(P::Super);
        self.build_time = 0.0;
        self.vuln = STEAL;
        self.hold_kick = true;
        self.lock_in = tune.c(P::LockIn) == 1;
        self.spill = tune.c(P::Spill) == 1;
        self.mega = tune.c(P::Mega);
        self.overtime = tune.c(P::Overtime) == 1;
        self.mercy = tune.c(P::Mercy) == 1;
        self.ff = [P::FfShots, P::FfBlasts, P::FfCrates, P::FfPad].map(|p| tune.c(p));
        self.luck = [P::RandSpawns, P::RandRiver, P::RandMega, P::RandSpread].map(|p| tune.c(p));
    }

    /// #35 (key 7): seconds to the next megacrate, fixed or rolled. A marked one is rolled
    /// that much sooner, so it lands as often on average.
    fn mega_gap(&mut self) -> f32 {
        match self.luck[2] {
            0 => self.n(P::MegaEvery),
            k => {
                let (from, to) = (self.n(P::MegaFrom), self.n(P::MegaTo).max(self.n(P::MegaFrom)));
                let warn = if k == 2 { self.n(P::MegaWarn) } else { 0.0 };
                (from + self.rand() * (to - from) - warn).max(1.0)
            }
        }
    }

    /// A tunable's value.
    fn n(&self, p: P) -> f32 {
        self.tv[p as usize]
    }

    /// Super strength, for a super item.
    fn k(&self, gold: bool) -> f32 {
        if gold { self.n(P::SuperStrength) } else { 1.0 }
    }

    /// A new match: round 1, and the match-long stats start over.
    fn reset(&mut self, tune: &Tune) {
        // #42: a match that ended on its own goes to the queue (key 3); anything else (R,
        // keys 1-2, start-up) plays the map on key 1.
        let queue = tune.c(P::Queue);
        if self.queued && queue > 0 {
            let pool: Vec<usize> = POOLS[tune.c(P::Pool)].to_vec();
            let at = pool.iter().position(|&m| m == self.map_i);
            self.map_i = match queue {
                1 => pool[at.map_or(0, |k| (k + 1) % pool.len())],
                2 => pool[(self.rand() * pool.len() as f32) as usize % pool.len()],
                _ => {
                    let rest: Vec<usize> = pool.iter().copied().filter(|&m| m != self.map_i || pool.len() == 1).collect();
                    rest[(self.rand() * rest.len() as f32) as usize % rest.len()]
                }
            };
        } else {
            self.map_i = tune.c(P::Map);
        }
        self.queued = false;
        self.tune_map = tune.c(P::Map);
        self.tune_sym = tune.c(P::Symmetry);
        self.rotate = self.tune_sym == 1;
        self.loss = tune.c(P::Loss);
        self.round = 1;
        self.wins = [0; 2];
        self.track = Track::default();
        self.match_end = None;
        self.reset_round(tune);
    }

    /// A fresh field: players, crates, camps and bases (every round in the rounds rule).
    fn reset_round(&mut self, tune: &Tune) {
        for c in self.crates.drain(..) {
            self.despawn.extend(c.ent);
        }
        self.lobs.clear();
        self.mines.clear();
        self.zones.clear();
        self.rally = [0.0; 2];
        for w in self.walls.drain(..) {
            self.despawn.extend(w.ent);
        }
        for m in self.monsters.drain(..) {
            self.despawn.extend(m.ent);
        }
        self.pickups.clear();
        set_layout(self.map_i, self.rotate);
        self.map = self.map_i < MAPS;
        self.still.clear();
        for sh in self.shots.drain(..) {
            self.despawn.extend(sh.ent);
        }
        for a in self.actors.drain(..) {
            self.despawn.extend(a.ent);
        }
        self.rules(tune);
        self.builds = [None, None];
        self.grab = None;
        for g in self.ghosts.drain(..) {
            self.despawn.extend(g.ent);
        }
        self.kick_hist.clear();
        self.team_size = team_size_for(tune, self.map_i);
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
        // #34: the top slots of each team are open seats (ADR 0012), filled from the queue at
        // a random time within the window, and by the cutoff at the latest (a bot takes a seat
        // nobody took). Key 9 puts you in blue's last open seat: blue's last bot starts instead.
        let length = tune.f(P::MatchLength);
        let cutoff = length * tune.f(P::SeatCutoff) / 100.0;
        let open = tune.c(P::OpenSeats).min(self.team_size.saturating_sub(1));
        if open > 0 && tune.c(P::YouLate) == 1 && self.team_size > 1 {
            let last = self.team_size - 1;
            self.actors[YOU].slot = last;
            self.actors[last].slot = 0;
            self.actors[last].class = CLASSES[0];
        }
        for i in 0..self.actors.len() {
            if self.actors[i].slot >= self.team_size - open {
                let at = (self.rand() * tune.f(P::SeatWindow)).min(cutoff);
                let a = &mut self.actors[i];
                a.open = true;
                a.seat_at = at;
                a.out_since = at;
            }
        }
        for i in 0..self.actors.len() {
            let p = self.spawn_point(i);
            let a = &mut self.actors[i];
            a.pos = p;
            a.facing = Vec3::X * toward_centre(a.team);
            // Each team starts with one player; the rest join from pad recipes. A red head
            // start (#15) brings some of red's in at once.
            let head = if a.team == 1 { tune.c(P::HeadStart) } else { 0 };
            if a.slot > head {
                a.dead = f32::INFINITY;
            }
        }
        self.actors[YOU].class = self.your_class;
        if self.map {
            self.reset_map();
        } else {
            self.reset_open();
        }
        self.match_left = tune.f(if self.loss == ROUNDS { P::RoundLength } else { P::MatchLength });
        self.over = 0.0;
        self.next_round = false;
        self.base_hp = [tune.f(P::BaseHp); 2];
        self.base_down = [0.0; 2];
        self.base_hit_t = [99.0; 2];
        self.mega_t = self.mega_gap();
        self.marked.clear();
        self.spawn_turn = 0;
        self.ot = 0.0;
        self.ot_grace = 0.0;
        self.mercy_t = 0.0;
        self.fired = 0;
        self.hits = 0;
        self.kills = [0; 2];
        self.lost = [0; 2];
        self.broken = [0; 2];
        self.pve_deaths = [0; 2];
        self.stolen = [0; 2];
        self.stolen_bunkered = [0; 2];
        self.bunker_time = [0.0; 2];
        self.wall_dmg = [0.0; 2];
        self.repaired = [0.0; 2];
        self.camp_heals = [0; 2];
        self.pack_heals = [0; 2];
        self.crafted_shots = [0; 2];
        self.plain_shots = [0; 2];
        self.mines_laid = [0; 2];
        self.mine_hits = [0; 2];
        self.mech_grabbed = [0; 2];
        self.mech_banked = [0; 2];
        self.ride_time = [0.0; 2];
        self.rides_max = 0;
        self.amb_healed = [0.0; 2];
        self.amb_time = 0.0;
        self.amb_kills = 0;
        self.barrages.clear();
        self.barrage_n = [0; 2];
        self.barrage_in_mark = [0; 2];
        self.barrage_in_fall = [0; 2];
        self.barrage_hit = [0; 2];
        self.gate_time = [0.0; 2];
        self.gate_held = [0.0; 2];
        self.dry = [0; 2];
        self.gun_lost = [0; 2];
        self.adren_cut = [0; 2];
        self.burst_given = [0.0; 2];
    }

    /// A walled map (#42): its starting crates and loot, a heal camp and a shield camp per
    /// side, and the objectives in the middle.
    fn reset_map(&mut self) {
        let l = layout();
        for &p in &l.crates {
            if blocked(p, HALF) {
                continue;
            }
            let p = (p / CRATE).round() * CRATE;
            self.spawn_crate(Kind::Normal, p.extend(HALF).xzy());
        }
        for &p in &l.loot {
            self.spawn_crate(Kind::Loot, Vec3::new(p.x, HALF, p.y));
        }
        for &(p, drop) in &l.camps {
            let (hp, dmg) = if drop == Drop::Super { (320.0, 12.0) } else { (200.0, 8.0) };
            self.add_monster(p.extend(0.0).xzy(), hp, dmg, drop);
        }
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
            pad_t: 0.0,
            ent: None,
            hist: VecDeque::new(),
            pred_until: 0.0,
            pred_mine: false,
            mine_at: 0.0,
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
            .fold(0.0, f32::max);
        Vec3::new(xz.x, top + HALF, xz.y)
    }

    /// The heights of a pad's 4 cells, the recipe they make, and the crates in it.
    fn pad_state(&self, team: usize) -> PadState {
        let cells = pad_cells(team);
        let mut st = PadState { h: [0; 4], shape: Shape::Empty, ids: Vec::new(), supers: 0 };
        // A knocked-out or destroyed base: its pad makes nothing.
        if self.builds[team].is_some() || !self.pad_up(team) {
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
        (item_of(slot, class, self.sets), supers > 0 && self.super_mode > 0)
    }

    /// The team's all 8 that's still out, if any: no new all 8 until it ends.
    fn capstone(&self, team: usize) -> Option<String> {
        if let Some(t) = self.actors.iter().filter(|a| a.team == team && a.alive()).find_map(|a| a.tank) {
            return Some(format!("{} {:.0} s", t.ride.name(), t.life.max(0.0)));
        }
        if let Some(w) = self.walls.iter().find(|w| w.team == team && w.bunker) {
            return Some(format!("bunker {:.0} s", w.life.max(0.0)));
        }
        if let Some(m) = self.actors.iter().filter(|a| a.team == team && a.alive()).find_map(|a| a.mark) {
            let what = if m.make == Make::Gate { "gate" } else { "barrage" };
            return Some(format!("{what} to place, {:.0} s", m.life.max(0.0)));
        }
        if let Some(b) = self.barrages.iter().find(|b| b.team == team) {
            return Some(format!("barrage, {} shells to fall", b.shells_left));
        }
        if let Some(w) = self.walls.iter().find(|w| w.team == team && w.pass) {
            return Some(format!("gate {:.0} s", w.life.max(0.0)));
        }
        (self.rally[team] > 0.0).then(|| format!("rally {:.0} s", self.rally[team]))
    }

    /// Hold-to-choose (rule 7): the recipes inside `team`'s pad that can open now, smallest
    /// first (player recipes only while the team has room).
    fn stages(&self, team: usize) -> Vec<(Slot, [u8; 4])> {
        let st = self.pad_state(team);
        if matches!(st.shape, Shape::Empty | Shape::Unaligned | Shape::Busy) {
            return Vec::new();
        }
        let room = self.room(team) > 0;
        let locked = self.capstone(team).is_some();
        sub_recipes(st.h, self.count_only)
            .into_iter()
            .filter(|(slot, _)| room || !matches!(slot, Slot::Player1 | Slot::Player2))
            .filter(|(slot, _)| !(locked && *slot == Slot::All8))
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
        stage_at_k(self.pad_kick[team], n)
    }

    /// #36: crate `c` as the server has it now.
    fn seen_now(&self, c: &Crate) -> Seen {
        let you = self.actors[YOU].pos;
        let mine = matches!(c.state, CrateState::Carried { by: YOU, .. });
        Seen { t: self.t, pos: c.pos, loose: c.loose(), mine, locked: c.locked, off: c.pos - you }
    }

    /// Whether `team` can open its pad: the shape is a recipe, and there's room for any
    /// players it brings.
    fn can_open(&self, team: usize, shape: Shape) -> bool {
        matches!(shape, Shape::Recipe(_)) && (shape.players() == 0 || self.room(team) > 0)
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
        let room = if builds { self.room(team) } else { 0 };
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
            self.use_up(&st.ids, team, Some(by));
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
        let class = if !matches!(make, Make::Players(_)) { format!("{} ", self.actors[by].class.name()) } else { String::new() };
        let what = format!("{class}{}", make.label(gold));
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
        self.use_up(&ids, team, Some(by));
        self.made[team].push(what.clone());
        let msg = format!("{side} pad kicked open by {who}: {label} -> {what}");
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
        self.apply(team, by, make, gold);
    }

    /// Removes a pad's crates once they're spent; they still count for the score.
    fn use_up(&mut self, ids: &[u32], team: usize, by: Option<usize>) {
        let worth: usize = self.crates.iter().filter(|c| ids.contains(&c.id)).map(|c| self.worth(c.kind)).sum();
        let gone: Vec<Crate> = self.crates.extract_if(.., |c| ids.contains(&c.id)).collect();
        // #36: you see your own kick-open at once only with prediction on.
        let predicted = by == Some(YOU) && self.acts_predicted();
        for c in gone {
            self.bury(c, predicted);
        }
        if by == Some(YOU) && self.lagged() {
            self.stats().kicks += 1;
        }
        self.banked[team] += worth;
    }

    /// #36: a crate leaves the server. You keep seeing it until the news reaches you.
    fn bury(&mut self, c: Crate, predicted: bool) {
        if predicted || self.view_delay() <= 0.0 || !self.lagged() || c.hist.is_empty() {
            self.despawn.extend(c.ent);
        } else {
            self.ghosts.push(Ghost { id: c.id, kind: c.kind, ent: c.ent, hist: c.hist, gone: self.t });
        }
    }

    /// #36: the round trip, in seconds.
    fn rtt(&self) -> f32 {
        PINGS[self.tv[P::Ping as usize] as usize] / 1000.0
    }

    /// #36: how far in the past you see everything but yourself.
    fn view_delay(&self) -> f32 {
        let rtt = self.rtt();
        if rtt <= 0.0 { 0.0 } else { rtt + self.n(P::Interp) / 1000.0 }
    }

    /// Your seat sees the world late: you play it, or it's the measuring bot.
    fn lagged(&self) -> bool {
        self.you_play || self.lag_bot
    }

    fn grabs_predicted(&self) -> bool {
        self.tv[P::GrabMode as usize] as usize == 1
    }

    fn acts_predicted(&self) -> bool {
        self.tv[P::ActMode as usize] as usize == 1
    }

    /// #36: grab reach, for everyone (it's a rule, so bots get it too).
    fn reach(&self) -> f32 {
        REACHES[self.tv[P::Reach as usize] as usize]
    }

    /// #36: the stats for the setup being played.
    fn stats(&mut self) -> &mut NetStats {
        &mut self.net.last_mut().expect("net stats start in step").1
    }

    /// A build timer finished: the result comes out.
    fn finish_build(&mut self, team: usize) {
        let Some(b) = self.builds[team].take() else { return };
        self.use_up(&b.ids, team, Some(b.by));
        let class = if !matches!(b.make, Make::Players(_)) { format!("{} ", self.actors[b.by].class.name()) } else { String::new() };
        let what = format!("{class}{}", b.make.label(b.gold));
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

    /// Gives `team` what a recipe makes. Items go to whoever kicked it open (or a living
    /// teammate if they're down).
    fn apply(&mut self, team: usize, by: usize, make: Make, gold: bool) {
        let by = if self.actors[by].alive() {
            Some(by)
        } else {
            (0..self.actors.len()).find(|&j| self.actors[j].team == team && self.actors[j].alive())
        };
        let pad = pads()[team];
        let front = Vec3::new(pad.x + toward_centre(team) * 1.6, 0.0, pad.y);
        let k = self.k(gold);
        let Some(b) = by else { return self.join_players(team, front, make, gold) };
        match make {
            Make::Players(_) => self.join_players(team, front, make, gold),
            Make::Gun(kind) => self.actors[b].gun = Some(self.new_gun(kind, gold)),
            Make::Alt(kind) => self.actors[b].alt = Some(self.new_alt(kind, gold)),
            Make::Toss(kind) => self.actors[b].toss = Some((kind, gold)),
            Make::PatchKit => {
                // +1 right-click charge if one is missing, else hp back on your nearest wall.
                let pos = self.actors[b].pos;
                let charges = if gold { 2 } else { 1 };
                let wall = (0..self.walls.len())
                    .filter(|&w| self.walls[w].team == team && !self.walls[w].gate && self.walls[w].hp < self.walls[w].max_hp)
                    .min_by(|&x, &y| self.walls[x].dist(pos.xz()).total_cmp(&self.walls[y].dist(pos.xz())));
                let what = match (&mut self.actors[b].alt, wall) {
                    (Some(alt), _) if alt.charges < alt.max => {
                        alt.charges = (alt.charges + charges).min(alt.max);
                        format!("+{charges} right-click charge")
                    }
                    (_, Some(w)) => {
                        let w = &mut self.walls[w];
                        let add = (self.tv[P::PatchHp as usize] * k).min(w.max_hp - w.hp);
                        w.hp += add;
                        self.repaired[team] += add;
                        format!("+{add:.0} hp on a wall")
                    }
                    (Some(alt), None) => {
                        // Nothing to fix: the charge goes over the top.
                        alt.charges += charges;
                        format!("+{charges} right-click charge (over full)")
                    }
                    (None, None) => "nothing to patch: no right-click item and no hurt wall".to_string(),
                };
                let who = if b == YOU { "you".to_string() } else { self.describe(b) };
                self.say(format!("patch kit ({who}): {what}"));
            }
            Make::Refill => {
                // Set B: left click to full, right click +1 (super: both to full); +25 hp if
                // you hold neither.
                let a = &mut self.actors[b];
                let held = a.gun.is_some() || a.alt.is_some();
                if let Some(g) = &mut a.gun {
                    g.ammo = g.max;
                }
                if let Some(alt) = &mut a.alt {
                    alt.charges = if gold { alt.max } else { (alt.charges + 1).min(alt.max) };
                }
                if !held {
                    a.hp = (a.hp + self.tv[P::RefillHp as usize] * k).min(100.0);
                }
            }
            Make::Tank | Make::Mech | Make::Ambulance | Make::AmbulanceShot => {
                let (ride, hp, life) = match make {
                    Make::Tank => (Ride::Tank, self.n(P::TankHp) * if gold { 1.5 } else { 1.0 }, self.n(P::TankTime)),
                    Make::Mech => (Ride::Mech, self.n(P::MechHp), self.n(P::MechTime)),
                    Make::Ambulance => (Ride::Ambulance, self.n(P::AmbHp), self.n(P::AmbTime)),
                    _ => (Ride::AmbulanceShot, self.n(P::AmbHp), self.n(P::AmbTime)),
                };
                let a = &mut self.actors[b];
                a.tank = Some(Tank { ride, hp, max: hp, life, gold, beat: 0.0 });
                a.gun = None;
                // A crate in your hands goes into the mech, or falls off.
                match (ride, a.carrying.take()) {
                    (Ride::Mech, Some(id)) => a.load.push(id),
                    (_, Some(id)) => {
                        a.carrying = Some(id);
                        self.release(b, Vec3::Y, Vec3::ZERO);
                    }
                    _ => {}
                }
            }
            Make::Bunker => self.add_bunker(team, gold),
            Make::Kit(kind) => self.actors[b].kit = Some((kind, gold)),
            Make::Barrage(_) | Make::Gate => {
                // Placed with left click, where you aim (you fire nothing until then).
                let left = if make == Make::Gate { 2 } else { 1 };
                let life = self.n(P::MarkTime);
                self.actors[b].mark = Some(Mark { make, gold, left, life });
            }
            Make::Rally => {
                let shield = if gold { 100.0 } else { self.n(P::RallyShield) };
                let t = self.n(P::RallyInvuln);
                for a in self.actors.iter_mut().filter(|a| a.team == team && a.alive()) {
                    a.hp = 100.0;
                    a.shield = a.shield.max(shield);
                    if gold {
                        a.slow = 0.0;
                        a.slow_immune = t;
                    }
                    self.booms.push((a.pos + Vec3::Y, 1.2, 0.5));
                }
                self.actors[b].invuln = t;
                // The rally's lockout lasts as long as the medic is invulnerable.
                self.rally[team] = t.max(0.5);
            }
        }
    }

    fn join_players(&mut self, team: usize, front: Vec3, make: Make, gold: bool) {
        let Make::Players(n) = make else { return };
        for m in 0..n {
            // #34: whoever has waited longest among those out and seated (an open seat waits
            // for the queue, then queues behind anyone knocked out before it filled).
            let Some(slot) = (0..self.actors.len())
                .filter(|&j| self.actors[j].team == team && self.actors[j].dead.is_infinite() && self.seated(&self.actors[j]))
                .min_by(|&i, &j| {
                    let (a, b) = (&self.actors[i], &self.actors[j]);
                    a.out_since.total_cmp(&b.out_since).then(a.slot.cmp(&b.slot))
                })
            else {
                break;
            };
            // A super crate in a player recipe: each joiner comes in holding their own
            // class's super left-click item (#22).
            let class = if slot == YOU { self.your_class } else { self.actors[slot].class };
            let gun = match item_of(Slot::Pair, class, self.sets) {
                Make::Gun(kind) if gold => Some(self.new_gun(kind, true)),
                _ => None,
            };
            let p = &mut self.actors[slot];
            p.dead = 0.0;
            p.hp = 100.0;
            p.pos = front + Vec3::Z * (m as f32 * 1.2 - 0.6);
            p.vel = Vec3::ZERO;
            p.hist.clear();
            p.gun = gun;
            if p.entered.is_none() {
                p.entered = Some(self.track.time);
                if p.open {
                    let (at, now) = (p.seat_at, self.track.time);
                    let who = if slot == YOU { "you".to_string() } else { format!("{} player {}", ["blue", "red"][team], p.slot + 1) };
                    self.say(format!("LATE JOINER {who} is in at {}:{:02} (seat filled {:.0} s before)", now as u32 / 60, now as u32 % 60, now - at));
                }
            }
        }
        let n = self.players(team);
        self.say(format!("{} has {n}/{} players", ["BLUE", "RED"][team], self.team_size));
    }

    fn new_gun(&self, kind: GunKind, gold: bool) -> Gun {
        let ammo = match kind {
            GunKind::Rapid => self.n(P::RapidRounds),
            GunKind::Rivet => self.n(P::Rivets),
            GunKind::Heal => self.n(P::HealShots),
            GunKind::Shotgun => self.n(P::ShotgunShells),
            GunKind::Mines => self.n(P::Mines),
        } as u32;
        Gun { kind, ammo, max: ammo, gold }
    }

    fn new_alt(&self, kind: AltKind, gold: bool) -> Alt {
        let n = match kind {
            AltKind::Rockets => self.n(P::RocketCharges),
            AltKind::ShieldWall => self.n(P::WallCharges),
            AltKind::Pulse => self.n(P::PulseCharges),
            AltKind::Launcher => self.n(P::LauncherCharges),
            AltKind::ShieldBurst => self.n(P::BurstCharges),
        } as u32;
        Alt { kind, charges: n, max: n, gold }
    }

    /// A shield wall `half_len` wide, facing `normal`.
    fn add_wall(&mut self, team: usize, pos: Vec3, normal: Vec3, half_len: f32, hp: f32, life: f32, gold: bool, bunker: bool, gate: bool) {
        self.walls.push(Wall {
            team,
            pos: pos.with_y(0.0),
            normal,
            half_len,
            hp,
            max_hp: hp,
            life,
            gold,
            bunker,
            gate,
            pass: false,
            barricade: false,
            h: WALL_H,
            ent: None,
        });
    }

    /// The engineer's all 8: three full sides round the pad and the side facing the arena
    /// with a gap in the middle. Super: a gate fills the gap that only enemies can't pass.
    fn add_bunker(&mut self, team: usize, gold: bool) {
        let p = pads()[team].extend(0.0).xzy();
        let t = toward_centre(team);
        let (hp, life) = (self.n(P::BunkerHp) * self.k(gold), self.n(P::BunkerLife));
        let r = BUNKER_R;
        self.add_wall(team, p - Vec3::X * t * r, Vec3::X, r + WALL_HALF_T, hp, life, gold, true, false);
        self.add_wall(team, p + Vec3::Z * r, Vec3::Z, r + WALL_HALF_T, hp, life, gold, true, false);
        self.add_wall(team, p - Vec3::Z * r, Vec3::Z, r + WALL_HALF_T, hp, life, gold, true, false);
        let side = (r - GAP_HALF) / 2.0;
        for sz in [-1.0, 1.0] {
            let at = p + Vec3::X * t * r + Vec3::Z * sz * (GAP_HALF + side);
            self.add_wall(team, at, Vec3::X, side + WALL_HALF_T, hp, life, gold, true, false);
        }
        if gold {
            self.add_wall(team, p + Vec3::X * t * r, Vec3::X, GAP_HALF, hp, life, gold, true, true);
        }
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
        let enemy = 1 - self.actors[who].team;
        if pad_contains(pads()[enemy], from) {
            self.stolen[enemy] += 1;
            self.fight(1);
            if self.walls.iter().any(|w| w.team == enemy && w.bunker) {
                self.stolen_bunkered[enemy] += 1;
            }
        }
        let team = self.actors[who].team;
        if !pad_contains(pads()[team], from) {
            self.track.carries[team] += 1;
        }
        if self.crates.iter().any(|c| c.id == id && c.kind == Kind::Mega && !pad_contains(pads()[team], c.pos)) {
            self.track.mega_grabbed[team] += 1;
        }
        if let Some(c) = self.crate_mut(id) {
            c.state = CrateState::Carried { by: who, lift_from: from, lift_t: 0.0 };
            c.vel = Vec3::ZERO;
            c.thrown_by = None;
            c.pad_t = 0.0;
        }
        let a = &mut self.actors[who];
        a.claim = None;
        // Adrenaline ends when you pick up a crate.
        if a.adren > 0.0 {
            a.adren = 0.0;
            let team = a.team;
            self.adren_cut[team] += 1;
            if who == YOU {
                self.say("adrenaline cancelled: you picked up a crate");
            }
        }
        let a = &mut self.actors[who];
        // A hauler mech stacks what it picks up.
        if a.tank.is_some_and(|t| t.ride == Ride::Mech) {
            a.load.push(id);
            let team = a.team;
            self.mech_grabbed[team] += 1;
        } else {
            a.carrying = Some(id);
        }
    }

    /// How many crates `who`'s hauler mech carries at most (0 when not in one).
    fn mech_cap(&self, who: usize) -> usize {
        match self.actors[who].tank {
            Some(t) if t.ride == Ride::Mech => self.n(P::MechCrates) as usize + t.gold as usize,
            _ => 0,
        }
    }

    /// A hauler mech takes its top crate in hand, to set down or throw.
    fn hand_top(&mut self, who: usize) {
        let a = &mut self.actors[who];
        if a.carrying.is_none()
            && let Some(id) = a.load.pop()
        {
            a.carrying = Some(id);
        }
    }

    /// `who` leaves their vehicle (destroyed or out of time): a mech's crates fall off.
    fn end_ride(&mut self, who: usize, why: &str) {
        let Some(t) = self.actors[who].tank.take() else { return };
        let load = std::mem::take(&mut self.actors[who].load);
        let pos = self.actors[who].pos;
        for (k, id) in load.into_iter().enumerate() {
            let a = k as f32 * 2.1 + self.rand();
            if let Some(c) = self.crate_mut(id) {
                c.state = CrateState::Free;
                c.pos = pos + Vec3::Y * (MECH_TOP + k as f32 * CRATE);
                c.vel = Vec3::new(a.cos() * 3.0, 3.0, a.sin() * 3.0);
                c.placed_by = None;
            }
        }
        let what = if who == YOU { "your".to_string() } else { format!("{}'s", self.describe(who)) };
        self.say(format!("{what} {} {why}", t.ride.name()));
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
        if !self.actors[who].alive() || self.actors[who].invuln > 0.0 {
            return;
        }
        // #35: friendly fire does a share of the hit.
        let friendly = by_team == self.actors[who].team;
        let own = friendly && by == Some(who);
        let amount = if friendly { amount * self.n(P::FfDamage) / 100.0 } else { amount };
        if own {
            self.track.self_dmg[by_team] += amount;
        } else if friendly {
            self.track.ff_dmg[by_team] += amount;
            if self.actors[who].carrying.is_some() {
                self.track.ff_drops[by_team] += 1;
            }
        }
        // A vehicle soaks hits until it's destroyed; the driver comes out unhurt.
        if let Some(tank) = &mut self.actors[who].tank {
            tank.hp -= amount;
            if tank.hp <= 0.0 {
                self.end_ride(who, "destroyed");
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
        // Permanent rule (#15): once the base is destroyed, the starting player is out too.
        let respawns = first && self.base_down[a.team].is_finite();
        let a = &mut self.actors[who];
        a.dead = if respawns { 3.0 } else { f32::INFINITY };
        a.out_since = self.track.time;
        a.claim = None;
        let lost_gun = a.gun.take().is_some();
        a.alt = None;
        a.toss = None;
        a.kit = None;
        a.adren = 0.0;
        a.regen_t = 0.0;
        let lost_mark = a.mark.take().is_some();
        a.shield = 0.0;
        a.slow = 0.0;
        let team = a.team;
        if lost_gun {
            self.gun_lost[team] += 1;
        }
        if lost_mark {
            self.say(format!("{} went down with an all 8 still to place: it's lost", self.describe(who)));
        }
        let a = &mut self.actors[who];
        // by_team 2 is a PvE monster.
        if own {
            self.track.self_ko[by_team] += 1;
            self.say(format!("{} knocked themselves out", if who == YOU { "YOU".to_string() } else { self.describe(who) }));
        } else if friendly {
            self.track.ff_ko[by_team] += 1;
            let by = by.map_or("a teammate".to_string(), |b| if b == YOU { "YOU".into() } else { self.describe(b) });
            self.say(format!("TEAM KNOCKOUT: {by} knocked out {}", if who == YOU { "YOU".to_string() } else { self.describe(who) }));
        } else if by_team < 2 {
            self.kills[by_team] += 1;
            if self.track.first_kill == 0.0 {
                self.track.first_kill = self.track.time;
            }
            self.fight(0);
            if self.actors.iter().any(|b| b.tank.is_some_and(|t| matches!(t.ride, Ride::Ambulance | Ride::AmbulanceShot))) {
                self.amb_kills += 1;
            }
        } else {
            self.pve_deaths[a.team] += 1;
        }
        // Super adrenaline: a kill reloads your left-click item.
        if let Some(k) = by
            && self.actors[k].adren > 0.0
            && self.actors[k].adren_gold
            && let Some(g) = &mut self.actors[k].gun
        {
            g.ammo = g.max;
        }
        let what = self.describe(who);
        if who == YOU {
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

    /// #34: seats a player recipe can bring in now: out, and (an open seat) already filled
    /// from the queue.
    fn room(&self, team: usize) -> usize {
        self.actors.iter().filter(|a| a.team == team && a.dead.is_infinite() && self.seated(a)).count()
    }

    fn seated(&self, a: &Actor) -> bool {
        !a.open || self.track.time >= a.seat_at
    }

    /// #34: a kill, steal or base hit, for "does the match end on a fight".
    fn fight(&mut self, kind: usize) {
        let t = self.track.time;
        let tr = &mut self.track;
        tr.last_fight = t;
        match kind {
            0 => tr.last_kill = t,
            1 => tr.last_steal = t,
            _ => {
                // Base hits come several a second: count one a second.
                if t - tr.last_base_hit < 1.0 {
                    return;
                }
                tr.last_base_hit = t;
            }
        }
        tr.fights.push(t);
    }

    /// Crates resting on the team's pad (with lock-in on, only locked-in ones; on a pad that
    /// is down, only locked-in ones), plus crates spent there on recipes and points banked.
    fn score(&self, team: usize) -> usize {
        let on_pad: usize = self
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind.builds() && pad_contains(pads()[team], c.pos))
            .filter(|c| self.builds[team].as_ref().is_none_or(|b| !b.ids.contains(&c.id)))
            .filter(|c| c.locked || (!self.lock_in && self.pad_up(team)))
            .map(|c| self.worth(c.kind))
            .sum();
        on_pad + self.banked[team]
    }

    /// Points a crate is worth on a pad: a megacrate is worth several.
    fn worth(&self, kind: Kind) -> usize {
        if kind == Kind::Mega { self.n(P::MegaWorth) as usize } else { 1 }
    }

    /// Whether `team`'s base can be hurt now: there is one, and it's standing.
    fn base_up(&self, team: usize) -> bool {
        self.loss != THEFT && self.base_down[team] <= 0.0
    }

    /// Whether `team`'s pad works: it doesn't while its base is knocked out or destroyed.
    fn pad_up(&self, team: usize) -> bool {
        self.base_down[team] <= 0.0
    }

    /// An enemy shot or blast hits `team`'s base.
    fn hurt_base(&mut self, team: usize, dmg: f32) {
        if !self.base_up(team) || self.over > 0.0 {
            return;
        }
        self.base_hp[team] -= dmg;
        self.base_hit_t[team] = 0.0;
        self.track.base_dmg[team] += dmg;
        self.fight(2);
        if self.base_hp[team] <= 0.0 {
            self.base_fall(team);
        }
    }

    /// `team`'s base falls. The production-loss rule decides what that does; the pad's crates
    /// that aren't locked in are lost: destroyed, or (spill on) thrown onto the map, at most
    /// the cap. Locked-in crates are safe.
    fn base_fall(&mut self, team: usize) {
        let by = 1 - team;
        let (side, them) = (["BLUE", "RED"][team], ["BLUE", "RED"][by]);
        self.base_hp[team] = 0.0;
        self.track.falls[team] += 1;
        self.pad_kick[team] = 0.0;
        self.kick_by[team] = None;
        if self.loss == ROUNDS {
            let msg = format!("{side} BASE DESTROYED: {them} wins the round");
            println!("t={:.1}s {msg}", self.t);
            self.say(msg);
            return self.end_round(Some(by), "base kill");
        }
        let pad = pads()[team];
        let mut open: Vec<(u32, f32, bool)> = self
            .crates
            .iter()
            .filter(|c| c.state == CrateState::Free && c.kind.builds() && pad_contains(pad, c.pos))
            .map(|c| (c.id, c.pos.y, c.locked))
            .collect();
        // Top crates go first.
        open.sort_by(|a, b| b.1.total_cmp(&a.1));
        let safe: Vec<u32> = open.iter().filter(|c| c.2).map(|c| c.0).collect();
        let open: Vec<u32> = open.iter().filter(|c| !c.2).map(|c| c.0).collect();
        let cap = if self.spill { self.n(P::SpillCap) as usize } else { usize::MAX };
        let lost: Vec<u32> = open.iter().copied().take(cap).collect();
        let kept: Vec<u32> = open.iter().copied().skip(cap).collect();
        if self.spill {
            for (k, &id) in lost.iter().enumerate() {
                let a = k as f32 * 2.4 + self.rand();
                let toward = Vec2::new(toward_centre(team), 0.0);
                if let Some(c) = self.crate_mut(id) {
                    let out = (toward + Vec2::from_angle(a) * 0.6).normalize_or(toward);
                    c.vel = Vec3::new(out.x * 7.0, 6.0, out.y * 7.0);
                    c.pos.y += 0.3;
                    c.settling = false;
                    c.pad_t = 0.0;
                }
            }
            self.track.spilled[team] += lost.len() as u32;
        } else {
            let gone: Vec<Crate> = self.crates.extract_if(.., |c| lost.contains(&c.id)).collect();
            for c in gone {
                self.bury(c, false);
            }
            self.track.destroyed[team] += lost.len() as u32;
        }
        let what = if lost.is_empty() {
            "no crates lost".to_string()
        } else if self.spill {
            format!("{} crate{} spilled onto the map", lost.len(), if lost.len() == 1 { "" } else { "s" })
        } else {
            format!("{} crate{} destroyed", lost.len(), if lost.len() == 1 { "" } else { "s" })
        };
        let msg = if self.loss == PERMANENT {
            // The pad is gone: what's left on it is banked, and nobody new comes in.
            let rest: Vec<u32> = safe.iter().chain(&kept).copied().collect();
            self.use_up(&rest, team, None);
            for a in self.actors.iter_mut().filter(|a| a.team == team && a.dead > 0.0) {
                a.dead = f32::INFINITY;
            }
            self.base_down[team] = f32::INFINITY;
            format!("{side} BASE DESTROYED for good ({what}): no more players, nobody respawns")
        } else {
            self.base_down[team] = self.n(P::KnockoutTime);
            let bonus = self.n(P::KnockoutBonus) as usize;
            self.banked[by] += bonus;
            format!("{side} BASE KNOCKED OUT ({what}): pad down {:.0} s, {them} +{bonus}", self.n(P::KnockoutTime))
        };
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
    }

    /// Rounds rule: a round ends (base kill, time, mercy). The match ends once a team has
    /// won most of best-of-N.
    fn end_round(&mut self, winner: Option<usize>, why: &str) {
        if self.over > 0.0 {
            return;
        }
        if let Some(w) = winner {
            self.wins[w] += 1;
        }
        let (b, r) = (self.score(0), self.score(1));
        let who = winner.map_or("draw".to_string(), |w| format!("{} wins", ["BLUE", "RED"][w]));
        let line = format!("round {}: BLUE {b} - {r} RED, {who} by {why}, rounds {}-{}", self.round, self.wins[0], self.wins[1]);
        println!("t={:.1}s {line}", self.t);
        self.say(line.clone());
        self.track.rounds.push(line);
        self.add_round_stats();
        let best = self.n(P::BestOf) as usize;
        // Draws don't count; stop after 2N rounds so a run of draws can't go on forever.
        if self.wins.iter().any(|&w| w > best / 2) || self.round >= best * 2 {
            let w = match self.wins[0].cmp(&self.wins[1]) {
                std::cmp::Ordering::Greater => Some(0),
                std::cmp::Ordering::Less => Some(1),
                std::cmp::Ordering::Equal => None,
            };
            self.match_end = Some((w, format!("rounds {}-{}", self.wins[0], self.wins[1])));
        } else {
            self.next_round = true;
            self.over = 3.0;
        }
    }

    /// Kills and steals this round, into the match-long stats.
    fn add_round_stats(&mut self) {
        for t in 0..2 {
            self.track.kills[t] += self.kills[t];
            self.track.stolen[t] += self.stolen[t];
        }
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
            // #35: friendly fire on blasts: your own, or everyone's.
            let spared = match self.ff[1] {
                0 => a.team == team,
                1 => a.team == team && owner != Some(j),
                _ => false,
            };
            if spared || !a.alive() {
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
        for w in self.walls.iter_mut().filter(|w| w.team != team && !w.gate) {
            if w.dist(at.xz()) < radius {
                w.hp -= dmg;
                self.wall_dmg[w.team] += dmg;
            }
        }
        for m in 0..self.monsters.len() {
            if self.monsters[m].alive() && self.monsters[m].pos.xz().distance(at.xz()) < radius + 0.6 {
                self.hurt_monster(m, dmg, owner);
            }
        }
        let enemy = 1 - team;
        if self.base_up(enemy) && base_block(enemy).hits(at.xz(), radius) {
            self.hurt_base(enemy, dmg);
        }
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
        // #35 (key 6): a random river objective comes back after a rolled time.
        if drop == Drop::Super && self.luck[1] == 1 {
            let (from, to) = (self.n(P::RiverFrom), self.n(P::RiverTo).max(self.n(P::RiverFrom)));
            self.monsters[m].respawn = from + self.rand() * (to - from);
        }
        let who = by.map_or("someone".to_string(), |b| if b == YOU { "YOU".into() } else { self.describe(b) });
        let what = match drop {
            Drop::Heal => "heal camp",
            Drop::Shield => "shield camp",
            Drop::Super | Drop::MedPack(_) => "river objective",
        };
        let msg = format!("{who} cleared the {what}");
        println!("t={:.1}s {msg}", self.t);
        self.say(msg);
        match drop {
            Drop::Heal | Drop::Shield | Drop::MedPack(_) => self.pickups.push(Pickup { pos: at.with_y(0.5), drop, life: 30.0 }),
            Drop::Super => {
                // #35 (key 6): fixed, a super crate (3 crates with super crates off). Random,
                // a roll: 40% that, 30% 3 crates, 20% a super crate and 2 crates, 10% a
                // megacrate, thrown out in random directions.
                let random = self.luck[1] == 1;
                let base = if self.super_mode > 0 { vec![Kind::Super] } else { vec![Kind::Normal; 3] };
                let roll = if random { self.rand() } else { 0.0 };
                let kinds = match roll {
                    r if r < 0.4 => base,
                    r if r < 0.7 => vec![Kind::Normal; 3],
                    r if r < 0.9 => vec![if self.super_mode > 0 { Kind::Super } else { Kind::Normal }, Kind::Normal, Kind::Normal],
                    _ if self.mega > 0 => vec![Kind::Mega],
                    _ => vec![Kind::Normal; 4],
                };
                let worth: usize = kinds.iter().map(|&k| self.worth(k)).sum();
                if let Some(t) = by.map(|b| self.actors[b].team) {
                    self.track.river_cleared[t] += 1;
                    self.track.river_crates[t] += worth as u32;
                }
                if random {
                    let what = match kinds.as_slice() {
                        [Kind::Mega] => "a MEGACRATE".to_string(),
                        [Kind::Super] => "a super crate".to_string(),
                        k if k.contains(&Kind::Super) => format!("a super crate and {} crates", k.len() - 1),
                        k => format!("{} crates", k.len()),
                    };
                    self.say(format!("the river objective dropped {what}"));
                }
                for (k, &kind) in kinds.iter().enumerate() {
                    let id = self.spawn_crate(kind, at + Vec3::Y * 1.0);
                    let (a, v) = if random { (self.rand() * std::f32::consts::TAU, 2.0 + self.rand() * 4.0) } else { (k as f32 * 2.1, 2.0) };
                    if let Some(c) = self.crate_mut(id) {
                        c.vel = Vec3::new(a.cos() * v, 6.0, a.sin() * v);
                    }
                }
            }
        }
    }
}

/// Movement speed scale: slowed in a super tank's slow zone.
fn slow_factor(a: &Actor) -> f32 {
    if a.slow > 0.0 { 0.6 } else { 1.0 }
}

/// #42 (key 5): the small map can play fewer per team; every other map plays the panel's.
fn team_size_for(tune: &Tune, map: usize) -> usize {
    let n = tune.c(P::TeamSize);
    if map == SMALL_MAP { n.min(5 - tune.c(P::SmallTeam)) } else { n }
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
// #36: the fake latency. The sim is the server; you see it late, except what you predict.
// ---------------------------------------------------------------------------------------

/// A crate you just threw, set down, punted or kicked loose: with prediction on, you see it
/// at once (and while it moves) instead of a round trip late.
fn mark_mine(s: &mut Sim, id: Option<u32>) {
    let until = s.t + s.view_delay() + 0.05;
    if s.acts_predicted()
        && let Some(c) = id.and_then(|id| s.crate_mut(id))
    {
        c.pred_mine = true;
        c.pred_until = until;
    }
}

/// Where you see crate `c`: where it really is if you predict it, in your hands once you
/// know it's there, else as the server had it one delay ago. None while you can't see it
/// yet.
fn crate_view(s: &Sim, c: &Crate) -> Option<Seen> {
    let now = s.seen_now(c);
    let d = s.view_delay();
    if d <= 0.0 || !s.lagged() || c.pred_until > s.t || now.mine && s.t >= c.mine_at {
        return Some(now);
    }
    let you = s.actors[YOU].pos;
    // A predicted grab the server turned down: in your hands until the answer arrives.
    if let Some(g) = &s.grab
        && g.predicted
        && !g.ok
        && g.id == c.id
    {
        let lift = ((s.t - g.at) / 0.1).min(1.0);
        return Some(Seen { pos: g.seen.lerp(you + Vec3::Y * HEAD, lift), mine: true, loose: false, ..now });
    }
    let mut v = seen(&c.hist, s.t - d)?;
    if v.mine {
        v.pos = you + v.off;
    }
    Some(v)
}

fn ghost_view(s: &Sim, g: &Ghost) -> Option<Seen> {
    let mut v = seen(&g.hist, s.t - s.view_delay())?;
    if v.mine {
        v.pos = s.actors[YOU].pos + v.off;
    }
    Some(v)
}

/// The nearest crate you see loose within reach: what a grab goes for.
fn seen_target(s: &Sim) -> Option<(u32, Vec3)> {
    let reach = s.reach();
    let you = s.actors[YOU].pos;
    let live = s.crates.iter().filter(|c| c.kind.builds()).filter_map(|c| crate_view(s, c).map(|v| (c.id, v)));
    let ghosts = s.ghosts.iter().filter(|g| g.kind.builds()).filter_map(|g| ghost_view(s, g).map(|v| (g.id, v)));
    live.chain(ghosts)
        .filter(|(_, v)| v.loose && !v.mine && v.pos.y - you.y < 3.0)
        .map(|(id, v)| (id, v.pos, v.pos.xz().distance(you.xz())))
        .filter(|x| x.2 <= reach + HALF)
        .min_by(|a, b| a.2.total_cmp(&b.2))
        .map(|x| (x.0, x.1))
}

/// You press grab. You pick the crate from what you see. The server judges the request
/// when it arrives (your character is predicted, so that's where you are now), against where
/// things really are, or with rewind on, against what you saw (capped). The answer reaches
/// you one round trip after you pressed.
fn request_grab(s: &mut Sim) {
    if s.grab.is_some() {
        return;
    }
    let Some((id, seen_pos)) = seen_target(s) else { return };
    let you = s.actors[YOU].pos;
    let team = s.actors[YOU].team;
    let reach = s.reach();
    let steal = pad_contains(pads()[1 - team], seen_pos);
    let back = s.view_delay().min(s.n(P::RewindCap) / 1000.0);
    let rewind = s.tv[P::Rewind as usize] as usize == 1;
    let in_reach = |p: Vec3| p.xz().distance(you.xz()) <= reach + HALF && p.y - you.y < 3.0;
    let (ok, why, rewound) = match s.crates.iter().find(|c| c.id == id) {
        None => (false, "it was gone (kicked open)", false),
        Some(c) if c.loose() && in_reach(c.pos) => (true, "", false),
        Some(c) if rewind && seen(&c.hist, s.t - back).is_some_and(|v| v.loose && !v.mine && in_reach(v.pos)) => (true, "", true),
        Some(c) if c.locked => (false, "it had locked in", false),
        Some(c) => match c.state {
            CrateState::Carried { by, .. } if s.actors[by].team == team => (false, "a teammate took it first", false),
            CrateState::Carried { .. } => (false, "an enemy took it first", false),
            CrateState::Free => (false, "it had moved out of reach", false),
        },
    };
    let t = s.t;
    let rtt = s.rtt();
    let predicted = s.grabs_predicted() && rtt > 0.0;
    if ok {
        if rewound {
            s.stats().rewound += 1;
            // Judged on what you saw: it comes out of whoever took it since, and a crate that
            // locked in since comes loose.
            let mut from = None;
            if let Some(c) = s.crate_mut(id) {
                if let CrateState::Carried { by, .. } = c.state {
                    from = Some(by);
                }
                c.state = CrateState::Free;
                c.locked = false;
                c.pad_t = 0.0;
            }
            if let Some(by) = from {
                let a = &mut s.actors[by];
                if a.carrying == Some(id) {
                    a.carrying = None;
                }
                a.load.retain(|&k| k != id);
                s.stats().yanked += 1;
                let who = s.describe(by);
                println!("t={t:.1}s rewind: your grab pulled a crate out of {who}'s hands");
            }
        }
        s.take(YOU, id);
        if let Some(c) = s.crate_mut(id) {
            c.mine_at = if predicted { t } else { t + rtt };
            c.pred_until = 0.0;
            c.pred_mine = false;
        }
    }
    let st = s.stats();
    if steal {
        st.steals += 1;
        st.steals_lost += !ok as u32;
    } else {
        st.grabs += 1;
        st.grabs_lost += !ok as u32;
    }
    if !ok {
        match why {
            "it had locked in" => st.locked += 1,
            "it had moved out of reach" => st.moved += 1,
            "it was gone (kicked open)" => st.gone += 1,
            _ => st.taken += 1,
        }
    }
    s.grab = Some(GrabReq { id, at: t, answer_at: t + rtt, seen: seen_pos, ok, steal, why, predicted });
}

/// A pad's kick progress as you see it: your own team's at once with prediction on, else
/// one round trip late (a replicated timer, not interpolated).
fn kick_view(s: &Sim, team: usize) -> f32 {
    let own = team == s.actors[YOU].team;
    let d = if own && s.acts_predicted() { 0.0 } else { s.rtt() };
    if d <= 0.0 || !s.lagged() {
        return s.pad_kick[team];
    }
    let want = s.t - d;
    s.kick_hist.iter().find(|h| h.0 >= want).map_or(s.pad_kick[team], |h| h.1[team])
}

/// After each tick: the grab's answer arrives, predicted crates stay predicted while they
/// move, ghosts leave once the news arrives, and every crate and pad is sampled.
fn net_step(s: &mut Sim) {
    let t = s.t;
    let d = s.view_delay();
    let rtt = s.rtt();
    if s.grab.as_ref().is_some_and(|g| t >= g.answer_at) {
        let g = s.grab.take().expect("checked");
        if g.ok {
            // Waited for the server: the crate lifts from where you saw it, now.
            if !g.predicted
                && rtt > 0.0
                && let Some(c) = s.crate_mut(g.id)
                && let CrateState::Carried { by: YOU, lift_from, lift_t } = &mut c.state
            {
                *lift_from = g.seen;
                *lift_t = 0.0;
            }
        } else {
            if g.predicted {
                s.stats().snapbacks += 1;
            }
            let what = if g.steal { "STEAL" } else { "GRAB" };
            let ms = (s.view_delay() * 1000.0).round();
            let msg = format!("{what} LOST: {} (you saw it {ms:.0} ms late)", g.why);
            println!("t={t:.1}s {msg}");
            s.say(msg);
        }
    }
    for c in &mut s.crates {
        if c.pred_mine {
            let moving = c.state == CrateState::Free && !c.locked && (c.vel.length_squared() > 0.0025 || c.settling);
            if moving {
                c.pred_until = t + d + 0.05;
            } else {
                c.pred_mine = false;
            }
        }
    }
    let you = s.actors[YOU].pos;
    for c in &mut s.crates {
        let mine = matches!(c.state, CrateState::Carried { by: YOU, .. });
        c.hist.push_back(Seen { t, pos: c.pos, loose: c.loose(), mine, locked: c.locked, off: c.pos - you });
        while c.hist.front().is_some_and(|h| h.t < t - 1.0) {
            c.hist.pop_front();
        }
    }
    s.kick_hist.push_back((t, s.pad_kick));
    while s.kick_hist.front().is_some_and(|h| h.0 < t - 1.0) {
        s.kick_hist.pop_front();
    }
    // A removed crate leaves your view when the news arrives (a despawn isn't interpolated).
    let gone: Vec<Ghost> = s.ghosts.extract_if(.., |g| t - rtt >= g.gone).collect();
    for g in gone {
        s.despawn.extend(g.ent);
    }
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

fn floor_under(p: Vec3, others: impl Iterator<Item = Vec3>) -> f32 {
    let mut floor: f32 = 0.0;
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
    let mut path = vec![p];
    let dt = 1.0 / 60.0;
    for _ in 0..600 {
        let grounded = crate_motion(
            &mut p,
            &mut v,
            &mut settling,
            |q| floor_under(q, others.iter().copied()),
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
    let top = floor_under(want.extend(99.0).xzy(), free);
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
    /// Left click this frame (places a barrage or gate).
    fire_tap: bool,
    kick_held: bool,
    kick: bool,
    /// Throw your 1-crate item (Q / LB).
    toss: bool,
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
    mut rt_was: Local<bool>,
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
    c.fire_tap = mouse.just_pressed(MouseButton::Left);
    c.kick_held = keys.pressed(KeyCode::KeyF);
    c.kick = keys.just_pressed(KeyCode::KeyF);
    c.toss = keys.just_pressed(KeyCode::KeyQ);
    c.drop = if keys.just_pressed(KeyCode::KeyT) {
        Some(Kind::Normal)
    } else if keys.just_pressed(KeyCode::KeyY) {
        Some(Kind::Super)
    } else if keys.just_pressed(KeyCode::KeyU) {
        Some(Kind::Mega)
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
        // RT: some pads (often on Linux) report the trigger only as an analog value or axis,
        // never as a digital press. Count any of them past a light pull.
        let rt = pad.pressed(GamepadButton::RightTrigger2)
            || pad.get(GamepadButton::RightTrigger2).unwrap_or(0.0) > 0.3
            || pad.get(GamepadAxis::RightZ).unwrap_or(-1.0) > 0.3;
        c.fire |= rt;
        c.fire_tap |= rt && !*rt_was;
        *rt_was = rt;
        c.kick_held |= pad.pressed(GamepadButton::East);
        c.kick |= pad.just_pressed(GamepadButton::East);
        c.toss |= pad.just_pressed(GamepadButton::LeftTrigger);
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
        if sel == P::MatchLength as usize && sim.loss != ROUNDS {
            sim.match_left = tune.f(P::MatchLength);
        }
        if sel == P::RoundLength as usize && sim.loss == ROUNDS {
            sim.match_left = tune.f(P::RoundLength);
        }
        if sel == P::Set as usize {
            let i = tune.c(P::Set);
            apply_set(&mut tune.vals, i);
        }
        if sel == P::Preset as usize {
            let i = tune.c(P::Preset);
            apply_preset(&mut tune.vals, i);
        }
    }
    if keys.just_pressed(KeyCode::KeyH) {
        tune.panel = !tune.panel;
    }
    if any(KeyCode::KeyR, GamepadButton::Start) {
        sim.reset(&tune);
        sim.say("match reset");
    }
    // #42: 1-5 step through the map switch. 1 and 2 restart on the new map; 3 and 4 act
    // when a match ends; 5 restarts if the small map is on.
    let rule_keys = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5];
    for (k, key) in rule_keys.iter().enumerate() {
        if keys.just_pressed(*key) {
            if let Value::Choice { i, opts } = &mut tune.vals[k].value {
                *i = (*i + 1) % opts.len();
            }
            tune.selected = k;
            let p = &tune.vals[k];
            let msg = format!("{}: {}", &p.name[2..], fmt_value(&p.value));
            let when = if k < 2 { "restarting" } else if k == 4 { "on the small map" } else { "from the next match" };
            sim.say(format!("{msg} ({when})"));
        }
    }
    for k in [KeyCode::KeyZ, KeyCode::KeyX, KeyCode::KeyN] {
        if keys.just_pressed(k) {
            drill(&mut sim, k, keys.pressed(KeyCode::ShiftLeft));
        }
    }
    // #36: L swaps between the ping and 0 ms (the no-latency reference).
    if keys.just_pressed(KeyCode::KeyL) {
        let now = tune.c(P::Ping);
        let back = if now == 0 { tune.last_ping.max(1) } else { 0 };
        if now != 0 {
            tune.last_ping = now;
        }
        set(&mut tune.vals, P::Ping, back as f32);
        sim.rules(&tune);
        sim.say(format!("ping: {} (live)", fmt_value(&tune.vals[P::Ping as usize].value)));
    }
    if keys.just_pressed(KeyCode::KeyO) {
        sim.truth = !sim.truth;
        let msg = if sim.truth { "showing the server's truth in orange (O hides it)" } else { "truth hidden" };
        sim.say(msg);
    }
    // G: red puts up a bunker round its pad now (Shift: a super one), to try raiding it.
    if keys.just_pressed(KeyCode::KeyG) {
        let gold = keys.pressed(KeyCode::ShiftLeft);
        match sim.capstone(1) {
            Some(c) => sim.say(format!("red's all 8 is locked: {c}")),
            None => {
                let lead = team_lead(&sim, 1);
                sim.apply(1, lead, Make::Bunker, gold);
                let what = format!("engineer {} (G)", Make::Bunker.label(gold));
                sim.made[1].push(what.clone());
                sim.say(format!("red put up a {what}: try raiding it"));
            }
        }
    }
    // V: your class's all 8 now (Shift: super), to try a vehicle without building 8.
    if keys.just_pressed(KeyCode::KeyV) {
        let gold = keys.pressed(KeyCode::ShiftLeft);
        match sim.capstone(0) {
            Some(c) => sim.say(format!("your all 8 is locked: {c}")),
            None if !sim.actors[YOU].alive() => sim.say("you're down"),
            None => {
                let make = item_of(Slot::All8, sim.actors[YOU].class, sim.sets);
                sim.apply(0, YOU, make, gold);
                let what = format!("{} {} (V)", sim.actors[YOU].class.name(), make.label(gold));
                sim.made[0].push(what.clone());
                sim.say(format!("you got a {what}"));
            }
        }
    }
    // K: red's base drops to 1 hp (Shift: yours), to try a base kill quickly.
    if keys.just_pressed(KeyCode::KeyK) {
        let team = if keys.pressed(KeyCode::ShiftLeft) { 0 } else { 1 };
        if sim.base_up(team) {
            sim.base_hp[team] = 1.0;
            sim.say(format!("{} base at 1 hp: one hit knocks it down", ["your", "red's"][team]));
        } else {
            sim.say("that base isn't standing (or the rule has no base)");
        }
    }
    if keys.just_pressed(KeyCode::KeyC) {
        sim.your_class = sim.your_class.next();
        sim.actors[YOU].class = sim.your_class;
        let c = sim.your_class.name();
        sim.say(format!("you're a {c} now"));
    }
    if keys.just_pressed(KeyCode::Enter) {
        let line = format!("playing {} ({}) | {}", MAP_NAMES[sim.map_i], ["mirror", "rotational"][sim.rotate as usize], rules_desc(&tune));
        println!("verdict: {line}");
        tune.verdicts.push(line);
        sim.say("picked this map and these switches (P prints them)");
    }
    if keys.just_pressed(KeyCode::KeyP) {
        println!("# launch-maps settings");
        for p in &tune.vals {
            println!("{:<20} = {}", p.name, fmt_value(&p.value));
        }
        if !tune.verdicts.is_empty() {
            println!("# verdicts");
            for v in &tune.verdicts {
                println!("- {v}");
            }
        }
        println!("# maps played, in order");
        println!("- {}", sim.played_maps.iter().map(|&m| MAP_SHORT[m]).collect::<Vec<_>>().join(", "));
        if !sim.log.is_empty() {
            println!("# results");
            for l in &sim.log {
                println!("- {l}");
            }
        }
        sim.say("printed settings, verdicts and results to the terminal");
    }
}

/// #36: the drill bench (`CRATE_BENCH=z|x|X|n`, `CRATE_REACTION=<ms>`, `CRATE_TRIALS=<n>`).
#[derive(Default)]
struct Bench {
    key: Option<KeyCode>,
    shift: bool,
    name: String,
    reaction: f32,
    /// This trial's reaction.
    react: f32,
    trials: u32,
    /// When the next trial starts (0 while one runs), and when this one started.
    next: f32,
    start: f32,
    target: u32,
    seen_at: Option<f32>,
    sent: bool,
    counts: bool,
    done: u32,
    won: u32,
    /// Lost: the answer said no, or you never saw it loose in time to press.
    denied: u32,
    missed: u32,
    rewound: u32,
    yanked: u32,
}

fn bench_step(s: &mut Sim) {
    let Some(mut b) = s.bench.take() else { return };
    let t = s.t;
    s.actors[YOU].invuln = 1.0;
    if b.next > 0.0 {
        if t >= b.next && s.actors[YOU].alive() && s.actors[YOU].carrying.is_none() && s.grab.is_none() {
            let before = s.next_id;
            // Spread the race: the rival starts within 1 m either way of the set distance,
            // and you react within 75 ms either way of the set reaction.
            let k = P::DrillRival as usize;
            let base = s.tv[k];
            s.tv[k] = (base + (s.rand() - 0.5) * 2.0).max(0.0);
            b.react = b.reaction + (s.rand() - 0.5) * 0.15;
            drill(s, b.key.unwrap_or(KeyCode::KeyN), b.shift);
            s.tv[k] = base;
            // Z and Shift+X need a rival; without a free one the trial doesn't count.
            let needs = b.key == Some(KeyCode::KeyZ) || b.shift;
            b.counts = !needs || s.actors.iter().any(|a| a.rush.is_some_and(|r| r.0 == before));
            b.target = before;
            b.start = t;
            b.next = 0.0;
            b.seen_at = None;
            b.sent = false;
        }
        s.bench = Some(b);
        return;
    }
    if b.seen_at.is_none() && seen_target(s).is_some_and(|(id, _)| id == b.target) {
        b.seen_at = Some(t);
    }
    let (rewound, yanked) = s.net.last().map_or((0, 0), |(_, st)| (st.rewound, st.yanked));
    if !b.sent && b.seen_at.is_some_and(|at| t >= at + b.react) {
        if seen_target(s).is_some_and(|(id, _)| id == b.target) {
            request_grab(s);
            b.sent = s.grab.is_some();
        }
    }
    let answered = b.sent && s.grab.is_none();
    let timed_out = !b.sent && t > b.start + 4.0;
    if answered || timed_out {
        let won = s.actors[YOU].carrying == Some(b.target);
        if b.counts {
            b.done += 1;
            b.won += won as u32;
            b.denied += (answered && !won) as u32;
            b.missed += timed_out as u32;
            b.rewound += rewound;
            b.yanked += yanked;
        }
        if let Some(st) = s.net.last_mut() {
            st.1.rewound = 0;
            st.1.yanked = 0;
        }
        // Clear the drill crate and anyone's hands, and go again.
        let gone: Vec<Crate> = s.crates.extract_if(.., |c| c.id == b.target).collect();
        for c in gone {
            s.despawn.extend(c.ent);
        }
        for a in &mut s.actors {
            if a.carrying == Some(b.target) {
                a.carrying = None;
            }
            a.rush = None;
        }
        b.next = t + 1.0;
        if b.done >= b.trials {
            let pct = |n: u32| 100.0 * n as f32 / b.done.max(1) as f32;
            println!(
                "bench drill={} ping={:.0} reach={:.1} rewind={} reaction={:.0} trials={} won={:.0}% denied={:.0}% missed={:.0}% rewound={} yanked={}",
                b.name,
                s.rtt() * 1000.0,
                s.reach(),
                s.tv[P::Rewind as usize] as usize,
                b.reaction * 1000.0,
                b.done,
                pct(b.won),
                pct(b.denied),
                pct(b.missed),
                b.rewound,
                b.yanked,
            );
            std::process::exit(0);
        }
    }
    s.bench = Some(b);
}

/// #36 drills, to play the contested cases on demand.
/// Z: a crate lands just in front of you, and a red rival rushes it from a few metres out.
/// X: you're beside red's pad, with a crate on it about to lock in. Shift+X: a red defender
/// rushes that crate instead. N: a crate slides past in front of you.
fn drill(s: &mut Sim, kind: KeyCode, shift: bool) {
    if !s.actors[YOU].alive() || s.actors[YOU].carrying.is_some() {
        s.say("drills need you up and empty-handed");
        return;
    }
    let rival = |s: &Sim| {
        (0..s.actors.len())
            .filter(|&j| s.actors[j].team == 1 && s.actors[j].alive() && s.actors[j].tank.is_none() && s.actors[j].carrying.is_none())
            .min_by(|&a, &b| s.actors[a].pos.distance(s.actors[YOU].pos).total_cmp(&s.actors[b].pos.distance(s.actors[YOU].pos)))
    };
    let you = s.actors[YOU].pos;
    let facing = s.actors[YOU].facing.with_y(0.0).normalize_or(Vec3::X);
    match kind {
        KeyCode::KeyZ => {
            let at = (you + facing * 1.4).with_y(HALF);
            let id = s.spawn_crate(Kind::Normal, at);
            match rival(s) {
                Some(j) => {
                    let from = at + facing * (s.n(P::DrillRival) + s.reach() + HALF);
                    let a = &mut s.actors[j];
                    a.pos = from.with_y(0.0);
                    a.vel = Vec3::ZERO;
                    a.rush = Some((id, s.t + 3.0));
                    s.say("SCRAMBLE: a red rival rushes the crate in front of you. Grab it first");
                }
                None => s.say("no red player free to rush it; grab it anyway"),
            }
        }
        KeyCode::KeyX => {
            let pad = pads()[1];
            let front = pad + Vec2::X * toward_centre(1) * (PAD_HALF + 0.6);
            let a = &mut s.actors[YOU];
            a.pos = front.extend(0.0).xzy();
            a.vel = Vec3::ZERO;
            let cells = pad_cells(1);
            let cell = cells
                .into_iter()
                .min_by(|x, y| {
                    let n = |c: Vec2| s.crates.iter().filter(|k| k.pos.xz().distance(c) < 0.2).count();
                    n(*x).cmp(&n(*y)).then(x.distance(front).total_cmp(&y.distance(front)))
                })
                .unwrap_or(pad);
            let at = s.target_at(cell);
            let id = s.spawn_crate(Kind::Normal, at);
            let lock = s.n(P::LockInTime) - s.n(P::DrillLock);
            if let Some(c) = s.crate_mut(id) {
                c.pad_t = lock.max(0.0);
            }
            if shift {
                match rival(s) {
                    Some(j) => {
                        let from = (at + Vec3::X * toward_centre(1) * -(s.n(P::DrillRival) + s.reach() + HALF)).with_y(0.0);
                        let a = &mut s.actors[j];
                        a.pos = from;
                        a.vel = Vec3::ZERO;
                        a.rush = Some((id, s.t + 3.0));
                        s.say("STEAL vs a DEFENDER: red rushes back to the crate on its pad");
                    }
                    None => s.say("no red player free to defend"),
                }
            } else {
                s.say(format!("STEAL vs LOCK-IN: the crate on red's pad locks in {:.1} s after it lands", s.n(P::DrillLock)));
            }
        }
        _ => {
            // Friction stops a sliding crate in v^2 / 2f: start it where it passes your front
            // at half its speed.
            let v = s.n(P::DrillSlide);
            let side = Vec3::new(-facing.z, 0.0, facing.x);
            let at = (you + facing * 1.3 - side * 0.75 * v * v / (2.0 * SLIDE_FRICTION)).with_y(HALF);
            let id = s.spawn_crate(Kind::Normal, at);
            if let Some(c) = s.crate_mut(id) {
                c.vel = side * v;
            }
            s.say("SLIDER: a crate slides past in front of you. Grab it as it passes");
        }
    }
}

/// Red marks a barrage where you stand (key B), unless its all 8 is locked.
fn red_barrage(sim: &mut Sim, gold: bool) {
    if let Some(c) = sim.capstone(1) {
        return sim.say(format!("red's all 8 is locked: {c}"));
    }
    let (lead, at, sparse) = (team_lead(sim, 1), sim.actors[YOU].pos.with_y(0.0), sim.sets[3] == 4);
    start_barrage(sim, 1, lead, at, gold, sparse);
    sim.made[1].push(format!("soldier {} (B)", Make::Barrage(sparse).label(gold)));
}

/// Red shuts its mid lane and a side lane with gate walls (key J), unless its all 8 is locked.
fn red_gates(sim: &mut Sim, gold: bool) {
    let lead = team_lead(sim, 1);
    match sim.capstone(1) {
        Some(c) => sim.say(format!("red's all 8 is locked: {c}")),
        None if !sim.map => sim.say("gates shut lanes: only on the lanes map"),
        None if !sim.actors[lead].alive() => sim.say("red's first player is down"),
        None => {
            sim.actors[lead].mark = Some(Mark { make: Make::Gate, gold, left: 2, life: 5.0 });
            // Red's half is +x: its mid-lane and a side-lane choke.
            for at in [Vec3::new(26.0, 0.0, 0.0), Vec3::new(26.0, 0.0, 21.5)] {
                place_mark(sim, lead, at);
            }
            sim.made[1].push(format!("engineer {} (J)", Make::Gate.label(gold)));
        }
    }
}

// ---------------------------------------------------------------------------------------
// Simulation step
// ---------------------------------------------------------------------------------------

fn step(time: Res<Time>, controls: Res<Controls>, tune: Res<Tune>, mut sim: ResMut<Sim>) {
    let s = &mut *sim;
    if team_size_for(&tune, s.map_i) != s.team_size
        || tune.c(P::Map) != s.tune_map
        || tune.c(P::Symmetry) != s.tune_sym
        || tune.c(P::Loss) != s.loss
    {
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
    // #36: grab stats are kept for each setup of the latency switch.
    let key = LAT_PS.iter().map(|&p| fmt_value(&tune.vals[p as usize].value)).collect::<Vec<_>>().join(" | ");
    if s.net.len() == 1 && s.net[0].0.is_empty() {
        s.net[0].0 = key.clone();
    }
    if s.net.last().is_none_or(|(k, _)| *k != key) {
        match s.net.iter().position(|(k, _)| *k == key) {
            Some(i) => {
                let e = s.net.remove(i);
                s.net.push(e);
            }
            None => s.net.push((key, NetStats::default())),
        }
    }
    for k in 0..tune.c(P::TimeScale).max(1) {
        tick(s, if k == 0 { &controls } else { &quiet }, &tune, dt);
    }
}

fn tick(s: &mut Sim, c: &Controls, tune: &Tune, dt: f32) {
    if s.over > 0.0 {
        s.over -= dt;
        if s.over <= 0.0 {
            if s.next_round {
                s.round += 1;
                s.reset_round(tune);
                s.say(format!("ROUND {}: rounds {}-{}", s.round, s.wins[0], s.wins[1]));
            } else {
                s.reset(tune);
            }
        }
        return;
    }
    s.t += dt;
    s.rules(tune);
    let elapsed = tune.f(P::MatchLength) - s.match_left;
    // Bases: a knocked-out pad rebuilds when its timer runs out.
    for team in 0..2 {
        s.base_hit_t[team] += dt;
        if s.base_down[team] > 0.0 && s.base_down[team].is_finite() {
            s.base_down[team] -= dt;
            if s.base_down[team] <= 0.0 {
                s.base_down[team] = 0.0;
                s.base_hp[team] = s.n(P::BaseHp);
                s.say(format!("{} base rebuilt: its pad works again", ["BLUE", "RED"][team]));
            }
        }
    }
    if s.all8_at.is_some_and(|at| elapsed >= at) && s.capstone(0).is_none() && s.actors[YOU].alive() && s.actors[YOU].carrying.is_none() {
        s.all8_at = None;
        let make = item_of(Slot::All8, s.your_class, s.sets);
        s.apply(0, YOU, make, false);
        s.made[0].push(format!("{} {} (CRATE_ALL8)", s.your_class.name(), make.name()));
    }
    if s.red_barrage_at.is_some_and(|at| elapsed >= at) {
        s.red_barrage_at = None;
        red_barrage(s, false);
    }
    if s.red_gates_at.is_some_and(|at| elapsed >= at) {
        s.red_gates_at = None;
        red_gates(s, false);
    }
    s.ally_stacks = tune.c(P::AllyStacks) == 1;
    s.you_play = tune.c(P::Seat) == 0;
    let t = s.t;
    for i in 0..s.actors.len() {
        let a = &mut s.actors[i];
        if a.alive() {
            a.played += dt;
        }
        a.fire_cd -= dt;
        a.alt_cd -= dt;
        a.stun -= dt;
        a.slow -= dt;
        a.slow_immune -= dt;
        a.invuln -= dt;
        a.root -= dt;
        a.reveal -= dt;
        a.adren -= dt;
        if a.regen_t > 0.0 && a.alive() {
            a.regen_t -= dt;
            a.hp = (a.hp + a.regen * dt).min(100.0);
        }
        if let Some(m) = &mut a.mark {
            m.life -= dt;
            if m.life <= 0.0 {
                a.mark = None;
                if i == YOU {
                    s.say("your all 8 ran out of time before you placed it");
                }
            }
        }
        let a = &mut s.actors[i];
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
        bench_step(s);
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
    lobs_step(s, dt);
    barrages_step(s, dt);
    walls_step(s, dt);
    vehicles_step(s, dt);
    mines_step(s, dt);
    monsters_step(s, dt);
    pickups_step(s, dt);
    refill(s, dt);
    mega_step(s, dt);
    marked_step(s, dt);
    for a in &mut s.actors {
        a.hist.push_back((t, a.pos));
        while a.hist.front().is_some_and(|h| h.0 < t - 1.0) {
            a.hist.pop_front();
        }
    }
    net_step(s);
    if let Some((w, how)) = s.match_end.take() {
        return end_match(s, tune, w, &how);
    }
    if s.over <= 0.0 {
        judge(s, tune, dt);
    }
}

/// #15: tracks the lead, and ends the match (or round) on elimination, mercy, time, or the
/// end of overtime.
fn judge(s: &mut Sim, tune: &Tune, dt: f32) {
    let (b, r) = (s.score(0), s.score(1));
    let lead = match b.cmp(&r) {
        std::cmp::Ordering::Greater => Some(0),
        std::cmp::Ordering::Less => Some(1),
        std::cmp::Ordering::Equal => None,
    };
    let gap = b.abs_diff(r);
    let up = [0, 1].map(|t| s.actors.iter().filter(|a| a.team == t && a.alive()).count() as f32);
    let length = tune.f(P::MatchLength);
    // #42: time spent carrying, and bots stood still away from the bases.
    let carrying = [0, 1].map(|t| s.actors.iter().filter(|a| a.team == t && a.alive() && a.carrying.is_some()).count() as f32);
    s.still.resize(s.actors.len(), (Vec2::ZERO, 0.0));
    let base_x = arena().x - 10.0;
    let mut stuck = 0.0;
    for (i, a) in s.actors.iter().enumerate() {
        let st = &mut s.still[i];
        if !a.alive() || a.pos.xz().distance(st.0) > 1.5 {
            *st = (a.pos.xz(), 0.0);
            continue;
        }
        st.1 += dt;
        if st.1 > 5.0 && a.pos.x.abs() < base_x && (i != YOU || !s.you_play) {
            stuck += dt;
        }
    }
    let tr = &mut s.track;
    tr.time += dt;
    tr.stuck += stuck;
    for t in 0..2 {
        tr.up[t] += up[t] * dt;
        tr.carry_t[t] += carrying[t] * dt;
    }
    // #34: who leads right now (ties count), and since when; the score at each quarter.
    if lead != tr.lead_now || tr.time <= dt * 1.5 {
        tr.lead_now = lead;
        tr.lead_since = tr.time;
    }
    if (b, r) != tr.scores {
        tr.scores = (b, r);
        tr.last_score = tr.time;
    }
    if tr.quarters.len() < 4 && tr.time >= length * (tr.quarters.len() + 1) as f32 / 4.0 {
        tr.quarters.push((b, r));
    }
    if let Some(l) = lead {
        if tr.leader.is_some_and(|x| x != l) {
            tr.lead_changes += 1;
        }
        tr.leader = Some(l);
        tr.max_lead[l] = tr.max_lead[l].max(gap);
        if gap >= 10 {
            tr.hopeless[1 - l] += dt;
        }
    }
    let now = s.track.time;
    for i in 0..s.actors.len() {
        let a = &s.actors[i];
        if a.open && a.seat_at > now - dt && a.seat_at <= now {
            let who = if i == YOU { "you".to_string() } else { format!("{} player {}", ["blue", "red"][a.team], a.slot + 1) };
            let n = s.room(a.team);
            s.say(format!(
                "SEAT FILLED at {}:{:02}: {who} is waiting for a player recipe ({} seat{} ready on {}'s pad)",
                now as u32 / 60,
                now as u32 % 60,
                n,
                if n == 1 { "" } else { "s" },
                ["blue", "red"][a.team]
            ));
        }
    }
    let finish = |s: &mut Sim, w: Option<usize>, how: &str| {
        if s.loss == ROUNDS {
            s.end_round(w, how);
            if let Some((w, how)) = s.match_end.take() {
                end_match(s, tune, w, &how);
            }
        } else {
            end_match(s, tune, w, how);
        }
    };
    // Permanent rule: a team with no base and nobody left is eliminated.
    if s.loss == PERMANENT
        && let Some(t) = (0..2).find(|&t| s.base_down[t].is_infinite() && s.players(t) == 0)
    {
        return finish(s, Some(1 - t), "elimination");
    }
    if s.mercy && gap >= s.n(P::MercyGap) as usize {
        if s.mercy_t == 0.0 {
            s.track.mercy_from = s.track.time;
        }
        s.mercy_t += dt;
        if s.mercy_t >= s.n(P::MercyHold) {
            return finish(s, lead, "mercy");
        }
    } else {
        s.mercy_t = 0.0;
    }
    s.match_left -= dt;
    if s.match_left > 0.0 {
        return;
    }
    if s.track.buzzer.is_none() {
        s.track.buzzer = Some((b, r));
    }
    if !s.overtime {
        return finish(s, lead, "time");
    }
    // Overtime: on while it's tied, or while the trailing team carries a crate or stands at
    // the leader's pad. It ends a second after they stop, or at the cap.
    let contest = |t: usize| {
        let pad = pads()[1 - t];
        s.actors.iter().any(|a| a.team == t && a.alive() && (a.carrying.is_some() || !a.load.is_empty() || a.pos.xz().distance(pad) < 5.0))
    };
    let on = lead.is_none_or(|l| contest(1 - l));
    if s.ot == 0.0 {
        s.say("OVERTIME: it runs while the trailing team carries a crate or contests the leader's pad");
    }
    s.ot += dt;
    s.track.overtime += dt;
    s.ot_grace = if on { 1.0 } else { s.ot_grace - dt };
    if s.ot_grace <= 0.0 || s.ot >= s.n(P::OvertimeCap) {
        // Nobody contesting at the buzzer: it ended on time.
        finish(s, lead, if s.ot > dt * 1.5 { "overtime" } else { "time" });
    }
}

/// #15: a megacrate drops every so often, while none is out: in the river, or (pulled) on
/// the trailing team's side, further the bigger the gap.
fn mega_step(s: &mut Sim, dt: f32) {
    if s.mega == 0 || s.crates.iter().any(|c| c.kind == Kind::Mega) || s.marked.iter().any(|m| m.1 == Kind::Mega) {
        return;
    }
    s.mega_t -= dt;
    if s.mega_t > 0.0 {
        return;
    }
    s.mega_t = s.mega_gap();
    let (b, r) = (s.score(0) as f32, s.score(1) as f32);
    let trailing = if b < r { Some(0) } else if r < b { Some(1) } else { None };
    let x = match (s.mega, trailing) {
        (2, Some(t)) => {
            let pull = s.n(P::MegaPull).min(arena().x * 0.6) * ((b - r).abs() / 10.0).min(1.0);
            -toward_centre(t) * pull
        }
        _ => 0.0,
    };
    let mut p = Vec2::new(x, (s.rand() - 0.5) * arena().y * 1.2);
    for _ in 0..30 {
        if !blocked(p, 0.8) {
            break;
        }
        p = Vec2::new(x + (s.rand() - 0.5) * 4.0, (s.rand() - 0.5) * arena().y * 1.4);
    }
    let p = (p / CRATE).round() * CRATE;
    let side = if p.x < -3.0 { 0 } else if p.x > 3.0 { 2 } else { 1 };
    let worth = s.n(P::MegaWorth);
    let wher = ["on blue's side", "in the middle", "on red's side"][side];
    // #35 (key 7): marked ahead, it shows where it will land first.
    if s.luck[2] == 2 {
        let warn = s.n(P::MegaWarn);
        s.marked.push((Vec3::new(p.x, 0.0, p.y), Kind::Mega, warn));
        s.say(format!("a MEGACRATE (worth {worth:.0}) is marked {wher}: lands in {warn:.0} s"));
    } else {
        drop_crate(s, Kind::Mega, p);
    }
}

/// #35: a crate drops from the sky now (a marked one, once its time is up).
fn drop_crate(s: &mut Sim, kind: Kind, p: Vec2) {
    s.spawn_crate(kind, Vec3::new(p.x, 10.0, p.y));
    if kind == Kind::Mega {
        let side = if p.x < -3.0 { 0 } else if p.x > 3.0 { 2 } else { 1 };
        s.track.mega_dropped[side] += 1;
        let t = s.track.time;
        s.track.mega_at.push(t);
        let worth = s.n(P::MegaWorth);
        s.say(format!("a MEGACRATE (worth {worth:.0}) dropped {}", ["on blue's side", "in the middle", "on red's side"][side]));
    }
}

/// #35: drops marked on the floor count down, then fall.
fn marked_step(s: &mut Sim, dt: f32) {
    for m in &mut s.marked {
        m.2 -= dt;
    }
    let due: Vec<(Vec3, Kind)> = s.marked.iter().filter(|m| m.2 <= 0.0).map(|m| (m.0, m.1)).collect();
    s.marked.retain(|m| m.2 > 0.0);
    for (p, kind) in due {
        drop_crate(s, kind, p.xz());
    }
}

/// #35 (key 5): fixed spawns take mirrored spots in turn, blue's half then red's, so both
/// halves get the same. `span` is the spawn area's half size.
fn fixed_spot(s: &mut Sim, span: Vec2) -> Vec2 {
    const SPOTS: [(f32, f32); 6] = [(0.35, 0.0), (0.75, 0.55), (0.35, -0.75), (0.75, -0.3), (0.35, 0.75), (0.75, 0.25)];
    for _ in 0..12 {
        let k = s.spawn_turn;
        s.spawn_turn += 1;
        let (fx, fz) = SPOTS[(k / 2) % SPOTS.len()];
        let side = if k % 2 == 0 { -1.0 } else { 1.0 };
        let p = (Vec2::new(side * fx * span.x, fz * span.y) / CRATE).round() * CRATE;
        let taken = s.marked.iter().any(|m| m.0.xz().distance(p) < CRATE) || s.crates.iter().any(|c| c.pos.xz().distance(p) < CRATE);
        if !blocked(p, 1.0) && !taken {
            return p;
        }
    }
    open_spot(|| s.rand(), span)
}

fn end_match(s: &mut Sim, tune: &Tune, winner: Option<usize>, how: &str) {
    let (b, r) = (s.score(0), s.score(1));
    if s.loss != ROUNDS {
        s.add_round_stats();
    }
    let won = winner.map_or("draw".to_string(), |w| format!("{} wins", ["BLUE", "RED"][w]));
    let tr = &s.track;
    let pair = |x: [u32; 2]| format!("{}-{}", x[0], x[1]);
    // How far behind the winner (or, in a draw, either team) ever was, and how long the
    // loser spent hopeless (behind by 10+).
    let (trailed, hopeless) = match winner {
        Some(w) => (tr.max_lead[1 - w], tr.hopeless[1 - w]),
        None => (tr.max_lead[0].max(tr.max_lead[1]), tr.hopeless[0].max(tr.hopeless[1])),
    };
    let result = format!(
        "result loss={} lockin={} spill={} mega={} ot={} mercy={} winner={} how={} score={b}-{r} rounds={}-{} time={:.0} overtime={:.0} \
         lead_changes={} max_lead={}-{} winner_trailed_by={trailed} loser_hopeless={hopeless:.0} base_dmg={:.0}-{:.0} falls={} \
         spilled={} destroyed={} mega_grabbed={} mega_dropped={}/{}/{} kills={} stolen={} raiding={:.0}-{:.0} avg_up={:.1}-{:.1} posted={:.0}-{:.0} base_shots={}-{}",
        s.loss,
        s.lock_in as u8,
        s.spill as u8,
        s.mega,
        s.overtime as u8,
        s.mercy as u8,
        winner.map_or("draw", |w| ["blue", "red"][w]),
        how.split(' ').next().unwrap_or(how),
        s.wins[0],
        s.wins[1],
        tr.time,
        tr.overtime,
        tr.lead_changes,
        tr.max_lead[0],
        tr.max_lead[1],
        tr.base_dmg[0],
        tr.base_dmg[1],
        pair(tr.falls),
        pair(tr.spilled),
        pair(tr.destroyed),
        pair(tr.mega_grabbed),
        tr.mega_dropped[0],
        tr.mega_dropped[1],
        tr.mega_dropped[2],
        pair(tr.kills),
        pair(tr.stolen),
        tr.raiding[0],
        tr.raiding[1],
        tr.up[0] / tr.time.max(1.0),
        tr.up[1] / tr.time.max(1.0),
        tr.posted[0],
        tr.posted[1],
        tr.base_shots[0],
        tr.base_shots[1],
    );
    println!("{result}");
    // #34: pacing.
    let len = tr.time;
    let timer = tune.f(P::MatchLength);
    let quiet = len - tr.last_fight;
    let on_fight = tr.fights.iter().any(|&t| t >= len - tune.f(P::FightWindow));
    let last_min = tr.fights.iter().filter(|&&t| t >= len - 60.0).count();
    let decided = if winner.is_some() && tr.lead_now == winner { tr.lead_since } else { len };
    let buzzer_gap = tr.buzzer.map_or(-1, |(b, r)| b.abs_diff(r) as i32);
    let quarters = tr.quarters.iter().map(|(b, r)| format!("{b}-{r}")).collect::<Vec<_>>().join("/");
    let mercy_from = if how.starts_with("mercy") { tr.mercy_from } else { -1.0 };
    let late: Vec<&Actor> = s.actors.iter().filter(|a| a.open).collect();
    let late_desc = late
        .iter()
        .map(|a| {
            let who = format!("{}{}", ["b", "r"][a.team], a.slot + 1);
            match a.entered {
                Some(e) => format!("{who}:seat{:.0}/in{:.0}/left{:.0}/alive{:.0}", a.seat_at, e, len - e, a.played),
                None => format!("{who}:seat{:.0}/never", a.seat_at),
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    let entered: Vec<f32> = late.iter().filter_map(|a| a.entered).collect();
    let share = if entered.is_empty() { -1.0 } else { entered.iter().map(|e| (len - e) / len).sum::<f32>() / entered.len() as f32 };
    let wait = if entered.is_empty() { -1.0 } else { late.iter().filter_map(|a| a.entered.map(|e| e - a.seat_at)).sum::<f32>() / entered.len() as f32 };
    let pacing = format!(
        "pacing timer={timer:.0} how={} winner={} len={len:.0} quiet={quiet:.0} on_fight={} fights_last_min={last_min} decided_at={decided:.0} \
         decided_share={:.2} buzzer_gap={buzzer_gap} quarters={quarters} final={b}-{r} mercy_from={mercy_from:.0} late_seats={} late_in={} \
         late_never={} late_share={share:.2} late_wait={wait:.0} last_kill={:.0} last_steal={:.0} last_base_hit={:.0} last_score={:.0} late=[{late_desc}]",
        how.split(' ').next().unwrap_or(how),
        winner.map_or("draw", |w| ["blue", "red"][w]),
        on_fight as u8,
        decided / len.max(1.0),
        late.len(),
        entered.len(),
        late.len() - entered.len(),
        tr.last_kill,
        tr.last_steal,
        tr.last_base_hit,
        tr.last_score,
    );
    println!("{pacing}");
    // #35: friendly fire and luck. "Luck" is the random goods a team got: normal crates that
    // spawned on its half, and crates (megacrates at their worth) from the river objectives
    // it cleared.
    let luck = |t: usize| tr.spawn_side[t] + tr.river_crates[t];
    let luckier = match winner {
        Some(w) if luck(w) > luck(1 - w) => "1",
        Some(w) if luck(w) < luck(1 - w) => "0",
        Some(_) => "tie",
        None => "draw",
    };
    let gaps: Vec<f32> = tr.mega_at.windows(2).map(|w| w[1] - w[0]).collect();
    let gap_desc = if gaps.is_empty() {
        "-".to_string()
    } else {
        let (lo, hi) = gaps.iter().fold((f32::MAX, 0.0f32), |(lo, hi), &g| (lo.min(g), hi.max(g)));
        format!("{:.0}/{lo:.0}/{hi:.0}", gaps.iter().sum::<f32>() / gaps.len() as f32)
    };
    let rate = |t: usize| tr.hit[t] as f32 / tr.fired[t].max(1) as f32;
    let ffline = format!(
        "ff rules={} mind={} winner={} score={b}-{r} ff_dmg={:.0}-{:.0} ff_ko={} self_dmg={:.0}-{:.0} self_ko={} crate_ff={} ff_drops={} \
         own_knocks={} knocks_stopped={} held={} fired={} hit={} hit_rate={:.2}-{:.2} kills={} spawn_side={} river_cleared={} \
         river_crates={} luck={}-{} winner_luckier={luckier} mega_n={} mega_gaps={gap_desc} first_mega={:.0}",
        tune.vals[P::FfShots as usize..=P::RandSpread as usize].iter().map(|p| fmt_value_index(&p.value)).collect::<String>(),
        (tune.f(P::BotsMind) > 0.5) as u8,
        winner.map_or("draw", |w| ["blue", "red"][w]),
        tr.ff_dmg[0],
        tr.ff_dmg[1],
        pair(tr.ff_ko),
        tr.self_dmg[0],
        tr.self_dmg[1],
        pair(tr.self_ko),
        pair(tr.crate_ff),
        pair(tr.ff_drops),
        pair(tr.own_knocks),
        pair(tr.knocks_stopped),
        pair(tr.held),
        pair(tr.fired),
        pair(tr.hit),
        rate(0),
        rate(1),
        pair(tr.kills),
        pair(tr.spawn_side),
        pair(tr.river_cleared),
        pair(tr.river_crates),
        luck(0),
        luck(1),
        tr.mega_at.len(),
        tr.mega_at.first().copied().unwrap_or(-1.0),
    );
    println!("{ffline}");
    // #42: the map, and what it did to fights, hauls and the bots.
    let a = arena();
    let per = a.x * a.y * 4.0 / (2.0 * s.team_size as f32);
    let carry = |t: usize| tr.carry_t[t] / tr.carries[t].max(1) as f32;
    let mapline = format!(
        "maps map={} sym={} team={} size={:.0}x{:.0} m2_per_player={per:.0} winner={} how={} score={b}-{r} gap={} close={} comeback={} \
         kills={} pve_deaths={} first_kill={:.0} fights_per_min={:.1} carries={} carry_s={:.1}-{:.1} stolen={} falls={} lead_changes={} \
         bot_still_s={:.0} len={:.0}",
        MAP_SHORT[s.map_i],
        if s.rotate { "rot" } else { "mirror" },
        s.team_size,
        a.x * 2.0,
        a.y * 2.0,
        winner.map_or("draw", |w| ["blue", "red"][w]),
        how.split(' ').next().unwrap_or(how),
        b.abs_diff(r),
        (b.abs_diff(r) <= 5) as u8,
        (winner.is_some() && trailed >= 5) as u8,
        tr.kills[0] + tr.kills[1],
        s.pve_deaths[0] + s.pve_deaths[1],
        if tr.first_kill > 0.0 { tr.first_kill } else { -1.0 },
        tr.fights.len() as f32 / (tr.time / 60.0).max(0.1),
        tr.carries[0] + tr.carries[1],
        carry(0),
        carry(1),
        tr.stolen[0] + tr.stolen[1],
        tr.falls[0] + tr.falls[1],
        tr.lead_changes,
        tr.stuck,
        tr.time,
    );
    println!("{mapline}");
    // #36: your seat's grabs so far under this setup (cumulative over matches).
    if let Some((_, st)) = s.net.last() {
        let rules = tune.vals[..RULES].iter().map(|p| fmt_value_index(&p.value)).collect::<String>();
        println!(
            "net rules={rules} grabs={} grabs_lost={} steals={} steals_lost={} taken={} moved={} locked={} gone={} rewound={} yanked={}",
            st.grabs, st.grabs_lost, st.steals, st.steals_lost, st.taken, st.moved, st.locked, st.gone, st.rewound, st.yanked
        );
    }
    let winner = format!("{won} by {how}");
    let seat = if tune.c(P::Seat) == 1 { "bot" } else { "you" };
    let made = |t: usize| if s.made[t].is_empty() { "nothing".to_string() } else { s.made[t].join(", ") };
    let rounds = if s.loss == ROUNDS { format!(" | {}", tr.rounds.join("; ")) } else { String::new() };
    let late_line = if late.is_empty() {
        "no open seats".to_string()
    } else {
        format!(
            "late joiners in {}/{} (seat filled to in: {wait:.0} s on average, then {:.0}% of the match left)",
            entered.len(),
            late.len(),
            share.max(0.0) * 100.0
        )
    };
    let line = format!(
        "match: BLUE {b} - {r} RED, {winner} | {:.0} s on a {:.0} min timer | {} | last fight {quiet:.0} s before the end, \
         {last_min} fights in the last minute | winner led for good from {:.0} s | {late_line} | {} | {} | blue player 1 = {seat} ({}) | {:.0} s, overtime {:.0} s | \
         lead changes {} | biggest lead blue {} red {} | winner trailed by up to {trailed} | loser behind by 10+ for {hopeless:.0} s | \
         base damage taken {:.0}-{:.0} | bases fallen {} | crates lost when a base fell: spilled {} destroyed {} | \
         megacrates grabbed {} (dropped blue side {} middle {} red side {}) | kills {} | crates stolen off pads {}{rounds} | \
         blue made: {} | red made: {}",
        len,
        timer / 60.0,
        if on_fight { "ENDED ON A FIGHT" } else { "ended quiet" },
        decided,
        MAP_NAMES[s.map_i],
        rules_desc(tune),
        s.your_class.name(),
        tr.time,
        tr.overtime,
        tr.lead_changes,
        tr.max_lead[0],
        tr.max_lead[1],
        tr.base_dmg[0],
        tr.base_dmg[1],
        pair(tr.falls),
        pair(tr.spilled),
        pair(tr.destroyed),
        pair(tr.mega_grabbed),
        tr.mega_dropped[0],
        tr.mega_dropped[1],
        tr.mega_dropped[2],
        pair(tr.kills),
        pair(tr.stolen),
        made(0),
        made(1),
    );
    println!("{line}");
    s.log.push(line);
    s.say(format!("MATCH OVER: blue {b} - {r} red, {winner}"));
    s.over = 4.0;
    s.next_round = false;
    s.matches_done += 1;
    s.played_maps.push(s.map_i);
    s.queued = true;
}

fn player_step(s: &mut Sim, c: &Controls, dt: f32) {
    if !s.actors[YOU].alive() {
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
        let speed = MOVE_SPEED * if carrying { CARRY_SPEED } else { ride_speed(a, &s.tv) } * slow_factor(a);
        let mut target = Vec3::new(c.mv.x, 0.0, -c.mv.y) * speed;
        if a.stun > 0.0 || a.root > 0.0 { target = Vec3::ZERO; }
        let k = 1.0 - (-dt * 18.0).exp();
        let h = a.vel.with_y(0.0).lerp(target, k);
        a.vel = h.with_y(a.vel.y);
        if c.jump && a.grounded && a.root <= 0.0 {
            a.vel.y = JUMP_VEL;
            a.grounded = false;
        }
    }

    // #36: while a grab's answer is on its way, E and right click do nothing.
    let waiting = s.grab.is_some();

    if s.actors[YOU].stun > 0.0 {
        return;
    }
    let facing = s.actors[YOU].facing;
    let in_mech = s.mech_cap(YOU) > 0;
    // A hauler mech's E and right click work on its top crate: take it in hand first.
    if in_mech && (c.throw || c.grab) && !waiting && !s.actors[YOU].load.is_empty() {
        let pos = s.actors[YOU].pos;
        let home = pos.xz().distance(pads()[s.actors[YOU].team]) < PAD_HALF + 1.6;
        let room = s.actors[YOU].load.len() < s.mech_cap(YOU);
        let grabbable = s.nearest_free(YOU, s.reach()).is_some();
        if c.throw || home || !room || !grabbable {
            s.hand_top(YOU);
        }
    }
    let carrying = s.actors[YOU].carrying.is_some();
    let in_tank = s.actors[YOU].tank.is_some();

    if let Some(kind) = c.drop {
        let at = s.place_target(YOU);
        s.spawn_crate(kind, at + Vec3::Y * 0.5);
    }

    if c.grab && in_tank && !in_mech {
        s.say("you can't carry crates in a vehicle (the hauler mech can)");
    } else if c.grab && !waiting {
        if carrying {
            let plan = throw_plan(s, c.aim_dist);
            let pos = s.actors[YOU].pos;
            let at = match plan.target {
                Some(t) if t.xz().distance(pos.xz()) <= PLACE_REACH => t,
                _ => s.place_target(YOU),
            };
            let id = s.actors[YOU].carrying;
            s.place(YOU, at);
            mark_mine(s, id);
            s.stats().throws += 1;
            if in_mech && pad_contains(pads()[s.actors[YOU].team], at) {
                s.mech_banked[s.actors[YOU].team] += 1;
            }
        } else if !in_mech || s.actors[YOU].load.len() < s.mech_cap(YOU) {
            request_grab(s);
        }
    }

    if c.throw && carrying && !waiting {
        let plan = throw_plan(s, c.aim_dist);
        let head = s.actors[YOU].pos + Vec3::Y * HEAD;
        let id = s.actors[YOU].carrying;
        s.release(YOU, plan.vel, plan.from - head);
        mark_mine(s, id);
        s.stats().throws += 1;
    }

    // Right click with empty hands: the loadout's right-click item.
    if c.throw && !carrying && !in_tank && !waiting {
        let range = c.aim_dist.map_or(8.0, |d| d.clamp(1.0, 30.0));
        use_alt(s, YOU, facing, range);
    }

    // Q: throw the 1-crate item where you aim (Set C: use it).
    if c.toss && !carrying && !in_tank {
        let pos = s.actors[YOU].pos;
        let at = c.aim_point.unwrap_or(pos + facing * 6.0);
        if s.actors[YOU].kit.is_some() {
            use_kit(s, YOU, at);
        } else {
            if s.actors[YOU].toss.is_none() {
                s.say("nothing to use: 1 crate makes your class's 1-crate item");
            }
            toss(s, YOU, at);
        }
    }

    // A barrage or gate waiting: left click places it where you aim, and fires nothing.
    if s.actors[YOU].mark.is_some() {
        if c.fire_tap && !carrying {
            let pos = s.actors[YOU].pos;
            let at = c.aim_point.unwrap_or(pos + facing * 10.0);
            place_mark(s, YOU, at);
        }
    } else if c.fire && !carrying && s.actors[YOU].fire_cd <= 0.0 {
        // Full lock: no shooting while carrying.
        let range = c.aim_dist.map_or(30.0, |d| d.clamp(1.0, 30.0));
        fire(s, YOU, facing, range);
    }

    // A hauler mech can kick once it's set its crates down.
    if !carrying && (!in_tank || in_mech && s.actors[YOU].load.is_empty()) {
        let pos = s.actors[YOU].pos;
        let my_pad = pads()[s.actors[YOU].team];
        // Hold kick beside your pad to open what's on it.
        if c.kick_held && pos.xz().distance(my_pad) < PAD_HALF + 1.4 {
            s.kicking[s.actors[YOU].team] = Some(YOU);
        }
        let mut opened = None;
        let mut punted = None;
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
                Kind::Normal if near && c.kick && punted.is_none() => {
                    cr.vel = facing * 9.0 + Vec3::Y * 3.0;
                    punted = Some(cr.id);
                }
                _ => {}
            }
        }
        if punted.is_some() {
            mark_mine(s, punted);
            s.stats().throws += 1;
        }
        if let Some((id, at)) = opened {
            let predicted = s.acts_predicted();
            let gone: Vec<Crate> = s.crates.extract_if(.., |cr| cr.id == id).collect();
            for cr in gone {
                s.bury(cr, predicted);
            }
            for i in 0..3 {
                let a = i as f32 * 2.1 + s.rand();
                let id = s.spawn_crate(Kind::Normal, at + Vec3::Y * 0.5);
                if let Some(cr) = s.crate_mut(id) {
                    cr.vel = Vec3::new(a.cos() * 3.0, 7.0, a.sin() * 3.0);
                }
                mark_mine(s, Some(id));
            }
            s.loot_respawn = 6.0;
            s.say("loot crate opened: 3 crates");
        }
    }
}

/// Fires `who`'s weapon: the tank's cannon, a crafted gun (until its ammo runs out), or the
/// standard gun. Sets the cooldown, shorter with adrenaline. Nothing while a barrage or gate
/// waits to be placed.
fn fire(s: &mut Sim, who: usize, dir: Vec3, range: f32) {
    if s.actors[who].mark.is_some() {
        return;
    }
    fire_weapon(s, who, dir, range);
    if s.actors[who].adren > 0.0 {
        let rate = s.n(P::AdrenRate);
        s.actors[who].fire_cd /= rate;
    }
}

/// #35 weapon spread (key 8): where a shot really goes. The tank cannon, the shotgun (its
/// own fan) and mines don't spread.
fn spread(s: &mut Sim, who: usize, dir: Vec3) -> Vec3 {
    let a = &s.actors[who];
    let kind = a.gun.map(|g| g.kind);
    let exact = a.tank.is_some() || matches!(kind, Some(GunKind::Shotgun | GunKind::Mines));
    if s.luck[3] == 0 || exact {
        return dir;
    }
    let cone = if kind == Some(GunKind::Rapid) { s.n(P::SpreadRapid) } else { s.n(P::SpreadCone) };
    // Bloom: the first shot is exact, and each shot held after it opens the cone a third.
    let k = if s.luck[3] == 2 {
        let a = &mut s.actors[who];
        let k = a.bloom;
        a.bloom = (a.bloom + 0.34).min(1.0);
        k
    } else {
        1.0
    };
    let t = (s.rand() * 2.0 - 1.0) * cone * k;
    Quat::from_rotation_y(t.to_radians()) * dir
}

fn fire_weapon(s: &mut Sim, who: usize, dir: Vec3, range: f32) {
    let dir = spread(s, who, dir);
    s.actors[who].since_shot = 0.0;
    let team = s.actors[who].team;
    if s.actors[who].gun.is_none_or(|g| g.kind != GunKind::Mines) {
        s.track.fired[team] += 1;
    }
    let a = &s.actors[who];
    let from = a.pos + Vec3::Y * CHEST + dir * 0.5;
    let team = a.team;
    if who == YOU {
        s.fired += 1;
    }
    let shot = |vel: Vec3, dmg: f32, life: f32| Shot::new(from, vel, Some(who), team, dmg, life);
    if let Some(tank) = a.tank {
        match tank.ride {
            Ride::Tank => {
                s.actors[who].fire_cd = 0.9;
                let blast = s.n(P::RocketBlast);
                s.shots.push(Shot { splash: blast, slow: tank.gold, ..shot(dir * 16.0, 80.0, range.max(6.0) / 16.0) });
            }
            Ride::Mech => stomp(s, who, tank.gold),
            Ride::Ambulance => {
                // A weak blaster.
                s.actors[who].fire_cd = 1.0 / FIRE_RATE;
                let dmg = s.n(P::AmbBlaster);
                s.shots.push(shot(dir * PROJ_SPEED, dmg, range.min(14.0) / PROJ_SPEED));
            }
            Ride::AmbulanceShot => {
                // The variant: heal shots the driver aims, weak against enemies. A super one's
                // shot carries a 25 shield every 5 s.
                s.actors[who].fire_cd = 1.0 / FIRE_RATE;
                let heal = s.n(P::AmbShotHeal) * s.k(tank.gold);
                let life = 12.0 / PROJ_SPEED;
                let dmg = s.n(P::AmbBlaster);
                s.shots.push(Shot { heal, amb: true, ..shot(dir * PROJ_SPEED, dmg, life) });
            }
        }
        return;
    }
    let Some(mut gun) = a.gun else {
        s.plain_shots[team] += 1;
        s.actors[who].fire_cd = 1.0 / FIRE_RATE;
        s.shots.push(shot(dir * PROJ_SPEED, SHOT_DMG, range / PROJ_SPEED));
        return;
    };
    s.crafted_shots[team] += 1;
    let k = s.k(gun.gold);
    match gun.kind {
        GunKind::Rapid => {
            s.actors[who].fire_cd = 1.0 / s.n(P::RapidRate);
            let dmg = s.n(P::RapidDmg) * k;
            s.shots.push(Shot { pierce: gun.gold as u8, ..shot(dir * PROJ_SPEED, dmg, range / PROJ_SPEED) });
        }
        GunKind::Rivet => {
            s.actors[who].fire_cd = 1.0 / s.n(P::RivetRate);
            let dmg = s.n(P::RivetDmg) * k;
            let stagger = if gun.gold { 0.2 } else { 0.0 };
            s.shots.push(Shot { rivet: true, stagger, ..shot(dir * PROJ_SPEED, dmg, range / PROJ_SPEED) });
        }
        GunKind::Heal => {
            // A visible heal shot: heals the first hurt teammate it touches, out to its range;
            // a weak shot against enemies.
            s.actors[who].fire_cd = 0.2;
            let heal = s.n(P::HealShot) * k;
            let life = s.n(P::HealRange) / PROJ_SPEED;
            s.shots.push(Shot { heal, bounce: gun.gold, ..shot(dir * PROJ_SPEED, heal, life) });
        }
        GunKind::Shotgun => {
            // A fan of pellets. Super: 7 pellets in a tighter fan (its extra strength).
            s.actors[who].fire_cd = 1.0 / s.n(P::ShotgunRate);
            let (n, spread) = if gun.gold { (7, s.n(P::ShotgunSpread) * 0.6) } else { (s.n(P::ShotgunPellets) as usize, s.n(P::ShotgunSpread)) };
            let (dmg, life) = (s.n(P::ShotgunDmg), s.n(P::ShotgunRange) / PROJ_SPEED);
            for k in 0..n {
                let t = if n > 1 { k as f32 / (n - 1) as f32 - 0.5 } else { 0.0 };
                // #35: a random cone scatters the pellets anywhere in the fan.
                let t = if s.luck[3] == 1 { s.rand() - 0.5 } else { t };
                let d = Quat::from_rotation_y((t * spread).to_radians()) * dir;
                s.shots.push(shot(d * PROJ_SPEED, dmg, life));
            }
        }
        GunKind::Mines => {
            // A mine at your feet. Past the most you can have out, your oldest one goes.
            s.actors[who].fire_cd = 0.5;
            let at = s.actors[who].pos.with_y(0.0);
            let out = s.mines.iter().filter(|m| m.owner == who).count();
            if out >= s.n(P::MinesOut) as usize
                && let Some(k) = s.mines.iter().position(|m| m.owner == who)
            {
                s.mines.remove(k);
            }
            s.mines.push(Mine { pos: at, team, owner: who, gold: gun.gold, arm: 0.5 });
            s.mines_laid[team] += 1;
        }
    }
    gun.ammo -= 1;
    s.actors[who].gun = if gun.ammo == 0 { None } else { Some(gun) };
    if gun.ammo == 0 {
        s.dry[team] += 1;
    }
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
    let k = s.k(alt.gold);
    match alt.kind {
        AltKind::Rockets => {
            s.actors[who].alt_cd = 1.0;
            let from = pos + Vec3::Y * CHEST + dir * 0.5;
            let (dmg, blast) = (s.n(P::RocketDmg) * k, s.n(P::RocketBlast));
            let rocket = Shot::new(from, dir * 14.0, Some(who), team, dmg, range.max(4.0) / 14.0);
            s.shots.push(Shot { splash: blast, split: alt.gold, ..rocket });
        }
        AltKind::ShieldWall => {
            // A wall where you aim, up to 8 m away, facing you.
            s.actors[who].alt_cd = 0.5;
            let mut at = pos + dir * range.clamp(1.5, 8.0);
            if !clear(pos.xz(), at.xz(), 0.5) {
                at = pos + dir * 1.5;
            }
            let (half, hp, life) = (s.n(P::WallLen) / 2.0, s.n(P::WallHp) * k, s.n(P::WallLife));
            s.add_wall(team, at, dir, half, hp, life, alt.gold, false, false);
        }
        AltKind::Pulse => {
            s.actors[who].alt_cd = 2.0;
            let (heal, r) = (s.n(P::PulseHp) * k, s.n(P::PulseRadius));
            for b in s.actors.iter_mut().filter(|b| b.team == team && b.alive() && b.pos.distance(pos) < r) {
                b.hp = (b.hp + heal).min(100.0);
                if alt.gold {
                    b.shield = (b.shield + 25.0).min(100.0);
                }
            }
            s.booms.push((pos + Vec3::Y * 0.2, r, 0.4));
        }
        AltKind::Launcher => {
            // Lobbed on a visible arc to where you aim; bounces once, goes off on the next
            // touch (or on an enemy).
            s.actors[who].alt_cd = 0.6;
            let from = pos + Vec3::Y * CHEST + dir * 0.5;
            let to = pos + dir * range.clamp(2.0, s.n(P::LauncherRange));
            let vel = velocity_to(from, to.with_y(0.15), 40f32.to_radians());
            s.lobs.push(Lob { kind: TossKind::Launched, pos: from, vel, team, owner: who, gold: alt.gold, fuse: 4.0, stuck: None, hops: 0, small: false, wave: 0 });
        }
        AltKind::ShieldBurst => {
            // A visible shot: the first ally it touches gets the shield.
            s.actors[who].alt_cd = 0.5;
            let from = pos + Vec3::Y * CHEST + dir * 0.5;
            let life = s.n(P::BurstRange) / PROJ_SPEED;
            let shield = s.n(P::BurstShield) * k;
            s.shots.push(Shot { shield, cleanse: alt.gold, ..Shot::new(from, dir * PROJ_SPEED, Some(who), team, 0.0, life) });
        }
    }
    alt.charges -= 1;
    s.actors[who].alt = if alt.charges == 0 { None } else { Some(alt) };
    if alt.charges == 0 && who == YOU {
        s.say("right-click item used up");
    }
}

/// Throws `who`'s 1-crate item at `at` on the jump-throw arc: further from the top of a jump.
/// Not while carrying (ADR 0003) or in a tank.
fn toss(s: &mut Sim, who: usize, at: Vec3) {
    let a = &s.actors[who];
    let Some((kind, gold)) = a.toss else { return };
    if a.carrying.is_some() || a.tank.is_some() || !a.alive() {
        return;
    }
    let from = a.pos + Vec3::Y * HEAD + a.facing * 0.3;
    let max = THROW_DIST * jump_boost(a);
    let flat = (at - a.pos).with_y(0.0);
    let flat = if flat.length() < 0.6 { a.facing * 0.6 } else { flat.clamp_length_max(max) };
    let to = a.pos + flat;
    let vel = velocity_to(from, to.with_y(0.15), THROW_ANGLE.to_radians());
    let (team, fuse) = (a.team, s.n(P::GrenadeFuse));
    s.lobs.push(Lob { kind, pos: from, vel, team, owner: who, gold, fuse, stuck: None, hops: 0, small: false, wave: 0 });
    s.actors[who].toss = None;
}

/// Uses `who`'s Set C 1-crate item (Q): adrenaline, a barricade where you aim, or a self-patch.
/// Not while carrying or in a vehicle.
fn use_kit(s: &mut Sim, who: usize, at: Vec3) {
    let a = &s.actors[who];
    let Some((kind, gold)) = a.kit else { return };
    if a.carrying.is_some() || a.tank.is_some() || !a.alive() {
        return;
    }
    let (pos, team, facing) = (a.pos, a.team, a.facing);
    let k = s.k(gold);
    match kind {
        KitKind::Adrenaline => {
            let t = if gold { s.n(P::AdrenSuperTime) } else { s.n(P::AdrenTime) };
            let a = &mut s.actors[who];
            a.adren = t;
            a.adren_gold = gold;
        }
        KitKind::Barricade => {
            let flat = (at - pos).with_y(0.0);
            let dir = flat.normalize_or(facing);
            let mut spot = pos + dir * flat.length().clamp(1.5, 8.0);
            if !clear(pos.xz(), spot.xz(), 0.5) {
                spot = pos + dir * 1.5;
            }
            let (half, hp, life) = (s.n(P::BarricadeLen) / 2.0, s.n(P::BarricadeHp) * k, s.n(P::BarricadeLife));
            s.add_wall(team, spot, dir, half, hp, life, gold, false, false);
            let w = s.walls.last_mut().unwrap();
            w.barricade = true;
            w.h = BARRICADE_H;
        }
        KitKind::SelfPatch => {
            let (now, over, secs) = (s.n(P::PatchNow) * k, s.n(P::PatchOver) * k, s.n(P::PatchSecs));
            let a = &mut s.actors[who];
            a.hp = (a.hp + now).min(100.0);
            a.regen = over / secs;
            a.regen_t = secs;
            if gold {
                a.shield = (a.shield + 25.0).min(100.0);
            }
            s.booms.push((pos + Vec3::Y, 0.8, 0.3));
        }
    }
    s.actors[who].kit = None;
}

/// Lane chokes and jungle entrances a gate can shut, on both halves: (centre, the way it
/// faces). A gate placed near one snaps to it.
fn chokes() -> Vec<(Vec2, Vec2)> {
    if map_on() { layout().chokes.clone() } else { Vec::new() }
}

/// Places `who`'s waiting all 8 at `at`: a barrage's circle, or one gate wall (snapped to a
/// choke within 5 m, else facing the nearest axis to your aim). Out to the mark range.
fn place_mark(s: &mut Sim, who: usize, at: Vec3) {
    let a = &s.actors[who];
    let Some(mut m) = a.mark else { return };
    let (pos, team, facing) = (a.pos, a.team, a.facing);
    let range = s.n(P::MarkRange);
    // Bots stand in for players who know the map: they skip the range.
    if who == YOU && at.xz().distance(pos.xz()) > range {
        s.say(format!("too far: you can mark out to {range:.0} m"));
        return;
    }
    match m.make {
        Make::Barrage(sparse) => {
            start_barrage(s, team, who, at.with_y(0.0), m.gold, sparse);
        }
        _ => {
            let near = chokes().into_iter().filter(|c| s.map && c.0.distance(at.xz()) < 5.0).min_by(|x, y| x.0.distance(at.xz()).total_cmp(&y.0.distance(at.xz())));
            let (spot, normal) = match near {
                Some((p, n)) => (p, n),
                None => {
                    let n = if facing.x.abs() >= facing.z.abs() { Vec2::X } else { Vec2::Y };
                    (at.xz(), n)
                }
            };
            let (half, hp, life) = (s.n(P::GateLen) / 2.0, s.n(P::GateHp) * s.k(m.gold), s.n(P::GateLife));
            s.add_wall(team, spot.extend(0.0).xzy(), normal.extend(0.0).xzy(), half, hp, life, m.gold, false, false);
            s.walls.last_mut().unwrap().pass = true;
            let msg = format!("{} gate wall up{}", ["BLUE", "RED"][team], if near.is_some() { " across a choke" } else { "" });
            println!("t={:.1}s {msg}", s.t);
            s.say(msg);
        }
    }
    m.left -= 1;
    s.actors[who].mark = if m.left == 0 { None } else { Some(m) };
    s.actors[who].fire_cd = 0.3;
}

/// A barrage marked on `at`: both teams see the circle; shells start falling after the delay.
fn start_barrage(s: &mut Sim, team: usize, owner: usize, at: Vec3, gold: bool, sparse: bool) {
    let (shells, over) = if sparse { (s.n(P::SparseShells), s.n(P::SparseTime)) } else { (s.n(P::BarrageShells), s.n(P::BarrageTime)) };
    let r = s.n(P::BarrageRadius);
    let inside = s.actors.iter().filter(|a| a.team != team && a.alive() && a.pos.xz().distance(at.xz()) < r).count();
    s.barrage_n[team] += 1;
    s.barrage_in_mark[team] += inside as u32;
    let id = s.next_id;
    s.next_id += 1;
    let shells = shells as u32;
    s.barrages.push(Barrage { id, team, owner, at, r, delay: s.n(P::BarrageDelay), shells_left: shells, every: over / shells.max(1) as f32, next: 0.0, gold, hit: 0 });
    let msg = format!("{} marked a {}barrage ({shells} shells): get out of the circle", ["BLUE", "RED"][team], if gold { "SUPER " } else { "" });
    println!("t={:.1}s {msg}", s.t);
    s.say(msg);
}

/// Barrages: after the delay, shells fall one by one on random spots in the circle.
fn barrages_step(s: &mut Sim, dt: f32) {
    let fall = s.n(P::ShellFall);
    let mut k = 0;
    while k < s.barrages.len() {
        let b = &mut s.barrages[k];
        let was = b.delay;
        b.delay -= dt;
        if b.delay > 0.0 {
            k += 1;
            continue;
        }
        if was > 0.0 {
            let (team, at, r) = (b.team, b.at, b.r);
            let inside = s.actors.iter().filter(|a| a.team != team && a.alive() && a.pos.xz().distance(at.xz()) < r).count();
            s.barrage_in_fall[team] += inside as u32;
        }
        let b = &mut s.barrages[k];
        b.next -= dt;
        let mut drops = Vec::new();
        while b.shells_left > 0 && b.next <= 0.0 {
            b.next += b.every;
            b.shells_left -= 1;
            drops.push((b.id, b.team, b.owner, b.at, b.r, b.gold));
        }
        for (id, team, owner, at, r, gold) in drops {
            // Uniform over the disc.
            let (u, v) = (s.rand(), s.rand() * std::f32::consts::TAU);
            let spot = at + (Vec2::from_angle(v) * r * u.sqrt()).extend(0.0).xzy();
            let pos = spot.with_y(16.0);
            s.lobs.push(Lob { kind: TossKind::Shell, pos, vel: Vec3::NEG_Y * fall, team, owner, gold, fuse: 99.0, stuck: None, hops: 0, small: false, wave: id });
        }
        let b = &s.barrages[k];
        let (id, team, hit) = (b.id, b.team, b.hit);
        if b.shells_left == 0 && !s.lobs.iter().any(|l| l.wave == id) {
            s.barrages.remove(k);
            s.barrage_hit[team] += hit.count_ones();
            let msg = format!("{} barrage over: {} enemies hit", ["BLUE", "RED"][team], hit.count_ones());
            println!("t={:.1}s {msg}", s.t);
            s.say(msg);
            continue;
        }
        k += 1;
    }
}

/// The hauler mech's left click: a stomp that shoves enemies close by away. No damage, so a
/// carrier keeps its crate, except against a super mech's stomp, which knocks it loose.
fn stomp(s: &mut Sim, who: usize, gold: bool) {
    s.actors[who].fire_cd = 1.5;
    let (pos, team) = (s.actors[who].pos, s.actors[who].team);
    let (r, push) = (s.n(P::StompRadius), s.n(P::StompPush) * s.k(gold));
    for j in 0..s.actors.len() {
        let b = &s.actors[j];
        if b.team == team || !b.alive() || b.pos.xz().distance(pos.xz()) > r {
            continue;
        }
        let out = (b.pos - pos).with_y(0.0).normalize_or(Vec3::X);
        // A super stomp knocks a crate off a carrier (the top one off an enemy mech).
        if gold && (b.carrying.is_some() || !b.load.is_empty()) {
            s.hand_top(j);
            s.release(j, out * 2.5 + Vec3::Y * 2.5, Vec3::ZERO);
            s.say(format!("super stomp knocked a crate off {}", s.describe(j)));
        }
        let b = &mut s.actors[j];
        b.shove += out * push;
        b.stun = b.stun.max(0.3);
    }
    s.booms.push((pos + Vec3::Y * 0.2, r, 0.3));
}

/// How fast a vehicle moves, as a share of the walking speed.
fn ride_speed(a: &Actor, tv: &[f32]) -> f32 {
    match a.tank {
        None => 1.0,
        Some(t) if t.ride == Ride::Mech => tv[P::MechSpeed as usize],
        Some(_) => 0.8,
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
        .filter(|p| p.pos.xz().distance(a.pos.xz()) < 12.0 && (!matches!(p.drop, Drop::Heal | Drop::MedPack(_)) || a.hp < 100.0))
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
    /// #35: jump-throw the crate onto this pad cell.
    Throw(Vec3),
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
    // No base left (permanent rule): nothing to haul for.
    if s.base_down[a.team].is_infinite() {
        return (None, if a.carrying.is_some() { Act::Drop } else { Act::None }, None);
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
        // #35: a bot that throws does it from 3-7 m out.
        let d = cell.xz().distance(a.pos.xz());
        if s.n(P::BotsThrow) > 0.5 && (3.0..7.0).contains(&d) && clear(a.pos.xz(), cell.xz(), 0.2) {
            return (None, Act::Throw(cell), None);
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
        // Super crates and megacrates are worth a longer walk.
        .map(|c| {
            let bonus = match c.kind {
                Kind::Super if s.super_mode > 0 => 8.0,
                Kind::Mega => 16.0,
                _ => 0.0,
            };
            (c, c.pos.distance(a.pos) - bonus)
        })
        .min_by(|x, y| x.1.total_cmp(&y.1))
        .map(|(c, _)| c);
    match pick {
        None => (None, Act::None, None),
        Some(c) if c.pos.xz().distance(a.pos.xz()) <= s.reach() + HALF => (None, Act::Grab(c.id), Some(c.id)),
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
        Shape::Recipe(Slot::Pair) if s.room(team) == 0 => s.pad_t[team] > 8.0,
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
    // Outside the bunker's front wall while there is one.
    let out = if s.walls.iter().any(|w| w.team == a.team && w.bunker) { 3.6 } else { 2.8 };
    Vec3::new(pad.x + toward_centre(a.team) * out, 0.0, pad.y + (a.slot as f32 - 1.0) * 1.6)
}

/// Bots and walls: the next point toward `goal` that doesn't walk into a wall, and the wall
/// to shoot when an enemy super bunker's gap is shut to this bot.
fn detour(s: &Sim, i: usize, goal: Vec3) -> (Vec3, Option<Vec3>) {
    let a = &s.actors[i];
    let (from, team) = (a.pos.xz(), a.team);
    let r = ACTOR_R * 0.8;
    let in_way = |p: Vec2, q: Vec2| s.walls.iter().find(|w| w.blocks_body(team) && w.crosses(p, q, r));
    // Pressed against an enemy gate (the route led here): keep at it and shoot it down.
    if let Some(w) = s.walls.iter().find(|w| w.pass && w.team != team && w.dist(from) < 1.2) {
        return (goal, Some(w.pos.with_y(CHEST)));
    }
    let Some(w) = in_way(from, goal.xz()) else { return (goal, None) };
    let to3 = |p: Vec2| p.extend(0.0).xzy();
    if w.pass {
        // An enemy gate: walk up to it, then stand and shoot it down.
        if w.dist(from) < 5.0 {
            return (to3(from), Some(w.pos.with_y(CHEST)));
        }
        return (goal, None);
    }
    // The best of some waypoints: in a straight line from here, shortest way to the goal.
    let best = |pts: &[Vec2]| {
        pts.iter()
            .copied()
            .filter(|&p| in_way(from, p).is_none())
            .min_by(|x, y| (from.distance(*x) + x.distance(goal.xz())).total_cmp(&(from.distance(*y) + y.distance(goal.xz()))))
    };
    if !w.bunker {
        // Round the nearer end of a shield wall, on our side of it.
        let n = w.normal.xz();
        let tan = Vec2::new(-n.y, n.x);
        let side = n * w.local(from).1.signum() * 0.7;
        let ends = [-1.0, 1.0].map(|k| w.pos.xz() + tan * k * (w.half_len + 0.9) + side);
        let far = ends.map(|e| e + tan * (e - w.pos.xz()).dot(tan).signum() * 0.5 - side * 2.0);
        return (to3(best(&ends).or_else(|| best(&far)).unwrap_or(ends[0])), None);
    }
    let pad = pads()[w.team];
    let t = toward_centre(w.team);
    let gap = Vec2::new(pad.x + t * BUNKER_R, pad.y);
    let in_ring = |p: Vec2| (p - pad).abs().max_element() < BUNKER_R;
    let gate = s.walls.iter().find(|g| g.gate && g.team == w.team && g.blocks_body(team));
    if let Some(g) = gate.filter(|_| in_ring(goal.xz()) && !in_ring(from)) {
        // Shut out: stand off and shoot the gate.
        let stand = gap + Vec2::X * t * 4.0;
        return (to3(stand), Some(g.pos.with_y(CHEST)));
    }
    let c = BUNKER_R + 1.3;
    let corners = [Vec2::new(t * c, c), Vec2::new(t * c, -c), Vec2::new(-t * c, c), Vec2::new(-t * c, -c)].map(|o| pad + o);
    if in_ring(from) == in_ring(goal.xz()) {
        return (to3(best(&corners).unwrap_or(corners[0])), None);
    }
    // In or out through the gap: line up with it, then walk through.
    let (mouth, past) = if in_ring(from) {
        (gap - Vec2::X * t * 1.0, gap + Vec2::X * t * 1.2)
    } else {
        (gap + Vec2::X * t * 1.5, gap - Vec2::X * t * 1.0)
    };
    let next = if from.distance(gap) < 0.5 || (from - gap).y.abs() < 0.6 && in_way(from, past).is_none() {
        past
    } else if in_way(from, mouth).is_none() {
        mouth
    } else {
        best(&corners).unwrap_or(mouth)
    };
    (to3(next), None)
}

/// Engineers with a rivet gun: a hurt wall of theirs within 10 m to repair.
fn wall_to_fix(s: &Sim, i: usize) -> Option<Vec3> {
    let a = &s.actors[i];
    if !a.gun.is_some_and(|g| g.kind == GunKind::Rivet) {
        return None;
    }
    s.walls
        .iter()
        .filter(|w| w.team == a.team && !w.gate && w.hp < w.max_hp * 0.8 && w.dist(a.pos.xz()) < 10.0)
        .min_by(|x, y| x.dist(a.pos.xz()).total_cmp(&y.dist(a.pos.xz())))
        .map(|w| w.pos.with_y(CHEST))
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
        j != i && j != human && b.team == team && b.alive() && b.carrying.is_none() && b.tank.is_none() && b.pos.xz().distance(pad) < me
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
    // A standing enemy base in range and in sight (#15).
    let base = base_pos(enemy);
    if a.carrying.is_none() && s.base_up(enemy) && a.pos.xz().distance(base) < 12.0 && clear(a.pos.xz(), base, 0.1) {
        return Some(base.extend(1.0).xzy());
    }
    if a.carrying.is_some() || a.pos.xz().distance(pad) > 11.0 {
        return None;
    }
    let crates = s.crates.iter().any(|c| c.state == CrateState::Free && c.kind.builds() && pad_contains(pad, c.pos));
    (crates && (s.builds[enemy].is_some() || matches!(s.vuln, SPLIT | SPILL))).then(|| pad.extend(CHEST).xzy())
}

/// #15: an enemy at bot `i`'s base while it's under attack, if `i` is close enough to come
/// back for it.
fn defend(s: &Sim, i: usize) -> Option<usize> {
    let team = s.actors[i].team;
    let base = base_pos(team);
    if !s.base_up(team) || s.base_hit_t[team] > 3.0 || s.actors[i].pos.xz().distance(base) > 30.0 {
        return None;
    }
    nearest_enemy(s, i, base.extend(0.0).xzy(), 12.0)
}

/// #15: where a raiding bot goes to shoot the enemy base. Everyone raids once its own base
/// is gone.
fn raid_goal(s: &Sim, i: usize) -> Option<Vec3> {
    let a = &s.actors[i];
    let enemy = 1 - a.team;
    if a.carrying.is_some() || a.tank.is_some() || !s.base_up(enemy) {
        return None;
    }
    // Raiders: the team's highest living slots, never its starting player (it hauls and
    // kicks) and its jungler last. Teams are small (joined players stay out once knocked
    // out), so one raids as soon as there are two players up.
    let team: Vec<usize> = (0..s.actors.len()).filter(|&j| s.actors[j].team == a.team && s.actors[j].alive()).collect();
    let mut can: Vec<usize> = team.iter().map(|&j| s.actors[j].slot).filter(|&k| k != 0).collect();
    can.sort_unstable_by_key(|&k| (k == 2, std::cmp::Reverse(k)));
    let raider = can.iter().take(s.n(P::Raiders) as usize).any(|&k| k == a.slot);
    if !(raider || s.base_down[a.team].is_infinite()) {
        return None;
    }
    let base = base_pos(enemy);
    let t = toward_centre(enemy);
    Some(Vec3::new(base.x + t * 6.5, 0.0, base.y + (a.slot as f32 - 3.0) * 1.6))
}

/// A bot player: carry home, fight what's close, chase enemy carriers to steal back,
/// otherwise haul (and steal). Builds pad shapes and kicks them open. Guards the pad when
/// there's nothing to haul.
/// #35: a bot that minds its allies holds fire while friendly fire would hurt one: an ally
/// in the line of a shot out to `d`, or (for a blast landing at `at`) within `r` of it.
fn ally_in_way(s: &Sim, i: usize, dir: Vec3, d: f32, blast: Option<(Vec3, f32)>) -> bool {
    if s.n(P::BotsMind) < 0.5 {
        return false;
    }
    let (pos, team) = (s.actors[i].pos, s.actors[i].team);
    let allies = || (0..s.actors.len()).filter(move |&j| j != i && s.actors[j].team == team && s.actors[j].alive());
    if let Some((at, r)) = blast {
        let me = s.ff[1] >= 1 && at.xz().distance(pos.xz()) < r + 0.5;
        let mate = s.ff[1] == 2 && allies().any(|j| s.actors[j].pos.xz().distance(at.xz()) < r + 0.5);
        return me || mate;
    }
    s.ff[0] == 1
        && allies().any(|j| {
            let to = (s.actors[j].pos - pos).with_y(0.0);
            let along = to.dot(dir);
            along > 0.0 && along < d + 1.0 && (to - dir * along).length() < 0.9
        })
}

fn ai_step(s: &mut Sim, i: usize, dt: f32) {
    if !s.actors[i].alive() {
        return;
    }
    // #36 drills: a rival rushing a crate, to contest your grab.
    if let Some((id, until)) = s.actors[i].rush {
        let at = s.crates.iter().find(|c| c.id == id && c.loose()).map(|c| c.pos);
        match at {
            Some(p) if s.t < until && s.actors[i].carrying.is_none() => {
                let to = (p - s.actors[i].pos).with_y(0.0);
                if to.length() <= s.reach() + HALF {
                    s.actors[i].rush = None;
                    s.take(i, id);
                } else {
                    let a = &mut s.actors[i];
                    a.facing = to.normalize();
                    a.vel = (a.facing * MOVE_SPEED * 0.85).with_y(a.vel.y);
                }
                return;
            }
            _ => s.actors[i].rush = None,
        }
    }
    // A barrage goes on the nearest enemy in sight within the mark range; gate walls across
    // the chokes nearest the bot that aren't shut yet.
    if let Some(m) = s.actors[i].mark
        && s.actors[i].carrying.is_none()
    {
        let pos = s.actors[i].pos;
        if m.make == Make::Gate {
            let team = s.actors[i].team;
            let free = chokes()
                .into_iter()
                .filter(|c| !s.walls.iter().any(|w| w.pass && w.team == team && w.pos.xz().distance(c.0) < 1.0))
                .min_by(|x, y| x.0.distance(pos.xz()).total_cmp(&y.0.distance(pos.xz())));
            if let Some((at, _)) = free.filter(|_| s.map) {
                place_mark(s, i, at.extend(0.0).xzy());
            } else {
                // The open arena has no chokes: across the way to the enemy pad.
                let at = pos + Vec3::X * toward_centre(team) * 6.0;
                place_mark(s, i, at);
            }
        } else if let Some(e) = nearest_enemy(s, i, pos, s.n(P::MarkRange)) {
            let at = s.actors[e].pos;
            place_mark(s, i, at);
        }
    }
    let pos = s.actors[i].pos;
    let carrying = s.actors[i].carrying.is_some();
    // A raider (#15) heads for the enemy base and keeps shooting it, fighting only someone
    // who gets close.
    let raid = raid_goal(s, i);
    if raid.is_some_and(|g| g.xz().distance(pos.xz()) < 3.0) {
        let team = s.actors[i].team;
        s.track.posted[team] += dt;
    }
    let near = nearest_enemy(s, i, pos, if raid.is_some() { 5.0 } else if s.map { 10.0 } else { 8.0 });
    let carrier = nearest_enemy(s, i, pos, if raid.is_some() { 5.0 } else { 14.0 }).filter(|&e| s.actors[e].carrying.is_some());
    let keep = s.actors[i].claim;
    let in_tank = s.actors[i].tank.is_some();
    let ride = s.actors[i].tank.map(|t| t.ride);
    let enemy_pad = pads()[1 - s.actors[i].team];
    let (target, goal, act, claim) = match (carrying, near, carrier) {
        // A hauler mech hauls, a few crates a trip, and stomps whoever gets close.
        (false, _, _) if ride == Some(Ride::Mech) => {
            let (goal, act, claim) = mech_plan(s, i);
            let goal = if goal.is_none() && act_none(&act) { Some(guard_post(s, i)) } else { goal };
            (nearest_enemy(s, i, pos, s.n(P::StompRadius) * 0.8), goal, act, claim)
        }
        // An ambulance stays with the most hurt teammate (or the one closest to a fight).
        (false, _, _) if matches!(ride, Some(Ride::Ambulance | Ride::AmbulanceShot)) => {
            (nearest_enemy(s, i, pos, 10.0), Some(ambulance_goal(s, i)), Act::None, None)
        }
        // A barrage to mark: push toward the enemy pad until someone shows up.
        (false, _, _) if s.actors[i].mark.is_some() => {
            let goal = Vec3::new(enemy_pad.x - toward_centre(1 - s.actors[i].team) * 12.0, 0.0, enemy_pad.y);
            (None, Some(goal), Act::None, None)
        }
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
        (false, None, None) if let Some(e) = defend(s, i) => (Some(e), Some(s.actors[e].pos), Act::None, keep),
        (false, None, None) if let Some(goal) = raid_goal(s, i) => {
            let team = s.actors[i].team;
            s.track.raiding[team] += dt;
            (None, Some(goal), Act::None, None)
        }
        (false, None, None) if let Some(goal) = jungle_goal(s, i) => (None, Some(goal), Act::None, None),
        _ => {
            let (goal, act, claim) = haul_plan(s, i);
            let idle = goal.is_none() && matches!(act, Act::None);
            (None, if idle { Some(guard_post(s, i)) } else { goal }, act, claim)
        }
    };
    // Walls: walk round a shield wall, in and out through a bunker's gap, and shoot an enemy
    // bunker whose gap is shut.
    let (goal, breach) = match goal {
        Some(g) => {
            let (g, b) = detour(s, i, g);
            (Some(g), b)
        }
        None => (None, None),
    };
    let speed = MOVE_SPEED * 0.85 * if carrying { CARRY_SPEED } else { ride_speed(&s.actors[i], &s.tv) } * slow_factor(&s.actors[i]);
    let spread = (s.rand() - 0.5) * 0.15;
    let jungling = target.is_none() && act_none(&act) && jungle_goal(s, i).is_some();
    {
        let a = &mut s.actors[i];
        a.claim = claim;
        let goal = goal.map(|g| steer(a, g, dt));
        let want = match goal.map(|g| (g - a.pos).with_y(0.0)) {
            Some(v) if v.length() > 0.4 && a.stun <= 0.0 && a.root <= 0.0 => v.normalize() * speed,
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
    // Set C kits: a self-patch when hurt, adrenaline and a barricade when a fight is close.
    let kit = if carrying { None } else { s.actors[i].kit };
    let fight = target.filter(|&e| s.actors[e].pos.xz().distance(pos.xz()) < 12.0);
    match (kit, fight) {
        (Some((KitKind::SelfPatch, _)), _) if s.actors[i].hp < 50.0 => use_kit(s, i, pos),
        (Some((KitKind::Adrenaline, _)), Some(_)) => use_kit(s, i, pos),
        (Some((KitKind::Barricade, _)), Some(e)) => {
            let to = s.actors[e].pos - pos;
            let team = s.actors[i].team;
            if !s.walls.iter().any(|w| w.team == team && !w.bunker && !w.pass && w.pos.distance(pos) < 6.0) {
                use_kit(s, i, pos + to.with_y(0.0).normalize_or_zero() * (to.length() * 0.4).clamp(2.0, 4.0));
            }
        }
        _ => {}
    }
    // Shield bursts: at a teammate in a fight (or hurt) who has no shield, in sight.
    if !carrying && s.actors[i].alt.is_some_and(|a| a.kind == AltKind::ShieldBurst) {
        let team = s.actors[i].team;
        let reach = s.n(P::BurstRange) * 0.9;
        let mate = (0..s.actors.len())
            .filter(|&j| j != i && s.actors[j].team == team && s.actors[j].alive() && s.actors[j].shield < 10.0 && s.actors[j].tank.is_none())
            .filter(|&j| s.actors[j].hp < 80.0 || nearest_enemy(s, j, s.actors[j].pos, 10.0).is_some())
            .filter(|&j| s.actors[j].pos.distance(pos) < reach && clear(pos.xz(), s.actors[j].pos.xz(), 0.1))
            .min_by(|&x, &y| s.actors[x].hp.total_cmp(&s.actors[y].hp));
        if let Some(j) = mate {
            let dir = (s.actors[j].pos - pos).with_y(0.0).normalize_or_zero();
            use_alt(s, i, dir, 10.0);
        }
    }
    // Med packs: thrown at the most hurt teammate close by (yourself included).
    if !carrying && s.actors[i].toss.is_some_and(|t| t.0 == TossKind::MedPack) {
        let team = s.actors[i].team;
        let mate = (0..s.actors.len())
            .filter(|&j| s.actors[j].team == team && s.actors[j].alive() && s.actors[j].hp < 60.0)
            .filter(|&j| s.actors[j].pos.distance(pos) < 8.0 && clear(pos.xz(), s.actors[j].pos.xz(), 0.1))
            .min_by(|&x, &y| s.actors[x].hp.total_cmp(&s.actors[y].hp));
        if let Some(j) = mate {
            let at = s.actors[j].pos;
            toss(s, i, at);
        }
    }
    let heal_shot = s.actors[i].gun.is_some_and(|g| g.kind == GunKind::Heal) || ride == Some(Ride::AmbulanceShot);
    let mines = s.actors[i].gun.is_some_and(|g| g.kind == GunKind::Mines);
    let hurt_mate = if !carrying && heal_shot {
        let team = s.actors[i].team;
        let reach = if in_tank { 11.0 } else { 9.0 };
        (0..s.actors.len())
            .filter(|&j| j != i && s.actors[j].team == team && s.actors[j].alive() && s.actors[j].hp < 80.0)
            .filter(|&j| s.actors[j].pos.distance(pos) < reach && clear(pos.xz(), s.actors[j].pos.xz(), 0.1))
            .min_by(|&x, &y| s.actors[x].hp.total_cmp(&s.actors[y].hp))
    } else {
        None
    };
    if let Some(m) = hurt_mate {
        let dir = (s.actors[m].pos - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        if s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, 12.0);
        }
    } else if mines {
        // Nail mines: one at your feet when an enemy is close, and (while you have fewer than
        // the most out) round your own pad, against thieves. Never two on top of each other.
        let spaced = !s.mines.iter().any(|m| m.owner == i && m.pos.xz().distance(pos.xz()) < 2.5);
        let close = target.is_some_and(|e| s.actors[e].pos.xz().distance(pos.xz()) < 8.0);
        let out = s.mines.iter().filter(|m| m.owner == i).count();
        let home = pos.xz().distance(pads()[s.actors[i].team]) < 10.0;
        let posted = target.is_none() && home && out < s.n(P::MinesOut) as usize;
        if let Some(e) = target {
            s.actors[i].facing = (s.actors[e].pos - pos).with_y(0.0).normalize_or(s.actors[i].facing);
        }
        if !carrying && spaced && (close || posted) && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            let f = s.actors[i].facing;
            fire(s, i, f, 1.0);
        }
    } else if let Some(e) = target {
        let (ep, ev) = (s.actors[e].pos, s.actors[e].vel.with_y(0.0));
        let d = ep.xz().distance(pos.xz());
        // Lead the target, then miss a little.
        let lead = ep + ev * (d / PROJ_SPEED);
        let dir = Quat::from_rotation_y(spread) * (lead - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        let alt = s.actors[i].alt.map(|a| a.kind);
        let ready = !carrying && s.actors[i].stun <= 0.0;
        // #35: where a blast would land, for bots that mind their allies.
        let rocket_at = pos + dir * d.min(6.0);
        let rocket_ok = !ally_in_way(s, i, dir, d, Some((rocket_at, s.n(P::RocketBlast))));
        let launch_ok = !ally_in_way(s, i, dir, d, Some((lead, s.n(P::LauncherBlast))));
        let grenade_ok = !ally_in_way(s, i, dir, d, Some((lead, s.n(P::GrenadeBlast))));
        let shot_ok = !ally_in_way(s, i, dir, d, None);
        let team = s.actors[i].team;
        let held = (ready && d < 12.0 && alt == Some(AltKind::Rockets) && !rocket_ok) as u32
            + (!carrying && d < 12.0 && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 && !shot_ok) as u32;
        s.track.held[team] += held;
        if ready && d < 12.0 && alt == Some(AltKind::Rockets) && rocket_ok {
            use_alt(s, i, dir, d.min(6.0));
        }
        if ready && launch_ok && d > 4.0 && d < s.n(P::LauncherRange) && alt == Some(AltKind::Launcher) {
            // Short of them: it bounces once on the way in.
            use_alt(s, i, dir, (lead - pos).with_y(0.0).length() * 0.7);
        }
        // Engineers put a wall between them and the fight, if they don't have one close.
        let team = s.actors[i].team;
        let walled = s.walls.iter().any(|w| w.team == team && !w.bunker && w.pos.distance(pos) < 6.0);
        if ready && d < 12.0 && alt == Some(AltKind::ShieldWall) && !walled {
            use_alt(s, i, dir, (d * 0.4).clamp(2.0, 4.0));
        }
        if ready && grenade_ok && d > 4.0 && d < 9.0 && s.actors[i].toss.is_some_and(|t| t.0 == TossKind::Grenade) {
            toss(s, i, lead);
        }
        if !carrying && shot_ok && d < 12.0 && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, 14.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    } else if ride == Some(Ride::Mech) {
        // The stomp only shoves players: nothing else to aim at.
    } else if let Some(at) = breach.or_else(|| wall_to_fix(s, i)) {
        // A shut enemy bunker in the way, or a hurt wall of ours to rivet.
        let dir = (at - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        if !carrying && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, (at - pos).length() + 1.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    } else if let Some(at) = monster_aim(s, i, jungling) {
        let dir = Quat::from_rotation_y(spread) * (at - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        let clear = !ally_in_way(s, i, dir, (at - pos).length(), None);
        if !carrying && clear && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            fire(s, i, dir, (at - pos).length() + 1.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    } else if let Some(at) = pad_aim(s, i) {
        // Nobody to fight: shoot the enemy pad when hits do something there.
        let dir = Quat::from_rotation_y(spread) * (at - pos).with_y(0.0).normalize_or_zero();
        s.actors[i].facing = dir;
        if !carrying && s.actors[i].fire_cd <= 0.0 && s.actors[i].stun <= 0.0 {
            let team = s.actors[i].team;
            s.track.base_shots[team] += 1;
            fire(s, i, dir, (at - pos).length() + 1.0);
            s.actors[i].fire_cd = (s.actors[i].fire_cd * 1.5).max(0.25);
        }
    }
    match act {
        Act::Grab(_) if i == YOU && s.lag_bot => request_grab(s),
        Act::Grab(id) => s.take(i, id),
        Act::Place(at) => {
            let mech = s.mech_cap(i) > 0;
            s.hand_top(i);
            s.place(i, at);
            if mech && pad_contains(pads()[s.actors[i].team], at) {
                s.mech_banked[s.actors[i].team] += 1;
            }
        }
        Act::Drop => {
            s.hand_top(i);
            let at = s.actors[i].pos.with_y(HALF) + s.actors[i].facing * -0.2;
            s.place(i, at);
        }
        Act::Throw(at) => {
            // Jump, then throw near the top of the jump (ADR 0001).
            let a = &mut s.actors[i];
            a.facing = (at - a.pos).with_y(0.0).normalize_or(a.facing);
            if a.grounded && a.stun <= 0.0 {
                a.vel.y = JUMP_VEL;
                a.grounded = false;
            } else if !a.grounded && a.vel.y < 1.0 {
                let from = a.pos + Vec3::Y * HEAD + a.facing * 0.3;
                let head = a.pos + Vec3::Y * HEAD;
                // A thrown crate slides on from where it touches down: aim short, so it
                // comes to rest on the cell.
                let mut aim = at;
                let mut vel = velocity_to(from, aim, THROW_ANGLE.to_radians());
                for _ in 0..6 {
                    let (_, landing, _) = predict(s, from, vel);
                    let off = (landing - at).with_y(0.0);
                    if off.length() < 0.15 {
                        break;
                    }
                    aim -= off * 0.8;
                    vel = velocity_to(from, aim, THROW_ANGLE.to_radians());
                }
                s.release(i, vel, from - head);
            }
        }
        Act::None => {}
    }
}

/// A hauler mech bot: pick up crates until it's full (or there are none close by), then set
/// them down on its pad one by one.
fn mech_plan(s: &Sim, i: usize) -> (Option<Vec3>, Act, Option<u32>) {
    let a = &s.actors[i];
    let (load, cap) = (a.load.len(), s.mech_cap(i));
    let pad = pads()[a.team];
    let claimed = |id: u32| s.actors.iter().enumerate().any(|(j, b)| j != i && b.team == a.team && b.claim == Some(id));
    let pick = s
        .crates
        .iter()
        .filter(|c| c.loose() && c.kind.builds() && c.pos.y < 3.0)
        .filter(|c| !pad_contains(pad, c.pos) && !claimed(c.id))
        .map(|c| (c, c.pos.distance(a.pos) - if c.kind == Kind::Super && s.super_mode > 0 { 8.0 } else { 0.0 }))
        .min_by(|x, y| x.1.total_cmp(&y.1))
        .map(|(c, _)| c);
    // With some on board, only stop for crates close by; else head home.
    let pick = pick.filter(|c| load < cap && (load == 0 || c.pos.distance(a.pos) < 10.0));
    if let Some(c) = pick {
        if c.pos.xz().distance(a.pos.xz()) <= s.reach() + HALF {
            return (None, Act::Grab(c.id), Some(c.id));
        }
        return (Some(c.pos.with_y(0.0)), Act::None, Some(c.id));
    }
    if load == 0 {
        return (None, Act::None, None);
    }
    match s.pad_cell(i) {
        Some(cell) if cell.xz().distance(a.pos.xz()) < 2.0 => (None, Act::Place(cell), None),
        Some(cell) => (Some(cell.with_y(0.0)), Act::None, None),
        None => {
            // The pad is full: set them down beside it.
            let front = pad_front(a.team, a.slot);
            if front.xz().distance(a.pos.xz()) < 1.0 { (None, Act::Drop, None) } else { (Some(front), Act::None, None) }
        }
    }
}

/// Where an ambulance bot goes: next to the most hurt teammate, else the teammate nearest the
/// enemy pad, else its guard post.
fn ambulance_goal(s: &Sim, i: usize) -> Vec3 {
    let a = &s.actors[i];
    let mates = || (0..s.actors.len()).filter(move |&j| j != i && s.actors[j].team == a.team && s.actors[j].alive());
    let enemy_pad = pads()[1 - a.team];
    let mate = mates()
        .filter(|&j| s.actors[j].hp < 90.0)
        .min_by(|&x, &y| s.actors[x].hp.total_cmp(&s.actors[y].hp))
        .or_else(|| mates().min_by(|&x, &y| s.actors[x].pos.xz().distance(enemy_pad).total_cmp(&s.actors[y].pos.xz().distance(enemy_pad))));
    match mate {
        Some(j) => {
            let to = s.actors[j].pos;
            let back = (a.pos - to).with_y(0.0).normalize_or(Vec3::X * -toward_centre(a.team));
            to + back * 2.0
        }
        None => guard_post(s, i),
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
        .collect();
    for a in s.actors.iter_mut().filter(|a| a.alive()) {
        // #35 bloom: the cone closes again once you've stopped firing for a moment.
        a.since_shot += dt;
        if a.since_shot > 0.35 {
            a.bloom = (a.bloom - 2.0 * dt).max(0.0);
        }
        a.vel.y -= G * dt;
        a.pos += (a.vel + a.shove) * dt;
        a.shove *= (-6.0 * dt).exp();
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
    // Jungle walls block everyone; so do standing bases (#15).
    let bases: Vec<Block> = (0..2).filter(|&t| s.loss != THEFT && s.base_down[t].is_finite()).map(base_block).collect();
    for a in s.actors.iter_mut().filter(|a| a.alive()) {
        for b in blocks().iter().chain(&bases) {
            let mut xz = a.pos.xz();
            if b.push_out(&mut xz, ACTOR_R).is_some() {
                a.pos.x = xz.x;
                a.pos.z = xz.y;
            }
        }
    }
    // Shield walls and bunkers block everyone; a bunker gate and a Set C gate only enemies.
    // Your team jumps over its super barricade. A super gate slows the enemies it holds.
    let gate_slow = s.tv[P::GateSlow as usize];
    for a in s.actors.iter_mut().filter(|a| a.alive()) {
        for w in s.walls.iter().filter(|w| w.blocks_body(a.team)) {
            if w.barricade && w.gold && w.team == a.team && a.pos.y > 0.35 {
                continue;
            }
            let mut xz = a.pos.xz();
            if w.push_out(&mut xz, ACTOR_R).is_some() {
                a.pos.x = xz.x;
                a.pos.z = xz.y;
                if w.pass {
                    s.gate_held[a.team] += dt;
                    if w.gold && a.slow_immune <= 0.0 {
                        a.slow = a.slow.max(gate_slow);
                    }
                }
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

/// Carried crates ride on the carrier's head; a hauler mech's stack on its roof.
fn crates_physics(s: &mut Sim) {
    for c in &mut s.crates {
        if let CrateState::Carried { by, lift_from, lift_t } = &mut c.state {
            let a = &s.actors[*by];
            let head = match a.load.iter().position(|&id| id == c.id) {
                Some(k) => a.pos + Vec3::Y * (MECH_TOP + HALF + k as f32 * CRATE),
                None => a.pos + Vec3::Y * HEAD,
            };
            c.pos = lift_from.lerp(head, (*lift_t / 0.1).min(1.0));
        }
    }
}

fn crates_step(s: &mut Sim, dt: f32) {
    let snapshot: Vec<(u32, Vec3, bool)> =
        s.crates.iter().map(|c| (c.id, c.pos, c.state == CrateState::Free)).collect();
    let mut thrown_hits = Vec::new();
    let ff_crates = s.ff[2] == 1;
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
                    |q| floor_under(q, others()),
                    dt,
                );
                if grounded {
                    c.thrown_by = None;
                }
                if let Some(by) = c.thrown_by {
                    let team = s.actors.get(by).map_or(0, |a| a.team);
                    let hit = s.actors.iter().enumerate().position(|(j, o)| {
                        (o.team != team || ff_crates)
                            && j != by
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
    // Shield walls stop sliding and low-flying crates.
    for c in s.crates.iter_mut().filter(|c| c.loose()) {
        for w in s.walls.iter().filter(|w| !w.gate && c.pos.y - HALF < w.h) {
            let mut xz = c.pos.xz();
            if let Some(n) = w.push_out(&mut xz, HALF) {
                c.pos.x = xz.x;
                c.pos.z = xz.y;
                let into = c.vel.xz().dot(n);
                if into < 0.0 {
                    c.vel -= (n * into * 1.4).extend(0.0).xzy();
                }
                c.settling = false;
            }
        }
    }
    // Curling knocks.
    let n = s.crates.len();
    // #35: a moving crate's team (whoever last let go of it), and whether `still` rests on
    // that team's own pad.
    let team_of = |c: &Crate| c.placed_by.and_then(|w| s.actors.get(w)).map(|a| a.team);
    let teams: Vec<Option<usize>> = s.crates.iter().map(team_of).collect();
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, b) = (&s.crates[i], &s.crates[j]);
            if a.state != CrateState::Free || b.state != CrateState::Free || !side_hit(a.pos, b.pos) || (a.locked && b.locked) {
                continue;
            }
            let own_pad = |still: &Crate, mover: &Crate, t: Option<usize>| {
                t.is_some_and(|t| pad_contains(pads()[t], still.pos) && still.kind.builds())
                    && still.vel.xz().length() < 0.3
                    && mover.vel.xz().length() > 0.3
            };
            let guard = if own_pad(a, b, teams[j]) { Some((i, j)) } else if own_pad(b, a, teams[i]) { Some((j, i)) } else { None };
            if let Some((_, mover)) = guard {
                let t = teams[mover].unwrap_or(0);
                if s.ff[3] == 0 {
                    // Off: your team's crate stops dead against your own pad's crates.
                    let (still, m) = guard.unwrap();
                    let d = s.crates[m].pos.xz() - s.crates[still].pos.xz();
                    let (axis, pen) = if (CRATE - d.x.abs()) < (CRATE - d.y.abs()) {
                        (Vec2::new(d.x.signum(), 0.0), CRATE - d.x.abs())
                    } else {
                        (Vec2::new(0.0, d.y.signum()), CRATE - d.y.abs())
                    };
                    let c = &mut s.crates[m];
                    c.pos += (axis * pen).extend(0.0).xzy();
                    if c.vel.xz().dot(axis) < 0.0 {
                        s.track.knocks_stopped[t] += 1;
                        c.vel = Vec3::new(0.0, c.vel.y.min(0.0), 0.0);
                    }
                    continue;
                }
                if (a.vel.xz() - b.vel.xz()).length() > 1.0 {
                    s.track.own_knocks[t] += 1;
                }
            }
            let (a, b) = (&s.crates[i], &s.crates[j]);
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
        if s.actors[who].team == team {
            s.track.crate_ff[team] += 1;
            let what = format!("{} hit {} with a thrown crate", if by == YOU { "YOU".to_string() } else { s.describe(by) }, if who == YOU { "YOU".to_string() } else { s.describe(who) });
            s.say(format!("TEAMMATE HIT: {what}"));
        }
        s.hurt(who, SHOT_DMG * 2.0, dir * 6.0, Some(by), team);
    }
}

fn shots_step(s: &mut Sim, dt: f32) {
    let mut hits = Vec::new();
    let mut heals = Vec::new();
    let mut booms = Vec::new();
    let mut monster_hits = Vec::new();
    let mut shields = Vec::new();
    let mut base_hits = Vec::new();
    for sh in &mut s.shots {
        sh.pos += sh.vel * dt;
        sh.life -= dt;
        sh.age += dt;
        let body = |a: &Actor| {
            let r = if a.tank.is_some() { 0.9 } else { ACTOR_R + 0.15 };
            a.alive() && sh.pos.xz().distance(a.pos.xz()) < r && sh.pos.y > a.pos.y && sh.pos.y < a.pos.y + 1.9
        };
        // Shield bursts stop on the first player they touch; an ally gets the shield.
        if sh.shield > 0.0
            && let Some(j) = (0..s.actors.len()).find(|&j| Some(j) != sh.owner && body(&s.actors[j]))
        {
            if s.actors[j].team == sh.team {
                shields.push((j, sh.shield, sh.cleanse, sh.team));
            }
            sh.life = 0.0;
            continue;
        }
        // Heal shots heal the first hurt teammate they touch (not the medic who fired).
        if sh.heal > 0.0
            && let Some(j) = (0..s.actors.len()).find(|&j| {
                let a = &s.actors[j];
                a.team == sh.team && Some(j) != sh.owner && Some(j) != sh.last && a.hp < 100.0 && body(a)
            })
        {
            heals.push((j, sh.heal, sh.bounce, sh.owner, sh.team, sh.amb));
            sh.life = 0.0;
            continue;
        }
        // #35: with friendly fire on, bullets (and rockets, under the blasts rule) stop on
        // allies too. Heal shots and shield bursts never hurt allies; nothing hits whoever
        // fired it.
        let ff = sh.heal == 0.0 && sh.shield == 0.0 && if sh.splash > 0.0 { s.ff[1] == 2 } else { s.ff[0] == 1 };
        let hit = (0..s.actors.len()).find(|&j| {
            (s.actors[j].team != sh.team || ff) && Some(j) != sh.owner && Some(j) != sh.last && body(&s.actors[j])
        });
        if let Some(j) = hit {
            if sh.splash > 0.0 {
                booms.push(Boom { at: sh.pos, team: sh.team, dmg: sh.dmg, r: sh.splash, owner: sh.owner, split: sh.split, slow: sh.slow });
                sh.life = 0.0;
            } else {
                hits.push((j, sh.vel.normalize(), sh.owner, sh.team, sh.dmg, sh.stagger));
                // A piercing shot goes on through one more.
                if sh.pierce > 0 {
                    sh.pierce -= 1;
                    sh.last = Some(j);
                } else {
                    sh.life = 0.0;
                }
            }
            continue;
        }
        let boom = Boom { at: sh.pos, team: sh.team, dmg: sh.dmg, r: sh.splash, owner: sh.owner, split: sh.split, slow: sh.slow };
        if blocked(sh.pos.xz(), 0.0) {
            if sh.splash > 0.0 {
                booms.push(boom);
            }
            sh.life = 0.0;
            continue;
        }
        if let Some(m) = s.monsters.iter().position(|m| m.alive() && m.pos.xz().distance(sh.pos.xz()) < 0.9 && sh.pos.y < 2.2) {
            if sh.splash > 0.0 {
                booms.push(boom);
            } else {
                monster_hits.push((m, sh.dmg, sh.owner));
            }
            sh.life = 0.0;
            continue;
        }
        // Bases (#15): an enemy shot hurts one; a blast goes off on it.
        if let Some(t) = (0..2).find(|&t| {
            t != sh.team && s.loss != THEFT && s.base_down[t] <= 0.0 && sh.pos.y < BASE_H && base_block(t).hits(sh.pos.xz(), 0.1)
        }) {
            if sh.splash > 0.0 {
                booms.push(boom);
            } else if sh.heal == 0.0 && sh.shield == 0.0 {
                base_hits.push((t, sh.dmg));
            }
            sh.life = 0.0;
            continue;
        }
        // Walls stop shots from both teams. Enemy shots hurt them; a rivet repairs its own
        // team's wall; a super shield wall lets its own team's shots through.
        if let Some(w) = s.walls.iter_mut().find(|w| !w.gate && w.touches(sh.pos.xz(), 0.1) && sh.pos.y < w.h) {
            if w.team != sh.team {
                if sh.splash == 0.0 {
                    w.hp -= sh.dmg;
                    s.wall_dmg[w.team] += sh.dmg;
                }
            } else if sh.rivet {
                let add = s.tv[P::RivetRepair as usize].min(w.max_hp - w.hp).max(0.0);
                w.hp += add;
                s.repaired[w.team] += add;
            } else if w.pass || (w.gold && !w.bunker && !w.barricade) {
                // Your own gate, or your super shield wall: your shots pass.
                continue;
            }
            if sh.splash > 0.0 {
                booms.push(boom);
            }
            sh.life = 0.0;
            continue;
        }
        if sh.life <= 0.0 && sh.splash > 0.0 {
            booms.push(boom);
        }
    }
    s.shots.retain(|sh| {
        if sh.life <= 0.0 { s.despawn.extend(sh.ent); }
        sh.life > 0.0
    });
    for (who, dir, owner, team, dmg, stagger) in hits {
        if owner == Some(YOU) {
            s.hits += 1;
        }
        if s.actors[who].team != team {
            s.track.hit[team] += 1;
        }
        s.hurt(who, dmg, dir * 1.5, owner, team);
        if stagger > 0.0 {
            let a = &mut s.actors[who];
            a.stun = a.stun.max(0.15 + stagger);
        }
    }
    for (j, heal, bounce, owner, team, amb) in heals {
        if amb {
            s.amb_healed[team] += heal.min(100.0 - s.actors[j].hp).max(0.0);
            // A super ambulance's heal shot carries a 25 shield every 5 s.
            if let Some(t) = owner.and_then(|o| s.actors[o].tank.as_mut())
                && t.gold
                && t.beat >= 5.0
            {
                t.beat = 0.0;
                s.actors[j].shield = (s.actors[j].shield + 25.0).min(100.0);
            }
        }
        s.actors[j].hp = (s.actors[j].hp + heal).min(100.0);
        s.booms.push((s.actors[j].pos + Vec3::Y, 0.5, 0.15));
        // Super heal: a second heal shot flies on to the nearest other hurt ally within 4 m.
        let from = s.actors[j].pos;
        let next = (0..s.actors.len())
            .filter(|&m| m != j && Some(m) != owner && s.actors[m].team == team && s.actors[m].alive() && s.actors[m].hp < 100.0)
            .filter(|&m| s.actors[m].pos.distance(from) < 4.0)
            .min_by(|&x, &y| s.actors[x].pos.distance(from).total_cmp(&s.actors[y].pos.distance(from)));
        if bounce && let Some(m) = next {
            let dir = (s.actors[m].pos - from).with_y(0.0).normalize_or_zero();
            let start = from + Vec3::Y * CHEST + dir * 0.6;
            s.shots.push(Shot { heal, last: Some(j), ..Shot::new(start, dir * PROJ_SPEED, owner, team, heal, 5.0 / PROJ_SPEED) });
        }
    }
    for (j, shield, cleanse, team) in shields {
        let a = &mut s.actors[j];
        let add = shield.min(100.0 - a.shield).max(0.0);
        a.shield += add;
        if cleanse {
            a.slow = 0.0;
            a.root = 0.0;
        }
        s.burst_given[team] += add;
        s.booms.push((s.actors[j].pos + Vec3::Y, 0.6, 0.15));
    }
    for (m, dmg, owner) in monster_hits {
        s.hurt_monster(m, dmg, owner);
    }
    for (t, dmg) in base_hits {
        s.hurt_base(t, dmg);
    }
    for b in booms {
        blast(s, b);
    }
}

/// A blast goes off: hurts and pushes, and a super rocket's splits into 3 small blasts and
/// a super tank shell leaves a slow zone.
fn blast(s: &mut Sim, b: Boom) {
    s.explode(b.at, b.team, b.dmg, b.r, b.owner);
    s.booms.push((b.at, b.r, 0.3));
    if b.split {
        for k in 0..3 {
            let off = Vec2::from_angle(k as f32 * 2.09 + 0.5) * b.r * 0.8;
            let at = b.at + off.extend(0.0).xzy();
            if !blocked(at.xz(), 0.0) {
                s.explode(at, b.team, b.dmg * 0.35, b.r * 0.5, b.owner);
                s.booms.push((at, b.r * 0.5, 0.3));
            }
        }
    }
    if b.slow {
        let life = s.n(P::SlowZone);
        s.zones.push((b.at.with_y(0.0), b.r, life, b.team));
    }
}

/// Grenades and med packs in flight. A grenade bounces (a super one sticks to the first
/// thing it touches) and goes off when its fuse runs out; a med pack lands as a pickup.
fn lobs_step(s: &mut Sim, dt: f32) {
    let arena = arena();
    let mut split = Vec::new();
    let mut k = 0;
    while k < s.lobs.len() {
        let l = &mut s.lobs[k];
        l.fuse -= dt;
        match l.stuck {
            Some(Some((j, off))) => {
                if s.actors[j].alive() {
                    l.pos = s.actors[j].pos + off;
                } else {
                    l.stuck = Some(None);
                }
            }
            Some(None) => {}
            None => {
                l.vel.y -= G * dt;
                l.pos += l.vel * dt;
                let mut touched = false;
                if l.pos.x.abs() > arena.x - 0.2 || l.pos.z.abs() > arena.y - 0.2 {
                    l.pos.x = l.pos.x.clamp(-arena.x + 0.2, arena.x - 0.2);
                    l.pos.z = l.pos.z.clamp(-arena.y + 0.2, arena.y - 0.2);
                    l.vel.x *= -0.4;
                    l.vel.z *= -0.4;
                    touched = true;
                }
                for b in blocks() {
                    let mut xz = l.pos.xz();
                    if let Some(n) = b.push_out(&mut xz, 0.15) {
                        l.pos = Vec3::new(xz.x, l.pos.y, xz.y);
                        let into = l.vel.xz().dot(n);
                        if into < 0.0 {
                            l.vel -= (n * into * 1.5).extend(0.0).xzy();
                        }
                        touched = true;
                    }
                }
                let (y, team) = (l.pos.y, l.team);
                for w in s.walls.iter().filter(|w| !w.gate && y < w.h && !(w.pass && w.team == team)) {
                    let mut xz = l.pos.xz();
                    if let Some(n) = w.push_out(&mut xz, 0.15) {
                        l.pos = Vec3::new(xz.x, l.pos.y, xz.y);
                        let into = l.vel.xz().dot(n);
                        if into < 0.0 {
                            l.vel -= (n * into * 1.5).extend(0.0).xzy();
                        }
                        touched = true;
                    }
                }
                let on_floor = l.pos.y <= 0.15;
                if on_floor {
                    l.pos.y = 0.15;
                    l.vel.y = if l.vel.y < -3.0 { -l.vel.y * 0.3 } else { 0.0 };
                    l.vel.x *= 0.6;
                    l.vel.z *= 0.6;
                    touched = true;
                }
                let body = (0..s.actors.len()).find(|&j| {
                    let a = &s.actors[j];
                    j != l.owner && a.alive() && a.pos.xz().distance(l.pos.xz()) < ACTOR_R + 0.2 && l.pos.y > a.pos.y && l.pos.y < a.pos.y + 1.9
                });
                if l.kind == TossKind::Grenade && l.gold {
                    if let Some(j) = body {
                        l.stuck = Some(Some((j, l.pos - s.actors[j].pos)));
                    } else if touched {
                        l.stuck = Some(None);
                        l.vel = Vec3::ZERO;
                    }
                }
                if l.kind == TossKind::MedPack && on_floor {
                    let (pos, gold) = (l.pos.with_y(0.5), l.gold);
                    s.lobs.remove(k);
                    s.pickups.push(Pickup { pos, drop: Drop::MedPack(gold), life: 30.0 });
                    continue;
                }
                // Set C: launcher grenades bounce once, then go off on the next touch (or on
                // an enemy); a super one splits into 3 on the bounce. Shells go off where they
                // land.
                if matches!(l.kind, TossKind::Launched | TossKind::Shell) {
                    let on_enemy = body.is_some_and(|j| s.actors[j].team != l.team);
                    let go = match l.kind {
                        TossKind::Shell => touched || on_enemy,
                        _ => on_enemy || (touched && l.hops >= 1) || l.fuse <= 0.0,
                    };
                    if !go && touched {
                        l.hops = 1;
                        l.vel.y = l.vel.y.max(4.0);
                        if l.gold && !l.small {
                            l.small = true;
                            for turn in [-0.6f32, 0.6] {
                                let vel = Quat::from_rotation_y(turn) * l.vel;
                                split.push(Lob { vel, ..*l });
                            }
                        }
                    }
                    if go {
                        let l = s.lobs.remove(k);
                        let shell = l.kind == TossKind::Shell;
                        let (dmg, r) = if shell { (s.n(P::ShellDmg), s.n(P::ShellBlast)) } else { (s.n(P::LauncherDmg), s.n(P::LauncherBlast)) };
                        let (dmg, r) = if l.small { (dmg * 0.5, r * 0.75) } else { (dmg, r) };
                        let at = l.pos.with_y(l.pos.y.max(0.5));
                        if shell {
                            let hit = (0..s.actors.len())
                                .filter(|&j| s.actors[j].team != l.team && s.actors[j].alive() && s.actors[j].pos.xz().distance(at.xz()) < r)
                                .fold(0u64, |m, j| m | 1 << j);
                            if let Some(b) = s.barrages.iter_mut().find(|b| b.id == l.wave) {
                                b.hit |= hit;
                            }
                        }
                        let b = Boom { at, team: l.team, dmg: dmg * s.k(l.gold), r, owner: Some(l.owner), split: false, slow: shell && l.gold };
                        blast(s, b);
                        continue;
                    }
                }
            }
        }
        let l = &s.lobs[k];
        if l.kind == TossKind::Grenade && l.fuse <= 0.0 {
            let (k_, r, dmg) = (s.k(l.gold), s.n(P::GrenadeBlast), s.n(P::GrenadeDmg));
            let b = Boom { at: l.pos.with_y(l.pos.y.max(0.5)), team: l.team, dmg: dmg * k_, r, owner: Some(l.owner), split: false, slow: false };
            s.lobs.remove(k);
            blast(s, b);
            continue;
        }
        k += 1;
    }
    s.lobs.extend(split);
    // Slow zones from a super tank's cannon.
    for z in &mut s.zones {
        z.2 -= dt;
    }
    s.zones.retain(|z| z.2 > 0.0);
    for a in s.actors.iter_mut().filter(|a| a.alive() && a.slow_immune <= 0.0) {
        if s.zones.iter().any(|z| z.3 != a.team && z.0.xz().distance(a.pos.xz()) < z.1) {
            a.slow = 0.1;
        }
    }
    for r in &mut s.rally {
        *r = (*r - dt).max(0.0);
    }
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
        // Lock-in (#15): a crate that has rested in a cell long enough locks in and scores.
        let lock_in = s.lock_in && s.pad_up(team);
        let lock_time = s.tv[P::LockInTime as usize];
        for c in s.crates.iter_mut().filter(|c| c.loose() && c.kind.builds() && pad_contains(pad, c.pos)) {
            if c.pos.y > HALF + 2.0 * CRATE - 0.1 && c.vel.length() < 0.5 {
                let out = (c.pos.xz() - pad).normalize_or(Vec2::X);
                c.vel = Vec3::new(out.x * 3.0, 2.0, out.y * 3.0);
                continue;
            }
            let level = (c.pos.y - HALF) / CRATE;
            let in_cell = cells.iter().any(|q| q.distance(c.pos.xz()) < 0.02) && (level - level.round()).abs() < 0.03;
            let resting = in_cell && c.vel.length() < 0.05 && !c.settling;
            c.pad_t = if resting { c.pad_t + dt } else { 0.0 };
            if lock_in && c.pad_t >= lock_time {
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
    for team in 0..2 {
        if s.walls.iter().any(|w| w.team == team && w.bunker) {
            s.bunker_time[team] += dt;
        }
        if s.walls.iter().any(|w| w.team == team && w.pass) {
            s.gate_time[team] += dt;
        }
    }
    for w in &mut s.walls {
        w.life -= dt;
    }
    let had = [0, 1].map(|t| s.walls.iter().any(|w| w.team == t && w.bunker));
    let mut broke = Vec::new();
    s.walls.retain(|w| {
        let gone = w.life <= 0.0 || w.hp <= 0.0;
        if gone {
            s.despawn.extend(w.ent);
            if w.hp <= 0.0 {
                broke.push((w.team, w.bunker));
            }
        }
        !gone
    });
    // A breached bunker is open: its gate goes with the first wall.
    for &(team, bunker) in &broke {
        if bunker {
            for w in s.walls.iter_mut().filter(|w| w.team == team && w.gate) {
                w.life = 0.0;
            }
        }
    }
    for (team, bunker) in broke {
        let what = if bunker { "bunker wall breached" } else { "wall broken" };
        s.say(format!("{} {what}", ["BLUE", "RED"][team]));
    }
    for team in 0..2 {
        if had[team] && !s.walls.iter().any(|w| w.team == team && w.bunker) {
            let msg = format!("{} bunker is down: all 8 open again", ["BLUE", "RED"][team]);
            println!("t={:.1}s {msg}", s.t);
            s.say(msg);
        }
    }
    for b in &mut s.booms {
        b.2 -= dt;
    }
    s.booms.retain(|b| b.2 > 0.0);
}

fn vehicles_step(s: &mut Sim, dt: f32) {
    let out = s.actors.iter().filter(|a| a.tank.is_some()).count();
    s.rides_max = s.rides_max.max(out);
    if s.actors.iter().any(|a| a.tank.is_some_and(|t| matches!(t.ride, Ride::Ambulance | Ride::AmbulanceShot))) {
        s.amb_time += dt;
    }
    for i in 0..s.actors.len() {
        let Some(t) = &mut s.actors[i].tank else { continue };
        t.life -= dt;
        t.beat += dt;
        let (t, team, pos) = (*t, s.actors[i].team, s.actors[i].pos);
        s.ride_time[team] += dt;
        if t.ride == Ride::Ambulance {
            // The aura: allies close by (the driver too) heal on their own. Super: a 25
            // shield every 5 s.
            let (r, heal) = (s.n(P::AmbRadius), s.n(P::AmbHeal) * s.k(t.gold) * dt);
            let shield = t.gold && t.beat >= 5.0;
            for b in s.actors.iter_mut().filter(|b| b.team == team && b.alive() && b.pos.xz().distance(pos.xz()) < r) {
                let add = heal.min(100.0 - b.hp).max(0.0);
                b.hp += add;
                s.amb_healed[team] += add;
                if shield {
                    b.shield = (b.shield + 25.0).min(100.0);
                }
            }
            if shield {
                s.booms.push((pos + Vec3::Y * 0.2, r, 0.3));
            }
        }
        if t.ride == Ride::Ambulance && t.beat >= 5.0
            && let Some(t) = &mut s.actors[i].tank
        {
            t.beat = 0.0;
        }
        if t.life <= 0.0 {
            s.end_ride(i, "ran out of time");
        }
    }
}

/// Nail mines: the first enemy to step on an armed one takes the damage and is rooted (a
/// super mine also shows them on your minimap).
fn mines_step(s: &mut Sim, dt: f32) {
    let (dmg, root, reveal) = (s.n(P::MineDmg), s.n(P::MineRoot), s.n(P::MineReveal));
    let mut k = 0;
    while k < s.mines.len() {
        let m = &mut s.mines[k];
        m.arm -= dt;
        let (pos, team) = (m.pos, m.team);
        let victim = (m.arm <= 0.0)
            .then(|| (0..s.actors.len()).find(|&j| {
                let a = &s.actors[j];
                a.team != team && a.alive() && a.pos.y < 0.5 && a.pos.xz().distance(pos.xz()) < MINE_R + ACTOR_R * 0.5
            }))
            .flatten();
        let Some(j) = victim else {
            k += 1;
            continue;
        };
        let m = s.mines.remove(k);
        s.mine_hits[team] += 1;
        if m.owner == YOU {
            s.hits += 1;
        }
        s.hurt(j, dmg * s.k(m.gold), Vec3::ZERO, Some(m.owner), team);
        let a = &mut s.actors[j];
        a.root = a.root.max(root);
        if m.gold {
            a.reveal = reveal;
        }
        s.booms.push((pos + Vec3::Y * 0.2, MINE_R, 0.3));
        let who = if j == YOU { "you".to_string() } else { s.describe(j) };
        s.say(format!("{who} stepped on a {}mine", if m.gold { "SUPER " } else { "" }));
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
            let team = s.actors[j].team;
            let pack = s.n(P::MedPackHp);
            let a = &mut s.actors[j];
            let what = match drop {
                Drop::Heal => {
                    a.hp = (a.hp + 60.0).min(100.0);
                    s.camp_heals[team] += 1;
                    "a heal (+60 hp)"
                }
                Drop::MedPack(gold) => {
                    a.hp = (a.hp + pack).min(100.0);
                    if gold {
                        a.shield = (a.shield + 25.0).min(100.0);
                    }
                    s.pack_heals[team] += 1;
                    if gold { "a SUPER med pack" } else { "a med pack" }
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
    let coming = s.marked.iter().filter(|m| m.1 == Kind::Normal).count();
    if loose + coming < want {
        let a = arena();
        // On the lanes map new crates land in the contested middle: the river and mid lane.
        let span = Vec2::new(a.x * if s.map { layout().mid_span } else { 0.4 }, a.y * 0.8);
        // #35 (key 5): a random spot or the next fixed one, dropped now or marked first.
        let p = if s.luck[0] == 2 { fixed_spot(s, span) } else { open_spot(|| s.rand(), span) };
        if p.x.abs() > 0.1 {
            s.track.spawn_side[(p.x > 0.0) as usize] += 1;
        }
        if s.luck[0] == 0 {
            drop_crate(s, Kind::Normal, p);
        } else {
            let warn = s.n(P::SpawnWarn);
            s.marked.push((Vec3::new(p.x, 0.0, p.y), Kind::Normal, warn));
        }
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
    mega: Handle<StandardMaterial>,
    monster_mesh: Handle<Mesh>,
    monster_mat: [Handle<StandardMaterial>; 2],
    marker: Handle<Mesh>,
    camp: Handle<StandardMaterial>,
    wall_mesh: Handle<Mesh>,
    wall_mat: [Handle<StandardMaterial>; 2],
    shot_mesh: Handle<Mesh>,
    shot_mat: [Handle<StandardMaterial>; 2],
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
    sim: Res<Sim>,
    mut ground: Single<&mut Transform, With<Ground>>,
    mut pad_views: Query<(&PadView, &mut Transform), (Without<Ground>, Without<BaseView>)>,
    mut base_views: Query<(&BaseView, &mut Transform, &mut Visibility), (Without<Ground>, Without<PadView>)>,
) {
    let a = arena();
    ground.scale = Vec3::new(a.x * 2.0, 1.0, a.y * 2.0);
    for (p, mut tf) in &mut pad_views {
        let pad = pads()[p.0];
        tf.translation = Vec3::new(pad.x, 0.01, pad.y);
    }
    // A base shrinks as it takes damage, and lies as rubble while it's down.
    for (b, mut tf, mut vis) in &mut base_views {
        let t = b.0;
        *vis = if sim.loss == THEFT { Visibility::Hidden } else { Visibility::Visible };
        let k = if sim.pad_up(t) { (sim.base_hp[t] / sim.n(P::BaseHp)).clamp(0.2, 1.0) } else { 0.1 };
        let p = base_pos(t);
        tf.translation = Vec3::new(p.x, BASE_H * k / 2.0, p.y);
        tf.scale = Vec3::new(1.0, k, 1.0);
    }
}

#[derive(Component)]
struct BaseView(usize);

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

/// The map's static meshes (walls, canal, floors), rebuilt when the map changes.
#[derive(Resource, Default)]
struct MapView {
    built: Option<(usize, bool)>,
    ents: Vec<Entity>,
}

fn sync_map(
    mut commands: Commands,
    sim: Res<Sim>,
    mut view: ResMut<MapView>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let key = (sim.map_i, sim.rotate);
    if view.built == Some(key) {
        return;
    }
    for e in view.ents.drain(..) {
        commands.entity(e).despawn();
    }
    view.built = Some(key);
    if !sim.map {
        return;
    }
    let l = layout();
    let wall = mats.add(Color::srgb(0.16, 0.3, 0.14));
    let canal = mats.add(Color::srgb(0.12, 0.25, 0.5));
    for (b, &wet) in l.blocks.iter().zip(&l.water) {
        let size = b.max - b.min;
        let c = (b.min + b.max) / 2.0;
        // The canal is fenced: like a wall, nothing crosses it but the bridges.
        let h = BLOCK_H;
        let e = commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(size.x, h, size.y))),
                MeshMaterial3d(if wet { canal.clone() } else { wall.clone() }),
                Transform::from_xyz(c.x, h / 2.0, c.y),
            ))
            .id();
        view.ents.push(e);
    }
    // Floors: jungle darker, lanes lighter, the middle (river, plaza, bridges) blue-grey.
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
    for &(b, kind) in &l.floors {
        let (color, y) = match kind {
            Floor::Jungle => (Color::srgb(0.2, 0.27, 0.18), 0.005),
            Floor::Field => (Color::srgb(0.36, 0.42, 0.28), 0.005),
            Floor::Lane => (Color::srgb(0.48, 0.45, 0.36), 0.006),
            Floor::Water => (Color::srgb(0.12, 0.25, 0.5), 0.007),
            Floor::Middle => (Color::srgb(0.22, 0.38, 0.55), 0.008),
        };
        floor(&mut commands, (b.min + b.max) / 2.0, b.max - b.min, color, y);
    }
    for (team, color) in TEAM_COLOR.into_iter().enumerate() {
        let x = (l.half.x - l.base / 2.0) * if team == 0 { -1.0 } else { 1.0 };
        floor(&mut commands, Vec2::new(x, 0.0), Vec2::new(l.base, l.half.y * 2.0), color.mix(&Color::srgb(0.3, 0.33, 0.3), 0.75), 0.004);
    }
}

/// Sizes the minimap to the bottom-right corner, and turns it off in the open arena.
fn minimap_view(sim: Res<Sim>, window: Single<&Window>, mut cam: Single<(&mut Camera, &mut Projection), With<Minimap>>) {
    let (cam, proj) = &mut *cam;
    cam.is_active = sim.map;
    if !sim.map {
        return;
    }
    let half = layout().half;
    if let Projection::Orthographic(o) = &mut **proj {
        o.scaling_mode = ScalingMode::Fixed { width: half.x * 2.0 + 2.0, height: half.y * 2.0 + 2.0 };
    }
    let size = window.physical_size();
    let w = (size.x as f32 * (0.3 * half.x / 45.0).min(0.3)) as u32;
    let h = (w as f32 * (half.y + 1.0) / (half.x + 1.0)) as u32;
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
    tune: Res<Tune>,
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
    let fog = tune.c(P::EnemyMap) == 0;
    for (_, marker, mut tf, mut vis) in &mut markers {
        let (pos, alive, scale) = match marker {
            // Enemies show only while your team can see them, or a super mine reveals them.
            Marker::Actor(i) => {
                let a = &sim.actors[*i];
                let shown = a.team == 0 || !fog || a.reveal > 0.0 || in_sight(&sim, *i);
                (a.pos, a.alive() && shown, if *i == YOU { 1.3 } else if a.reveal > 0.0 { 1.8 } else { 1.0 })
            }
            Marker::Monster(m) => (sim.monsters[*m].pos, sim.monsters[*m].alive(), 0.8),
        };
        tf.translation = pos.with_y(30.0);
        tf.scale = Vec3::splat(scale);
        *vis = if alive && sim.map { Visibility::Visible } else { Visibility::Hidden };
    }
}

/// Whether a living blue player can see actor `i`: within 20 m with no jungle wall between.
fn in_sight(sim: &Sim, i: usize) -> bool {
    let p = sim.actors[i].pos.xz();
    sim.actors.iter().any(|a| a.team == 0 && a.alive() && a.pos.xz().distance(p) < 20.0 && clear(a.pos.xz(), p, 0.1))
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
            scaling_mode: ScalingMode::Fixed { width: 92.0, height: 52.0 },
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
    for (team, color) in TEAM_COLOR.into_iter().enumerate() {
        commands.spawn((
            BaseView(team),
            Mesh3d(meshes.add(Cuboid::new(BASE_HALF * 2.0, BASE_H, BASE_HALF * 2.0))),
            MeshMaterial3d(mats.add(StandardMaterial { base_color: color, emissive: color.to_linear() * 0.3, ..default() })),
            Transform::default(),
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
        mega: mats.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.8, 0.1),
            emissive: LinearRgba::rgb(2.0, 1.4, 0.1),
            ..default()
        }),
        monster_mesh: meshes.add(Sphere::new(0.8)),
        monster_mat: [mats.add(Color::srgb(0.55, 0.4, 0.2)), mats.add(Color::srgb(0.6, 0.25, 0.7))],
        marker: meshes.add(Cylinder::new(1.1, 0.1)),
        camp: mats.add(StandardMaterial { base_color: Color::srgb(1.0, 0.85, 0.2), unlit: true, ..default() }),
        wall_mesh: meshes.add(Cuboid::new(WALL_HALF_T * 2.0, WALL_H, 1.0)),
        wall_mat: TEAM_COLOR.map(|c| {
            mats.add(StandardMaterial { base_color: c.with_alpha(0.45), alpha_mode: AlphaMode::Blend, ..default() })
        }),
        shot_mesh: meshes.add(Sphere::new(0.15)),
        shot_mat: [
            mats.add(StandardMaterial { emissive: LinearRgba::rgb(0.6, 1.4, 3.0), ..default() }),
            mats.add(StandardMaterial { emissive: LinearRgba::rgb(3.0, 0.6, 0.3), ..default() }),
        ],
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
    // #36: crates as you'd see them online, and removed ones until the news arrives.
    let views: Vec<Option<Seen>> = sim.crates.iter().map(|c| crate_view(&sim, c)).collect();
    let ghost_views: Vec<Option<Seen>> = sim.ghosts.iter().map(|g| ghost_view(&sim, g)).collect();
    for (g, v) in sim.ghosts.iter().zip(&ghost_views) {
        if let Some(e) = g.ent
            && let Ok((mut tf, mut vis)) = q.get_mut(e)
        {
            if let Some(v) = v {
                tf.translation = v.pos;
            }
            *vis = if v.is_some() { Visibility::Visible } else { Visibility::Hidden };
        }
    }
    for (c, v) in sim.crates.iter_mut().zip(views) {
        match c.ent {
            Some(e) => {
                if let Ok((mut tf, mut vis)) = q.get_mut(e) {
                    if let Some(v) = v {
                        tf.translation = v.pos;
                    }
                    *vis = if v.is_some() { Visibility::Visible } else { Visibility::Hidden };
                }
            }
            None => {
                let mat = match c.kind {
                    Kind::Loot => looks.loot.clone(),
                    Kind::Super => looks.sup.clone(),
                    Kind::Mega => looks.mega.clone(),
                    Kind::Normal => looks.normal.clone(),
                };
                let vis = if v.is_some() { Visibility::Visible } else { Visibility::Hidden };
                c.ent = Some(
                    commands
                        .spawn((Mesh3d(looks.crate_mesh.clone()), MeshMaterial3d(mat), Transform::from_translation(v.map_or(c.pos, |v| v.pos)), vis))
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
            // The mesh is 1 m long on z and thin on x: turn x to face the wall's normal.
            let tf = Transform::from_translation(w.pos + Vec3::Y * w.h / 2.0)
                .with_rotation(Quat::from_rotation_arc(Vec3::X, w.normal))
                .with_scale(Vec3::new(1.0, if w.gate { 0.15 } else { w.h / WALL_H }, w.half_len * 2.0));
            w.ent = Some(commands.spawn((Mesh3d(m), MeshMaterial3d(mat), tf)).id());
        }
    }
    // #36: others' shots, a delay late like their shooters (yours are predicted).
    let d = if sim.you_play { sim.view_delay() } else { 0.0 };
    for sh in &mut sim.shots {
        let late = sh.owner != Some(YOU) && d > 0.0;
        let at = if late { sh.pos - sh.vel * d } else { sh.pos };
        let shown = !late || sh.age >= d;
        match sh.ent {
            Some(e) => {
                if let Ok((mut tf, mut vis)) = q.get_mut(e) {
                    tf.translation = at;
                    *vis = if shown { Visibility::Visible } else { Visibility::Hidden };
                }
            }
            None => {
                let mat = looks.shot_mat[sh.team].clone();
                let vis = if shown { Visibility::Visible } else { Visibility::Hidden };
                sh.ent = Some(
                    commands
                        .spawn((Mesh3d(looks.shot_mesh.clone()), MeshMaterial3d(mat), Transform::from_translation(at), vis))
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
    // #35: marked drops: a ring where it lands that closes as the drop comes, and a beam.
    for (p, kind, left) in &sim.marked {
        let (col, warn) = if *kind == Kind::Mega {
            (Color::srgb(1.0, 0.8, 0.1), sim.n(P::MegaWarn))
        } else {
            (Color::srgb(0.9, 0.9, 0.9), sim.n(P::SpawnWarn))
        };
        let k = (left / warn.max(0.1)).clamp(0.0, 1.0);
        let at = p.with_y(0.05);
        g.circle(ground_circle(at), HALF * 1.3, col);
        g.circle(ground_circle(at), HALF * 1.3 + 2.0 * k, col.with_alpha(0.6));
        g.line(at, at + Vec3::Y * 6.0, col.with_alpha(0.5));
    }
    // #15: base hp bars and knockout timers, lock-in progress, megacrate beacons.
    for t in 0..2 {
        if sim.loss == THEFT {
            break;
        }
        let p = base_pos(t).extend(BASE_H + 0.6).xzy();
        let (w, z) = (Vec3::X * 0.9, Vec3::Z * 0.0);
        g.line(p - w + z, p + w + z, Color::srgba(0.2, 0.2, 0.2, 0.9));
        if sim.pad_up(t) {
            let k = (sim.base_hp[t] / sim.n(P::BaseHp)).clamp(0.0, 1.0);
            let col = if sim.base_hit_t[t] < 0.3 { Color::WHITE } else { TEAM_COLOR[t] };
            for dy in [0.0, 0.05, 0.1] {
                g.line(p - w + Vec3::Y * dy, p - w + w * 2.0 * k + Vec3::Y * dy, col);
            }
        } else if sim.base_down[t].is_finite() {
            let k = sim.base_down[t] / sim.n(P::KnockoutTime);
            g.circle(ground_circle(base_pos(t).extend(0.05).xzy()), 1.2, Color::srgb(1.0, 0.5, 0.1));
            g.circle(ground_circle(base_pos(t).extend(0.05).xzy()), 1.2 * k.clamp(0.0, 1.0), Color::srgb(1.0, 0.8, 0.2));
        } else {
            let b = base_pos(t).extend(0.1).xzy();
            g.line(b + Vec3::new(-0.9, 0.0, -0.9), b + Vec3::new(0.9, 0.0, 0.9), Color::srgb(1.0, 0.2, 0.2));
            g.line(b + Vec3::new(-0.9, 0.0, 0.9), b + Vec3::new(0.9, 0.0, -0.9), Color::srgb(1.0, 0.2, 0.2));
        }
    }
    // #36: crates as you see them.
    for c in &sim.crates {
        let Some(v) = crate_view(&sim, c) else { continue };
        if !(v.loose || v.locked) {
            continue;
        }
        if c.kind == Kind::Mega {
            g.line(v.pos, v.pos + Vec3::Y * 8.0, Color::srgb(1.0, 0.85, 0.1));
        }
        let home = (0..2).find(|&t| pad_contains(pads()[t], v.pos));
        if let Some(t) = home.filter(|_| sim.lock_in) {
            let tf = Transform::from_translation(v.pos).with_scale(Vec3::splat(CRATE * 1.04));
            if v.locked {
                g.cube(tf, TEAM_COLOR[t]);
            } else if c.pad_t > 0.0 {
                let k = (c.pad_t / sim.n(P::LockInTime)).min(1.0);
                g.circle(ground_circle(v.pos + Vec3::Y * (HALF + 0.02)), HALF * k, TEAM_COLOR[t]);
            }
        }
    }
    let you_play = tune.c(P::Seat) == 0;
    let a = &sim.actors[YOU];
    let pos_of = |i: usize| shown_at(&sim, i, &tune);
    if you_play && a.alive() {
        let chest = a.pos + Vec3::Y * CHEST;
        g.line(chest, chest + a.facing * 8.0, Color::srgba(1.0, 1.0, 1.0, 0.5));
        let plan = a.carrying.filter(|_| sim.grab.is_none()).map(|_| throw_plan(&sim, controls.aim_dist));
        if let Some(p) = controls.aim_point
            && plan.as_ref().is_none_or(|p| p.target.is_none())
        {
            g.circle(ground_circle(p), 0.35, Color::srgb(1.0, 0.3, 0.3));
            g.circle(ground_circle(p.with_y(0.03)), 0.35, Color::srgba(1.0, 0.3, 0.3, 0.4));
            g.line(p.with_y(0.03), p, Color::srgba(1.0, 0.3, 0.3, 0.4));
        }
        if a.carrying.is_none() || sim.grab.is_some() {
            g.circle(ground_circle(a.pos + Vec3::Y * 0.03), sim.reach() + HALF, Color::srgba(1.0, 1.0, 1.0, 0.15));
            if sim.grab.is_none()
                && let Some((_, at)) = seen_target(&sim)
            {
                g.cube(Transform::from_translation(at).with_scale(Vec3::splat(CRATE * 1.08)), Color::WHITE);
            }
        }
        // #36: the reach while the server's answer is on its way (not when predicted).
        if let Some(p) = &sim.grab
            && !p.predicted
        {
            let span = (p.answer_at - p.at).max(1e-3);
            let k = ((sim.t - p.at) / span).clamp(0.0, 1.0);
            // Out to the crate over the first half, held there until the answer.
            let hand = chest.lerp(p.seen, (k * 2.0).min(1.0));
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
        let kick = kick_view(&sim, team);
        if kick > 0.0 {
            let k = (kick / KICK_TIME).min(1.0);
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
            if let Some(k) = stage_at_k(kick_view(&sim, team), stages.len()) {
                let ids = sim.stage_ids(team, stages[k].1);
                for v in sim.crates.iter().filter(|c| ids.contains(&c.id)).filter_map(|c| crate_view(&sim, c)) {
                    g.cube(Transform::from_translation(v.pos).with_scale(Vec3::splat(CRATE * 1.06)), Color::srgb(1.0, 0.8, 0.1));
                }
            }
        }
    }
    // Locked crates get a dark outline: asleep, untouchable until kicked open.
    for v in sim.crates.iter().filter_map(|c| crate_view(&sim, c)).filter(|v| v.locked) {
        g.cube(Transform::from_translation(v.pos).with_scale(Vec3::splat(CRATE * 1.02)), Color::srgb(0.1, 0.1, 0.1));
    }
    // #36 (key O): the server's truth, in orange, wherever it differs from what you see.
    if sim.truth {
        let orange = Color::srgb(1.0, 0.5, 0.0);
        for c in &sim.crates {
            let off = crate_view(&sim, c).is_none_or(|v| v.pos.distance(c.pos) > 0.05);
            if off {
                g.cube(Transform::from_translation(c.pos).with_scale(Vec3::splat(CRATE * 0.9)), orange);
            }
        }
        for (i, b) in sim.actors.iter().enumerate().filter(|(i, b)| *i != YOU && b.alive()) {
            if pos_of(i).distance(b.pos) > 0.05 {
                g.circle(ground_circle(b.pos + Vec3::Y * 0.05), ACTOR_R, orange);
                g.line(b.pos + Vec3::Y * 0.05, b.pos + Vec3::Y * 1.8, orange);
            }
        }
    }
    // Tanks: a hull around the driver, and its health.
    for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive()) {
        if let Some(t) = &b.tank {
            let p = pos_of(i);
            let rot = Transform::IDENTITY.looking_to(b.facing, Vec3::Y).rotation;
            let team = TEAM_COLOR[b.team];
            // Tank: a low hull and a barrel. Mech: a tall frame on two legs. Ambulance: a white
            // box with a red cross on the roof, and its aura on the floor.
            let (size, y) = match t.ride {
                Ride::Tank => (Vec3::new(1.8, 1.0, 2.4), 0.6),
                Ride::Mech => (Vec3::new(1.6, 1.2, 1.6), 1.9),
                Ride::Ambulance | Ride::AmbulanceShot => (Vec3::new(1.7, 1.4, 2.6), 0.8),
            };
            let hull = Transform::from_translation(p + Vec3::Y * y).with_rotation(rot);
            match t.ride {
                Ride::Tank => {
                    g.cube(hull.with_scale(size), team);
                    g.line(p + Vec3::Y * 1.2, p + Vec3::Y * 1.2 + b.facing * 1.8, team);
                }
                Ride::Mech => {
                    g.cube(hull.with_scale(size), team);
                    let side = Vec3::new(-b.facing.z, 0.0, b.facing.x) * 0.5;
                    let step = (sim.t * 6.0).sin() * 0.3 * (b.vel.with_y(0.0).length() > 0.5) as u8 as f32;
                    for (k, sgn) in [(1.0, 1.0), (-1.0, -1.0)] {
                        let hip = p + Vec3::Y * 1.3 + side * k;
                        g.line(hip, p + side * k + b.facing * step * sgn, team);
                    }
                }
                Ride::Ambulance | Ride::AmbulanceShot => {
                    g.cube(hull.with_scale(size), Color::WHITE);
                    let roof = p + Vec3::Y * 1.55;
                    let side = Vec3::new(-b.facing.z, 0.0, b.facing.x);
                    let red = Color::srgb(1.0, 0.15, 0.15);
                    g.line(roof - side * 0.5, roof + side * 0.5, red);
                    g.line(roof - b.facing * 0.5, roof + b.facing * 0.5, red);
                    g.circle(ground_circle(p.with_y(0.05)), 1.0, team);
                    if t.ride == Ride::Ambulance {
                        let r = sim.n(P::AmbRadius);
                        g.circle(ground_circle(p.with_y(0.05)), r, Color::srgba(0.3, 1.0, 0.4, 0.7));
                        g.circle(ground_circle(p.with_y(0.05)), r * (0.3 + 0.7 * (sim.t * 1.5).fract()), Color::srgba(0.3, 1.0, 0.4, 0.25));
                    }
                }
            }
            let top = p + Vec3::Y * (y + size.y / 2.0 + 0.8) - Vec3::X * 0.6;
            g.line(top, top + Vec3::X * 1.2 * (t.hp / t.max).max(0.0), Color::srgb(0.3, 0.8, 1.0));
            if t.gold {
                g.cube(hull.with_scale(size + Vec3::splat(0.1)), Color::srgb(1.0, 0.85, 0.2));
            }
        }
        if let Some(gun) = &b.gun {
            let col = if gun.gold { Color::srgb(1.0, 0.85, 0.2) } else { Color::srgb(0.9, 0.9, 0.9) };
            let p = pos_of(i) + Vec3::Y * CHEST;
            g.line(p, p + b.facing * 1.0, col);
        }
    }
    // Class markers over heads: soldier ^, engineer square, medic +.
    {
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
    // Walls: hp bar over the middle, gold edges on a super one, a dashed gate.
    let gold = Color::srgb(1.0, 0.85, 0.2);
    for w in &sim.walls {
        let tan = Vec3::new(-w.normal.z, 0.0, w.normal.x);
        let top = w.pos + Vec3::Y * (w.h + 0.3);
        if w.gate {
            for k in 0..4 {
                let a = w.pos + tan * (-w.half_len + k as f32 * w.half_len / 2.0) + Vec3::Y * 0.3;
                g.line(a, a + tan * w.half_len / 4.0, TEAM_COLOR[w.team]);
            }
            continue;
        }
        g.line(top - tan * 0.8, top - tan * 0.8 + tan * 1.6 * (w.hp / w.max_hp).max(0.0), Color::srgb(1.0, 0.3, 0.3));
        if w.gold {
            let a = w.pos + Vec3::Y * w.h;
            g.line(a - tan * w.half_len, a + tan * w.half_len, gold);
        }
        // A Set C gate: bars across it in its team's colour (its team walks through).
        if w.pass {
            let n = (w.half_len * 2.0) as i32;
            for k in 0..=n {
                let a = w.pos + tan * (-w.half_len + k as f32);
                g.line(a, a + Vec3::Y * w.h, TEAM_COLOR[w.team]);
            }
        }
    }
    // Grenades (red, the ring grows as the fuse runs down) and med packs (green).
    for l in &sim.lobs {
        match l.kind {
            TossKind::Grenade => {
                g.sphere(Isometry3d::from_translation(l.pos), 0.18, if l.gold { gold } else { Color::srgb(1.0, 0.2, 0.1) });
                let fuse = (l.fuse / sim.n(P::GrenadeFuse)).clamp(0.0, 1.0);
                g.circle(ground_circle(l.pos.with_y(0.04)), sim.n(P::GrenadeBlast) * (1.0 - fuse), Color::srgba(1.0, 0.2, 0.1, 0.6));
            }
            TossKind::MedPack => {
                g.sphere(Isometry3d::from_translation(l.pos), 0.25, Color::srgb(0.2, 1.0, 0.3));
            }
            TossKind::Launched => {
                let r = if l.small { 0.13 } else { 0.2 };
                g.sphere(Isometry3d::from_translation(l.pos), r, if l.gold { gold } else { TEAM_COLOR[l.team].mix(&Color::srgb(0.2, 0.2, 0.2), 0.4) });
            }
            TossKind::Shell => {
                // A falling shell, and where it lands: a ring that closes as it falls.
                g.sphere(Isometry3d::from_translation(l.pos), 0.3, Color::srgb(1.0, 0.45, 0.1));
                let r = sim.n(P::ShellBlast) * (0.3 + 0.7 * (l.pos.y / 16.0).clamp(0.0, 1.0));
                g.circle(ground_circle(l.pos.with_y(0.06)), r, Color::srgb(1.0, 0.2, 0.1));
            }
        }
    }
    // Barrages: the marked circle in the marker's colour, both teams see it; a ring closes in
    // over the delay, then it flashes while shells fall.
    for b in &sim.barrages {
        let c = b.at.with_y(0.07);
        let col = TEAM_COLOR[b.team].mix(&Color::srgb(1.0, 0.2, 0.1), 0.5);
        g.circle(ground_circle(c), b.r, col);
        g.circle(ground_circle(c), b.r + 0.15, col);
        if b.delay > 0.0 {
            let k = (b.delay / sim.n(P::BarrageDelay).max(0.1)).clamp(0.0, 1.0);
            g.circle(ground_circle(c), b.r * k, Color::srgba(1.0, 0.3, 0.1, 0.8));
        } else if (sim.t * 6.0).fract() < 0.5 {
            g.circle(ground_circle(c), b.r * 0.5, Color::srgba(1.0, 0.3, 0.1, 0.6));
        }
        if b.gold {
            g.circle(ground_circle(c), b.r + 0.3, gold);
        }
    }
    // Adrenaline: a red ring at the feet; self-patch: a green one.
    for (i, b) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive()) {
        if b.adren > 0.0 {
            g.circle(ground_circle(pos_of(i).with_y(0.09)), 0.85, Color::srgb(1.0, 0.25, 0.25));
        }
        if b.regen_t > 0.0 {
            g.circle(ground_circle(pos_of(i).with_y(0.09)), 0.75, Color::srgb(0.3, 1.0, 0.4));
        }
    }
    for (at, r, _, team) in &sim.zones {
        g.circle(ground_circle(at.with_y(0.05)), *r, TEAM_COLOR[*team].mix(&Color::srgb(0.7, 0.2, 1.0), 0.6));
        g.circle(ground_circle(at.with_y(0.05)), r * 0.6, Color::srgba(0.7, 0.2, 1.0, 0.5));
    }
    for (i, _) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive() && b.invuln > 0.0) {
        g.sphere(Isometry3d::from_translation(pos_of(i) + Vec3::Y * 1.0), 1.1, gold);
    }
    for (i, _) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive() && b.slow > 0.0) {
        g.circle(ground_circle(pos_of(i).with_y(0.06)), 0.7, Color::srgb(0.7, 0.2, 1.0));
    }
    // Your aim for a 1-crate throwable or a shield wall.
    if you_play && a.alive() && a.carrying.is_none() && a.tank.is_none() {
        let aim = controls.aim_point.unwrap_or(a.pos + a.facing * 6.0);
        if a.toss.is_some() {
            let flat = (aim - a.pos).with_y(0.0).clamp_length_max(THROW_DIST * jump_boost(a));
            g.circle(ground_circle((a.pos + flat).with_y(0.05)), 0.5, Color::srgba(1.0, 0.5, 0.2, 0.7));
        }
        if a.kit.is_some_and(|k| k.0 == KitKind::Barricade) {
            let flat = (aim - a.pos).with_y(0.0);
            let dir = flat.normalize_or(a.facing);
            let at = (a.pos + dir * flat.length().clamp(1.5, 8.0)).with_y(0.05);
            let tan = Vec3::new(-dir.z, 0.0, dir.x) * sim.n(P::BarricadeLen) / 2.0;
            g.line(at - tan, at + tan, Color::srgba(1.0, 0.8, 0.4, 0.7));
        }
        // A barrage's circle, or where a gate wall goes (snapped to a choke when close).
        if let Some(m) = a.mark {
            let far = aim.xz().distance(a.pos.xz()) > sim.n(P::MarkRange);
            let col = if far { Color::srgba(0.6, 0.6, 0.6, 0.5) } else { Color::srgb(1.0, 0.5, 0.2) };
            if m.make == Make::Gate {
                let near = chokes().into_iter().filter(|c| sim.map && c.0.distance(aim.xz()) < 5.0).min_by(|x, y| x.0.distance(aim.xz()).total_cmp(&y.0.distance(aim.xz())));
                let (p, n) = near.unwrap_or((aim.xz(), if a.facing.x.abs() >= a.facing.z.abs() { Vec2::X } else { Vec2::Y }));
                let tan = Vec3::new(-n.y, 0.0, n.x) * sim.n(P::GateLen) / 2.0;
                let at = p.extend(0.1).xzy();
                g.line(at - tan, at + tan, col);
                g.line(at - tan + Vec3::Y * WALL_H, at + tan + Vec3::Y * WALL_H, col);
            } else {
                g.circle(ground_circle(aim.with_y(0.07)), sim.n(P::BarrageRadius), col);
            }
        }
        if a.alt.is_some_and(|x| x.kind == AltKind::ShieldWall) {
            let d = controls.aim_dist.unwrap_or(8.0).clamp(1.5, 8.0);
            let at = (a.pos + a.facing * d).with_y(0.05);
            let tan = Vec3::new(-a.facing.z, 0.0, a.facing.x) * sim.n(P::WallLen) / 2.0;
            g.line(at - tan, at + tan, Color::srgba(0.6, 0.8, 1.0, 0.5));
        }
    }
    // Nail mines: a spiked ring in the layer's team colour (gold edge when super), dim until
    // armed. Both teams see them.
    for m in &sim.mines {
        let c = m.pos.with_y(0.04);
        let col = if m.arm > 0.0 { TEAM_COLOR[m.team].with_alpha(0.35) } else { TEAM_COLOR[m.team] };
        g.circle(ground_circle(c), MINE_R * 0.6, col);
        for k in 0..6 {
            let d = Vec3::from(Quat::from_rotation_y(k as f32 * 1.047) * Vec3::X);
            g.line(c + d * MINE_R * 0.6, c + d * MINE_R, col);
        }
        if m.gold {
            g.circle(ground_circle(c), MINE_R, Color::srgb(1.0, 0.85, 0.2));
        }
    }
    // Revealed by a super mine: a magenta ring round them.
    for (i, _) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive() && b.reveal > 0.0) {
        g.circle(ground_circle(pos_of(i).with_y(0.07)), 1.2, Color::srgb(1.0, 0.2, 0.9));
    }
    for (i, _) in sim.actors.iter().enumerate().filter(|(_, b)| b.alive() && b.root > 0.0) {
        let p = pos_of(i).with_y(0.08);
        g.line(p - Vec3::X * 0.6, p + Vec3::X * 0.6, Color::srgb(0.8, 0.8, 0.8));
        g.line(p - Vec3::Z * 0.6, p + Vec3::Z * 0.6, Color::srgb(0.8, 0.8, 0.8));
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
    let what = if sim.loss == ROUNDS {
        format!("ROUND {} (best of {:.0}, rounds {}-{})", sim.round, sim.n(P::BestOf), sim.wins[0], sim.wins[1])
    } else {
        "MATCH".to_string()
    };
    let counts = if sim.lock_in { "locked-in pad crates + spent" } else { "pad crates + spent" };
    s += &format!(
        "{what} {}:{:02}   BLUE {} - {} RED   ({counts})   kills {}-{}\n",
        (left / 60.0) as u32,
        (left % 60.0) as u32,
        sim.score(0),
        sim.score(1),
        sim.kills[0],
        sim.kills[1],
    );
    // #42: the map, its symmetry and floor per player, and the queue.
    let ar = arena();
    let queue = ["you pick", "rotate", "random", "random, no repeats"][tune.c(P::Queue)];
    let next = if tune.c(P::Queue) == 0 { MAP_SHORT[tune.c(P::Map)].to_string() } else { format!("{queue} from {}", POOL_NAMES[tune.c(P::Pool)]) };
    s += &format!(
        "MAP  {} | {} | {}v{}, {:.0} m2 per player | next: {next}\n",
        MAP_NAMES[sim.map_i],
        if !sim.map { "-" } else if sim.rotate { "ROTATIONAL" } else { "MIRROR" },
        sim.team_size,
        sim.team_size,
        ar.x * ar.y * 4.0 / (2.0 * sim.team_size as f32),
    );
    if !sim.played_maps.is_empty() {
        let last: Vec<&str> = sim.played_maps.iter().rev().take(8).rev().map(|&m| MAP_SHORT[m]).collect();
        s += &format!("     played: {}\n", last.join(" > "));
    }
    // #36: the latency being played, and what it did to your grabs.
    let rtt = sim.rtt();
    if rtt > 0.0 {
        s += &format!(
            "NET  ping {:.0} ms: others shown {:.0} ms late | grabs {} | throws {} | reach {:.1} m | rewind {}   (L: 0 ms, O: truth)\n",
            rtt * 1000.0,
            sim.view_delay() * 1000.0,
            if sim.grabs_predicted() { "PREDICTED" } else { "wait for the server" },
            if sim.acts_predicted() { "PREDICTED" } else { "wait for the server" },
            sim.reach(),
            if sim.tv[P::Rewind as usize] as usize == 1 { "ON" } else { "off" },
        );
    }
    if let Some((_, st)) = sim.net.last().filter(|_| rtt > 0.0) {
        s += &format!("     {}\n", st.line());
    }
    if let Some(g) = &sim.grab {
        s += &format!(
            "     {} sent, answer in {:.0} ms{}\n",
            if g.steal { "STEAL" } else { "GRAB" },
            ((g.answer_at - sim.t) * 1000.0).max(0.0),
            if g.predicted { " (shown in your hands already)" } else { "" },
        );
    }
    if sim.match_left <= 0.0 && sim.overtime && sim.over <= 0.0 {
        s += &format!("OVERTIME {:.0} s (of {:.0}): on while the trailing team carries or contests\n", sim.ot, sim.n(P::OvertimeCap));
    }
    if sim.mercy && sim.mercy_t > 0.0 {
        s += &format!("MERCY in {:.0} s: the gap is {} or more\n", sim.n(P::MercyHold) - sim.mercy_t, sim.n(P::MercyGap));
    }
    // #34: pacing.
    let tr = &sim.track;
    let mm = |t: f32| format!("{}:{:02}", t.max(0.0) as u32 / 60, t.max(0.0) as u32 % 60);
    let length = tune.f(P::MatchLength);
    let since = if tr.fights.is_empty() { "no fight yet".to_string() } else { format!("last fight {:.0} s ago", tr.time - tr.last_fight) };
    let leading = match tr.lead_now {
        Some(t) => format!("{} has led since {}", ["BLUE", "RED"][t], mm(tr.lead_since)),
        None => format!("tied since {}", mm(tr.lead_since)),
    };
    s += &format!("timer {} | played {} | {since} | {leading}\n", mm(length), mm(tr.time));
    let cutoff = length * tune.f(P::SeatCutoff) / 100.0;
    let seats: Vec<String> = sim
        .actors
        .iter()
        .enumerate()
        .filter(|(_, a)| a.open)
        .map(|(i, a)| {
            let who = if i == YOU { "YOU".to_string() } else { format!("{}{}", ["blue ", "red "][a.team], a.slot + 1) };
            match a.entered {
                Some(e) => format!("{who} in at {}", mm(e)),
                None if tr.time >= a.seat_at => format!("{who} WAITING at the pad since {}", mm(a.seat_at)),
                None => format!("{who} queued, seat fills at {}", mm(a.seat_at)),
            }
        })
        .collect();
    if !seats.is_empty() {
        s += &format!("open seats (taken until {}): {}\n", mm(cutoff), seats.join(" | "));
    }
    if sim.loss != THEFT {
        let base = |t: usize| {
            if sim.base_down[t].is_infinite() {
                "DESTROYED".to_string()
            } else if sim.base_down[t] > 0.0 {
                format!("DOWN, rebuilds in {:.0} s", sim.base_down[t])
            } else {
                format!("{:.0}/{:.0} hp", sim.base_hp[t].max(0.0), sim.n(P::BaseHp))
            }
        };
        s += &format!("bases: blue {}   red {}   ({})\n", base(0), base(1), LOSS_OPTS[sim.loss]);
    } else {
        s += "bases: none (theft only)\n";
    }
    s += &format!(
        "players: BLUE {}/{n}  RED {}/{n}   crates stolen off pads: blue {} red {}\n",
        sim.players(0),
        sim.players(1),
        sim.stolen[0],
        sim.stolen[1],
        n = sim.team_size,
    );
    let cap = |t: usize| sim.capstone(t).map_or("ready".to_string(), |c| format!("LOCKED ({c} still out)"));
    s += &format!("all 8: blue {}   red {}\n", cap(0), cap(1));
    s += &format!("your pad: {}\n", pad_readout(&sim, 0));
    s += &format!("red pad:  {}\n", pad_readout(&sim, 1).replace("   HOLD F: LONGER = MORE CRATES", ""));
    if tune.c(P::Seat) == 1 {
        s += "your seat is a BOT: watching\n";
    } else {
        s += &format!(
            "you ({}): {}{}  hp {:.0}{}{}{}{}\n",
            a.class.name(),
            if a.carrying.is_some() && sim.grab.is_none() { "CARRYING" } else { "empty-handed" },
            if a.grounded { "" } else { ", airborne" },
            a.hp.max(0.0),
            if a.shield > 0.0 { format!("  shield {:.0}", a.shield) } else { String::new() },
            if a.invuln > 0.0 { format!("  INVULNERABLE {:.1}s", a.invuln) } else { String::new() },
            if a.slow > 0.0 { "  SLOWED" } else { "" },
            if a.alive() {
                String::new()
            } else if a.dead.is_infinite() {
                "  OUT: in when blue opens a player recipe".to_string()
            } else {
                format!("  DOWN {:.1}s", a.dead)
            },
        );
        if let Some(t) = &a.tank {
            let what = match t.ride {
                Ride::Tank => "cannon only, can't carry".to_string(),
                Ride::Mech => format!(
                    "carrying {}/{} (E grab, E again or beside your pad to set one down, RMB throw one), LMB stomp, no weapon",
                    a.load.len(),
                    sim.mech_cap(YOU)
                ),
                Ride::Ambulance => format!("allies within {:.0} m heal {:.0} hp/s, LMB weak blaster", sim.n(P::AmbRadius), sim.n(P::AmbHeal)),
                Ride::AmbulanceShot => format!("LMB heal shot (+{:.0} hp to the first hurt ally it touches)", sim.n(P::AmbShotHeal)),
            };
            s += &format!("IN A {}{}: hp {:.0}, {:.0} s left, {what}\n", if t.gold { "SUPER " } else { "" }, t.ride.name().to_uppercase(), t.hp, t.life);
        }
        if a.root > 0.0 {
            s += &format!("ROOTED by a nail mine ({:.1} s)\n", a.root);
        }
        s += &match &a.gun {
            Some(gun) if gun.kind == GunKind::Mines => format!(
                "left click: {} ({} mines left, {} out of {})\n",
                Make::Gun(gun.kind).label(gun.gold),
                gun.ammo,
                sim.mines.iter().filter(|m| m.owner == YOU).count(),
                sim.n(P::MinesOut)
            ),
            Some(gun) => format!("left click: {} ({} shots left)\n", Make::Gun(gun.kind).label(gun.gold), gun.ammo),
            None => "left click: standard gun\n".to_string(),
        };
        if let Some(alt) = &a.alt {
            s += &format!("right click (empty hands): {} ({} left)\n", Make::Alt(alt.kind).label(alt.gold), alt.charges);
        }
        if let Some((kind, gold)) = a.toss {
            s += &format!("Q: throw your {}\n", Make::Toss(kind).label(gold));
        }
        if let Some((kind, gold)) = a.kit {
            s += &format!("Q: use your {}\n", Make::Kit(kind).label(gold));
        }
        if a.adren > 0.0 {
            s += &format!("ADRENALINE {:.1} s ({:.2}x fire rate; picking up a crate ends it{})\n", a.adren, sim.n(P::AdrenRate), if a.adren_gold { "; a kill reloads" } else { "" });
        }
        if let Some(m) = a.mark {
            let what = if m.make == Make::Gate {
                format!("place a gate wall ({} left; near a choke it snaps across it)", m.left)
            } else {
                format!("mark the barrage circle ({:.0} m across)", sim.n(P::BarrageRadius) * 2.0)
            };
            s += &format!("LMB: {what}, out to {:.0} m. No shooting until then ({:.0} s left)\n", sim.n(P::MarkRange), m.life);
        }
        if sim.barrages.iter().any(|b| b.team == 1 && b.at.xz().distance(a.pos.xz()) < b.r) {
            s += "INCOMING BARRAGE: GET OUT OF THE CIRCLE\n";
        }
        if a.carrying.is_some() {
            let boost = throw_plan(&sim, controls.aim_dist).boost;
            s += &format!("throw reach: {:.1} m  (jump x{boost:.2})\n", THROW_DIST * boost);
        }
    }
    if sim.map {
        let camp = |m: &Monster| {
            let mid = layout().middle;
            let name = match (m.drop, m.home.z > 0.0, m.home.x < 0.0) {
                (Drop::Super, true, _) => format!("{mid} N (super crate)"),
                (Drop::Super, false, _) => format!("{mid} S (super crate)"),
                (d, _, west) => format!("{} {}", if west { "blue-side" } else { "red-side" }, if d == Drop::Heal { "heal" } else { "shield" }),
            };
            // #35: a random river objective's time back is hidden.
            match (m.alive(), m.drop == Drop::Super && sim.luck[1] == 1) {
                (true, _) => format!("{name} up"),
                (false, true) => format!("{name} back in ?"),
                (false, false) => format!("{name} {:.0}s", m.respawn),
            }
        };
        s += &format!("camps: {}\n", sim.monsters.iter().map(camp).collect::<Vec<_>>().join(" | "));
    }
    // #35: the rule switch, the next megacrate, and what friendly fire has done.
    let tr = &sim.track;
    if sim.mega > 0 {
        let mega = if let Some(m) = sim.marked.iter().find(|m| m.1 == Kind::Mega) {
            let side = if m.0.x < -3.0 { "blue's side" } else if m.0.x > 3.0 { "red's side" } else { "the middle" };
            format!("MARKED on {side}, lands in {:.0} s", m.2)
        } else if sim.crates.iter().any(|c| c.kind == Kind::Mega) {
            "one is out".to_string()
        } else if sim.luck[2] == 0 {
            format!("next in {:.0} s", sim.mega_t)
        } else {
            "next: ?".to_string()
        };
        s += &format!("megacrate: {mega}\n");
    }
    let ff = |t: usize| {
        format!(
            "{:<4} allies -{:.0} hp ({} out), self -{:.0} ({} out), crate hits {}, carriers dropped {}, own-pad knocks {}",
            ["blue", "red"][t],
            tr.ff_dmg[t],
            tr.ff_ko[t],
            tr.self_dmg[t],
            tr.self_ko[t],
            tr.crate_ff[t],
            tr.ff_drops[t],
            tr.own_knocks[t]
        )
    };
    s += &format!("friendly fire: {}\n               {}\n", ff(0), ff(1));
    let acc = if sim.fired > 0 { 100.0 * sim.hits as f32 / sim.fired as f32 } else { 0.0 };
    s += &format!("your shots {} hits {} ({acc:.0}%)\n", sim.fired, sim.hits);
    for (m, _) in &sim.msgs {
        s += &format!("> {m}\n");
    }
    s += "\nWASD move  mouse aim  Space jump  E grab/place  RMB throw (empty hands: right-click item)  LMB shoot  Q throw 1-crate item  F kick\n";
    s += "1-5 map switch  C your class  ENTER pick this map  T crate  Y super crate  U megacrate  K red base to 1 hp (Shift: yours)  G red bunker  V your all 8  R reset  P print  H hide boxes";
    hud.0 = s;
}

/// The first player on `team` (its starting player).
fn team_lead(sim: &Sim, team: usize) -> usize {
    (0..sim.actors.len()).find(|&j| sim.actors[j].team == team && sim.actors[j].slot == 0).unwrap_or(YOU)
}

/// What a pad says about itself: the recipes inside it, smallest first, with what each
/// makes for the kicker (you, or red's first player) and any super twist.
fn pad_readout(sim: &Sim, team: usize) -> String {
    if sim.base_down[team].is_infinite() {
        return "DESTROYED: makes nothing for the rest of the match".into();
    }
    if !sim.pad_up(team) {
        return format!("DOWN: makes nothing for {:.0} s", sim.base_down[team]);
    }
    let st = sim.pad_state(team);
    let n = st.ids.len();
    let locked = (st.h == [2; 4]).then(|| sim.capstone(team)).flatten().map_or(String::new(), |c| format!("   (all 8 LOCKED: {c})"));
    match st.shape {
        Shape::Empty => return "empty".into(),
        Shape::Unaligned => return "a crate is out of line: kicking breaks them for nothing".into(),
        _ => {}
    }
    let stages = sim.stages(team);
    if stages.is_empty() {
        return format!("{n} crate{}: no recipe inside yet, add crates{locked}", if n == 1 { "" } else { "s" });
    }
    let at = stage_at_k(kick_view(sim, team), stages.len());
    let kicker = if team == 0 { YOU } else { team_lead(sim, 1) };
    let each: Vec<String> = stages
        .iter()
        .enumerate()
        .map(|(k, &(slot, take))| {
            let ids = sim.stage_ids(team, take);
            let supers = sim.crates.iter().filter(|c| ids.contains(&c.id) && c.kind == Kind::Super).count();
            let (make, gold) = sim.make_for(slot, supers, sim.actors[kicker].class);
            let what = format!("{} -> {}", slot.label(sim.count_only), make.label(gold));
            if at == Some(k) { format!("[{what}]") } else { what }
        })
        .collect();
    let tip = if at.is_some() { "   RELEASE F TO OPEN" } else { "   HOLD F: LONGER = MORE CRATES" };
    format!("{}{locked}{tip}", each.join(" > "))
}

fn guide_text(
    tune: Res<Tune>,
    sim: Res<Sim>,
    mut guide: Single<(&mut Text, &mut Visibility), (With<GuideText>, Without<Hud>, Without<Panel>)>,
) {
    let (text, vis) = &mut *guide;
    **vis = if tune.panel { Visibility::Visible } else { Visibility::Hidden };
    let mut s = "MAP SWITCH (press the number. R restarts)\n".to_string();
    for (k, p) in tune.vals[..RULES].iter().enumerate() {
        let opts = match &p.value {
            Value::Choice { i, opts } => opts
                .iter()
                .enumerate()
                .map(|(j, o)| {
                    // Map names are long: show them short, and the picked one in full.
                    let o = if k == 0 && j != *i { MAP_SHORT[j] } else { o };
                    if j == *i { format!("[{o}]") } else { o.to_string() }
                })
                .collect::<Vec<_>>()
                .join("  "),
            Value::Num { .. } => fmt_value(&p.value),
        };
        s += &format!(" {} {:<18} {opts}\n", k + 1, &p.name[2..]);
    }
    let _ = sim;
    s += "\nTry the full map against the small one, crossroads and canal: where do fights happen, how long are hauls, do bots cope?";
    s += "\nWATCH: does a map feel different enough to be worth shipping? Does rotational symmetry feel fair? Does the queue's pick feel right?";
    s += "\nRules: #36's verdict at 0 ms, ADR 0013-0016. Kit: the Final set from #22 (C changes your class). ENTER = pick this map";
    if !tune.verdicts.is_empty() {
        s += &format!(" ({} picked so far, P prints them)", tune.verdicts.len());
    }
    text.0 = s;
}

fn panel(tune: Res<Tune>, mut panel: Single<(&mut Text, &mut Visibility), (With<Panel>, Without<Hud>)>) {
    let (text, vis) = &mut *panel;
    **vis = if tune.panel { Visibility::Visible } else { Visibility::Hidden };
    let mut s = "TUNING (core feel #9, pad joins #10, grammar #17, kit #22 fixed)\n\n".to_string();
    // A window of rows round the cursor: the list is long.
    const ROWS: usize = 26;
    let n = tune.vals.len();
    let from = tune.selected.saturating_sub(ROWS / 2).min(n.saturating_sub(ROWS));
    if from > 0 {
        s += "  ...\n";
    }
    for (i, p) in tune.vals.iter().enumerate().skip(from).take(ROWS) {
        let cursor = if i == tune.selected { ">" } else { " " };
        s += &format!("{cursor} {:<22} {}\n", p.name, fmt_value(&p.value));
    }
    if from + ROWS < n {
        s += "  ...\n";
    }
    s += "\nUp/Down pick  Left/Right change (Shift x5)";
    text.0 = s;
}

#[derive(Resource, Default)]
struct Smoke {
    exit_at: Option<Duration>,
    shot: bool,
}

fn smoke_exit(time: Res<Time>, sim: Res<Sim>, mut smoke: ResMut<Smoke>, mut exit: MessageWriter<AppExit>, mut commands: Commands) {
    if std::env::var("CRATE_MATCHES").ok().and_then(|v| v.parse::<usize>().ok()).is_some_and(|n| sim.matches_done >= n) {
        exit.write(AppExit::Success);
        return;
    }
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
    let mut vals = defaults();
    // The kit picked in #22.
    let set = vals[P::Set as usize].value.clone();
    if let Value::Choice { i, .. } = set {
        apply_set(&mut vals, i);
    }
    let tune = Tune {
        vals,
        selected: 0,
        panel: true,
        verdicts: Vec::new(),
        last_ping: 2,
    };
    let sim = Sim::new(&tune);
    // Bulk bot runs: no window, no GPU. Each window holds ~65 MiB of the GPU's 256 MiB BAR1
    // aperture, and 4+ at once starve the compositor and freeze the desktop. The sim steps
    // a fixed 1/30 s frame (the `step` cap) as fast as the CPU goes.
    if std::env::var("CRATE_HEADLESS").as_deref() == Ok("1") {
        App::new()
            .add_plugins(MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(Duration::ZERO)))
            .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 30.0)))
            .insert_resource(tune)
            .insert_resource(sim)
            .init_resource::<Controls>()
            .init_resource::<Smoke>()
            .add_systems(Startup, init_from_env)
            .add_systems(Update, (step, smoke_exit).chain())
            .run();
        return;
    }
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "big_back launch maps (PROTOTYPE #42)".into(),
                // Smoke runs often land on a hidden workspace, where vsync blocks forever.
                present_mode: if std::env::var("CRATE_EXIT_AFTER").is_ok() || std::env::var("CRATE_MATCHES").is_ok() {
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

