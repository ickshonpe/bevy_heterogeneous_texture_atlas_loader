use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_heterogeneous_texture_atlas_loader::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Results,
}

#[derive(AssetCollection)]
struct MyTextureAtlas {
    #[asset(path = "manifest.ron")]
    handle: Handle<TextureAtlas>,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn show_atlas(
    mut commands: Commands,
    atlases: Res<Assets<TextureAtlas>>,
    my_texture_atlas: Res<MyTextureAtlas>,
) {
    let atlas = atlases.get(&my_texture_atlas.handle).unwrap();
    commands.spawn_bundle(SpriteBundle {
        texture: atlas.texture.clone(),
        ..Default::default()
    });
    let asset_paths = [
        "example.png#rothko",
        "example.png#face",
        "example.png#patches",
    ];
    for (index, &name) in asset_paths.iter().enumerate() {
        let target = -300. * Vec3::X + (100. * index as f32 - 100.) * Vec3::Y + 0.25 * Vec3::ONE;

        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(index),
            texture_atlas: my_texture_atlas.handle.clone(),
            transform: Transform::from_translation(target),
            ..Default::default()
        });

        let index_from_handle = atlas.get_texture_index(&Handle::weak(name.into())).unwrap();
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(index_from_handle),
            texture_atlas: my_texture_atlas.handle.clone(),
            transform: Transform::from_translation(target + 100. * Vec3::X),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_state(GameState::Loading)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Results)
                .with_collection::<MyTextureAtlas>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(TextureAtlasLoaderPlugin)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::Results).with_system(show_atlas))
        .run();
}
