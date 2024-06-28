use rand;

use sdk::items::{Artifact, Material, Weapon};
use sdk::xp::{Xp, XpCurve};

fn main() {
    let mut a = Weapon::new("Piss Chalice");
    println!("{:#?}", a);
    println!("XP needed: {}", a.xp_to_next_level() - a.xp());

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let xp: u64 = buffer.trim().parse().unwrap();
        let m = Material::new("Piss Chalice Enhancement", xp);
        a.upgrade(vec![Box::new(m)]);
        println!("{:#?}", a);
        println!("XP needed: {}", a.xp_to_next_level() - a.xp());
    }
}
