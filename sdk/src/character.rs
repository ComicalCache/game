use std::collections::BTreeMap;

use crate::{
    stats::{
        create::{defense, health_mana, lvl_xp, simple},
        Stat, StatType,
    },
    Inventory,
};

use uuid::Uuid;

pub struct Character {
    id: Uuid,
    name: String,

    stats: BTreeMap<StatType, Stat>,

    inventory: Inventory,
}

impl Character {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Character {
            id: uuid::Uuid::new_v4(),
            name: String::from(name.as_ref()),

            stats: BTreeMap::from([
                lvl_xp!(Level, 1, 0),
                health_mana!(Health, 1000, 1000, 2.0),
                health_mana!(Mana, 350, 350, 1.5),
                simple!(AttackDamage, 0),
                simple!(AttackSpeed, 1.0),
                simple!(MagicDamage, 0),
                defense!(0, 0, 0.0),
                lvl_xp!(Strength, 1, 0),
                lvl_xp!(Wisdom, 1, 0),
                lvl_xp!(Crafting, 1, 0),
                lvl_xp!(Enhancement, 1, 0),
                lvl_xp!(Combat, 1, 0),
                lvl_xp!(Magic, 1, 0),
                lvl_xp!(Luck, 1, 0),
            ]),

            inventory: Inventory::default(),
        }
    }
}
