use std::fmt::Display;

use crate::utils::{
    self,
    cmdln::{self, Bar},
};

pub mod create {
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
    pub(crate) use lvl_xp;

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
    pub(crate) use health_mana;

    macro_rules! defense {
        ($phys:literal, $magic:literal, $dodge:literal) => {
            (
                StatType::Defense,
                Stat::Defense {
                    physical_armor: $phys,
                    magic_resistance: $magic,
                    dodge_chance: $dodge,
                },
            )
        };
    }
    pub(crate) use defense;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StatType {
    Level,
    Health,
    Mana,
    Defense,
    Wisdom,
    Crafting,
    Enhancement,
    Combat,
    Magic,
    Luck,
}

impl Display for StatType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
    Wisdom {
        level: u32,
        xp: u64,
    },
    Defense {
        physical_armor: u32,
        magic_resistance: u32,
        dodge_chance: f64,
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

impl Stat {
    pub fn xp_curve(level: u32) -> u64 {
        const BASE_XP: f64 = 1000.0;
        const LINEAR_FACTOR: f64 = 1.7;
        const EXPONENT: f64 = 1.3;

        (BASE_XP * f64::powf(level as f64, EXPONENT) + (level as f64 * LINEAR_FACTOR)) as u64
    }
}

#[cfg(feature = "cmdln")]
use cmdln::config::*;

#[cfg(feature = "cmdln")]
impl Stat {
    fn xp_bar(stat_type: StatType, level: u32, xp: u64) -> String {
        const LEVEL: &str = "Level";
        const XP: &str = "XP";

        let xp_to_next_level = Stat::xp_curve(level);
        let mut bar = Bar::new(
            BAR_BRACKET_LEFT.to_string(),
            BAR_BRACKET_RIGHT.to_string(),
            BAR_FILL,
            BAR_BLANK,
            BAR_WIDTH,
        );
        bar.labels(
            Some(format!("{LEVEL} {level}")),
            Some(format!(
                "{} {XP}",
                cmdln::thousands_separator(xp_to_next_level)
            )),
        );
        bar.progress(
            ((xp as f64 / xp_to_next_level as f64) * BAR_WIDTH as f64) as usize,
            Some(cmdln::thousands_separator(xp)),
        );

        // stat_type.to_string() needed becuase else the indentation doesn't work :/ maybe fmt bug?
        format!(
            "{:^1$}\n{bar}",
            stat_type.to_string(),
            BAR_WIDTH + BAR_BRACKET_LEFT.len() + BAR_BRACKET_RIGHT.len()
        )
    }

    fn regen_stat_bar(stat_type: StatType, max: u64, current: u64, regen: f64) -> String {
        #[allow(non_snake_case)]
        let (REGEN, STAT) = match stat_type {
            StatType::Health => ("HP/s", "HP"),
            StatType::Mana => ("Mana/s", "Mana"),
            _ => unreachable!("{stat_type} is invalid"),
        };

        let mut bar = Bar::new(
            BAR_BRACKET_LEFT.to_string(),
            BAR_BRACKET_RIGHT.to_string(),
            BAR_FILL,
            BAR_BLANK,
            BAR_WIDTH,
        );
        bar.labels(
            Some(format!("{regen:.02} {REGEN}")),
            Some(format!("{} {STAT}", cmdln::thousands_separator(max))),
        );
        bar.progress(
            ((current as f64 / max as f64) * BAR_WIDTH as f64) as usize,
            Some(cmdln::thousands_separator(current)),
        );

        // stat_type.to_string() needed becuase else the indentation doesn't work :/ maybe fmt bug?
        format!(
            "{:^1$}\n{bar}",
            stat_type.to_string(),
            BAR_WIDTH + BAR_BRACKET_LEFT.len() + BAR_BRACKET_RIGHT.len()
        )
    }
}

#[cfg(feature = "cmdln")]
impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Stat::*;

        let out = match self {
            Level { level, xp } => Stat::xp_bar(StatType::Level, *level, *xp),
            Health {
                max,
                current,
                regen,
            } => Stat::regen_stat_bar(StatType::Health, *max, *current, *regen),
            Mana {
                max,
                current,
                regen,
            } => Stat::regen_stat_bar(StatType::Mana, *max, *current, *regen),
            Wisdom { level, xp } => Stat::xp_bar(StatType::Wisdom, *level, *xp),
            Defense {
                physical_armor,
                magic_resistance,
                dodge_chance,
            } => {
                const DEFENSE_TITLE: &str = "Defense";
                const ARMOR: &str = "Armor:";
                const MAGIC_RESISTANCE: &str = "Magic Resistance:";
                const DODGE_CHANCE: &str = "Dodge Chance:";
                const LABEL_OFFSET: usize = utils::const_max(
                    ARMOR.len(),
                    utils::const_max(MAGIC_RESISTANCE.len(), DODGE_CHANCE.len()),
                );

                format!(
                    "{DEFENSE_TITLE:^2$}\n\
                    {ARMOR:<3$} {}\n\
                    {MAGIC_RESISTANCE:<3$} {}\n\
                    {DODGE_CHANCE:<3$} {dodge_chance:.02}%",
                    cmdln::thousands_separator(*physical_armor),
                    cmdln::thousands_separator(*magic_resistance),
                    BAR_WIDTH + BAR_BRACKET_LEFT.len() + BAR_BRACKET_RIGHT.len(),
                    LABEL_OFFSET,
                )
            }
            Crafting { level, xp } => Stat::xp_bar(StatType::Crafting, *level, *xp),
            Enhancement { level, xp } => Stat::xp_bar(StatType::Enhancement, *level, *xp),
            Combat { level, xp } => Stat::xp_bar(StatType::Combat, *level, *xp),
            Magic { level, xp } => Stat::xp_bar(StatType::Magic, *level, *xp),
            Luck { level, xp } => Stat::xp_bar(StatType::Luck, *level, *xp),
        };

        write!(f, "{}", out)
    }
}
