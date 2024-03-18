pub struct Trainer {
    pub name: String,
    pub trainer_id: u16,
    // pub party: Vec<Pokemon>
}

impl Trainer {

    pub fn new(name: &str) -> Self {
        let trainer_id = rand::random::<u16>();
        Trainer {
            name: String::from(name),
            trainer_id,
        }
    }

}
