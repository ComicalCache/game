use std::cmp::min;
use std::collections::BTreeMap;

use rand::seq::IteratorRandom;
use rand::thread_rng;
use uuid::Uuid;

use crate::items::bonus_stats::{BonusStat, BonusStatType};
use crate::xp::{Xp, XpCurve};

#[derive(Debug)]
pub struct Armor {
    id: Uuid,
    name: String,

    xp: u64,
    level: u32,

    main_stat_type: BonusStatType,
    main_stat: BonusStat,
    sub_stats: BTreeMap<BonusStatType, BonusStat>,
}


impl Xp for Armor {
    fn xp(&self) -> u64 {
        self.xp
    }
}

impl XpCurve for Armor {
    fn xp_to_next_level(&self) -> u64 {
        const BASE_XP: f64 = 1000.0;
        const LINEAR_FACTOR: f64 = 1.7;
        const EXPONENT: f64 = 1.3;

        let level = self.level as f64;
        (BASE_XP * f64::powf(level, EXPONENT) + (level * LINEAR_FACTOR)) as u64
    }
}

impl Armor {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let main_stat_type = *BonusStatType::random_from([BonusStatType::ArmorFlat,
            BonusStatType::ArmorPercent,
            BonusStatType::MagicResistanceFlat,
            BonusStatType::MagicResistancePercent].iter()).unwrap();

        Armor {
            id: Uuid::new_v4(),
            name: String::from(name.as_ref()),

            xp: 0,
            level: 1,

            main_stat_type,
            main_stat: BonusStat::from_type(main_stat_type),
            sub_stats: BTreeMap::new(),
        }
    }

    pub fn upgrade(&mut self, xp_source: Vec<Box<dyn Xp>>) {
        let mut upgrade_xp = xp_source.iter().map(|s| s.xp()).sum();

        while upgrade_xp != 0 {
            let xp_needed = self.xp_to_next_level() - self.xp;
            let used_xp = min(xp_needed, upgrade_xp);
            upgrade_xp -= used_xp;

            self.xp += used_xp;
            if used_xp == xp_needed {
                self.level += 1;

                // upgrade main stat
                self.main_stat.upgrade();

                // upgrade random sub stat
                let sub_stats_len = self.sub_stats.len();
                if sub_stats_len > 0 && self.level % 5 == 0 {
                    self.sub_stats.values_mut().choose(&mut thread_rng()).unwrap().upgrade();
                }

                if self.level % 10 == 0 {
                    // add random sub stat
                    // TODO: implement character wisdom limiter
                    let bonus_stat_type = rand::random();
                    self.sub_stats.insert(bonus_stat_type, BonusStat::from_type(bonus_stat_type));
                }
            }
        }
    }
}
