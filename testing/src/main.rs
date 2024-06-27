use sdk::bonus_stats::{BonusStat, BonusStatType};
use sdk::items::{Artifact, Material};
use sdk::XpCurve;

fn main() {
    let mut a = Artifact::new(
        "Piss Chalice",
        BonusStatType::AttackDamagePercent,
        BonusStat::AttackDamagePercent(2.0),
    );
    println!("{:?}", a);
    println!("Needed: {}", Artifact::xp_to_next_level(a.level) - a.xp);

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let xp: u64 = buffer.trim().parse().unwrap();
        let m = Material::new("Piss Chalice Enhancement", xp);
        a.upgrade(vec![m]);
        println!("{:?}", a);
        println!("Needed: {}", Artifact::xp_to_next_level(a.level) - a.xp);
    }
}
