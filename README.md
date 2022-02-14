# Bevy Heterogenous Texture Atlas Loader

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases according to a RON file manifest.

It works, but the implementation could be improved a lot. Any suggestions would be very welcome.

Suports Bevy 0.6
#
## Basic usage
1. Add the `TextureAtlasManifestLoaderPlugin` to your Bevy App.


2. Load the texture atlas manifest using the asset server:
    ```rust
    let manifest: Handle<TextureAtlasManifest> = asset_server.load("manifest.ron");
    ```
    The plugin listens for AssetServer events. Once the manifest file is loaded it will automatically begin loading the corresponding image file.


3. Once the image file is loaded, it constructs the TextureAtlas and then emits a TextureAtlasManifestLoadedEvent event. A strong handle to the new atlas can be retrieved from that event's `atlas` field.

#

## Detailed Example

We have a sprite sheet `example.png` for our game,

 ![/assets/example.png](/assets/example.png)

that has sprites with irregular sizes and positions.


1. First create a `manifest.ron` manifest file in your assets folder

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
            (
                name: "yellow", 
                x: 18, 
                y: 19, 
                w: 46, 
                h: 48
            ),
            (
                name: "face", 
                x: 93, 
                y: 108, 
                w: 32, 
                h: 31
            ),
            (
                name: "patches", 
                x: 176, 
                y: 34, 
                w: 20, 
                h: 34
            ),
        ])
    )
    ```
    * You can call the manifest anything you like, not only `manifest.ron`.
    * The file path is relative to the root assets folder, not to the manifest file.
    * The sprite indices in the output TextureAtlas are ordered implicitly according to the order of the input list sprite rects.
    * Use `name: ""` to skip naming a sprite in a `NamedSprites` list
2. Add this crate's dependency to your project's `Cargo.toml` ```[dependencies]``` section

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
        let manifest: Handle<TextureAtlasManifest> = asset_server.load("manifest.ron");
        commands.insert_resource(manifest);
    }

    fn on_atlas_loaded(
        mut commands: Commands,
        mut events: EventReader<TextureAtlasManifestLoadedEvent>,
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
        .add_plugin(TextureAtlasManifestLoaderPlugin)
        .add_startup_system(setup)
        .add_system(on_atlas_loaded)
        .run();
    }
    ```
4. Result

    ![/assets/example.png](/assets/beautiful.png)


#
### Other Questions
#### "Manifest not found"?
> You need to keep a strong handle to the manifest,
otherwise it will be dropped before the texture atlas is created.



