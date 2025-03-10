<div align="center">
    <h1>Dyrah</h1>
    <p>A simple <a href="https://tibia.com">Tibia</a>-like MORPG written entirely in Rust</p>
</div>

![Dyrah Screenshot](assets/screenshot.png)

---  
  
As players venture deeper into mysterious caves, forests and towering ruins of Dyrah, they’ll encounter a variety of increasingly challenging enemies. Navigating through dark passages that may lead to hidden treasures or deadly traps, each area is a test of skill, strategy, and sometimes sheer luck. With mythical creatures lurking everywhere, from sneaky goblins and trolls to fearsome hydras and dragons, every adventure is a chance to prove yourself
  
The world is unforgiving—dying means losing character progression, valuable items, and facing other harsh penalties, making each decision critical. Players must weigh risks carefully, balancing the desire for exploration against the looming threat of permanent loss

## Getting Started
### Dependencies
- Rust (stable)

### Compiling From Source
**Download Dyrah**  
`git clone https://github.com/opensource-force/dyrah; cd dyrah`

**Compiling/executing the server**  
`cargo run -rp dyrah_server`

**Compiling/executing the client**  
`cargo run -rp dyrah_client`

*For detailed build instructions for the client, defer to the [Macroquad build instructions](https://github.com/not-fl3/macroquad#build-instructions)*

*Alternatively, execute `exec.sh` to run the server & client simultaneously*

## Features & Development
Dyrah is in early stages of development, so expect bugs and missing features

### Core Systems
- [x] Multiplayer server-client
- [x] Entity management (ECS)
- [x] Input handling
- [x] Tilemap
- [x] Camera
- [x] Collision detection
- [ ] Combat system
- [ ] Tile-based movement
- [ ] AI behavior
- [ ] Pathfinding
- [ ] NPC interaction
- [ ] Quest system

### Gameplay Mechanics
- [ ] Character progression  
- [ ] Inventory management  
- [ ] Social integration  
- [ ] Questing  
- [ ] NPCs  
- [ ] Economy  
- [ ] Player trading  
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
### Recommended Tools
Map files are created with the [Tiled](https://www.mapeditor.org/) map editor, in which the capability to export as a JSON file is provided. This JSON information is utilized to render a tileset, which can be created with a pixel art editor like [Aseprite](https://www.aseprite.org/)

### Docs
- [Macroquad](https://docs.rs/macroquad/latest/macroquad/)

### Guides
- **Macroquad**
    - [Basic introduction](https://www.gyata.ai/rust/macroquad#q-introduction)
    - [Fish platformer](https://macroquad.rs/articles/fish-tutorial/)
- [Tiled](https://not-fl3.github.io/platformer-book/tiled/index.html)
- **Isometric Projection**
    - [Creating Worlds](https://code.tutsplus.com/creating-isometric-worlds-a-primer-for-game-developers--gamedev-6511t)
    - [Map/World Translations](https://github.com/not-fl3/macroquad/pull/598/files)
    - [Math](https://clintbellanger.net/articles/isometric_math/)
- [MDN Game Development](https://developer.mozilla.org/en-US/docs/Games)

### Examples
- [Wrym](https://github.com/wick3dr0se/wrym/tree/main/examples)
- [Secs](https://github.com/wick3dr0se/secs/tree/main/examples)
- **Macroquad**
    - [Awesome](https://github.com/ozkriff/awesome-quads)
    - [Repository](https://github.com/not-fl3/macroquad/tree/master/examples)
    - [Rusty roguelike](https://github.com/rust-gamedev/rust-game-ports/tree/master/rusty_roguelike-macroquad)
- [SDL2](https://github.com/wick3dr0se/sdl-game/)

## Contributing
Fork the repository, push changes to a new branch (e.g. USERNAME/FEATURE) and submit a pull request

There is always something to improve upon or implement. Any contributions are greatly appreciated!

Dyrah is actively discussed in the [Open Source Force Discord](https://opensourceforce.net/discord) community. Join us and check out the Dyrah thread in the showcase forum for easier engagement

## Credits
Thanks to the open source community for making projects like this possible and especially OSF (Open Source Force) for their direct contributions

Thanks to [Seth on itch.io](https://itch.io/profile/sethbb) for the [32rogues](https://sethbb.itch.io/32rogues) tileset used to prototype the development of Dyrah
