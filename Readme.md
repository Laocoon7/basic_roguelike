# Basic Roguelike
This is less of a tutorial, and more of a journal. Just wanted to make a super basic roguelike I can build off of and tinker with later on using the libraries I enjoy using. It's probably neither the best code, nor the best design for making one.

## How to Write a Roguelike in 15 Steps
https://roguebasin.com/index.php/How_to_Write_a_Roguelike_in_15_Steps
I intend on following these steps in each git branch.

## Step 1 - Decide to write a game
```
Most of you will have this step behind you already, but there are some hints about the direction of the first step. The best reason to start developing your own roguelike game is to create a game that you will enjoy playing yourself.

Don't start by asking around about the definition of roguelike -- you don't need it. If the game you write is not considered roguelike by others, but it's still fun to play for you -- you succeeded. It's not like there's some kind of a contest to write a game meeting certain specifications.

Don't plan too much. Of course, if you want certain things in your game, you must write it so that there is room for them -- but don't even try to anticipate everything -- it's just impossible. When you write a design doc, you make a lot of decisions. Most of the decisions can't be made without performing some test first -- usually by writing small programs utilizing a given feature. It's best to make the decisions when your project has already reached the appropriate stage -- when you don't need to write an additional program, because your project already has everything you need.

It's no fun to just proceed according to plan -- leave some space for improvisation. Don't be afraid about making mistakes or implementing something in an inflexible way -- you can improve it when you need it -- most of the time it will be okay.
```
I have spent years designing and thinking about making roguelikes. I've never made it very far as all the designing and thinking makes my scope so large. I'm working on this.

## Step 2 - Hello world!
```
Prepare your programming environment. Choose your language and platform, choose appropriate compilers/interpreters, editor, version control, automated build, and other utilities. Set them up so that you're comfortable with them.

Decide on which libraries you're going to use -- it can change later, but it's usually a painful change. Don't think too much about portability -- it can be fixed later. Think about what you need and what you're comfortable with.

Decide on the language you want to write comments and object names in your code, as well as the language you want to be used in your game. It's strongly recommended to use English in your source code -- you can get more help this way from others.

Don't worry about i18n yet; translation is usually a very late step in the development process.

Write a simple 'Hello world!' program and test whether it works. Test your libraries, etc. -- you don't want any surprises.

Start coding.
```
Check out `Cargo.toml` to see many of the libraries I'll be using. The biggest is [Bevy](https://github.com/bevyengine/bevy), but there will be others. [brt](https://github.com/Laocoon7/brt) is a toolkit I tinker with and so I normally point it to my local version. I've setup Bevy with some optimizations, .vscode with some helpers, I've also broken the code down into a MVC approach.

Writing code for ECS' is great and keeps file sizes small, but instead it grows directory sizes. I have found that by attempting to separate out the model from the view I can shrink the number of files in directories. This will probably change, but I like it for now..