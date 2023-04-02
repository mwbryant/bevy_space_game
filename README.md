# Bevy Space Game

This is a [Bevy](https://bevyengine.org/) game which runs a cellular automata based gas simulation.  Multiple gases including O2 and CO2 are free to flow and diffuse around room and respect walls which can be dynamically changed at runtime.  The player entity also inhales O2 and exhales CO2 and visualizations can be activated to show the different gases in a room.  Gases are all modeled with the ideal gas law and moles are conserved through all operations.  

![Example Gif](gifs/gas_sample.gif)

The game also features pixel perfect click detection and a particle system implementation. Clicking will currently both place walls and print what object in the game world you clicked on.  Right clicking will destroy walls allowing gases to flow.

This follows the devlogs at [LogicProjects on Youtube](https://www.youtube.com/watch?v=z62OTMVL6Xhttps://www.youtube.com/watch?v=z62OTMVL6X00).
All code and art was created by LogicProjects and are free to use in any way without restriction.  

This is meant to be an educational project and you should feel free to use any code in your own projects!

# Usage

```
cargo run
```
