use crate::xp::xp_curve::XpCurveOption;

macro_rules! lvl_xp {
        ($kind:ident, $lvl:literal, $xp:literal) => {
            (
                StatType::$kind,
                Stat::$kind {
                    level: $lvl,
                    xp: $xp,
                },
            )
        };
    }
pub(in crate::character) use lvl_xp;

macro_rules! health_mana {
        ($kind:ident, $max:literal, $curr:literal, $reg:literal) => {
            (
                StatType::$kind,
                Stat::$kind {
                    max: $max,
                    current: $curr,
                    regen: $reg,
                },
            )
        };
    }
pub(in crate::character) use health_mana;

macro_rules! simple {
        ($kind:ident, $val:literal) => {
            (StatType::$kind, Stat::$kind($val))
        };
    }
pub(in crate::character) use simple;

macro_rules! defense {
        ($phys:literal, $magic:literal, $dodge:literal) => {
            (
                StatType::Defense,
                Stat::Defense {
                    armor: $phys,
                    magic_resistance: $magic,
                    dodge_chance: $dodge,
                },
            )
        };
    }
pub(in crate::character) use defense;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StatType {
    Level,
    Health,
    Mana,
    AttackDamage,
    AttackSpeed,
    MagicDamage,
    Defense,
    Strength,
    Wisdom,
    Crafting,
    Enhancement,
    Combat,
    Magic,
    Luck,
}

#[derive(Debug)]
pub enum Stat {
    Level {
        level: u32,
        xp: u64,
    },
    Health {
        max: u64,
        current: u64,
        // sec
        regen: f64,
    },
    Mana {
        max: u64,
        current: u64,
        // sec
        regen: f64,
    },
    AttackDamage(u64),
    AttackSpeed(f64),
    MagicDamage(u64),
    Defense {
        armor: u32,
        magic_resistance: u32,
        dodge_chance: f64,
    },
    Strength {
        level: u32,
        xp: u64,
    },
    Wisdom {
        level: u32,
        xp: u64,
    },
    Crafting {
        level: u32,
        xp: u64,
    },
    Enhancement {
        level: u32,
        xp: u64,
    },
    Combat {
        level: u32,
        xp: u64,
    },
    Magic {
        level: u32,
        xp: u64,
    },
    Luck {
        level: u32,
        xp: u64,
    },
}

impl XpCurveOption for Stat {
    fn xp_to_next_level(&self) -> Option<u64> {
        use Stat::*;

        const BASE_XP: f64 = 1000.0;
        const LINEAR_FACTOR: f64 = 1.7;
        const EXPONENT: f64 = 1.3;

        match self {
            Level { level, .. } | Strength { level, .. } | Wisdom { level, .. } | Crafting { level, .. } | Enhancement { level, .. } | Combat { level, .. } | Magic { level, .. } | Luck { level, .. } => Some(
                (BASE_XP * f64::powf(*level as f64, EXPONENT) + (*level as f64 * LINEAR_FACTOR)) as u64,
            ),
            _ => None,
        }
    }
}
