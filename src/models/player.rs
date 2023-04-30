
#[derive(Clone)]
pub struct Player {
    pub username: String,
    pub score: u16
}

impl Player {
    pub fn new(username: String) -> Player {
        Player {
            username,
            score: 0
        }
    }
}