A simple [Tibia](https://www.tibia.com/news/?subtopic=latestnews)-like RPG

## Build Dependencies
- [Rust](https://www.rust-lang.org/) toolchain ([rustup](https://rustup.rs/))
- macroquad (cargo)
- [macroquad-tiled](https://docs.rs/macroquad-tiled/latest/macroquad_tiled/) (cargo)

## Compilation + Execution
`cargo run --release`

*If you installed `cargo`, dependencies can just be installed by `cargo install`*

## World
Dyhra is two-dimensional but despite that, takes advantage of isometric designs to give a three-dimensional feel. It should purvey depth and capture an immersive experience with various structures such as buildings and caves The world should be filled with wildlife such as deer, wolves, bears and more. Some agressive, some not

As a player finds their way deeper into a cave, tower or the likes, they should face increasingly difficult enemies. Players will have to adventure and risk getting lost, stuck (forever) or killed. Dying in Dyhra is intended to be unforgiving and therefore a player will lose character progression, some items on hand and face other penalties

## Map
In a top-down world, only the size of the camera can be seen at any time. Tiles outside of the camera have less reason for being rendered than in other perspectives. (A procedurally generated map may be more efficent when the map grows)

Map files are created with a map editor such as [Tiled](https://www.mapeditor.org/), in which the capability to export as a JSON file is provided. Macroquad-tiled utilizes this JSON information to properly iterate a PNG tileset (easily made with a pixel art editor like [Aseprite](https://www.aseprite.org/)). Tiles are 32x32, drawn as 32x16 to give a proper alignment and stacking affect

https://not-fl3.github.io/platformer-book/tiled/index.html/

## Design
Dyhra's design choices are heavily inspired by Tibia - A 2D, top-down, oblique projection, persistent world RPG. To keep things more simple art-wise, an isometric perspective is used

https://www.tibiafanart.com/tibia-is-made-of-pixels/
https://opengameart.org/forumtopic/reverse-engineering-ultima-vii-isometrics-wip/
https://otland.net/threads/sprite-creature-perspective-tutorial.8824/

## Systems & Mechanics
If it's marked off, it means some basic implementation is at least in place. It may not be complete or very functional (yet)

- [x] Tilemap
- [x] Entity management
- [x] Input handling
- [x] Camera
- [x] AI algorithim
- [x] Collision detection
- [ ] Combat
- [ ] Character progression
- [ ] Inventory management
- [ ] Social integration
- [ ] Questing
- [ ] NPCs
- [ ] Economy
- [ ] Trading
- [ ] Rune making
- [ ] Alchemy
- [ ] Crafting
- [ ] Housing
- [ ] Events
- [ ] Guilds
- [ ] Leaderboard
- [ ] Achievements
- [ ] Raids


## Resources
gameloop, framecap, ECS, texture manager..  
https://github.com/wick3dr0se/sdl-game/

### Art
map  
https://scrabling.itch.io/pixel-isometric-tiles

### Tiled
map editor tutorial  
https://not-fl3.github.io/platformer-book/tiled/index.html

### Macroquad
platformer tutorial  
https://macroquad.rs/articles/fish-tutorial/

mesh tilemap  
https://github.com/Jakkestt/tiled_quad/blob/main/src/tiled_quad.rs

docs  
https://docs.rs/macroquad/latest/macroquad/

macroquad-tiled docs  
https://docs.rs/macroquad-tiled/latest/macroquad_tiled/

examples  
https://github.com/not-fl3/macroquad/tree/master/examples

### Isometric perspective
isometric math  
https://clintbellanger.net/articles/isometric_math/

world to map & map to world translations  
https://github.com/not-fl3/macroquad/pull/598/files