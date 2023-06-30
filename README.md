# Annabel-Rust
Developers: David Baker-Robinson, Leo Lu, Daniel Trang  
CS410P: Programming in Rust  

## How to run the game
```
cargo run
```
## About this Game  
This Bevy-Rust version of our game, Annabel, has been ported from our original efforts in Godot 4.0 written in GDscript. It is not a 1-to-1 representation, however it serves as an example of what can be done using Bevy's game engine.

## What we built (... so far)
We have used the Bevy ECS game engine to build a simalacrum of a game we built in Godot. It currently implements
collisions, event handling, conditional spawning and unspawning game logic, and custom sprites. We learned alot 
about how to code in a paradigm that was quite different that OOP game engines like Godot and managed to 
learn a lot of Rust as well! Even though the Bevy system is primarily ECS and uses a flat data structure to quickly access
in-world entities it also has a version of a hierarchy that allows child entities to be added to parent entities and
then uses pointers to refer to each other. The game's objective is to collect as many flowers as possible without
dying. You have to avoid the thorn attacks while collecting the flowers. You have a specific amount of health
and will die if struck by too many thorns. The positive flowers are spawned randomly throughout the level.

## What didn't go so well
There were limited tools that integrated with Bevy smoothly. Of the ones we've found, most of them were still young in development and fixing bugs. For example a tool for creating tilemaps: LDTK. This is one of the more popular and user-friendly tools which supports Bevy. However, as you can see below, the JSON file exported from their software still has bugs with more complex tilesets that require padding, offset, and non-standard settings.   

<img src="https://i.ibb.co/XWwbD6V/expected.png" width="400" height="240">  
<img src="https://i.ibb.co/bPYgJbc/shattered.png" width="400" height="240">  

(The left image is our custom map created in LDTK; the right image is after importing into Bevy... **kaboomed**)

Also, the final score would only show sometimes, and logs do not indicate clearly why. Possibly a concurrency issue.
<img src="https://media.discordapp.net/attachments/1067002426957254657/1119165111244951663/image.png?width=1136&height=905" width="380" height="300">

Alot of the development process was based on reading the docs and there was a limited number of youtube tutorials that
applied to our project. So we ended up going through example projects, bevy_cheatbook, and bevy_docs to learn
how to use Bevy.

## Reflections  
Major roadblocks include Bevy's ever-changing APIs as it is still in it's infancy. Bevy is version sensitive and not very backward compatible. Much of our code is hacked together from what limited resources and documentation is published by Bevy's small community of developers.

As of this writing, we are using Bevy version 0.10 which came with many changes to its ECS model. More specifically, there has been upgrades to the startup system order implementation which affect the order in which plugins and dependencies must be added to the system. We also saw a big loss in useful plugin & tools that were supported by Bevy 0.7-8 but fell out of touch due to the lack of maintaining developers. 

In order to make progress, we've stuck with vanilla Bevy plugins and looked for plugins & tools that were stable in the last few versions. In cases where we couldn't find better options, we've also spent countless hours taking old coding patterns and upgrading them to the latest methods by manually sifting through version change documentation. Suffice to say, it has been a painful experience.

Nonetheless, we see the potential for Bevy's ECS model to provide a fast, easy to use, modular approach to designing games. When Bevy reaches a more stable state, it could be a great game engine for indie developers.