use rand::random;
use crate::pokemon::Pokemon;

#[derive(Debug)]
pub struct Trainer {
    pub name: String,
    pub trainer_id: u16,
    pub party: Vec<Pokemon>
}

impl Trainer {

    pub fn new(name: &str) -> Self {
        let trainer_id = random::<u16>();
        Trainer {
            name: String::from(name),
            trainer_id,
            party: vec![]
        }
    }

}
