use bevy::prelude::*;
use bevy_heterogeneous_texture_atlas_loader::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let atlas: Handle<TextureAtlas> = asset_server.load("manifest.ron");

    for i in 0..3 {
        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(i),
            texture_atlas: atlas.clone(),
            transform: Transform::from_translation(
                (-100. + i as f32 * 100.) * Vec3::X + 50. * Vec3::Y,
            ),
            ..Default::default()
        });
    }

    let atlas: Handle<TextureAtlas> = asset_server.load("another.ron");
    for i in 0..2 {
        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(i),
            texture_atlas: atlas.clone(),
            transform: Transform::from_translation(
                (-50. + i as f32 * 100.) * Vec3::X - 50. * Vec3::Y,
            ),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TextureAtlasLoaderPlugin)
        .add_startup_system(setup)
        .run();
}
