#!/usr/bin/env python3
"""Generate src/assets/manifest.rs from the models on disk under assets/models/.

Re-run after adding, removing or renaming any model:

    python3 tools/gen_asset_manifest.py          # write src/assets/manifest.rs
    python3 tools/gen_asset_manifest.py --check  # exit 1 if the file is stale

Layout it expects: assets/models/<source>/<pack>/<category>[/<sub>]/<name>.glb
(.gltf works too). For each category it emits:

  * `<Category>Assets`: one `Handle<WorldAsset>` field per model, loading
    `GltfAssetLabel::Scene(0)` (the `#Scene0` label), named `<pack>[_<sub>]_<name>`;
  * `<Category>Animations`: for every model with animations, a
    `Vec<Handle<AnimationClip>>` holding `GltfAssetLabel::Animation(n)` for each
    clip, in file order;
  * `clips::<field>::<CLIP>`: `usize` index constants into that Vec, named
    from the clip names stored in the glTF.

It also checks every model: the file parses, it has a scene, and every
external buffer/image URI resolves to a file on disk. Any failure aborts.
"""
import argparse
import json
import os
import re
import struct
import sys
import urllib.parse

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ASSETS = os.path.join(REPO, "assets")
MODELS = os.path.join(ASSETS, "models")
OUT = os.path.join(REPO, "src", "assets", "manifest.rs")

# Category folder -> Rust type stem. Order is the order collections load in.
CATEGORIES = {
    "characters": "Character",
    "mechs": "Mech",
    "vehicles": "Vehicle",
    "weapons": "Weapon",
    "props": "Prop",
    "environment": "Environment",
    "modular": "Modular",
    "ships": "Ship",
}

RUST_KEYWORDS = {"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
                 "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
                 "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
                 "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
                 "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen"}


def ident(s: str) -> str:
    s = re.sub(r"[^0-9A-Za-z]+", "_", s)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    s = re.sub(r"_+", "_", s).strip("_").lower()
    if not s or s[0].isdigit() or s in RUST_KEYWORDS:
        s = "m_" + s
    return s


def read_json(path):
    data = open(path, "rb").read()
    if data[:4] == b"glTF":
        n = struct.unpack_from("<I", data, 12)[0]
        return json.loads(data[20:20 + n])
    return json.loads(data)


def check_uris(path, js):
    base = os.path.dirname(path)
    for kind in ("buffers", "images"):
        for item in js.get(kind, []):
            uri = item.get("uri")
            if uri and not uri.startswith("data:"):
                target = os.path.normpath(os.path.join(base, urllib.parse.unquote(uri)))
                if not os.path.isfile(target):
                    raise SystemExit(f"error: {os.path.relpath(path, REPO)} references missing {uri}")
                if not target.startswith(ASSETS + os.sep):
                    raise SystemExit(f"error: {os.path.relpath(path, REPO)} references {uri} outside assets/")


def rust_str(s: str) -> str:
    return json.dumps(s)  # JSON string escaping is valid Rust for these paths


def collect():
    cats = {c: [] for c in CATEGORIES}
    seen = {}
    for source in sorted(os.listdir(MODELS)):
        sdir = os.path.join(MODELS, source)
        if not os.path.isdir(sdir):
            continue
        for pack in sorted(os.listdir(sdir)):
            pdir = os.path.join(sdir, pack)
            for dp, dns, fns in os.walk(pdir):
                dns.sort()
                for fn in sorted(fns):
                    if not fn.lower().endswith((".glb", ".gltf")):
                        continue
                    full = os.path.join(dp, fn)
                    rel_pack = os.path.relpath(full, pdir).replace(os.sep, "/")
                    parts = rel_pack.split("/")
                    if parts[0] not in CATEGORIES:
                        raise SystemExit(f"error: {os.path.relpath(full, REPO)}: unknown category "
                                         f"'{parts[0]}' (expected one of {', '.join(CATEGORIES)})")
                    js = read_json(full)
                    check_uris(full, js)
                    if not js.get("scenes"):
                        raise SystemExit(f"error: {os.path.relpath(full, REPO)} has no scenes")
                    stem = os.path.splitext(parts[-1])[0]
                    field = ident("_".join([pack, *parts[1:-1], stem]))
                    if field in seen:
                        field = ident(f"{source}_{field}")
                    if field in seen:
                        raise SystemExit(f"error: field name clash '{field}': {seen[field]} vs {full}")
                    seen[field] = full
                    asset_path = os.path.relpath(full, ASSETS).replace(os.sep, "/")
                    scene = js.get("scene", 0)
                    clips = [a.get("name") or f"animation_{i}" for i, a in enumerate(js.get("animations", []))]
                    cats[parts[0]].append(dict(field=field, path=asset_path, scene=scene, clips=clips))
    return cats


def clip_consts(clips):
    out, used = [], set()
    for i, name in enumerate(clips):
        c = re.sub(r"[^0-9A-Za-z]+", "_", name.split("|")[-1])
        c = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", c)
        c = re.sub(r"_+", "_", c).strip("_").upper() or f"CLIP_{i}"
        if c[0].isdigit():
            c = "CLIP_" + c
        base, n = c, 2
        while c in used:
            c = f"{base}_{n}"
            n += 1
        used.add(c)
        out.append((c, i, name))
    return out


def render(cats):
    w = []
    w.append("// @generated by tools/gen_asset_manifest.py from the files under assets/models/.")
    w.append("// Do not edit by hand: run `python3 tools/gen_asset_manifest.py` instead.")
    w.append("#![allow(missing_docs, clippy::all)]")
    w.append("")
    w.append("use bevy::prelude::*;")
    w.append("use bevy_asset_loader::prelude::*;")
    w.append("")
    loaders = []
    total_models = total_clips = 0
    for cat, stem in CATEGORIES.items():
        models = cats[cat]
        if not models:
            continue
        total_models += len(models)
        ty = f"{stem}Assets"
        loaders.append(ty)
        w.append(f"/// Every model in `assets/models/*/*/{cat}/`, as its default glTF scene (`#Scene0`).")
        w.append("#[derive(AssetCollection, Resource)]")
        w.append(f"pub struct {ty} {{")
        for m in models:
            w.append(f"    #[asset(path = {rust_str(m['path'] + '#Scene' + str(m['scene']))})]")
            w.append(f"    pub {m['field']}: Handle<WorldAsset>,")
        w.append("}")
        w.append("")
        w.append(f"impl {ty} {{")
        w.append("    /// `(field name, scene handle)` for every model in this collection.")
        w.append("    pub fn all(&self) -> Vec<(&'static str, &Handle<WorldAsset>)> {")
        w.append("        vec![")
        for m in models:
            w.append(f"            ({rust_str(m['field'])}, &self.{m['field']}),")
        w.append("        ]")
        w.append("    }")
        w.append("}")
        w.append("")
        animated = [m for m in models if m["clips"]]
        if animated:
            aty = f"{stem}Animations"
            loaders.append(aty)
            w.append(f"/// Animation clips (`#AnimationN`) of every animated model in `{cat}/`.")
            w.append(f"/// Index each Vec with the constants in [`clips`], e.g. `clips::{animated[0]['field']}::{clip_consts(animated[0]['clips'])[0][0]}`.")
            w.append("#[derive(AssetCollection, Resource)]")
            w.append(f"pub struct {aty} {{")
            for m in animated:
                total_clips += len(m["clips"])
                paths = ", ".join(rust_str(f"{m['path']}#Animation{i}") for i in range(len(m["clips"])))
                w.append(f"    #[asset(paths({paths}), collection(typed))]")
                w.append(f"    pub {m['field']}: Vec<Handle<AnimationClip>>,")
            w.append("}")
            w.append("")
            w.append(f"impl {aty} {{")
            w.append("    /// `(field name, clips in file order)` for every animated model in this collection.")
            w.append("    pub fn all(&self) -> Vec<(&'static str, &[Handle<AnimationClip>])> {")
            w.append("        vec![")
            for m in animated:
                w.append(f"            ({rust_str(m['field'])}, self.{m['field']}.as_slice()),")
            w.append("        ]")
            w.append("    }")
            w.append("}")
            w.append("")
    w.append("/// Clip indices per animated model, named from the clip names in the glTF.")
    w.append("pub mod clips {")
    for cat in CATEGORIES:
        for m in cats[cat]:
            if not m["clips"]:
                continue
            w.append(f"    pub mod {m['field']} {{")
            for c, i, name in clip_consts(m["clips"]):
                w.append(f"        /// glTF animation {i}: `{name}`")
                w.append(f"        pub const {c}: usize = {i};")
            names = ", ".join(rust_str(n) for n in m["clips"])
            w.append(f"        /// Clip names in file order.")
            w.append(f"        pub const NAMES: &[&str] = &[{names}];")
            w.append("    }")
    w.append("}")
    w.append("")
    w.append("/// Adds every generated collection to a loading state:")
    w.append("/// `LoadingState::new(S::Loading).continue_to_state(S::Ready).with(load_all_collections)`")
    w.append("/// or `load_all_collections(LoadingState::new(..))`.")
    w.append("pub fn load_all_collections<T: ConfigureLoadingState>(state: T) -> T {")
    w.append("    state")
    for ty in loaders:
        w.append(f"        .load_collection::<{ty}>()")
    w.append("}")
    w.append("")
    w.append("/// Every model scene in every loaded collection, as `(category, field name, handle)`.")
    w.append("/// Collections that are not inserted yet are skipped.")
    w.append("pub fn all_scenes(world: &World) -> Vec<(&'static str, &'static str, Handle<WorldAsset>)> {")
    w.append("    let mut out = Vec::new();")
    for cat, stem in CATEGORIES.items():
        if cats[cat]:
            w.append(f"    if let Some(c) = world.get_resource::<{stem}Assets>() {{")
            w.append(f"        out.extend(c.all().into_iter().map(|(n, h)| ({rust_str(cat)}, n, h.clone())));")
            w.append("    }")
    w.append("    out")
    w.append("}")
    w.append("")
    w.append("/// Clips of every animated model in every loaded collection, as")
    w.append("/// `(field name, clip names, clip handles)`; both lists are in glTF file order.")
    w.append("pub fn all_clips(world: &World) -> Vec<(&'static str, &'static [&'static str], Vec<Handle<AnimationClip>>)> {")
    w.append("    let mut out = Vec::new();")
    for cat, stem in CATEGORIES.items():
        animated = [m for m in cats[cat] if m["clips"]]
        if animated:
            w.append(f"    if let Some(c) = world.get_resource::<{stem}Animations>() {{")
            for m in animated:
                w.append(f"        out.push(({rust_str(m['field'])}, clips::{m['field']}::NAMES, c.{m['field']}.clone()));")
            w.append("    }")
    w.append("    out")
    w.append("}")
    w.append("")
    w.append(f"/// Number of models in the manifest.")
    w.append(f"pub const MODEL_COUNT: usize = {total_models};")
    w.append(f"/// Number of animation clips in the manifest.")
    w.append(f"pub const CLIP_COUNT: usize = {total_clips};")
    w.append("")
    return "\n".join(w), total_models, total_clips


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true", help="fail if manifest.rs is out of date")
    a = ap.parse_args()
    text, n, c = render(collect())
    if a.check:
        cur = open(OUT).read() if os.path.exists(OUT) else ""
        if cur != text:
            print("src/assets/manifest.rs is stale; run python3 tools/gen_asset_manifest.py", file=sys.stderr)
            return 1
        print(f"manifest up to date ({n} models, {c} clips)")
        return 0
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w") as f:
        f.write(text)
    print(f"wrote {os.path.relpath(OUT, REPO)}: {n} models, {c} animation clips")
    return 0


if __name__ == "__main__":
    sys.exit(main())
