use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rand::seq::IteratorRandom;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum BonusStatType {
    MaxHealthFlat,
    MaxHealthPercent,
    HealthRegen,
    MaxManaFlat,
    MaxManaPercent,
    ManaRegen,
    AttackDamageFlat,
    AttackDamagePercent,
    AttackSpeed,
    MagicDamageFlat,
    MagicDamagePercent,
    ArmorFlat,
    ArmorPercent,
    MagicResistanceFlat,
    MagicResistancePercent,
    DodgeChance,
}

impl Distribution<BonusStatType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BonusStatType {
        use BonusStatType::*;

        // ! THIS MUST BE UPDATED WHEN BonusStatType MIGHT BE EXPANDED
        match rng.gen_range(0..16) {
            0 => MaxHealthFlat,
            1 => MaxHealthPercent,
            2 => HealthRegen,
            3 => MaxManaFlat,
            4 => MaxManaPercent,
            5 => ManaRegen,
            6 => AttackDamageFlat,
            7 => AttackDamagePercent,
            8 => AttackSpeed,
            9 => MagicDamageFlat,
            10 => MagicDamagePercent,
            11 => ArmorFlat,
            12 => ArmorPercent,
            13 => MagicResistanceFlat,
            14 => MagicResistancePercent,
            15 => DodgeChance,
            _ => unreachable!(),
        }
    }
}

impl BonusStatType {
    pub fn random_from<'a, I: Iterator<Item=&'a BonusStatType>>(iter: I) -> Option<&'a Self> {
        iter.choose(&mut rand::thread_rng())
    }
}

#[derive(Debug)]
pub enum BonusStat {
    MaxHealthFlat(u64),
    MaxHealthPercent(f64),
    HealthRegen(f64),
    MaxManaFlat(u64),
    MaxManaPercent(f64),
    ManaRegen(f64),
    AttackDamageFlat(u64),
    AttackDamagePercent(f64),
    AttackSpeed(f64),
    MagicDamageFlat(u64),
    MagicDamagePercent(f64),
    ArmorFlat(u32),
    ArmorPercent(f32),
    MagicResistanceFlat(u32),
    MagicResistancePercent(f32),
    DodgeChance(f64),
}

impl BonusStat {
    pub fn from_type(r#type: BonusStatType) -> Self {
        use BonusStat::*;
        let mut rng = rand::thread_rng();

        match r#type {
            BonusStatType::MaxHealthFlat => MaxHealthFlat(rng.gen_range(77..137)),
            BonusStatType::MaxHealthPercent => MaxHealthPercent(rng.gen_range(0.17..0.37)),
            BonusStatType::HealthRegen => HealthRegen(rng.gen_range(0.02..0.07)),
            BonusStatType::MaxManaFlat => MaxManaFlat(rng.gen_range(27..57)),
            BonusStatType::MaxManaPercent => MaxManaPercent(rng.gen_range(0.17..0.37)),
            BonusStatType::ManaRegen => ManaRegen(rng.gen_range(0.02..0.07)),
            BonusStatType::AttackDamageFlat => AttackDamageFlat(rng.gen_range(17..47)),
            BonusStatType::AttackDamagePercent => AttackDamagePercent(rng.gen_range(0.17..0.37)),
            BonusStatType::AttackSpeed => AttackSpeed(rng.gen_range(0.02..0.07)),
            BonusStatType::MagicDamageFlat => MagicDamageFlat(rng.gen_range(17..47)),
            BonusStatType::MagicDamagePercent => MagicDamagePercent(rng.gen_range(0.17..0.37)),
            BonusStatType::ArmorFlat => ArmorFlat(rng.gen_range(17..47)),
            BonusStatType::ArmorPercent => ArmorPercent(rng.gen_range(0.17..0.37)),
            BonusStatType::MagicResistanceFlat => MagicResistanceFlat(rng.gen_range(17..47)),
            BonusStatType::MagicResistancePercent => {
                MagicResistancePercent(rng.gen_range(0.17..0.37))
            }
            BonusStatType::DodgeChance => DodgeChance(rng.gen_range(0.02..0.07)),
        }
    }

    pub fn upgrade(&mut self) {
        let mut rng = rand::thread_rng();

        match self {
            BonusStat::MaxHealthFlat(v) => *v += rng.gen_range(27..57),
            BonusStat::MaxHealthPercent(v) => *v += rng.gen_range(0.07..0.22),
            BonusStat::HealthRegen(v) => *v += rng.gen_range(0.01..0.04),
            BonusStat::MaxManaFlat(v) => *v += rng.gen_range(12..37),
            BonusStat::MaxManaPercent(v) => *v += rng.gen_range(0.07..0.22),
            BonusStat::ManaRegen(v) => *v += rng.gen_range(0.01..0.04),
            BonusStat::AttackDamageFlat(v) => *v += rng.gen_range(7..27),
            BonusStat::AttackDamagePercent(v) => *v += rng.gen_range(0.07..0.22),
            BonusStat::AttackSpeed(v) => *v += rng.gen_range(0.01..0.04),
            BonusStat::MagicDamageFlat(v) => *v += rng.gen_range(7..27),
            BonusStat::MagicDamagePercent(v) => *v += rng.gen_range(0.10..0.22),
            BonusStat::ArmorFlat(v) => *v += rng.gen_range(7..27),
            BonusStat::ArmorPercent(v) => *v += rng.gen_range(0.10..0.22),
            BonusStat::MagicResistanceFlat(v) => *v += rng.gen_range(7..27),
            BonusStat::MagicResistancePercent(v) => *v += rng.gen_range(0.07..0.22),
            BonusStat::DodgeChance(v) => *v += rng.gen_range(0.01..0.04),
        }
    }
}
