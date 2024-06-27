use uuid::Uuid;

use crate::xp::Xp;

#[derive(Debug)]
pub struct Material {
    id: Uuid,
    name: String,

    xp: u64,
}

impl Material {
    pub fn new<S: AsRef<str>>(name: S, xp: u64) -> Self {
        Material {
            id: uuid::Uuid::new_v4(),
            name: String::from(name.as_ref()),

            xp,
        }
    }
}

impl Xp for Material {
    fn xp(&self) -> u64 {
        self.xp
    }
}
