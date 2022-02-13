use bevy::prelude::*;
use bevy::math::vec2;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use bevy::asset::{LoadContext, BoxedFuture, LoadedAsset, AssetLoader};
use bevy::sprite::TextureAtlas;
use serde::Deserialize;

#[derive(Debug, TypeUuid)]
#[uuid = "b00584ad-0507-44ed-a89c-e6758f3576f6"]
pub struct HeterogeneousTextureAtlasManifest {
    path: String,
    pub atlas: Handle<TextureAtlas>,
    sprite_rects: Vec<(String, bevy::sprite::Rect)>,
    pub indices: HashMap<String, usize>
}   

impl HeterogeneousTextureAtlasManifest {
    pub fn get(&self, sprite_name: &str) -> usize {
        self.indices[sprite_name]
    }
}

#[derive(Debug, Deserialize)]
pub struct SpriteSheetManifest {
    pub path: String,
    pub rects: Vec<(String, u32, u32, u32, u32)>,
}

#[derive(Default)]
pub struct HeterogeneousTextureAtlasManifestLoader;

impl AssetLoader for HeterogeneousTextureAtlasManifestLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let SpriteSheetManifest { path, rects } = ron::de::from_bytes(bytes)?;
            let sprite_rects: Vec<(String, bevy::sprite::Rect)> = rects.into_iter()
                .map(|(name, left, right, bottom, top)| (
                    name,
                    bevy::sprite::Rect { 
                        min: vec2(left as f32, bottom as f32),
                        max: vec2(right as f32, top as f32),
                    }
                ))
                .collect();
            sprite_rects.iter().for_each(|x| println!("{:?}", x));
            let manifest = HeterogeneousTextureAtlasManifest {
                path,
                atlas: Handle::default(),
                sprite_rects,
                indices: HashMap::default()
            };
            load_context.set_default_asset(LoadedAsset::new(manifest));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

pub fn heterogeneous_atlas_manifest_events_handler(
    mut local: Local<HashMap<Handle<Image>, Handle<HeterogeneousTextureAtlasManifest>>>,
    mut manifest_events: EventReader<AssetEvent<HeterogeneousTextureAtlasManifest>>,
    mut image_events: EventReader<AssetEvent<Image>>,
    asset_server: Res<AssetServer>,
    mut manifestos: ResMut<Assets<HeterogeneousTextureAtlasManifest>>,
    images: Res<Assets<Image>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut event_writer: EventWriter<HeterogeneousTextureAtlasLoadedEvent>,
) {
    let image_map = &mut *local;
    for event in manifest_events.iter() {
        match event {
            AssetEvent::Created { handle: manifest_handle } => {
                let manifest = manifestos.get_mut(manifest_handle).unwrap();
                let image_handle: Handle<Image> = asset_server.load(&manifest.path);
                image_map.insert(image_handle.clone_weak(), manifest_handle.clone_weak());
            },
            _ => {},
        }
    }
    if image_map.is_empty() {
        return;
    }
    for event in image_events.iter() {
        match event {
            AssetEvent::Created { handle: image_handle } => {
                if let Some(manifest_handle) = image_map.remove(image_handle) {
                    let manifest = manifestos.get_mut(&manifest_handle).unwrap();
                    let image = images.get(image_handle).unwrap();
                    let image_dimensions = Vec2::new(
                        image.texture_descriptor.size.width as f32,
                        image.texture_descriptor.size.height as f32
                    );
                    let mut atlas = TextureAtlas::new_empty(
                        image_handle.clone(),
                        image_dimensions
                    );
                    for (name, sprite_rect) in manifest.sprite_rects.iter() {
                        let index = atlas.add_texture(*sprite_rect);
                        manifest.indices.insert(name.clone(), index);
                    }
                    let atlas_handle = atlases.add(atlas);
                    manifest.atlas = atlas_handle.clone();
                    event_writer.send(HeterogeneousTextureAtlasLoadedEvent { 
                        manifest: manifest_handle, 
                        atlas: atlas_handle 
                    });
                }
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct HeterogeneousTextureAtlasLoadedEvent {
    pub manifest: Handle<HeterogeneousTextureAtlasManifest>, 
    pub atlas: Handle<TextureAtlas>,
}

pub struct HeterogeneousTextureAtlasLoaderPlugin;

impl Plugin for HeterogeneousTextureAtlasLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<HeterogeneousTextureAtlasLoadedEvent>()
        .add_asset::<HeterogeneousTextureAtlasManifest>()
        .init_asset_loader::<HeterogeneousTextureAtlasManifestLoader>()
        .add_system(heterogeneous_atlas_manifest_events_handler);
    }
}