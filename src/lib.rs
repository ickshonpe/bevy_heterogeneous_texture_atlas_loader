use std::path::Path;

use bevy::prelude::*;
use bevy::math::vec2;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use bevy::asset::{LoadContext, BoxedFuture, LoadedAsset, AssetLoader, AssetPath};
use bevy::sprite::TextureAtlas;
use serde::Deserialize;

#[derive(Debug, TypeUuid)]
#[uuid = "b00584ad-0507-44ed-a89c-e6758f3576f6"]
pub struct TextureAtlasManifest {
    pub atlas: Handle<TextureAtlas>,
    pub named_sprites: HashMap<String, usize>
}   

impl TextureAtlasManifest {
    pub fn get_atlas_index(&self, sprite_name: &str) -> usize {
        self.named_sprites[sprite_name]
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
enum SpriteRects {
    NamedSprites(Vec<NamedSpriteRect>),
    Sprites(Vec<SpriteRect>),
}

#[derive(Debug, Deserialize)]
struct Manifest {
    path: String,
    width: u32,
    height: u32,
    sprites: SpriteRects,
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
            // load manifest data
            let manifest: Manifest = ron::de::from_bytes(bytes)?;
            
            // get the image handle
            let image_asset_path = AssetPath::new_ref(Path::new(&manifest.path), None);
            let image_handle: Handle<Image> = load_context.get_handle(image_asset_path.clone());

            // create the texture atlas
            let mut texture_atlas = TextureAtlas::new_empty(
                image_handle,
                manifest.width as f32 * Vec2::X + manifest.height as f32 * Vec2::Y
                    // Had to comprimise on ergonomics here and demand the image dimensions from the user,
                    // because with this method we have to create the texture atlas before the image is loaded.
            );
            
            let rects: Vec<(String, bevy::sprite::Rect)> = manifest.sprites.into();
            let mut named_sprites = HashMap::default();
            for (name, sprite_rect) in rects.into_iter() {
                let index = texture_atlas.add_texture(sprite_rect);
                if name != "" {                    
                    if let Some(_rect) = named_sprites.insert(name.clone(), index) {
                        warn!("Sprite name {name} in manifest for texture atlas {} not unique", manifest.path);
                    }
                }
            }
            let atlas_asset = LoadedAsset::new(texture_atlas).with_dependency(image_asset_path);
            let atlas_handle = load_context.set_labeled_asset("texture_atlas", atlas_asset);

            // create asset
            let manifest = TextureAtlasManifest {
                atlas: atlas_handle,
                named_sprites
            };
            let manifest_asset = LoadedAsset::new(manifest);
            load_context.set_default_asset(manifest_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
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
        .add_asset::<TextureAtlasManifest>()
        .init_asset_loader::<TextureAtlasManifestLoader>();
    }
}