A simple [Tibia](https://www.tibia.com/news/?subtopic=latestnews)-like RPG

## World
Dyhra is two-dimensional but despite that, takes advantage of isometric designs to give a three-dimensional feel. It should purvey depth and capture an immersive experience with various structures such as castles and caves. The world is filled with wildlife such as deer, wolves, bears and more. Some agressive, some not

As a player finds their way deeper into a cave, tower or the likes, they should face increasingly difficult enemies. Players will have to adventure and risk getting lost, stuck (forever) or killed. Dying in Dyhra is intended to be unforgiving and therefore a player will lose character progression, some items on hand and face other penalties

## Map
In a top-down world, only the size of the camera can be seen at any time. Tiles outside of the camera have less reason for being rendered than in other perspectives, therefore procedural generation is particularly taken advantage of

Map files are created with a map editor such as [Tiled](https://www.mapeditor.org/), in which the capability to export as a JSON file is provided. Macroquad-tiled utilizes this JSON information to properly iterate a PNG tileset (easily made with a pixel art editor like [Aseprite](https://www.aseprite.org/)). Tiles are 32x32, drawn as 32x16 to give a proper alignment and stacking effect

## Systems & Mechanics
If it's marked off, it means some basic implementation is at least in place. It may not be complete or very functional (yet)

- [x] Tilemap
- [x] Entity management
- [x] Input handling
- [x] Camera
- [x] AI algorithim
- [x] Collision detection
- [x] Procedural rendering
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

## Dependencies
- macroquad (cargo)
- [macroquad-tiled](https://docs.rs/macroquad-tiled/latest/macroquad_tiled/) (cargo)

## Compilation + Execution
install dependencies (first time)
`cargo install`

`cargo run --release`

## Resources
https://developer.mozilla.org/en-US/docs/Games

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
https://github.com/ozkriff/awesome-quads  
https://github.com/not-fl3/macroquad/tree/master/examples

### Isometric perspective
https://code.tutsplus.com/creating-isometric-worlds-a-primer-for-game-developers--gamedev-6511t

isometric math  
https://clintbellanger.net/articles/isometric_math/

world to map & map to world translations  
https://github.com/not-fl3/macroquad/pull/598/files