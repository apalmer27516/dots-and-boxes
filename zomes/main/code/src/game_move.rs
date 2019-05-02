use hdk::{
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData
};

use crate::game_state::GameState;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Pos {
	x: u32,
	y: u32
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct MoveInput {
	pub game: Address,
	pub move_type: String,
	pub from: Pos,
	pub to: Pos,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Move {
	pub game: Address,
	pub author: Address,
	pub move_type: String,
	pub from: Pos,
	pub to: Pos,
	pub previous_move: Address
}

impl Move {
	fn is_valid(&self, _game_state: GameState) -> Result<(), String> {
		Ok(())
	}
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "move",
        description: "A move by an agent in an game",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | validation_data: hdk::EntryValidationData<Move>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data: _} => {
                    Move::from(entry).is_valid(GameState::new())
                },
                _ => {
                    Err("Cannot modify or delete a move".into())
                }
            }
        }
    )
}