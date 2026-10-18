//! Loads every generated asset collection in a loading state, then lays every
//! model out in a grid (one block per category) so the art can be inspected.
//!
//!     cargo run --example asset_viewer
//!
//! Controls: hold right mouse (or press M) to look, WASD to move, Q/E down/up,
//! Shift to run, scroll to change speed. The overlay names the model nearest
//! to the centre of the view. Animated models play their idle clip (or clip 0).
//!
//! `ASSET_VIEWER_EXIT_AFTER=<seconds>` quits automatically once every model has
//! been spawned and fitted, for smoke tests; with `ASSET_VIEWER_SCREENSHOT=<file.png>`
//! it also saves a screenshot just before quitting.

use std::time::Duration;

use bevy::camera::primitives::Aabb;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::world_serialization::WorldInstanceReady;
use bevy_asset_loader::prelude::*;
use big_back::assets::{
    all_clips, all_scenes, init_io_task_pool_for_bulk_gltf, load_all_collections, MODEL_COUNT,
};

/// Grid cell size in metres; every model is scaled to fit inside `FIT`.
const CELL: f32 = 4.0;
const FIT: f32 = 3.0;
/// Gap between category blocks.
const BLOCK_GAP: f32 = 12.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum ViewerState {
    #[default]
    Loading,
    Ready,
}

#[derive(Component)]
struct Exhibit {
    category: &'static str,
    name: &'static str,
    cell: Vec3,
    /// Size before fitting, in the model's own units.
    native_size: Option<Vec3>,
    clip: Option<(Handle<AnimationGraph>, AnimationNodeIndex, &'static str)>,
}

/// Counts frames after the instance is ready, so bounds and transforms settle.
#[derive(Component)]
struct FitPending(u32);

#[derive(Component)]
struct Overlay;

#[derive(Resource, Default)]
struct Stats {
    spawned: usize,
    fitted: usize,
    exit_at: Option<Duration>,
    screenshot_taken: bool,
}

fn main() {
    // Loading ~1400 glTF files at once needs a bigger IO thread stack (see the docs).
    init_io_task_pool_for_bulk_gltf();
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "big_back asset viewer".into(),
                    ..default()
                }),
                ..default()
            }),
            FreeCameraPlugin,
        ))
        .init_state::<ViewerState>()
        .add_loading_state(load_all_collections(
            LoadingState::new(ViewerState::Loading).continue_to_state(ViewerState::Ready),
        ))
        .init_resource::<Stats>()
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 400.0,
            ..default()
        })
        .add_observer(on_instance_ready)
        .add_systems(Startup, setup)
        .add_systems(Update, loading_overlay.run_if(in_state(ViewerState::Loading)))
        .add_systems(OnEnter(ViewerState::Ready), spawn_grid)
        .add_systems(
            Update,
            (fit_exhibits, describe_nearest, exit_when_done).run_if(in_state(ViewerState::Ready)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 12.0, -10.0).looking_at(Vec3::new(10.0, 0.0, 10.0), Vec3::Y),
        FreeCamera {
            walk_speed: 10.0,
            run_speed: 40.0,
            ..default()
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(30.0, 60.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Overlay,
        Text::new(format!("Loading {MODEL_COUNT} models...")),
        TextFont::from_font_size(16.0),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            ..default()
        },
    ));
}

fn loading_overlay(time: Res<Time>, mut text: Single<&mut Text, With<Overlay>>) {
    text.0 = format!("Loading {MODEL_COUNT} models... {:.0}s", time.elapsed_secs());
}

/// Lays out one square block per category along +Z, models in rows along +X.
fn spawn_grid(world: &mut World) {
    let scenes = all_scenes(world);
    let clips = all_clips(world);
    let mut graphs_to_add = Vec::new();

    let mut exhibits = Vec::with_capacity(scenes.len());
    let mut block_z = 0.0;
    let mut category = "";
    let mut index = 0usize;
    let mut cols = 1usize;
    let mut blocks: Vec<(&'static str, Vec3, Vec2)> = Vec::new();
    for (cat, name, scene) in &scenes {
        if *cat != category {
            if !category.is_empty() {
                let rows = index.div_ceil(cols);
                block_z += rows as f32 * CELL + BLOCK_GAP;
            }
            category = cat;
            index = 0;
            let count = scenes.iter().filter(|(c, _, _)| c == cat).count();
            cols = (count as f32).sqrt().ceil().max(1.0) as usize;
            let rows = count.div_ceil(cols);
            blocks.push((cat, Vec3::new(0.0, 0.0, block_z), Vec2::new(cols as f32, rows as f32) * CELL));
        }
        let cell = Vec3::new(
            (index % cols) as f32 * CELL + CELL * 0.5,
            0.0,
            block_z + (index / cols) as f32 * CELL + CELL * 0.5,
        );
        index += 1;

        let clip = clips.iter().find(|(n, _, _)| n == name).map(|(_, names, handles)| {
            let pick = names
                .iter()
                .position(|n| n.to_lowercase().contains("idle"))
                .unwrap_or(0);
            let (graph, nodes) = AnimationGraph::from_clips(handles.iter().cloned());
            graphs_to_add.push(graph);
            (graphs_to_add.len() - 1, nodes[pick], names[pick])
        });
        exhibits.push((*cat, *name, scene.clone(), cell, clip));
    }

    let graph_handles: Vec<Handle<AnimationGraph>> = {
        let mut graphs = world.resource_mut::<Assets<AnimationGraph>>();
        graphs_to_add.into_iter().map(|g| graphs.add(g)).collect()
    };

    for (category, name, scene, cell, clip) in exhibits {
        world.spawn((
            WorldAssetRoot(scene),
            Transform::from_translation(cell),
            Exhibit {
                category,
                name,
                cell,
                native_size: None,
                clip: clip.map(|(i, node, n)| (graph_handles[i].clone(), node, n)),
            },
        ));
    }

    // A floor tile under each category block.
    let mut meshes_to_spawn = Vec::new();
    for (i, (_, origin, size)) in blocks.iter().enumerate() {
        meshes_to_spawn.push((i, *origin, *size));
    }
    let mesh = world.resource_mut::<Assets<Mesh>>().add(Plane3d::default().mesh().size(1.0, 1.0));
    for (i, origin, size) in meshes_to_spawn {
        let shade = if i % 2 == 0 { 0.18 } else { 0.24 };
        let material = world.resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial {
            base_color: Color::srgb(shade, shade, shade + 0.03),
            perceptual_roughness: 1.0,
            ..default()
        });
        world.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(origin + Vec3::new(size.x * 0.5, -0.01, size.y * 0.5))
                .with_scale(Vec3::new(size.x + 2.0, 1.0, size.y + 2.0)),
        ));
    }

    world.resource_mut::<Stats>().spawned = scenes.len();
    info!("spawned {} models in {} category blocks", scenes.len(), blocks.len());
}

/// When a model's scene instance is ready, start its animation and queue it for fitting.
fn on_instance_ready(
    ready: On<WorldInstanceReady>,
    exhibits: Query<&Exhibit>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    mut commands: Commands,
) {
    let root = ready.entity;
    let Ok(exhibit) = exhibits.get(root) else {
        return;
    };
    if let Some((graph, node, _)) = &exhibit.clip {
        for e in children.iter_descendants(root) {
            if let Ok(mut player) = players.get_mut(e) {
                player.play(*node).repeat();
                commands.entity(e).insert(AnimationGraphHandle(graph.clone()));
            }
        }
    }
    commands.entity(root).insert(FitPending(0));
}

/// Scales each model so its bounds fit in a `FIT` cube, centred on its cell, resting on y = 0.
fn fit_exhibits(
    mut roots: Query<(Entity, &mut Transform, &GlobalTransform, &mut Exhibit, &mut FitPending)>,
    children: Query<&Children>,
    bounds: Query<(&Aabb, &GlobalTransform)>,
    mut stats: ResMut<Stats>,
    mut commands: Commands,
) {
    for (root, mut transform, root_global, mut exhibit, mut pending) in &mut roots {
        pending.0 += 1;
        if pending.0 < 3 {
            continue;
        }
        let to_local = root_global.affine().inverse();
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        for e in children.iter_descendants(root) {
            let Ok((aabb, global)) = bounds.get(e) else {
                continue;
            };
            let (c, h) = (Vec3::from(aabb.center), Vec3::from(aabb.half_extents));
            for corner in 0..8 {
                let sign = Vec3::new(
                    if corner & 1 == 0 { -1.0 } else { 1.0 },
                    if corner & 2 == 0 { -1.0 } else { 1.0 },
                    if corner & 4 == 0 { -1.0 } else { 1.0 },
                );
                let p = to_local.transform_point3(global.transform_point(c + h * sign));
                min = min.min(p);
                max = max.max(p);
            }
        }
        if min.x > max.x {
            // No bounds yet (meshes still being prepared); give up after a while.
            if pending.0 < 120 {
                continue;
            }
            warn!("{}/{}: no mesh bounds found", exhibit.category, exhibit.name);
            commands.entity(root).remove::<FitPending>();
            stats.fitted += 1;
            continue;
        }
        let size = max - min;
        let scale = FIT / size.max_element().max(1e-4);
        let anchor = Vec3::new((min.x + max.x) * 0.5, min.y, (min.z + max.z) * 0.5);
        transform.scale = Vec3::splat(scale);
        transform.translation = exhibit.cell - anchor * scale;
        exhibit.native_size = Some(size);
        commands.entity(root).remove::<FitPending>();
        stats.fitted += 1;
    }
}

fn describe_nearest(
    camera: Single<&GlobalTransform, With<Camera3d>>,
    exhibits: Query<&Exhibit>,
    stats: Res<Stats>,
    mut text: Single<&mut Text, With<Overlay>>,
) {
    let focus = camera.translation() + camera.forward() * 8.0;
    let nearest = exhibits
        .iter()
        .min_by(|a, b| a.cell.distance_squared(focus).total_cmp(&b.cell.distance_squared(focus)));
    let mut s = format!(
        "{} models ({} fitted)  |  RMB/M look, WASD move, Q/E down/up, Shift run, scroll speed\n",
        stats.spawned, stats.fitted
    );
    if let Some(e) = nearest {
        s += &format!("[{}] {}", e.category, e.name);
        if let Some(size) = e.native_size {
            s += &format!("  native size {:.2} x {:.2} x {:.2}", size.x, size.y, size.z);
        }
        if let Some((_, _, clip)) = &e.clip {
            s += &format!("  playing \"{clip}\"");
        }
    }
    text.0 = s;
}

fn exit_when_done(
    time: Res<Time>,
    mut stats: ResMut<Stats>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    let Some(secs) = std::env::var("ASSET_VIEWER_EXIT_AFTER").ok().and_then(|s| s.parse::<f32>().ok()) else {
        return;
    };
    if stats.spawned == 0 || stats.fitted < stats.spawned {
        return;
    }
    let deadline = *stats
        .exit_at
        .get_or_insert(time.elapsed() + Duration::from_secs_f32(secs));
    if let Ok(path) = std::env::var("ASSET_VIEWER_SCREENSHOT")
        && !stats.screenshot_taken
        && time.elapsed() + Duration::from_secs(1) >= deadline
    {
        stats.screenshot_taken = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    if time.elapsed() >= deadline {
        info!("asset viewer: {} of {} models fitted, exiting", stats.fitted, stats.spawned);
        exit.write(AppExit::Success);
    }
}
