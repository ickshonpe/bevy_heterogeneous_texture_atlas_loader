use bevy::asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy::utils::HashMap;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct SpriteRect {
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
            h: val.h,
        }
    }
}

#[derive(Debug, Deserialize)]
struct NamedSpriteRect {
    name: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Deserialize)]
enum SpriteRects {
    NamedSprites(Vec<NamedSpriteRect>),
    AnonymousSprites(Vec<SpriteRect>),
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
            SpriteRects::AnonymousSprites(rects) => rects
                .into_iter()
                .map(|rect| NamedSpriteRect::from(rect))
                .collect(),
        }
        .into_iter()
        .map(|NamedSpriteRect { name, x, y, w, h }| {
            (
                name,
                bevy::sprite::Rect {
                    min: vec2(x as f32, y as f32),
                    max: vec2((x + w - 1) as f32, (y + h - 1) as f32),
                },
            )
        })
        .collect()
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
            let mut texture_atlas = TextureAtlas::new_empty(
                image_handle,
                manifest.width as f32 * Vec2::X + manifest.height as f32 * Vec2::Y, // Had to comprimise on ergonomics here and demand the image dimensions from the user,
                                                                                    // because with this method we have to create the texture atlas before the image is loaded.
            );

            let rects: Vec<(String, bevy::sprite::Rect)> = manifest.sprites.into();
            for (name, sprite_rect) in rects.into_iter() {
                let index = texture_atlas.add_texture(sprite_rect);
                if name != "" {
                    let handles = texture_atlas
                        .texture_handles
                        .get_or_insert(HashMap::default());
                    let mut handle_name = manifest.path.to_owned();
                    handle_name.push_str("#");
                    handle_name.push_str(&name);
                    let handle: Handle<Image> = load_context.get_handle(handle_name);
                    if let Some(_rect) = handles.insert(handle.as_weak(), index) {
                        warn!(
                            "Sprite name {name} in manifest for texture atlas {} not unique",
                            manifest.path
                        );
                    }
                }
            }

            // create and return the asset
            let atlas_asset = LoadedAsset::new(texture_atlas).with_dependency(image_asset_path);
            load_context.set_default_asset(atlas_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

pub struct TextureAtlasLoaderPlugin;

impl Plugin for TextureAtlasLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<TextureAtlasLoader>();
    }
}
