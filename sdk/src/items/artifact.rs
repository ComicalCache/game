use uuid::Uuid;

#[derive(Debug)]
pub struct Artifact {
    id: Uuid,
    name: String,

    xp: u64,
    level: u32,
}
