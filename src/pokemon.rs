use rand::random;

use crate::combat::Move;
use crate::trainer::Trainer;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum GrowthRate {
    MediumFast = 0,
    MediumSlow = 3,
    Fast = 4,
    Slow = 5,
}

#[derive(Debug)]
pub enum Status {
    Asleep = 0x04,
    Poisoned = 0x08,
    Burned = 0x05,
    Frozen = 0x20,
    Paralyzed = 0x40,
}

/// The base stats for a given Pokemon species
/// This is also used to store IVs
#[derive(Copy, Clone, Debug)]
pub struct BaseStats {
    hp: u8,
    attack: u8,
    defense: u8,
    speed: u8,
    special: u8
}

impl BaseStats {

    pub fn new(hp: u8, attack: u8, defense: u8, speed: u8, special: u8) -> Self {
        Self { hp, attack, defense, speed, special }
    }

    pub fn generate_ivs() -> Self {

        let attack = random::<u8>() % 16;
        let defense = random::<u8>() % 16;
        let speed = random::<u8>() % 16;
        let special = random::<u8>() % 16;

        let hp = (attack & 1) << 3 | (defense & 1) << 2 | (speed & 1) << 1 | (special & 1);

        BaseStats {
            hp,
            attack,
            defense,
            speed,
            special,
        }

    }

}

impl Into<Stats> for BaseStats {
    fn into(self) -> Stats {
        Stats {
            hp: self.hp as u16,
            attack: self.attack as u16,
            defense: self.defense as u16,
            speed: self.speed as u16,
            special: self.special as u16,
        }
    }
}

/// This is also used to store EVs
#[derive(Debug)]
pub struct Stats {
    hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special: u16
}

impl Stats {

    pub fn new(hp: u16, attack: u16, defense: u16, speed: u16, special: u16) -> Self {
        Self { hp, attack, defense, speed, special }
    }

    pub fn calc_new_evs(&mut self, opponent_stats: &BaseStats) {
        self.hp += opponent_stats.hp as u16;
        self.attack += opponent_stats.attack as u16;
        self.defense += opponent_stats.defense as u16;
        self.speed += opponent_stats.speed as u16;
        self.special += opponent_stats.special as u16;
    }

}

#[derive(Debug)]
pub struct PokemonSpecies {
    pokedex_number: u8,
    name: String,
    base_stats: BaseStats,
    type1: Type,
    type2: Type,
    catch_rate: u8,
    base_exp_yield: u8,
    growth_rate: GrowthRate,
    initial_moves: MoveSet,
}

impl PokemonSpecies {

    // TODO: Have a `from_name()` method
    // TODO: Have a `from_pokedex_id` method

    pub fn new(pokedex_number: u8, name: String, base_stats: BaseStats, type1: Type, type2: Type, catch_rate: u8, base_exp_yield: u8, growth_rate: GrowthRate, initial_moves: MoveSet) -> Self {
    
        Self {
            pokedex_number,
            name,
            base_stats,
            type1,
            type2,
            catch_rate,
            base_exp_yield,
            growth_rate,
            initial_moves
        }

    }

}
#[derive(Copy, Clone, Debug)]
pub struct MoveSet {
    move1: Option<Move>,
    move2: Option<Move>,
    move3: Option<Move>,
    move4: Option<Move>,
    move1_pp: u8,
    move2_pp: u8,
    move3_pp: u8,
    move4_pp: u8,
}

impl MoveSet {

    pub fn empty() -> Self {
        Self {
            move1: None,
            move2: None,
            move3: None,
            move4: None,
            move1_pp: 0,
            move2_pp: 0,
            move3_pp: 0,
            move4_pp: 0,
        }
    }

    pub fn new(
        move1: Option<Move>,
        move2: Option<Move>,
        move3: Option<Move>,
        move4: Option<Move>,
        move1_pp: u8,
        move2_pp: u8,
        move3_pp: u8,
        move4_pp: u8,
    ) -> Self {
        Self { move1, move2, move3, move4, move1_pp, move2_pp, move3_pp, move4_pp }
    }

    pub fn replace_move(&mut self, index: usize, new_move: Option<Move>) {

        let (move_ref, move_pp_ref) = match index {
            1 => ( &mut self.move1, &mut self.move1_pp ),
            2 => ( &mut self.move2, &mut self.move2_pp ),
            3 => ( &mut self.move3, &mut self.move3_pp ),
            4 => ( &mut self.move4, &mut self.move4_pp ),
            _ => panic!("Invalid move index!")
        };

        *move_ref = new_move;
        *move_pp_ref = 0; // TODO: Set this to the default PP of the move if Some(Move)

    }

}
/// Represent a player's caught Pokemon
#[derive(Debug)]
pub struct Pokemon {
    species: PokemonSpecies,
    current_hp: u16,
    level: u8,
    stats: Stats,
    status: u8,
    original_trainer: Trainer,
    nickname: Option<String>,
    exp: u32,
    evs: Stats,
    ivs: BaseStats,
    moveset: MoveSet,
}

impl Pokemon {

    pub fn new(species: PokemonSpecies, level: u8, original_trainer: Trainer, moveset: Option<MoveSet>, nickname: Option<String>) -> Self {

        let moves = match moveset {
            Some(moves) => moves,
            None => species.initial_moves
        };
        let ivs = BaseStats::generate_ivs();
        let evs = Stats::new(0, 0, 0, 0, 0);
        let stats: Stats = species.base_stats.into();
        let status = 0;
        let mut pokemon = Self {
            species,
            current_hp: stats.hp,
            level: 1,
            stats,
            status,
            original_trainer,
            nickname,
            exp: 0,
            evs,
            ivs,
            moveset: moves
        };

        for _ in 1..level {
            pokemon.level_up();
        }

        pokemon

    }

    pub fn set_status(&mut self, status: Status) {
        self.status |= status as u8;
    }

    pub fn level_up(&mut self) {

        self.level += 1;
        self.stats.hp = Self::calc_level_up_hp(self.level, self.species.base_stats.hp, self.ivs.hp, self.evs.hp);
        self.stats.attack = Self::calc_level_up_stat(self.level, self.species.base_stats.attack, self.ivs.attack, self.evs.attack);
        self.stats.defense = Self::calc_level_up_stat(self.level, self.species.base_stats.defense, self.ivs.defense, self.evs.defense);
        self.stats.speed = Self::calc_level_up_stat(self.level, self.species.base_stats.speed, self.ivs.speed, self.evs.speed);
        self.stats.special = Self::calc_level_up_stat(self.level, self.species.base_stats.special, self.ivs.special, self.evs.special);

    }

    pub fn calc_level_up_hp(level: u8, base: u8, iv: u8, ev: u16) -> u16 {
        Self::calc_level_up_stat(level, base, iv, ev) + level as u16 + 5
    }

    pub fn calc_level_up_stat(level: u8, base: u8, iv: u8, ev: u16) -> u16 {

        let base_default = (base + iv) * 2;

        let mut stat_sqrt = f64::sqrt(ev.into());
        stat_sqrt = f64::ceil(stat_sqrt) / 4.0;
        stat_sqrt = f64::floor(stat_sqrt);
        
        let mut new_stat = (base_default as f64 + stat_sqrt) * level as f64;
        new_stat = f64::floor(new_stat / 100.0);
        new_stat as u16 + 5

    }
}
