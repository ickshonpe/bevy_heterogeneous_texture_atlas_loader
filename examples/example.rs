use bevy::prelude::*;
use bevy_heterogeneous_texture_atlas_loader::*;

#[derive(Deref, DerefMut, Resource)]
pub struct MyTextureAtlas(Handle<TextureAtlas>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let atlas: Handle<TextureAtlas> = asset_server.load("manifest.ron");
    commands.insert_resource(MyTextureAtlas(atlas));
}

fn on_loaded(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<TextureAtlas>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(atlas) = atlases.get(handle) {
                commands.spawn(SpriteBundle {
                    texture: atlas.texture.clone(),
                    ..Default::default()
                });
                for (index, &name) in ["rothko", "face", "patches"].iter().enumerate() {
                    let target =
                        -300. * Vec3::X + (100. * index as f32 - 100.) * Vec3::Y + 0.25 * Vec3::ONE;
                    commands.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(index),
                        texture_atlas: handle.clone(),
                        transform: Transform::from_translation(target),
                        ..Default::default()
                    });
                    let index_from_handle =
                        atlas.get_texture_index(&Handle::weak(name.into())).unwrap();
                    commands.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(index_from_handle),
                        texture_atlas: handle.clone(),
                        transform: Transform::from_translation(target + 100. * Vec3::X),
                        ..Default::default()
                    });
                }
            }
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
