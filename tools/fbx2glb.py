# Usage: python -I fbx2glb.py out_dir fbx_root   (converts every .fbx under fbx_root, keeping subfolders)  (runs inside a bpy venv)
import sys, os, bpy
out_dir, root = sys.argv[1], sys.argv[2]
inputs = [os.path.join(dp, f) for dp, _, fs in os.walk(root) for f in fs if f.lower().endswith('.fbx')]
os.makedirs(out_dir, exist_ok=True)
for src in inputs:
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.import_scene.fbx(filepath=src, use_anim=True, automatic_bone_orientation=False)
    for act in bpy.data.actions:
        act.name = act.name.split('|')[-1]
    out = os.path.join(out_dir, os.path.relpath(os.path.splitext(src)[0], root) + ".glb")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    bpy.ops.export_scene.gltf(filepath=out, export_format='GLB', export_animations=True,
                              export_apply=False, export_yup=True, export_image_format='AUTO')
    print("OK", src, "->", out, os.path.getsize(out))
