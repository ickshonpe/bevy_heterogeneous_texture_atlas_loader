# Bevy Heterogenous Texture Atlas Loader

Load heterogenous texture atlases according to a RON file manifest.  
  
  

# Example

## Problem
You have a beautiful sprite sheet `example.png` for your game:

 ![/assets/example.png](/assets/example.png)

You want to load it into Bevy but the sprites are all different sizes.  
  


## Solution

### You create a `manifest.ron` manifest file 

```
(
    path: "example.png",
    rects: [
        ("yellow", 16, 64, 19, 67),
        ("face", 93, 125, 108, 139),
        ("patches", 176, 196, 34, 68),
    ]
)
```
* You can call the manifest anything you like, not only `manifest.ron`.
* The `path` is relative to the root assets directory, not to the manifest file.
* The `rects` coords are in order min_x, max_x, min_y, max_y.
* `rects` is a list not a map to preserve ordering. For this example, in the output texture atlas "yellow" will be texture index 0,
    "face" will be texture index 1, and "patches" will be texture index 2, the same order as in the manifest file.  

  


### Add the bevy heterogeneous texture atlas loader to your Cargo.toml `[dependencies]`

```
bevy_heterogeneous_texture_atlas_loader = { github = "https://github.com/ickshonpe/bevy_heterogeneous_texture_atlas_loader" }
```
### Add the Plugin to your Bevy App
```rust
use heterogeneous_texture_atlas_loader::*;
pub main() {
    let app = App::new();

    ...

    .add_plugin(HeterogeneousTextureAtlasLoaderPlugin);
}
```

### Load the manifest using the AssetServer in a startup system
```rust
pub fn setup(
    asset_server: Res<AssetServer>,
) {
 let _manifest: Handle<HeterogeneousTextureAtlasManifest> = asset_server.load("manifest.ron");
}
```

### Plugin sends an event once the TextureAtlas is ready
```rust
pub fn once_atlas_loaded(
    mut commands: Commands,
    mut events: EventReader<HeterogeneousTextureAtlasLoadedEvent>,
    mut manifests: Res<Assets<HeterogeneousTextureAtlasManifest>>,
) {
    for event in events {
        let manifest = manifests.get(&event.0).unwrap();
        commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(manifest.get("face")),
            texture_atlas: manifest.atlas.clone(),
            ..Default::default()
        });
    }
}
```
#
### FAQ
Can I use this with bevy_asset_loader?
> I wish

Why are the names of everything so long?  
> lols

Crash during loading?
> There might be a scheduling bug. Not fixed

Example doesn't work?
> Not tested


