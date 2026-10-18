#!/usr/bin/env python3
"""Import downloaded asset packs into assets/models/<source>/<pack>/<category>/.

Every glTF/GLB found by a pack's rules is repackaged, losslessly, as a single
.glb file:
  * all buffers (external .bin, data: URIs, GLB BIN chunks) are merged into
    the GLB's BIN chunk;
  * images embedded as data: URIs are moved into the BIN chunk;
  * external images are copied to <pack>/textures/ (snake_case names) and the
    image URI is rewritten to the new relative path, so references still
    resolve.
Nothing else about the model (meshes, materials, skins, animations) changes.

Packs that ship only FBX are first converted with tools/fbx2glb.py (Blender);
their output directory is then imported like any other GLB pack.

Usage:
    python3 -I tools/import_packs.py --work <dir> [--only pack_a,pack_b]

<dir> holds downloads/<pack>/... (extracted archives) and converted/<pack>/...
(FBX conversions). Files are only *read* from there; nothing is executed.
Re-running replaces assets/models/<source>/<pack>/ for each imported pack.
"""
import argparse
import base64
import fnmatch
import hashlib
import json
import os
import re
import shutil
import struct
import sys
import urllib.parse

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MODELS = os.path.join(REPO, "assets", "models")

# --------------------------------------------------------------------------
# Pack spec. `dir` is relative to --work. `rules` are (glob, category) pairs
# matched against the path relative to `dir`; the first match wins. A category
# may include one sub-folder ("weapons/parts"). Files matched by no rule are
# ignored. `texture_dirs` are fallback folders (relative to `dir`) searched for
# images a .gltf names but that are not next to it. `variations` are extra textures (alternate colour maps) copied to
# textures/variations/ even though no model references them.
# --------------------------------------------------------------------------
PACKS = [
    # ---------------------------------------------------------- Quaternius
    dict(source="quaternius", pack="animated_mech", dir="downloads/animatedmech",
         license="License.txt",
         rules=[("Textured/glTF/*.gltf", "mechs/textured"),
                ("Flat Colors/glTF/*.gltf", "mechs/flat")],
         variations=["Textured/Textures/Color Variations/*.png"]),
    dict(source="quaternius", pack="animated_robot", dir="converted/animatedrobot",
         license="../../downloads/animatedrobot/License.txt",
         rules=[("*.glb", "characters")]),
    dict(source="quaternius", pack="cyberpunk_game_kit", dir="downloads/cyberpunkgamekit",
         license="License.txt",
         rules=[("Character/*.gltf", "characters"),
                ("Enemies/Turret_*.gltf", "props"),
                ("Enemies/*.gltf", "characters"),
                ("Pickups and Objects/*.gltf", "props"),
                ("Platforms/Platform_*.gltf", "modular"),
                ("Platforms/*.gltf", "props")]),
    dict(source="quaternius", pack="downtown_city", dir="downloads/downtown-city-megakit/x",
         license="License_Standard.txt",
         rules=[("Exports/glTF (Godot)/Prop_*.gltf", "props"),
                ("Exports/glTF (Godot)/*.gltf", "modular")]),
    dict(source="quaternius", pack="ultimate_space_kit", dir="downloads/ultimatespacekit",
         license="License.txt",
         rules=[("Characters/GLTF/Mech_*.gltf", "mechs"),
                ("Characters/GLTF/*.gltf", "characters"),
                ("Vehicles/GLTF/Spaceship_*.gltf", "ships"),
                ("Vehicles/GLTF/*.gltf", "vehicles"),
                ("Items/GLTF/*.gltf", "props"),
                ("Environment/GLTF/*.gltf", "environment")]),
    dict(source="quaternius", pack="ultimate_spaceships", dir="downloads/ultimatespaceships",
         license="License.txt",
         rules=[("*/glTF/*.gltf", "ships")]),
    dict(source="quaternius", pack="modular_scifi_megakit",
         dir="downloads/modular-sci-fi-megakit/x/Modular SciFi MegaKit[Standard]",
         license="License_Standard.txt", texture_dirs=["Textures"],
         rules=[("glTF/Aliens/*.gltf", "characters"),
                ("glTF/Props/*.gltf", "props"),
                ("glTF/*/*.gltf", "modular")]),
    dict(source="quaternius", pack="scifi_essentials", dir="downloads/sci-fi-essentials-kit/x",
         license="License_Standard.txt",
         rules=[("glTF/Gun_*.gltf", "weapons"),
                ("glTF/Enemy_*.gltf", "characters"),
                ("glTF/*.gltf", "props")]),
    dict(source="quaternius", pack="ultimate_modular_scifi", dir="converted/ultimatemodularscifi",
         license="../../downloads/ultimatemodularscifi/License.txt",
         rules=[("Props_*.glb", "props"),
                ("*.glb", "modular"),
                ("*/*.glb", "modular")]),
    dict(source="quaternius", pack="universal_base_characters",
         dir="downloads/universal-base-characters/x/Universal Base Characters[Standard]",
         license="License_Standard.txt",
         rules=[("Base Characters/Godot - UE/*.gltf", "characters"),
                ("Hairstyles/Rigged to Head Bone/glTF (Godot -Unreal)/*.gltf", "characters/hair_rigged"),
                ("Hairstyles/Origin at 0/glTF (Godot)/*.gltf", "characters/hair")]),
    dict(source="quaternius", pack="ultimate_animated_character",
         dir="downloads/ultimatedanimatedcharacter",
         license="License.txt",
         rules=[("glTF/*.gltf", "characters")]),
    dict(source="quaternius", pack="animated_alien", dir="converted/animatedalien",
         license="../../downloads/animatedalien/License.txt",
         rules=[("*.glb", "characters")]),
    dict(source="quaternius", pack="scifi_modular_guns", dir="downloads/scifimodularguns",
         license="License.txt",
         rules=[("Guns/glTF/*.gltf", "weapons"),
                ("Modular Parts/glTF/*.gltf", "weapons/parts"),
                ("Modular Parts/glTF/*.glb", "weapons/parts")]),
    dict(source="quaternius", pack="scifi_guns", dir="converted/scifigun",
         license="../../downloads/scifigun/License.txt",
         rules=[("*.glb", "weapons")]),
    dict(source="quaternius", pack="ultimate_guns", dir="converted/ultimategun",
         license="../../downloads/ultimategun/License.txt",
         rules=[("Accessories/*.glb", "weapons/attachments"),
                ("*.glb", "weapons")]),
    # -------------------------------------------------------------- Kenney
    dict(source="kenney", pack="modular_space_kit", dir="downloads/kenney-modular-space-kit/x",
         license="License.txt",
         rules=[("Models/GLB format/*.glb", "modular")],
         variations=["Models/Textures/*.png"]),
    dict(source="kenney", pack="blaster_kit", dir="downloads/kenney-blaster-kit/x",
         license="License.txt",
         rules=[("Models/GLB format/crate-*.glb", "props"),
                ("Models/GLB format/target-*.glb", "props"),
                ("Models/GLB format/*.glb", "weapons")],
         variations=["Models/Textures/*.png"]),
    dict(source="kenney", pack="factory_kit", dir="downloads/kenney-factory-kit/x",
         license="License.txt",
         rules=[("Models/GLB format/*.glb", "modular")],
         variations=["Models/Textures/*.png"]),
    dict(source="kenney", pack="platformer_kit", dir="downloads/kenney-platformer-kit/x",
         license="License.txt",
         rules=[("Models/GLB format/character-*.glb", "characters")]
               + [(f"Models/GLB format/{p}*.glb", "environment") for p in (
                   "block", "platform", "tree", "fence", "hedge", "grass", "flower", "plant",
                   "rock", "stone", "mushroom", "ladder", "pole")]
               + [("Models/GLB format/*.glb", "props")],
         variations=["Models/Textures/*.png"]),
    dict(source="kenney", pack="car_kit", dir="downloads/kenney-car-kit/x",
         license="License.txt",
         rules=[("Models/GLB format/debris-*.glb", "props"),
                ("Models/GLB format/cone*.glb", "props"),
                ("Models/GLB format/box.glb", "props"),
                ("Models/GLB format/wheel-*.glb", "vehicles/wheels"),
                ("Models/GLB format/*.glb", "vehicles")],
         variations=["Models/Textures/*.png"]),
    dict(source="kenney", pack="city_kit_industrial", dir="downloads/kenney-city-kit-industrial/x",
         license="License.txt",
         rules=[("Models/GLB format/*.glb", "environment")],
         variations=["Models/Textures/*.png"]),
    dict(source="kenney", pack="city_kit_roads", dir="downloads/kenney-city-kit-roads/x",
         license="License.txt",
         rules=[("Models/GLB format/*.glb", "modular")],
         variations=["Models/Textures/*.png"]),
]

CATEGORIES = {"characters", "mechs", "vehicles", "weapons", "props", "environment", "modular", "ships"}


def snake(name: str) -> str:
    """'Spaceship_RaeTheRedPanda' -> 'spaceship_rae_the_red_panda', 'blaster-a' -> 'blaster_a'."""
    s = re.sub(r"[^0-9A-Za-z]+", "_", name)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return re.sub(r"_+", "_", s).strip("_").lower()


def find_ci(path: str) -> str:
    """Resolve a path, tolerating case mismatches (assets authored on Windows)."""
    if os.path.exists(path):
        return path
    d, base = os.path.split(path)
    d = find_ci(d)
    for f in os.listdir(d):
        if f.lower() == base.lower():
            return os.path.join(d, f)
    raise FileNotFoundError(path)


def read_gltf(path):
    data = open(path, "rb").read()
    if data[:4] == b"glTF":
        _, _, total = struct.unpack_from("<III", data, 0)
        off, js, bin_chunk = 12, None, None
        while off < total:
            ln, typ = struct.unpack_from("<II", data, off)
            chunk = data[off + 8: off + 8 + ln]
            if typ == 0x4E4F534A:
                js = json.loads(chunk)
            elif typ == 0x004E4942:
                bin_chunk = chunk
            off += 8 + ln
        return js, bin_chunk
    return json.loads(data), None


def decode_data_uri(uri):
    header, payload = uri.split(",", 1)
    mime = header[5:].split(";")[0]
    return mime, (base64.b64decode(payload) if ";base64" in header else urllib.parse.unquote_to_bytes(payload))


class Textures:
    """Copies external textures into <pack>/textures/, de-duplicated by content."""

    def __init__(self, root):
        self.root = root
        self.by_hash = {}
        self.names = {}

    def add(self, src, subdir=""):
        blob = open(src, "rb").read()
        h = hashlib.sha256(blob).hexdigest()
        key = (h, subdir)
        if key in self.by_hash:
            return self.by_hash[key]
        stem, ext = os.path.splitext(os.path.basename(src))
        name = snake(stem) + ext.lower()
        rel = os.path.join(subdir, name)
        n = 2
        while rel in self.names and self.names[rel] != h:
            rel = os.path.join(subdir, f"{snake(stem)}_{n}{ext.lower()}")
            n += 1
        self.names[rel] = h
        dst = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        with open(dst, "wb") as f:
            f.write(blob)
        self.by_hash[key] = dst
        return dst


def to_glb(src, dst, textures, texture_dirs=()):
    js, glb_bin = read_gltf(src)
    src_dir = os.path.dirname(src)
    out = bytearray()

    def append(blob, align=16):
        while len(out) % align:
            out.append(0)
        start = len(out)
        out.extend(blob)
        return start

    # Merge every buffer into one.
    bases = []
    for i, b in enumerate(js.get("buffers", [])):
        uri = b.get("uri")
        if uri is None:
            blob = glb_bin or b""
        elif uri.startswith("data:"):
            blob = decode_data_uri(uri)[1]
        else:
            try:
                path = find_ci(os.path.join(src_dir, urllib.parse.unquote(uri)))
            except FileNotFoundError:
                # Vendor export bug seen in the wild: the .gltf names "<stem>_001.bin"
                # but the pack ships "<stem>.bin". Accept it only if the size matches.
                path = os.path.splitext(src)[0] + ".bin"
                if not (os.path.exists(path) and os.path.getsize(path) == b["byteLength"]):
                    raise
                print(f"  note: {os.path.basename(src)} names missing {uri}; using {os.path.basename(path)}")
            blob = open(path, "rb").read()
        bases.append(append(blob[: b["byteLength"]]))
    for bv in js.get("bufferViews", []):
        bv["byteOffset"] = bv.get("byteOffset", 0) + bases[bv["buffer"]]
        bv["buffer"] = 0

    # Images: embed data URIs, relocate external files.
    for img in js.get("images", []):
        uri = img.get("uri")
        if uri is None:
            continue
        if uri.startswith("data:"):
            mime, blob = decode_data_uri(uri)
            js.setdefault("bufferViews", []).append(
                {"buffer": 0, "byteOffset": append(blob), "byteLength": len(blob)})
            img["bufferView"] = len(js["bufferViews"]) - 1
            img["mimeType"] = mime or "image/png"
            del img["uri"]
        else:
            name = urllib.parse.unquote(uri)
            # Vendor export bug: "T_Eye_Normal_png.png" referenced, "T_Eye_Normal.png" shipped.
            candidates = [name] + ([name[:-8] + ".png"] if name.endswith("_png.png") else [])
            found = None
            for cand in candidates:
                for d in (src_dir, *texture_dirs):
                    try:
                        found = find_ci(os.path.join(d, cand))
                        break
                    except FileNotFoundError:
                        pass
                if found:
                    if cand != name:
                        print(f"  note: {os.path.basename(src)} names missing {name}; using {cand}")
                    break
            if found is None:
                raise FileNotFoundError(f"{src}: image {uri}")
            tex = textures.add(found)
            img["uri"] = urllib.parse.quote(os.path.relpath(tex, os.path.dirname(dst)).replace(os.sep, "/"))

    while len(out) % 4:
        out.append(0)
    if out:
        js["buffers"] = [{"byteLength": len(out)}]
    else:
        js.pop("buffers", None)
    jbytes = json.dumps(js, separators=(",", ":")).encode()
    jbytes += b" " * (-len(jbytes) % 4)
    chunks = struct.pack("<II", len(jbytes), 0x4E4F534A) + jbytes
    if out:
        chunks += struct.pack("<II", len(out), 0x004E4942) + bytes(out)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    with open(dst, "wb") as f:
        f.write(struct.pack("<III", 0x46546C67, 2, 12 + len(chunks)) + chunks)


def import_pack(spec, work):
    base = os.path.join(work, spec["dir"])
    dest = os.path.join(MODELS, spec["source"], spec["pack"])
    if os.path.isdir(dest):
        shutil.rmtree(dest)
    os.makedirs(dest)
    textures = Textures(os.path.join(dest, "textures"))
    files = sorted(os.path.relpath(os.path.join(dp, f), base).replace(os.sep, "/")
                   for dp, _, fs in os.walk(base) for f in fs)
    done, count = set(), 0
    for rel in files:
        cat = next((c for pat, c in spec["rules"] if fnmatch.fnmatchcase(rel, pat)), None)
        if cat is None:
            continue
        assert cat.split("/")[0] in CATEGORIES, cat
        name = snake(os.path.splitext(os.path.basename(rel))[0])
        dst = os.path.join(dest, cat, name + ".glb")
        if dst in done:
            print(f"  skip duplicate {rel} -> {os.path.relpath(dst, REPO)}")
            continue
        done.add(dst)
        to_glb(os.path.join(base, rel), dst, textures,
               [os.path.join(base, d) for d in spec.get("texture_dirs", [])])
        count += 1
    for pat in spec.get("variations", []):
        for rel in files:
            if fnmatch.fnmatchcase(rel, pat):
                textures.add(os.path.join(base, rel), "variations")
    shutil.copy(os.path.join(base, spec["license"]), os.path.join(dest, os.path.basename(spec["license"])))
    size = sum(os.path.getsize(os.path.join(dp, f)) for dp, _, fs in os.walk(dest) for f in fs)
    print(f"{spec['source']}/{spec['pack']}: {count} models, {size / 1e6:.1f} MB")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--work", required=True, help="dir containing downloads/ and converted/")
    ap.add_argument("--only", default="", help="comma-separated pack names")
    a = ap.parse_args()
    only = set(filter(None, a.only.split(",")))
    for spec in PACKS:
        if not only or spec["pack"] in only:
            import_pack(spec, os.path.abspath(a.work))


if __name__ == "__main__":
    sys.exit(main())
