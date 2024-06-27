use rand;

use sdk::items::bonus_stats::BonusStat;
use sdk::items::{Artifact, Material};
use sdk::xp::{Xp, XpCurve};

fn main() {
    let mut rng = rand::thread_rng();

    let bonus_stat_type = rand::random();

    let mut a = Artifact::new(
        "Piss Chalice",
        bonus_stat_type,
        BonusStat::from_type(bonus_stat_type, &mut rng),
    );
    println!("{:#?}", a);
    println!("XP needed: {}", a.xp_to_next_level() - a.xp());

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let xp: u64 = buffer.trim().parse().unwrap();
        let m = Material::new("Piss Chalice Enhancement", xp);
        a.upgrade(vec![Box::new(m)], &mut rng);
        println!("{:#?}", a);
        println!("XP needed: {}", a.xp_to_next_level() - a.xp());
    }
}
