use db::types::PokemonModel;

pub mod data;
pub mod types;

/// Fetches a Pokemon model from the Pokedex specified by its name. If there
/// is no Pokemon with the given name in the Pokedex, `None` is returned.

pub fn pokemon_by_name(name: &str) -> Option<&'static PokemonModel> {
    for model in data::POKEDEX {
        if model.name == name {
            return Some(model);
        }
    }
    None
}

