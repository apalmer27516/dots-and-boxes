
use crate::game::Game;
use crate::game_move::Move;
use crate::your_game::MoveType;
use crate::your_game::state::Line; 
//This is an older reference, but is still in the API doc
//use hdk::holochain_core_types::cas::content::Address;
use hdk::{
    holochain_persistence_api::{
        cas::content::{Address},
    }
};
use super::{
    GameState,
};


/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 * 
 * This function  takes the current game and the game state (which includes all the existing moves) 
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 * 
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */

impl Move {
	pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        //Check if a move is valid given the current game and its state
        is_players_turn(self.author.clone(), &game, &game_state)?; //"?" operator bails early if error
        match self.move_type.clone() {
            MoveType::Place{x, y, direction} => {
                let pos = Line{x, y, direction};
                pos.is_in_bounds()?;
                pos.is_empty(&game_state)?;
                Ok(()) // if we made it this far success!
            }
        }
    }
}

//Another helper for checking if it is the player's turn. Players take turns except for when
//1 or 2 boxes is completed by a line and then that player gets to go again.
fn is_players_turn(player: Address, game: &Game, game_state: &GameState) -> Result<(), String> {
	let moves = &game_state.moves;
    //Returns 0 if no moves or 1 if more
    match moves.last() {
        Some(last_move) => {
            match last_move.move_type.clone() {
                MoveType::Place{x, y, direction} => {
                    let next_line = Line {x, y, direction};
                    if next_line.is_part_of_box(game_state) > 0 {
                        if last_move.author != player {
                            Err("Other player completed 1-2 boxes and gets another turn.".into())
                        } else {
                            Ok(())
                        }
                    } else if last_move.author == player {
                        Err("it is not this players turn".into())
                    } else {
                        Ok(())
                    }
                }
            }
        },
        None => { //also need to handle the case where no moves have been made yet
            if game.player_2 == player {
                Ok(()) // Player 2 can go first by convention 
            }
            else {
                Err("Player 2 must make the first move".into())
            }
        }
    }
}