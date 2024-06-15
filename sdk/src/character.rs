use std::{collections::BTreeMap, fmt::Display};

use crate::{
    stats::{
        create::{defense, health_mana, lvl_xp},
        Stat, StatType,
    },
    utils::cmdln,
    Inventory,
};

use uuid::Uuid;

pub struct Character {
    id: Uuid,
    name: String,

    stats: BTreeMap<StatType, Stat>,

    inventory: Inventory,
}

#[cfg(feature = "cmdln")]
impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write; // for writing to string

        let mut out = String::new();

        // UUID in '[xXxXxX...XxXxXx]' format
        writeln!(out, "{}", cmdln::shorten_uuid(self.id))?;

        // Name
        const NAME_LABEL: &str = "Name";
        writeln!(out, "{NAME_LABEL}: {}", self.name)?;

        // Stats header
        const STATS_TITLE: &str = " STATS ";
        writeln!(out, "\n {:=^1$} ", STATS_TITLE, cmdln::config::BAR_WIDTH)?;

        for (_, stat) in self.stats.iter() {
            writeln!(out, "{stat}\n")?;
        }

        // remove last two linebreaks
        out.pop();
        out.pop();

        write!(f, "{out}")
    }
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
                defense!(20, 20, 0.0),
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
