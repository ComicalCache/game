use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug)]
pub struct Artifact {
    id: Uuid,
    name: String,

    xp: u64,
    level: u32,
}

#[cfg(feature = "cmdln")]
impl Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
