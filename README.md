# Dyhra

A simple [Tibia](https://www.tibia.com/news/?subtopic=latestnews)-like RPG

---

As players venture deeper into mysterious caves, forests and towering ruins of Dyhra, they’ll encounter a variety of increasingly challenging enemies. Navigating through dark passages that may lead to hidden treasures or deadly traps, each area is a test of skill, strategy, and sometimes sheer luck. With mythological creatures lurking everywhere, from sneaky goblins and trolls to fearsome hydras and dragons, every adventure is a chance to prove yourself

The world is unforgiving—dying means losing character progression, valuable items, and facing other harsh penalties, making each decision critical. Players must weigh risks carefully, balancing the desire for exploration against the looming threat of permanent loss

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

## Compilation + Execution
install dependencies (first time)
`cargo install`

`cargo run --release`

## Map
Map files are created with a map editor such as [Tiled](https://www.mapeditor.org/), in which the capability to export as a JSON file is provided. Macroquad-tiled utilizes this JSON information to properly iterate a PNG tileset (easily made with a pixel art editor like [Aseprite](https://www.aseprite.org/))

## Resources
https://developer.mozilla.org/en-US/docs/Games

gameloop, framecap, ECS, texture manager..  
https://github.com/wick3dr0se/sdl-game/

### Macroquad
tutorials  
https://www.gyata.ai/rust/macroquad#1-introduction  
https://macroquad.rs/articles/fish-tutorial/ (platformer)

mesh tilemap  
https://github.com/Jakkestt/tiled_quad/blob/main/src/tiled_quad.rs

docs  
https://docs.rs/macroquad/latest/macroquad/

examples  
https://github.com/rust-gamedev/rust-game-ports/tree/master/rusty_roguelike-macroquad  
https://github.com/amethyst/bracket-lib/tree/master/rltk  
https://github.com/not-fl3/miniquad/#building-examples  
https://github.com/ozkriff/awesome-quads  
https://github.com/not-fl3/macroquad/tree/master/examples

macroquad-tiled docs  
https://docs.rs/macroquad-tiled/latest/macroquad_tiled/

### Shipyard
tutorial  
https://leudz.github.io/shipyard/guide/0.7/welcome.html
https://tung.github.io/ruggrogue/source-code-guide/

docs  
https://docs.rs/shipyard/latest/shipyard/

examples  
https://github.com/tung/ruggrogue
https://github.com/griffi-gh/kubi

### Tiled
map editor tutorial  
https://not-fl3.github.io/platformer-book/tiled/index.html

### Isometric Projection
tutorial  
https://code.tutsplus.com/creating-isometric-worlds-a-primer-for-game-developers--gamedev-6511t

isometric math  
https://clintbellanger.net/articles/isometric_math/

world to map & map to world translations  
https://github.com/not-fl3/macroquad/pull/598/files