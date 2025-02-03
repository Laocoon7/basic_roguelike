# Basic Roguelike
This is less of a tutorial, and more of a journal. Just wanted to make a super basic roguelike I can build off of and tinker with later on using the libraries I enjoy using. It's probably neither the best code, nor the best design for making one.

## How to Write a Roguelike in 15 Steps
https://roguebasin.com/index.php/How_to_Write_a_Roguelike_in_15_Steps
I intend on following these steps in each git branch.

## Step 4 - The map
```
Decide on your map structure. Don't over-generalize -- you can add things later. Make your (empty) map displayed on the screen. Scrolled if you need it. Add some elements to your map to see if they are displayed correctly (just hard-code them, you don't need a level generator yet).

Make your '@' appear on the map -- not as it's element at first, just as a cursor. Test the scrolling, maybe make a 'look' command.

Now turn '@' into creature. You still don't have time implemented, so the keyboard-reading routines will just move it around the map. Include the checks for walls in the movement code. Maybe add doors and open/close commands.
```
Made a simple Map which holds entities. We create entities and give each a TileType and a position. We'll also keep track of actors positions on the map. Now that there is terrain to look at, we can have the camera follow the player.

To handle walls, we can turn move into an entity command. Actions will need access to the world, and each action will be performed by an entity, so this feels right.

In order to create a `look` command, we need to describe something to look at so we have a `Description` component. We also need to track the look targeting state. Since targeting is a different set of input, we'll just have a different state for each type of targeting we want to do (only looking for now). This will allow us to create a group of entities for selection, draw an outline around each, move them without moving the player, and get their positions once we are done targetting.

It's quite a rough system, but it's working..