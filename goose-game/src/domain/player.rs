use super::Location;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Player {
    name: String,
    location: Location,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            location: Location::start(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn set_location(&mut self, new_location: Location) {
        self.location = new_location;
    }
}
