# Dots and Boxes
This game was started as a part of the June 2019 HoloChain DevCamp. I still have other tests to add, other odds and ends, and hopefully a GUI, but here is a functional version that is passing a fairly complete integration test. See below for an overview of the framework this was built within, but first a primer to this particular game.

# Board and Moves
The gameboard is a grid of N x N. Grid points are labeled from a lower left origin using an X and Y axis. Numbering starts from 0. A move represents the creation of a line. It specifies an X,Y coordinate and a direction of Up (draws a vertical line) or Right (draws a horizontal line). //Game, author, and previous move have to also be included with a move to make it unique.//

# Validation Rules
* Status
    * Game must have 2 players
    * Game cannot have more than 2 players
* Location
    * Line segment must not already exist
    * Line segment must be within bounds
* Turns
    * Other player must have moved previously and not completed any boxes Unless there are no moves and then the player #2 goes first.

# State
* Summary Info
    * Number of completed boxes for each player
    * Whose turn is next
    * Game Status: Ready, In Progress, Surrendered, Completed
* Full list of all line segments - grouped horizontally and vertically (Tic-tac-toe grouped by players so that turns could be checked so might have to group both ways.)

## CLI Board representation

```
2·―·―·
   | | |
y 1·―·―·
   | | |
  0·―·―·
   0 1 2
     x
Char Position in rendering matrix for board:
0,0,Right=>2,1
1,1,Right=>4,3
0,0,Up=>1,2
1,1,Up=>3,4
```

**Function:**

Store all moves into a full array for horizontal and vertical line segments indicating true/false if the line exists. //Alternatively: Could store the sparse array of lines. The contains logic will let me find if all the components of the box exist.//

* If no moves recorded and 2 players, state is ready
* If a player has resigned, state is surrendered
* If all boxes completed, state is Completed .Boxes = (N-1)^2
* Otherwise it is In Progress:
    * Check if new line would complete a box
        * If new line is Horizontal, Check if box will be completed by checking that line segments exist for: V:x,y; V:x+1,y; V:x,y-1; V:x+1,y-1; H:x,y+1; and H:x,y-1 (Need to avoid checking out of boundary conditions)
        * If new line is Vertical, Check if box will be completed by checking that line segments exist for: H:x-1,y; H:x-1,y+1; H:x,y; H:x,y+1; V:x-1,y; and V:x+1,y1 (Need to avoid checking out of boundary conditions)
    * If new box then increment player's by 1 or 2 accordingly and flag next turn as theirs. Otherwise, flag opposite player's turns as next.


## Guide to implementing your game

Hopefully you have completed the exercises in [the previous workbook](https://hackmd.io/aGR24Y91Te28dfdYn4WdVw?both) and have a pretty good idea of the state, moves and validation required for your game.

The generic game framework makes it easy to develop new two player simple games on Holochain. All that is required is an implementation of:

#### A `MoveType` enum
This defines the types of moves that can be played in a game. e.g. for checkers we have 

#### A `GameState` struct
This defines that the state of game after a particular number of moves have occurred. This must implement the functions `initial`, `render` and `evolve`

#### Implement `is_valid` for Move

This is used to perform validation. This will be evaulated each time a player makes a move to check it is allowed.

In this workbook we will implement tic-tac-toe within the generic game framework.


## First steps

- **✍️First up you will need to clone the [empty game framework repo](https://github.com/willemolding/generic-game-holochain/pull/new/empty-framework).**

Take a look around and you will see it exposes some entries and zome functions in the `lib.rs`. You will also notice that this project uses multiple rust files to define the zome. It might be worth revisiting [the rust book chapter on packages, crates and modules](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) if you get confused.

All of the places you need to fill in code have been annotated with the tag `<<DEVCAMP-TODO>>`. This should make it easier for you to navigate the codebase. Also note some files have a `DEVCAMP- DO NOT EDIT` header.

## Defining Moves

Lets start with the easiest part. Defining what kind of moves are allowed. In tic-tac-toe there is only one type of move, placing a piece in a position.

- **✍️Follow the steps below to add a move to the `MoveType` enum which represents placing a piece in a position**

![](https://i.imgur.com/BW0Rsfn.gif)

We will revisit the `describe()` function later.

## Defining the Game State

The state of a game of tic-tac-toe is also pretty simple. It is just the location of the pieces places by each player. The state should also store the list of moves made so far.

- **✍️Follow the steps to add a state object for a tic-tac-toe game** 


![](https://i.imgur.com/nuhNtIU.gif)

Obviously the initial game board has no moves and no pieces.

- **✍️Also implement the `initial() function`** 

![](https://i.imgur.com/uOHV4my.gif)

We will also revisit the `render()` function later

### State.evolve()

The state evolution function is generally the most difficult part of implementing a game. It is important to remember that *no validation should be done in this function*. It is safe to assume that the state is already valid as is the next_move.

Evolving the state in tic-tac-toe is easy. The new move must be added to the list of moves and then the piece added to the correct player vec.

- **✍️Follow the steps below to add a state evolution for tic-tac-toe.**

![](https://i.imgur.com/6zxco6A.gif)

## Validation

Validation for moves is defined by implementing your own `is_valid` function on the `Move` struct. 

Before we do that we will add some helper functions to make the validation nice and readable.

- ✍️**Add the following helpers on the Piece struct: `is_in_bounds` and `is_empty`**

![](https://i.imgur.com/kp0n5Ue.gif)

- ✍️**Add a helper function to check if it is a given players turn**

![](https://i.imgur.com/UIHoDUa.gif)

and with these we can write a clean is_valid implementation for move

- ✍️**Implement the `is_valid` function for a move**

![](https://i.imgur.com/RJ1UZYP.gif)

- **✍️Note: Don't forget to add the following imports to the validation.rs file**
```
use crate::your_game::MoveType;
use hdk::holochain_core_types::cas::content::Address;
```

This function will be consumed by the generic-game framework and run before any move entry can be added to the DHT. Because this is run by all agents that might have to hold the entry it makes cheating practically impossible!

## Testing

For this example we will only add end-to-end tests to test the zome functions and validation work correctly.

- **✍️Open up the `test/index.js` file to add tests. Follow each of the below guides to use the testing framework to add a new game, make a valid move and make an invalid move**


Create a new game between Alice and Bob
![](https://i.imgur.com/bBKVBRe.gif)

Bob must make the first move
![](https://i.imgur.com/2kHSCGH.gif)

Alice tries to place a piece in the same place. This should fail our validation
![](https://i.imgur.com/dxtp4Rw.gif)

- **✍️Run the tests by running `hc test` from the nix-shell in the project root.**

## Exercises

#### 1. Add some more tests

Add some more calls to `make_move` such that all of the validation failures are tested (e.g. playing out of turn, playing out of bounds, playing on an occupied position).

Use the `get_state` zome function to check that the game state looks as expected in each case e.g.

```javascript
const game_state = await alice.callSync('main', 'get_state',{
    game_address: create_game_result.Ok
})
```

*Remember you can `console.log` any results to visually inspect them when writing tests*

#### 2. Add win conditions

The current implementation doesn't know when the game is over.
- Add a new field to the state struct which encodes if/which player has won
- Add some extra logic to the state evolution function which checks all the moves made and updates the state with the victory
- The validation should not allow any moves to be made after the game has been won

#### 3. Add a `Resign` move

The current implementation only has a single move type `Place`. Add another move type variant, `Resign`, which takes no parameters. On detecting a resign move the state evolution function should be updated to say the other player has won

