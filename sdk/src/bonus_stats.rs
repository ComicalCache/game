#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
