mod combat;
mod pokemon;
mod trainer;

use pokemon::{Type, GrowthRate};
use trainer::Trainer;

fn main() {
    let trainer: Trainer = Trainer::new("Rhyse");
    println!("{}", Type::Dragon as u8);
    println!("{}", GrowthRate::Fast as u8);
    println!("Hello, world!, {} {}", trainer.name, trainer.trainer_id);
}

// What should the API look like?
//
// let charizard: Pokemon = Pokemon::new(Pokedex::Charizard, level: 4, moves: vec![Move::Scratch]);
// charizard.get_stats();
// charizard.level_up();
// 
// let ash: Trainer = Trainer::new("ASH");
// ash.add_to_party(&mut charizard);
