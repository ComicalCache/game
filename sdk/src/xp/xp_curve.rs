pub trait XpCurve {
    fn xp_to_next_level(&self) -> u64;
}

pub trait XpCurveOption {
    fn xp_to_next_level(&self) -> Option<u64>;
}

impl<T: XpCurve> XpCurveOption for T {
    fn xp_to_next_level(&self) -> Option<u64> {
        Some((self as &dyn XpCurve).xp_to_next_level())
    }
}
