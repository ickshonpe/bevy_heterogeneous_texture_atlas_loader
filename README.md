# Bevy Heterogenous Texture Atlas Loader

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases according to a RON file manifest.

Suports Bevy 0.6
#
## Basic usage
1. Add the `TextureAtlasLoaderPlugin` to your Bevy App.

2. Add the atlas source image and `.ron` manifest to your assets folder.

2. Load the texture atlas manifest using the asset server:
    ```rust
    let atlas: Handle<TextureAtlas> = asset_server.load("manifest.ron");
    ```
    The plugin will then load the atlas image and create the TextureAtlas asset automatically.

#

## Detailed Example

* Given a sprite sheet with irregular sized and positioned sprites.

    ![/assets/example.png](/assets/example.png)


1. First create a `manifest.ron` manifest file in your assets folder.
    You can give each sprite a unique name that can be used to look
    up their TextureAtlas index using a weak `Handle<Image>` with the asset_path 
    `"example.png#sprite_name"`.

    ```
    (
        // Path of the texture atlas source image file relative to
        // the root assets folder.
        path: "example.png",        

        // width of the source image in pixels
        width: 256,                

        // height of the source image in pixels 
        height: 256,              

        // NamedSprites variant allows you give each sprite an identifying 
        // asset path.  
        sprites: NamedSprites ([    
            (
                // use a weak handle with the asset path
                //      "example.png#yellow" 
                // to retrieve this sprite's index.
                name: "yellow",     

                // top left y coordinate of the sprite in pixels
                y: 19,              

                // top left x coordinate of the sprite in pixels
                x: 18,              

                // width of the sprite in pixels
                w: 46,              

                // height of the sprite in pixels
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
    You can omit the names of the sprites if you don't need them:
    ```
    (
        path: "example.png",
        width: 256,
        height: 256,
        // Sprites variant accessible only by usize index.
        sprites: Sprites ([         
            (    
                // sprite at atlas index 0
                x: 18, 
                y: 19, 
                w: 46, 
                h: 48
            ),
            (                       
                // sprite at atlas index 1
                x: 93, 
                y: 108, 
                w: 32, 
                h: 31
            ),
            (                       
                // sprite at atlas index 2
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
    bevy_heterogeneous_texture_atlas_loader = "0.4"
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

    fn on_loaded(
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
                        for (index, &name) in [
                            "example.png#yellow",
                            "example.png#face",
                            "example.png#tpatches",
                        ].iter().enumerate() {
                            let target = 
                                -300. * Vec3::X 
                                + (100. * index as f32 - 100.) * Vec3::Y 
                                + 0.25 * Vec3::ONE;

                            commands
                            .spawn_bundle(SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(index),
                                texture_atlas: handle.clone(),
                                transform: Transform::from_translation(target),
                                ..Default::default()
                            });

                            let index_from_handle = atlas.get_texture_index(&Handle::weak(name.into())).unwrap();
                            commands
                            .spawn_bundle(SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(index_from_handle),
                                texture_atlas: handle.clone(),
                                transform: Transform::from_translation(target + 100. * Vec3::X),
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
        .add_plugin(TextureAtlasLoaderPlugin)
        .add_startup_system(setup)
        .add_system(on_loaded)
        .run();
    }
    ```

4. Result

    ![/assets/example.png](/assets/beautiful.png)

