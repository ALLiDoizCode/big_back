# Asset credits

Every pack below is dedicated to the public domain under
[CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/). The license was
checked on each pack's page and in the license file shipped with it, which is kept in the
pack's folder. Attribution is not required, but the authors are credited here anyway.

All packs were downloaded on **2026-10-18**.

## Quaternius ([quaternius.com](https://quaternius.com), [Patreon](https://www.patreon.com/quaternius))

| Pack | Folder | Source | License | Notes |
| --- | --- | --- | --- | --- |
| Animated Mech Pack | `models/quaternius/animated_mech/` | https://quaternius.com/packs/animatedmech.html | CC0 1.0 | Textured and flat-colour versions; colour-variation textures in `textures/variations/` |
| Animated Robot Pack | `models/quaternius/animated_robot/` | https://quaternius.com/packs/animatedrobot.html | CC0 1.0 | Shipped as FBX only; converted to GLB with Blender 5.0.1 |
| Cyberpunk Game Kit | `models/quaternius/cyberpunk_game_kit/` | https://quaternius.com/packs/cyberpunkgamekit.html | CC0 1.0 | |
| Downtown City MegaKit (Standard) | `models/quaternius/downtown_city/` | https://quaternius.com/packs/downtowncitymegakit.html | CC0 1.0 | Free Standard tier from itch.io; the paid Source tier was not downloaded |
| Ultimate Space Kit | `models/quaternius/ultimate_space_kit/` | https://quaternius.com/packs/ultimatespacekit.html | CC0 1.0 | |
| Ultimate Spaceships Pack | `models/quaternius/ultimate_spaceships/` | https://quaternius.com/packs/ultimatespaceships.html | CC0 1.0 | Default textures only; the four alternate colour textures per ship (~160 MB) were left out |
| Modular Sci-Fi MegaKit (Standard) | `models/quaternius/modular_scifi_megakit/` | https://quaternius.com/packs/modularscifimegakit.html | CC0 1.0 | Free Standard tier from itch.io; the paid Pro and Source tiers were not downloaded |
| Sci-Fi Essentials Kit (Standard) | `models/quaternius/scifi_essentials/` | https://quaternius.com/packs/scifiessentialskit.html | CC0 1.0 | Free Standard tier from itch.io; the paid Pro and Source tiers were not downloaded |
| Ultimate Modular Sci-Fi Pack | `models/quaternius/ultimate_modular_scifi/` | https://quaternius.com/packs/ultimatemodularscifi.html | CC0 1.0 | Shipped as FBX only; converted to GLB with Blender 5.0.1 |
| Universal Base Characters (Standard) | `models/quaternius/universal_base_characters/` | https://quaternius.com/packs/universalbasecharacters.html | CC0 1.0 | Free Standard tier from itch.io; the paid Source tier was not downloaded |
| Ultimate Animated Character Pack | `models/quaternius/ultimate_animated_character/` | https://quaternius.com/packs/ultimatedanimatedcharacter.html | CC0 1.0 | |
| Animated Alien Pack | `models/quaternius/animated_alien/` | https://quaternius.com/packs/animatedalien.html | CC0 1.0 | Shipped as FBX only; converted to GLB with Blender 5.0.1 |
| Sci-Fi Modular Gun Pack | `models/quaternius/scifi_modular_guns/` | https://quaternius.com/packs/scifimodularguns.html | CC0 1.0 | |
| Sci-Fi Gun Pack | `models/quaternius/scifi_guns/` | https://quaternius.com/packs/scifigun.html | CC0 1.0 | Shipped as FBX only; converted to GLB with Blender 5.0.1 |
| Ultimate Guns Pack | `models/quaternius/ultimate_guns/` | https://quaternius.com/packs/ultimategun.html | CC0 1.0 | Shipped as FBX only; converted to GLB with Blender 5.0.1 |

## Kenney ([kenney.nl](https://kenney.nl))

| Pack | Folder | Source | License | Notes |
| --- | --- | --- | --- | --- |
| Modular Space Kit | `models/kenney/modular_space_kit/` | https://kenney.nl/assets/modular-space-kit | CC0 1.0 | |
| Blaster Kit | `models/kenney/blaster_kit/` | https://kenney.nl/assets/blaster-kit | CC0 1.0 | |
| Factory Kit | `models/kenney/factory_kit/` | https://kenney.nl/assets/factory-kit | CC0 1.0 | |
| Platformer Kit | `models/kenney/platformer_kit/` | https://kenney.nl/assets/platformer-kit | CC0 1.0 | |
| Car Kit | `models/kenney/car_kit/` | https://kenney.nl/assets/car-kit | CC0 1.0 | |
| City Kit (Industrial) | `models/kenney/city_kit_industrial/` | https://kenney.nl/assets/city-kit-industrial | CC0 1.0 | |
| City Kit (Roads) | `models/kenney/city_kit_roads/` | https://kenney.nl/assets/city-kit-roads | CC0 1.0 | |

Kenney packs share one `textures/colormap.png` per pack. The alternate palettes from the
pack (`variation-*.png`) are in `textures/variations/`.

## Processing

Every model was repacked, without loss, into a single `.glb` by `tools/import_packs.py`.
External textures were moved to the pack's `textures/` folder and their references
rewritten. Packs shipped only as FBX were converted with `tools/fbx2glb.py` (Blender 5.0.1,
glTF exporter defaults). Animation clip names were kept, with Blender's `Armature|` prefix
removed. See `docs/assets.md`.
