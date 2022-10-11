# Bevy Heterogenous Texture Atlas Loader

[![crates.io](https://img.shields.io/crates/v/bevy_heterogeneous_texture_atlas_loader)](https://crates.io/crates/bevy_heterogeneous_texture_atlas_loader)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_heterogeneous_texture_atlas_loader)
[![crates.io](https://img.shields.io/crates/d/bevy_heterogeneous_texture_atlas_loader)](https://crates.io/crates/bevy_heterogeneous_texture_atlas_loader)

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases from a RON file manifest.

## Bevy Compatibility

| version | bevy |
| ------- | ---- |
| 0.7+    | 0.8  |
| 0.6     | 0.7  |
| < 0.6   | 0.6  |

#
## Basic usage
1. Add to your project's `Cargo.toml` ```[dependencies]``` section

    ```toml
    bevy_heterogeneous_texture_atlas_loader = "0.9"
    ```

1. Add the `TextureAtlasLoaderPlugin` to your Bevy App.
    ```rust
    use bevy_heterogeneous_texture_atlas_loader::*;
    app.add_plugin(TextureAtlasLoaderPlugin);
    ```

2. Add the atlas source image and `.ron` manifest to your assets folder.

3. Load the texture atlas manifest using the asset server:
    ```rust
    let texture_atlas: Handle<TextureAtlas> = asset_server.load("<path>.ron");
    ```
    The plugin will then load the atlas image and create the TextureAtlas asset automatically.

4. The `TextureAtlas`'s sprite indices respect the order of the sprites in the manifest. 
    Atlas index 0 will be the first sprite in the manifest, 1 the second, and so on.
    You can also use the `TextureAtlas::get_texture_index` method to look up the index using an asset path:
    ```rust
    texture_atlas.get_texture_index(&Handle::weak("example.png#sprite_name".into()))
    ```

    which you can see used in `\examples\example.rs`

#

## The Manifest 

* To create a manifest for a sprite sheet with irregular sized and positioned sprites like:

    ![/assets/example.png](/assets/example.png)


1. Create a .ron file in your assets folder. 

    The sprite indices in the output TextureAtlas are ordered implicitly according to the order of the input list sprite rects.

*   The `name` field is used to give a sprite a unique name that can be used to look
    up their TextureAtlas index using a weak `Handle<Image>` with the asset_path 
    `"example.png#sprite_name"`.

    ```rust
    (
        // Path to the texture atlas source image file 
        path: "example.png",        

        // List of sprites        
        sprites: [    
            (
                // use a weak handle with the asset path
                //      "example.png#rothko" 
                // to retrieve this sprite's index using TextureAtlas::get_texture_index.
                name: "rothko",  

                // top left x coordinate of the sprite in pixels
                x: 18,           

                // top left y coordinate of the sprite in pixels
                y: 19,              

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
* If you don't need names for the sprites, you can leave out the `name` field:
    ```rust
    (
        path: "example.png",

        sprites:[         
            (    
                // sprite at atlas index 0
                x: 18, 
                y: 19, 
                w: 46, 
                h: 48
            ),
            // ...
        ]
    )
    ```
  
## Examples

* `minimal.rs`

    Loads a texture atlas and cycles through its textures. Run with
    ```
    cargo run --example minimal
    ```

* `example.rs` 

    Example of loading and displaying a texture atlas. Run with
    ```
    cargo run --example example
    ```

* `bevy_asset_loader.rs`

    Example using `bevy_asset_loader` to manage loading. Run with
    ```
    cargo run --example bevy_asset_loader
    ```