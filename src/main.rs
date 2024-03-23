mod combat;
mod pokemon;
mod trainer;

use pokemon::{BaseStats, Pokemon, Stats, MoveSet, GrowthRate, PokemonSpecies, Type};
use trainer::Trainer;

fn main() {

    let trainer: Trainer = Trainer::new("Rhyse");
    println!("Hello, world!, {:?}", trainer);

    let ivs = BaseStats::generate_ivs();
    println!("{:?}", ivs);

    let mut evs = Stats::new(100, 100, 100, 100, 100);
    println!("{:?}", evs);

    let opponent_stats = BaseStats::new(100, 100, 100, 100, 100); 
    evs.calc_new_evs(&opponent_stats);
    println!("{:?}", evs);

    let base_stats = BaseStats::new(10, 5, 6, 8, 7);
    let initial_moves = MoveSet::empty();
    let charizard_base = PokemonSpecies::new(3, "Charizard".to_string(), base_stats, Type::Fire, Type::Flying, 200, 200, GrowthRate::Fast, initial_moves);
    let charizard = Pokemon::new(charizard_base, 82, trainer, None, None).unwrap_or_else(|err| panic!("{}", err));
    println!("{}", charizard.exp_for_next_level());

}

// What should the API look like?
//
// let charizard: Pokemon = Pokemon::new(Pokedex::Charizard, level: 4, moves: vec![Move::Scratch]);
// charizard.get_stats();
// charizard.level_up();
// 
// let ash: Trainer = Trainer::new("ASH");
// ash.add_to_party(&mut charizard);
