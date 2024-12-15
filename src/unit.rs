pub trait Attack {
    fn attack(&self, target: &mut dyn Targetable);
}

pub trait Targetable {
    fn take_damage(&mut self, damage: i32);
    fn health(&self) -> i32;
    fn name(&self) -> &str;
}

pub struct Unit {
    name: String,
    health: i32,
    attack_ability: Box<dyn Attack>,
}

impl Unit {
    pub fn new(name: String, health: i32, attack: Box<dyn Attack>) -> Unit {
        Unit {
            name,
            health,
            attack_ability: attack,
        }
    }

    pub fn attack_target(&self, target: &mut dyn Targetable) {
        self.attack_ability.attack(target);
    }
}

impl Targetable for Unit {
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
