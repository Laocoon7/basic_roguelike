# Basic Roguelike
This is less of a tutorial, and more of a journal. Just wanted to make a super basic roguelike I can build off of and tinker with later on using the libraries I enjoy using. It's probably neither the best code, nor the best design for making one.

## How to Write a Roguelike in 15 Steps
https://roguebasin.com/index.php/How_to_Write_a_Roguelike_in_15_Steps
I intend on following these steps in each git branch.

## Step 3 - It's a boy!
```
Start with screen output and keyboard input. Decide preemptively on your screen layout (easy, you can change it later) and make the routines that display map characters, status lines and messages.

Make the routine to read key-presses (no config files, no keys redefinition, Preferred Key Controls).

Make a '@ walking around the empty screen' demo. Play with it a little, clean things up, play some more, imagining the game is finished and you're playing it.

Make the message handling routines -- especially for the debugging messages -- they come in handy.
```
### Controller module:
- Input Handling

### Model module:
- Player Tag
- Position component
- Spawn Player

### Ui module:
- Screen Layout
- Camera Spawning
- GameLog
- Stats Panel

### View module:
- add sprites to actors/player
- move sprites around based on their position