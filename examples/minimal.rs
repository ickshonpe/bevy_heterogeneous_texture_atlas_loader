use bevy::prelude::*;
use bevy_heterogeneous_texture_atlas_loader::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let texture_atlas_handle: Handle<TextureAtlas> = asset_server.load("anonymous_manifest.ron");
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    });
}

fn cycle_sprites(
    mut timer: Local<f32>,
    time: Res<Time>,
    atlas_assets: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    *timer += time.delta_seconds();
    if 1.0 < *timer {
        *timer = 0.;
        query.for_each_mut(|(mut texture_atlas_sprite, texture_atlas_handle)| {
            if let Some(texture_atlas) = atlas_assets.get(texture_atlas_handle) {
                texture_atlas_sprite.index = (texture_atlas_sprite.index + 1) % texture_atlas.len();
            }
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TextureAtlasLoaderPlugin)
        .add_startup_system(setup)
        .add_system(cycle_sprites)
        .run();
}
