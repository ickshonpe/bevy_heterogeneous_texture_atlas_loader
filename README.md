# Bevy Heterogenous Texture Atlas Loader

Bevy Heterogenous Texture Atlas Loader allows you to load heterogenous texture atlases according to a RON file manifest.

Suports Bevy 0.6
#
## Basic usage
1. Add to your project's `Cargo.toml` ```[dependencies]``` section

    ```
    bevy_heterogeneous_texture_atlas_loader = "0.5"
    ```

1. Add the `TextureAtlasLoaderPlugin` to your Bevy App.
    ```
    use bevy_heterogeneous_texture_atlas_loader::*;
    app.add_plugin(TextureAtlasLoaderPlugin)
    ```

2. Add the atlas source image and `.ron` manifest to your assets folder.

2. Load the texture atlas manifest using the asset server:
    ```rust
    let atlas: Handle<TextureAtlas> = asset_server.load("<path>.ron");
    ```
    The plugin will then load the atlas image and create the TextureAtlas asset automatically.

#

## The Manifest 

* To create a manifest for a sprite sheet with irregular sized and positioned sprites like:

    ![/assets/example.png](/assets/example.png)


1. Create a .ron file in your assets folder. 


    Each sprite can be given a unique name that can be used to look
    up their TextureAtlas index using a weak `Handle<Image>` with the asset_path 
    `"example.png#sprite_name"`.

    ```
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
                //      "example.png#yellow" 
                // to retrieve this sprite's index.
                name: "yellow",     

                // top left y coordinate of the sprite in pixels
                y: 19,              

                // top left x coordinate of the sprite in pixels
                x: 18,              

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
    You can omit the names of the sprites if you don't need them:
    ```
    (
        path: "example.png",
        width: 256,
        height: 256,
        // Sprites variant accessible only by usize index.
        sprites: Sprites ([         
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

    * The sprite indices in the output TextureAtlas are ordered implicitly according to the order of the input list sprite rects.
    * Use `name: ""` to skip naming a sprite in a `NamedSprites` list
  
## Examples

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
