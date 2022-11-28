use uuid::Uuid;

pub struct IdGeneratorAutoincrement {
    value: usize,
}

pub struct IdGeneratorUuid {}


impl IdGeneratorAutoincrement {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn inc(&mut self) -> usize {
        self.value += 1;
        return self.value;
    }
}

impl IdGeneratorUuid {
    pub fn new() -> Self {
        Self {}
    }

    pub fn gen(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
