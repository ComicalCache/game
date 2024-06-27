use std::{cmp::min, collections::BTreeMap};

use rand::{seq::IteratorRandom, Rng};
use uuid::Uuid;

use crate::{
    items::bonus_stats::{BonusStat, BonusStatType},
    xp::{Xp, XpCurve},
};

#[derive(Debug)]
pub struct Artifact {
    id: Uuid,
    name: String,

    xp: u64,
    level: u32,

    main_stat_type: BonusStatType,
    main_stat: BonusStat,
    sub_stats: BTreeMap<BonusStatType, BonusStat>,
}

impl Xp for Artifact {
    fn xp(&self) -> u64 {
        self.xp
    }
}

impl XpCurve for Artifact {
    fn xp_to_next_level(&self) -> u64 {
        const BASE_XP: f64 = 1000.0;
        const LINEAR_FACTOR: f64 = 1.7;
        const EXPONENT: f64 = 1.3;

        (BASE_XP * f64::powf(self.level as f64, EXPONENT) + (self.level as f64 * LINEAR_FACTOR))
            as u64
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

    pub fn upgrade<R: Rng>(&mut self, xp_source: Vec<Box<dyn Xp>>, rng: &mut R) {
        let mut upgrade_xp = xp_source.iter().map(|s| s.xp()).sum();

        while upgrade_xp != 0 {
            let xp_needed = self.xp_to_next_level() - self.xp;
            let used_xp = min(xp_needed, upgrade_xp);
            upgrade_xp -= used_xp;

            self.xp += used_xp;
            if used_xp == xp_needed {
                self.level += 1;

                // upgrade main stat
                self.main_stat.upgrade(rng);

                // upgrade random substat
                let sub_stats_len = self.sub_stats.len();
                if sub_stats_len > 0 && self.level % 2 == 0 {
                    self.sub_stats
                        .values_mut()
                        .choose(rng)
                        .unwrap()
                        .upgrade(rng);
                }

                if self.level % 4 == 0 {
                    // add random substat
                    let bonus_stat_type = rand::random();
                    self.sub_stats
                        .insert(bonus_stat_type, BonusStat::from_type(bonus_stat_type, rng));
                }
            }
        }
    }
}
