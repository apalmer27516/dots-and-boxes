use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::MoveType;

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */

//Character Constants for use with rendering the board
const GRID_DOT: char = '·';
const EMPTY_SPACE: char = ' ';
const VERTICAL_LINE: char = '|';
const HORIZONTAL_LINE: char = '―';

fn digit_to_char(number: &usize) -> char {
    //Not going to code for larger than single digit b/c don't want to spend the effort to format the command line
    match number {
        0 => {'0'},
        1 => {'1'},
        2 => {'2'},
        3 => {'3'},
        4 => {'4'},
        5 => {'5'},
        6 => {'6'},
        7 => {'7'},
        8 => {'8'},
        9 => {'9'},
        _ => {EMPTY_SPACE}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum GameStatus {
    Ready,
	InProgress,
    Surrendered, //Player has resigned
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    // Implement your own game state
    // May be helpful to split this into state for each player, but I think I just need the lines
    pub lines: Vec<Line>,
    //pub player1_moves: Vec<Line>,
    //pub player2_moves: Vec<Line>,
    pub player1_boxes: usize,
    pub player2_boxes: usize,
    pub status: GameStatus,
    pub next_player: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum LineDirection {
    Up,
	Right,
}

//Faster way to put in the comparison trait
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Line {
    pub x: usize, //usize is unsigned so Rust already ensures that the number is not negative.
    pub y: usize,
    pub direction: LineDirection,
}

//Built-in Rust trait to implement an equality determination 
/* impl PartialEq for Piece {
    fn eq(&self, other: &Self)-> bool {
        self.x == other.x && self.y == other.y
    }
} */

// <<DEVCAMP-TODO>> Can extend BOARD_SIZE in the future to be specified when a new game is created
const BOARD_SIZE: usize = 3;
//How do you specific exponents?
const MAX_BOXES: usize = (BOARD_SIZE - 1) * (BOARD_SIZE - 1);

impl Line {
	pub fn is_in_bounds(&self) -> Result<(), String> {
        match self.direction {
            LineDirection::Up => {
                if self.x < BOARD_SIZE && self.y < (BOARD_SIZE - 1) {
                    Ok(())
                } else {
                    Err("Vertical Line is not in bounds: Y must be between 0 and N-1".into())
                }
            },
            LineDirection::Right => {
                if self.x < (BOARD_SIZE - 1) && self.y < BOARD_SIZE {
                    Ok(())
                } else {
                    Err("Horizontal Line is not in bounds: X must be between 0 and N-1".into())
                }
            },
        }
    }

    pub fn is_empty(&self, game_state: &GameState) -> Result<(), String> {
        if game_state.lines.contains(self) {
            Err("Location is not empty".into())
        } else {
            Ok(())
        }
    }

/*     Check if line would complete 1 or 2 boxes (Need to avoid checking out of boundary conditions)
*   Create 6 prototype lines to do the check.
*   A) If new line is Horizontal, Check if box will be completed by checking that line segments 
*       exist for: V:x,y; V:x+1,y; V:x,y-1; V:x+1,y-1; H:x,y+1; and H:x,y-1 
*       
*   B) If new line is Vertical, Check if box will be completed by checking that line segments 
*       exist for: H:x-1,y; H:x-1,y+1; H:x,y; H:x,y+1; V:x-1,y; and V:x+1,y1  */
    pub fn is_part_of_box(&self, game_state: &GameState) -> usize {
        let mut num_boxes = 0;
        match self.direction {
            LineDirection::Right => {
                //Look to see if completes top or bottom box
                if self.y < (BOARD_SIZE - 1) {
                    let l1 = Line {x: self.x, y: self.y, direction:LineDirection::Up};
                    let l2 = Line {x: self.x, y: self.y + 1, direction:LineDirection::Right};
                    let l3 = Line {x: self.x + 1, y: self.y, direction:LineDirection::Up};
                    if game_state.lines.contains(&l1) && 
                    game_state.lines.contains(&l2) &&
                    game_state.lines.contains(&l3) {num_boxes = num_boxes + 1;}
                }
                if self.y > 0 {
                    let l4 = Line {x: self.x + 1, y: self.y - 1, direction:LineDirection::Up};
                    let l5 = Line {x: self.x, y: self.y - 1, direction:LineDirection::Right};
                    let l6 = Line {x: self.x, y: self.y - 1, direction:LineDirection::Up};
                    if game_state.lines.contains(&l4) && 
                    game_state.lines.contains(&l5) &&
                    game_state.lines.contains(&l6) {num_boxes = num_boxes + 1;}     
                }           
                num_boxes
            }
            LineDirection::Up => {
                //Look to see if completes left or right box
                if self.x < (BOARD_SIZE - 1) {
                    let l1 = Line {x: self.x, y: self.y + 1, direction:LineDirection::Right};
                    let l2 = Line {x: self.x + 1, y: self.y, direction:LineDirection::Up};
                    let l3 = Line {x: self.x, y: self.y, direction:LineDirection::Right};
                    if game_state.lines.contains(&l1) && 
                    game_state.lines.contains(&l2) &&
                    game_state.lines.contains(&l3) {num_boxes = num_boxes + 1;}
                }
                if self.x > 0 {
                    let l4 = Line {x: self.x - 1, y: self.y, direction:LineDirection::Right};
                    let l5 = Line {x: self.x - 1, y: self.y, direction:LineDirection::Up};
                    let l6 = Line {x: self.x - 1, y: self.y + 1, direction:LineDirection::Right};
                    if game_state.lines.contains(&l4) && 
                    game_state.lines.contains(&l5) &&
                    game_state.lines.contains(&l6) {num_boxes = num_boxes + 1;}  
                }
                num_boxes
            }
        }
    }
}

impl GameState {
    pub fn initial() -> Self {
        // return an initial state of a game
        Self{
            moves: Vec::new(),
            lines: Vec::new(),
            player1_boxes: 0,
            player2_boxes: 0,
            status: GameStatus::Ready,
            next_player: 2, //usize b/c just going to be an indicator of the player - not the address
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP>> return a pretty formatting string representation
        //Put a newline in the string we will eventually return to initialize it
        let mut disp = "\n".to_string();

        let mut board = [[EMPTY_SPACE; (BOARD_SIZE * 2)]; (BOARD_SIZE * 2)];
        //Add identifying numbers to the grid
        //NOTE: These ranges start inclusive at the beginning and exclusive at the end!
        for y in 0..BOARD_SIZE {
            board[0][(2*y)+1] = digit_to_char(&y);
        }
        for x in 0..BOARD_SIZE {
            board[(2*x)+1][0] = digit_to_char(&x);
        }
        //Populate the grid w/dots on the vertices
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                board[(2*x)+1][(2*y)+1] = GRID_DOT;
            }
        }
        //Draw any lines recorded in the moves
        for l in self.lines.iter() {
            match l.direction {
                LineDirection::Right => {
                    board[(l.x+1)*2][(l.y*2)+1] = HORIZONTAL_LINE;
                }
                LineDirection::Up => {
                    board[(l.x*2)+1][(l.y+1)*2] = VERTICAL_LINE;
                }
            }
        }

        //Format the final string
        for y in (0..(BOARD_SIZE * 2)).rev() {
            for x in 0..(BOARD_SIZE * 2) {
                //disp.push_str(&format!("x={},y={}", x, y));
                disp.push_str(&format!("{}", board[x][y]));
            }
            disp.push('\n');
        }
        disp.push('\n');
        disp.push_str(&format!("Player 1 boxes completed:{}\n", self.player1_boxes));
        disp.push_str(&format!("Player 2 boxes completed:{}\n", self.player2_boxes));
        //Need to do translation by match for game status
        disp.push_str(&format!("Game Status:{}\n", 
            match self.status {
                GameStatus::Ready => {"Ready"},
                GameStatus::InProgress => {"InProgress"},
                GameStatus::Surrendered => {"Surrendered"},
                GameStatus::Completed => {"Completed"},} ));
        disp.push_str(&format!("Next Player's Turn:{}\n", self.next_player));
            
        disp.to_string()
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // <<DEVCAMP>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid

        /*  1) If no moves recorded and 2 players, state is ready
        *   2) If a player has resigned, state is surrendered
        *   3) If all boxes completed, state is Completed. Boxes = (N-1)^2
        *   4) Otherwise it is In Progress: Check if new line would complete a box
        *   If new box then increment player's by 1 or 2 accordingly and flag next turn as theirs. 
        *   Otherwise, flag opposite player's turns as next. */

        //Unpack the move
        let mut moves = self.moves.clone();
        let mut lines = self.lines.clone();
        let mut player1_boxes = self.player1_boxes.clone();
        let mut player2_boxes = self.player2_boxes.clone();
        //Don't assign these values because I don't look at them before overriding.
        let status; //= self.status.clone();
        let next_player; // = self.next_player.clone();

        //Add the new move to the state
        moves.push(next_move.clone());

        match next_move.clone().move_type {
            MoveType::Place{x, y, direction} => {
                let next_line = Line {x, y, direction};
                let num_boxes = next_line.is_part_of_box(self);
                
                //Figure out which player made the move
                if game.player_1 == next_move.author {
                    if num_boxes > 0 {
                        player1_boxes = player1_boxes + num_boxes;
                        next_player = 1;
                    } else {
                        next_player = 2;
                    }

                } else { //player 2
                    if num_boxes > 0 {
                        player2_boxes = player2_boxes + num_boxes;
                        next_player = 2;
                    } else {
                        next_player = 1;
                    }
                }
                lines.push(next_line);
                if player1_boxes + player2_boxes == MAX_BOXES {
                    status = GameStatus::Completed;
                } else {
                    status = GameStatus::InProgress;
                }

                //Finally return the new state
                GameState {
                    moves,
                    lines,
                    player1_boxes,
                    player2_boxes,
                    status,
                    next_player,
                }
            }
            //Add in MoveType of Resigned
        }
    }

}
