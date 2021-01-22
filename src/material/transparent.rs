use super::Material;

#[derive(Clone)]
pub struct Transparent {}

impl Transparent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Transparent {}
