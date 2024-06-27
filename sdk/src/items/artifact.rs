use std::{cmp::min, collections::BTreeMap};

use uuid::Uuid;

use crate::{
    bonus_stats::{BonusStat, BonusStatType},
    XpCurve,
};

use super::Material;

#[derive(Debug)]
pub struct Artifact {
    id: Uuid,
    name: String,

    pub xp: u64,
    pub level: u32,

    main_stat_type: BonusStatType,
    main_stat: BonusStat,
    sub_stats: BTreeMap<BonusStatType, BonusStat>,
}

impl XpCurve for Artifact {
    fn xp_to_next_level(level: u32) -> u64 {
        const BASE_XP: f64 = 1000.0;
        const LINEAR_FACTOR: f64 = 1.7;
        const EXPONENT: f64 = 1.3;

        (BASE_XP * f64::powf(level as f64, EXPONENT) + (level as f64 * LINEAR_FACTOR)) as u64
    }
}

impl Artifact {
    pub fn new<S: AsRef<str>>(
        name: S,
        main_stat_type: BonusStatType,
        main_stat: BonusStat,
    ) -> Self {
        Artifact {
            id: uuid::Uuid::new_v4(),
            name: String::from(name.as_ref()),

            xp: 0,
            level: 1,

            main_stat_type,
            main_stat,
            sub_stats: BTreeMap::new(),
        }
    }

    pub fn upgrade(&mut self, materials: Vec<Material>) {
        let mut upgrade_xp = materials.iter().map(Material::xp).sum();

        while upgrade_xp != 0 {
            let xp_needed = Artifact::xp_to_next_level(self.level) - self.xp;
            let used_xp = min(xp_needed, upgrade_xp);
            upgrade_xp -= used_xp;

            self.xp += used_xp;
            if used_xp == xp_needed {
                self.level += 1;
            }
        }
    }
}
