pub struct Grass {
    pub growth: u8,
}

impl Grass {
    pub fn new() -> Self {
        Self {
            growth: 1,
        }
    }

    pub fn tick(&mut self) {
        if self.growth < 5 {
            self.growth += 1;
        }
    }
}