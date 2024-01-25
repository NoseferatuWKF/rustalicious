use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Unit {
    _name: &'static str,
    unit_type: UnitType,
    turn_gauge: usize,
    hp: u16,
    atk: u8,
    def: u8,
    spd: u8,
}

impl Unit {
    pub fn new(_name: &'static str, unit_type: UnitType, turn_gauge: usize, hp: u16, atk: u8, def: u8, spd: u8) -> Unit {
        Unit { _name, unit_type, turn_gauge, hp, atk, def, spd, }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnitType {
    Character,
    Enemy,
}

#[derive(Debug)]
pub struct ATBManager {
    characters: HashMap<&'static str, Unit>,
    enemies: HashMap<&'static str, Unit>,
}

impl ATBManager {
    pub fn new(characters: HashMap<&'static str, Unit>, enemies: HashMap<&'static str, Unit>) -> ATBManager {
        ATBManager { characters, enemies, }
    }

    fn set_target(targets: HashMap<&str, Unit>) -> Option<(&str, Unit)> {
        targets.into_iter().next()
    }

    pub fn start(&mut self) {
        if self.characters.len() == 0 || self.enemies.len() == 0 {
            return;
        }
        let turn_rate: usize = 100;
        let mut enemies = self.enemies.clone();
        let mut characters = self.characters.clone();
        let mut units = characters.iter_mut().chain(&mut enemies).collect::<Vec<_>>();
        loop {
            for (key, unit) in units.iter_mut() {
                unit.turn_gauge = turn_rate.min(unit.turn_gauge + (unit.spd as usize));
                let mut _fill = turn_rate.min(unit.turn_gauge);
                let mut _gap = turn_rate.min(turn_rate - unit.turn_gauge.min(turn_rate));
                match unit.unit_type {
                    UnitType::Character => {
                        let character = self.characters.get(key.clone()).unwrap();
                        println!("[\x1b[;32;1m{:|<_fill$}{: <_gap$}\x1b[;0m] {} [HP: {}]", "", "", key, character.hp);
                        if unit.turn_gauge == turn_rate {
                            if let Some((key, mut enemy)) = ATBManager::set_target(self.enemies.clone()) {
                                enemy.hp -= enemy.hp.min((unit.atk - enemy.def.min(unit.atk)) as u16);
                                self.enemies.insert(key, enemy);
                                if enemy.hp == 0 {
                                    self.enemies.remove(key);
                                    return self.start();
                                }
                            }
                            unit.turn_gauge = 0;
                            _fill = 0 as usize;
                            _gap = turn_rate as usize;
                        }
                    },
                    UnitType::Enemy => {
                        let enemy = self.enemies.get(key.clone()).unwrap();
                        println!("[\x1b[;31;1m{:|<_fill$}{: <_gap$}\x1b[;0m] {} [HP: {}]", "", "", key, enemy.hp);
                        if unit.turn_gauge == turn_rate {
                            if let Some((key, mut character)) = ATBManager::set_target(self.characters.clone()) {
                                character.hp -= character.hp.min((unit.atk - character.def.min(unit.atk)) as u16);
                                self.characters.insert(key, character);
                                if character.hp == 0 {
                                    self.characters.remove(key);
                                    return self.start();
                                }
                            }
                            unit.turn_gauge = 0;
                            _fill = 0 as usize;
                            _gap = turn_rate as usize;
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
            std::process::Command::new("clear").status().expect("failed");
        }
    }
}
