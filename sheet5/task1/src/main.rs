pub mod db;
mod engine;
mod game;

/// Initiates a pokemon fight between player red and player blue
fn main() {
    game::fight("Red", "Blue");
}