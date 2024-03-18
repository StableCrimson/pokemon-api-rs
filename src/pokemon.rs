use crate::combat::Move;
use crate::trainer::Trainer;

pub enum Type {
    Normal = 0x00,
    Fighting = 0x01,
    Flying = 0x02,
    Poison = 0x03,
    Ground = 0x04,
    Rock = 0x05,
    Bird = 0x06,
    Bug = 0x07,
    Ghost = 0x08,
    Unused = 0x09,
    Fire = 0x14,
    Water = 0x15,
    Grass = 0x16,
    Electric = 0x17,
    Psychic = 0x18,
    Ice = 0x19,
    Dragon = 0x1A,
}

pub enum GrowthRate {
    MediumFast = 0,
    MediumSlow = 3,
    Fast = 4,
    Slow = 5,
}

pub enum Status {
    Asleep = 0x04,
    Poisoned = 0x08,
    Burned = 0x05,
    Frozen = 0x20,
    Paralyzed = 0x40,
}
pub struct PokemonSpecies {
    pokedex_number: u8,
    base_hp: u8,
    base_attack: u8,
    base_defense: u8,
    base_speed: u8,
    type1: Type,
    type2: Type,
    catch_rate: u8,
    base_exp_yield: u8,
    growth_rate: u8,
    initial_moves: Vec<Move>,
}

/// Represent a player's caught Pokemon
pub struct Pokemon {
    species: PokemonSpecies,
    current_hp: u16,
    level: u8,
    status: u8,
    move1: Move,
    move2: Move,
    move3: Move,
    move4: Move,
    original_trainer: Trainer,
    nickname: String,
    exp: u32,
    hp_ev: u16,
    attack_ev: u16,
    defense_ev: u16,
    speed_ev: u16,
    special_ev: u16,
    iv: u16,
    move1_pp: u8,
    move2_pp: u8,
    move3_pp: u8,
    move4_pp: u8,
    max_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special: u16,
}

impl Pokemon {

    fn set_status(&mut self, status: Status) {
        self.status |= status as u8;
    }

}
