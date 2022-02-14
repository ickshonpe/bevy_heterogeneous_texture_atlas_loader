use bevy::prelude::*;
use bevy::math::vec2;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use bevy::asset::{LoadContext, BoxedFuture, LoadedAsset, AssetLoader};
use bevy::sprite::TextureAtlas;
use serde::Deserialize;

#[derive(Debug, TypeUuid)]
#[uuid = "b00584ad-0507-44ed-a89c-e6758f3576f6"]
pub struct TextureAtlasManifest {
    path: String,
    sprite_rects: Vec<(String, bevy::sprite::Rect)>,
    pub atlas: Handle<TextureAtlas>,
    pub indices: HashMap<String, usize>
}   

impl TextureAtlasManifest {
    pub fn get_index(&self, sprite_name: &str) -> usize {
        self.indices[sprite_name]
    }
}

#[derive(Debug, Deserialize)]
pub struct SpriteRect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl From<SpriteRect> for NamedSpriteRect {
    fn from(val: SpriteRect) -> Self {
        NamedSpriteRect {
            name: "".into(),
            x: val.x, 
            y: val.y,
            w: val.w,
            h: val.h
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NamedSpriteRect {
    name: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Deserialize)]
pub enum SpriteRects {
    NamedSprites(Vec<NamedSpriteRect>),
    Sprites(Vec<SpriteRect>),
}

impl From<SpriteRects> for Vec<(String, bevy::sprite::Rect)> {
    fn from(rects: SpriteRects) -> Self {
        match rects {
            SpriteRects::NamedSprites(rects) => rects,
            SpriteRects::Sprites(rects) => {
                rects.into_iter().map(|rect| NamedSpriteRect::from(rect)).collect()
            },
        }
        .into_iter()
        .map(|NamedSpriteRect { name, x, y, w, h } | (
            name,
            bevy::sprite::Rect { 
                min: vec2(x as f32, y as f32),
                max: vec2((x + w - 1) as f32,  (y + h - 1) as f32),
            }
        ))
        .collect()
    }
}


#[derive(Default)]
pub struct TextureAtlasManifestLoader;

impl AssetLoader for TextureAtlasManifestLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let (path, rects): (String, SpriteRects) = ron::de::from_bytes(bytes)?;
            let sprite_rects: Vec<(String, bevy::sprite::Rect)> = rects.into();
            let manifest = TextureAtlasManifest {
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

pub fn manifest_events_handler(
    mut local: Local<HashMap<Handle<Image>, Handle<TextureAtlasManifest>>>,
    mut manifest_events: EventReader<AssetEvent<TextureAtlasManifest>>,
    mut image_events: EventReader<AssetEvent<Image>>,
    asset_server: Res<AssetServer>,
    mut manifests: ResMut<Assets<TextureAtlasManifest>>,
    mut images: ResMut<Assets<Image>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut event_writer: EventWriter<TextureAtlasManifestLoadedEvent>,
) {
    let image_map = &mut *local;
    for event in manifest_events.iter() {
        match event {
            AssetEvent::Created { handle: manifest_handle } => {
                let manifest = manifests.get_mut(manifest_handle).expect("Manifest asset not found.");
                let image_handle: Handle<Image> = asset_server.load(&manifest.path);
                image_map.insert(image_handle, manifest_handle.clone());
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
                if let Some(mut manifest_handle) = image_map.remove(image_handle) {
                    let mut image_handle = image_handle.clone();
                    image_handle.make_strong(&mut images);
                    let manifest = manifests.get_mut(&manifest_handle).expect("Manifest asset not found.");
                    let image = images.get(&image_handle).expect("Image asset not found.");
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
                        if name != "" {
                            manifest.indices.insert(name.clone(), index);
                        }
                    }
                    let atlas_handle = atlases.add(atlas);
                    manifest.atlas = atlas_handle.clone();
                    manifest_handle.make_strong(&mut manifests);
                    event_writer.send(TextureAtlasManifestLoadedEvent { 
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
pub struct TextureAtlasManifestLoadedEvent {
    pub manifest: Handle<TextureAtlasManifest>,
    pub atlas: Handle<TextureAtlas>,
}

pub struct TextureAtlasManifestLoaderPlugin;

impl Plugin for TextureAtlasManifestLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<TextureAtlasManifestLoadedEvent>()
        .add_asset::<TextureAtlasManifest>()
        .init_asset_loader::<TextureAtlasManifestLoader>()
        .add_system_to_stage(
            CoreStage::PreUpdate,
            manifest_events_handler
        );
    }
}