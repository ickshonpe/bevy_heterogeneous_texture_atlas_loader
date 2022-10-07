# Bevy Heterogenous Texture Atlas Loader

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases from a RON file manifest.

## Bevy Compatibility

| version | bevy |
| ------- | ---- |
| 0.7     | 0.8  |
| 0.6     | 0.7  |
| < 0.6   | 0.6  |

#
## Basic usage
1. Add to your project's `Cargo.toml` ```[dependencies]``` section

    ```toml
    bevy_heterogeneous_texture_atlas_loader = "0.8.1"
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

    The `sprites` field has `NamedSprites` and `AnonymousSprites` variants.

*   `NamedSprites` can be given a unique name that can be used to look
    up their TextureAtlas index using a weak `Handle<Image>` with the asset_path 
    `"example.png#sprite_name"`.

    Use `name: ""` to skip naming a sprite in a `NamedSprites` list

    ```rust
    (
        // Path to the texture atlas source image file 
        path: "example.png",        

        // width of the source image in pixels
        width: 256,                

        // height of the source image in pixels 
        height: 256,              

        // NamedSprites variant allows you give each sprite an identifying 
        // asset path.  
        sprites: NamedSprites ([    
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
* If you don't need names for the sprites, use the `AnonymousSprites` variant:
    ```rust
    (
        path: "example.png",
        width: 256,
        height: 256,
        // AnonymousSprites variant sprites are accessible only by their index.
        sprites: AnonymousSprites ([         
            (    
                // sprite at atlas index 0
                x: 18, 
                y: 19, 
                w: 46, 
                h: 48
            ),
            (                       
                // sprite at atlas index 1
                x: 93, 
                y: 108, 
                w: 32, 
                h: 31
            ),
            (                       
                // sprite at atlas index 2
                x: 176, 
                y: 34, 
                w: 20, 
                h: 34
            ),
        ])
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