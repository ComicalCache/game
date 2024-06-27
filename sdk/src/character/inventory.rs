use crate::items::{Armor, Artifact, Consumable, Material, Weapon};

#[derive(Default, Debug)]
pub struct Inventory {
    armor: Vec<Armor>,
    artifacts: Vec<Artifact>,
    consumables: Vec<Consumable>,
    materials: Vec<Material>,
    weapons: Vec<Weapon>,

    base_currency: u64,
    premium_currency: u64,
}
