//! Typed handles to every model under `assets/models/`.
//!
//! The collections live in [`manifest`], which is generated from the files on
//! disk by `tools/gen_asset_manifest.py` (see `docs/assets.md`). Load them in a
//! loading state with [`load_all_collections`] or one at a time with
//! `.load_collection::<CharacterAssets>()`, then read them as resources.
//!
//! ```no_run
//! use bevy::prelude::*;
//! use big_back::assets::{clips, MechAnimations, MechAssets};
//!
//! // Spawn a model (once the loading state has inserted `MechAssets`).
//! fn spawn_mech(mut commands: Commands, mechs: Res<MechAssets>) {
//!     commands.spawn((
//!         WorldAssetRoot(mechs.animated_mech_textured_george.clone()),
//!         Transform::from_xyz(0.0, 0.0, 5.0),
//!     ));
//! }
//!
//! // Build an animation graph from a model's clips and pick one by name.
//! fn george_graph(
//!     anims: Res<MechAnimations>,
//!     mut graphs: ResMut<Assets<AnimationGraph>>,
//! ) -> (Handle<AnimationGraph>, AnimationNodeIndex) {
//!     let (graph, nodes) =
//!         AnimationGraph::from_clips(anims.animated_mech_textured_george.iter().cloned());
//!     (graphs.add(graph), nodes[clips::animated_mech_textured_george::IDLE])
//! }
//! ```

pub mod manifest;

pub use manifest::*;

/// Stack size for IO task-pool threads; see [`init_io_task_pool_for_bulk_gltf`].
pub const IO_THREAD_STACK_SIZE: usize = 64 * 1024 * 1024;

/// Creates Bevy's IO task pool with a large thread stack. Call it before adding
/// `DefaultPlugins` (`TaskPoolPlugin` keeps a pool that already exists).
///
/// Why: in Bevy 0.19, the glTF loader decodes the textures of any file with
/// more than one texture inside an `IoTaskPool` scope, and a blocked scope runs
/// other queued tasks on the same thread. When hundreds of glTF files are
/// requested at once, as loading every collection does, those loads nest on
/// one stack and overflow the default 2 MiB thread stack. The nesting is bounded
/// by the number of glTF files in flight (about 20 KiB each in dev builds), so
/// 64 MiB covers several thousand files. The memory is reserved, not committed.
pub fn init_io_task_pool_for_bulk_gltf() {
    use bevy::tasks::{available_parallelism, IoTaskPool, TaskPoolBuilder};
    // Same thread count as Bevy's default policy: 25% of cores, 1..=4.
    let threads = (available_parallelism() / 4).clamp(1, 4);
    IoTaskPool::get_or_init(|| {
        TaskPoolBuilder::new()
            .num_threads(threads)
            .stack_size(IO_THREAD_STACK_SIZE)
            .thread_name("IO Task Pool".to_string())
            .build()
    });
}
