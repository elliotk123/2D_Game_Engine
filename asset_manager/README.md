# Asset Manifest Format (v1)

## Overview

manifest.toml is the root file.

It may contain asset definitions inline and/or reference external TOML files that contain asset definitions.

Identity for an asset is by the key field (string). Keys must be unique (rules below).

## Root manifest.toml
version: int (required)
asset_definition_files: [string] (optional)
asset_definitions: table (optional)

### Fields
version: int (required)
asset_definition_files: [string] (optional)
asset_definitions: table (optional)

### Rules
You may use inline only, external only, or both.
External files are loaded in order, then inline definitions are merged last.
Duplicate keys (per asset type) are errors by default.

### Template
```
# manifest.toml
version = 1

# Optional list of TOML files containing [asset_definitions]
asset_definition_files = [
  # "assets/sprites.toml",
  # "assets/player.toml",
  # "assets/enemies.toml",
  # "assets/audio.toml",
]

# Optional inline asset definitions (merged last)
# [asset_definitions]
# ...
```

[asset_definitions] contract
[asset_definitions] may appear:

inline in manifest.toml, and/or
inside referenced definition files (e.g. assets/player.toml)

#### Supported sections
[[asset_definitions.sprites]]\
[[asset_definitions.spritesheets]]\
nested: [[asset_definitions.spritesheets.sprites]]\
[[asset_definitions.animations]]\
[[asset_definitions.audio]]\

## Supported Asset types
the following types are suppported both inline and externally for the asset_manifest
### Sprite
#### Fields
- **key**: string *(required, unique)*
- **file_path**: string *(required) - if defined individually; omitted if defined within **spritesheet***
- **size**: [int,int] *(required)* 
- **offset**: [int,int] *(required)*
#### Rules
- key must be a unique among all **Sprite assets**, this includes sprites defined in external files and within sprite sheets.
- file_path is an inherited value when a **Sprite** is defined within a **SpriteSheet** it can therefore be omitted in this instance.

### SpriteSheet
#### Fields
- **key**: string *(required,unique)*
- **file_path**: string *(required)*
- **quantity**: int *(required)*
- **sprites**: [Sprite] *(required)- nested under the **SpriteSheet***
#### Rules
- key must be a unique among all **SpriteSheet** assets.
- each nested **Sprite** must have a unique key among all sprite assets

### Animation
#### Fields
- **key**: string *(required,unique)*
- **framerate**: int *(required)*
- **sprite_keys**: [string] *(required) - references **Sprite** key values*
#### Rules
- key must be a unique among all **Animation** assets.
- each sprite_key must refer to an existing defined **Sprite** asset key.
    *Note: This is for simple constant framerate animations (e.g., ambient background loops). More complex animation should be handled programmatically.*

### Audio
#### Fields
- **key**: string *(required,unique)*
- **file_path**: string *(required)*
- **time_offset**: int *(required)*
- **duration**: int *(required)*
#### Rules
- key must be a unique among all **Audio** assets.
- time_offset must be greater than or equal to zero
- duration must be greater than zero

## Units & Conventions
- dimensions are to be specified in pixels
- time is to be specified in milliseconds

## Merge and uniqueness rules
### Merge order
1) Load manifest.toml
2) Load each file in asset_definition_files in order
3) Merge inline [asset_definitions] last

### Duplicate keys
**sprite**:      key *must* be unique across all **sprite**s (including those in **spritesheet**s)
**spritesheet**: key *must* be unique across all **spritesheet**s
**animation**:   key *must* be unique across all **animation**s
**audio**:       key *must* be unique across all **audio**

## Validation Rules
What causes errors.

## Examples
below are some example toml file definitions covering multiple scenarios to help with building new asset definitions

### Example 1 - Inline-only manifest( all in one file)
this example defines an animation of a flying bird with all assets defined in the root manifest.toml file

#### manifest.toml
```
# manifest.toml
version = 1

[asset_definitions]

[[asset_definitions.sprites]]
key = "bird_0"
file_path = "assets/sprites/bird_0.png"
size = [32, 32]
offset = [0, 0]

[[asset_definitions.sprites]]
key = "bird_1"
file_path = "assets/sprites/bird_1.png"
size = [32, 32]
offset = [0, 0]

[[asset_definitions.sprites]]
key = "bird_2"
file_path = "assets/sprites/bird_2.png"
size = [32, 32]
offset = [0, 0]

[[asset_definitions.sprites]]
key = "bird_3"
file_path = "assets/sprites/bird_3.png"
size = [32, 32]
offset = [0, 0]

[[asset_definitions.animations]]
key = "bird_fly"
framerate = 12
sprite_keys = ["bird_0", "bird_1", "bird_2", "bird_3"]
```

### Example 2 - External-only manifest (all assets found in referenced files)
this example defines external files for grouped assets should you wish to organise assets in this manor
#### manifest.toml
```
# manifest.toml
version = 1

asset_definition_files = [
  "assets/sprites.toml",
  "assets/player.toml",
  "assets/enemies.toml",
  "assets/ui_sheet.toml",
  "assets/audio.toml",
]
```

### Example 3 - Mixed (external + inline additions)
this example shows how to include both external and inline definitions of assets
#### manifest.toml
```
# manifest.toml
version = 1

asset_definition_files = [
  "assets/sprites.toml"
]

[asset_definitions]

[[asset_definitions.sprites]]
key = "debug_marker"
file_path = "assets/sprites/debug_marker.png"
size = [8, 8]
offset = [0, 0]
```

### Example 4 - External asset definition file
this example shows how an externally referenced file may be constructed
#### external_assets.toml
```
# assets/example_assets.toml
# This file defines asset definitions referenced by manifest.toml
# All paths are relative to the manifest.toml location (recommended)

[asset_definitions]

# --------------------
# Sprites
# --------------------

# [[asset_definitions.sprites]]
# key = "example_sprite"
# file_path = "assets/sprites/example_sprite.png"
# size = [32, 32]
# offset = [0, 0]

# --------------------
# Sprite sheets
# --------------------

# [[asset_definitions.spritesheets]]
# key = "example_sheet"
# file_path = "assets/sheets/example_sheet.png"
# quantity = 1
#
#   [[asset_definitions.spritesheets.sprites]]
#   key = "example_sheet_sprite_0"
#   size = [32, 32]
#   offset = [0, 0]

# --------------------
# Animations (simple constant framerate)
# --------------------

# [[asset_definitions.animations]]
# key = "example_animation"
# framerate = 10
# sprite_keys = ["example_sheet_sprite_0"]

# --------------------
# Audio clips
# --------------------

# [[asset_definitions.audio]]
# key = "example_audio"
# file_path = "assets/audio/example_audio.wav"
# time_offset = 0      # ms
# duration = 500      # ms
```

### Example 5 - Sprite asset definition
This example shows the template for a Sprite asset
```
[[asset_definitions.sprites]]
key = "enemy_idle"
file_path = "assets/sprites/enemy_idle.png"
size = [24, 24]
offset = [0, 0]
```

### Example 6 - SpriteSheet asset definition
This example shows the template for a SpriteSheet asset
'''
[[asset_definitions.spritesheets]]
key = "effects_sheet"
file_path = "assets/sheets/effects.png"
quantity = 3

  [[asset_definitions.spritesheets.sprites]]
  key = "sparkle_0"
  size = [16, 16]
  offset = [0, 0]

  [[asset_definitions.spritesheets.sprites]]
  key = "sparkle_1"
  size = [16, 16]
  offset = [16, 0]

  [[asset_definitions.spritesheets.sprites]]
  key = "sparkle_2"
  size = [16, 16]
  offset = [32, 0]
'''

### Example 7 - Animation asset definition
This example shows the template for an Animation asset
```
[[asset_definitions.animations]]
key = "slime_bounce"
framerate = 8
sprite_keys = ["slime_0", "slime_1", "slime_2"]
```

### Example 8 - Audio Asset definition
This example shows the template for an Audio asset
```
[[asset_definitions.audio]]
key = "slime_squish"
file_path = "assets/audio/slime_squish.wav"
time_offset = 250
duration = 400
```