mod concepts;

fn main() {
    atb();
}

fn atb() {
    use crate::concepts::rpg::atb::*;
    use std::collections::HashMap;

    let characters = HashMap::from([
        ("Hero", Unit::new("Hero", UnitType::Character, 0, 1000, 25, 15, 6)),
        ("Archer", Unit::new("Archer", UnitType::Character, 0, 500, 23, 7, 9)),
    ]);

    let enemies = HashMap::from([
        ("Dragon",Unit::new("Dragon", UnitType::Enemy, 0, 1250, 30, 20, 3)),
    ]);

    let mut atb = ATBManager::new(characters, enemies);

    atb.start();
}
