# Assets

Third-party 3D art lives in `assets/models/`. Bevy loads it from there natively, as glTF
binaries (`.glb`), through typed `bevy_asset_loader` collections. Credits and licenses are
in [`assets/CREDITS.md`](../assets/CREDITS.md).

## Versions

| Crate | Version | Why |
| --- | --- | --- |
| `bevy` | **0.19.1** | Pinned for the planned stack (lightyear 0.30, avian3d 0.7, bevy_hanabi 0.19, bevy_firework 0.10), which supports Bevy 0.19 so far. 0.20.0 is out but not used yet. |
| `bevy_asset_loader` | **0.27.0** | The release line for Bevy 0.19 (0.28 requires Bevy 0.20). |
| Blender (`bpy` module) | 5.0.1 | Only for converting FBX to GLB, run from a throwaway Python venv. Not a project dependency. |

Bevy 0.19 renamed a few things that the code here relies on. A glTF scene is a
`WorldAsset`, spawned with `WorldAssetRoot(handle)`, and a spawned instance fires
`WorldInstanceReady`. The asset labels are still `GltfAssetLabel::Scene(n)` (`#SceneN`) and
`GltfAssetLabel::Animation(n)` (`#AnimationN`).

## Layout conventions

```
assets/
  CREDITS.md
  models/<source>/<pack>/
    License.txt                    # the pack's own license file, unchanged
    textures/                      # textures shared by the pack's models
      variations/                  # alternate palettes nobody references (team colours etc.)
    <category>/[<sub>/]<name>.glb
```

- `<source>` is the author or vendor: `quaternius`, `kenney`, and later `synty`.
- `<pack>` is the pack name in snake_case, short but recognisable: `scifi_essentials`,
  `city_kit_roads`.
- `<category>` is one of: `characters`, `mechs`, `vehicles`, `weapons`, `props`,
  `environment`, `modular` (snap-together kit pieces such as walls, floors, roads and
  platforms), `ships`. One extra level is allowed for groups within a category, for
  example `weapons/parts`, `weapons/attachments`, `characters/hair_rigged`,
  `vehicles/wheels`, `mechs/flat`.
- File names are lowercase snake_case versions of the original names, for example
  `Spaceship_RaeTheRedPanda` becomes `spaceship_rae_the_red_panda.glb`.
- Every model is a self-contained `.glb`. Meshes, animations and embedded images are inside
  it. External textures sit in the pack's `textures/` folder and are referenced by relative
  path, such as `../textures/colormap.png`, which Bevy resolves. Several models share one
  texture, and Bevy loads it once.
- Only formats Bevy uses are kept. FBX, OBJ, `.blend`, engine project files and preview
  renders are dropped.

## Generated collections

`src/assets/manifest.rs` is generated from the files on disk. Never edit it by hand. For
each category it contains:

- `<Category>Assets`, for example `CharacterAssets` or `ModularAssets`: one
  `Handle<WorldAsset>` per model, loaded with the `#Scene0` label. Fields are named
  `<pack>[_<sub>]_<name>`, for example `scifi_essentials_prop_crate`.
- `<Category>Animations`: for each animated model, a `Vec<Handle<AnimationClip>>` holding
  every `#AnimationN` clip in file order.
- `clips::<field>::<CLIP>`: index constants named from the clip names in the glTF, for
  example `clips::ultimate_animated_character_wizard::IDLE`. They index both the clip
  `Vec` and the node list returned by `AnimationGraph::from_clips`.
- `load_all_collections(state)`, which adds every collection to a `LoadingState`. Also
  `all_scenes(world)` and `all_clips(world)` for tools such as the viewer.

**Loading everything at once:** call `big_back::assets::init_io_task_pool_for_bulk_gltf()`
before adding `DefaultPlugins`. In Bevy 0.19 the glTF loader decodes textures inside an IO
task-pool scope, and blocked scopes run other loads on the same thread. If about 1,400 files
are requested together, the loads nest and overflow the default 2 MiB IO thread stack. The
helper gives those threads a 64 MiB stack. That memory is reserved, not committed. A game
that loads one level's collections at a time will probably never need it.

### Regenerating the manifest

Run this after adding, removing or renaming any file under `assets/models/`:

```sh
python3 tools/gen_asset_manifest.py          # rewrites src/assets/manifest.rs
python3 tools/gen_asset_manifest.py --check  # exits 1 if it is stale (for CI)
```

The generator also validates every model. Each file must parse and contain a scene, every
external buffer or image URI must resolve to a file inside `assets/`, and every category
folder must be one of the names above. Any failure aborts with the offending path.

## Spawning an asset

```rust
fn spawn_mech(mut commands: Commands, mechs: Res<MechAssets>) {
    commands.spawn((
        WorldAssetRoot(mechs.animated_mech_textured_george.clone()),
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}
```

Load the collections first, in a loading state:

```rust
app.init_state::<GameState>().add_loading_state(load_all_collections(
    LoadingState::new(GameState::Loading).continue_to_state(GameState::InGame),
));
// ...or pick only what you need: .load_collection::<MechAssets>().load_collection::<MechAnimations>()
```

To animate, build a graph from the model's clips and play one by name. When the instance
fires `WorldInstanceReady`, insert the `AnimationGraphHandle` on the `AnimationPlayer`
entity under the spawned root:

```rust
let (graph, nodes) = AnimationGraph::from_clips(anims.animated_mech_textured_george.iter().cloned());
let idle = nodes[clips::animated_mech_textured_george::IDLE];
```

`src/assets/mod.rs` has the same example as a compile-checked doctest, and
`examples/asset_viewer.rs` has a complete one.

## Asset viewer

```sh
cargo run --example asset_viewer
```

The viewer loads every collection inside a loading state and lays all models out in one
block per category. Each model is scaled to fit a 3 m cell, and the overlay shows its native
size. Animated models play their idle clip. Controls: hold the right mouse button (or press
M) to look, WASD to move, Q/E to go down/up, Shift to run, and scroll to change speed.

For a smoke test, `ASSET_VIEWER_EXIT_AFTER=5 cargo run --example asset_viewer` exits once
everything has loaded and been fitted. Add `ASSET_VIEWER_SCREENSHOT=out.png` to save a
frame first.

## Adding a new pack

1. **Check the license.** Only CC0 or other licenses the team has approved. Keep the pack's
   license file.
2. **Download it outside the repo** and treat it as untrusted data. Put each archive in its
   own empty folder, extract it there, and never run anything from it. The layout the
   importer expects is `<work>/downloads/<pack>/...`.
3. **Prefer the pack's own glTF/GLB.** If it ships only FBX, convert it with Blender (next
   section).
4. **Describe the pack** in `PACKS` in `tools/import_packs.py`: source, pack name, folder,
   license file, and glob rules that map files to categories (the first match wins).
   Optionally add `texture_dirs` for textures not stored next to the `.gltf`, and
   `variations` for alternate palettes.
5. **Import, regenerate and check:**
   ```sh
   python3 -I tools/import_packs.py --work <work> --only <pack>
   python3 tools/gen_asset_manifest.py
   cargo build --examples && cargo run --example asset_viewer
   ```
6. **Add a row to `assets/CREDITS.md`** with the pack, author, URL, license and download
   date.

The importer repacks each `.gltf` or `.glb` into one `.glb` without loss. It merges the
buffers, embeds data-URI images, and moves external textures to `textures/` with rewritten
URIs. It also works around two export bugs seen in Quaternius packs: a `.gltf` naming
`<stem>_001.bin` when `<stem>.bin` ships (used only if the byte length matches), and
`*_png.png` texture names.

### FBX → GLB (Synty and other FBX-only packs)

Blender is not installed system-wide. Its official Python module works headless:

```sh
uv venv --python 3.11 <work>/bpyenv
VIRTUAL_ENV=<work>/bpyenv uv pip install bpy
<work>/bpyenv/bin/python -I tools/fbx2glb.py <work>/converted/<pack> <work>/downloads/<pack>/FBX
```

`tools/fbx2glb.py` imports each FBX and exports a GLB with animations, stripping Blender's
`Armature|` prefix from clip names. Converted output goes in `<work>/converted/<pack>/`, and
the pack's `PACKS` entry points `dir` there. Checked against a Quaternius model that also
ships a glTF, the conversion gave the same bounds, clips and skin.

**Synty packs** are paid and FBX-only, so they always take this path. They usually rely on a
shared texture atlas that the FBX references by an absolute or missing path. Expect to
assign the pack's atlas in Blender before export, or to list it under `texture_dirs`.
Converted Synty art goes to `assets/models/synty/<pack>/<category>/...`, like any other
source. Check the Synty license before committing: it is not CC0, and it may not allow
redistributing the source art in a public repository.

## Repository size

`assets/` is about 430 MB. `.gitattributes` tracks `*.glb`, `*.gltf`, `*.bin` and image files
with Git LFS. Install `git-lfs` and run `git lfs install` before committing the assets.
