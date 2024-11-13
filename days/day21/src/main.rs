use core::panic;

use common::read_input;

#[derive(Debug, Clone)]
struct Item {
    #[allow(dead_code)]
    name: String,
    price: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone)]
enum Equip {
    Weapon(Item),
    Armor(Item),
    Ring(Item),
}

impl Equip {
    fn new_weapon(name: &str, price: i32, damage: i32) -> Self {
        Self::Weapon(Item {
            name: name.to_string(),
            price,
            damage,
            armor: 0,
        })
    }

    fn new_armor(name: &str, price: i32, armor: i32) -> Self {
        Self::Armor(Item {
            name: name.to_string(),
            price,
            damage: 0,
            armor,
        })
    }

    fn new_ring(name: &str, price: i32, damage: i32, armor: i32) -> Self {
        Self::Ring(Item {
            name: name.to_string(),
            price,
            damage,
            armor,
        })
    }

    fn get_damage(&self) -> i32 {
        match self {
            Equip::Weapon(w) => w.damage,
            Equip::Armor(_) => 0,
            Equip::Ring(r) => r.damage,
        }
    }

    fn get_armor(&self) -> i32 {
        match self {
            Equip::Weapon(_) => 0,
            Equip::Armor(a) => a.armor,
            Equip::Ring(r) => r.armor,
        }
    }

    fn get_price(&self) -> i32 {
        match self {
            Equip::Weapon(w) => w.price,
            Equip::Armor(a) => a.price,
            Equip::Ring(r) => r.price,
        }
    }
}

#[derive(Debug)]
struct Hero {
    hp: i32,
    weapon: Equip,
    armor: Option<Equip>,
    left_ring: Option<Equip>,
    right_ring: Option<Equip>,
}

impl Hero {
    fn new(hp: i32, weapon: Equip) -> Self {
        Self {
            hp,
            weapon,
            armor: None,
            left_ring: None,
            right_ring: None,
        }
    }

    fn with_armor(mut self, armor: Equip) -> Self {
        self.armor = Some(armor);
        self
    }

    fn with_left_ring(mut self, ring: Equip) -> Self {
        self.left_ring = Some(ring);
        self
    }

    fn with_right_ring(mut self, ring: Equip) -> Self {
        self.right_ring = Some(ring);
        self
    }

    fn get_damage(&self) -> i32 {
        let mut damage = self.weapon.get_damage();
        damage += if let Some(ring) = &self.left_ring {
            ring.get_damage()
        } else {
            0
        };
        damage += if let Some(ring) = &self.right_ring {
            ring.get_damage()
        } else {
            0
        };
        damage
    }

    fn get_armor(&self) -> i32 {
        let mut damage = if let Some(armor) = &self.armor {
            armor.get_armor()
        } else {
            0
        };
        damage += if let Some(ring) = &self.left_ring {
            ring.get_armor()
        } else {
            0
        };
        damage += if let Some(ring) = &self.right_ring {
            ring.get_armor()
        } else {
            0
        };
        damage
    }

    fn hit(&mut self, boss: &mut Boss) {
        boss.hp -= 1.max(self.get_damage() - boss.armor)
    }

    fn fight(&mut self, boss: &mut Boss) -> bool {
        self.hit(boss);
        if boss.hp <= 0 {
            return true;
        }
        boss.hit(self);
        if self.hp <= 0 {
            return false;
        }
        self.fight(boss)
    }

    fn forecast_price(
        price_kind: PriceKind,
        outcome: Outcome,
        equips: &mut Vec<Vec<Equip>>,
    ) -> i32 {
        let equips = if let PriceKind::Max = price_kind {
            equips.reverse();
            equips
        } else {
            equips
        };

        equips
            .iter()
            .find(|c| {
                let (w, a, r1, r2) = c.iter().fold(
                    (None, None, None, None),
                    |(mut w, mut a, mut r1, mut r2), e| {
                        match e {
                            Equip::Weapon(_) => w = Some(e.clone()),
                            Equip::Armor(_) => a = Some(e.clone()),
                            Equip::Ring(_) => {
                                if r1.is_none() {
                                    r1 = Some(e.clone())
                                } else if r2.is_none() {
                                    r2 = Some(e.clone())
                                } else {
                                    panic!("Too many rings in stock")
                                }
                            }
                        };
                        (w, a, r1, r2)
                    },
                );
                let input = read_input("day21.txt");
                let mut boss = Boss::from(input);
                let mut hero = if let Some(w) = w {
                    Hero::new(100, w)
                } else {
                    panic!("No weapon for the hero!")
                };
                hero = if let Some(a) = a {
                    hero.with_armor(a)
                } else {
                    hero
                };
                hero = if let Some(r1) = r1 {
                    hero.with_left_ring(r1)
                } else {
                    hero
                };
                hero = if let Some(r2) = r2 {
                    hero.with_right_ring(r2)
                } else {
                    hero
                };
                if let Outcome::Win = outcome {
                    hero.fight(&mut boss)
                } else {
                    !hero.fight(&mut boss)
                }
            })
            .map(|c| c.iter().map(|e| e.get_price()).sum::<i32>())
            .unwrap()
    }
}

enum PriceKind {
    Min,
    Max,
}

enum Outcome {
    Win,
    Lose,
}

#[derive(Debug)]
struct Boss {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl From<String> for Boss {
    fn from(input: String) -> Self {
        let stats = input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Self {
            hp: stats[0],
            damage: stats[1],
            armor: stats[2],
        }
    }
}

impl Boss {
    fn hit(&self, hero: &mut Hero) {
        hero.hp -= 1.max(self.damage - hero.get_armor());
    }
}

struct Shop {
    items: Vec<Equip>,
}

impl Shop {
    fn new() -> Self {
        let dagger = Equip::new_weapon("Dagger", 8, 4);
        let shortsword = Equip::new_weapon("Shortsword", 10, 5);
        let warhammer = Equip::new_weapon("Warhammer", 25, 6);
        let longsword = Equip::new_weapon("Longsword", 40, 7);
        let greataxe = Equip::new_weapon("Greataxe", 74, 8);
        let leather = Equip::new_armor("Leather", 13, 1);
        let chainmail = Equip::new_armor("Chainmail", 31, 2);
        let splintmail = Equip::new_armor("Splintmail", 53, 3);
        let bandedmail = Equip::new_armor("Leather", 75, 4);
        let platemail = Equip::new_armor("Platemail", 102, 5);
        let damage_1 = Equip::new_ring("Damage +1", 25, 1, 0);
        let damage_2 = Equip::new_ring("Damage +2", 50, 2, 0);
        let damage_3 = Equip::new_ring("Damage +3", 100, 3, 0);
        let defense_1 = Equip::new_ring("Defense +1", 20, 0, 1);
        let defense_2 = Equip::new_ring("Defense +2", 40, 0, 2);
        let defense_3 = Equip::new_ring("Defense +3", 80, 0, 3);
        Self {
            items: vec![
                dagger, shortsword, warhammer, longsword, greataxe, leather, chainmail, splintmail,
                bandedmail, platemail, damage_1, damage_2, damage_3, defense_1, defense_2,
                defense_3,
            ],
        }
    }

    fn combinations(&self) -> Vec<Vec<Equip>> {
        fn combine(stock: &[Equip], max_equips: usize) -> Vec<Vec<Equip>> {
            if stock.is_empty() {
                return vec![vec![]];
            }
            if max_equips == 0 {
                return vec![vec![]];
            }
            let (first_equip, rest_equip) = stock.split_first().unwrap();
            let combination_with_first = combine(rest_equip, max_equips - 1);
            let combination_with_first = combination_with_first
                .iter()
                .map(|combination| {
                    let mut updated = combination.clone();
                    updated.insert(0, first_equip.clone());
                    updated
                })
                .collect::<Vec<_>>();

            let mut combinations_without_first = combine(rest_equip, max_equips).clone();
            let mut result = combination_with_first.clone();
            result.append(&mut combinations_without_first);
            result
        }
        let mut combinations = combine(&self.items, 4)
            .iter()
            .cloned()
            .filter(|c| {
                let (w, a, r) = c.iter().fold((0, 0, 0), |(mut w, mut a, mut r), e| {
                    match e {
                        Equip::Weapon(_) => w += 1,
                        Equip::Armor(_) => a += 1,
                        Equip::Ring(_) => r += 1,
                    };
                    (w, a, r)
                });
                w == 1 && a <= 1 && r <= 2
            })
            .collect::<Vec<_>>();
        combinations.sort_by(|c1, c2| {
            let p1: i32 = c1.iter().map(|e| e.get_price()).sum();
            let p2: i32 = c2.iter().map(|e| e.get_price()).sum();
            p1.cmp(&p2)
        });
        combinations
    }
}

fn main() {
    let shop = Shop::new();
    let mut combinations = shop.combinations();
    println!(
        "Part 1 = {}",
        Hero::forecast_price(PriceKind::Min, Outcome::Win, &mut combinations)
    );
    println!(
        "Part 2 = {}",
        Hero::forecast_price(PriceKind::Max, Outcome::Lose, &mut combinations)
    );
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn part1() {
        let shortsword = Equip::new_weapon("Shortsword", 10, 5);
        let bandedmail = Equip::new_armor("Leather", 75, 4);
        let defense_1 = Equip::new_ring("Defense +1", 20, 0, 1);
        let mut hero = Hero::new(8, shortsword)
            .with_armor(bandedmail)
            .with_left_ring(defense_1);
        let mut boss = Boss::from(
            r#"Hit Points: 12
Damage: 7
Armor: 2"#
                .to_string(),
        );
        assert!(hero.fight(&mut boss));
    }
}
