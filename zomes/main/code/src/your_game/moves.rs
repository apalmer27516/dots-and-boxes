use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};
use crate::your_game::state::LineDirection;

/**
 *
 * The MoveType enum defines all the types of moves that are valid in your game and the 
 * data they carry. The gameboard is a grid of N x N. Grid points are labeled from a lower 
 * left origin using an X and Y axis. Numbering starts from 0. A move represents the 
 * creation of a line. It specifies an X,Y coordinate and a direction of Up 
 * (draws a vertical line) or Right (draws a horizontal line).
 *
 */

//<<DEVCAMP-TODO>>: Add in "Resign" as a MoveType

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
    Place {x: usize, y: usize, direction: LineDirection }
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
		// <<DEVCAMP-TODO>> SHOULD RETURN AN EXAMPLE OF EACH VARIANT
		vec![MoveType::Place{x:0,y:0,direction:LineDirection::Up}]
	}
}
