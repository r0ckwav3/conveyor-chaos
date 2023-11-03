# conveyor-chaos
A small game made in rust with [GGEZ](https://ggez.rs/).

In Conveyor Chaos, use a set of suprisingly versitile conveyor belts to merge, split and manipulate objects to your will. Heavily inspired by Zachtronics games such as Opus Magnum and Molek Syntez.

## Roadmap
- [X] Barebones Solution Editor
- [X] Fully functional solution simulator
  - [X] Pushing
  - [X] Priority
  - [X] Merging
  - [X] Splitting
  - [X] Output
- [ ] All the tiles/tools
  - [X] Repeating Inputs
  - [X] Rotation
  - [ ] Delay tiles
  - [X] Alternating tiles
- [ ] Tutorial (probably just in a markdown file in this repo)
- [ ] Better Win and Error States
  - [X] Popup box
  - [X] Error location indicators
  - [ ] Non Runtime errors (like not having placed all inputs and outputs)
- [ ] Multiple Levels
- [ ] A menu and stuff
- [ ] Better Textures
  - [X] Better Tile Textures
  - [ ] Better Background Textures
  - [ ] Better Menu Textures
- [ ] In game tutorial
- [ ] Improve code structure using the Drawable Trait
- [ ] Whatever else I want if I'm still working on this project

## Known bugs
 - Rotating a held tile will also rotate the tile you are hovering over.
