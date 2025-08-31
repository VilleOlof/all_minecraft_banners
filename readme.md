# minecraft_banners

this project includes all **1,475,646,641,940,097,552** banners in Minecraft!!  

fast minecraft banner generation with `50µs~` on average (20x40px).  
technically supports custom patterns (tho rng will be different).  
generate new banners/pattern lists or get consistent ones with a seed.  

there is a module called `mcb.rs` which could almost be it's own crate,  
it handles the banner themself and the overlaying of patterns & color.  
`generation.rs` handles the... well generation of patterns, seeds & such.  
`main.rs` just holds the axum api and the route logic.  

## api
base: *0.0.0.0:8213*

- `/`  
    Basis string to check if the server is online.  
- `/banner`  
    Generate a random banner with a new random seed.  
- `/banner/:seed`  
    Generate a banner based from a seed.  
- `/pattern`  
    Generate a list of random patterns with a new random seed.  
- `/pattern/:seed`  
    Generate a list of patterns from a seed.  
- `/seed`  
    Generate a new seed within the determined range.  
- `/metadata`  
    Returns a list of all available banner patterns, all the colors & how many combinations are possible.  


`/banner`, `/banner:id`, `/pattern` & `/pattern/:id` all accepts some query arguments.  
- `base_color`  
    Specify a base color to always use instead of a random seeded one.  
    Specify the number representing the color in the enum (0-15).  
- `layers`  
    Specify a list of layers that will override the random seeded one.  
    Example: `?layers=&layers=&layers=[2, 7]` will always set layer 3 to `bricks` with the color `Gray`.  
    This can be used with any of the above endpoints & takes priority over any randomness.  
- `max_layers`  
    Specify how many layers will be used to generate the banner.  
