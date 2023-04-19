#[derive(Clone, PartialEq)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
