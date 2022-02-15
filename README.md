# Bevy Heterogenous Texture Atlas Loader

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases according to a RON file manifest.

Suports Bevy 0.6
#
## Basic usage
1. Add the `TextureAtlasManifestLoaderPlugin` to your Bevy App.


2. Load the texture atlas manifest using the asset server:
    ```rust
    let atlas: Handle<TextureAtlas> = asset_server.load("manifest.ron");
    ```
    The plugin will then load the atlas image and create the TextureAtlas asset automatically.

    Once the manifest is loaded, the handle can be accessed from the ```.atlas``` field,
    or directly using the asset path `"manifest.ron"` like so:

    ```rust
    fn some_system(
        assets: Res<Assets<TextureAtlas>>,
        ...
    ) {
        let atlas = assets.get("manifest.ron").unwrap();
        ...
    );
    ```

#

## Detailed Example

* Given a sprite sheet with irregular sized and positioned sprites.

    ![/assets/example.png](/assets/example.png)


1. First create a `manifest.ron` manifest file in your assets folder

    ```
    (
        path: "example.png",
        width: 256,
        height: 256,
        sprites: Sprites ([
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
        path: "example.png",
        width: 256,
        height: 256,
        sprites: NamedSprites ([
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
    bevy_heterogeneous_texture_atlas_loader = "0.2"
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
    let atlas: Handle<TextureAtlas> = asset_server.load("manifest.ron");
    commands.insert_resource(atlas);
}

fn on_manifest_loaded(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<TextureAtlas>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(atlas) = atlases.get(handle) {
                    
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
                            texture_atlas: handle.clone(),
                            transform: Transform::from_translation(target),
                            ..Default::default()
                        });
                    }
                }
            },
            _ => {}
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TextureAtlasManifestLoaderPlugin)
    .add_startup_system(setup)
    .add_system(on_manifest_loaded)
    .run();
}
    ```

4. Result

    ![/assets/example.png](/assets/beautiful.png)



