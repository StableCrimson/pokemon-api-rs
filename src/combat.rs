use crate::{
    pokemon::{Pokemon, Status},
    trainer::Trainer,
};
use rand::random;

#[derive(PartialEq)]
pub enum PokeBall {
    Normal,
    Great,
    Ultra,
    Master,
}

#[derive(Copy, Clone, Debug)]
pub enum MoveType {
    Normal,
    Status,
    Special,
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    // name: String,
    move_type: MoveType,
    accuracy: u8,
}

fn is_crit() -> bool {
    false
}

fn did_move_hit() -> bool {
    todo!()
}

fn calc_damage() -> u16 {
    todo!()
}

pub fn calc_exp_gain(trainer: &Trainer, winner: &Pokemon, loser: &Pokemon) -> u16 {
    let trainer_battle_bonus = if loser.is_wild() { 1.0 } else { 1.5 };

    // TODO: This should check if the pokemon is holding the Lucky Egg or not (then it would be
    // 1.5)
    let egg_modifier = 1.0;
    let loser_level = loser.level as f64;
    let base_yield = loser.species.base_exp_yield as f64;

    // TODO: Check if Exp all is in the players bag
    // if it isn't:
    //  Number of pokemon that participated but haven't fainted
    // if it is:
    //  https://bulbapedia.bulbagarden.net/wiki/Experience#In_the_core_series
    let portion = 1.0;

    let outsider_bonus = if winner.is_outsider(trainer.trainer_id) {
        1.5
    } else {
        1.0
    };

    let exp = ((base_yield * loser_level) / 7.0)
        * (1.0 / portion)
        * egg_modifier
        * trainer_battle_bonus
        * outsider_bonus;

    exp as u16
}

pub fn attack() {}

pub fn catch(pokeball: PokeBall, pokemon: &Pokemon) -> bool {
    if pokeball == PokeBall::Master {
        return true;
    }

    let n = match pokeball {
        PokeBall::Normal => random::<u8>(),
        PokeBall::Great => random::<u8>() % 200,
        _ => random::<u8>() % 150,
    };

    let mut status_threshold = 0;

    if pokemon.has_status_effect(Status::Paralyzed) ||
    pokemon.has_status_effect(Status::Poisoned) ||
    pokemon.has_status_effect(Status::Burned) {
        status_threshold = 12;
    }

    if pokemon.has_status_effect(Status::Asleep) ||
    pokemon.has_status_effect(Status::Frozen) {
        status_threshold = 25;
    } 

    if n < status_threshold {
        return true;
    }

    if n - status_threshold > pokemon.species.catch_rate {
        return false;
    }

    let m = random::<u8>();

    let ball_modifier = match pokeball {
        PokeBall::Great => 8,
        _ => 12,
    };

    let f =
        (pokemon.stats().hp as u64 * 255 * 4) as u8 / (pokemon.current_hp * ball_modifier) as u8;

    if f >= m {
        return true;
    }

    // let shake_modifier = match pokeball {
    //     PokeBall::PokeBall => 255,
    //     PokeBall::GreatBall => 200,
    //     _ => 150,
    // };
    //
    // let d = (pokemon.species.catch_rate() as u16 * 100) / shake_modifier;
    //
    // if d >= 256 {
    //     println!("The ball shakes 3 times");
    // }
    //
    // let s = if pokemon.has_status_effect(Status::Asleep) {
    //     10
    // } else if pokemon.has_status_effect(Status::Frozen) {
    //     10
    // } else if pokemon.has_status_effect(Status::Paralyzed) {
    //     5
    // } else if pokemon.has_status_effect(Status::Poisoned) {
    //     5
    // } else if pokemon.has_status_effect(Status::Burned) {
    //     5
    // } else {
    //     0
    // };
    //
    // let x = ((d * f) / 255) + s;
    //
    // if x < 10 {
    //     println!("The ball misses completely");
    // } else if x < 30 {
    //     println!("The ball shakes once, but the pokemon breaks free")
    false
}
