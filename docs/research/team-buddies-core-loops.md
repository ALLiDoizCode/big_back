# Team Buddies (PS1, 2000): core game loops

Research notes for designing a new game based on Team Buddies' core loops. Compiled 2026-10-18. Extended the same day with facts read from the game's own disc data and the decompilation project (see [Sources: decompilation & game data](#sources-decompilation--game-data)).

## How to read this

Every claim carries a source tag. The tag says how far to trust the claim:

| Tag | Meaning | Confidence |
|---|---|---|
| **[D]** | Disc data: the retail North American disc (SLUS-00869). This covers the game's own text (VR-training tutorial, mission and world briefings, menu strings) and its data tables, which I parsed myself | **Highest.** This is what the shipped game contains |
| **[D·inf]** | A value read from a disc table whose *field meaning* is inferred from patterns, not confirmed by code. The reasoning is given each time | Medium. The number is exact; what it measures is a best guess |
| **[C]** | Decompilation code ([hcbt/tbuddies-decomp](https://github.com/hcbt/tbuddies-decomp) at commit `d2ca1dc`) | High for what the code does. The decomp is early and mostly unnamed assembly, so it was used to confirm loaders, not to name every field |
| **[P]** | Primary: official Psygnosis/Sony press kit, press release or developer interview, or a 2000 magazine review | High for design intent and features. Medium for exact rules, because press copy is promotional |
| **[F]** | Hands-on fan FAQ from 2001 ("Twilight", GameFAQs), written by someone who finished every mission | Medium. It is secondary, but it is the only detailed source on controls and rules |
| **[S]** | Wiki or aggregator (Wikipedia, Giant Bomb, PSX Data Center) | Low. Use only to find leads; treat as unverified |

Where a [D] or [C] fact replaces an older claim, the old tag is kept next to it and marked **verified**, **corrected** or **still unverified**.

**Gap: the manual was not found, but it is now largely covered.** No scan of the instruction manual turned up on archive.org (searched by title and by serials SLUS-00869 and SCES-01923), replacementdocs or gamesdatabase.org. However, the disc holds the game's own rules text: a 9-lesson VR-training tutorial, 32 mission briefings, 8 world briefings that introduce each new Buddy class, and 5 multiplayer-mode rule screens. Together with the crate-recipe tables, these now cover the stack recipes, the scoring rules and most class abilities. The exact button mapping is still open (see [Open questions](#open-questions)).

### Correction to the brief

The brief names the developer as **MediaVision**. Every primary source says otherwise. The game was developed by **Psygnosis's Camden Studio in London** with a team of 14, and published by SCEE in Europe and Midway in North America ([P] press kit developer sheet [051199DE](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/051199DE.RTF); [P] producer Q&A [261099Q&](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099Q%26.RTF); [S] [Wikipedia](https://en.wikipedia.org/wiki/Team_Buddies)). The press kit calls Team Buddies the studio's last PlayStation title before it moved to PS2 work ([P] [051199DE](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/051199DE.RTF)).

### Key sources

- **Team Buddies UK press kit disc** (archive.org item [Team-Buddies_Press-Kit](https://archive.org/details/Team-Buddies_Press-Kit)). It holds these documents:
  - the producer interview with James Baker ([261099Q&](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099Q%26.RTF))
  - the AI interview with lead AI programmer Alex Whittaker ([261099AI](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099AI.RTF))
  - the AI press release ([261099A1](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099A1.RTF))
  - the story overview ([150300ST](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/150300ST.RTF))
  - the character "vox pop" interviews ([300500TB](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/300500TB.RTF))
  - the developer sheet ([051199DE](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/051199DE.RTF))
  - screenshots, such as [STACKING.TIF](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/SCREENSH/STACKING.TIF)
- **Sony PlayStation Autumn/Winter 1999 press CD**: the [overview sheet](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbover.rtf/99tbover.pdf) and the [September 1999 press release](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbrel.rtf/99tbrel.pdf).
- **ECTS 2000 PSone press kit**: the [June 2000 press release](https://discmaster.textfiles.com/file/18770/PSone_Presskit_ECTS2000.iso/software/team%20buddies/text/teamb-pressrel.rtf/teamb-pressrel.pdf).
- **2000 magazine reviews**:
  - *CVG* #227, p.124 ([PDF](https://retrocdn.net/images/8/8f/CVG_UK_227.pdf))
  - *EGM* #136, p.260 ([PDF](https://retrocdn.net/images/c/c0/EGM_US_136.pdf))
  - *GameFan* vol. 8 #11, pp.25 and 98 ([archive.org](https://archive.org/details/Gamefan_Vol_8_Issue_11/page/n101/mode/2up))
  - *GameSpot* ([archived](https://web.archive.org/web/20141017160644/http://www.gamespot.com/reviews/team-buddies-review/1900-2630100/))
  - *GameRevolution* ([review](https://www.gamerevolution.com/review/34069-team-buddies-review))
- **Fan FAQ**: "Team Buddies FAQ/Walkthrough" v2.5 by Twilight, 2001, originally on GameFAQs ([mirror PDF](https://www.video-games-museum.com/en/faqs/Playstation/29082-Team-Buddies.pdf)). The mirror's header credits "Haste", but the document itself says it was written by Twilight. Cited below as **FAQ**.
- **Game disc and decompilation**: see [Sources: decompilation & game data](#sources-decompilation--game-data) for the disc, the archive layout, the citation format (`BUDDIES.DAT #452 › TOYS.BIN` and similar) and the decomp commit.

---

## 1. Moment-to-moment loop

### The core verbs

The producer described the loop in one line: "The player builds up his team by collecting and stacking crates together and then kicking them open to reveal more Buddies, weapons and vehicles" ([P] [Baker Q&A](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099Q%26.RTF)). Crates "are parachuted into the battlefield", and "the more crates you find and stack, the bigger and better your toys will be" (same source).

The resulting verb chain:

1. **Pick up** a crate.
2. **Carry** it or **throw** it.
3. **Place** it on your stacking pad.
4. Optionally **merge** it with more crates.
5. **Kick it open** to get a "battle toy".
6. **Equip** the toy or **drive** it.
7. **Fight**.

Each step is detailed below.

### Controls (default layout; [F], with functions confirmed by [D])

| Button | Function |
|---|---|
| D-pad | Move the Buddy or vehicle |
| Square | Attack; pick up a crate; throw a crate |
| Triangle | Switch to another Buddy on your team |
| Circle | Kick a crate open; enter or exit a vehicle; drop crates from a vehicle |
| X | Jump; accelerate a vehicle; keep a plane aloft |
| R1 | Strafe or lock-on circle-strafe; vehicle modifier (rotates a tank turret, rolls a plane) |
| R2 | Binoculars, used to aim orders |
| L1 | Give an order to whatever the binoculars target |
| L2 | Give a build order (hold for a "free build" order) |
| Select | Rotate the camera |
| Circle + Square | Drop the current weapon |

Source: [F] FAQ §3 and §5.2.

The contemporary reviews agree on the overall shape:

- *GameSpot* wrote that the scheme "uses every button of the controller ... The D-pad maneuvers your Buddy, X shoots, and the other buttons control Buddy swaps, team commands, and object manipulation" ([P] [GameSpot](https://web.archive.org/web/20141017160644/http://www.gamespot.com/reviews/team-buddies-review/1900-2630100/)). **Conflict, probably explained:** *GameSpot* says X shoots, while the FAQ maps attack to Square. The game ships three preset layouts (Control Type A, B and C) plus a user-defined one, and the options screen lists jump, fire and action as remappable ([D] `BUDDIES.DAT #9`, the UI string table, "Control pause menu" and "control types" groups). So the two sources most likely used different presets (inference). Which preset is the default, and its exact mapping, is **still unverified**: the tutorial text refers to buttons by glyph codes, not names.
- The remappable actions are named Build, Order, Move, Rotate View, Binoculars, Swap, Lock On, Action, Jump, Fire and Pause ([D] `BUDDIES.DAT #9`, "button names" group). This **verifies** the FAQ's list of functions.
- *GameFan* confirms that build orders are cycled with **L2** and that **R1 is lock-on**. It also warns that "elevation and splash damage are deadly", so firing at point-blank range kills you ([P] [GameFan v8 #11 p.98](https://archive.org/details/Gamefan_Vol_8_Issue_11/page/n101/mode/2up)).

Both control sources describe a third-person camera with overhead and "binocular" views ([P] [1999 overview](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbover.rtf/99tbover.pdf)).

### Carrying and stacking crates

These rules come from [F] FAQ §3.2 except where noted. The VR-training tutorial text ([D] `BUDDIES.DAT #3`, byte offsets given) now confirms several of them:

- **Carrying:** you cannot attack while carrying a crate. You can throw it, for example down a hill so teammates can pick it up. **[D] adds a rule:** a crate can only be thrown while the Buddy is in the air, so a throw is jump-then-throw (offset 1652). The tutorial script checks for this before it lets the lesson continue ([D] `BUDDIES.DAT #4`, lesson 0).
- **Preview:** placing a crate on your pad shows a floating icon of what you would get if you opened it now. **Verified** [D]: small pictures of the build appear as you stack.
- **Merging:** a crate placed next to another on the pad merges with it into a bigger crate. Bigger crates give better toys but take longer to build. **Verified** [D] in principle: the tutorial says more crates give better toys (just after offset 7179).
  - A merged crate cannot be picked up or stolen. **Still unverified** [F].
  - A single, unmerged crate on a pad **can** be carried off by an enemy. **Still unverified** [F].
  - The **Ninja** Buddy's sword can split merged crates, including on an enemy's pad. **Verified** [D]: the World 3 briefing says the katana cuts through crates that have already been joined, and presents the Ninja as the sabotage unit (`BUDDIES.DAT #445 › W3_TEXT.TXT`, offset 996). Whether a split returns the single crates or destroys them is **still unverified**.
- **Opening:** you kick a crate open with Circle. A crate kicked open anywhere *off* a pad gives a small amount of health and ammo. **Corrected** [D]: an off-pad crate gives health **or** ammo, not both. The tutorial says this is the quickest way to reload and makes a steady crate supply the first goal of every mission (offset 3154). Kicking open enemy crates is therefore both healing and sabotage.
- **Abandoned toys:** **[D] adds a rule.** A finished toy left on the pad for too long gets opened by your own AI teammates. If you want an item for yourself, you have to be at the pad when it finishes (offset 10193).
- **More builders, faster builds:** **[D]** the tutorial says more Buddies build toys faster (offset 10705).
- **Wounded AI:** badly hurt Buddies, on either side, run to the nearest crates and break them for health.
- **Enemy pads:** the AI is designed to "run over and steal a crate from a base" ([P] [Whittaker AI Q&A](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099AI.RTF)). *EGM*'s reviewer found that repeatedly running "over to the Milkman's base to steal and break his boxes" was a highlight in multiplayer ([P] [EGM #136](https://retrocdn.net/images/c/c0/EGM_US_136.pdf)).
- **Pad shape:** the press screenshot shows a pad with a **2×2 footprint** marked by inward-pointing arrows. On it sits a merged crate that includes one blue super crate, and a Buddy is carrying a yellow crate with a red border ([P] [STACKING.TIF](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/SCREENSH/STACKING.TIF)). Wikipedia and Giant Bomb describe the pad as a **2×2×2** volume ([S] [Wikipedia](https://en.wikipedia.org/wiki/Team_Buddies)). That matches the FAQ's largest shape, eight crates as a "square cube". **Verified** [D]: the tutorial says the pad has four squares (offset 5267), and that putting eight crates on it fills it and always gives a vehicle (offset 7029). Four squares, two layers high, is the 2×2×2 volume.
- **Crate colours:** **verified** [D]. Red crates are the common kind and blue crates are the rare super kind (`BUDDIES.DAT #451 › W1_TEXT.TXT`, offset 707).

### Stack shape to output category

**Now verified from the game's recipe tables.** Every recipe set on the disc is a small file holding a count (always 6), followed by six slots. Each slot is a pair of toy numbers, a normal result and a super result ([D] `BUDDIES.DAT #452 › *_CRATECONTENTS.BIN` and `*_BT_*.BIN`, 28 bytes each). The toy numbers index the game's 180-entry toy list: names in `BUDDIES.DAT #10`, and the matching category and stat-table index in `#452 › TOYS.BIN`. The code that builds the toy list walks the names and the toy records in step, so name *n* is toy *n* ([C] `asm/eng/nonmatchings/eng/func_80071DBC.s`, which loads disc entry 10 and calls `func_80071B70` per toy).

Which shape is which slot:

| Slot | Crates and arrangement | Category | Evidence |
|---|---|---|---|
| 0 | 1 crate | Light weapon | [D] tutorial: one crate gives an Uzi (offset 1038); the VR-training set's slot 0 is the Uzi |
| 1 | 2 crates side by side (flat) | Grenade / throwable weapon | [D] tutorial: two side-by-side crates give grenades, and that horizontal stacks always give weapons (offsets 5615, 5735) |
| 2 | 2 crates stacked vertically | Basic Buddy | [D] tutorial: two stacked crates give an Infuntry, and that vertical stacks always give Buddies (offsets 6009, 6158) |
| 3 | 4 crates flat (the 2×2 layer) | Heavy weapon | [D·inf] slot 3 is a weapon in every set, so it is a horizontal shape; it is the larger one because slot 1 is already the 2-flat shape |
| 4 | 4 crates vertical (2 wide, 2 high) | Specialist Buddy, usually the world's new class | [D·inf] slot 4 is a Buddy in every set, so it is a vertical shape; it is the larger one because slot 2 is already the 2-high column |
| 5 | 8 crates (full 2×2×2 cube) | Vehicle | [D] tutorial: eight crates always give a vehicle (offset 7029); the VR set's slot 5 is a tank (offset 7179) |
| any | Same shape plus at least one blue super crate | The slot's **paired super toy** | [D] one super crate is enough to upgrade the whole build (offset 6857); one super crate on a 1-crate build gives the "Super Uzi" (offset 6690) |

The build-order button cycles through the same slot order. One press orders an Uzi (slot 0), and three presses order a Buddy (slot 2). A coloured silhouette on the pad shows the ordered shape ([D] offsets 10039, 10438, 10467). This **verifies** the FAQ's L2 description and confirms that slot order is also the order of the build-order menu.

**Corrected: a super crate swaps in a specific item, not a generic boost.** The FAQ and Wikipedia describe the super result as a stronger, faster version of the same thing. The tables show that each slot names its own super toy, and it is often a different item. Two examples: the Plains vehicle slot gives a Baby Rocket Tank, but with a super crate it gives a Baby Tandem Tank; the Flying 2 set's vehicle slot gives a Plane, or a Copter Gunship with a super crate. For Buddies, the super toy is a separate "Super ___" class with its own stat record (see [Buddy classes](#buddy-classes-7)).

**Per-world recipes (multiplayer "Battle Toys" sets for the 8 worlds)** ([D] `BUDDIES.DAT #452 › 0_BT_PARK_AREA.BIN` … `7_BT_MOON_AREA.BIN`). Each cell is *normal → with super crate*. The FAQ called this table "pretty impossible to make"; it is now complete:

| World set | 1 crate | 2 flat | 2 vertical | 4 flat | 4 vertical | 8 (vehicle) |
|---|---|---|---|---|---|---|
| 1 Park | Uzi 9mm → Uzi 12mm | Grenades → Super Grenades | Infuntry → Super Infuntry | Bazooka → Supazooka | Commando → Super Commando | Baby Tank → Baby Gatling Tank |
| 2 Plains | Shotgun → Shotgun Upgrade | Time Bomb → Time Bomb Upgrade | Infuntry → Super Infuntry | Gatling Gun → Gatling Cannon | **Medic** → Super Medic | Baby Rocket Tank → Baby Tandem Tank |
| 3 Desert | Pocket Rocket → Super Pocket Rocket | Napalm Grenade → Super Napalm Grenade | Commando → Super Commando | Flame Thrower → Flamejet | **Ninja** → Super Ninja | Medium Tank → Medium Stealth Rocket Tank |
| 4 Woods | Fireball → Thunderball | Cluster Grenade → Super Cluster Grenade | Commando → Super Commando | 4-Pack Rocket → Super 4-Pack Rocket | **Cyborg** → Super Cyborg | Tyrant Tank Rocket → Killer Tank Rocket |
| 5 Ice | Ice Gun → Liquid Nitro Gun | Ice Grenade → Super Ice Grenade | Commando → Super Commando | Power Orb Gun → Multi Orb Gun | **Stealthy** → Super Stealth | Baby Ice Tank → Medium Stealth Rocket Tank |
| 6 Oriental | Tazar → Tazar Upgrade | Scatter Bomb → Super Scatter Bomb | Commando → Super Commando | 4-Pack Missile → Super 4-Pack Missile | **Super** → Super Super Buddie | Baby Hover Tank → Super Hover Tank Flame |
| 7 Jungle | Pocket Missile → Pocket Missile Upgrade | Lightning Grenade → Super Lightning Grenade | Cyborg → Super Cyborg | Lightning Gun → Krypto Light Gun | Stealthy → Super Stealth | Killer Homing Missile Tank → Super Flame Tank |
| 8 Moon | Laser Rifle → Laser Cannon | Dam Buster → Super Dam Buster | Cyborg → Super Cyborg | Pulse Rifle → Pulse Cannon | Super → Super Super Buddie | Mech Walker → Mechwalker Upgrade |

Patterns visible in the table [D]:

- **The 4-vertical slot introduces each world's new Buddy class**, in the same order the FAQ gives for availability: Medic (W2), Ninja (W3), Cyborg (W4), Stealthy (W5), Super (W6). This **verifies** the FAQ's "Available from" column.
- **The 2-vertical "basic" Buddy is promoted as the campaign goes on:** Infuntry in Worlds 1–2, Commando in Worlds 3–6, Cyborg in Worlds 7–8. This **closes** the old "which class comes from 2 versus 4 crates" question.
- **Every slot escalates by world.** The light weapon goes Uzi → Shotgun → Pocket Rocket → … → Laser Rifle, and the vehicle goes baby tanks → medium and heavy tanks → Mech Walker. The in-fiction TV producers adding "bigger, more vicious weapons" ([P] story) is built straight into the tables.

**Campaign missions have their own sets.** There are 41 per-level sets (`0_CRATECONTENTS.BIN` … `40_CRATECONTENTS.BIN`). Most copy or slightly vary their world's set; for example, Plains missions swap the 2-vertical or 4-vertical class or the tank variant. Some are special-purpose. **Verified** [D]: this is the FAQ's "sometimes even level to level". The per-level set index appears to follow the order of the level-name list (`BUDDIES.DAT #759 › LEVELS_ENG.TXT`) [D·inf]. Set 14 is the only one containing the "V Boss Tank Sheep Sucking", and name 14 is *Ewe Fiend*, the Wolf-Tank boss. Set 27 is a Park set, and name 27 is *On Yer Bike*, a World 1 mission. Set 4 (*The Rong Brothers*) gives a **Plane from just two flat crates**, which fits that briefing's promise that R&D is sending a plane ([D] `#414 › 008_TEXT.TXT`).

**Multiplayer sets.** The battle setup screen has a "Battle Toys" option with 25 choices: World 1–8, Flame, Lightning, Grenades 1–2, Heavy 1–2, Rocket, Missile, Special, Flying 1–4, Stealth 1–2, Space and Bikes ([D] `BUDDIES.DAT #9`, "crate settings" group, offset 4580). Each choice has a matching `*_BT_*.BIN` table. Examples: *Flying 1* builds Microlite → Plane from 8 crates; *Bikes* builds Bike → Motorbike; *Special* builds Laser, the Ankh, the Armageddon Barrel and a hover rocket tank. **Verified:** this is the FAQ's "Crate Contents" selection, and its Flame set exists. "Funky" is not the name of any set table. It appears only in results messages that make "Funky crate contents" available ([D] `#760 › LEVELCOMMENTS_ENG.TXT`), alongside a "secret crate contents selection" unlock (`#762`). So Funky is probably the label for the whole secret-set menu (inference).

So each shape means a *tier slot*, and the world or set chooses what fills it, now including the super variant:

```
shape → slot 0..5                         (fixed: 1, 2-flat, 2-up, 4-flat, 4-up, 8)
(world or set, slot, super?) → toy        (data table, 6 × 2 entries per set)
```

**Super crates:** one super crate is enough to make the whole build "super" ([F] FAQ §3.2). **Verified** [D] (tutorial offset 6857).

**Megacrates** are grey 8×-size crates. They cannot be built. They drop when a trigger fires, such as a switch, a timer or an event, and they hold a strong vehicle or weapon. In the campaign this is gear from outside the current world's set ([F] FAQ §3.2; [S] Giant Bomb). According to the FAQ, the enemy AI breaks megacrates open but never uses what is inside. **Partly verified** [D]: the game has on-screen alerts for "megacrate dropped" and "drop zone activated", so both megacrates and extra drop zones are switched on by events during play ([D] `BUDDIES.DAT #9`, "Misc. in game text" group). Several briefings point the player at switches or event triggers for megacrates ([D] `#408 › 002_TEXT.TXT`, `#431 › 025_TEXT.TXT`). The AI's handling of megacrates is **still unverified**.

### Combat

- **Weapons and health:**
  - Weapons use ammo. Health and ammo both start at 100 on the HUD ([P] [STACKING.TIF](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/SCREENSH/STACKING.TIF)).
  - **[D] adds a rule:** ammo belongs to the Buddy, not to the weapon or vehicle. Picking up a new gun does not bring its own magazine; it draws on the same pool (`BUDDIES.DAT #3`, offset 15937).
  - An Ammo Cache power-up raises ammo to its maximum of 500 ([F] FAQ §6.2). **Consistent** with [D·inf]: the Armageddon Barrel's per-shot ammo cost field is 500, a full pool (see [Weapons](#weapons-by-stack-tier)).
  - Splash damage hurts your own side, and elevation matters ([P] [GameFan](https://archive.org/details/Gamefan_Vol_8_Issue_11/page/n101/mode/2up)).
- **Death:** a killed Buddy leaves its "smoking boots" behind ([F] FAQ §2; [P] story [150300ST](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/150300ST.RTF)).
- **Vehicles** ([F] FAQ §5.3, §6.3):
  - Vehicles are neutral, so anyone can hijack one.
  - While inside, the driver takes no damage; the vehicle does.
  - A wrecked vehicle explodes, and you can drive a dying one into enemies as a bomb.
  - Driving over crates collects them, up to a per-vehicle limit. Wikipedia says most vehicles carry 2 and larger ones carry 3 ([S]).
  - Vehicles crush Buddies. Cyborgs are the exception.
  - Mounted weapons use no ammo.
- **Terrain:** the ground deforms with craters and scorch marks ([S] Wikipedia).
- **Breakable scenery:** trees and other objects can be destroyed for power-ups and sometimes weapons ([F] FAQ §5.3; [S] Wikipedia caption).
- **Power-ups** ([F] FAQ §6.2): Health; Ammo; Ammo Cache; Gun Frenzy (unlimited ammo); Pumped Up (full heal); Rocket Boots; Fishy (slow) / Phreak (double speed), which share one random icon; Shield (invincible); Jetpack; Base Repair; and "?" (random, multiplayer only). Only one power-up can be active at a time. Baker's Q&A also mentions roller skates and a jet pack as pickups ([P]).
  - **[D] full list:** the toy table defines 22 power-up entries. They are Sugar, Ammo, Speed Up, Gun Frenzy (listed twice), Pumped Up, Ammo Cache, Shield, Jet Pack, Rocket Boots, Big Slap, Base Repair, Teleporter, Super Vision, Floater, Break Weapon, Freeze, Phreak, Fishy, Heavy Boots, Random and Rubbish (category 0 in `BUDDIES.DAT #452 › TOYS.BIN`; names from `#10`). The engine also names equipment models for jetpack, rocket boots, **skates**, goggles and boots (ENG.BIN strings next to the data-file names). The skates **verify** Baker's "roller skates". A per-power-up number table exists (`#452 › POWERUP.BIN`, read by [C] `asm/eng/nonmatchings/eng/func_8008102C.s`), but what it measures (duration?) is unconfirmed.

### Buddy AI and squad commands

**Squad size.** "Each team consists of up to four of the little darlings" ([P] [Baker Q&A](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099Q%26.RTF)). You control one Buddy directly and can switch to any of the others at will ([P] [1999 release](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbrel.rtf/99tbrel.pdf)).

**Command model** ([F] FAQ §3.1):

- **Aiming orders.** Raise the binoculars (R2), put the target square on something, and press L1:
  - Tap L1 to send the nearest Buddy.
  - Hold L1 to send the whole team.
  - The Buddy who takes the order salutes.
- **What an order does depends on the target:**

  | Target | Effect |
  |---|---|
  | Your base | Defend it |
  | Enemy base | Attack it |
  | Enemy Buddy or vehicle | Kill it |
  | Ally | Escort it |
  | Red or super crate | Bring it to your pad |
  | Megacrate | Open it |
  | Empty vehicle | Get in |
  | Empty ground | Move there |
  | Nothing, or yourself | Follow me (the default "protect" mode) |

- **Build orders.** Press L2 to choose what the AI should build. A ghost outline appears on the pad showing where AI Buddies will place crates. Hold L2 for "free build", where the AI decides what to build. Only Buddies in "protect me" mode build. Buddies in vehicles or turrets do not.
  - **Verified** [D]: tapping the build button cycles the build configurations; a coloured silhouette shows the ordered shape; holding it gives a free-build order (a world icon appears), and tapping again cancels it (`BUDDIES.DAT #3`, offsets 10039–10467 and the free-build lines after them).
  - **Verified** [D]: a single tap of the order button orders one Buddy and holding it for about a second orders the whole team. Orders are context-sensitive: your base means defend and the enemy base means attack.
  - **Corrected detail** [D]: the tutorial says teammates do nothing unless ordered. The FAQ's default "protect me" mode is presumably their idle behaviour, not autonomous play (inference).

**AI architecture** ([P] [Whittaker AI Q&A](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099AI.RTF); [P] [AI press release](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099A1.RTF)):

- **Data-driven state machines.** Agents run on Augmented Transition Networks: percept-driven state machines with registers that act as short-term memory. Designers edit the behaviour data in a dedicated editor, like level data, "with no impact on the programmer's time".
  - **Verified** [D][C]: the AI really is shipped as data. The same bundle that holds the stat tables also holds `PERCEPT.BIN` (807 entries), `ATTITUDE.BIN` (184) and `ACTION.BIN` (675) (`BUDDIES.DAT #452`). The engine's loader opens that bundle (disc entry 0x1C4 = 452), looks each file up by name, and passes it to its own parser ([C] `asm/eng/nonmatchings/eng/func_80080D70.s`, which calls `func_800C6198` for percepts, `func_800C6408` for attitudes and `func_800CC63C` for actions). GAME.BIN keeps debug labels for "Percept Bins", "Attitude Percept Bins", "Action List" and "Sub Action List", which match the press kit's vocabulary of percepts, attitudes and (sub-)actions.
- **Goal-directed crate use.** Registers let an agent reason about one specific crate: "look for the nearest crate, decide to go and get it to take back to the base, and use it to build a shotgun."
- **Decisions the AI makes:** what to build, whether to fight or flee, when to switch strategy, how to obey player orders, and how to coordinate ("one agent to do something while the others wait or help out").
- **Named enemy strategies:**
  - **ambush**: lure the player back to a base where the team is armed and waiting
  - **crate theft** from bases
  - **joyriding** vehicles
  - **sniping**
  - **assassination**
- **No cheating.** The enemy AI uses the same intelligence as your team and does not see the whole map.
- **"Artificial Incompetence"** ([P] 1999 overview; Whittaker):
  - Agents are built smart first, then made "stupid in a more slapstick way". For example, an agent builds a bazooka and then shoots a teammate with it.
  - Each agent has four attitude percepts: aggressive, defensive, hindering and stupid.
  - **Aggression starts at zero and rises as the level goes on**, so late-level fights are harder. **Still unverified:** the attitude table has no field names, and the decomp's AI code is unnamed assembly, so the ramp could not be traced.
  - A random percept is added "with the 91 other variables" to make behaviour fuzzier. **Still unverified.**
- **How it played.** Whittaker: "it's actually quite difficult to win a game" ([P]). Critics split:
  - *GameSpot*: "your CPU-controlled teammates are useless" ([P]).
  - *GameFan*: Buddies "frequently get themselves killed by friendly fire" ([P]).
  - FAQ: "Your team's AI is good to begin with, however if you learn team command properly, it'll become lethal" ([F]).

---

## 2. Match loop

### Base, pad and the "decapitation" rule

- **Base layout:** each team has a stacking pad and a large base building next to it.
- **Losing the base:** destroying a base disables that team's pad, so it can no longer build Buddies, weapons or vehicles ([S] Wikipedia; [S] Giant Bomb; [F] FAQ §5.3). The FAQ's advice is to "always damage an enemy bases, if not destroy them". Destroying bases is the main strategic goal, and the press kit's pitch is "attack the enemies bases and bring their crate stacking to a screeching halt" ([P] [1999 overview](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbover.rtf/99tbover.pdf)).
  - **Verified** [D]. The tutorial says that once a base is destroyed its team can no longer use the pad to stack crates, so it can neither upgrade weapons nor build teammates (`BUDDIES.DAT #3`, offset 3750). The game shows a "stacking pad deactivated" alert and a per-colour "base destroyed" alert for each of the nine team colours (blue, brown, green, grey, orange, pink, purple, red, yellow) ([D] `#9`, offset 6179). The first mission's briefing repeats the rule as a tip ([D] `#407 › 001_TEXT.TXT`).
  - **[D] adds:** a base's health is shown by a flag beside it, which lowers as the base takes damage; when the flag reaches the ground, the base is destroyed (`#3`, offset 3621). In some campaign missions losing your own base is itself a fail condition ("your base was destroyed" is one of the mission-fail messages, `#760 › LEVELCOMMENTS_ENG.TXT`).
- **Fighting on without a base:** a team that has lost its base can still smash its leftover crates for health and ammo, and pick up weapons from the map or from enemies. A fast unit can grab a weapon from an enemy pad as it is opened ([S] Wikipedia; [F] FAQ §5.3). **Partly verified** [D]: in Domination no team can be wiped out before time runs out, but a team without a base will struggle to arm itself (`#415 › 009_TEXT.TXT`).
- **Alerts:** the radar flashes red when your base is under fire ([F]). **[D] adds:** coloured arrows on the radar's edge always point to each enemy pad unless it is in range, and grey dots mark civilians and animals, which are hostile to every team alike (`#3`, HUD lesson).

### Crate supply and contest

- **Drop zones:** crates fall continuously at set "crate drop zones" ([P] *GameRevolution*: "Crates fall from the sky constantly"; [P] Baker: "parachuted"). A typical map has a small drop near each base and a larger one in the middle of the map ([S] Wikipedia). Super crates sometimes drop at separate zones far from your base ([F] FAQ §3.2, §5.3).
- **Scarcity missions:** some campaign missions limit the crate supply ("sometimes crates in your area are limited, so build wisely" ([P] *GameRevolution*)) or start you with a fixed crate budget, so you have to steal ([S] Giant Bomb).
- **Ways to contest crates:**
  - steal single crates from enemy pads
  - kick enemy crates open
  - split merged crates with a Ninja
  - intercept carriers
  - use vehicles as fast crate haulers

  The FAQ also notes that lions eat crates ([F] FAQ §5.3, §6.5).
- **Neutral animals** add noise: "crate stealing cheeky monkeys, buddy loving pigs and irritable rams" ([P] [1999 overview](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbover.rtf/99tbover.pdf)), plus bears, bulls and so on ([F]; [P] *GameRevolution*).

### Team growth

- **Starting team:** you start with one Buddy, or a handful (*CVG*: "You kick off with a handful of unarmed buddies" ([P] [CVG #227](https://retrocdn.net/images/8/8f/CVG_UK_227.pdf))).
- **Growing the team:** you build more Buddies with vertical stacks, up to four in total ([P] Baker; [S] Wikipedia: "Up to three additional team members").
- **Typical build order** in FAQ walkthroughs ([F] FAQ §4.1):
  1. Build a Buddy.
  2. Order the AI to build another Buddy.
  3. Order the AI to build heavy weapons for everyone.
  4. Grab a super crate for one key unit.
  5. Attack, leaving one Buddy guarding the base.

### Win conditions and match length

- **Skirmish / Deathmatch:** eliminate every member of every opposing team ([S] Wikipedia). Matches always have a time limit ([S] Giant Bomb). **Corrected** [D]: the battle setup's "Battle Limit" option offers Infinite, Short, Medium, Long or Points, so a match can be untimed (`BUDDIES.DAT #9`, "time selections" group, offset 4523). The other battle options are Battle Type, Battle Toys (the crate set) and Ghosts.
- **Campaign missions:** a timer of **5–10 minutes, or no limit**, depending on the mission. Each has prime objectives and optional sub-objectives, and the mission ends the moment the prime objectives are done ([F] FAQ §4; timers listed per mission). From the FAQ's list of the 32 main missions, timed missions run 5, 6, 7, 8 or 10 minutes. **Partly verified** [D]: the results screen has three outcomes: incomplete; complete but sub-objective missed; and complete with all objectives met (`#760 › LEVELCOMMENTS_ENG.TXT`). The briefings give some timers directly; for example, *Bowwow Ker-Pow* is a 5-minute hold-out (`#410 › 004_TEXT.TXT`). The per-mission timer values are **still [F]**. `LEVELS.BIN` has a field holding 300 (5 minutes?) in almost every story record, but I could not confirm that it is the timer.
- **Estimated length:** about 5–10 minutes per match. This is inferred from the timers and is not measured.

### Campaign mission types

Classified from the FAQ's per-mission objectives ([F] FAQ §4; examples confirmed by [P] reviews). **Verified** [D] against the 32 in-game briefings (`BUDDIES.DAT #407`–`#438`, `001_TEXT.TXT`–`032_TEXT.TXT`). Every type below has at least one briefing, and the briefings add a few objective details the FAQ lacks: rescue three pigs by stunning and carrying them (006); keep every camel of a convoy alive (010); destroy 3 generators and then the boss's power exchange (030); carry Ikimen one at a time onto a teleport pad after lighting 4 beacons (031).

| Type | Examples ([F]) | Primary corroboration |
|---|---|---|
| Annihilate the enemy | Battle Hill, Quarrelsome Quarry, Ice Scream, Space City | [P] *GameRevolution*: "Destroying all other colors" |
| Recover an item or vehicle and bring it home | On Yer Bike (bike), Country Vile (tractor), Fumble in the Jungle (Stealth Harrier) | [P] *CVG*: "retrieving the stolen bicycle"; *GameRevolution*: "Repossessing your bike" |
| Herd or rescue animals | Piggy in the Middle, Herdy Birdy (penguins), Camel Strain | [P] *CVG*: "rounding up the lost pets"; Baker: "rescuing pigs from certain sausagedom" |
| Escort VIPs or hostages | The Rong Brothers (generals plus bus), Little Red Riding Hood, Inflammable Cannibals (6 hostages) | [P] *GameSpot*: "hostage freeing" |
| Hold out against a timer | Bowwow Ker-Pow (5 min), the Droopy Wood research-facility defence | — |
| Capture or convert points | That's Rubbish (right 6 bins), A-Nile-Ation (5 ankhs), Chinese Burn (5 lanterns) | [P] *GameRevolution*: trash cans |
| Collectibles (often sub-objectives) | 10 rubbish scraps | [P] *GameSpot*: "waste disposal, weapon component gathering" |
| Boss | Ewe Fiend (Wolf-Tank), Mad Mechs, Bossa Nova (Megamech) | [P] Baker: "dealing with big-headed scientists" |

In boss missions "you can't build anything, and are limited to a single Buddie" ([F] FAQ §3). Giant Bomb adds that some bosses destroy your base at the start ([S]).

---

## 3. Meta and progression loop

### Campaign structure

The campaign is called "Story Mode". You pick a team colour, then play the worlds in order. Each world's missions must be finished before the next world opens ([F] FAQ §6.4). There are eight worlds:

1. Central Park
2. Insane Plains
3. Dirtbox Desert
4. Droopy Wood
5. Chilling Fields
6. Ori-Mental Peaks
7. Rubber Jungle
8. Loon-A Moon

Sources: [F] FAQ table of contents; [S] [PSX Data Center](https://psxdatacenter.com/games/P/T/SCES-01923.html). Baker describes "Buddie World" as seven areas plus the finale ([P]), which fits seven planet worlds plus the moon.

**The mission count conflicts between sources:**

| Source | Count |
|---|---|
| [P] Sept 1999 release | "64 incredibly challenging levels in the regular Story Mode" and "32 playgrounds spanning 8 themed areas" ([99tbrel](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbrel.rtf/99tbrel.pdf)) |
| [P] 1999 overview | "8 Playgrounds, 32 Arenas and 64 manic missions" ([99tbover](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbover.rtf/99tbover.pdf)) |
| [P] June 2000 ECTS release | "**32** incredibly challenging levels in the regular Story Mode" ([teamb-pressrel](https://discmaster.textfiles.com/file/18770/PSone_Presskit_ECTS2000.iso/software/team%20buddies/text/teamb-pressrel.rtf/teamb-pressrel.pdf)) |
| [P] *GameSpot* and *GameRevolution* | 64 missions |
| [F] FAQ | Says 64 in total, but its walkthrough documents **32 story missions, 4 per world**. Missions also unlock separate **VR (training) missions** |
| **[D] the disc** | **32 story missions: 8 worlds × 4.** Records 0–31 of the level table each carry a world number 1–8 and a slot number 1–4, with every world/slot pair used exactly once (`BUDDIES.DAT #453 › LEVELS.BIN`, 48 records of 128 bytes; fields at +0x24 and +0x28 [D·inf], read as world and slot because their values fill an 8 × 4 grid exactly). The disc also has exactly 32 mission briefings (`001`–`032_TEXT.TXT`), 32 mission splash screens (`SP_BH` … `SP_SC` in SYS.BIN's file list) and 32 story names in `LEVELS_ENG.TXT` |

**Corrected: there are 32 missions, not 64.** The rest of the level table is not a second campaign:

- Records 32–33 form a ninth group: VR training and the lobby.
- Records 34–47 have no world number. They are presumably multiplayer arenas: the unlock messages name about twenty battle, capture, domination and Bomb-Ball maps, such as *Footie Loose*, *Golf War* and *Moon Goes Boom* (`#762 › SECRETCOMMENTS_ENG.TXT`).
- The VR training is a 9-lesson tutorial (`#761 › TUTORIALS_ENG.TXT`; script `#4`, lessons 0–8), not a set of 32 missions.

The 1999 "64 missions / 32 arenas" figures do not match the shipped data. The June 2000 release's "32 levels" does. That the 64 figure was a pre-release plan is an inference.

**[D] which missions belong to which world.** Read in order, the 32 story names in `LEVELS_ENG.TXT` fall into blocks of four, one block per world. The exception is *On Yer Bike*, which is listed 28th but whose level record is World 1. The blocks agree with the FAQ's per-world mission lists, and the briefings 001–004, 005–008, … 029–032 match those blocks by content. The order of missions *within* a world is **not settled**: the level table's slot field and the briefing numbering disagree for some worlds. For example, the data puts *Piggy in the Middle* in World 2 slot 1, but briefing 005 is the quarry mission and the pig rescue is 006. Mission names are not stored next to the briefings, so I have not paired them one by one.

Each world also changes which battle toys the stack shapes produce ([F]; [S] Wikipedia).

### Medals and unlocks

- **Medals:** finishing a mission with *all* objectives, prime and sub, earns a gold medal. Gold is required for some unlocks ([F] FAQ §4, §6.4). **Verified** [D]: the results text separates "complete, sub-objective missed" from "complete with all objectives met", and some unlock messages are tied to the higher grade. One example is "all objectives met: Funky crate contents available"; another is the "exemplary performance" message that unlocks Ghost mode, the Baddie skins and the Funky set together (`BUDDIES.DAT #760 › LEVELCOMMENTS_ENG.TXT`).
- **What unlocks, and where it can be used.** Everything unlocks in the campaign but is used **only in multiplayer** ([F] FAQ §3, §6.4; [P] *GameRevolution*: "a new option opens in the multi-player"; [P] *EGM*: "you unlock more and more secrets to use in multiplayer"):
  - new modes
  - new maps
  - new crate-content sets (each world's toy set, plus secret sets such as "Flame Weapons" and "Funky")
  - skins (naked, undead, English football kit, VR and others). **Verified** [D]: the skin menu lists Default, Naked, Undead, Football, Suits, Space, VR and Baddies (`#9`, "skin names" group)
  - VR missions. **Corrected** [D]: the unlock messages name VR *maps* for multiplayer (for example "VR Battle Hill" and the "Virtual Brutality" domination map), not extra VR campaign missions (`#762 › SECRETCOMMENTS_ENG.TXT`)

  **[D] the full unlock list.** There are 40 unlock messages. They cover the Domination, Bomb-Ball and Capture modes (each unlocked together with a first map); 7 world crate sets (Insane Plains through Loon-A Moon); 5 named secret sets (Heavy Weapons, Rockets, Flying 1, Flying 2, Flame Weapons) plus a general "secret crate contents selection" and the Funky set; 6 skins; Ghost mode and the Baddies team; and 21 multiplayer maps. Some of these maps are new, multiplayer-only arenas, such as *Pork Scratchin'*, *Hot Doggie*, *Running Lamb*, *Radgy Goo Goo Chopper*, *Footie Loose*, *Golf War*, *Slippery Pop-Shot* and *Moon Goes Boom*. Which mission triggers which message is **still [F]**: the messages are stored as a plain list.
- **Five unlock examples** ([F] FAQ §4, §6.4):

  | Mission | Unlocks |
  |---|---|
  | On Yer Bike | Capture mode |
  | That's Rubbish | Domination mode |
  | The Rong Brothers | Bomb-Ball mode |
  | Bossa Nova (final mission) | Ghost option |
  | Country Vile | Insane Plains crate contents |

### Multiplayer

- **Players and screens:** 1–4 players, with a Multitap needed for 3–4. Two players get a horizontal split screen; three or four get a quad split ([P] Baker Q&A; [P] 1999 overview). There is no link-cable support ([F]; [P] *GameRevolution*).
- **Win condition choice** [D]: every scoring mode lets you pick either a time limit, where the most points at full time wins, or a points target, where the first team to reach it wins (`#439`–`#443`, `033`–`037_TEXT.TXT`).
- **End-of-match awards** [D]: the results screen shows kills and points and hands out joke awards. They are Friendly Fire, Self Preservation, Uber Ninja, Crate Hog, Kamikaze, Safari, Chicken, Who Dares Wins, Demolition Expert, Panzer General and Biggles (`#9`, "End of game summary text"). The battle-stats screen tracks enemies killed, NPCs wasted, own forces lost, enemy bases destroyed and "wanton destruction" (`#763 › WINDOWDESCRIPTIONS_ENG.TXT`).
- **Skirmish teams:** up to 4 teams, in any mix of human and AI. AI teams fight each other rather than ganging up on the human ([F] FAQ §6.4). Giant Bomb says there is no co-op mode ([S]).
- **Modes and their rules:**

  | Mode | How it is unlocked ([F]) | Rules | Conflicts |
  |---|---|---|---|
  | **Deathmatch** | Available from the start | Last team standing; you choose the crate set ([S] Giant Bomb, Wikipedia; [F]) | — |
  | **Domination** | That's Rubbish | Touch objects to recolour them; hold them over time to score; dead teams respawn ([F]; [S] Wikipedia) | **Resolved** [D]: 1 point each time a point stays your colour for 5 seconds, so more points held means faster scoring. Wikipedia was right and Giant Bomb wrong. No team can be wiped out before time runs out (`#442 › 036_TEXT.TXT`; `#415 › 009_TEXT.TXT`; tutorial `#3` offset 16951). Two asymmetric variants exist: one side keeps bins upright while the other tips them over, and one side lights lanterns while the other puts them out (`#441 › 035_TEXT.TXT`, `#443 › 037_TEXT.TXT`) |
  | **Capture** | On Yer Bike | Stun an enemy's pen animal (dog, pig, penguin or sheep) and carry it to your pen. Respawns are on ([F]; [S] Wikipedia; [P] *GameRevolution*: "stun stray dogs") | **Resolved** [D]: 10 points for dropping an enemy animal in your pen, 15 if your own team's animal is also in the pen. You carry a stunned animal with the crate-pickup button (`#439 › 033_TEXT.TXT`). FAQ and Wikipedia are **verified** |
  | **Bomb-Ball** | The Rong Brothers | Teams of 3. Carry a bomb and throw it between the enemy goalposts; it explodes after a fuse. No building, but weapons can be found ([F]; [S]) | **Resolved** [D]: throwing the ball lights a **3-second** fuse. 1 point for throwing it between the posts, plus a **5-point** bonus if it then explodes in the opponent's goal area (`#440 › 034_TEXT.TXT`). The FAQ's **1 + 5** is **verified**; Wikipedia (1 + 6) and Giant Bomb (5 + 7) are **wrong**. Team size of 3 is still [F] |
  | **Ghost** (a Deathmatch option) | Bossa Nova | A wiped-out team's last Buddy returns as an invisible, floating ghost. Touching fresh "smoking boots" revives it ([F]; [S] Wikipedia; [P] Baker: "scare the smug gits with a Ghost Buddie") | [S] PSX Data Center counts it as a fifth mode. The FAQ says it is an option, not a mode |

  Baker also mentions "Capture the Pig" ([P]), which is presumably an early name for Capture.

### Difficulty

- **No difficulty setting found.** None of the sources mentions difficulty levels. Treat their existence as unverified.
- **How hard it played:** sources disagree.
  - Very hard:
    - *GameFan*: "this game is balls hard ... 'Piece of cake' (first mission) to 'Whoa Nelly!' (second mission)" ([P] [GameFan p.98](https://archive.org/details/Gamefan_Vol_8_Issue_11/page/n101/mode/2up))
    - *GameSpot*: "too uninteresting and difficult" ([P])
    - Whittaker: the AI regularly beats humans ([P])
  - Too accessible for some:
    - *EGM* (Ryan): "it gets a little too silly at times; it's easy to see this game was made for strategy gamers of all ages, which kind of alienates hardcore fans" ([P] [EGM #136](https://retrocdn.net/images/c/c0/EGM_US_136.pdf)). This is about tone and audience, not a difficulty complaint
- **Cheats:** the game has many cheat codes, including "Easy Gun Building", "Easy Buddie Building", "All Crates Build Like Super Crates" and "Finish Build Shape" ([F] FAQ §5.2). **Verified** [D]: all four names are among the 22 cheat-activation messages. Others include Level Win, All Levels Open, Invulnerable Base, Extra Time, Free Jetpack, Big Feet and Big Guns. The pause menu also has a Cheats page with health and ammo toggles, an exploration mode, "finish current (super) build shape", and options to terminate a Buddy or make it invincible (`BUDDIES.DAT #9`, "Cheats pause menu" and "Cheat activation messages" groups).
- **No difficulty setting:** **still none found** [D]. The menu string table has no difficulty option. There is also a hidden, half-finished cheat menu unlocked with a Konami-code variant ([S] [32bits](https://32bits.substack.com/p/under-the-microscope-team-buddies)). The cheat names show which parts of the build loop the developers treated as adjustable.

---

## 4. Units and items

### Buddy classes (7)

The class roster is confirmed by the press kit's character interviews: Infuntry, Commando, Medic, Ninja, Cyborg, Super and Stealth ([P] [300500TB](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/300500TB.RTF)). Baker also says there are "7 different Buddie types" ([P]). The 1999 materials list a **Karate** Buddy, while the 2000 materials list a **Ninja** Buddy ([P] [99tbrel](https://discmaster.textfiles.com/file/18679/Sony%20PlayStation%20autumn-winter%20collection%2099%20-%20press%20cd%203.iso/buddies/text/99tbrel.rtf/99tbrel.pdf) vs [teamb-pressrel](https://discmaster.textfiles.com/file/18770/PSone_Presskit_ECTS2000.iso/software/team%20buddies/text/teamb-pressrel.rtf/teamb-pressrel.pdf)). This looks like a rename during development (inference). PSX Data Center lists both Karate and Ninja ([S]).

**[D] the data has eight classes, and one is unused.** The toy table and the Buddy stat table define Infuntry, Stealthy, Commando, **Kung Fu**, Ninja, Cyborg, Medic and Super, each with a separate "Super ___" upgraded class (`BUDDIES.DAT #10` toys 28–35 and 104–111; `#452 › BUDDIES.BIN`). Kung Fu and Super Kung Fu appear in **none** of the 68 crate sets, so the player can never build them. The 1999 press "Karate" Buddy is therefore most likely Kung Fu, a class that was cut but left in the data, with the Ninja taking its role. This **revises** the earlier "rename" inference.

All rows below are [F] FAQ §6.1 unless noted. The world briefings that introduce each class ([D] `BUDDIES.DAT #444`–`#448`, `W2`–`W6_TEXT.TXT`) now confirm most rows.

| Class | Available from | Melee | Abilities / role | Notes and conflicts |
|---|---|---|---|---|
| **Infuntry** | World 1 | Weak punch | Baseline; cheap; cannon fodder | Spelled "INFUNTRY" in the press kit ([P]) and in the game ([D]) |
| **Commando** | World 1 | Strong uppercut | Faster, tougher and smarter; "super-strafe" (circle-strafes without slowing) | [P] Baker: "gung-ho commando Buddie". **Verified** [D]: the tutorial calls Commandos veteran Infuntry who are stronger and faster, says they keep full speed while strafe-locked because they are better shots, and gives them an uppercut (`#3`, offsets 14311, 14426) |
| **Medic** | World 2 | Bicycle pump | Regenerates own HP; can see other Buddies' health bars | **Resolved: the FAQ was wrong.** [D] the World 2 briefing says Medics pump wounded Buddies back up to full health. Holding attack while touching a friend heals them; doing it to an enemy has an odd, comic effect. Medics also heal themselves and can see others' damage (`#444 › W2_TEXT.TXT`, offset 474). This **verifies** *GameRevolution* and Giant Bomb. A mission tip adds that the Medic stuns animals faster than other troops (`#412 › 006_TEXT.TXT`) |
| **Ninja** | World 3 | Katana | Very fast, jumps high; **splits merged crates** on any pad, so is the anti-economy unit; cuts down trees | [P] Baker: "ninja Buddie with his sword". **Verified** [D]: the briefing stresses high-speed running and jumping and says the katana cuts through joined crates, marking the Ninja as the sabotage choice (`#445 › W3_TEXT.TXT`, offset 996) |
| **Cyborg** | World 4 | Electric punch (strong stun) | Heavy armour, resists fire; cannot be crushed by vehicles; can stomp Buddies; can see vehicle HP. Slow, jumps poorly | Anti-vehicle and base defence. **Verified** [D] except vehicle-HP sight: very high damage tolerance, especially against fire; tanks cannot flatten him; an electric punch against vehicles; jumps on Buddies to squash them (`#446 › W4_TEXT.TXT`, offset 413) |
| **Stealthy** | World 5 | Steals weapons and ammo | Invisibility, broken by firing, being shot or entering water. Best scout | Press kit name: "Stealth". **Verified** [D]: invisible until it fires a weapon; holding attack while touching an enemy steals its weapon (`#447 › W5_TEXT.TXT`, offset 407). That the stealth effect fails over water is hinted in a briefing tip (`#422 › 016_TEXT.TXT`). Ammo theft is still [F] |
| **Super** | World 6 | Spin punch | Glides; the super-crate version flies. Strongest armour; laser eyes with unlimited ammo; sees invisible units. **Cannot use weapons or vehicles** | [P] Baker: "SuperBuddie with his laser eyes". **Verified** [D]: tougher even than the Cyborg; laser eyes; holding jump gives a glide; the upgraded Super can fly; refuses all weapons and vehicles (`#448 › W6_TEXT.TXT`, offset 791). Its stat record names a built-in weapon, entry 37, which is "Laser Eyes" in the toy table [D·inf] |

A super crate makes any class stronger and faster ([F]; [S] Wikipedia). **Verified, more precisely** [D]: a super crate builds a separate "Super ___" class, and its stat record has higher values in the fields that look like health and speed (table below).

**Which classes come from which stack: resolved** [D]. See the per-world recipe table above. The 2-vertical slot gives the world's basic Buddy (Infuntry → Commando → Cyborg as the campaign goes on), and the 4-vertical slot gives the specialist.

#### Buddy stats

The Buddy table has 62 records of 128 bytes (`BUDDIES.DAT #452 › BUDDIES.BIN`). It covers every character in the game, including animals, civilians, bosses and Baddies; record *n* matches the *n*-th entry of the skeleton list in `#500`. The toy table gives the record number for each buildable class. **No field names exist on the disc, and the decomp has not yet named the code that reads this table.** The meanings below are therefore **[D·inf]**: I chose them because each fits every textual claim about the classes. The raw values are exact.

| Class (record) | +0x04 "max health" normal → super | +0x44 "move speed" normal → super | +0x08 "jump impulse" (more negative = higher) | Other notable fields |
|---|---|---|---|---|
| Infuntry (1 → 42) | 80 → 110 | 102 → 115 | −118 | — |
| Commando (0 → 36) | 120 → 160 | 110 → 115 | −118 | +0x18 = 6.0 in 4.12 fixed point, against 1.0 for Infuntry: a strong-melee candidate, matching the "painful uppercut" |
| Medic (3 → 41) | 85 → 120 | 102 → 110 | −118 | — |
| Ninja (33 → 37) | 120 → 150 | 128 → 140 | −150 | Highest jump, with Super |
| Cyborg (31 → 38) | 200 → 250 | 71 → 89 | −100 | Slowest and lowest jump (FAQ: "slow, jumps poorly"). +0x74 = 500 against 50 for most classes, possibly the fire resistance |
| Stealthy (4 → 40) | 100 → 130 | 89 → 102 | −118 | The only class with +0x50 = 1, likely the stealth flag |
| Super (5 → 39) | 200 → 250 | 128 → 128 | −150 | +0x54 = weapon 37 (Laser Eyes). +0x64 = 15000; for Super Super Buddie it is 0x8000 (32768), far above every other class (7280–9100), which fits "the upgraded Super can fly" |
| Kung Fu (2 → 43), unused | 125 | 140 | −140 | — |

Why these readings:

- **+0x04 as health:** Infuntry is lowest. Commando, called "stronger", is higher. Cyborg and Super are highest. Every super class is higher than its base class.
- **+0x44 as move speed:** Commando beats Infuntry, called "faster". Ninja is high, called "very fast". Cyborg is lowest, called "slow". Every super class is at least as fast as its base class.
- **+0x08 as jump impulse:** Ninja and Super have the strongest values ("jumps high", "glides"). Cyborg has the weakest ("jumps poorly").
- **Caveat:** the briefing says Super withstands more than the Cyborg, but their +0x04 values are equal (200/250). So either armour lives in another field, or +0x04 is not the whole story.
- **Linked records:** +0x7C of each base record points to its super record, and +0x7E points back. For example, Infuntry's +0x7C is 42, Super Infuntry; this link is exact [D].

### Weapons by stack tier

The FAQ does not give a per-world table. The table below was **Giant Bomb's list ([S])**. It is now **verified, with corrections** [D] against the world recipe sets (see [the per-world recipe table](#stack-shape-to-output-category)). Giant Bomb's order is the world order, Worlds 1–8. Its extra "Ankh" and "Armageddon Barrel" entries come only from the secret *Special* multiplayer set, not from any world.

| Tier | Items listed by Giant Bomb | [D] super-crate version, in world order |
|---|---|---|
| Light (1 crate) | Uzi 9mm, Shotgun, Pocket Rocket, Fireball, Ice Gun, Tazar, Pocket Missile, Laser Rifle | Uzi 12mm, Shotgun Upgrade, Super Pocket Rocket, Thunderball, Liquid Nitro Gun, Tazar Upgrade, Pocket Missile Upgrade, Laser Cannon |
| Grenades (2 flat) | Grenades, Time Bomb, Napalm Grenade, Cluster Grenade, Ice Grenade, Scatter Bomb, Lightning Grenade, Dam Buster, Ankh | Super Grenades, Time Bomb Upgrade, Super Napalm, Super Cluster, Super Ice, Super Scatter Bomb, Super Lightning Grenade, Super Dam Buster |
| Heavy (4 flat) | Bazooka, Gatling Gun, Flamethrower, 4-Pack Rocket, Power Orb Gun, 4-Pack Missile, Lightning Gun, Pulse Rifle, Armageddon Barrel | Supazooka, Gatling Cannon, Flamejet, Super 4-Pack Rocket, Multi Orb Gun, Super 4-Pack Missile, Krypto Light Gun, Pulse Cannon |

**[D] weapons that exist but are never built.** The weapon table holds 89 entries (`#452 › WEAPONS.BIN`, 100 bytes each), and 59 of them are named toys. The toy list names a Pistol, Blunderbuss, Longbow, Super Shotgun, Paint Gun, Mine Machine, Snowball and plain Laser. Except for Laser, which is in the *Special* and *Space* sets, none of these is in any crate set. They are cut content, or weapons given out by scripts (inference).

**Weapon stats [D·inf].** The meanings are inferred as for Buddies; the values are exact.

| Weapon | +0x08 "ammo per shot" | +0x50 "projectiles per shot" | +0x1C "shots per burst"? |
|---|---|---|---|
| Uzi 9mm / Uzi 12mm | 1 / 1 | 1 | 25 / 30 |
| Shotgun / Shotgun Upgrade | 3 / 3 | **2 / 4** | 1 |
| Grenades / Super Grenades | 8 / 10 | 1 | 1 |
| Bazooka / Supazooka | 5 / 5 | 1 | 1 / 2 |
| Gatling Gun / Gatling Cannon | 1 / 1 | 1 | 90 |
| Flame Thrower / Flamejet | 3 / 5 | 1 | 10 |
| Time Bomb | 8 | 1 | 2 |
| Laser Rifle | 4 | 1 | 8 |
| Pulse Rifle | 1 | 1 | 90 |
| Armageddon Barrel | **500** | 1 | 1 |

Why these readings:

- **+0x08 as ammo per shot:** ammo is a single per-Buddy pool capped at 500. The Armageddon Barrel's 500 means one shot empties a full pool, which fits a doomsday weapon. Rapid-fire guns cost 1, and grenades cost 8–10.
- **+0x50 as projectiles per shot:** the Shotgun Upgrade doubles it from 2 to 4.
- **Not decoded:** damage per hit is not settled. It may sit in the projectile table (`#452 › PROJECTILES.BIN`, 82 × 112 bytes), which the weapons point into. I did not decode it.

Primary and FAQ sources confirm these names:

- **[P] Baker:** Uzi, the "mega-destructive Armageddon Gun", Rocket Packs and Laser weapons.
- **[P] Developer sheet:** flamethrower.
- **[P] *GameFan*:** shotguns, bazookas, bombs and gatling guns.
- **[F] FAQ:** Bazooka and Gatling Gun as World 1 heavy weapons; Gatling Cannon, Supazooka, Multi Orb Gun, 4-Pack Rocket, Krypto Light Gun, Plasma Cannon, Laser Rifle and Time Bomb.
- **[F] FAQ §6.5:** World 5's set uses freezing. Hits on a semi-frozen target do double damage, and a hit on a fully frozen target kills it.

### Vehicles

- **[P] Baker:** tank, stealth tank and bomber. The press kit art includes a mech walker ([MECHWALK.TIF](https://archive.org/details/Team-Buddies_Press-Kit)) and a screenshot named "SNOWMOBI".
- **[F] FAQ:** Motorbike, Copter Gunship, Harrier, Stealth Harrier, Medium Ice Tank, Tyrant Tank, Killer Tank, Flamejet, Flying Saucer, Stealth Bomber, mech, and tractor and bike as mission objects. Also sentry turrets you can climb into.
- **[S] Giant Bomb:** Tank, Helicopter, Aircraft, Bikes, Jeep, Mechwalker and UFO. Wikipedia adds a golf cart.
- **How you get them:** build them from a full 8-crate cube, or find them in megacrates. Megacrates can be towed only by a Mech ([S] Wikipedia).
- **[D] the full roster.** The toy list has 81 vehicle entries (category 1 in `#452 › TOYS.BIN`), which point into an 84-record vehicle table. They include 28 named base vehicles: Bike, Motorbike, Baby/Medium/Super/Tyrant/Killer tanks and their Gatling, Tandem, Stealth, Ice, Hover, Rocket and Flame variants, Jeep, Microlite, Plane, Stealth Bomber, Flying Saucer, Laser Bike, MRLS, Rocket, Snowmobile, Snowbike, Frost Saucer, Mech Walker and Moon Buggy. They also include 16 turrets (manned and automatic cannon, gatling, homing, flame, ice, laser, four-pack and deathray), mission props (Golf Cart, Tractor, Bus) and boss machines (the sheep-sucking boss tank, the Mech Walker upgrade). This **verifies** the FAQ and Giant Bomb names and adds many variants. Only the vehicles listed in the recipe table above, plus those in the secret sets (Microlite, Plane, Copter Gunship, Harrier, Stealth Harrier, Stealth Bomber, Bike, Motorbike, Flying Saucer), can be built.
- **Vehicle durability [D·inf].** The vehicle table has 84 records of 176 bytes (`#452 › VEHICLES.BIN`). Field +0x38 rises steadily with vehicle tier, so it is most likely hit points: Microlite 60, Bike 65, Motorbike 90, Golf Cart 100, Baby Tank 140, Plane and Copter 150, Baby Tandem Tank 175, Medium Tank 200, Harrier and Stealth Bomber 250, Tyrant Tank 300, Killer Tank 350, Flying Saucer 400, Super Tank 425, Mech Walker 600, Mech Walker Upgrade 800. Field +0x00 groups the vehicles into what look like movement types (tanks, wheeled, bikes, mech, planes, saucer, helicopter). Field +0x04 points to the vehicle's built-in weapon. That mounted weapons use no ammo is still [F].
- **Crate capacity: still unverified.** Field +0xA8 holds 0–3 (bikes and tanks 2; planes, saucer, mech and golf cart 3; copters 1). That fits Wikipedia's "2, larger ones 3", but I could not confirm that this field is the crate limit.

---

## 5. Design intent, tone and reception

### Origin and influences

"Team Buddies began as a Tetris-style puzzle game involving crates and soldiers. This gradually evolved into an all-out combat game with crate stacking as the central game mechanic." The art style "pays homage to the classics, such as BomberMan, Cannon Fodder, Tetris and Gauntlet" ([P] [Baker Q&A](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/261099Q%26.RTF)). The crate-stacking pad is what remains of that puzzle prototype.

### Genre positioning

- **Developers:**
  - Asked whether it is a strategy game, Baker said "Not really ... explosive arcade style action and frenzied battles, but there are also elements of strategic and tactical gameplay" ([P]).
  - The pitch line was "Basically the game is built on loads of excitement and lots of boom" ([P]).
  - Whittaker contrasts the game with Quake, where interaction is only "through the barrel of a gun". Here Buddies "have a resource to collect, they've got to do something intelligent with that resource to gain weapons, and ... communicate to form a cohesive strategy" ([P]).
- **Reviewers:** the reviews compare it to:
  - *EGM*: "a simple version of Command and Conquer" and "Bomberman-style action"
  - *CVG*: "Hogs of War, Worms"
  - *GameFan*: "Midway's true Gauntlet"
  - *GameRevolution*: like C&C skirmish

  Sources: [P] [EGM](https://retrocdn.net/images/c/c0/EGM_US_136.pdf), [CVG](https://retrocdn.net/images/8/8f/CVG_UK_227.pdf), [GameFan](https://archive.org/details/Gamefan_Vol_8_Issue_11/page/n25/mode/2up), *GameRevolution*.

### Tone

- **Contrast as a selling point:** cute, inflatable, bean-shaped Buddies against extreme violence and swearing. "The shiny, happy graphics contrast wildly with the unfolding carnage" ([P] 1999 release). The rating was 15+ in Europe ([P] ECTS release).
- **Voices:** each class has a British comedic stereotype voice ([P] [vox pop](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/300500TB.RTF)). In Europe each team colour has a national stereotype; the North American release toned the dialogue down for an E rating ([S] Wikipedia).
- **Story framing:** the crates are a TV show. Baddies on the moon drop crates and stacking pads onto Buddie World to make the Buddies fight for their ratings, and "Every so often they add new, bigger, more vicious weapons", which explains the escalating weapon sets per world in-fiction ([P] [150300ST](https://archive.org/download/Team-Buddies_Press-Kit/TEAMBUDD/TEXT/150300ST.RTF)).

### Reception

Scores ranged widely:

| Outlet | Score |
|---|---|
| GameRankings | 72% |
| IGN | 8.5/10 |
| *GameRevolution* | B+ (8/10) |
| *CVG* | 4/5 |
| *EGM* | 7/10 from each of three critics (per Wikipedia; the review's category scores are Visuals 7, Sound 8, Ingenuity 8, Replay 8) |
| *GameSpot* | 4.8/10 |

Sources: [S] [Wikipedia](https://en.wikipedia.org/wiki/Team_Buddies) review table; the *GameSpot* score and *EGM*'s category scores were checked directly; the rest are as Wikipedia reports them.

**What reviewers said worked:**

- **Balancing building against fighting.** *CVG*: "you've got to balance building with going out and killing things. The pace is frenetic, and the variety of missions ... is great" ([P]).
- **Four-player chaos and sabotage.** *CVG*: "Things rock when four of you each take a team and try and kill one another ... playing fast and being easy to pick up" ([P]). *EGM*: stealing and breaking a rival's boxes was "incredibly funny" ([P]).
- **Depth hidden under the cute look.** *GameFan* (Eggo): "the sheer amount of strategy and thought that must go into each mission is staggering"; GN: "a lot of depth" ([P]). IGN called it "a thinking man's shooter" ([S] via Wikipedia).
- **Campaign unlocks feeding multiplayer.** *EGM*, *GameRevolution* and *CVG*: "progression in single player opens up new battlefields and game modes" ([P]).
- **Presentation.** Every review praises the audio, visuals and humour.

**What reviewers said failed:**

- **Controls and command overload.** *GameSpot*: "Keeping track of four Buddies while performing three or more maneuvers at once is painfully confusing" ([P]). *EGM*'s worst feature was "Sometimes confusing interface"; it is "occasionally difficult to place the blocks in the right spots; trying to figure out what exactly you're building can also be a challenge" ([P]). *GameFan* ECM: "requires a PhD in mechanical engineering to come to grips" ([P]).
- **Stacking felt like a chore to some.** *GameSpot*: "Block stacking is too time consuming", and the game repeats itself because there is "barely sufficient variation in troops" ([P]).
- **Ally AI.** Covered under squad commands above: "useless" (*GameSpot*) and prone to friendly fire (*GameFan*).
- **Camera.** *GameFan*: scenery doesn't turn transparent ([P]). *EGM*: "odd camera perspectives" ([P]).
- **Difficulty spikes.** Covered under difficulty above (*GameFan*).
- **Thin multiplayer for some.** *GameFan* ECM: multiplayer "devolves into mindless carnage" ([P]). *CVG* and *EGM* felt the opposite.

---

## 6. Takeaways for a new game

> **SYNTHESIS, NOT SOURCED.** The rest of this section is my interpretation of the evidence above. It is design opinion, not fact about Team Buddies. It was revised after the disc data came in. The main changes are that the shape grammar is now known exactly, that super crates choose an *alternative* item rather than boosting stats, and that each world adds one new class through a fixed slot.

### The loops as a diagram

```
             ┌──────────────── MATCH LOOP (≈5–10 min) ────────────────┐
             │                                                         │
 drop zones ─┼─► GATHER ──► HAUL ──► STACK on 2×2×2 pad ──► OPEN ──►    │
 (contested; │   crate     carry/    shape = slot 0..5        kick     │
  events can │             jump-     (flat = weapon,                   │
  open more) │             throw/     upright = Buddy,                 │
             │             vehicle    cube = vehicle)                  │
             │                        +1 blue crate = slot's alt toy   │
             │                                     ▼                   │
             │                         OUTPUT: Buddy | weapon | vehicle│
             │   off-pad kick ─► health OR ammo (ammo = per-Buddy pool)│
             │                                     ▼                   │
             │      COMMAND squad (escort / defend / attack / build)   │
             │                                     ▼                   │
             │   FIGHT ◄──► RAID enemy pad (steal / smash / split) ◄──┐│
             │      │                                                 ││
             │      └─► DECAPITATE: base flag hits 0 ⇒ pad deactivated┘│
             │                                                         │
             └── win: wipe out / objective / points or timer ──────────┘
                                   ▼
 META LOOP:  32 missions = 8 worlds × 4 (gold = all sub-objectives)
             ─► unlock crate set / mode / map / skin
             ─► next world: same 6 slots, escalated items, +1 new class
```

### Essential, the core that makes it Team Buddies

1. **One physical resource that you carry with your body.** Crates are not a number in the HUD. You carry them, and while carrying you cannot shoot, so gathering is risky and can be intercepted. Throwing requires a jump first, which adds a small skill test to fast hauling.
2. **Spatial crafting with a two-rule grammar.** The game teaches the whole system in two sentences: lay crates *flat* for weapons, stack them *upright* for Buddies. Add the full cube for vehicles and the single crate for a sidearm. Size picks the tier inside each family (2 versus 4 crates). Six slots are enough for a whole game's worth of variety, and opponents can read your intent from the pad's shape.
3. **Vulnerable work-in-progress.** Single crates can be stolen; merging locks them in but costs time. One specialist (the Ninja) can still cut joined crates. This gives you choices about tempo and risk, and a sabotage verb, which reviewers found the funniest part. Finished toys left unattended get claimed by teammates, so even your own pad needs watching.
4. **Production can be destroyed.** Destroying the base deactivates the pad, so the decisive move is economic decapitation, not just killing units. The base's health flag is visible to everyone, so pressure on it is readable.
5. **The squad is the army.** At most 4 units, and you can possess any of them. RTS ideas of production, defence and raids fit into a third-person action game you can play with a gamepad.
6. **Raw crates are also consumables, and ammo is one pool.** Kicking a crate open off-pad gives health or ammo. Ammo belongs to the Buddy and weapons spend it at very different rates (the inferred costs run from 1 per bullet to 500 per Armageddon shot). So every crate is a choice between spending now and investing later, and a stronger gun is also a hungrier one.

### Strong but separable

- **A fixed slot table per world or set, as data.** Each world is just 6 slots × (normal, super) = 12 toy IDs, and each mission can override the set (41 per-level sets, 27 multiplayer sets). This is a very cheap way to get variety and campaign progression without new mechanics, it doubles as multiplayer loadout selection, and it is easy to tune.
- **Teach a new class per world through one slot.** The 4-upright slot is where each world introduces its newcomer (Medic, Ninja, Cyborg, Stealthy, Super). Meanwhile the 2-upright "basic" Buddy is quietly promoted over the campaign (Infuntry → Commando → Cyborg). That is a clean onboarding curve: one new piece per world, while the floor rises.
- **Super crates as a branch, not a multiplier.** One rare blue crate turns the build into a *different* toy chosen per slot (for example Plane → Copter Gunship, Rocket Tank → Tandem Tank, or a separate Super class). This is more interesting than "+20%", and it makes rare crates worth fighting for.
- **Event megacrates and event-activated drop zones** as map-control objectives.
- **Asymmetric classes, each answering an economy or combat role:** anti-vehicle (Cyborg), anti-economy (Ninja), scout or thief (Stealthy), **healer** (Medic: now verified to heal allies to full), and a hero who can't use weapons (Super). The cut Kung Fu class shows the designers trimmed to one class per role.
- **Data-driven ally and enemy AI** (percept, attitude and action tables shipped as data) with readable strategies (ambush, theft, joyride). The aggression ramp is still only a developer claim.

### Incidental, and fine to drop or change

- The PS1 control mapping (three presets plus custom), binocular targeting and Multitap split-screen.
- The British swearing and the Baddie-TV-show framing. The tone of contrast is useful, but the specifics are not.
- Specific weapon names, worlds and animal NPCs.
- Ghost mode and Bomb-Ball. They are fun modes, but not core. Their scoring is now known (Bomb-Ball: 1 for the throw plus a 5-point bonus for the explosion), if a simple "goal plus bonus" model is wanted.

### Problems a new game should design around (from the reception)

- **Make stack outcomes legible before you commit.** Show a ghost preview and a recipe overlay. *EGM* said it was hard to see what you were building, though the original did show an item preview. The game's own flat/upright rule is a good basis for that overlay; show the super branch too.
- **Make squad orders context-sensitive with one button,** and reduce simultaneous inputs. "Painfully confusing" was the main complaint.
- **Keep stacking short,** or delegate it to the AI by default, so it does not become chore time ("too time consuming").
- **Give the ally AI visible intent and a sensible default.** The tutorial says teammates do nothing unless ordered. That may explain the "useless teammates" complaint (inference). A new game should give idle allies a useful default job, such as hauling crates or guarding the pad. Avoid friendly-fire deaths that feel unfair.
- **Smooth the campaign difficulty curve.** It spiked hard in early missions.

---

## Sources: decompilation & game data

Added 2026-10-18. Only facts, numbers and short descriptions in my own words are recorded here. No disc files, extracted assets or decomp source are in this repo.

### Decompilation

- **Project:** [hcbt/tbuddies-decomp](https://github.com/hcbt/tbuddies-decomp), a matching decompilation of the North American release (SLUS-008.69).
- **Commit used:** `d2ca1dce934263d553cfd8f835c81438c0df906f` (2026-09-15). All `asm/…` paths above are at this commit.
- **State at that commit:** the executable and seven overlays (ENG, GAME, MNU, MPLR, ROT, SYS, TUTO) are split into per-function assembly with 191 functions matched to C and about 2,620 still in assembly. Only one symbol is named (`CdInit`); every other function is `func_<address>`. I therefore used the decomp to confirm *how the data is loaded*, not to name every table field:
  - `asm/eng/nonmatchings/eng/func_80080D70.s`: opens disc entry 0x1C4 (#452) as a bundle and dispatches `TOYS.BIN`, `POWERUP.BIN`, `PERCEPT.BIN`, `ATTITUDE.BIN`, `ACTION.BIN` and `STATICS.BIN` to their parsers.
  - `asm/eng/nonmatchings/eng/func_80071AB8.s`: parses `TOYS.BIN`, reading a count and then 12-byte records (category, index, a 16-bit value).
  - `asm/eng/nonmatchings/eng/func_80071DBC.s`: loads the toy-name text (disc entry 10) and pairs name *n* with toy record *n*.
  - `asm/eng/nonmatchings/eng/func_8008102C.s`: parses `POWERUP.BIN`.
  - `asm/sys/nonmatchings/sys/func_8003C2AC.s`: looks up a bundle member by name (40-byte directory entries).
- **Version match:** the SHA-1 of every binary on the disc used here matches the hash the decomp verifies against (`psxdecomp.toml`), so the code and the data come from the same build.

### Game data

- **Disc:** Team Buddies (USA), serial SLUS-00869, the user's own image (`Team Buddies (USA).zip`, two-track BIN/CUE). It was read as untrusted data in a scratch directory; nothing on it was executed.
- **Main archive:** `BUDDIES.DAT` (79.9 MB). It starts with "BD", then a table of (start sector, byte size) pairs for 767 entries. I cite entries as **`BUDDIES.DAT #n`**, the 0-based position in that table. Many entries are "BIND" bundles: a member count, then 40-byte entries of name, offset and size. I cite a bundle member as **`#n › NAME`**.
- **Entries cited:**

  | Entry | Start sector in BUDDIES.DAT | What it is |
  |---|---|---|
  | #3 | 80 | VR-training tutorial text (cited by byte offset within the entry) |
  | #4 | 91 | Tutorial lesson script, plain text, lessons 0–8 |
  | #9 | 108 | Menu and UI string table (control types, battle options, crate-set names, skins, cheats, alerts) |
  | #10 | 112 | Toy-name list, 180 names, in toy-number order |
  | #407–#438 | 31538 onward | Mission briefings `001`–`032_TEXT.TXT`, one bundle per mission |
  | #439–#443 | 32462 onward | Multiplayer-mode rule screens `033`–`037_TEXT.TXT` (Capture, Bomb-Ball, bin Domination, Domination, lantern Domination) |
  | #444–#451 | 32557 onward | World briefings `W2`–`W8`, `W1_TEXT.TXT` |
  | #452 | 32893 | Gameplay data bundle: `TOYS`, `POWERUP`, `PROJECTILES`, `WEAPONS`, `VEHICLES`, `ACTION`, `PERCEPT`, `ATTITUDE`, `BUDDIES`, `STATICS`, plus 41 `n_CRATECONTENTS.BIN` and 27 `n_BT_*.BIN` recipe sets |
  | #453 | 32951 | Second data bundle, including `LEVELS.BIN` (48 level records of 128 bytes) |
  | #500 | 36588 | Character skeleton bundle (62 `ST*.BIN`, in the same order as `BUDDIES.BIN`) |
  | #759–#763 | 38848 onward | Level names, results messages, tutorial menu, unlock messages, stats-screen labels (English and four other languages) |

- **Table formats confirmed:**
  - **Recipe set:** u32 count = 6, then 6 × (u16 normal toy, u16 super toy).
  - **TOYS.BIN:** u32 count = 180, then 180 × (u32 category, u32 index, u16 value). Category 0 = power-up, 1 = vehicle, 2 = weapon, 3 = Buddy; the index points into that category's table.
  - **BUDDIES.BIN:** 62 × 128 B. **WEAPONS.BIN:** 89 × 100 B. **VEHICLES.BIN:** 84 × 176 B. **PROJECTILES.BIN:** 82 × 112 B. Each has a u32 count first.
- **SYS.BIN** lists 32 mission splash screens (`SPLASH\SP_<initials>_NTSC.TIM`). The initials follow the level-name list.

---

## Open questions

1. **Manual scan** (PAL SCES-01923 or NTSC-U SLUS-00869). **Mostly closed** by the in-game tutorial and briefings. Still useful for the exact default button layout.
2. **Exact recipe per world:** **closed** [D]. See the per-world recipe table. Still open: the exact geometry of the 4-upright shape (assumed 2 wide × 2 high), and which per-level set belongs to which mission (inferred from name order).
3. **Mode scoring:** **closed** [D] for Bomb-Ball (1 + 5, 3-second fuse), Capture (10 / 15) and Domination (1 point per 5 s held). Still open: the Bomb-Ball team size.
4. **64 versus 32 missions:** **closed** [D]: 32 story missions (8 × 4) plus a 9-lesson tutorial and about 21 multiplayer maps.
5. **Unit stats** (HP, speed, damage): **partly closed.** Exact values are recorded, but field meanings are [D·inf]. Next step: trace the code that reads `BUDDIES.BIN` +0x04/+0x44 and `WEAPONS.BIN` +0x08 in the decomp (GAME.BIN/ENG.BIN), or use an emulator memory watch. Damage per hit (probably in `PROJECTILES.BIN`) is not decoded.
6. **Medic healing on allies:** **closed** [D]: Medics heal allies to full health. The FAQ was wrong.
7. **Controls:** *GameSpot* says X shoots; the FAQ says Square attacks. **Probably explained** by the three control presets; the default mapping is still unverified.
8. **AI aggression ramp:** the press kit's "aggression starts at zero and rises" claim is **still unverified**. The attitude table (`#452 › ATTITUDE.BIN`, 184 entries) has no field names.
9. **Crate rules still from the FAQ:** whether merged crates can be stolen, what the Ninja's cut leaves behind, vehicle crate capacity, crate drop timing (`LEVELS.BIN` has candidate fields that were not confirmed), and per-mission timers.
10. **Developer TV interview:** the British TV show "Cheats" segment ([archive.org](https://archive.org/details/Cheats_Team-Buddies)) may contain developer commentary. Not reviewed.
