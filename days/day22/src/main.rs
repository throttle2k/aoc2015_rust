use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Character {
    hp: i32,
    mana: i32,
    armor: i32,
    damage: i32,
}

#[derive(Clone, Debug)]
struct Spell {
    name: String,
    cost: i32,
    effect: Effect,
}

#[derive(Clone)]
struct Effect {
    duration: i32,
    apply: CloneableFnBox,
    revert: CloneableFnBox,
}

trait CloneableFn: Fn(&mut Character, &mut Character) + 'static {
    fn clone_box(&self) -> Box<dyn CloneableFn>;
}

impl<T> CloneableFn for T
where
    T: Fn(&mut Character, &mut Character) + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFn> {
        Box::new(self.clone())
    }
}

struct CloneableFnBox {
    inner: Box<dyn CloneableFn>,
}

impl CloneableFnBox {
    fn call(&self, player: &mut Character, boss: &mut Character) {
        (self.inner)(player, boss);
    }
}

impl Clone for CloneableFnBox {
    fn clone(&self) -> Self {
        CloneableFnBox {
            inner: self.inner.clone_box(),
        }
    }
}

impl fmt::Debug for CloneableFnBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

impl fmt::Debug for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Effect")
            .field("duration", &self.duration)
            .finish()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GameStateKey {
    player_hp: i32,
    player_armor: i32,
    boss_hp: i32,
    mana: i32,
    effects: Vec<(String, i32)>,
    is_player_turn: bool,
    hard_mode: bool,
}

#[derive(Clone, Debug)]
struct GameState {
    player: Character,
    boss: Character,
    mana_spent: i32,
    active_effects: Vec<(String, Effect)>,
    is_player_turn: bool,
    hard_mode: bool,
}

impl GameState {
    fn to_key(&self) -> GameStateKey {
        GameStateKey {
            player_hp: self.player.hp,
            player_armor: self.player.armor,
            boss_hp: self.boss.hp,
            mana: self.player.mana,
            effects: self
                .active_effects
                .iter()
                .map(|(name, effect)| (name.clone(), effect.duration))
                .collect(),
            is_player_turn: self.is_player_turn,
            hard_mode: self.hard_mode,
        }
    }
}

fn simulate_turn(
    mut state: GameState,
    spells: &[Spell],
    memo: &mut HashMap<GameStateKey, Option<i32>>,
) -> Option<i32> {
    let key = state.to_key();

    if let Some(cached) = memo.get(&key) {
        return cached.clone();
    }

    if state.is_player_turn && state.hard_mode {
        state.player.hp -= 1;
    }

    apply_effects(&mut state);

    if state.player.hp <= 0 {
        return None;
    }

    if state.boss.hp <= 0 {
        return Some(state.mana_spent);
    }

    let mut results = Vec::new();
    if state.is_player_turn {
        let available_spells = spells
            .iter()
            .filter(|spell| {
                spell.cost <= state.player.mana
                    && state
                        .active_effects
                        .iter()
                        .all(|(active_name, _effect)| *active_name != spell.name)
            })
            .collect::<Vec<&Spell>>();
        for spell in available_spells {
            let mut new_state = state.clone();
            new_state.player.mana -= spell.cost;
            new_state.mana_spent += spell.cost;
            new_state.is_player_turn = false;

            new_state
                .active_effects
                .push((spell.name.clone(), spell.effect.clone()));

            let result = simulate_turn(new_state, spells, memo);
            if let Some(cost) = result {
                results.push(cost);
            }
        }
    } else {
        state.player.hp -= (state.boss.damage - state.player.armor).max(1);
        state.is_player_turn = true;
        let result = simulate_turn(state, spells, memo);
        if let Some(cost) = result {
            results.push(cost);
        }
    }

    let result = results.into_iter().min();
    memo.insert(key, result);
    result
}

fn apply_effects(state: &mut GameState) {
    state.active_effects.retain_mut(|(_, effect)| {
        effect.apply.call(&mut state.player, &mut state.boss);
        effect.duration -= 1;
        if effect.duration <= 0 {
            effect.revert.call(&mut state.player, &mut state.boss);
            false
        } else {
            true
        }
    });
}

fn main() {
    let spells = vec![
        Spell {
            name: "Magic Missile".to_owned(),
            cost: 53,
            effect: Effect {
                duration: 0,
                apply: CloneableFnBox {
                    inner: Box::new(|_player: &mut Character, boss: &mut Character| boss.hp -= 4),
                },
                revert: CloneableFnBox {
                    inner: Box::new(|_player: &mut Character, _boss: &mut Character| {}),
                },
            },
        },
        Spell {
            name: "Drain".to_owned(),
            cost: 73,
            effect: Effect {
                duration: 0,
                apply: CloneableFnBox {
                    inner: Box::new(|player: &mut Character, boss: &mut Character| {
                        player.hp += 2;
                        boss.hp -= 2;
                    }),
                },
                revert: CloneableFnBox {
                    inner: Box::new(|_player: &mut Character, _boss: &mut Character| {}),
                },
            },
        },
        Spell {
            name: "Shield".to_owned(),
            cost: 113,
            effect: Effect {
                duration: 6,
                apply: CloneableFnBox {
                    inner: Box::new(|player: &mut Character, _boss: &mut Character| {
                        player.armor = 7
                    }),
                },
                revert: CloneableFnBox {
                    inner: Box::new(|player: &mut Character, _boss: &mut Character| {
                        player.armor = 0
                    }),
                },
            },
        },
        Spell {
            name: "Poison".to_owned(),
            cost: 173,
            effect: Effect {
                duration: 6,
                apply: CloneableFnBox {
                    inner: Box::new(|_player: &mut Character, boss: &mut Character| {
                        boss.hp -= 3;
                    }),
                },
                revert: CloneableFnBox {
                    inner: Box::new(|_player: &mut Character, _boss: &mut Character| {}),
                },
            },
        },
        Spell {
            name: "Recharge".to_owned(),
            cost: 229,
            effect: Effect {
                duration: 5,
                apply: CloneableFnBox {
                    inner: Box::new(|player: &mut Character, _boss: &mut Character| {
                        player.mana += 101
                    }),
                },
                revert: CloneableFnBox {
                    inner: Box::new(|_player: &mut Character, _boss: &mut Character| {}),
                },
            },
        },
    ];

    let initial_state = GameState {
        player: Character {
            hp: 50,
            mana: 500,
            armor: 0,
            damage: 0,
        },
        boss: Character {
            hp: 51,
            mana: 0,
            armor: 0,
            damage: 9,
        },
        mana_spent: 0,
        active_effects: Vec::new(),
        is_player_turn: true,
        hard_mode: false,
    };

    let mut memo = HashMap::new();
    if let Some(min_mana) = simulate_turn(initial_state, &spells, &mut memo) {
        println!("Part 1: {}", min_mana);
    } else {
        println!("No solution found.");
    }

    let initial_state = GameState {
        player: Character {
            hp: 50,
            mana: 500,
            armor: 0,
            damage: 0,
        },
        boss: Character {
            hp: 51,
            mana: 0,
            armor: 0,
            damage: 9,
        },
        mana_spent: 0,
        active_effects: Vec::new(),
        is_player_turn: true,
        hard_mode: true,
    };

    let mut memo = HashMap::new();
    if let Some(min_mana) = simulate_turn(initial_state, &spells, &mut memo) {
        println!("Part 2: {}", min_mana);
    } else {
        println!("No solution found.");
    }
}
