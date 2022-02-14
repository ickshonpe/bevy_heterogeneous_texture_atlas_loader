# Bevy Heterogenous Texture Atlas Loader

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases according to a RON file manifest.


This works (as far as I can tell), but the implementation could be improved a lot. Any suggestions would be very welcome.

Suports Bevy 0.6

#

## Usage Example

We have a sprite sheet `example.png` for our game,

 ![/assets/example.png](/assets/example.png)

that has sprites with irregular sizes and positions.

## How to load the sprite sheet

1. Create a `manifest.ron` manifest file in your assets folder

```
(
    "example.png",
    Sprites ([
        (
            x: 18, 
            y: 19, 
            w: 46, 
            h: 48
        ),
        (
            x: 93, 
            y: 108, 
            w: 32, 
            h: 31
        ),
        (
            x: 176, 
            y: 34, 
            w: 20, 
            h: 34
        ),
    ])
)
```
Alternatively, you can give each sprite a unique name that can be used to look
up their TextureAtlas index.

```
(
    "example.png",
    NamedSprites ([
        {
            name: "yellow", 
            x: 18, 
            y: 19, 
            w: 46, 
            h: 48
        },
        {
            name: "face", 
            x: 93, 
            y: 108, 
            w: 32, 
            h: 31
        },
        {
            name: "patches", 
            x: 176, 
            y: 34, 
            w: 20, 
            h: 34
        },
    ])
)
```
* You can call the manifest anything you like, not only `manifest.ron`.
* The file path is relative to the root assets folder, not to the manifest file.
* The sprites are in a list not a map to preserve ordering. The sprite indices in the ouput ordered implicitly according to the order of the input list.
* Use `name: ""` to skip naming a sprite in a `NamedSprites` list
2. Add the BHTAL dependency to your `Cargo.toml`

```
bevy_heterogeneous_texture_atlas_loader = "0.1.2"
```


3. Write the app


```rust
use bevy::prelude::*;
use bevy_heterogeneous_texture_atlas_loader::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let manifest: Handle<HeterogeneousTextureAtlasManifest> = asset_server.load("manifest.ron");
    commands.insert_resource(manifest);
}

fn on_atlas_loaded(
    mut commands: Commands,
    mut events: EventReader<HeterogeneousTextureAtlasLoadedEvent>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    for event in events.iter() {
        let atlas = atlases.get(&event.atlas).unwrap();
        commands
        .spawn_bundle(SpriteBundle {
            texture: atlas.texture.clone(),
            ..Default::default()
        });
        for i in 0..3 {
            let target = -200. * Vec3::X + (100. * i as f32 - 100.) * Vec3::Y;
            commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(i),
                texture_atlas: event.atlas.clone(),
                transform: Transform::from_translation(target),
                ..Default::default()
            });
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HeterogeneousTextureAtlasLoaderPlugin)
    .add_startup_system(setup)
    .add_system(on_atlas_loaded)
    .run();
}
```
4. Result

 ![/assets/example.png](/assets/beautiful.png)


#
### FAQ
#### Can I use this with bevy_asset_loader?
> I wish
#### "Manifest not found"?
> You need to store a strong handle to the manifest in a resource or something,
otherwise it will be dropped before the texture atlas is created.
#### Why are the names for everything so long?
> Shorter than "**Bevy Heterogenous Texture Atlas Ron Manifest Asynchronous Loader**"



