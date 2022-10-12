use bevy::asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy::utils::{HashMap, HashSet};
use serde::Deserialize;
use std::path::Path;
use std::sync::Mutex;
use lazy_static::*;

lazy_static! {
    static ref LOADING: Mutex<
        HashMap<
            Handle<Image>, 
            HashSet<Handle<TextureAtlas>>
        >
    > = Mutex::new(HashMap::default());
}

#[derive(Debug, Deserialize)]
struct Sprite {
    #[serde(default)]
    name: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    path: String,
    sprites: Vec<crate::Sprite>,
}

impl From<Sprite> for (String, bevy::sprite::Rect) {
    fn from(sprite_rect: Sprite) -> Self {
        let Sprite { name, x, y, w, h } = sprite_rect;
        (
            name,
            bevy::sprite::Rect {
                min: vec2(x as f32, y as f32),
                max: vec2((x + w - 1) as f32, (y + h - 1) as f32),
            },
        )
    }
}

#[derive(Default)]
struct TextureAtlasLoader;

impl AssetLoader for TextureAtlasLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            // load manifest data
            let manifest: Manifest = ron::de::from_bytes(bytes)?;

            // get the image handle
            let image_asset_path = AssetPath::new_ref(Path::new(&manifest.path), None);
            let image_handle: Handle<Image> = load_context.get_handle(image_asset_path.clone());

            // create the texture atlas
            let mut texture_atlas = TextureAtlas::new_empty(image_handle.clone(), -Vec2::ONE);

            for (name, sprite_rect) in manifest.sprites.into_iter().map(|sprite| sprite.into()) {
                let index = texture_atlas.add_texture(sprite_rect);
                if !name.is_empty() {
                    let handles = texture_atlas
                        .texture_handles
                        .get_or_insert(HashMap::default());
                    if let Some(_rect) = handles.insert(Handle::weak(name.clone().into()), index) {
                        warn!(
                            "Sprite name {name} in manifest for texture atlas {} not unique",
                            manifest.path
                        );
                    }
                }
            }

            let asset_path = AssetPath::new(load_context.path().into(), None);
            let texture_atlas_handle: Handle<TextureAtlas> = load_context.get_handle(asset_path);

            // create and return the asset
            let atlas_asset = LoadedAsset::new(texture_atlas).with_dependency(image_asset_path);
            let mut loading = LOADING.lock().unwrap();
            loading
                .entry(image_handle.clone_weak())
                .or_insert_with(HashSet::default)
                .insert(texture_atlas_handle.clone_weak());
            
            load_context.set_default_asset(atlas_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

fn set_texture_atlas_size(
    mut image_asset_events: EventReader<AssetEvent<Image>>,
    images: Res<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for image_asset_event in image_asset_events.iter() {
        if let AssetEvent::Created {handle: image_handle} = image_asset_event {
            let mut loading = LOADING.lock().unwrap();
            if let Some(atlases) = loading.get_mut(image_handle) {
                for atlas_handle in atlases.drain() {
                    if let Some(texture_atlas) = texture_atlases.get_mut(&atlas_handle) {
                        texture_atlas.size = images.get(image_handle).unwrap().size();
                    }
                }
                loading.remove(image_handle);
            }
        }
    }
}

pub struct TextureAtlasLoaderPlugin;

impl Plugin for TextureAtlasLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<TextureAtlasLoader>()
            .add_system(set_texture_atlas_size);
    }
}
