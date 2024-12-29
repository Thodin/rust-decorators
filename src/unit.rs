pub trait Attack {
    fn attack<Target: Targetable>(&self, target: &mut Target);
}

pub trait Targetable {
    fn take_damage(&mut self, damage: i32);
    fn health(&self) -> i32;
    fn name(&self) -> &str;
}

pub struct Unit<T: Attack> {
    name: String,
    health: i32,
    attack_ability: T,
}

impl<T: Attack> Unit<T> {
    pub fn new(name: String, health: i32, attack: T) -> Unit<T> {
        Unit {
            name,
            health,
            attack_ability: attack,
        }
    }

    pub fn attack_target<Target: Targetable>(&self, target: &mut Target) {
        self.attack_ability.attack(target);
    }
}

impl<T: Attack> Targetable for Unit<T> {
    fn take_damage(&mut self, damage: i32) {
        if damage > 0 {
            println!("{} takes {} damage.", self.name, damage);
            self.health -= damage;
        }
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn name(&self) -> &str {
        &self.name
    }
}
